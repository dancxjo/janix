use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct Message {
    pub object_id: u32,
    pub opcode: u16,
    pub args: Vec<u32>, // A simplified arg payload since everything is 32-bit aligned
    pub bytes: Vec<u8>, // Raw string/varlen bytes
}

pub fn encode_header(object_id: u32, opcode: u16, size: u16, buf: &mut [u8]) {
    buf[0..4].copy_from_slice(&object_id.to_ne_bytes());
    let size_op = ((size as u32) << 16) | (opcode as u32);
    buf[4..8].copy_from_slice(&size_op.to_ne_bytes());
}

pub fn decode_header(buf: &[u8]) -> (u32, u16, u16) {
    let mut b = [0u8; 4];
    b.copy_from_slice(&buf[0..4]);
    let object_id = u32::from_ne_bytes(b);
    b.copy_from_slice(&buf[4..8]);
    let size_op = u32::from_ne_bytes(b);
    let size = (size_op >> 16) as u16;
    let opcode = (size_op & 0xFFFF) as u16;
    (object_id, opcode, size)
}

pub struct MessageBuilder {
    buf: Vec<u8>,
}

impl MessageBuilder {
    pub fn new(object_id: u32, opcode: u16) -> Self {
        let mut buf = Vec::new();
        buf.resize(8, 0); // Reserve space for header

        let mut b = Self { buf };
        encode_header(object_id, opcode, 8, &mut b.buf);
        b
    }

    pub fn push_u32(&mut self, val: u32) {
        self.buf.extend_from_slice(&val.to_ne_bytes());
        self.update_header();
    }

    pub fn push_i32(&mut self, val: i32) {
        self.buf.extend_from_slice(&val.to_ne_bytes());
        self.update_header();
    }

    pub fn push_array(&mut self, bytes: &[u8]) {
        self.push_u32(bytes.len() as u32);
        self.buf.extend_from_slice(bytes);
        let padding = (4 - (bytes.len() % 4)) % 4;
        for _ in 0..padding {
            self.buf.push(0);
        }
        self.update_header();
    }

    pub fn push_string(&mut self, val: &str) {
        let len = val.len() as u32 + 1; // +1 for null terminator
        self.push_u32(len);
        let mut bytes = val.as_bytes().to_vec();
        bytes.push(0);
        let padding = (4 - (bytes.len() % 4)) % 4;
        for _ in 0..padding {
            bytes.push(0);
        }
        self.buf.extend_from_slice(&bytes);
        self.update_header();
    }

    fn update_header(&mut self) {
        let size = self.buf.len();
        let mut b = [0u8; 4];
        b.copy_from_slice(&self.buf[4..8]);
        let size_op = u32::from_ne_bytes(b);
        let opcode = (size_op & 0xFFFF) as u16;
        encode_header(
            u32::from_ne_bytes(self.buf[0..4].try_into().unwrap()),
            opcode,
            size as u16,
            &mut self.buf,
        );
    }

    pub fn build(self) -> Vec<u8> {
        self.buf
    }
}
