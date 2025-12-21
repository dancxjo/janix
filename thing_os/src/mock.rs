use alloc::vec::Vec;
use alloc::collections::VecDeque;
use core::cell::RefCell;
use abi::{KernelRequest, KernelResponse};

thread_local! {
    static RESPONSES: RefCell<VecDeque<KernelResponse>> = RefCell::new(VecDeque::new());
    static REQUESTS: RefCell<Vec<KernelRequest>> = RefCell::new(Vec::new());
}

pub fn set_responses(resps: Vec<KernelResponse>) {
    RESPONSES.with(|r| *r.borrow_mut() = resps.into());
    REQUESTS.with(|r| r.borrow_mut().clear());
}

pub fn get_requests() -> Vec<KernelRequest> {
    REQUESTS.with(|r| r.borrow().clone())
}

pub fn handle_syscall(request: KernelRequest) -> KernelResponse {
    REQUESTS.with(|r| r.borrow_mut().push(request));
    RESPONSES.with(|r| {
        r.borrow_mut()
            .pop_front()
            .expect("mock response exhausted")
    })
}
