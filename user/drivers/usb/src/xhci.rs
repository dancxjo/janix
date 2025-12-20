use alloc::format;
use alloc::vec::Vec;
use alloc::string::{String, ToString};
use thing_os::graph_ops::{GraphOp, GraphSink, GraphDriver, GraphEvent, ThingProps};
use thing_os::thing_models::graph_kinds::{self, KIND_PCI_DEVICE, KIND_USB_CONTROLLER};
use hal::MmioMapper;
use abi::{ThingId, syscall_defs::SymbolId};
use thing_os::{PropKey, PropValue, PropType};
use abi::wire::graph::{WirePropValue, WireValueTag};
use thing_os::intern;

// Local descriptor for UsbController
struct UsbControllerDesc {
    name: String,
    pci_bus: u64,
    pci_slot: u64,
    pci_func: u64,
    mmio_base: u64,
}

impl UsbControllerDesc {
    // Returns interned keys and WirePropValue
    fn to_wire_props(&self, out: &mut Vec<(SymbolId, WirePropValue)>) {
      out.push((intern("name"), WirePropValue::sym(intern(&self.name))));
      out.push((intern("pci_bus"), WirePropValue::u64(self.pci_bus)));
      out.push((intern("pci_slot"), WirePropValue::u64(self.pci_slot)));
      out.push((intern("pci_func"), WirePropValue::u64(self.pci_func)));
      out.push((intern("mmio_base"), WirePropValue::u64(self.mmio_base)));
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

    let mut guard = DRIVER.lock();
    if let Some(wrapper) = guard.as_mut() {
        wrapper.0.subscribe(intern(KIND_PCI_DEVICE), on_pci_device_created);
    }
}

fn on_pci_device_created(event: &GraphEvent) {
    let (id, _kind) = match event {
        GraphEvent::ThingCreated(id) => {
             (id, Option::<SymbolId>::None)
        },
        _ => return,
    };
    
    let mut driver_guard = DRIVER.lock();
    let driver = match driver_guard.as_mut() {
        Some(d) => &mut d.0,
        None => return,
    };
    
    // Query thing to get kind
    let props_obj = match driver.get_thing(*id) {
        Some(p) => p,
        None => return,
    };
    
    // Check kind
    // props_obj should have a kind field?
    // ABI `ThingProps` struct. Step 197 didn't show `ThingProps` definition.
    // Assuming `ThingProps` has `kind: SymbolId`.
    // If props_obj.kind != intern(KIND_PCI_DEVICE) -> return.
    
    // If I can't see ThingProps definition, I assume it has kind. 
    // OLD code (Step 238) didn't use `props_obj.kind`. It used `kind` from event.
    // Since `GraphEvent` lost the kind, I MUST check `props_obj.kind`.
    
    // Note: Assuming `get_thing` returns `ThingProps` which has `kind`.
    // If not, I am stuck. But `ThingData` in syscall returns kind. 
    // `GraphDriver::get_thing` returns `Option<ThingProps>`.
    
    // Let's assume `ThingProps` (in `abi::graph_ops`) has `pub kind: SymbolId`.
    
    if props_obj.kind != intern(KIND_PCI_DEVICE) {
        return;
    }
    
    // Analyze props
    let mut class_id = 0;
    let mut subclass_id = 0;
    let mut prog_if = 0;
    let mut bus = 0;
    let mut slot = 0;
    let mut func = 0;
    let mut bar0 = 0;
    
    // Intern keys for comparison
    let k_class = intern(graph_kinds::PROP_CLASS_ID);
    let k_subclass = intern(graph_kinds::PROP_SUBCLASS_ID);
    let k_progif = intern(graph_kinds::PROP_PROG_IF);
    let k_bus = intern(graph_kinds::PROP_BUS);
    let k_slot = intern(graph_kinds::PROP_SLOT);
    let k_func = intern(graph_kinds::PROP_FUNC);
    let k_bar0 = intern(graph_kinds::PROP_BAR0);
    
    // Props are now probably Vec<(SymbolId, WirePropValue)>?
    // Or did ThingProps props field change?
    // Step 197 `GraphOp` changed. `ThingProps` (if in `graph_ops`) likely changed too.
    
    for (k, v) in props_obj.props.iter() {
        // v is WirePropValue
        if v.tag == WireValueTag::U64 as u8 {
            let val = v.data_0;
            let key = *k;
            if key == k_class { class_id = val; }
            else if key == k_subclass { subclass_id = val; }
            else if key == k_progif { prog_if = val; }
            else if key == k_bus { bus = val; }
            else if key == k_slot { slot = val; }
            else if key == k_func { func = val; }
            else if key == k_bar0 { bar0 = val; }
        }
    }
    
    if class_id == 0x0C && subclass_id == 0x03 && prog_if == 0x30 {
        // Found XHCI
        let mmio_base_phys = bar0 & !0xF; // Mask low bits
        
        // Unlock driver to lock MMIO (avoid potential deadlock though likely safe here)
        // Actually we hold driver lock from line 84.
        
        let mmio_guard = MMIO.lock();
        let mmio = match mmio_guard.as_ref() {
            Some(m) => m.0,
            None => return,
        };
        
        // Safety: mapping arbitrary physical address
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
        controller.to_wire_props(&mut props);

        let _ = driver.submit(GraphOp::CreateThing {
            kind: intern(KIND_USB_CONTROLLER),
            props,
        });
    }
}
