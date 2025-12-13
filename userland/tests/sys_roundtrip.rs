use std::cell::RefCell;
use std::collections::VecDeque;

use abi::{KernelRequest, KernelResponse, PropKey, PropType, PropValue, Thing, ThingId};
use runtime::Sys;
use userland_std::{create_thing, load_thing, user_update_thing};

// MockSys implementation
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

    fn pop_request(&self) -> Option<KernelRequest> {
        self.requests.borrow_mut().pop()
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

    fn time_now_ns(&mut self) -> u64 {
        0
    }
    fn time_monotonic_ns(&mut self) -> u64 {
        0
    }
    fn time_system_ns(&mut self) -> u64 {
        0
    }
    fn sleep_for_ns(&mut self, _delta_ns: u64) {}
    fn sleep_until_ns(&mut self, _deadline_ns: u64) {}
    fn yield_now(&mut self) {}
    fn exit_thread(&mut self) -> ! {
        panic!("exit")
    }
}

// Test Thing
#[derive(Clone, Debug, PartialEq)]
struct TestThing {
    id: ThingId,
    val: u64,
}

impl Thing for TestThing {
    const KIND: &'static str = "TestThing";
    const DESCRIPTION: &'static str = "A test thing";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push(("val", PropValue::U64(self.val)));
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut val = 0;
        for (k, v) in props.iter().flatten() {
            if *k == "val" {
                if let PropValue::U64(v) = v {
                    val = *v;
                }
            }
        }
        TestThing { id, val }
    }

    fn schema() -> &'static [(&'static str, PropType)] {
        &[("val", PropType::U64)]
    }
}

#[test]
fn test_create_thing_roundtrip() {
    let sys = MockSys::with_responses(vec![KernelResponse::ThingCreated { id: ThingId(100) }]);

    let thing = TestThing {
        id: ThingId(0),
        val: 42,
    };
    let result = create_thing(&sys, &thing);

    assert_eq!(result, Some(ThingId(100)));

    let req = sys.pop_request().unwrap();
    if let KernelRequest::ThingCreate { kind, props } = req {
        assert_eq!(kind, "TestThing");
        assert_eq!(props.len(), 1);
        assert_eq!(props[0].0, "val");
        assert_eq!(props[0].1, PropValue::U64(42));
    } else {
        panic!("Unexpected request: {:?}", req);
    }
}

#[test]
fn test_load_thing_roundtrip() {
    // We need to leak the props to match the static lifetime requirement of KernelResponse
    let props: &'static [Option<(PropKey, PropValue)>] =
        Box::leak(vec![Some(("val", PropValue::U64(123)))].into_boxed_slice());

    let sys = MockSys::with_responses(vec![KernelResponse::ThingData {
        id: ThingId(200),
        kind: "TestThing",
        props,
    }]);

    let thing = load_thing::<TestThing>(&sys, ThingId(200)).unwrap();

    assert_eq!(thing.id, ThingId(200));
    assert_eq!(thing.val, 123);

    let req = sys.pop_request().unwrap();
    if let KernelRequest::ThingGet { id } = req {
        assert_eq!(id, ThingId(200));
    } else {
        panic!("Unexpected request: {:?}", req);
    }
}

#[test]
fn test_update_thing_roundtrip() {
    let sys = MockSys::with_responses(vec![KernelResponse::Success { data: None }]);

    let props: &'static [(PropKey, PropValue)] =
        Box::leak(vec![("val", PropValue::U64(999))].into_boxed_slice());

    let result = user_update_thing(&sys, ThingId(300), props);
    assert!(result.is_ok());

    let req = sys.pop_request().unwrap();
    if let KernelRequest::ThingUpdate {
        id,
        props: req_props,
    } = req
    {
        assert_eq!(id, ThingId(300));
        assert_eq!(req_props.len(), 1);
        assert_eq!(req_props[0].0, "val");
        assert_eq!(req_props[0].1, PropValue::U64(999));
    } else {
        panic!("Unexpected request: {:?}", req);
    }
}
