extern crate alloc;

use abi::errors::Errno;
use abi::syscall::vfs_flags::{O_CREAT, O_RDONLY, O_TRUNC, O_WRONLY};
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use stem::syscall::vfs::{
    vfs_close, vfs_mkdir, vfs_open, vfs_read, vfs_readdir, vfs_seek, vfs_stat, vfs_write,
};
use stem::thing::ThingId;

pub const SESSION_ROOT: &str = "/session";
pub const SEATS_ROOT: &str = "/session/seat0";
pub const POINTER_ROOT: &str = "/session/seat0/pointer";
pub const POINTER_STATE_ROOT: &str = "/session/seat0/pointer/state";
pub const WINDOWS_ROOT: &str = "/session/windows";
pub const SURFACES_ROOT: &str = "/session/surfaces";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AttachedBuffer {
    pub fd: u32,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub format: u32,
}

pub fn ensure_session_roots() {
    let _ = vfs_mkdir(SESSION_ROOT);
    let _ = vfs_mkdir(SEATS_ROOT);
    let _ = vfs_mkdir(POINTER_ROOT);
    let _ = vfs_mkdir(POINTER_STATE_ROOT);
    let _ = vfs_mkdir(WINDOWS_ROOT);
    let _ = vfs_mkdir(SURFACES_ROOT);
    ensure_file(&format!("{}/events", POINTER_ROOT), "");
    ensure_file(&format!("{}/state/x", POINTER_ROOT), "0\n");
    ensure_file(&format!("{}/state/y", POINTER_ROOT), "0\n");
    ensure_file(&format!("{}/state/buttons", POINTER_ROOT), "0\n");
    ensure_file(&format!("{}/state/focus_surface", POINTER_ROOT), "");
    ensure_file(&format!("{}/state/sx", POINTER_ROOT), "0\n");
    ensure_file(&format!("{}/state/sy", POINTER_ROOT), "0\n");
}

pub fn ensure_window_tree(id: &str) {
    let base = window_path(id);
    let _ = vfs_mkdir(&base);
    let _ = vfs_mkdir(&format!("{}/shell", base));
    let _ = vfs_mkdir(&format!("{}/shell/requested", base));
    let _ = vfs_mkdir(&format!("{}/shell/current", base));
    let _ = vfs_mkdir(&format!("{}/bind", base));
    let _ = vfs_mkdir(&format!("{}/status", base));

    ensure_file(&format!("{}/shell/role", base), "toplevel\n");
    ensure_file(&format!("{}/shell/title", base), "");
    ensure_file(&format!("{}/shell/app_id", base), "");
    ensure_file(&format!("{}/shell/parent", base), "");
    ensure_file(&format!("{}/shell/requested/maximize", base), "0\n");
    ensure_file(&format!("{}/shell/requested/fullscreen", base), "0\n");
    ensure_file(&format!("{}/shell/requested/minimize", base), "0\n");
    ensure_file(&format!("{}/shell/current/x", base), "");
    ensure_file(&format!("{}/shell/current/y", base), "");
    ensure_file(&format!("{}/shell/current/width", base), "");
    ensure_file(&format!("{}/shell/current/height", base), "");
    ensure_file(&format!("{}/shell/current/z", base), "");
    ensure_file(&format!("{}/shell/current/activated", base), "0\n");
    ensure_file(&format!("{}/shell/current/maximized", base), "0\n");
    ensure_file(&format!("{}/shell/current/fullscreen", base), "0\n");
    ensure_file(&format!("{}/shell/current/resizing", base), "0\n");
    ensure_file(&format!("{}/bind/surface", base), "");
    ensure_file(&format!("{}/events", base), "");
    ensure_file(&format!("{}/status/mapped", base), "0\n");
    ensure_file(&format!("{}/status/focused", base), "0\n");
    ensure_file(&format!("{}/status/last_configure_serial", base), "0\n");
    ensure_file(&format!("{}/status/client_pid", base), "0\n");
    ensure_file(&format!("{}/status/closing", base), "0\n");
}

pub fn ensure_surface_tree(id: &str) {
    let base = surface_path(id);
    let _ = vfs_mkdir(&base);
    let _ = vfs_mkdir(&format!("{}/status", base));

    ensure_file(&format!("{}/attach", base), "");
    ensure_file(&format!("{}/damage", base), "");
    ensure_file(&format!("{}/commit", base), "0\n");
    ensure_file(&format!("{}/input_region", base), "");
    ensure_file(&format!("{}/opaque_region", base), "");
    ensure_file(&format!("{}/status/mapped", base), "0\n");
    ensure_file(&format!("{}/status/last_commit", base), "0\n");
    ensure_file(&format!("{}/status/configured_serial", base), "0\n");
    ensure_file(&format!("{}/status/width", base), "0\n");
    ensure_file(&format!("{}/status/height", base), "0\n");
    ensure_file(&format!("{}/status/buffer_attached", base), "0\n");
}

pub fn window_path(id: &str) -> String {
    format!("{}/{}", WINDOWS_ROOT, id)
}

pub fn surface_path(id: &str) -> String {
    format!("{}/{}", SURFACES_ROOT, id)
}

pub fn scene_id_from_name(name: &str) -> ThingId {
    let raw = if let Some(parsed) = parse_u64(name) {
        parsed
    } else {
        stable_name_hash(name)
    };
    let mut bytes = [0u8; 16];
    bytes[..8].copy_from_slice(&raw.to_le_bytes());
    ThingId(bytes)
}

pub fn list_dir(path: &str) -> Vec<String> {
    let fd = match vfs_open(path, O_RDONLY) {
        Ok(fd) => fd,
        Err(_) => return Vec::new(),
    };
    let mut buf = [0u8; 4096];
    let n = match vfs_readdir(fd, &mut buf) {
        Ok(n) => n,
        Err(_) => {
            let _ = vfs_close(fd);
            return Vec::new();
        }
    };
    let _ = vfs_close(fd);

    let mut out = Vec::new();
    let mut offset = 0usize;
    while offset < n {
        let mut end = offset;
        while end < n && buf[end] != 0 {
            end += 1;
        }
        if end > offset {
            if let Ok(name) = core::str::from_utf8(&buf[offset..end]) {
                if name != "." && name != ".." {
                    out.push(name.to_string());
                }
            }
        }
        offset = end.saturating_add(1);
    }
    out
}

pub fn read_text(path: &str) -> Option<String> {
    let fd = vfs_open(path, O_RDONLY).ok()?;
    let size = vfs_stat(fd).ok()?.1 as usize;
    let mut data = Vec::new();
    if size > 0 {
        data.resize(size, 0);
        let n = vfs_read(fd, &mut data).ok()?;
        data.truncate(n);
    }
    let _ = vfs_close(fd);
    Some(String::from_utf8_lossy(&data).trim().to_string())
}

pub fn write_text(path: &str, text: &str) -> Result<(), Errno> {
    let fd = vfs_open(path, O_WRONLY | O_CREAT | O_TRUNC).map_err(|e| e)?;
    if !text.is_empty() {
        let _ = vfs_write(fd, text.as_bytes())?;
    }
    let _ = vfs_close(fd);
    Ok(())
}

pub fn append_line(path: &str, line: &str) -> Result<(), Errno> {
    let fd = vfs_open(path, O_WRONLY | O_CREAT).map_err(|e| e)?;
    let (_, size, _) = vfs_stat(fd)?;
    let _ = vfs_seek(fd, size as i64, 0)?;
    let _ = vfs_write(fd, line.as_bytes())?;
    let _ = vfs_close(fd);
    Ok(())
}

pub fn read_u64(path: &str) -> Option<u64> {
    parse_u64(&read_text(path)?)
}

pub fn read_bool(path: &str) -> bool {
    matches!(
        read_text(path).as_deref(),
        Some("1") | Some("true") | Some("yes")
    )
}

pub fn parse_attach_payload(text: &str) -> Option<AttachedBuffer> {
    let mut fd = None;
    let mut width = None;
    let mut height = None;
    let mut stride = None;
    let mut format = Some(1u32);

    for line in text.lines() {
        let (key, value) = line.split_once('=')?;
        let value = value.trim();
        match key.trim() {
            "fd" => fd = parse_u64(value).map(|v| v as u32),
            "width" => width = parse_u64(value).map(|v| v as u32),
            "height" => height = parse_u64(value).map(|v| v as u32),
            "stride" => stride = parse_u64(value).map(|v| v as u32),
            "format" => format = parse_u64(value).map(|v| v as u32),
            _ => {}
        }
    }

    let fd = fd?;
    let width = width?;
    let height = height?;
    let stride = stride.unwrap_or(width.saturating_mul(4));
    let format = format.unwrap_or(1);
    if width == 0 || height == 0 || stride == 0 {
        return None;
    }
    Some(AttachedBuffer {
        fd,
        width,
        height,
        stride,
        format,
    })
}

pub fn encode_configure_event(serial: u64, width: i32, height: i32, states: &[&str]) -> String {
    let mut out = format!(
        "{{\"type\":\"configure\",\"serial\":{},\"width\":{},\"height\":{},\"states\":[",
        serial,
        width.max(0),
        height.max(0)
    );
    for (idx, state) in states.iter().enumerate() {
        if idx > 0 {
            out.push(',');
        }
        out.push('"');
        out.push_str(state);
        out.push('"');
    }
    out.push_str("]}\n");
    out
}

pub fn encode_close_event() -> String {
    "{\"type\":\"close\"}\n".to_string()
}

pub fn pointer_events_path() -> String {
    format!("{}/events", POINTER_ROOT)
}

pub fn pointer_state_path(name: &str) -> String {
    format!("{}/{}", POINTER_STATE_ROOT, name)
}

pub fn encode_pointer_enter_event(surface_id: u64, sx_fp16: i32, sy_fp16: i32) -> String {
    format!(
        "{{\"type\":\"enter\",\"surface_id\":{},\"sx\":{},\"sy\":{}}}\n",
        surface_id, sx_fp16, sy_fp16
    )
}

pub fn encode_pointer_leave_event(surface_id: u64) -> String {
    format!("{{\"type\":\"leave\",\"surface_id\":{}}}\n", surface_id)
}

pub fn encode_pointer_motion_event(
    x_fp16: i32,
    y_fp16: i32,
    sx_fp16: i32,
    sy_fp16: i32,
    surface_id: Option<u64>,
) -> String {
    match surface_id {
        Some(surface_id) => format!(
            "{{\"type\":\"motion\",\"x\":{},\"y\":{},\"sx\":{},\"sy\":{},\"surface_id\":{}}}\n",
            x_fp16, y_fp16, sx_fp16, sy_fp16, surface_id
        ),
        None => format!(
            "{{\"type\":\"motion\",\"x\":{},\"y\":{},\"sx\":{},\"sy\":{}}}\n",
            x_fp16, y_fp16, sx_fp16, sy_fp16
        ),
    }
}

pub fn encode_pointer_button_event(
    button: u8,
    pressed: bool,
    x_fp16: i32,
    y_fp16: i32,
    sx_fp16: i32,
    sy_fp16: i32,
    surface_id: Option<u64>,
) -> String {
    match surface_id {
        Some(surface_id) => format!(
            "{{\"type\":\"button\",\"button\":{},\"pressed\":{},\"x\":{},\"y\":{},\"sx\":{},\"sy\":{},\"surface_id\":{}}}\n",
            button,
            if pressed { "true" } else { "false" },
            x_fp16,
            y_fp16,
            sx_fp16,
            sy_fp16,
            surface_id
        ),
        None => format!(
            "{{\"type\":\"button\",\"button\":{},\"pressed\":{},\"x\":{},\"y\":{},\"sx\":{},\"sy\":{}}}\n",
            button,
            if pressed { "true" } else { "false" },
            x_fp16,
            y_fp16,
            sx_fp16,
            sy_fp16
        ),
    }
}

pub fn encode_pointer_frame_event() -> String {
    "{\"type\":\"frame\"}\n".to_string()
}

pub fn logical_to_fixed_16_16(value: i32) -> i32 {
    value.saturating_mul(1 << 16)
}

fn ensure_file(path: &str, default_text: &str) {
    if vfs_open(path, O_RDONLY).is_ok() {
        return;
    }
    let _ = write_text(path, default_text);
}

fn parse_u64(text: &str) -> Option<u64> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return None;
    }
    if let Some(hex) = trimmed.strip_prefix("0x") {
        u64::from_str_radix(hex, 16).ok()
    } else {
        trimmed.parse::<u64>().ok()
    }
}

fn stable_name_hash(name: &str) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for byte in name.as_bytes() {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(0x100_0000_01b3);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attach_payload_requires_dimensions_and_fd() {
        let payload = "fd=17\nwidth=320\nheight=200\nstride=1280\nformat=1\n";
        let parsed = parse_attach_payload(payload).expect("payload");
        assert_eq!(parsed.fd, 17);
        assert_eq!(parsed.width, 320);
        assert_eq!(parsed.height, 200);
        assert_eq!(parsed.stride, 1280);
        assert_eq!(parsed.format, 1);
    }

    #[test]
    fn configure_event_is_ndjson() {
        let line = encode_configure_event(42, 800, 600, &["activated", "resizing"]);
        assert!(line.ends_with('\n'));
        assert!(line.contains("\"serial\":42"));
        assert!(line.contains("\"activated\""));
        assert!(line.contains("\"resizing\""));
    }

    #[test]
    fn pointer_motion_event_uses_fixed_point_fields() {
        let line = encode_pointer_motion_event(1 << 16, 2 << 16, 3 << 16, 4 << 16, Some(7));
        assert!(line.ends_with('\n'));
        assert!(line.contains("\"type\":\"motion\""));
        assert!(line.contains("\"x\":65536"));
        assert!(line.contains("\"surface_id\":7"));
    }

    #[test]
    fn fixed_point_helper_scales_integers() {
        assert_eq!(logical_to_fixed_16_16(12), 12 << 16);
        assert_eq!(logical_to_fixed_16_16(-3), -3 << 16);
    }

    #[test]
    fn scene_id_is_stable_for_names() {
        let a = scene_id_from_name("demo");
        let b = scene_id_from_name("demo");
        assert_eq!(a, b);
    }
}
