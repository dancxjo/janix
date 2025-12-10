use std::cell::RefCell;
use std::collections::VecDeque;

use abi::{
    KernelRequest, KernelResponse, PropKey, PropType, PropValue, ThingId,
};
use userland_rt::Sys;
use userland_std::{
    create_transaction, find_thing, Thing,
};

#[derive(Default)]
struct MockSys {
    responses: RefCell<VecDeque<KernelResponse>>,
    requests: RefCell<Vec<KernelRequest>>,
}

impl MockSys {
    fn with_responses(resps: Vec<KernelResponse>) -> Self {
        Self {
            responses: RefCell::new(resps.into()),
            ..Default::default()
        }
    }
}

impl Sys for MockSys {
    fn syscall(&self, request: KernelRequest) -> KernelResponse {
        self.requests.borrow_mut().push(request);
        self.responses
            .borrow_mut()
            .pop_front()
            .expect("mock response exhausted")
    }

    fn time_now_ns(&mut self) -> u64 { 0 }
    fn time_monotonic_ns(&mut self) -> u64 { 0 }
    fn time_system_ns(&mut self) -> u64 { 0 }
    fn sleep_for_ns(&mut self, _delta_ns: u64) {}
    fn sleep_until_ns(&mut self, _deadline_ns: u64) {}
    fn yield_now(&mut self) {}
    fn exit_thread(&mut self) -> ! { panic!("exit") }
}

#[derive(Clone, Debug)]
struct DummyThing {
    id: ThingId,
    flag: bool,
}

impl Thing for DummyThing {
    const KIND: &'static str = "Dummy";
    const DESCRIPTION: &'static str = "dummy";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push(("flag", PropValue::Bool(self.flag)));
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut flag = false;
        for (k, v) in props.iter().flatten() {
            if *k == "flag" {
                if let PropValue::Bool(b) = v {
                    flag = *b;
                }
            }
        }
        DummyThing { id, flag }
    }

    fn schema() -> &'static [(&'static str, PropType)] {
        &[("flag", PropType::Bool)]
    }
}

fn props_slice(props: Vec<(PropKey, PropValue)>) -> &'static [Option<(PropKey, PropValue)>] {
    Box::leak(
        props
            .into_iter()
            .map(Some)
            .collect::<Vec<_>>()
            .into_boxed_slice(),
    )
}

#[test]
fn find_thing_returns_none_on_no_match() {
    // Simulate a scan where no Thing matches the predicate.
    // find_thing scans 128 IDs. We'll provide 128 responses of ThingData,
    // but none of them will match the predicate.
    // Actually, find_thing implementation in userland_std/src/lib.rs:
    // for i in 0..128 { load_thing... }
    // So we need 128 responses.
    
    let mut responses = Vec::new();
    for i in 0..128 {
        responses.push(KernelResponse::ThingData {
            id: ThingId(i),
            kind: "Dummy",
            props: props_slice(vec![("flag", PropValue::Bool(false))]),
        });
    }
    
    let sys = MockSys::with_responses(responses);
    
    // Predicate looks for flag == true, but all are false
    let result = find_thing::<DummyThing>(&sys, |t| t.flag == true);
    assert!(result.is_none());
}

#[test]
fn create_transaction_aborted_does_not_persist() {
    // This test verifies that if we create a transaction but don't commit it,
    // we don't see a CommitTransaction syscall.
    
    let sys = MockSys::with_responses(vec![
        KernelResponse::TransactionCreated { tx_id: abi::TransactionId(1) },
        // No commit response needed if we don't commit
    ]);

    let tx_id = create_transaction(&sys);
    assert_eq!(tx_id, Some(abi::TransactionId(1)));

    // Simulate deciding to abort (just dropping the ID and not calling commit)
    
    let requests = sys.requests.borrow();
    assert_eq!(requests.len(), 1);
    match &requests[0] {
        KernelRequest::CreateTransaction => {},
        _ => panic!("Expected CreateTransaction"),
    }
    // Verify NO CommitTransaction
}

#[test]
fn shared_buffer_and_display_open_sad_path() {
    // Test open_primary_display_buffer when the display thing is missing or malformed.
    // userland_std::open_primary_display_buffer calls find_thing internally to find the display.
    // If find_thing fails (returns None), open_primary_display_buffer should return Err.
    
    // We'll simulate find_thing failing by providing responses that are NOT displays or don't match.
    // find_thing scans 128 IDs. We can just provide 128 "None" responses (e.g. Error or wrong kind).
    
    let mut responses = Vec::new();
    for _ in 0..128 {
        responses.push(KernelResponse::Error { message: "Not found" });
    }
    
    let sys = MockSys::with_responses(responses);
    
    let result = userland_std::open_primary_display_buffer(&mut {sys});
    assert!(result.is_err());
}
