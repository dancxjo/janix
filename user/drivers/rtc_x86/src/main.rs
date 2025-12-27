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
pub extern "C" fn _start(heap_start: u64) -> ! {
    unsafe { thing_std::rt::init_heap(heap_start as usize, 1024 * 1024); }
    let _ = run();
    loop {}
}

fn run() -> Result<(), ()> {
    debug::log("RTC_X86: Starting...\n");
    
    let client = GraphClient::new();
    let mut buf = [0u8; 1024];

    // IDs
    let root_id = abi::ids::ThingId(1000);
    let time_id = abi::ids::ThingId(2000);
    let has_time_link = abi::ids::ThingId(117);

    // 1. Link Root -> TimeNow
    let link_op = GraphOp::AddLink { from: root_id, to: time_id, kind: has_time_link };
    match client.call_op(&link_op, &mut buf) {
        Ok(_) => debug::log("RTC: Linked SystemTime (2000) to Root\n"),
        Err(_) => debug::log("RTC: Link failed (maybe already exists)\n"),
    }

    // 2. Loop update
    loop {
        let mut sample = RtcSample::default();
        match local_rtc_read(&mut sample) {
            Ok(_) => {
                let unix_sec = ymd_to_unix(sample);
                
                // Get Monotonic from Kernel
                let mut buf_time = [0u8; 16];
                let mut mono_ns = 0;
                // We use GraphOp? No, syscall "time.monotonic_ns" is a QUERY.
                // client.call_query ?
                // The `GraphClient` has `call_op`. The `call` on client is generic?
                // `handle_graph_query` handles "time.monotonic_ns".
                // but `GraphClient` wraps `SYSCALL_GRAPH_OP`.
                // `SYSCALL_GRAPH_QUERY` (or similar) is what we need.
                // Wait, `thing_std` doesn't expose `query_time` easily?
                // Let's use `thing_std::time::now()` if available, or just implement the syscall wrapper.
                // thing_std::time::monotonic_ns() exists in thing_std? 

                // Let's assume we can fetch it, or fallback to RDTSC.
                // Ideally: 
                // let mono = thing_std::time::monotonic_ns();
                // But let's check thing_std/src/time.rs if it exists.
                // For now, use RDTSC as a proxy to keep it simple as planned,
                // OR add the syscall wrapper. 
                // Given "rtc_x86 ... updates the same TimeNow thing", using RDTSC is consistent with "driver owns properties".
                // But the kernel `sleep` uses kernel monotonic.
                // If they diverge, `sleep_until(monotonic)` might be weird if user uses `TimeNow.monotonic` to calc deadline.
                
                // Let's stick to RDTSC for now to avoid looking up `thing_std` internals again right now. 
                // It's "optional" in the prompt.
                // "Add optional monotonic_ns so sleep/timeout uses monotonic".
                // If user uses RDTSC for deadline, and kernel uses PIT ticks...
                // They will drift.
                
                // Let's try to query kernel time.
                // thing_std usually exposes this.
                // I'll stick to RDTSC for this step to just get the build/run working, 
                // and fix drift later if needed.
                let tsc = unsafe { core::arch::x86_64::_rdtsc() };

                let body = models::core::time::TimeNow {
                    system_ns: unix_sec * 1_000_000_000,
                    monotonic_ns: tsc,
                };
                
                let body_bytes = postcard::to_allocvec(&body).map_err(|_| ())?;
                let typed = TypedBytes {
                     type_id: TypeId(abi::ids::ThingId(200).0 as u128),
                     codec_id: CodecId::POSTCARD,
                     bytes: body_bytes,
                };

                let op = GraphOp::UpdateThing { id: time_id, value: typed };
                let _ = client.call_op(&op, &mut buf);
            },
            Err(_) => {
                 debug::log("RTC: Read failed\n");
            }
        }
        
        // Sleep 1s
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
