pub mod limine_fb;
pub mod ps2;
pub mod rtc;

pub struct ProviderStorage {
    pub rtc: Option<rtc::RtcProvider>,
    pub ps2: Option<ps2::Ps2Provider>,
    pub limine_fb: Option<limine_fb::LimineFramebufferProvider>,
}

impl ProviderStorage {
    pub const fn new() -> Self {
        Self {
            rtc: None,
            ps2: None,
            limine_fb: None,
        }
    }
}
