use alloc::format;
use alloc::vec::Vec;
use alloc::string::{String, ToString};
use abi::graph_ops::{GraphOp, GraphSink, GraphDriver, GraphEvent, ThingProps};
use abi::graph_kinds::{self, KIND_PCI_DEVICE, KIND_USB_CONTROLLER};
use hal::MmioMapper;
use abi::{PropKey, PropValue, PropType, ThingId};

// Local descriptor for UsbController
struct UsbControllerDesc {
    name: String,
    pci_bus: u64,
    pci_slot: u64,
    pci_func: u64,
    mmio_base: u64,
}

impl UsbControllerDesc {
    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
      out.push(("name", PropValue::Str(self.name.clone())));
      out.push(("pci_bus", PropValue::U64(self.pci_bus)));
      out.push(("pci_slot", PropValue::U64(self.pci_slot)));
      out.push(("pci_func", PropValue::U64(self.pci_func)));
      out.push(("mmio_base", PropValue::U64(self.mmio_base)));
    }
}

// Wrapper to make traits Sync/Send for static storage
struct SyncMmio(pub &'static dyn MmioMapper);
unsafe impl Sync for SyncMmio {}
unsafe impl Send for SyncMmio {}

struct SyncDriver(pub &'static mut dyn GraphDriver);
unsafe impl Sync for SyncDriver {}
unsafe impl Send for SyncDriver {}

use spin::Mutex;
static MMIO: Mutex<Option<SyncMmio>> = Mutex::new(None);
static DRIVER: Mutex<Option<SyncDriver>> = Mutex::new(None);

pub fn register_watcher(mmio: &dyn MmioMapper, graph: &mut dyn GraphDriver) {
    // SAFETY: We assume mmio and graph live for the entire kernel lifetime.
    let static_mmio: &'static dyn MmioMapper = unsafe { core::mem::transmute(mmio) };
    let static_graph: &'static mut dyn GraphDriver = unsafe { core::mem::transmute(graph) };
    
    *MMIO.lock() = Some(SyncMmio(static_mmio));
    *DRIVER.lock() = Some(SyncDriver(static_graph));

    // We can't lock DRIVER while subscribing if we fear recursion, but subscribe shouldn't recurse here.
    // However, static_graph is already moved into the Option (if we moved it). 
    // Wait, SyncDriver holds a reference. We have a copy of the reference.
    
    // We cannot use `static_graph` directly if we put it in the mutex? 
    // Yes we can, it's a Copy/Clone reference (fat pointer).
    // Actually `&mut` is NOT Copy. 
    
    // So we need to re-borrow from the Mutex OR just use the pointer we have before creating the wrapper.
    // BUT we already transmuted it to static mutable reference.
    // Rust does not allow two mutable references. using `static_graph` here and putting it in Mutex might be aliasing violation
    // if we access it from another thread.
    // But we are single threaded here.
    
    // To be safe: We should convert to raw pointer or just realize `&mut` is unique.
    // We put it in the Mutex. To use it, we must lock the mutex.
    
    let mut guard = DRIVER.lock();
    if let Some(wrapper) = guard.as_mut() {
        wrapper.0.subscribe(KIND_PCI_DEVICE, on_pci_device_created);
    }
}

fn on_pci_device_created(event: &GraphEvent) {
    // Fixed: logic to extract kind correctly from event
    let (id, kind) = match event {
        GraphEvent::ThingCreated { id, kind, .. } => (id, kind),
        // We only care about creation
        _ => return,
    };
    
    if *kind != KIND_PCI_DEVICE {
        return;
    }

    let mut driver_guard = DRIVER.lock();
    let driver = match driver_guard.as_mut() {
        Some(d) => &mut d.0,
        None => return,
    };
    
    // Query props
    let props_obj = match driver.get_thing(*id) {
        Some(p) => p,
        None => return,
    };
    
    // Analyze props
    let mut class_id = 0;
    let mut subclass_id = 0;
    let mut prog_if = 0;
    let mut bus = 0;
    let mut slot = 0;
    let mut func = 0;
    let mut bar0 = 0;
    
    for (k, v) in props_obj.props.iter() {
        if let PropValue::U64(val) = v {
            match *k {
                graph_kinds::PROP_CLASS_ID => class_id = *val,
                graph_kinds::PROP_SUBCLASS_ID => subclass_id = *val,
                graph_kinds::PROP_PROG_IF => prog_if = *val,
                graph_kinds::PROP_BUS => bus = *val,
                graph_kinds::PROP_SLOT => slot = *val,
                graph_kinds::PROP_FUNC => func = *val,
                graph_kinds::PROP_BAR0 => bar0 = *val,
                _ => {}
            }
        }
    }
    
    if class_id == 0x0C && subclass_id == 0x03 && prog_if == 0x30 {
        // Found XHCI
        let mmio_base_phys = bar0 & !0xF; // Mask low bits
        
        // Don't hold driver lock while locking MMIO if possible? 
        // Should be fine, different mutexes.
        let mmio_guard = MMIO.lock();
        let mmio = match mmio_guard.as_ref() {
            Some(m) => m.0,
            None => return,
        };
        
        let mmio_ptr = unsafe { mmio.map_mmio(mmio_base_phys, 4096) };
        let mmio_base_virt = mmio_ptr as u64;

        let name = format!("xhci_{:02x}_{:02x}_{:x}", bus, slot, func);
        
        let controller = UsbControllerDesc {
            name,
            pci_bus: bus,
            pci_slot: slot,
            pci_func: func,
            mmio_base: mmio_base_virt,
        };

        let mut props = Vec::new();
        controller.to_props(&mut props);

        let _ = driver.submit(GraphOp::CreateThing {
            kind: KIND_USB_CONTROLLER,
            props,
        });
    }
}
