#![no_std]
#![no_main]

extern crate alloc;
use stem::thing::sys as thingsys;
use stem::thing::ThingId;
use abi::schema::{keys, kinds};

#[stem::main]
fn main(_arg: usize) -> ! {
    let new_name = "nebula-fern";
    
    let mut buf = [ThingId::default(); 1];
    if let Ok(count) = thingsys::find(kinds::DEV_HOST, &mut buf) {
        if count > 0 {
            if let Ok(sym) = thingsys::intern(new_name) {
                thingsys::prop_set(buf[0], keys::NAME, sym as u64).expect("set prop");
                stem::println!("SETHOSTNAME: Updated hostname to {}", new_name);
            } else {
                stem::println!("SETHOSTNAME: Failed to intern name");
            }
        } else {
            stem::println!("SETHOSTNAME: No dev.Host node found");
        }
    } else {
        stem::println!("SETHOSTNAME: find failed");
    }
    
    stem::syscall::exit(0);
}
