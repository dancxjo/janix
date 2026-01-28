use abi::display_driver_protocol as drvproto;
use abi::driver_frame::FrameReader;

#[derive(Clone, Copy)]
struct DriverConfig {
    caps: u32,
    max_rects: u16,
    split_writes: bool,
    burst: bool,
}

struct Link {
    client_rx: FrameReader<4096>,
    driver_rx: FrameReader<4096>,
}

impl Link {
    fn new() -> Self {
        Self {
            client_rx: FrameReader::new(),
            driver_rx: FrameReader::new(),
        }
    }

    fn client_send(&mut self, data: &[u8]) {
        self.driver_rx.push(data);
    }

    fn driver_send(&mut self, data: &[u8]) {
        self.client_rx.push(data);
    }
}

struct DriverSim {
    cfg: DriverConfig,
    seen_hello: bool,
    seen_present: Option<drvproto::PresentHeader>,
}

impl DriverSim {
    fn new(cfg: DriverConfig) -> Self {
        Self {
            cfg,
            seen_hello: false,
            seen_present: None,
        }
    }

    fn pump(&mut self, link: &mut Link) {
        while let Some((header, payload)) = link.driver_rx.next_message() {
            match header.msg_type {
                drvproto::MSG_HELLO => {
                    self.seen_hello = true;
                    let want_caps = drvproto::decode_hello_payload_le(payload)
                        .map(|hello| hello.want_caps)
                        .unwrap_or(0);
                    let welcome = drvproto::WelcomePayload {
                        proto_major: drvproto::PROTO_MAJOR,
                        proto_minor: drvproto::PROTO_MINOR,
                        have_caps: self.cfg.caps & want_caps,
                        max_rects: self.cfg.max_rects,
                        reserved: 0,
                    };
                    let mut welcome_bytes = [0u8; drvproto::WELCOME_PAYLOAD_WIRE_SIZE];
                    let len = drvproto::encode_welcome_payload_le(&welcome, &mut welcome_bytes).unwrap();
                    let mut msg_buf = [0u8; 64];
                    let msg_len = drvproto::encode_message(&mut msg_buf, drvproto::MSG_WELCOME, &welcome_bytes[..len]).unwrap();
                    if self.cfg.split_writes && msg_len >= 3 {
                        let part = msg_len / 3;
                        let mut offset = 0;
                        for i in 0..3 {
                            let remaining = msg_len - offset;
                            let chunk = if i == 2 { remaining } else { part.max(1) };
                            link.driver_send(&msg_buf[offset..offset + chunk]);
                            offset += chunk;
                        }
                    } else {
                        link.driver_send(&msg_buf[..msg_len]);
                    }
                }
                drvproto::MSG_BIND => {
                    let mut ack_buf = [0u8; 64];
                    let ack_len = drvproto::encode_message(&mut ack_buf, drvproto::MSG_ACK, &[]).unwrap();
                    if self.cfg.burst {
                        let ack2_len = drvproto::encode_message(&mut ack_buf[ack_len..], drvproto::MSG_ACK, &[]).unwrap();
                        link.driver_send(&ack_buf[..ack_len + ack2_len]);
                    } else {
                        link.driver_send(&ack_buf[..ack_len]);
                    }
                }
                drvproto::MSG_PRESENT => {
                    if let Some(present) = drvproto::decode_present_header_le(payload) {
                        self.seen_present = Some(present);
                    }
                    let mut ack_buf = [0u8; 64];
                    let ack_len = drvproto::encode_message(&mut ack_buf, drvproto::MSG_ACK, &[]).unwrap();
                    link.driver_send(&ack_buf[..ack_len]);
                }
                _ => {}
            }
        }
    }
}

struct BloomSim {
    negotiated: Option<(u32, u16)>,
    negotiation_count: u32,
    rect_count: u32,
}

impl BloomSim {
    fn new(rect_count: u32) -> Self {
        Self {
            negotiated: None,
            negotiation_count: 0,
            rect_count,
        }
    }

    fn send_hello(&mut self, link: &mut Link) {
        let hello = drvproto::HelloPayload {
            proto_major: drvproto::PROTO_MAJOR,
            proto_minor: drvproto::PROTO_MINOR,
            want_caps: drvproto::CAP_DIRTY_RECTS | drvproto::CAP_FULLFRAME,
        };
        let mut hello_bytes = [0u8; drvproto::HELLO_PAYLOAD_WIRE_SIZE];
        let len = drvproto::encode_hello_payload_le(&hello, &mut hello_bytes).unwrap();
        let mut msg_buf = [0u8; 64];
        let msg_len = drvproto::encode_message(&mut msg_buf, drvproto::MSG_HELLO, &hello_bytes[..len]).unwrap();
        link.client_send(&msg_buf[..msg_len]);
    }

    fn pump(&mut self, link: &mut Link) {
        while let Some((header, payload)) = link.client_rx.next_message() {
            if header.msg_type == drvproto::MSG_WELCOME {
                if let Some(welcome) = drvproto::decode_welcome_payload_le(payload) {
                    if self.negotiated.is_none() {
                        self.negotiated = Some((welcome.have_caps, welcome.max_rects));
                        self.negotiation_count += 1;
                        self.send_bind(link);
                        self.send_present(link);
                    }
                }
            }
        }
    }

    fn send_bind(&self, link: &mut Link) {
        let bind = drvproto::BindPayload {
            bytespace_id: 1,
            width: 800,
            height: 600,
            stride: 3200,
            format: 2,
        };
        let mut bind_bytes = [0u8; drvproto::BIND_PAYLOAD_WIRE_SIZE];
        let len = drvproto::encode_bind_payload_le(&bind, &mut bind_bytes).unwrap();
        let mut msg_buf = [0u8; 128];
        let msg_len = drvproto::encode_message(&mut msg_buf, drvproto::MSG_BIND, &bind_bytes[..len]).unwrap();
        link.client_send(&msg_buf[..msg_len]);
    }

    fn send_present(&self, link: &mut Link) {
        let (caps, max_rects) = self.negotiated.unwrap();
        let use_rects = (caps & drvproto::CAP_DIRTY_RECTS != 0) && self.rect_count <= max_rects as u32;
        let flags = if use_rects { 0 } else { drvproto::PRESENT_FLAG_FULLFRAME };
        let present_rect_count = if use_rects { self.rect_count } else { 0 };

        let rects = core::iter::repeat(drvproto::Rect { x: 0, y: 0, w: 1, h: 1 })
            .take(present_rect_count as usize);
        let mut payload = [0u8; 8 + 16 * 8];
        let payload_len = drvproto::encode_present_payload_with_flags_le(
            present_rect_count,
            flags,
            rects,
            &mut payload,
        )
        .unwrap();
        let mut msg_buf = [0u8; 256];
        let msg_len = drvproto::encode_message(&mut msg_buf, drvproto::MSG_PRESENT, &payload[..payload_len]).unwrap();
        link.client_send(&msg_buf[..msg_len]);
    }
}

fn run_scenario(cfg: DriverConfig, rect_count: u32) -> (DriverSim, BloomSim, Link) {
    let mut link = Link::new();
    let mut driver = DriverSim::new(cfg);
    let mut bloom = BloomSim::new(rect_count);

    bloom.send_hello(&mut link);
    driver.pump(&mut link);
    bloom.pump(&mut link);
    driver.pump(&mut link);

    (driver, bloom, link)
}

#[test]
fn negotiation_with_rects_uses_rect_presents() {
    let cfg = DriverConfig {
        caps: drvproto::CAP_DIRTY_RECTS | drvproto::CAP_FULLFRAME,
        max_rects: 8,
        split_writes: false,
        burst: false,
    };
    let (driver, bloom, _link) = run_scenario(cfg, 2);
    assert!(driver.seen_hello);
    assert_eq!(bloom.negotiation_count, 1);
    let present = driver.seen_present.expect("present");
    assert_eq!(present.rect_count, 2);
    assert_eq!(present._pad & drvproto::PRESENT_FLAG_FULLFRAME, 0);
}

#[test]
fn negotiation_with_tiny_max_rects_falls_back() {
    let cfg = DriverConfig {
        caps: drvproto::CAP_DIRTY_RECTS | drvproto::CAP_FULLFRAME,
        max_rects: 1,
        split_writes: false,
        burst: false,
    };
    let (driver, _bloom, _link) = run_scenario(cfg, 4);
    let present = driver.seen_present.expect("present");
    assert_eq!(present.rect_count, 0);
    assert_ne!(present._pad & drvproto::PRESENT_FLAG_FULLFRAME, 0);
}

#[test]
fn negotiation_fullframe_only_always_fullframe() {
    let cfg = DriverConfig {
        caps: drvproto::CAP_FULLFRAME,
        max_rects: 8,
        split_writes: false,
        burst: false,
    };
    let (driver, _bloom, _link) = run_scenario(cfg, 3);
    let present = driver.seen_present.expect("present");
    assert_eq!(present.rect_count, 0);
    assert_ne!(present._pad & drvproto::PRESENT_FLAG_FULLFRAME, 0);
}

#[test]
fn negotiation_with_fragmented_welcome() {
    let cfg = DriverConfig {
        caps: drvproto::CAP_DIRTY_RECTS | drvproto::CAP_FULLFRAME,
        max_rects: 8,
        split_writes: true,
        burst: false,
    };
    let (driver, bloom, _link) = run_scenario(cfg, 2);
    assert!(driver.seen_hello);
    assert_eq!(bloom.negotiation_count, 1);
    let present = driver.seen_present.expect("present");
    assert_eq!(present.rect_count, 2);
}
