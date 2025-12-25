use bridge_hosted::HostedBridge;
use kernel_core::Kernel;

fn main() {
    let mut k = Kernel::new(HostedBridge);
    k.boot();
}
