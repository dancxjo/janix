//! Root service message handlers, organized by domain.

pub mod batch;
pub mod bytespace;
pub mod debug;
pub mod encode;
pub mod graph;
pub mod logging;
pub mod stream;

pub use bytespace::*;
pub use debug::*;
pub use graph::*;
pub use logging::*;
pub use stream::*;
pub mod watch;
pub use watch::*;

/// Common handler result type: (status, value)
pub type HandlerResult = (i32, u64);
