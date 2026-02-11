use crate::api_v1::error_response;
use crate::error::ApiError;
use crate::http::Request;
use alloc::format;
use alloc::vec::Vec;
// use stem::thing::{ThingId, HandleId}; // Import HandleId trait for from_u64
use abi::schema::{keys, kinds};
use sha2::{Digest, Sha256};
use stem::info;
use stem::thing::sys::{
    bytespace_create, bytespace_map, bytespace_unmap, create_node, intern, prop_set,
};

/// POST /upload
/// Headers:
/// - X-File-Name: <filename> (optional, defaults to "upload_<timestamp>")
/// - Content-Length: <size>
/// Body: Raw file content
pub fn handle_upload_new(req: &Request<'_>, body: &[u8]) -> (&'static str, Vec<u8>) {
    // 1. Get filename
    let filename = req
        .get_header("X-File-Name")
        .or_else(|| req.get_header("x-file-name")) // Case insensitive check just in case
        .unwrap_or("unnamed_upload");

    // Decouple filename from path if sent full path (simple basename)
    let basename = filename.split('/').last().unwrap_or(filename);

    info!(
        "anther: Receiving upload '{}' ({} bytes)",
        basename,
        body.len()
    );

    // 2. Compute Hash
    let mut hasher = Sha256::new();
    hasher.update(body);
    let hash_bytes = hasher.finalize();
    let hash = u64::from_le_bytes([
        hash_bytes[0],
        hash_bytes[1],
        hash_bytes[2],
        hash_bytes[3],
        hash_bytes[4],
        hash_bytes[5],
        hash_bytes[6],
        hash_bytes[7],
    ]);

    // 3. Create Bytespace
    let bs_id = match bytespace_create(body.len(), 0, 0) {
        Ok(id) => id,
        Err(e) => {
            return (
                "500 Internal Server Error",
                ApiError::internal(format!("Failed to create bytespace: {:?}", e))
                    .to_json()
                    .into_bytes(),
            );
        }
    };

    // 4. Map & Write Data
    let ptr = match bytespace_map(bs_id) {
        Ok(ptr) => ptr,
        Err(e) => {
            // Cleanup bytespace if map fails? currently no delete syscall exposed easily here but in future yes
            return (
                "500 Internal Server Error",
                ApiError::internal(format!("Failed to map bytespace: {:?}", e))
                    .to_json()
                    .into_bytes(),
            );
        }
    };

    unsafe {
        core::ptr::copy_nonoverlapping(body.as_ptr(), ptr as *mut u8, body.len());
    }

    let _ = bytespace_unmap(bs_id, ptr);

    // 5. Create CONTENT_FILE node
    let file_id = match create_node(kinds::CONTENT_FILE) {
        Ok(id) => id,
        Err(e) => {
            return (
                "500 Internal Server Error",
                ApiError::internal(format!("Failed to create file node: {:?}", e))
                    .to_json()
                    .into_bytes(),
            );
        }
    };

    // 6. Set Properties
    let name_sym = intern(basename).unwrap_or(0);
    // Determine mime based on extension manually (simple heuristic)
    let mime = if basename.ends_with(".png") {
        "image/png"
    } else if basename.ends_with(".jpg") {
        "image/jpeg"
    } else if basename.ends_with(".svg") {
        "image/svg+xml"
    } else if basename.ends_with(".ttf") {
        "font/ttf"
    } else if basename.ends_with(".txt") {
        "text/plain"
    } else {
        "application/octet-stream"
    };
    let mime_sym = intern(mime).unwrap_or(0);

    let _ = prop_set(file_id, keys::FILE_NAME, name_sym as u64);
    let _ = prop_set(file_id, keys::FILE_BYTESPACE, bs_id.to_u64_lossy());
    let _ = prop_set(file_id, keys::FILE_SIZE, body.len() as u64);
    let _ = prop_set(file_id, keys::FILE_HASH, hash);
    if mime_sym != 0 {
        let _ = prop_set(file_id, keys::FILE_MIME, mime_sym as u64);
    }

    // Also set generic NAME for easy finding
    let _ = prop_set(file_id, keys::NAME, name_sym as u64);

    info!(
        "anther: Upload successful. Created file node {:?} for '{}'",
        file_id, basename
    );

    // 7. Return Success
    ("200 OK", b"{\"status\":\"ok\"}".to_vec())
}
