//! Device Registry for capability-based device claiming
//! 
//! This module tracks claimable devices and their allowed I/O port ranges
//! and MMIO BARs. When a task claims a device, it receives a handle that
//! authorizes resource access within the device's declared ranges.

use spin::Mutex;

/// Maximum number of devices in the registry
const MAX_DEVICES: usize = 16;

/// Maximum number of claimed devices across all tasks
const MAX_CLAIMS: usize = 32;

/// Maximum BARs per device
const MAX_BARS: usize = 6;

/// A device entry in the registry
#[derive(Clone, Copy)]
pub struct DeviceEntry {
    pub kind: &'static str,
    pub ioport_ranges: &'static [(u16, u16)], // (start, end) inclusive
    pub graph_id: u64, // ThingId in the graph
    pub mmio_bars: [u64; MAX_BARS],   // BAR physical addresses
    pub mmio_sizes: [u64; MAX_BARS],  // BAR sizes
}

impl DeviceEntry {
    pub const fn new_legacy(kind: &'static str, ioport_ranges: &'static [(u16, u16)], graph_id: u64) -> Self {
        Self {
            kind,
            ioport_ranges,
            graph_id,
            mmio_bars: [0; MAX_BARS],
            mmio_sizes: [0; MAX_BARS],
        }
    }

    pub const fn new_mmio(kind: &'static str, graph_id: u64, bars: [u64; MAX_BARS], sizes: [u64; MAX_BARS]) -> Self {
        Self {
            kind,
            ioport_ranges: &[],
            graph_id,
            mmio_bars: bars,
            mmio_sizes: sizes,
        }
    }
}

/// DMA buffer allocation for a claim
#[derive(Clone, Copy, Default)]
pub struct DmaBuffer {
    pub phys_addr: u64,
    pub virt_addr: u64,
    pub page_count: usize,
    pub valid: bool,
}

/// A claimed device
#[derive(Clone, Copy, Default)]
pub struct ClaimedDevice {
    pub device_index: usize,
    pub task_id: u64,
    pub valid: bool,
    pub mapped_bar_virt: [u64; MAX_BARS], // Virtual addresses of mapped BARs
    pub dma_buffers: [DmaBuffer; 4],      // Up to 4 DMA buffers per claim
}

/// Global device registry
pub static REGISTRY: Mutex<DeviceRegistry> = Mutex::new(DeviceRegistry::new());

pub struct DeviceRegistry {
    devices: [Option<DeviceEntry>; MAX_DEVICES],
    device_count: usize,
    claims: [ClaimedDevice; MAX_CLAIMS],
}

impl DeviceRegistry {
    pub const fn new() -> Self {
        Self {
            devices: [None; MAX_DEVICES],
            device_count: 0,
            claims: [ClaimedDevice { 
                device_index: 0, 
                task_id: 0, 
                valid: false,
                mapped_bar_virt: [0; MAX_BARS],
                dma_buffers: [DmaBuffer { phys_addr: 0, virt_addr: 0, page_count: 0, valid: false }; 4],
            }; MAX_CLAIMS],
        }
    }

    /// Register a device in the registry. Returns device index.
    pub fn register(&mut self, entry: DeviceEntry) -> Option<usize> {
        if self.device_count >= MAX_DEVICES {
            return None;
        }
        let idx = self.device_count;
        self.devices[idx] = Some(entry);
        self.device_count += 1;
        Some(idx)
    }

    /// Get device by index
    pub fn get(&self, index: usize) -> Option<&DeviceEntry> {
        if index < self.device_count {
            self.devices[index].as_ref()
        } else {
            None
        }
    }

    /// Find device by graph ID
    pub fn find_by_graph_id(&self, graph_id: u64) -> Option<usize> {
        for i in 0..self.device_count {
            if let Some(entry) = &self.devices[i] {
                if entry.graph_id == graph_id {
                    return Some(i);
                }
            }
        }
        None
    }

    /// Claim a device for a task. Returns claim handle (index) or None if already claimed.
    pub fn claim(&mut self, device_index: usize, task_id: u64) -> Option<usize> {
        // Check device exists
        if device_index >= self.device_count || self.devices[device_index].is_none() {
            return None;
        }

        // Check not already claimed
        for claim in &self.claims {
            if claim.valid && claim.device_index == device_index {
                // Already claimed
                return None;
            }
        }

        // Find free claim slot
        for (i, claim) in self.claims.iter_mut().enumerate() {
            if !claim.valid {
                claim.device_index = device_index;
                claim.task_id = task_id;
                claim.valid = true;
                claim.mapped_bar_virt = [0; MAX_BARS];
                claim.dma_buffers = [DmaBuffer::default(); 4];
                return Some(i);
            }
        }

        None // No free slots
    }

    /// Get BAR info for a claimed device
    pub fn get_bar_info(&self, claim_handle: usize, bar_index: usize) -> Option<(u64, u64)> {
        if claim_handle >= MAX_CLAIMS || bar_index >= MAX_BARS {
            return None;
        }
        let claim = &self.claims[claim_handle];
        if !claim.valid {
            return None;
        }
        
        if let Some(device) = self.get(claim.device_index) {
            let addr = device.mmio_bars[bar_index];
            let size = device.mmio_sizes[bar_index];
            if addr != 0 && size != 0 {
                return Some((addr, size));
            }
        }
        None
    }

    /// Record a BAR mapping for a claim
    pub fn set_bar_mapping(&mut self, claim_handle: usize, bar_index: usize, virt_addr: u64) {
        if claim_handle < MAX_CLAIMS && bar_index < MAX_BARS {
            self.claims[claim_handle].mapped_bar_virt[bar_index] = virt_addr;
        }
    }

    /// Allocate DMA buffer tracking slot
    pub fn alloc_dma_slot(&mut self, claim_handle: usize, phys: u64, virt: u64, pages: usize) -> Option<usize> {
        if claim_handle >= MAX_CLAIMS {
            return None;
        }
        let claim = &mut self.claims[claim_handle];
        if !claim.valid {
            return None;
        }
        
        for (i, buf) in claim.dma_buffers.iter_mut().enumerate() {
            if !buf.valid {
                buf.phys_addr = phys;
                buf.virt_addr = virt;
                buf.page_count = pages;
                buf.valid = true;
                return Some(i);
            }
        }
        None
    }

    /// Check if a port access is authorized for a given claim handle
    pub fn check_port_access(&self, claim_handle: usize, port: u16) -> bool {
        if claim_handle >= MAX_CLAIMS {
            return false;
        }
        let claim = &self.claims[claim_handle];
        if !claim.valid {
            return false;
        }
        
        if let Some(device) = self.get(claim.device_index) {
            for &(start, end) in device.ioport_ranges {
                if port >= start && port <= end {
                    return true;
                }
            }
        }
        false
    }

    /// Get claim by handle and verify task ownership
    pub fn verify_claim(&self, claim_handle: usize, task_id: u64) -> bool {
        if claim_handle >= MAX_CLAIMS {
            return false;
        }
        let claim = &self.claims[claim_handle];
        claim.valid && claim.task_id == task_id
    }

    /// Release a claim
    pub fn release(&mut self, claim_handle: usize, task_id: u64) -> bool {
        if claim_handle >= MAX_CLAIMS {
            return false;
        }
        let claim = &mut self.claims[claim_handle];
        if claim.valid && claim.task_id == task_id {
            claim.valid = false;
            true
        } else {
            false
        }
    }
}

// Static device definitions for legacy devices
pub static CMOS_IOPORT_RANGES: &[(u16, u16)] = &[(0x70, 0x71)];
pub static PS2_IOPORT_RANGES: &[(u16, u16)] = &[(0x60, 0x64)];
