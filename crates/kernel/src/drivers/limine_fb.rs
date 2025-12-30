use crate::bridge::FullMachineBridge;
use crate::Kernel;

// Function `init` removed to avoid `limine` dependency in kernel.
// Use `init_with_info` instead, extracting data in `arch`.

pub fn init_with_info<B: FullMachineBridge>(
    k: &mut Kernel<B>,
    info: abi::wire::machine::FbGetInfoResp,
) {
    use abi::symbols::sym;
    use abi::wire::machine::IFACE_FRAMEBUFFER;

    use crate::machine::providers::limine_fb::LimineFramebufferProvider;

    let provider = LimineFramebufferProvider::new(info);
    k.machine_providers.limine_fb = Some(provider);
    if let Some(provider) = &k.machine_providers.limine_fb {
        let ctx = provider as *const _ as *const ();
        k.machine.register_provider(
            sym(IFACE_FRAMEBUFFER),
            1,
            sym("fb0"),
            LimineFramebufferProvider::META,
            LimineFramebufferProvider::vtable(),
            ctx,
        );
    }
}
