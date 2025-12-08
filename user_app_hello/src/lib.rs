#![no_std]

use abi::{KernelRequest, KernelResponse, NodeId};
use userland_rt::Sys;

pub fn run<S: Sys>(sys: &S) {
    // Log a hello message into the kernel log
    let _ = sys.syscall(KernelRequest::Log {
        message: "Hello from user_app_hello (kernel-side)!",
    });

    // Query some nodes in the graph
    for i in 0..5 {
        let node_id = NodeId(i);
        let resp = sys.syscall(KernelRequest::GraphQuery { node_id });
        match resp {
            KernelResponse::NodeData { value: _, .. } => {
                let _ = sys.syscall(KernelRequest::Log {
                    message: "Queried node in user_app_hello",
                });
                // (We don't have string formatting here; just log a generic line.)
            }
            _ => {
                let _ = sys.syscall(KernelRequest::Log {
                    message: "Node not found in user_app_hello",
                });
            }
        }
    }

    // Create and commit a transaction
    let resp = sys.syscall(KernelRequest::CreateTransaction);
    if let KernelResponse::TransactionCreated { tx_id } = resp {
        let _ = sys.syscall(KernelRequest::Log {
            message: "Transaction created in user_app_hello",
        });

        let commit_resp = sys.syscall(KernelRequest::CommitTransaction { tx_id });
        if matches!(commit_resp, KernelResponse::Success { .. }) {
            let _ = sys.syscall(KernelRequest::Log {
                message: "Transaction committed in user_app_hello",
            });
        } else {
            let _ = sys.syscall(KernelRequest::Log {
                message: "Failed to commit transaction in user_app_hello",
            });
        }
    } else {
        let _ = sys.syscall(KernelRequest::Log {
            message: "Failed to create transaction in user_app_hello",
        });
    }

    let _ = sys.syscall(KernelRequest::Log {
        message: "Goodbye from user_app_hello (kernel-side)!",
    });
}
