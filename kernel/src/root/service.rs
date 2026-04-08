//! Root service main loop and message dispatch.

use super::symbols::Interner;
use super::{RootMsg, RootOp};
use crate::BootRuntime;
use crate::root::handlers as root_handlers;
use core::sync::atomic::Ordering;

pub extern "C" fn root_main<R: BootRuntime>(_arg: usize) -> ! {
    // DIAG: raw serial marker - bypasses log infrastructure entirely
    crate::runtime_base().putchar(b'R');
    crate::runtime_base().putchar(b'M');
    crate::runtime_base().putchar(b'\n');
    crate::kinfo!("ROOT: started once");

    let mut interner = Interner::new();

    let mut iteration = 0u64;
    loop {
        iteration = iteration.wrapping_add(1);
        let mut processed_this_round = 0;

        // Process messages until queue is empty or we hit a safety limit.
        while processed_this_round < 128 {
            if let Some(msg) = super::pop_msg() {
                handle_msg::<R>(&mut interner, msg);
                processed_this_round += 1;
            } else {
                break;
            }
        }


        if processed_this_round == 0 {
            super::ROOT_ASLEEP.store(true, Ordering::Release);
            if super::queue_len() > 0 {
                super::ROOT_ASLEEP.store(false, Ordering::Release);
                continue;
            }
            unsafe {
                crate::task::block_current_erased();
            }
        } else if processed_this_round >= 128 {
            unsafe {
                crate::sched::yield_now_current();
            }
        }
    }
}

/// Returns a short name for the RootOp type for logging
#[allow(dead_code)]
fn msg_type_name(op: &RootOp) -> &'static str {
    match op {
        RootOp::Intern { .. } => "Intern",
        RootOp::GetKind { .. } => "GetKind",
        RootOp::CreateNode { .. } => "CreateNode",
        RootOp::ApplyBatch { .. } => "ApplyBatch",
        RootOp::Link { .. } => "Link",
        RootOp::Find { .. } => "Find",
        RootOp::Query { .. } => "Query",
        RootOp::PropGet { .. } => "PropGet",
        RootOp::PropSet { .. } => "PropSet",
        RootOp::WatchSubscribe { .. } => "WatchSubscribe",
        RootOp::StreamPoll { .. } => "StreamPoll",
        RootOp::WatchOpen { .. } => "WatchOpen",
        RootOp::WatchNext { .. } => "WatchNext",
        RootOp::WatchPoll { .. } => "WatchPoll",
        RootOp::WatchRegisterWaiter { .. } => "WatchRegisterWaiter",
        RootOp::WatchUnregisterWaiter { .. } => "WatchUnregisterWaiter",
        RootOp::WatchClose { .. } => "WatchClose",
        RootOp::DescribeThing { .. } => "DescribeThing",
        RootOp::DescribeSymbol { .. } => "DescribeSymbol",
        RootOp::DescribeEdge { .. } => "DescribeEdge",
        RootOp::DumpEdges { .. } => "DumpEdges",
        RootOp::GetEdges { .. } => "GetEdges",
        RootOp::GetProps { .. } => "GetProps",
        RootOp::DumpGraph { .. } => "DumpGraph",
        RootOp::LogEvent { .. } => "LogEvent",
        RootOp::PropsGetMany { .. } => "PropsGetMany",
        RootOp::ResolvePath { .. } => "ResolvePath",
        RootOp::Unlink { .. } => "Unlink",
        RootOp::DirList { .. } => "DirList",
        RootOp::OrphanThing { .. } => "OrphanThing",
        RootOp::CleanupTaskThings { .. } => "CleanupTaskThings",
        _ => "Deprecated",
    }
}

fn handle_msg<R: BootRuntime>(
    interner: &mut Interner,
    msg: RootMsg,
) {
    let (status, value) = match msg.op {
        RootOp::Intern { name } => root_handlers::handle_intern(interner, &name),

        // All graph-related operations stubbed out
        RootOp::GetKind { .. }
        | RootOp::CreateNode { .. }
        | RootOp::Link { .. }
        | RootOp::Find { .. }
        | RootOp::Query { .. }
        | RootOp::PropGet { .. }
        | RootOp::PropSet { .. }
        | RootOp::PropsGetMany { .. }
        | RootOp::ResolvePath { .. }
        | RootOp::Unlink { .. }
        | RootOp::DirList { .. }
        | RootOp::OrphanThing { .. }
        | RootOp::CleanupTaskThings { .. }
        | RootOp::WatchSubscribe { .. }
        | RootOp::StreamPoll { .. }
        | RootOp::WatchOpen { .. }
        | RootOp::WatchNext { .. }
        | RootOp::WatchPoll { .. }
        | RootOp::WatchRegisterWaiter { .. }
        | RootOp::WatchUnregisterWaiter { .. }
        | RootOp::WatchClose { .. }
        | RootOp::DescribeThing { .. }
        | RootOp::DescribeSymbol { .. }
        | RootOp::DescribeEdge { .. }
        | RootOp::DumpEdges { .. }
        | RootOp::GetEdges { .. }
        | RootOp::GetProps { .. }
        | RootOp::DumpGraph { .. }
        | RootOp::LogEvent { .. }
        | RootOp::ApplyBatch { .. } 
        | RootOp::BytespaceCreate { .. }
        | RootOp::BytespaceCreateFromPtr { .. }
        | RootOp::BytespaceWrite { .. }
        | RootOp::BytespaceRead { .. }
        | RootOp::BytespaceInfo { .. }
        | RootOp::BytespaceMap { .. }
        | RootOp::BytespaceUnmap { .. }
        | RootOp::BytespacePhys { .. }
        | RootOp::BytespaceTruncate { .. } => (-38, 0), // ENOSYS
    };

    if let Some(reply) = msg.reply {
        reply.status.store(status as i32, Ordering::Relaxed);
        reply.value.store(value as u64, Ordering::Relaxed);
        reply.done.store(1, Ordering::SeqCst);
        super::async_ops::notify_completion(&reply);

        let waiter = reply.waiting_task.load(Ordering::SeqCst);
        if waiter != 0 {
            unsafe {
                crate::sched::wake_task_erased(waiter);
            }
        }
    }
}
