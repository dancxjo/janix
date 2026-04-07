//! Async/Await runtime for ThingOS.
//!
//! Provides a cooperative executor that natively integrates with `WaitSet`.
//! Operations return `Poll::Pending` and register their `WaitSpec` dynamically.

use crate::errors::Errno;
use crate::wait_set::{WaitSet, WaitToken, WaitEvents};
use alloc::sync::Arc;
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll, Waker, RawWaker, RawWakerVTable};
use core::sync::atomic::{AtomicPtr, Ordering};

/// A global pointer to the currently running reactor.
/// While ThingOS is mostly single-threaded, `AtomicPtr` safe-guards concurrent
/// thread setups if each thread invokes `block_on` sequentially. 
static CURRENT_REACTOR: AtomicPtr<Reactor> = AtomicPtr::new(core::ptr::null_mut());

/// The active I/O reactor instance backing `block_on`.
pub struct Reactor {
    wait_set: core::cell::RefCell<WaitSet>,
    wakers: core::cell::RefCell<BTreeMap<WaitToken, Waker>>,
}

impl Reactor {
    pub fn new() -> Self {
        Self {
            wait_set: core::cell::RefCell::new(WaitSet::new()),
            wakers: core::cell::RefCell::new(BTreeMap::new()),
        }
    }

    /// Access the current thread-local reactor.
    pub fn current() -> Option<&'static Reactor> {
        let ptr = CURRENT_REACTOR.load(Ordering::SeqCst);
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { &*ptr })
        }
    }

    /// Register a read interest on a port, saving the waker.
    pub fn add_port_readable(&self, handle: u64, waker: Waker) -> Result<WaitToken, Errno> {
        let mut ws = self.wait_set.borrow_mut();
        let token = ws.add_port_readable(handle)?;
        self.wakers.borrow_mut().insert(token, waker);
        Ok(token)
    }

    pub fn remove(&self, token: WaitToken) {
        let mut ws = self.wait_set.borrow_mut();
        ws.remove(token);
        self.wakers.borrow_mut().remove(&token);
    }
}

/// Dummy waker implementation for the top-level executor loop right now.
/// In cooperative single-threaded systems, the WaitSet effectively acts as the Waker.
struct DummyWaker;

impl DummyWaker {
    fn raw_waker() -> RawWaker {
        unsafe fn clone(_: *const ()) -> RawWaker { DummyWaker::raw_waker() }
        unsafe fn wake(_: *const ()) {}
        unsafe fn wake_by_ref(_: *const ()) {}
        unsafe fn drop(_: *const ()) {}

        static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);
        RawWaker::new(core::ptr::null(), &VTABLE)
    }
}

/// Blocks the current thread, driving the given future to completion utilizing `WaitSet`.
pub fn block_on<F: Future>(future: F) -> F::Output {
    let reactor = Box::new(Reactor::new());
    let reactor_ptr = Box::into_raw(reactor);
    
    // Install reactor
    CURRENT_REACTOR.store(reactor_ptr, Ordering::SeqCst);

    let waker = unsafe { Waker::from_raw(DummyWaker::raw_waker()) };
    let mut cx = Context::from_waker(&waker);
    let mut pinned = Box::pin(future);

    loop {
        if let Poll::Ready(res) = pinned.as_mut().poll(&mut cx) {
            // Cleanup reactor
            CURRENT_REACTOR.store(core::ptr::null_mut(), Ordering::SeqCst);
            unsafe {
                let _ = Box::from_raw(reactor_ptr);
            }
            return res;
        }

        // Future returned Pending. The reactor is populated with `WaitSpec`s.
        // Wait until something fires.
        let events = {
            let reactor = unsafe { &*reactor_ptr };
            let ws = reactor.wait_set.borrow();
            if ws.is_empty() {
                // If the future is pending but nothing is registered, it's a deadlock
                crate::warn!("block_on: Future pending but WaitSet is empty! Yielding...");
                crate::yield_now();
                continue;
            }
            // Block until event
            ws.wait(None::<crate::time::Duration>).expect("WaitSet failure")
        };

        // For each fired token, we trigger the wakers via wake() and remove the token.
        // The Future will be re-polled and will register interest again if it encounters EAGAIN.
        {
            let reactor = unsafe { &*reactor_ptr };
            let mut wakers = reactor.wakers.borrow_mut();
            let mut ws = reactor.wait_set.borrow_mut();
            for ev in events.iter() {
                if let Some(waker) = wakers.remove(&ev.token()) {
                    waker.wake();
                }
                ws.remove(ev.token());
            }
        }
    }
}

// -----------------------------------------------------------------------------
// Async Implementations

use crate::syscall;

/// Equivalent to a standard Port handle, optimized for async usage.
pub struct AsyncPort {
    handle: u64,
}

impl AsyncPort {
    pub fn new(handle: u64) -> Self {
        Self { handle }
    }

    pub fn handle(&self) -> u64 {
        self.handle
    }

    /// Read asynchronously from the port. 
    /// If empty, it registers into the active `Reactor` to await `WaitKind::Port`.
    pub async fn recv(&self, buf: &mut [u8]) -> Result<usize, Errno> {
        core::future::poll_fn(|cx| {
            // Attempt standard read
            match syscall::port_recv(self.handle as syscall::PortHandle, buf) {
                Ok(n) => Poll::Ready(Ok(n)),
                Err(Errno::EAGAIN) => {
                    // Queue for async wait
                    if let Some(reactor) = Reactor::current() {
                        let _ = reactor.add_port_readable(self.handle, cx.waker().clone());
                        Poll::Pending
                    } else {
                        // If no runtime is active, we just return EAGAIN instead of implicit sleep
                        Poll::Ready(Err(Errno::EAGAIN))
                    }
                }
                Err(e) => Poll::Ready(Err(e)),
            }
        }).await
    }
}

pub struct Select2<F1, F2> {
    f1: F1,
    f2: F2,
}

impl<F1, F2> Future for Select2<F1, F2>
where
    F1: Future + Unpin,
    F2: Future + Unpin,
{
    type Output = Result<F1::Output, F2::Output>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Poll::Ready(res1) = Pin::new(&mut self.f1).poll(cx) {
            return Poll::Ready(Ok(res1));
        }
        if let Poll::Ready(res2) = Pin::new(&mut self.f2).poll(cx) {
            return Poll::Ready(Err(res2));
        }
        Poll::Pending
    }
}

/// A simple combinator to await one of two unpinned futures.
/// Returns `Ok(out)` if `f1` completes first, or `Err(out)` if `f2` completes first.
pub fn select2<F1, F2>(f1: F1, f2: F2) -> Select2<F1, F2>
where
    F1: Future + Unpin,
    F2: Future + Unpin,
{
    Select2 { f1, f2 }
}
