//! VirtIO network device driver
//!
//! Implements a driver for virtio-net PCI devices (VirtIO 1.0+).

use crate::virtio::{VirtioPciDevice, Virtqueue, DeviceStatus, features, net_features};
use crate::net::{Nic, NicStats, TxError, EthernetFrame};
use alloc::vec::Vec;

/// VirtIO network device header (prepended to each frame)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct VirtioNetHeader {
    flags: u8,
    gso_type: u8,
    hdr_len: u16,
    gso_size: u16,
    csum_start: u16,
    csum_offset: u16,
    num_buffers: u16, // Only if VIRTIO_NET_F_MRG_RXBUF
}

impl VirtioNetHeader {
    fn zeroed() -> Self {
        Self {
            flags: 0,
            gso_type: 0,
            hdr_len: 0,
            gso_size: 0,
            csum_start: 0,
            csum_offset: 0,
            num_buffers: 0,
        }
    }
}

/// VirtIO network device configuration (device-specific)
#[repr(C)]
struct VirtioNetConfig {
    mac: [u8; 6],
    status: u16,
    max_virtqueue_pairs: u16,
    mtu: u16,
}

/// VirtIO network device driver
pub struct VirtioNetDevice {
    /// PCI device abstraction
    pci_dev: VirtioPciDevice,
    /// Receive queue (queue 0)
    rx_queue: Virtqueue,
    /// Transmit queue (queue 1)
    tx_queue: Virtqueue,
    /// MAC address
    mac: [u8; 6],
    /// Link status
    link_up: bool,
    /// Statistics
    stats: NicStats,
    /// RX buffer pool (physical addresses)
    rx_buffers_phys: Vec<u64>,
    /// RX buffer pool (virtual addresses)
    rx_buffers_virt: Vec<u64>,
    /// TX buffer (physical address)
    tx_buffer_phys: u64,
    /// TX buffer (virtual address)
    tx_buffer_virt: u64,
    /// HHDM offset for address translation
    hhdm_offset: u64,
}

impl VirtioNetDevice {
    /// Create and initialize a virtio-net device
    pub fn new(
        pci_dev: VirtioPciDevice,
        hhdm_offset: u64,
    ) -> Result<Self, &'static str> {
        crate::kinfo!("VirtIO-Net: Initializing device...");

        // Reset device
        pci_dev.reset();
        
        // Set ACKNOWLEDGE status
        pci_dev.write_status(DeviceStatus::Acknowledge as u8);
        
        // Set DRIVER status
        pci_dev.write_status(DeviceStatus::Acknowledge as u8 | DeviceStatus::Driver as u8);
        
        // Read device features
        let device_features = pci_dev.read_device_features_64();
        crate::kinfo!("VirtIO-Net: Device features: 0x{:x}", device_features);
        
        // Negotiate features
        let mut driver_features = features::VIRTIO_F_VERSION_1; // Modern device
        
        // Enable MAC address feature if available
        if device_features & net_features::VIRTIO_NET_F_MAC != 0 {
            driver_features |= net_features::VIRTIO_NET_F_MAC;
        }
        
        // Enable status feature if available
        if device_features & net_features::VIRTIO_NET_F_STATUS != 0 {
            driver_features |= net_features::VIRTIO_NET_F_STATUS;
        }
        
        pci_dev.write_driver_features_64(driver_features);
        crate::kinfo!("VirtIO-Net: Negotiated features: 0x{:x}", driver_features);
        
        // Set FEATURES_OK status
        pci_dev.write_status(
            DeviceStatus::Acknowledge as u8 
            | DeviceStatus::Driver as u8 
            | DeviceStatus::FeaturesOk as u8
        );
        
        // Verify FEATURES_OK
        if pci_dev.read_status() & DeviceStatus::FeaturesOk as u8 == 0 {
            pci_dev.write_status(DeviceStatus::Failed as u8);
            return Err("Device did not accept features");
        }
        
        // Read MAC address from device config
        let mac = if driver_features & net_features::VIRTIO_NET_F_MAC != 0 {
            [
                pci_dev.read_device_u8(0),
                pci_dev.read_device_u8(1),
                pci_dev.read_device_u8(2),
                pci_dev.read_device_u8(3),
                pci_dev.read_device_u8(4),
                pci_dev.read_device_u8(5),
            ]
        } else {
            [0x52, 0x54, 0x00, 0x12, 0x34, 0x56] // Default MAC
        };
        
        crate::kinfo!("VirtIO-Net: MAC address: {:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]);
        
        // Read link status
        let link_up = if driver_features & net_features::VIRTIO_NET_F_STATUS != 0 {
            let status = pci_dev.read_device_u32(6) as u16;
            (status & 1) != 0 // Bit 0 = link up
        } else {
            true // Assume up if not supported
        };
        
        crate::kinfo!("VirtIO-Net: Link status: {}", if link_up { "UP" } else { "DOWN" });
        
        // Set up virtqueues
        const QUEUE_SIZE: u16 = 64;
        
        // Allocate RX queue
        let rx_queue_mem_size = Virtqueue::memory_size(QUEUE_SIZE);
        let rx_queue_phys = crate::memory::alloc_contiguous_frames(
            (rx_queue_mem_size + 4095) / 4096
        ).ok_or("Failed to allocate RX queue memory")?;
        let rx_queue_virt = rx_queue_phys + hhdm_offset;
        let rx_queue = Virtqueue::new(QUEUE_SIZE, rx_queue_phys, rx_queue_virt);
        
        // Configure RX queue (queue 0)
        pci_dev.select_queue(0);
        pci_dev.write_queue_size(QUEUE_SIZE);
        let (desc, avail, used) = rx_queue.addresses();
        pci_dev.write_queue_desc(desc);
        pci_dev.write_queue_avail(avail);
        pci_dev.write_queue_used(used);
        pci_dev.enable_queue();
        
        crate::kinfo!("VirtIO-Net: RX queue configured (size={})", QUEUE_SIZE);
        
        // Allocate TX queue
        let tx_queue_mem_size = Virtqueue::memory_size(QUEUE_SIZE);
        let tx_queue_phys = crate::memory::alloc_contiguous_frames(
            (tx_queue_mem_size + 4095) / 4096
        ).ok_or("Failed to allocate TX queue memory")?;
        let tx_queue_virt = tx_queue_phys + hhdm_offset;
        let tx_queue = Virtqueue::new(QUEUE_SIZE, tx_queue_phys, tx_queue_virt);
        
        // Configure TX queue (queue 1)
        pci_dev.select_queue(1);
        pci_dev.write_queue_size(QUEUE_SIZE);
        let (desc, avail, used) = tx_queue.addresses();
        pci_dev.write_queue_desc(desc);
        pci_dev.write_queue_avail(avail);
        pci_dev.write_queue_used(used);
        pci_dev.enable_queue();
        
        crate::kinfo!("VirtIO-Net: TX queue configured (size={})", QUEUE_SIZE);
        
        // Allocate RX buffers
        const NUM_RX_BUFFERS: usize = 32;
        const RX_BUFFER_SIZE: usize = 2048;
        let mut rx_buffers_phys = Vec::new();
        let mut rx_buffers_virt = Vec::new();
        
        for _ in 0..NUM_RX_BUFFERS {
            let phys = crate::memory::alloc_contiguous_frames(
                (RX_BUFFER_SIZE + 4095) / 4096
            ).ok_or("Failed to allocate RX buffer")?;
            let virt = phys + hhdm_offset;
            rx_buffers_phys.push(phys);
            rx_buffers_virt.push(virt);
        }
        
        // Allocate TX buffer
        const TX_BUFFER_SIZE: usize = 2048;
        let tx_buffer_phys = crate::memory::alloc_contiguous_frames(
            (TX_BUFFER_SIZE + 4095) / 4096
        ).ok_or("Failed to allocate TX buffer")?;
        let tx_buffer_virt = tx_buffer_phys + hhdm_offset;
        
        let mut device = Self {
            pci_dev,
            rx_queue,
            tx_queue,
            mac,
            link_up,
            stats: NicStats::default(),
            rx_buffers_phys,
            rx_buffers_virt,
            tx_buffer_phys,
            tx_buffer_virt,
            hhdm_offset,
        };
        
        // Pre-fill RX queue with buffers
        device.refill_rx_queue();
        
        // Set DRIVER_OK status
        device.pci_dev.write_status(
            DeviceStatus::Acknowledge as u8 
            | DeviceStatus::Driver as u8 
            | DeviceStatus::FeaturesOk as u8
            | DeviceStatus::DriverOk as u8
        );
        
        crate::kinfo!("VirtIO-Net: Device initialized successfully");
        
        Ok(device)
    }
    
    /// Pre-fill RX queue with available buffers
    fn refill_rx_queue(&mut self) {
        for i in 0..self.rx_buffers_phys.len() {
            if let Some(desc_idx) = self.rx_queue.add_buffer(
                self.rx_buffers_phys[i],
                2048,
                true, // Write (device writes to this buffer)
            ) {
                self.rx_queue.kick(desc_idx);
            }
        }
        // Notify device
        self.pci_dev.notify_queue(0);
    }
}

impl Nic for VirtioNetDevice {
    fn mac(&self) -> [u8; 6] {
        self.mac
    }
    
    fn link_up(&self) -> bool {
        self.link_up
    }
    
    fn poll_rx(&mut self) -> Option<&[u8]> {
        // Check if there are used buffers
        if let Some((desc_idx, bytes_written)) = self.rx_queue.get_used() {
            // For now, we'll skip actual frame processing
            // Just reclaim the buffer
            self.rx_queue.reclaim(desc_idx);
            self.stats.rx_packets += 1;
            self.stats.rx_bytes += bytes_written as u64;
        }
        
        None // TODO: Return actual frame data
    }
    
    fn tx(&mut self, bytes: &[u8]) -> Result<(), TxError> {
        if bytes.len() > 1514 {
            return Err(TxError::FrameTooLarge);
        }
        
        const HEADER_SIZE: usize = core::mem::size_of::<VirtioNetHeader>();
        let total_size = HEADER_SIZE + bytes.len();
        
        // Allocate descriptor
        let desc_idx = self.tx_queue.add_buffer(
            self.tx_buffer_phys,
            total_size as u32,
            false, // Read (device reads from this buffer)
        ).ok_or(TxError::QueueFull)?;
        
        // Write virtio header (all zeros for basic mode)
        let header = VirtioNetHeader::zeroed();
        unsafe {
            core::ptr::write(self.tx_buffer_virt as *mut VirtioNetHeader, header);
        }
        
        // Copy frame after header
        unsafe {
            core::ptr::copy_nonoverlapping(
                bytes.as_ptr(),
                (self.tx_buffer_virt + HEADER_SIZE as u64) as *mut u8,
                bytes.len(),
            );
        }
        
        // Make buffer available and notify device
        self.tx_queue.kick(desc_idx);
        self.pci_dev.notify_queue(1);
        
        self.stats.tx_packets += 1;
        self.stats.tx_bytes += bytes.len() as u64;
        
        // Reclaim TX descriptor immediately (simple synchronous mode)
        // In a real implementation, we'd do this in an interrupt handler
        core::hint::spin_loop();
        if let Some((reclaim_idx, _)) = self.tx_queue.get_used() {
            self.tx_queue.reclaim(reclaim_idx);
        }
        
        Ok(())
    }
    
    fn stats(&self) -> NicStats {
        self.stats
    }
}
