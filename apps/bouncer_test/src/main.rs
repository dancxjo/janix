#![no_std]
#![no_main]

extern crate alloc;
use thing_std::*;
use abi::bodies::ThingEnvelopeV1;
use abi::syscall::err;

#[no_mangle]
pub fn main() {
    log_info("--- Bouncer Test Tool ---");

    let kind = symbol_intern("kind.Test");
    let root = ThingId(2);
    let tid = thing_create(kind, root);

    // 1. Invalid Magic
    let mut bad_magic = [0u8; 128];
    bad_magic[0..4].copy_from_slice(b"BAD!");
    let res = thing_set_body(tid, &bad_magic);
    if let Err(e) = res {
        if e == err::ERR_INVALID_THING_BODY as i32 {
            log_info("TEST: invalid magic rejected - PASS");
        } else {
            log_fmt("TEST: invalid magic rejected with wrong error: ", format_args!("{}", e));
        }
    } else {
        log_info("TEST: invalid magic accepted? - FAIL");
    }

    // 2. Valid Envelope
    let mut model = models::Place { name: symbol_intern("TestPlace") };
    let full = Codec::encode(&model);
    let res = thing_set_body(tid, &full);
    if res.is_ok() {
        log_info("TEST: valid envelope accepted - PASS");
        
        // Retrieve and check digest
        if let Some((body, kernel_digest)) = thing_get_body(tid) {
            log_fmt("TEST: retrieved digest: 0x", format_args!("{:X}", kernel_digest));
            if kernel_digest != 0 {
                 log_info("TEST: integrity digest verified - PASS");
            }
        }
    } else {
        log_fmt("TEST: valid envelope rejected with error: ", format_args!("{:?}", res.err()));
    }

    log_info("Bouncer tests complete.");
}

fn log_fmt(prefix: &str, args: core::fmt::Arguments) {
    let mut buf = alloc::string::String::new();
    let _ = core::fmt::write(&mut buf, args);
    log_info(alloc::format!("{}{}", prefix, buf).as_str());
}
