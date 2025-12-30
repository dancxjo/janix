use crate::bridge::FullMachineBridge;
use crate::drivers::video::DiscoveredFramebuffer;
use crate::Kernel;

/// Map and publish a discovered framebuffer into the graph.
pub fn publish_framebuffer<B: FullMachineBridge>(
    k: &mut Kernel<B>,
    discovered: &DiscoveredFramebuffer,
) -> u64 {
    let user_virt_addr = 0x1_0000_0000u64;

    map_framebuffer(k, discovered, user_virt_addr);

    let fb_info_user_va = abi::wire::machine::FbGetInfoResp {
        width: discovered.info.width,
        height: discovered.info.height,
        stride: discovered.info.stride,
        format: discovered.info.format,
        addr: user_virt_addr,
        size: discovered.size,
    };
    crate::drivers::limine_fb::init_with_info(k, fb_info_user_va);

    use abi::wire::typed::{CodecId, TypeId, TypedBytes};
    use thing_models::builtins::core_kinds::DisplayFramebufferBody;
    use thing_models::builtins::ids::THING_LINK_KIND;
    use thing_models::builtins::ids::{
        THING_BOOT_ROOT, THING_DISPLAY_FRAMEBUFFER_KIND, THING_HAS_DEVICE_KIND,
    };
    use thing_models::link::LinkBody;
    use thing_models::value::ThingBody;

    let fb_body = DisplayFramebufferBody {
        width: discovered.info.width as u64,
        height: discovered.info.height as u64,
        pitch: discovered.info.stride as u64,
        format: discovered.info.format,
        address: user_virt_addr,
    };

    let bytes = postcard::to_allocvec(&fb_body).unwrap();
    let tb = ThingBody::from(&TypedBytes {
        type_id: TypeId(THING_DISPLAY_FRAMEBUFFER_KIND.0 as u128),
        codec_id: CodecId::POSTCARD,
        bytes,
    })
    .unwrap();

    let fb_id = k.graph.create_thing(THING_DISPLAY_FRAMEBUFFER_KIND, tb);

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

    fb_id.0
}

fn map_framebuffer<B: FullMachineBridge>(
    k: &Kernel<B>,
    discovered: &DiscoveredFramebuffer,
    user_virt_addr: u64,
) {
    let page_size = 4096u64;
    let phys_base = discovered.lfb_phys & !(page_size - 1);
    let offset = discovered.lfb_phys - phys_base;
    let total_len = offset + discovered.size;
    let page_count = (total_len + page_size - 1) / page_size;

    for i in 0..page_count {
        let phys = phys_base + i * page_size;
        let virt = user_virt_addr + i * page_size;
        if let Err(_) = k.bridge.map_user_mmio(virt, phys, 0) {
            k.bridge.log("FRAMEBUFFER: map_user_mmio failed\n");
            break;
        }
    }
}
