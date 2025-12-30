use crate::bridge::HardwareBridge;
use crate::Kernel;

// Function `init` removed to avoid `limine` dependency in kernel.
// Use `init_with_info` instead, extracting data in `arch`.

pub fn init_with_info<B: HardwareBridge>(
    k: &mut Kernel<B>,
    info: abi::wire::machine::FbGetInfoResp,
) {
    use abi::symbols::sym;
    use abi::wire::machine::IFACE_FRAMEBUFFER;
    use abi::wire::typed::{CodecId, TypeId, TypedBytes};
    use thing_models::builtins::core_kinds::DisplayFramebufferBody;
    use thing_models::builtins::ids::{
        THING_BOOT_ROOT, THING_DISPLAY_FRAMEBUFFER_KIND, THING_HAS_DEVICE_KIND,
    };
    use thing_models::link::LinkBody;
    use thing_models::value::ThingBody;

    use crate::machine::providers::limine_fb::LimineFramebufferProvider;

    let fb_body = DisplayFramebufferBody {
        width: info.width as u64,
        height: info.height as u64,
        pitch: info.stride as u64,
        format: info.format,
        address: info.addr,
    };

    let provider = LimineFramebufferProvider::new(info);
    k.machine_providers.limine_fb = Some(provider);
    if let Some(provider) = &k.machine_providers.limine_fb {
        let ctx = provider as *const _ as *const ();
        k.machine.register_provider(
            sym(IFACE_FRAMEBUFFER),
            1,
            sym("fb0"),
            LimineFramebufferProvider::META,
            &LimineFramebufferProvider::VTABLE,
            ctx,
        );
    }

    let bytes = postcard::to_allocvec(&fb_body).unwrap();
    let tb = ThingBody::from(&TypedBytes {
        type_id: TypeId(THING_DISPLAY_FRAMEBUFFER_KIND.0 as u128),
        codec_id: CodecId::POSTCARD,
        bytes: bytes,
    })
    .unwrap();

    let fb_id = k.graph.create_thing(THING_DISPLAY_FRAMEBUFFER_KIND, tb);

    use thing_models::builtins::ids::THING_LINK_KIND;
    let link = LinkBody {
        from: THING_BOOT_ROOT,
        to: fb_id,
        predicate: THING_HAS_DEVICE_KIND,
    };
    let lb = ThingBody::from(&TypedBytes {
        type_id: TypeId(THING_LINK_KIND.0 as u128),
        codec_id: CodecId::POSTCARD,
        bytes: postcard::to_allocvec(&link).unwrap(),
    })
    .unwrap();
    k.graph.create_thing(THING_LINK_KIND, lb);
}
