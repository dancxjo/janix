use std::env;
use std::process;

mod fetch;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: xtask <command>");
        process::exit(1);
    }
    match args[1].as_str() {
        "fetch" => {
            if let Err(e) = fetch::fetch() {
                eprintln!("Fetch failed: {:?}", e);
                process::exit(1);
            }
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            process::exit(1);
        }
    }
}
