//! Root service main loop and message dispatch.

use super::symbols::Interner;
use super::{RootMsg, RootOp};
use crate::BootRuntime;
use crate::root::handlers as root_handlers;
use crate::root::handlers::bytespace::BytespaceManager;
use core::sync::atomic::Ordering;

pub extern "C" fn root_main<R: BootRuntime>(_arg: usize) -> ! {
    // DIAG: raw serial marker - bypasses log infrastructure entirely
    crate::runtime_base().putchar(b'R');
    crate::runtime_base().putchar(b'M');
    crate::runtime_base().putchar(b'\n');
    crate::kinfo!("ROOT: started once");

    let mut bytespaces = BytespaceManager::new();
    let mut interner = Interner::new();

    let mut iteration = 0u64;
    loop {
        iteration = iteration.wrapping_add(1);
        let mut processed_this_round = 0;

        // Process messages until queue is empty or we hit a safety limit.
        // We use a larger limit (128) than before to improve throughput for logging storms.
        while processed_this_round < 128 {
            if let Some(msg) = super::pop_msg() {
                handle_msg::<R>(&mut bytespaces, &mut interner, msg);
                processed_this_round += 1;
            } else {
                break;
            }
        }

        // Periodic memory stats (check once per round if it's time)
        if iteration % 1000 == 0 {
            let symbol_count = interner.names.len();
            let bytespaces_count = bytespaces.spaces.len();
            crate::kinfo!(
                "ROOT STATS: iter={} bytespaces={} symbols={} drops={}",
                iteration,
                bytespaces_count,
                symbol_count,
                super::inbox_drop_count()
            );
        }

        if processed_this_round == 0 {
            // Signal intent to sleep
            super::ROOT_ASLEEP.store(true, Ordering::Release);

            // Double check queue to avoid missed wakeups race condition
            // (a message could have arrived just *after* we finished pop_msg
            // but *before* we set ROOT_ASLEEP)
            if super::queue_len() > 0 {
                super::ROOT_ASLEEP.store(false, Ordering::Release);
                continue;
            }

            // Only block if we truly ran out of work
            unsafe {
                crate::task::block_current_erased();
            }
        } else if processed_this_round >= 128 {
            // If we hit the limit, there might be more but we should yield to let other
            // high-priority tasks run (like the compositor or driver) then resume.
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
        RootOp::BytespaceCreate { .. } => "BytespaceCreate",
        RootOp::BytespaceInfo { .. } => "BytespaceInfo",
        RootOp::BytespaceMap { .. } => "BytespaceMap",
        RootOp::BytespaceUnmap { .. } => "BytespaceUnmap",
        RootOp::BytespacePhys { .. } => "BytespacePhys",
        RootOp::BytespaceRead { .. } => "BytespaceRead",
        RootOp::BytespaceWrite { .. } => "BytespaceWrite",
        RootOp::BytespaceCreateFromPtr { .. } => "BytespaceCreateFromPtr",
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
        RootOp::BytespaceTruncate { .. } => "BytespaceTruncate",
        RootOp::ResolvePath { .. } => "ResolvePath",
        RootOp::Unlink { .. } => "Unlink",
        RootOp::DirList { .. } => "DirList",
        RootOp::OrphanThing { .. } => "OrphanThing",
        RootOp::CleanupTaskThings { .. } => "CleanupTaskThings",
    }
}

fn handle_msg<R: BootRuntime>(
    bytespaces: &mut BytespaceManager,
    interner: &mut Interner,
    msg: RootMsg,
) {
    let (status, value) = match msg.op {
        RootOp::Intern { name } => root_handlers::handle_intern(interner, &name),

        RootOp::BytespaceCreate { len, flags, format } => {
            root_handlers::bytespace::handle_bytespace_create::<R>(bytespaces, len, flags, format)
        }
        RootOp::BytespaceCreateFromPtr { ptr, len } => {
            root_handlers::bytespace::handle_bytespace_create_from_ptr::<R>(bytespaces, ptr, len)
        }
        RootOp::BytespaceWrite {
            id,
            offset,
            ptr,
            len,
        } => root_handlers::bytespace::handle_bytespace_write(bytespaces, id, offset, ptr, len),
        RootOp::BytespaceRead {
            id,
            offset,
            ptr,
            len,
        } => root_handlers::bytespace::handle_bytespace_read(bytespaces, id, offset, ptr, len),
        RootOp::BytespaceInfo { id } => {
            root_handlers::bytespace::handle_bytespace_info(bytespaces, &msg, id)
        }
        RootOp::BytespaceMap { id, tid } => {
            root_handlers::bytespace::handle_bytespace_map(bytespaces, &msg, id, tid)
        }
        RootOp::BytespaceUnmap { id, user_va, tid } => {
            root_handlers::bytespace::handle_bytespace_unmap(id, user_va, tid)
        }
        RootOp::BytespacePhys { id } => {
            root_handlers::bytespace::handle_bytespace_phys(bytespaces, &msg, id)
        }
        RootOp::BytespaceTruncate { id, new_len } => {
            root_handlers::bytespace::handle_bytespace_truncate(bytespaces, id, new_len)
        }

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
        | RootOp::ApplyBatch { .. } => (-38, 0), // ENOSYS
    };

    if let Some(reply) = msg.reply {
        reply.status.store(status, Ordering::Relaxed);
        reply.value.store(value, Ordering::Relaxed);
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
