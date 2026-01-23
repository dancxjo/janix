#![feature(restricted_std)]
use std::time::Duration;
use std::vec::Vec;

fn main() {
    println!("std_smoke: hello from std");

    std::fs::write("/tmp/hello.txt", "thingos std").expect("write tmp file");
    match std::fs::read_to_string("/tmp/hello.txt") {
        Ok(content) => println!("std_smoke: read back '{}'", content),
        Err(e) => println!("std_smoke: read failed: {}", e),
    }

    let mut data = Vec::new();
    for i in 0..50_000u32 {
        data.push(i);
    }
    println!("std_smoke: allocated {} ints", data.len());

    let worker = std::thread::spawn(|| {
        for i in 0..5 {
            println!("std_smoke: thread {}", i);
            std::thread::sleep(Duration::from_millis(10));
        }
    });

    std::thread::sleep(Duration::from_millis(5));
    worker.join().unwrap();

    eprintln!("std_smoke: done");
    // std::process::exit(0); // Implicit
}
