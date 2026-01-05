//! xHCI (USB 3.0) Host Controller Driver
//!
//! This crate provides a minimal xHCI driver for DMA bring-up.
//! It implements controller initialization and NO-OP command submission.

#![no_std]

use core::ptr::{read_volatile, write_volatile};

// ============================================================================
// TRB (Transfer Request Block) Definitions
// ============================================================================

/// TRB types (6-bit field in TRB[3].type)
pub mod trb_type {
    pub const NORMAL: u8 = 1;
    pub const SETUP_STAGE: u8 = 2;
    pub const DATA_STAGE: u8 = 3;
    pub const STATUS_STAGE: u8 = 4;
    pub const LINK: u8 = 6;
    pub const NO_OP_CMD: u8 = 23;
    pub const ENABLE_SLOT: u8 = 9;
    pub const COMMAND_COMPLETION: u8 = 33;
    pub const PORT_STATUS_CHANGE: u8 = 34;
}

/// Completion codes
pub mod completion_code {
    pub const INVALID: u8 = 0;
    pub const SUCCESS: u8 = 1;
    pub const DATA_BUFFER_ERROR: u8 = 2;
    pub const BABBLE_DETECTED: u8 = 3;
    pub const TRB_ERROR: u8 = 5;
    pub const SHORT_PACKET: u8 = 13;
}

/// 16-byte Transfer Request Block
#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, Default)]
pub struct Trb {
    pub param_lo: u32,
    pub param_hi: u32,
    pub status: u32,
    pub control: u32,
}

impl Trb {
    pub const fn new() -> Self {
        Self { param_lo: 0, param_hi: 0, status: 0, control: 0 }
    }

    /// Create a NO-OP Command TRB
    pub fn no_op_cmd(cycle: bool) -> Self {
        Self {
            param_lo: 0,
            param_hi: 0,
            status: 0,
            control: ((trb_type::NO_OP_CMD as u32) << 10) | (cycle as u32),
        }
    }

    /// Create a Link TRB pointing to the start of the ring
    pub fn link(ring_phys: u64, cycle: bool) -> Self {
        Self {
            param_lo: ring_phys as u32,
            param_hi: (ring_phys >> 32) as u32,
            status: 0,
            // Toggle cycle bit on link traversal
            control: ((trb_type::LINK as u32) << 10) | (1 << 1) | (cycle as u32),
        }
    }

    pub fn trb_type(&self) -> u8 {
        ((self.control >> 10) & 0x3F) as u8
    }

    pub fn cycle_bit(&self) -> bool {
        (self.control & 1) != 0
    }

    pub fn completion_code(&self) -> u8 {
        ((self.status >> 24) & 0xFF) as u8
    }
}

// ============================================================================
// Event Ring Segment Table Entry
// ============================================================================

#[repr(C, align(64))]
#[derive(Clone, Copy, Debug, Default)]
pub struct ErstEntry {
    pub ring_segment_base: u64,
    pub ring_segment_size: u16,
    pub reserved: u16,
    pub reserved2: u32,
}

// ============================================================================
// xHCI Capability Registers
// ============================================================================

#[repr(C)]
pub struct CapRegs {
    pub caplength: u8,
    pub reserved: u8,
    pub hciversion: u16,
    pub hcsparams1: u32,
    pub hcsparams2: u32,
    pub hcsparams3: u32,
    pub hccparams1: u32,
    pub dboff: u32,
    pub rtsoff: u32,
    pub hccparams2: u32,
}

impl CapRegs {
    pub unsafe fn read(base: *const u8) -> CapRegsData {
        CapRegsData {
            caplength: read_volatile(base),
            hciversion: read_volatile(base.add(2) as *const u16),
            hcsparams1: read_volatile(base.add(4) as *const u32),
            hcsparams2: read_volatile(base.add(8) as *const u32),
            hccparams1: read_volatile(base.add(0x10) as *const u32),
            dboff: read_volatile(base.add(0x14) as *const u32),
            rtsoff: read_volatile(base.add(0x18) as *const u32),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct CapRegsData {
    pub caplength: u8,
    pub hciversion: u16,
    pub hcsparams1: u32,
    pub hcsparams2: u32,
    pub hccparams1: u32,
    pub dboff: u32,
    pub rtsoff: u32,
}

impl CapRegsData {
    pub fn max_slots(&self) -> u8 {
        (self.hcsparams1 & 0xFF) as u8
    }

    pub fn max_ports(&self) -> u8 {
        ((self.hcsparams1 >> 24) & 0xFF) as u8
    }
}

// ============================================================================
// xHCI Operational Registers
// ============================================================================

/// Offsets within operational register space
pub mod op_reg {
    pub const USBCMD: usize = 0x00;
    pub const USBSTS: usize = 0x04;
    pub const PAGESIZE: usize = 0x08;
    pub const DNCTRL: usize = 0x14;
    pub const CRCR: usize = 0x18;
    pub const DCBAAP: usize = 0x30;
    pub const CONFIG: usize = 0x38;
}

/// USBCMD bits
pub mod usbcmd {
    pub const RUN_STOP: u32 = 1 << 0;
    pub const HCRST: u32 = 1 << 1;
    pub const INTE: u32 = 1 << 2;
    pub const HSEE: u32 = 1 << 3;
}

/// USBSTS bits
pub mod usbsts {
    pub const HCH: u32 = 1 << 0;      // Host Controller Halted
    pub const HSE: u32 = 1 << 2;      // Host System Error
    pub const EINT: u32 = 1 << 3;     // Event Interrupt
    pub const PCD: u32 = 1 << 4;      // Port Change Detect
    pub const CNR: u32 = 1 << 11;     // Controller Not Ready
}

// ============================================================================
// xHCI Runtime Registers
// ============================================================================

/// Offsets within runtime register space
pub mod rt_reg {
    pub const MFINDEX: usize = 0x00;
    // Interrupter registers start at offset 0x20
    pub const IR0: usize = 0x20;
}

/// Offsets within an interrupter register set (32 bytes each)
pub mod ir_reg {
    pub const IMAN: usize = 0x00;
    pub const IMOD: usize = 0x04;
    pub const ERSTSZ: usize = 0x08;
    pub const RESERVED: usize = 0x0C;
    pub const ERSTBA: usize = 0x10;  // 64-bit
    pub const ERDP: usize = 0x18;    // 64-bit
}

/// IMAN bits
pub mod iman {
    pub const IP: u32 = 1 << 0;  // Interrupt Pending
    pub const IE: u32 = 1 << 1;  // Interrupt Enable
}

// ============================================================================
// xHCI Controller
// ============================================================================

pub struct XhciController {
    mmio_base: *mut u8,
    cap: CapRegsData,
    op_base: *mut u8,
    rt_base: *mut u8,
    db_base: *mut u8,
}

impl XhciController {
    /// Create a new controller from MMIO base address
    ///
    /// # Safety
    /// The MMIO base must be a valid mapped address to xHCI registers.
    pub unsafe fn new(mmio_base: *mut u8) -> Self {
        let cap = CapRegs::read(mmio_base);
        let op_base = mmio_base.add(cap.caplength as usize);
        let rt_base = mmio_base.add(cap.rtsoff as usize);
        let db_base = mmio_base.add(cap.dboff as usize);

        Self {
            mmio_base,
            cap,
            op_base,
            rt_base,
            db_base,
        }
    }

    pub fn cap(&self) -> &CapRegsData {
        &self.cap
    }

    // --- Register access helpers ---

    unsafe fn read_op(&self, offset: usize) -> u32 {
        read_volatile(self.op_base.add(offset) as *const u32)
    }

    unsafe fn write_op(&self, offset: usize, val: u32) {
        write_volatile(self.op_base.add(offset) as *mut u32, val);
    }

    unsafe fn read_op64(&self, offset: usize) -> u64 {
        read_volatile(self.op_base.add(offset) as *const u64)
    }

    unsafe fn write_op64(&self, offset: usize, val: u64) {
        write_volatile(self.op_base.add(offset) as *mut u64, val);
    }

    unsafe fn write_rt64(&self, offset: usize, val: u64) {
        write_volatile(self.rt_base.add(offset) as *mut u64, val);
    }

    unsafe fn read_rt(&self, offset: usize) -> u32 {
        read_volatile(self.rt_base.add(offset) as *const u32)
    }

    unsafe fn write_rt(&self, offset: usize, val: u32) {
        write_volatile(self.rt_base.add(offset) as *mut u32, val);
    }

    // --- High-level operations ---

    /// Wait for controller to be ready (CNR = 0)
    pub fn wait_ready(&self) -> bool {
        for _ in 0..10000 {
            let sts = unsafe { self.read_op(op_reg::USBSTS) };
            if (sts & usbsts::CNR) == 0 {
                return true;
            }
            core::hint::spin_loop();
        }
        false
    }

    /// Stop the controller (set RUN=0, wait for HCH=1)
    pub fn stop(&self) -> bool {
        unsafe {
            let cmd = self.read_op(op_reg::USBCMD);
            self.write_op(op_reg::USBCMD, cmd & !usbcmd::RUN_STOP);
        }
        for _ in 0..10000 {
            let sts = unsafe { self.read_op(op_reg::USBSTS) };
            if (sts & usbsts::HCH) != 0 {
                return true;
            }
            core::hint::spin_loop();
        }
        false
    }

    /// Reset the controller (set HCRST=1, wait for HCRST=0 and CNR=0)
    pub fn reset(&self) -> bool {
        unsafe {
            self.write_op(op_reg::USBCMD, usbcmd::HCRST);
        }
        // Wait for HCRST to clear
        for _ in 0..100000 {
            let cmd = unsafe { self.read_op(op_reg::USBCMD) };
            if (cmd & usbcmd::HCRST) == 0 {
                return self.wait_ready();
            }
            core::hint::spin_loop();
        }
        false
    }

    /// Set the max device slots enabled
    pub fn set_max_slots(&self, slots: u8) {
        unsafe {
            self.write_op(op_reg::CONFIG, slots as u32);
        }
    }

    /// Set DCBAAP (Device Context Base Address Array Pointer)
    pub fn set_dcbaap(&self, phys: u64) {
        unsafe {
            self.write_op64(op_reg::DCBAAP, phys);
        }
    }

    /// Set CRCR (Command Ring Control Register)
    /// phys should be 64-byte aligned, RCS=1 sets initial cycle state
    pub fn set_crcr(&self, phys: u64, rcs: bool) {
        unsafe {
            self.write_op64(op_reg::CRCR, phys | (rcs as u64));
        }
    }

    /// Configure Interrupter 0 with event ring
    pub fn setup_interrupter(
        &self,
        erst_phys: u64,
        erst_size: u16,
        event_ring_phys: u64,
    ) {
        let ir0 = rt_reg::IR0;
        unsafe {
            // Set ERSTSZ (number of segments)
            self.write_rt(ir0 + ir_reg::ERSTSZ, erst_size as u32);

            // Set ERSTBA (event ring segment table base address)
            self.write_rt64(ir0 + ir_reg::ERSTBA, erst_phys);

            // Set ERDP (event ring dequeue pointer)
            // Bits 3:0 must be preserved/cleared, bit 3 = EHB (Event Handler Busy)
            self.write_rt64(ir0 + ir_reg::ERDP, event_ring_phys);

            // Enable interrupts: set IE, clear IP
            let iman = self.read_rt(ir0 + ir_reg::IMAN);
            self.write_rt(ir0 + ir_reg::IMAN, iman | iman::IE | iman::IP);
        }
    }

    /// Start the controller (set RUN=1, INTE=1)
    pub fn start(&self) {
        unsafe {
            let cmd = self.read_op(op_reg::USBCMD);
            self.write_op(op_reg::USBCMD, cmd | usbcmd::RUN_STOP | usbcmd::INTE);
        }
    }

    /// Ring doorbell for a slot (0 = host controller command ring)
    pub fn ring_doorbell(&self, slot: u8, target: u8) {
        unsafe {
            let offset = (slot as usize) * 4;
            write_volatile(self.db_base.add(offset) as *mut u32, target as u32);
        }
    }

    /// Read current USBSTS
    pub fn read_status(&self) -> u32 {
        unsafe { self.read_op(op_reg::USBSTS) }
    }

    /// Clear event interrupt bit
    pub fn clear_event_interrupt(&self) {
        unsafe {
            let sts = self.read_op(op_reg::USBSTS);
            if (sts & usbsts::EINT) != 0 {
                self.write_op(op_reg::USBSTS, usbsts::EINT);
            }
        }
    }

    /// Update ERDP to acknowledge processed events
    pub fn update_erdp(&self, erdp_phys: u64) {
        let ir0 = rt_reg::IR0;
        unsafe {
            // Set EHB bit (bit 3) to clear it
            self.write_rt64(ir0 + ir_reg::ERDP, erdp_phys | (1 << 3));
        }
    }

    /// Read the current ERDP
    pub fn read_erdp(&self) -> u64 {
        let ir0 = rt_reg::IR0;
        unsafe {
            read_volatile(self.rt_base.add(ir0 + ir_reg::ERDP) as *const u64)
        }
    }
}

// ============================================================================
// Command Ring Helper
// ============================================================================

pub struct CommandRing {
    pub ring_virt: *mut Trb,
    pub ring_phys: u64,
    pub capacity: usize,
    pub enqueue_idx: usize,
    pub cycle: bool,
}

impl CommandRing {
    /// Create a command ring
    ///
    /// # Safety
    /// ring_virt must point to zeroed, properly aligned memory.
    /// The last TRB slot is reserved for a Link TRB.
    pub unsafe fn new(ring_virt: *mut Trb, ring_phys: u64, capacity: usize) -> Self {
        // Initialize all TRBs to zero (already done by DMA alloc)
        // Write Link TRB at the end
        let link_idx = capacity - 1;
        let link = Trb::link(ring_phys, true); // Initial cycle = 1
        write_volatile(ring_virt.add(link_idx), link);

        Self {
            ring_virt,
            ring_phys,
            capacity,
            enqueue_idx: 0,
            cycle: true,
        }
    }

    /// Enqueue a TRB and return the physical address of the enqueued TRB
    pub unsafe fn enqueue(&mut self, mut trb: Trb) -> Option<u64> {
        if self.enqueue_idx >= self.capacity - 1 {
            // Should not happen if we wrap at link
            return None;
        }

        // Set cycle bit
        trb.control = (trb.control & !1) | (self.cycle as u32);

        let phys = self.ring_phys + (self.enqueue_idx * 16) as u64;
        write_volatile(self.ring_virt.add(self.enqueue_idx), trb);

        self.enqueue_idx += 1;

        // Check if we hit the Link TRB
        if self.enqueue_idx == self.capacity - 1 {
            // Toggle cycle and wrap
            self.cycle = !self.cycle;
            // Update Link TRB cycle bit
            let link_ptr = self.ring_virt.add(self.capacity - 1);
            let mut link = read_volatile(link_ptr);
            link.control = (link.control & !1) | (self.cycle as u32);
            write_volatile(link_ptr, link);
            self.enqueue_idx = 0;
        }

        Some(phys)
    }
}

// ============================================================================
// Event Ring Helper
// ============================================================================

pub struct EventRing {
    pub ring_virt: *const Trb,
    pub ring_phys: u64,
    pub capacity: usize,
    pub dequeue_idx: usize,
    pub cycle: bool,
}

impl EventRing {
    pub unsafe fn new(ring_virt: *const Trb, ring_phys: u64, capacity: usize) -> Self {
        Self {
            ring_virt,
            ring_phys,
            capacity,
            dequeue_idx: 0,
            cycle: true,
        }
    }

    /// Check for a pending event and consume it if present
    pub unsafe fn poll(&mut self) -> Option<Trb> {
        let trb = read_volatile(self.ring_virt.add(self.dequeue_idx));

        // Check if cycle bit matches expected
        if trb.cycle_bit() != self.cycle {
            return None;
        }

        // Consume the event
        self.dequeue_idx += 1;
        if self.dequeue_idx >= self.capacity {
            self.dequeue_idx = 0;
            self.cycle = !self.cycle;
        }

        Some(trb)
    }

    /// Get the current dequeue pointer physical address
    pub fn dequeue_phys(&self) -> u64 {
        self.ring_phys + (self.dequeue_idx * 16) as u64
    }
}
