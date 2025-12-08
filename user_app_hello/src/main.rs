use user_app_hello::run;
use userland_rt::HostedSys;

fn main() {
    println!("=== user_app_hello (host) ===");

    let sys = HostedSys;
    run(&sys);

    println!("Finished running user_app_hello through HostedSys");
}
