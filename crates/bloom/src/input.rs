pub struct PointerInput {
    ring_ptr: *const u8,
    capacity: u32,
    read_seq: u64,
    pub x: i32,
    pub y: i32,
    pub buttons: u16,
    pub screen_w: u32,
    pub screen_h: u32,
}

impl PointerInput {
    pub fn new(ring_ptr: *const u8, capacity: u32, screen_w: u32, screen_h: u32) -> Self {
        Self {
            ring_ptr,
            capacity,
            read_seq: 0,
            x: (screen_w / 2) as i32,
            y: (screen_h / 2) as i32,
            buttons: 0,
            screen_w,
            screen_h,
        }
    }

    pub fn poll(&mut self) -> (i32, i32, u16) {
        unsafe {
             if self.ring_ptr.is_null() {
                 return (self.x, self.y, self.buttons);
             }

            const HEADER_SIZE: usize = 48;
            const RECORD_HEADER_SIZE: usize = 24; 
            const EV_POINTER_DELTA: u16 = 1;

            let magic = u32::from_le_bytes([
                *self.ring_ptr, *self.ring_ptr.add(1), *self.ring_ptr.add(2), *self.ring_ptr.add(3),
            ]);
            if magic != 0x544E5645 {
                return (self.x, self.y, self.buttons);
            } 

            // capacity is passed in new(), but strictly it's in header too.
            // We trust the passed capacity matches mapping size?
            // Actually the ring header has capacity (offset 8).
            // Code in app.rs read it from header.
            // Let's read from header to be safe or consistent.
            let header_capacity = u32::from_le_bytes([
                *self.ring_ptr.add(8), *self.ring_ptr.add(9), *self.ring_ptr.add(10), *self.ring_ptr.add(11),
            ]);
            let capacity = core::cmp::min(header_capacity, self.capacity);

            let ring_base = self.ring_ptr.add(HEADER_SIZE);

            let mut offset: u32 = 0;
            let max_iters = capacity / 32;

            for _ in 0..max_iters {
                if (offset + (HEADER_SIZE as u32) + (RECORD_HEADER_SIZE as u32)) > capacity {
                    break;
                }

                let rec_ptr = ring_base.add(offset as usize);

                let len = u16::from_le_bytes([*rec_ptr, *rec_ptr.add(1)]);
                if len == 0 || len < RECORD_HEADER_SIZE as u16 {
                    break;
                }
                if (offset + (HEADER_SIZE as u32) + (len as u32)) > capacity {
                    break;
                }

                let kind = u16::from_le_bytes([*rec_ptr.add(2), *rec_ptr.add(3)]);

                let seq = u64::from_le_bytes([
                    *rec_ptr.add(8), *rec_ptr.add(9), *rec_ptr.add(10), *rec_ptr.add(11),
                    *rec_ptr.add(12), *rec_ptr.add(13), *rec_ptr.add(14), *rec_ptr.add(15),
                ]);

                if seq > self.read_seq && kind == EV_POINTER_DELTA {
                    let payload_ptr = rec_ptr.add(RECORD_HEADER_SIZE);
                    let dx = i16::from_le_bytes([*payload_ptr, *payload_ptr.add(1)]);
                    let dy = i16::from_le_bytes([*payload_ptr.add(2), *payload_ptr.add(3)]);
                    let buttons = u16::from_le_bytes([*payload_ptr.add(4), *payload_ptr.add(5)]);

                    self.x = (self.x + (dx as i32)).clamp(0, self.screen_w as i32 - 1);
                    self.y = (self.y + (dy as i32)).clamp(0, self.screen_h as i32 - 1);
                    self.buttons = buttons;

                    self.read_seq = seq;
                }

                offset += ((len as u32) + 7) & !7;
            }
        }
        (self.x, self.y, self.buttons)
    }
}
