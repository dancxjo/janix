use limine::BaseRevision;
use limine::request::{
    FramebufferRequest, HhdmRequest, MemoryMapRequest, MpRequest, RequestsEndMarker,
    RequestsStartMarker, ModuleRequest,
};
use kernel::{BootModuleDesc, BootModuleKind};

#[used]
#[unsafe(link_section = ".requests")]
pub static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[unsafe(link_section = ".requests")]
pub static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
#[unsafe(link_section = ".requests")]
pub static MODULE_REQUEST: ModuleRequest = ModuleRequest::new();

#[used]
#[unsafe(link_section = ".requests")]
pub static MEMORY_MAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();

#[used]
#[unsafe(link_section = ".requests")]
pub static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();

#[used]
#[unsafe(link_section = ".requests")]
pub static SMP_REQUEST: MpRequest = MpRequest::new();

#[used]
#[unsafe(link_section = ".requests_start_marker")]
pub static _START_MARKER: RequestsStartMarker = RequestsStartMarker::new();

#[used]
#[unsafe(link_section = ".requests_end_marker")]
pub static _END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

pub fn get_modules() -> &'static [BootModuleDesc] {
    if let Some(resp) = MODULE_REQUEST.get_response() {
        static mut MODULES: [BootModuleDesc; 16] = [BootModuleDesc {
            name: "",
            bytes: &[],
            phys_start: 0,
            phys_end: 0,
            kind: BootModuleKind::Unknown,
        }; 16];
        static mut COUNT: usize = 0;
        
        unsafe {
            if COUNT == 0 {
                for (i, m) in resp.modules().into_iter().enumerate() {
                    if i >= 16 { break; }
                    let addr = m.addr();
                    let size = m.size();
                    let path = m.path().to_str().unwrap_or("");
                    
                    MODULES[i] = BootModuleDesc {
                        name: path,
                        bytes: core::slice::from_raw_parts(addr as *const u8, size as usize),
                        phys_start: addr as u64,
                        phys_end: (addr as u64) + size,
                        kind: BootModuleKind::Unknown,
                    };
                    COUNT += 1;
                }
            }
            &MODULES[..COUNT]
        }
    } else {
        &[]
    }
}
