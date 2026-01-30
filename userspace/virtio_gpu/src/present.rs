//! VirtIO GPU presentation model - DisplaySurface, FramePool, PresentQueue
//!
//! This module replaces the old "swapchain" concept with a cleaner presentation
//! model that better matches VirtIO GPU 2D semantics and graph-style rendering.
//!
//! # Architecture
//!
//! - **DisplaySurface**: Represents a display output (scanout/head) with mode info
//! - **FrameResource**: A GPU resource + backing memory that can hold a rendered frame
//! - **FramePool**: Manages a ring of FrameResources (triple-buffer by default)
//! - **PresentQueue**: Handles presenting frames with damage tracking and completion
//!
//! # Present Flow
//!
//! 1. Acquire a free frame from the pool: `pool.acquire_frame() -> FrameHandle`
//! 2. Render into the frame's backing memory
//! 3. Present with damage info: `queue.present(surface, frame, damage_rects)`
//! 4. Driver sends VirtIO commands (TRANSFER_TO_HOST_2D, RESOURCE_FLUSH, SET_SCANOUT)
//! 5. Frame becomes in-flight, returns to pool after completion

use alloc::vec::Vec;
use crate::Rect;

/// Frame state machine
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameState {
    /// Frame is available for acquisition
    Free,
    /// Frame has been acquired for rendering
    Acquired,
    /// Frame has been presented and is in-flight (GPU processing)
    /// The u64 is a present sequence number for tracking
    InFlight(u64),
}

/// A single frame resource (GPU resource + backing memory)
pub struct FrameResource {
    /// VirtIO GPU resource ID
    pub resource_id: u32,
    /// Physical address of backing memory
    pub phys_addr: u64,
    /// Bytespace ID for this frame
    pub bytespace_id: u64,
    /// Size of backing memory in bytes
    pub size: usize,
    /// Width in pixels
    pub width: u32,
    /// Height in pixels
    pub height: u32,
    /// Stride in bytes
    pub stride: u32,
    /// Pixel format (e.g., BGRA8888)
    pub format: u32,
    /// Virtual address for CPU mapping (0 if not mapped)
    pub virt_addr: u64,
    /// Current state of this frame
    pub state: FrameState,
    /// Last present sequence when this frame was presented
    pub last_present_seq: u64,
}

impl FrameResource {
    /// Create a new frame resource
    pub fn new(
        resource_id: u32,
        phys_addr: u64,
        bytespace_id: u64,
        size: usize,
        width: u32,
        height: u32,
        stride: u32,
        format: u32,
        virt_addr: u64,
    ) -> Self {
        Self {
            resource_id,
            phys_addr,
            bytespace_id,
            size,
            width,
            height,
            stride,
            format,
            virt_addr,
            state: FrameState::Free,
            last_present_seq: 0,
        }
    }

    /// Check if this frame is free to acquire
    pub fn is_free(&self) -> bool {
        self.state == FrameState::Free
    }

    /// Mark frame as acquired
    pub fn mark_acquired(&mut self) {
        self.state = FrameState::Acquired;
    }

    /// Mark frame as in-flight with a present sequence number
    pub fn mark_in_flight(&mut self, seq: u64) {
        self.state = FrameState::InFlight(seq);
        self.last_present_seq = seq;
    }

    /// Mark frame as free (ready for reuse)
    pub fn mark_free(&mut self) {
        self.state = FrameState::Free;
    }
}

/// Display surface (scanout/head configuration)
pub struct DisplaySurface {
    /// Scanout ID (usually 0 for primary display)
    pub scanout_id: u32,
    /// Current display width
    pub width: u32,
    /// Current display height
    pub height: u32,
    /// Pixel format
    pub format: u32,
    /// Currently scanned-out resource (None if not set)
    pub current_resource: Option<u32>,
}

impl DisplaySurface {
    /// Create a new display surface
    pub fn new(scanout_id: u32, width: u32, height: u32, format: u32) -> Self {
        Self {
            scanout_id,
            width,
            height,
            format,
            current_resource: None,
        }
    }

    /// Update the mode (resolution) of this surface
    pub fn update_mode(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        // Mode change invalidates current scanout
        self.current_resource = None;
    }
}

/// Handle to an acquired frame
pub struct FrameHandle {
    /// Index into the frame pool
    pub index: usize,
}

/// Frame pool - manages a ring of presentable frame resources
pub struct FramePool {
    /// The frame resources
    frames: Vec<FrameResource>,
    /// Next frame index to try acquiring
    next_acquire_idx: usize,
}

impl FramePool {
    /// Create a new frame pool with the given frames
    pub fn new(frames: Vec<FrameResource>) -> Self {
        Self {
            frames,
            next_acquire_idx: 0,
        }
    }

    /// Acquire a free frame for rendering
    ///
    /// Returns None if no frames are currently available.
    /// This can happen if all frames are in-flight.
    pub fn acquire_frame(&mut self) -> Option<FrameHandle> {
        // Try to find a free frame starting from next_acquire_idx
        for i in 0..self.frames.len() {
            let idx = (self.next_acquire_idx + i) % self.frames.len();
            if self.frames[idx].is_free() {
                self.frames[idx].mark_acquired();
                self.next_acquire_idx = (idx + 1) % self.frames.len();
                return Some(FrameHandle { index: idx });
            }
        }
        None
    }

    /// Get a frame resource by handle (immutable)
    pub fn get_frame(&self, handle: &FrameHandle) -> Option<&FrameResource> {
        self.frames.get(handle.index)
    }

    /// Get a frame resource by handle (mutable)
    pub fn get_frame_mut(&mut self, handle: &FrameHandle) -> Option<&mut FrameResource> {
        self.frames.get_mut(handle.index)
    }

    /// Release a frame back to the pool (mark as free)
    ///
    /// Called internally by PresentQueue when a frame completes
    pub fn release_frame(&mut self, index: usize) {
        if let Some(frame) = self.frames.get_mut(index) {
            frame.mark_free();
        }
    }

    /// Get the number of frames in the pool
    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }

    /// Calculate buffer age for a frame at the given index
    ///
    /// Returns 0 if never presented, or the number of presents since this buffer was last used
    pub fn buffer_age(&self, index: usize, current_seq: u64) -> u32 {
        if let Some(frame) = self.frames.get(index) {
            if frame.last_present_seq == 0 {
                0 // Never presented
            } else {
                current_seq.saturating_sub(frame.last_present_seq) as u32
            }
        } else {
            0
        }
    }

    /// Find a frame by resource ID
    pub fn find_frame_by_resource(&self, resource_id: u32) -> Option<usize> {
        self.frames.iter().position(|f| f.resource_id == resource_id)
    }
}

/// Pending present operation
struct PendingPresent {
    /// Frame index in the pool
    frame_index: usize,
    /// Present sequence number
    sequence: u64,
}

/// Present queue - manages presenting frames to the display
pub struct PresentQueue {
    /// Pending presents (frames in-flight)
    pending: Vec<PendingPresent>,
    /// Next present sequence number
    next_sequence: u64,
    /// Conservative in-flight limit (fallback when fences aren't available)
    /// Only allow this many frames to be in-flight at once
    max_in_flight: usize,
}

impl PresentQueue {
    /// Create a new present queue
    ///
    /// `max_in_flight`: Maximum number of frames that can be in-flight simultaneously.
    /// Set to 2 for conservative double-buffering behavior without fences.
    pub fn new(max_in_flight: usize) -> Self {
        Self {
            pending: Vec::new(),
            next_sequence: 1,
            max_in_flight,
        }
    }

    /// Check if we can present (haven't hit the in-flight limit)
    pub fn can_present(&self) -> bool {
        self.pending.len() < self.max_in_flight
    }

    /// Enqueue a frame for presentation
    ///
    /// Returns the present sequence number.
    /// The caller should then send VirtIO GPU commands (TRANSFER, FLUSH, SET_SCANOUT).
    ///
    /// The frame will be marked as in-flight in the pool.
    pub fn enqueue_present(&mut self, pool: &mut FramePool, handle: FrameHandle) -> u64 {
        let seq = self.next_sequence;
        self.next_sequence += 1;

        // Mark frame as in-flight
        if let Some(frame) = pool.get_frame_mut(&handle) {
            frame.mark_in_flight(seq);
        }

        // Add to pending list
        self.pending.push(PendingPresent {
            frame_index: handle.index,
            sequence: seq,
        });

        seq
    }

    /// Complete the oldest pending present
    ///
    /// Call this when you've received confirmation that the GPU has processed
    /// the present (e.g., after virtqueue response or a conservative timer).
    ///
    /// Returns the frame index that was completed, or None if no presents are pending.
    pub fn complete_oldest(&mut self, pool: &mut FramePool) -> Option<usize> {
        if let Some(pending) = self.pending.first() {
            let frame_idx = pending.frame_index;
            self.pending.remove(0);
            pool.release_frame(frame_idx);
            Some(frame_idx)
        } else {
            None
        }
    }

    /// Complete all pending presents (for shutdown or drain)
    pub fn drain(&mut self, pool: &mut FramePool) {
        for pending in self.pending.drain(..) {
            pool.release_frame(pending.frame_index);
        }
    }

    /// Get the number of pending presents
    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }

    /// Get the current sequence number
    pub fn current_sequence(&self) -> u64 {
        self.next_sequence.saturating_sub(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn frame_resource_state_machine() {
        let mut frame = FrameResource::new(1, 0x1000, 0, 4096, 100, 100, 400, 1, 0);
        assert_eq!(frame.state, FrameState::Free);
        assert!(frame.is_free());

        frame.mark_acquired();
        assert_eq!(frame.state, FrameState::Acquired);
        assert!(!frame.is_free());

        frame.mark_in_flight(42);
        assert_eq!(frame.state, FrameState::InFlight(42));
        assert_eq!(frame.last_present_seq, 42);
        assert!(!frame.is_free());

        frame.mark_free();
        assert_eq!(frame.state, FrameState::Free);
        assert!(frame.is_free());
    }

    #[test]
    fn frame_pool_acquire_release() {
        let frames = vec![
            FrameResource::new(1, 0x1000, 0, 4096, 100, 100, 400, 1, 0),
            FrameResource::new(2, 0x2000, 0, 4096, 100, 100, 400, 1, 0),
            FrameResource::new(3, 0x3000, 0, 4096, 100, 100, 400, 1, 0),
        ];

        let mut pool = FramePool::new(frames);
        assert_eq!(pool.frame_count(), 3);

        // Acquire first frame
        let handle1 = pool.acquire_frame().expect("should acquire frame 1");
        assert_eq!(handle1.index, 0);

        // Acquire second frame
        let handle2 = pool.acquire_frame().expect("should acquire frame 2");
        assert_eq!(handle2.index, 1);

        // Release first frame
        pool.release_frame(handle1.index);

        // Acquire third frame (skips released frame for now due to round-robin)
        let handle3 = pool.acquire_frame().expect("should acquire frame 3");
        assert_eq!(handle3.index, 2);

        // Acquire the released frame (wrapped around)
        let handle4 = pool.acquire_frame().expect("should acquire released frame");
        assert_eq!(handle4.index, 0);

        // Now all frames are acquired, should return None
        assert!(pool.acquire_frame().is_none());
    }

    #[test]
    fn present_queue_basic_flow() {
        let frames = vec![
            FrameResource::new(1, 0x1000, 0, 4096, 100, 100, 400, 1, 0),
            FrameResource::new(2, 0x2000, 0, 4096, 100, 100, 400, 1, 0),
        ];

        let mut pool = FramePool::new(frames);
        let mut queue = PresentQueue::new(2);

        assert!(queue.can_present());
        assert_eq!(queue.pending_count(), 0);

        // Acquire and present first frame
        let handle1 = pool.acquire_frame().unwrap();
        let seq1 = queue.enqueue_present(&mut pool, handle1);
        assert_eq!(seq1, 1);
        assert_eq!(queue.pending_count(), 1);

        // Acquire and present second frame
        let handle2 = pool.acquire_frame().unwrap();
        let seq2 = queue.enqueue_present(&mut pool, handle2);
        assert_eq!(seq2, 2);
        assert_eq!(queue.pending_count(), 2);

        // Hit the limit
        assert!(!queue.can_present());

        // Complete oldest
        let completed = queue.complete_oldest(&mut pool);
        assert_eq!(completed, Some(0));
        assert_eq!(queue.pending_count(), 1);

        // Can present again
        assert!(queue.can_present());
    }

    #[test]
    fn buffer_age_calculation() {
        let frames = vec![
            FrameResource::new(1, 0x1000, 0, 4096, 100, 100, 400, 1, 0),
            FrameResource::new(2, 0x2000, 0, 4096, 100, 100, 400, 1, 0),
        ];

        let mut pool = FramePool::new(frames);
        let mut queue = PresentQueue::new(2);

        // Initially, buffer age is 0 (never presented)
        assert_eq!(pool.buffer_age(0, 0), 0);

        // Acquire and present
        let handle = pool.acquire_frame().unwrap();
        queue.enqueue_present(&mut pool, handle);

        // Complete it
        queue.complete_oldest(&mut pool);

        // After 1 present, age is 1 for present_seq=2
        assert_eq!(pool.buffer_age(0, 2), 1);

        // After 5 presents, age is 5 for present_seq=6
        assert_eq!(pool.buffer_age(0, 6), 5);
    }
}
