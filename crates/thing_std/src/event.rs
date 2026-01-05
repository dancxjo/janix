//! EventStream reader/writer.
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
pub struct EventStreamReader {
    base: *const u8,
    capacity: u32,
    last_seq: u64,
}

impl EventStreamReader {
    pub fn new(ptr: *const u8) -> Option<Self> {
        if ptr.is_null() { return None; }
        let magic = unsafe { u32::from_le_bytes([*ptr, *ptr.add(1), *ptr.add(2), *ptr.add(3)]) };
        if magic != EVENT_STREAM_MAGIC { return None; }
        let capacity = unsafe { u32::from_le_bytes([*ptr.add(8), *ptr.add(9), *ptr.add(10), *ptr.add(11)]) };
        Some(Self { base: ptr, capacity, last_seq: 0 })
    }

    fn current_write_seq(&self) -> u64 {
        unsafe { (*(self.base as *const EventStreamHeader)).write_seq.load(Ordering::Acquire) }
    }

    pub fn dropped(&self) -> u64 {
        unsafe { (*(self.base as *const EventStreamHeader)).dropped.load(Ordering::Relaxed) }
    }

    pub fn has_pending(&self) -> bool {
        self.current_write_seq() > self.last_seq
    }

    pub fn poll(&mut self) -> Option<EventRecordView<'_>> {
        let write_seq = self.current_write_seq();
        if self.last_seq >= write_seq { return None; }

        let ring_base = unsafe { self.base.add(HEADER_SIZE) };
        let target_seq = self.last_seq + 1;
        let mut offset: u32 = 0;
        let max_records = self.capacity / (RECORD_HEADER_SIZE as u32);

        for _ in 0..max_records {
            if offset + (RECORD_HEADER_SIZE as u32) > self.capacity { break; }
            let record_ptr = unsafe { ring_base.add(offset as usize) };
            
            let len = unsafe { u16::from_le_bytes([*record_ptr, *record_ptr.add(1)]) };
            if len == 0 || len < RECORD_HEADER_SIZE as u16 { break; }

            let seq = unsafe {
                u64::from_le_bytes([
                    *record_ptr.add(8), *record_ptr.add(9), *record_ptr.add(10), *record_ptr.add(11),
                    *record_ptr.add(12), *record_ptr.add(13), *record_ptr.add(14), *record_ptr.add(15),
                ])
            };

            if seq == target_seq {
                let kind = unsafe { u16::from_le_bytes([*record_ptr.add(2), *record_ptr.add(3)]) };
                let flags = unsafe { u16::from_le_bytes([*record_ptr.add(4), *record_ptr.add(5)]) };
                let ts_mono = unsafe {
                    u64::from_le_bytes([
                        *record_ptr.add(16), *record_ptr.add(17), *record_ptr.add(18), *record_ptr.add(19),
                        *record_ptr.add(20), *record_ptr.add(21), *record_ptr.add(22), *record_ptr.add(23),
                    ])
                };
                let payload_len = (len as usize).saturating_sub(RECORD_HEADER_SIZE);
                let payload = if payload_len > 0 {
                    unsafe { core::slice::from_raw_parts(record_ptr.add(RECORD_HEADER_SIZE), payload_len) }
                } else { &[] };
                self.last_seq = target_seq;
                return Some(EventRecordView { kind, flags, seq, ts_mono, payload });
            }

            offset += ((len as u32) + 7) & !7;
            if offset >= self.capacity { offset = 0; }
        }
        None
    }

    pub fn reset_to_current(&mut self) { self.last_seq = self.current_write_seq(); }
    pub fn last_seq(&self) -> u64 { self.last_seq }
}

// Legacy stub
pub fn decode_event(_id: abi::ids::ThingId) -> Option<()> { None }

/// Writer for an EventStream bytespace.
pub struct EventStreamWriter {
    base_ptr: *mut u8,
    capacity: u32,
}

impl EventStreamWriter {
    pub unsafe fn new(ptr: *mut u8, capacity: u32) -> Self {
        Self { base_ptr: ptr, capacity }
    }

    pub fn push(&mut self, kind: u16, flags: u16, timestamp: u64, payload: &[u8]) {
        unsafe {
            let header_ptr = self.base_ptr as *mut EventStreamHeader;
            let payload_len = payload.len();
            let total_len = abi::events::record_size(payload_len);
            
            let current_seq = (*header_ptr).write_seq.load(Ordering::Relaxed);
            let current_off = (*header_ptr).write_off.load(Ordering::Relaxed) as usize;
            
            let mut next_off = current_off + total_len;
            let record_offset = if next_off > self.capacity as usize {
                next_off = total_len;
                0
            } else { current_off };

            let ring_base = self.base_ptr.add(HEADER_SIZE);
            let record_dst = ring_base.add(record_offset) as *mut abi::events::EventRecord;
            
            if payload_len > 0 {
                let payload_dst = (record_dst as *mut u8).add(RECORD_HEADER_SIZE);
                core::ptr::copy_nonoverlapping(payload.as_ptr(), payload_dst, payload_len);
            }
            
            let record = abi::events::EventRecord {
                len: total_len as u16,
                kind,
                flags,
                reserved: 0,
                seq: current_seq + 1,
                ts_mono: timestamp,
            };
            
            core::ptr::write_volatile(record_dst, record);
            
            (*header_ptr).write_off.store(next_off as u32, Ordering::Relaxed);
            (*header_ptr).write_seq.store(current_seq + 1, Ordering::Release);
        }
    }
}
