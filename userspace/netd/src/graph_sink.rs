//! Graph storage for fetched network resources

use abi::kinds::*;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use stem::thing::sys::*;
use stem::thing::ThingId;
use stem::xml::ingest::{ingest_xml_to_graph, SysGraphApply, XmlIngestOptions};
use stem::{info, warn};

#[derive(Debug)]
pub enum GraphError {
    CreateFailed,
    StoreFailed,
    XmlParseFailed,
}

pub fn store_fetch_result(
    url: &str,
    status_code: u16,
    body: &[u8],
) -> Result<ThingId, GraphError> {
    // Create node under /net/fetch/<timestamp>
    let timestamp = stem::time::now_unix_seconds();
    let node_id = create_node(KIND_NET_FETCH_RESOURCE).map_err(|_| GraphError::CreateFailed)?;

    info!("GRAPH: Created node {:?} for fetch result", node_id);

    // Compute URL hash for property storage
    let url_hash = hash_string(url);

    // Set properties
    prop_set(node_id, PROP_NET_URL, url_hash as u64).ok();
    prop_set(node_id, PROP_NET_STATUS, status_code as u64).ok();
    prop_set(node_id, PROP_NET_FETCHED_AT, timestamp).ok();

    // Create bytespace for body
    let bs_id = bytespace_create(body.len(), 0, 0).map_err(|_| GraphError::StoreFailed)?;

    // Write body to bytespace
    bytespace_write(bs_id, 0, body).map_err(|_| GraphError::StoreFailed)?;

    // Link bytespace to node
    link(node_id, intern("bytespace").unwrap_or(0), bs_id).ok();

    info!("GRAPH: Stored {} bytes in bytespace {:?}", body.len(), bs_id);

    // Try XML import
    if status_code >= 200 && status_code < 300 {
        match try_xml_import(node_id, body) {
            Ok(result) => {
                // Store the XML root node ID (ThingId is 16 bytes)
                let xml_bytes = result.root_element.0;
                let xml_hash = hash_bytes(&xml_bytes);
                prop_set(node_id, PROP_RESOURCE_XML_DOCUMENT, xml_hash as u64).ok();
                info!("GRAPH: XML import succeeded, root {:?}", result.root_element);
            }
            Err(e) => {
                warn!("GRAPH: XML import failed: {:?}, trying sanitizer", e);

                // Try sanitizing HTML
                let sanitized = sanitize_html(body);
                match try_xml_import(node_id, &sanitized) {
                    Ok(result) => {
                        prop_set(node_id, PROP_RESOURCE_XML_SANITIZED, 1).ok();
                        let xml_bytes = result.root_element.0;
                        let xml_hash = hash_bytes(&xml_bytes);
                        prop_set(node_id, PROP_RESOURCE_XML_DOCUMENT, xml_hash as u64).ok();
                        info!("GRAPH: XML import succeeded after sanitization, root {:?}", result.root_element);
                    }
                    Err(e2) => {
                        warn!("GRAPH: XML import failed even after sanitization: {:?}", e2);
                        let error_hash = hash_string(&format!("{:?}", e2));
                        prop_set(node_id, PROP_XML_IMPORT_ERROR, error_hash as u64).ok();
                    }
                }
            }
        }
    }

    Ok(node_id)
}

fn try_xml_import(parent: ThingId, xml_data: &[u8]) -> Result<stem::xml::ingest::XmlIngestResult, GraphError> {
    let mut apply = SysGraphApply;
    let options = XmlIngestOptions {
        attach_under: Some(parent),
        ..Default::default()
    };

    ingest_xml_to_graph(xml_data, options, &mut apply).map_err(|_| GraphError::XmlParseFailed)
}

fn sanitize_html(html: &[u8]) -> Vec<u8> {
    let html_str = core::str::from_utf8(html).unwrap_or("");
    let mut result = String::new();

    // Remove DOCTYPE
    let without_doctype = if let Some(pos) = html_str.find(">") {
        if html_str[..pos].contains("DOCTYPE") {
            &html_str[pos + 1..]
        } else {
            html_str
        }
    } else {
        html_str
    };

    // Remove comments
    let mut working = String::from(without_doctype);
    while let Some(start) = working.find("<!--") {
        if let Some(end) = working[start..].find("-->") {
            working.replace_range(start..start + end + 3, "");
        } else {
            break;
        }
    }

    // Self-close void tags
    let void_tags = ["br", "meta", "link", "img", "input", "hr", "area", "base", "col", "embed", "param", "source", "track", "wbr"];
    for tag in &void_tags {
        let open = format!("<{} ", tag);
        let close = format!("<{}>", tag);
        working = working.replace(&close, &format!("<{}/>", tag));
        
        // Find tags with attributes that aren't self-closed
        let mut pos = 0;
        while let Some(start) = working[pos..].find(&open) {
            let abs_start = pos + start;
            if let Some(end) = working[abs_start..].find(">") {
                let abs_end = abs_start + end;
                let tag_content = &working[abs_start..abs_end];
                if !tag_content.ends_with("/") {
                    working.replace_range(abs_end..abs_end + 1, "/>");
                }
                pos = abs_end + 2;
            } else {
                break;
            }
        }
    }

    // Ensure root element
    let trimmed = working.trim();
    if !trimmed.starts_with("<html") {
        result.push_str("<html>");
        result.push_str(trimmed);
        result.push_str("</html>");
    } else {
        result.push_str(trimmed);
    }

    result.into_bytes()
}

fn hash_string(s: &str) -> u32 {
    let mut hash: u32 = 5381;
    for byte in s.bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(byte as u32);
    }
    hash
}

fn hash_bytes(bytes: &[u8]) -> u32 {
    let mut hash: u32 = 5381;
    for &byte in bytes {
        hash = hash.wrapping_mul(33).wrapping_add(byte as u32);
    }
    hash
}
