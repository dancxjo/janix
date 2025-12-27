#![no_std]
#![no_main]

extern crate alloc;

use thing_std::{GraphClient, debug};
use abi::wire::time::RtcSample;
use abi::wire::graph::{GraphOp, GraphReply};
use abi::wire::typed::{TypedBytes, TypeId, CodecId};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct SystemTimeProps {
    unix_seconds: u64,
}

#[no_mangle]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let _ = run();
    loop {}
}

fn run() -> Result<(), ()> {
    debug::log("RTC_X86: Starting...\n");
    
    let client = GraphClient::new();
    let mut buf = [0u8; 1024];

    // Create SystemTime Thing
    // Kind 200 (SystemTime)
    let initial_props = SystemTimeProps { unix_seconds: 0 };
    let initial_bytes = postcard::to_allocvec(&initial_props).map_err(|_| ())?;
    let typed_body = TypedBytes { 
        type_id: TypeId(200), 
        codec_id: CodecId::POSTCARD, 
        bytes: initial_bytes 
    };
    
    let op = GraphOp::CreateThing { kind: abi::ids::ThingId(200), value: typed_body };
    let reply = client.call_op(&op, &mut buf).map_err(|_| ())?;
    
    let sys_time_id = match reply {
        GraphReply::Created { id } => {
            use thing_std::console::{Console, StdoutConsole};
            let c = StdoutConsole;
            c.write_str("RTC: CreateThing success ID: ");
            c.write_u64(id.0);
            c.write_str("\n");
            id
        },
        _ => {
            debug::log("RTC: CreateThing failed\n");
            return Err(());
        }
    };

    let link_op = GraphOp::AddLink { from: abi::ids::ThingId(1000), to: sys_time_id, kind: abi::ids::ThingId(100) };
    let reply_link = client.call_op(&link_op, &mut buf);
    match reply_link {
        Ok(_) => debug::log("RTC: Linked SystemTime to Root\n"),
        Err(_) => debug::log("RTC: Link failed\n"),
    }

                let op = GraphOp::UpdateThing { id: sys_time_id, value: body };
                let _ = client.call_op(&op, &mut buf);
            },
            Err(_) => {
                 debug::log("RTC: Read failed\n");
            }
        }
        
        let _ = thing_std::time::sleep_s(&client, 1);
    }
}

fn ymd_to_unix(sample: RtcSample) -> u64 {
    let year = sample.year as u64;
    let mut total_days = 0;
    
    for y in 1970..year {
        let is_leap = (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0);
        total_days += if is_leap { 366 } else { 365 };
    }
    
    let is_leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
    let days_in_month = [31, if is_leap { 29 } else { 28 }, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    
    for m in 0..(sample.mon - 1) as usize {
        total_days += days_in_month[m];
    }
    
    total_days += (sample.day - 1) as u64;
    
    let total_sec = total_days * 86400 + 
                    sample.hour as u64 * 3600 + 
                    sample.min as u64 * 60 + 
                    sample.sec as u64;
    total_sec
}

fn local_rtc_read(out: &mut RtcSample) -> Result<(), ()> {
    let n = abi::syscall_defs::SYSCALL_RTC_READ;
    let ret: usize;
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") n,
            in("rdi") (out as *mut RtcSample as usize),
            in("rsi") 0,
            lateout("rax") ret,
            out("rcx") _,
            out("r11") _,
            options(nostack, preserves_flags)
        );
    }
    if ret == 0 { Ok(()) } else { Err(()) }
}
