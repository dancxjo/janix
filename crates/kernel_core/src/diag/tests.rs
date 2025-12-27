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
            true
        });

        assert_eq!(drained, 2);
        assert_eq!(count, 2);
    }

    #[test]
    fn test_ring_drain_backpressure() {
        let ring = LogRing::global();
        // clear ring
        ring.drain(|_| true);

        ring.push(EntryKind::Log, 1, "Msg 1", 0,0,0,0);
        ring.push(EntryKind::Log, 2, "Msg 2", 0,0,0,0);
        ring.push(EntryKind::Log, 3, "Msg 3", 0,0,0,0);

        let mut count = 0;
        let drained = ring.drain(|entry| {
            count += 1;
            // Stop after 2
            if count >= 2 {
                return false;
            }
            true
        });

        assert_eq!(drained, 1, "Should have consumed 1 (msg1), stopped on msg2");
        assert_eq!(count, 2, "Should have inspected 2 (msg1, msg2)");

        // Drain remainder
        let mut count2 = 0;
        let drained2 = ring.drain(|entry| {
            count2 += 1;
            let msg = unsafe {
                    let len = entry.msg_len as usize;
                    core::str::from_utf8(&entry.msg_bytes[..len]).unwrap()
            };
            if count2 == 1 { assert_eq!(msg, "Msg 2"); }
            if count2 == 2 { assert_eq!(msg, "Msg 3"); }
            true
        });

        assert_eq!(drained2, 2, "Should drain remaining 2");
    }
}
