#[cfg(test)]
mod tests {
    use super::super::ring::{LogRing, EntryKind};

    // Minimal test for ring behavior

    #[test]
    fn test_ring_push_drain() {
        let ring = LogRing::global();
        ring.push(EntryKind::Log, 2, "Test Message", 1, 2, 3, 4);

        // Push a second one
        ring.push(EntryKind::Error, 4, "Error Msg", 5, 6, 7, 8);

        let mut count = 0;
        let drained = ring.drain(|entry| {
            count += 1;
            if count == 1 {
                assert_eq!(entry.level, 2);
                assert_eq!(entry.payload_a, 1);
                 let msg = unsafe {
                    let len = entry.msg_len as usize;
                    core::str::from_utf8(&entry.msg_bytes[..len]).unwrap()
                };
                assert_eq!(msg, "Test Message");
            } else if count == 2 {
                assert_eq!(entry.level, 4);
                let msg = unsafe {
                    let len = entry.msg_len as usize;
                    core::str::from_utf8(&entry.msg_bytes[..len]).unwrap()
                };
                assert_eq!(msg, "Error Msg");
            }
        });

        assert_eq!(drained, 2);
        assert_eq!(count, 2);
    }
}
