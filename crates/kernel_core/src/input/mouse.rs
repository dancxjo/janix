pub struct MouseState {
    pub x: i32,
    pub y: i32,
    pub buttons: u8,
    pub screen_width: i32,
    pub screen_height: i32,
}

static mut MOUSE_STATE: MouseState = MouseState { x: 0, y: 0, buttons: 0, screen_width: 1024, screen_height: 768 };

// Simple packet buffer
static mut PACKET: [u8; 3] = [0; 3];
static mut PACKET_IDX: usize = 0;

pub fn update_packet(byte: u8) {
    unsafe {
        PACKET[PACKET_IDX] = byte;
        PACKET_IDX += 1;
        if PACKET_IDX == 3 {
            process_packet();
            PACKET_IDX = 0;
        }
    }
}

unsafe fn process_packet() {
    let flags = PACKET[0];
    let x_raw = PACKET[1];
    let y_raw = PACKET[2];

    if (flags & 0x08) == 0 {
        // Bit 3 should be 1. Sync error?
        return;
    }

    let x_sign = (flags & 0x10) != 0;
    let y_sign = (flags & 0x20) != 0;

    let mut dx: i32 = x_raw as i32;
    let mut dy: i32 = y_raw as i32;

    if x_sign { dx |= !0xFF; }
    if y_sign { dy |= !0xFF; }

    // Invert Y for screen coords
    dy = -dy;

    MOUSE_STATE.x += dx;
    MOUSE_STATE.y += dy;
    MOUSE_STATE.buttons = flags & 0x07;

    // Clamp
    if MOUSE_STATE.x < 0 { MOUSE_STATE.x = 0; }
    if MOUSE_STATE.y < 0 { MOUSE_STATE.y = 0; }
    if MOUSE_STATE.x >= MOUSE_STATE.screen_width { MOUSE_STATE.x = MOUSE_STATE.screen_width - 1; }
    if MOUSE_STATE.y >= MOUSE_STATE.screen_height { MOUSE_STATE.y = MOUSE_STATE.screen_height - 1; }

    // Trigger cursor update
    crate::graph::cursor::update_cursor(MOUSE_STATE.x, MOUSE_STATE.y);
}

pub fn set_screen_size(w: i32, h: i32) {
    unsafe {
        MOUSE_STATE.screen_width = w;
        MOUSE_STATE.screen_height = h;
        // Center initial
        if MOUSE_STATE.x == 0 && MOUSE_STATE.y == 0 {
            MOUSE_STATE.x = w / 2;
            MOUSE_STATE.y = h / 2;
        }
    }
}
