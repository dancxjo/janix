use kernel::{
    BootModuleDesc, BootRuntime, BootRuntimeBase, BootTasking, FrameAllocatorHook, FramebufferInfo,
    IrqState, MapKind, MapPerms, PhysRange, UserEntry, UserTaskSpec,
};

pub trait ArchRuntime {
    type Context: Copy + Default;
    type AddressSpace: Copy + Default;

    fn init(&self, hhdm_offset: u64);
    fn putchar(&self, c: u8);
    fn halt(&self) -> !;
    fn mono_ticks(&self) -> u64;
    fn mono_freq_hz(&self) -> u64;
    fn irq_disable(&self) -> IrqState;
    fn irq_restore(&self, state: IrqState);
    fn setup_preemption_timer(&self, _hz: u32) {}

    // SIMD - defaults
    fn simd_init_cpu(&self) {}
    fn simd_state_layout(&self) -> (usize, usize) {
        (0, 1)
    }
    unsafe fn simd_save(&self, _dst: *mut u8) {}
    unsafe fn simd_restore(&self, _src: *const u8) {}

    // Very early initialization (e.g., stack mode switching)
    unsafe fn early_init(&self) {}

    // Barriers - defaults
    fn threads_supported(&self) -> bool {
        false
    }
    fn fence_full(&self) {}
    fn icache_invalidate(&self) {}

    // Wait for interrupt - low-power idle until next IRQ
    fn wait_for_interrupt(&self) {}

    // Tasking - defaults
    fn init_kernel_context(
        &self,
        _entry: extern "C" fn(usize) -> !,
        _stack_top: u64,
        _arg: usize,
    ) -> Self::Context {
        Self::Context::default()
    }
    fn init_user_context(
        &self,
        _spec: UserTaskSpec<Self::AddressSpace>,
        _kstack_top: u64,
    ) -> Self::Context {
        Self::Context::default()
    }
    unsafe fn switch(&self, _from: &mut Self::Context, _to: &Self::Context) {
        // No-op
    }
    unsafe fn enter_user(&self, _entry: UserEntry) -> ! {
        panic!("enter_user not implemented for this architecture");
    }

    fn make_user_address_space(&self) -> Self::AddressSpace {
        Self::AddressSpace::default()
    }
    fn active_address_space(&self) -> Self::AddressSpace {
        Self::AddressSpace::default()
    }
    fn activate_address_space(&self, _aspace: Self::AddressSpace) {
        // No-op
    }

    fn map_page(
        &self,
        _aspace: Self::AddressSpace,
        _virt: u64,
        _phys: u64,
        _perms: MapPerms,
        _kind: MapKind,
        _allocator: &dyn FrameAllocatorHook,
    ) -> Result<(), ()> {
        Ok(())
    }
    fn unmap_page(&self, _aspace: Self::AddressSpace, _virt: u64) -> Result<Option<u64>, ()> {
        Ok(None)
    }
    fn translate(&self, _aspace: Self::AddressSpace, _virt: u64) -> Option<u64> {
        None
    }
    fn tlb_flush_page(&self, _virt: u64) {}

    // IO Port primitives (x86-only, stubs for other archs)
    fn ioport_read_u8(&self, _port: u16) -> u8 { 0 }
    fn ioport_read_u16(&self, _port: u16) -> u16 { 0 }
    fn ioport_read_u32(&self, _port: u16) -> u32 { 0 }
    fn ioport_write_u8(&self, _port: u16, _value: u8) {}
    fn ioport_write_u16(&self, _port: u16, _value: u16) {}
    fn ioport_write_u32(&self, _port: u16, _value: u32) {}

    fn debug_active_aspace_root(&self) -> u64 {
        0
    }
}

// --- Generic Runtime ---

pub struct Runtime<A: ArchRuntime> {
    pub arch: A,
    pub limine: LimineRuntimeData,
}

impl<A: ArchRuntime> Runtime<A> {
    pub const fn new(arch: A) -> Self {
        Self {
            arch,
            limine: LimineRuntimeData::new(),
        }
    }
}

pub struct LimineRuntimeData {}

impl LimineRuntimeData {
    pub const fn new() -> Self {
        Self {}
    }

    pub fn phys_memory_map(&self) -> &'static [PhysRange] {
        crate::mem::memory_map()
    }

    pub fn phys_to_virt_offset(&self) -> u64 {
        crate::requests::HHDM_REQUEST
            .get_response()
            .map(|r| r.offset())
            .unwrap_or(0)
    }

    pub fn modules(&self) -> &'static [BootModuleDesc] {
        crate::requests::get_modules()
    }

    pub fn framebuffer(&self) -> Option<FramebufferInfo> {
        crate::framebuffer::get_info()
    }

    pub fn acpi_rsdp(&self) -> Option<u64> {
        crate::requests::RSDP_REQUEST
            .get_response()
            .map(|r| r.address() as u64)
    }

    pub fn dtb_ptr(&self) -> Option<u64> {
        crate::requests::DTB_REQUEST
            .get_response()
            .map(|r| r.dtb_ptr() as u64)
    }
}

impl<A: ArchRuntime + 'static> BootRuntimeBase for Runtime<A> {
    fn putchar(&self, c: u8) {
        self.arch.putchar(c)
    }
    fn mono_ticks(&self) -> u64 {
        self.arch.mono_ticks()
    }
    fn mono_freq_hz(&self) -> u64 {
        self.arch.mono_freq_hz()
    }
}

impl<A: ArchRuntime + 'static> BootRuntime for Runtime<A> {
    type Tasking = Self;
    fn tasking(&self) -> &Self {
        self
    }

    fn halt(&self) -> ! {
        self.arch.halt()
    }

    fn wait_for_interrupt(&self) {
        self.arch.wait_for_interrupt()
    }

    fn simd_init_cpu(&self) {
        self.arch.simd_init_cpu()
    }
    fn simd_state_layout(&self) -> (usize, usize) {
        self.arch.simd_state_layout()
    }
    unsafe fn simd_save(&self, dst: *mut u8) {
        unsafe { self.arch.simd_save(dst) }
    }
    unsafe fn simd_restore(&self, src: *const u8) {
        unsafe { self.arch.simd_restore(src) }
    }

    unsafe fn early_init(&self) {
        unsafe { self.arch.early_init() }
    }

    fn threads_supported(&self) -> bool {
        self.arch.threads_supported()
    }
    fn fence_full(&self) {
        self.arch.fence_full()
    }

    fn phys_memory_map(&self) -> &'static [PhysRange] {
        self.limine.phys_memory_map()
    }
    fn phys_to_virt_offset(&self) -> u64 {
        self.limine.phys_to_virt_offset()
    }
    fn modules(&self) -> &'static [BootModuleDesc] {
        self.limine.modules()
    }
    fn framebuffer(&self) -> Option<FramebufferInfo> {
        self.limine.framebuffer()
    }

    fn irq_disable(&self) -> IrqState {
        self.arch.irq_disable()
    }
    fn irq_restore(&self, state: IrqState) {
        self.arch.irq_restore(state)
    }

    fn acpi_rsdp(&self) -> Option<u64> {
        self.limine.acpi_rsdp()
    }
    fn dtb_ptr(&self) -> Option<u64> {
        self.limine.dtb_ptr()
    }

    // IO Port forwarding
    fn ioport_read_u8(&self, port: u16) -> u8 {
        self.arch.ioport_read_u8(port)
    }
    fn ioport_read_u16(&self, port: u16) -> u16 {
        self.arch.ioport_read_u16(port)
    }
    fn ioport_read_u32(&self, port: u16) -> u32 {
        self.arch.ioport_read_u32(port)
    }
    fn ioport_write_u8(&self, port: u16, value: u8) {
        self.arch.ioport_write_u8(port, value)
    }
    fn ioport_write_u16(&self, port: u16, value: u16) {
        self.arch.ioport_write_u16(port, value)
    }
    fn ioport_write_u32(&self, port: u16, value: u32) {
        self.arch.ioport_write_u32(port, value)
    }

    fn setup_preemption_timer(&self, hz: u32) {
        self.arch.setup_preemption_timer(hz)
    }

    fn debug_active_aspace_root(&self) -> u64 {
        self.arch.debug_active_aspace_root()
    }
}

impl<A: ArchRuntime + 'static> BootTasking for Runtime<A> {
    type Runtime = Self;
    type Context = A::Context;
    type AddressSpace = A::AddressSpace;

    fn init(&self, hhdm_offset: u64) {
        self.arch.init(hhdm_offset)
    }

    fn init_kernel_context(
        &self,
        entry: extern "C" fn(usize) -> !,
        stack_top: u64,
        arg: usize,
    ) -> Self::Context {
        self.arch.init_kernel_context(entry, stack_top, arg)
    }

    fn init_user_context(
        &self,
        spec: UserTaskSpec<Self::AddressSpace>,
        kstack_top: u64,
    ) -> Self::Context {
        self.arch.init_user_context(spec, kstack_top)
    }

    unsafe fn switch(&self, from: &mut Self::Context, to: &Self::Context) {
        unsafe { self.arch.switch(from, to) }
    }

    unsafe fn enter_user(&self, entry: UserEntry) -> ! {
        unsafe { self.arch.enter_user(entry) }
    }

    fn make_user_address_space(&self) -> Self::AddressSpace {
        self.arch.make_user_address_space()
    }

    fn active_address_space(&self) -> Self::AddressSpace {
        self.arch.active_address_space()
    }

    fn activate_address_space(&self, aspace: Self::AddressSpace) {
        self.arch.activate_address_space(aspace)
    }

    fn map_page(
        &self,
        aspace: Self::AddressSpace,
        virt: u64,
        phys: u64,
        perms: MapPerms,
        kind: MapKind,
        allocator: &dyn FrameAllocatorHook,
    ) -> Result<(), ()> {
        self.arch
            .map_page(aspace, virt, phys, perms, kind, allocator)
    }

    fn unmap_page(&self, aspace: Self::AddressSpace, virt: u64) -> Result<Option<u64>, ()> {
        self.arch.unmap_page(aspace, virt)
    }

    fn translate(&self, aspace: Self::AddressSpace, virt: u64) -> Option<u64> {
        self.arch.translate(aspace, virt)
    }

    fn tlb_flush_page(&self, virt: u64) {
        self.arch.tlb_flush_page(virt)
    }
}
