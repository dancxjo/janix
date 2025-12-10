extern crate alloc;

use alloc::boxed::Box;
use alloc::collections::VecDeque;
use alloc::vec;
use alloc::vec::Vec;
use core::cell::RefCell;
use core::mem;

use abi::{
    KernelRequest, KernelResponse, PixelFormat, PropKey, PropValue, SharedBufferInfo, ThingId,
};
use userland_rt::Sys;
use userland_std::{PrimaryDisplayBuffer, SharedBufferMapping, Thing};

#[derive(Default)]
pub struct MockSys {
    responses: RefCell<VecDeque<KernelResponse>>,
    requests: RefCell<Vec<KernelRequest>>,
    time: RefCell<u64>,
}

impl MockSys {
    pub fn with_responses(responses: Vec<KernelResponse>) -> Self {
        Self {
            responses: RefCell::new(responses.into()),
            ..Default::default()
        }
    }

    pub fn push_response(&self, response: KernelResponse) {
        self.responses.borrow_mut().push_back(response);
    }

    pub fn drain_requests(&self) -> Vec<KernelRequest> {
        std::mem::take(&mut self.requests.borrow_mut())
    }

    pub fn set_time(&self, value: u64) {
        *self.time.borrow_mut() = value;
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
        let mut t = self.time.borrow_mut();
        *t = t.saturating_add(1);
        *t
    }

    fn time_monotonic_ns(&mut self) -> u64 {
        self.time_now_ns()
    }

    fn time_system_ns(&mut self) -> u64 {
        self.time_now_ns()
    }

    fn sleep_for_ns(&mut self, _delta_ns: u64) {}

    fn sleep_until_ns(&mut self, _deadline_ns: u64) {}

    fn yield_now(&mut self) {}

    fn exit_thread(&mut self) -> ! {
        panic!("exit_thread called on MockSys")
    }
}

pub fn leaked_props(props: Vec<(PropKey, PropValue)>) -> &'static [Option<(PropKey, PropValue)>] {
    Box::leak(
        props
            .into_iter()
            .map(Some)
            .collect::<Vec<_>>()
            .into_boxed_slice(),
    )
}

pub fn thing_data_response<T: Thing>(id: ThingId, thing: &T) -> KernelResponse {
    let mut props = Vec::new();
    thing.to_props(&mut props);
    KernelResponse::ThingData {
        id,
        kind: T::KIND,
        props: leaked_props(props),
    }
}

pub fn list_responses<T: Thing, F: Fn(&T) -> ThingId>(
    things: Vec<T>,
    id_fn: F,
) -> Vec<KernelResponse> {
    let mut responses = Vec::new();
    for thing in things {
        let id = id_fn(&thing);
        responses.push(KernelResponse::ThingListEntry { id: Some(id) });
        responses.push(thing_data_response(id, &thing));
    }
    responses.push(KernelResponse::ThingListEntry { id: None });
    responses
}

pub fn success() -> KernelResponse {
    KernelResponse::Success { data: None }
}

pub struct FramebufferFixture {
    pub fb: PrimaryDisplayBuffer,
    pub buffer: Vec<u32>,
}

impl FramebufferFixture {
    pub fn new(width: u32, height: u32) -> Self {
        let mut buffer = vec![0u32; (width * height) as usize];
        let info = SharedBufferInfo {
            width,
            height,
            stride: width * 4,
            pixel_format: PixelFormat::Rgba8888,
        };

        let mut fb = PrimaryDisplayBuffer {
            display_id: ThingId(1),
            buffers: [
                SharedBufferMapping {
                    id: ThingId(2),
                    info,
                    ptr: buffer.as_mut_ptr() as *mut u8,
                    size: buffer.len() * mem::size_of::<u32>(),
                },
                SharedBufferMapping {
                    id: ThingId(3),
                    info,
                    ptr: buffer.as_mut_ptr() as *mut u8,
                    size: buffer.len() * mem::size_of::<u32>(),
                },
            ],
            active_buffer_index: 0,
            info,
            ptr: buffer.as_mut_ptr() as *mut u8,
        };
        fb.update_active_index(0);

        Self { fb, buffer }
    }
}

#[derive(Clone, Debug)]
pub struct DummyThing {
    pub id: ThingId,
    pub flag: bool,
}

impl DummyThing {
    pub fn new(flag: bool) -> Self {
        DummyThing {
            id: ThingId(0),
            flag,
        }
    }
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

    fn schema() -> &'static [(&'static str, abi::PropType)] {
        &[("flag", abi::PropType::Bool)]
    }
}
