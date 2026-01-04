fn main() {
    println!("kind.Window: 0x{:X}", hash("kind.Window"));
    println!("schema.Window@1: 0x{:X}", hash("schema.Window@1"));
}

const fn hash(s: &str) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        hash ^= bytes[i] as u64;
        hash = hash.wrapping_mul(0x100000001b3);
        i += 1;
    }
    hash
}
