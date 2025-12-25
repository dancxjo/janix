use bridge_hosted::HostedBridge;
use kernel_core::Kernel;

fn main() {
let k = Kernel::new(HostedBridge);
k.boot();
}
