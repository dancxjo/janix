//! VirtIO userspace driver library
//!
//! This crate provides a reusable VirtIO device abstraction that can be used
//! by device-specific drivers (GPU, NET, etc.).
//!
//! # Example
//! ```ignore
//! use virtio::{VirtioDevice, VIRTIO_STATUS_DRIVER_OK};
//!
//! let mut dev = VirtioDevice::new(device_id)?;
//! dev.init(desired_features)?;
//! dev.setup_queue(0, 128)?;
//! dev.driver_ok();
//! ```

#![no_std]

extern crate alloc;

pub mod constants;
pub mod device;
pub mod virtqueue;

pub use constants::*;
pub use device::VirtioDevice;
pub use virtqueue::Virtqueue;
