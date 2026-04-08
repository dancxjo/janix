#![feature(restricted_std)]
#![no_main]

extern crate alloc;

mod driver;
mod protocol;

use abi::schema::keys;
use driver::Rtl8168Driver;
use protocol::{NetDriverMsg, MSG_FRAME_RX, MSG_FRAME_TX, MSG_MAC_REQ, MSG_MAC_RESP};
use stem::syscall::{channel_create, channel_recv, channel_send};
use stem::thing::sys as thingsys;
use stem::{error, info, warn};

const KIND_NET_DRIVER: &str = "svc.net.Driver";

#[stem::main]
fn main(_arg: usize) -> ! {
    info!("RTL8168D: starting Realtek RTL8111/8168 driver");

    let mut driver = match Rtl8168Driver::find_and_claim() {
        Ok(d) => d,
        Err(e) => {
            error!("RTL8168D: init failed: {:?}", e);
            loop {
                stem::time::sleep_ms(1000);
            }
        }
    };

    let mac = driver.mac();
    info!(
        "RTL8168D: MAC {:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}, link={}",
        mac[0],
        mac[1],
        mac[2],
        mac[3],
        mac[4],
        mac[5],
        if driver.link_up() { "up" } else { "down" }
    );

    let (tx_write, tx_read) = match channel_create(65536) {
        Ok(h) => h,
        Err(e) => {
            error!("RTL8168D: TX port create failed: {:?}", e);
            loop {
                stem::time::sleep_ms(1000);
            }
        }
    };
    let (rx_write, rx_read) = match channel_create(65536) {
        Ok(h) => h,
        Err(e) => {
            error!("RTL8168D: RX port create failed: {:?}", e);
            loop {
                stem::time::sleep_ms(1000);
            }
        }
    };

    let svc_id = match thingsys::create_node(KIND_NET_DRIVER) {
        Ok(id) => id,
        Err(e) => {
            error!("RTL8168D: service node create failed: {:?}", e);
            loop {
                stem::time::sleep_ms(1000);
            }
        }
    };

    let mac_packed = (mac[0] as u64)
        | ((mac[1] as u64) << 8)
        | ((mac[2] as u64) << 16)
        | ((mac[3] as u64) << 24)
        | ((mac[4] as u64) << 32)
        | ((mac[5] as u64) << 40);
    let _ = thingsys::prop_set(svc_id, "net.mac", mac_packed);
    let _ = thingsys::prop_set(svc_id, keys::WRITE_PORT_HANDLE, tx_write as u64);
    let _ = thingsys::prop_set(svc_id, "net.rx_port", rx_read as u64);
    let _ = thingsys::prop_set(
        svc_id,
        keys::LINK_STATUS,
        if driver.link_up() { 1 } else { 0 },
    );
    let _ = thingsys::prop_set(
        svc_id,
        keys::IRQ_MODE,
        if driver.irq_enabled() { 1 } else { 0 },
    );

    info!(
        "RTL8168D: service ready (tx_port={} rx_port={})",
        tx_write, rx_read
    );

    let mut tx_msg_buf = [0u8; 2048];
    loop {
        // Opportunistically drain TX requests.
        match channel_recv(tx_read, &mut tx_msg_buf) {
            Ok(n) if n > 0 => {
                if let Some(msg) = NetDriverMsg::decode(&tx_msg_buf[..n]) {
                    match msg.msg_type {
                        MSG_FRAME_TX => {
                            if let Err(e) = driver.tx(msg.payload) {
                                warn!("RTL8168D: TX failed: {}", e);
                            }
                        }
                        MSG_MAC_REQ => {
                            let m = NetDriverMsg::new(MSG_MAC_RESP, &mac);
                            let _ = channel_send(rx_write, &m.encode());
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }

        // Interrupt-driven path (falls back to short sleep if MSI isn't available).
        driver.wait_for_irq();
        let isr = driver.consume_interrupts();
        if (isr & (1 << 5)) != 0 {
            let _ = thingsys::prop_set(
                svc_id,
                keys::LINK_STATUS,
                if driver.link_up() { 1 } else { 0 },
            );
        }

        if (isr & ((1 << 0) | (1 << 1) | (1 << 4) | (1 << 6))) != 0 || !driver.irq_enabled() {
            while let Some(frame) = driver.poll_rx() {
                let msg = NetDriverMsg::new(MSG_FRAME_RX, frame);
                if let Err(e) = channel_send(rx_write, &msg.encode()) {
                    warn!("RTL8168D: RX forward failed: {:?}", e);
                    break;
                }
            }
        }
    }
}
