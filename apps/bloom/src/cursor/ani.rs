//! .ani (Animated Cursor) file parser

extern crate alloc;
use super::cur::load_cur;
use super::{CursorAsset, CursorFrame};
use alloc::vec::Vec;

const JIFFY_TO_MS: u32 = 17;

/// Parse a .ani file from raw bytes
pub fn load_ani(data: &[u8]) -> Option<CursorAsset> {
    if data.len() < 12 {
        return None;
    }
    if &data[0..4] != b"RIFF" {
        return None;
    }
    if &data[8..12] != b"ACON" {
        return None;
    }

    let mut parser = RiffParser::new(&data[12..]);
    let mut default_rate = 10u32;
    let mut rates: Vec<u32> = Vec::new();
    let mut sequence: Vec<u32> = Vec::new();
    let mut frames: Vec<CursorFrame> = Vec::new();

    while let Some((id, chunk_data)) = parser.next_chunk() {
        match &id {
            b"anih" => {
                if chunk_data.len() >= 36 {
                    default_rate = u32::from_le_bytes([
                        chunk_data[24],
                        chunk_data[25],
                        chunk_data[26],
                        chunk_data[27],
                    ]);
                }
            }
            b"rate" => {
                let count = chunk_data.len() / 4;
                for i in 0..count {
                    let offset = i * 4;
                    if offset + 4 <= chunk_data.len() {
                        let jiffies = u32::from_le_bytes([
                            chunk_data[offset],
                            chunk_data[offset + 1],
                            chunk_data[offset + 2],
                            chunk_data[offset + 3],
                        ]);
                        rates.push(jiffies * JIFFY_TO_MS);
                    }
                }
            }
            b"seq " => {
                let count = chunk_data.len() / 4;
                for i in 0..count {
                    let offset = i * 4;
                    if offset + 4 <= chunk_data.len() {
                        let idx = u32::from_le_bytes([
                            chunk_data[offset],
                            chunk_data[offset + 1],
                            chunk_data[offset + 2],
                            chunk_data[offset + 3],
                        ]);
                        sequence.push(idx);
                    }
                }
            }
            b"LIST" => {
                if chunk_data.len() >= 4 && &chunk_data[0..4] == b"fram" {
                    let mut list_parser = RiffParser::new(&chunk_data[4..]);
                    while let Some((icon_id, icon_data)) = list_parser.next_chunk() {
                        if &icon_id == b"icon" {
                            if let Some(frame) = load_cur(icon_data) {
                                frames.push(frame);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    if frames.is_empty() {
        return None;
    }

    let delays_ms = if rates.is_empty() {
        let default_ms = default_rate * JIFFY_TO_MS;
        alloc::vec![default_ms; frames.len()]
    } else {
        rates
    };

    Some(CursorAsset {
        frames,
        delays_ms,
        sequence,
        is_animated: true,
    })
}

struct RiffParser<'a> {
    data: &'a [u8],
    offset: usize,
}

impl<'a> RiffParser<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data, offset: 0 }
    }

    fn next_chunk(&mut self) -> Option<([u8; 4], &'a [u8])> {
        if self.offset + 8 > self.data.len() {
            return None;
        }
        let mut id = [0u8; 4];
        id.copy_from_slice(&self.data[self.offset..self.offset + 4]);
        let size = u32::from_le_bytes([
            self.data[self.offset + 4],
            self.data[self.offset + 5],
            self.data[self.offset + 6],
            self.data[self.offset + 7],
        ]) as usize;
        let data_start = self.offset + 8;
        let data_end = (data_start + size).min(self.data.len());
        self.offset = (data_end + 1) & !1;
        Some((id, &self.data[data_start..data_end]))
    }
}
