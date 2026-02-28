#![no_std]
#![cfg_attr(not(test), no_main)]

extern crate alloc;
extern crate stem;

use abi::ids::HandleId;
use described::{
    DescribeMode, DescribeRequest, DescribeSink, DescriptionService, SysGraph, SystemClock,
    ViewSpec,
};
use llm_stub::StubLlmClient;
use stem::thing::ThingId;

struct ConsoleSink;

impl DescribeSink for ConsoleSink {
    fn on_delta(&mut self, delta: &llm::ChatDelta) {
        stem::print!("{}", delta.text);
    }
}

#[cfg_attr(not(test), stem::main)]
fn main(arg: usize) -> ! {
    if arg == 0 {
        stem::println!("usage: sysdescribe <THING_ID> [--short|--long] [--view node|nhop]");
        stem::syscall::exit(1);
    }

    let thing_id = ThingId::from_u64(arg as u64);
    let client = StubLlmClient::new();
    let clock = SystemClock;
    let service = DescriptionService::new(&client, &clock);
    let mut graph = SysGraph;
    let mut sink = ConsoleSink;

    let req = DescribeRequest {
        thing_id,
        view_spec: ViewSpec::NodeOnly,
        mode: DescribeMode::Short,
    };

    match service.describe(&mut graph, req, Some(&mut sink)) {
        Ok(resp) => {
            stem::println!();
            stem::println!(
                "description_id {} bytes={}",
                resp.description_id.to_u64_lossy(),
                resp.text.len()
            );
            stem::syscall::exit(0);
        }
        Err(err) => {
            stem::println!();
            stem::println!("sysdescribe error: {:?}", err);
            stem::syscall::exit(1);
        }
    }
}
