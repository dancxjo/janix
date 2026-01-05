//! EventStream reader for userspace consumers.
//!
//! Provides lock-free iteration over event records from a mapped EventStream bytespace.

use abi::events::{EventStreamHeader, HEADER_SIZE, RECORD_HEADER_SIZE, EVENT_STREAM_MAGIC};
use core::sync::atomic::Ordering;

/// A view into a single event record
pub struct EventRecordView<'a> {
    pub kind: u16,
    pub flags: u16,
    pub seq: u64,
    pub ts_mono: u64,
    pub payload: &'a [u8],
}

/// Reader for an EventStream bytespace.
///
/// Tracks the last consumed sequence number and iterates over new records.
pub struct EventStreamReader {
    base: *const u8,
    capacity: u32,
    last_seq: u64,
}

impl EventStreamReader {
    /// Create a new reader from a mapped bytespace pointer.
    ///
    /// Returns `None` if the magic number is invalid.
    pub fn new(ptr: *const u8) -> Option<Self> {
        if ptr.is_null() {
            return None;
        }

        let magic = unsafe {
            u32::from_le_bytes([*ptr, *ptr.add(1), *ptr.add(2), *ptr.add(3)])
        };

        if magic != EVENT_STREAM_MAGIC {
            return None;
        }

        let capacity = unsafe {
            u32::from_le_bytes([*ptr.add(8), *ptr.add(9), *ptr.add(10), *ptr.add(11)])
        };

        Some(Self { base: ptr, capacity, last_seq: 0 })
    }

    fn current_write_seq(&self) -> u64 {
        unsafe {
            let header = self.base as *const EventStreamHeader;
            (*header).write_seq.load(Ordering::Acquire)
        }
    }

    /// Get the number of dropped records
    pub fn dropped(&self) -> u64 {
        unsafe {
            let header = self.base as *const EventStreamHeader;
            (*header).dropped.load(Ordering::Relaxed)
        }
    }

    /// Check if there are new records available
    pub fn has_pending(&self) -> bool {
        self.current_write_seq() > self.last_seq
    }

    /// Poll for the next record since last_seq.
    pub fn poll(&mut self) -> Option<EventRecordView<'_>> {
        let write_seq = self.current_write_seq();
        if self.last_seq >= write_seq {
            return None;
        }

        let ring_base = unsafe { self.base.add(HEADER_SIZE) };
        let target_seq = self.last_seq + 1;

        let mut offset: u32 = 0;
        let max_records = self.capacity / (RECORD_HEADER_SIZE as u32);

        for _ in 0..max_records {
            if offset + (RECORD_HEADER_SIZE as u32) > self.capacity {
                break;
            }

            let record_ptr = unsafe { ring_base.add(offset as usize) };

            let len = unsafe { u16::from_le_bytes([*record_ptr, *record_ptr.add(1)]) };
            if len == 0 || len < RECORD_HEADER_SIZE as u16 {
                break;
            }

            let kind = unsafe { u16::from_le_bytes([*record_ptr.add(2), *record_ptr.add(3)]) };
            let flags = unsafe { u16::from_le_bytes([*record_ptr.add(4), *record_ptr.add(5)]) };
            let seq = unsafe {
                u64::from_le_bytes([
                    *record_ptr.add(8), *record_ptr.add(9), *record_ptr.add(10), *record_ptr.add(11),
                    *record_ptr.add(12), *record_ptr.add(13), *record_ptr.add(14), *record_ptr.add(15),
                ])
            };
            let ts_mono = unsafe {
                u64::from_le_bytes([
                    *record_ptr.add(16), *record_ptr.add(17), *record_ptr.add(18), *record_ptr.add(19),
                    *record_ptr.add(20), *record_ptr.add(21), *record_ptr.add(22), *record_ptr.add(23),
                ])
            };

            if seq == target_seq {
                let payload_len = (len as usize).saturating_sub(RECORD_HEADER_SIZE);
                let payload = if payload_len > 0 {
                    unsafe { core::slice::from_raw_parts(record_ptr.add(RECORD_HEADER_SIZE), payload_len) }
                } else {
                    &[]
                };
                self.last_seq = target_seq;
                return Some(EventRecordView { kind, flags, seq, ts_mono, payload });
            }

            offset += ((len as u32) + 7) & !7;
            if offset >= self.capacity {
                offset = 0;
            }
        }

        None
    }

    /// Reset to current write position (useful after overruns)
    pub fn reset_to_current(&mut self) {
        self.last_seq = self.current_write_seq();
    }

    /// Get the last consumed sequence number
    pub fn last_seq(&self) -> u64 {
        self.last_seq
    }
}

// Legacy stub
pub fn decode_event(_id: abi::ids::ThingId) -> Option<()> {
    None
}
