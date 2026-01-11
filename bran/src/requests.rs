use limine::{
    BaseRevision, FramebufferRequest, PhysMemoryMapRequest, ModuleRequest,
    RsdpRequest, DtbRequest, HhdmRequest
};

pub static BASE_REVISION: BaseRevision = BaseRevision::new();

pub static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

pub static MEMMAP_REQUEST: PhysMemoryMapRequest = PhysMemoryMapRequest::new();

pub static MODULE_REQUEST: ModuleRequest = ModuleRequest::new();

pub static RSDP_REQUEST: RsdpRequest = RsdpRequest::new();

pub static DTB_REQUEST: DtbRequest = DtbRequest::new();

pub static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();
