use crate::test_support::MockSys;
use abi::{KernelRequest, KernelResponse, ThingId};
use userland_std::Thing;

#[test]
fn test_mock_sys_push_response() {
    // This test verifies that MockSys::push_response works as expected.
    // We'll seed the mock with a response, then make a request that consumes it.

    let sys = MockSys::default();

    // Push a response for a ThingCreate request
    sys.push_response(KernelResponse::ThingCreated { id: ThingId(123) });

    // Make the request
    let result = userland_std::create_thing(&sys, &crate::test_support::DummyThing::new(true));

    assert_eq!(result, Some(ThingId(123)));

    // Verify the request was recorded
    let requests = sys.drain_requests();
    assert_eq!(requests.len(), 1);
    match &requests[0] {
        KernelRequest::ThingCreate { kind, .. } => {
            assert_eq!(*kind, "Dummy");
        }
        _ => panic!("Expected ThingCreate request"),
    }
}

#[test]
fn test_mock_sys_push_response_sequence() {
    // Verify that responses are consumed in FIFO order
    let sys = MockSys::default();

    sys.push_response(KernelResponse::ThingCreated { id: ThingId(1) });
    sys.push_response(KernelResponse::ThingCreated { id: ThingId(2) });

    let res1 = userland_std::create_thing(&sys, &crate::test_support::DummyThing::new(true));
    let res2 = userland_std::create_thing(&sys, &crate::test_support::DummyThing::new(false));

    assert_eq!(res1, Some(ThingId(1)));
    assert_eq!(res2, Some(ThingId(2)));
}

#[test]
fn test_mock_sys_unexpected_response() {
    // Verify graceful handling of unexpected response types
    let sys = MockSys::default();

    // Push an Error response where a ThingCreated is expected
    sys.push_response(KernelResponse::Error {
        message: "Something went wrong",
    });

    let result = userland_std::create_thing(&sys, &crate::test_support::DummyThing::new(true));

    // create_thing returns None on non-ThingCreated response
    assert_eq!(result, None);
}
