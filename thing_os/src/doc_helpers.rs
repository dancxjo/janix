#[cfg(not(target_os = "none"))]
use std::cell::RefCell;
#[cfg(not(target_os = "none"))]
use std::collections::VecDeque;

#[cfg(not(target_os = "none"))]
use abi::{KernelRequest, KernelResponse, PropKey, PropType, PropValue, Thing, ThingId};
#[cfg(not(target_os = "none"))]
use runtime::Sys;

#[cfg(not(target_os = "none"))]
/// Simple helper for doc tests that drives the syscall interface with canned responses.
pub struct DocSys {
    pub requests: RefCell<Vec<KernelRequest>>,
    responses: RefCell<VecDeque<KernelResponse>>,
    pub time: RefCell<u64>,
    pub slept_for: RefCell<Vec<u64>>,
}

#[cfg(not(target_os = "none"))]
impl DocSys {
    pub fn with_responses(responses: Vec<KernelResponse>) -> Self {
        Self {
            requests: RefCell::new(Vec::new()),
            responses: RefCell::new(responses.into()),
            time: RefCell::new(0),
            slept_for: RefCell::new(Vec::new()),
        }
    }

    pub fn push_response(&self, response: KernelResponse) {
        self.responses.borrow_mut().push_back(response);
    }

    pub fn props_slice(
        props: Vec<(PropKey, PropValue)>,
    ) -> &'static [Option<(PropKey, PropValue)>] {
        Box::leak(
            props
                .into_iter()
                .map(Some)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        )
    }
}

#[cfg(not(target_os = "none"))]
impl Sys for DocSys {
    fn syscall(&self, request: KernelRequest) -> KernelResponse {
        self.requests.borrow_mut().push(request);
        self.responses
            .borrow_mut()
            .pop_front()
            .expect("doc helper responses exhausted")
    }

    fn time_now_ns(&mut self) -> u64 {
        let mut t = self.time.borrow_mut();
        *t += 1;
        *t
    }

    fn time_monotonic_ns(&mut self) -> u64 {
        self.time_now_ns()
    }

    fn time_system_ns(&mut self) -> u64 {
        self.time_now_ns()
    }

    fn sleep_for_ns(&mut self, delta_ns: u64) {
        self.slept_for.borrow_mut().push(delta_ns);
    }

    fn sleep_until_ns(&mut self, deadline_ns: u64) {
        self.slept_for.borrow_mut().push(deadline_ns);
    }

    fn yield_now(&mut self) {}

    fn exit_thread(&mut self) -> ! {
        panic!("doc helper exit_thread invoked")
    }
}

#[cfg(not(target_os = "none"))]
/// A lightweight Thing implementation for doc/test snippets.
#[derive(Clone, Debug)]
pub struct DummyThing {
    pub id: ThingId,
    pub flag: bool,
}

#[cfg(not(target_os = "none"))]
impl DummyThing {
    pub fn new(flag: bool) -> Self {
        DummyThing {
            id: ThingId(0),
            flag,
        }
    }
}

#[cfg(not(target_os = "none"))]
impl Thing for DummyThing {
    const KIND: &'static str = "DocDummy";
    const DESCRIPTION: &'static str = "Dummy Thing used in documentation snippets";

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
