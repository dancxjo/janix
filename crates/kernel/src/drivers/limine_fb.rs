use crate::bridge::HardwareBridge;
use crate::Kernel;

pub unsafe fn init<B: HardwareBridge>(
    k: &mut Kernel<B>,
    fb_response: Option<&limine::response::FramebufferResponse>,
) {
    use abi::wire::typed::{CodecId, TypeId, TypedBytes};
    use thing_models::builtins::core_kinds::DisplayFramebufferBody;
    use thing_models::builtins::ids::{
        THING_BOOT_ROOT, THING_DISPLAY_FRAMEBUFFER_KIND, THING_HAS_DEVICE_KIND,
    };
    use thing_models::value::ThingBody;

    if let Some(resp) = fb_response {
        if let Some(fb) = resp.framebuffers().next() {
            k.bridge.log("DRIVER(limine_fb): Publishing...\n");

            let user_virt_addr = 0x1_0000_0000u64;

            let fb_body = DisplayFramebufferBody {
                width: fb.width(),
                height: fb.height(),
                pitch: fb.pitch(),
                format: 32,
                address: user_virt_addr,
            };

            let bytes = postcard::to_allocvec(&fb_body).unwrap();
            let tb = ThingBody::from(&TypedBytes {
                type_id: TypeId(THING_DISPLAY_FRAMEBUFFER_KIND.0 as u128),
                codec_id: CodecId::POSTCARD,
                bytes: bytes,
            })
            .unwrap();

            let fb_id = k.graph.create_thing(THING_DISPLAY_FRAMEBUFFER_KIND, tb);

            use thing_models::builtins::ids::THING_LINK_KIND;
            use thing_models::link::LinkBody;
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
    }
}
