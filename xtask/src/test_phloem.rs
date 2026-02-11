use anyhow::{Result, bail};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

const HOST: &str = "127.0.0.1";
const PORT: u16 = 2323;

pub fn run() -> Result<()> {
    println!("Connecting to {}:{}...", HOST, PORT);
    let mut stream = connect()?;
    println!("Connected!");

    // Read banner (best effort)
    let _ = read_response(&mut stream, false);

    run_cmd(
        &mut stream,
        "MERGE (n:TestNode {val: \"hello\"}) RETURN n",
        |res| res.contains("ok:") && res.contains("(id:"),
    )?;

    run_cmd(&mut stream, "MATCH (n:TestNode) RETURN n", |res| {
        res.contains("ok:") && res.contains("TestNode")
    })?;

    run_cmd(&mut stream, "SET n.newprop = 123", |res| {
        res.contains("ok:")
    })?;

    run_cmd(&mut stream, "MERGE (a:NodeA {name: \"A\"})", |_| true)?;
    run_cmd(&mut stream, "MERGE (b:NodeB {name: \"B\"})", |_| true)?;

    run_cmd(
        &mut stream,
        "MERGE (a)-[:LINKS_TO]->(b) RETURN a, b",
        |res| res.contains("ok: merged edge"),
    )?;

    run_cmd(
        &mut stream,
        "MATCH (a)-[:LINKS_TO]->(b) RETURN a, b",
        |res| res.contains("ok:") && res.contains("NodeA") && res.contains("NodeB"),
    )?;

    run_cmd(&mut stream, "QUIT", |_| true)?;

    println!("Test passed");
    Ok(())
}

fn connect() -> Result<TcpStream> {
    for i in 0..60 {
        match TcpStream::connect((HOST, PORT)) {
            Ok(s) => {
                s.set_read_timeout(Some(Duration::from_secs(2)))?;
                s.set_write_timeout(Some(Duration::from_secs(2)))?;
                return Ok(s);
            }
            Err(_) => {
                thread::sleep(Duration::from_secs(1));
                if i % 5 == 0 {
                    println!("Retrying connection... {}", i);
                }
            }
        }
    }
    bail!("Failed to connect to {}:{}", HOST, PORT);
}

fn run_cmd<F>(stream: &mut TcpStream, cmd: &str, check: F) -> Result<String>
where
    F: Fn(&str) -> bool,
{
    println!("> {}", cmd);
    stream.write_all(cmd.as_bytes())?;
    stream.write_all(b"\n")?;

    let response = read_response(stream, true)?;
    println!("< {}", response.trim());

    if !check(&response) {
        bail!("Check failed for command: {}", cmd);
    }

    Ok(response)
}

fn read_response(stream: &mut TcpStream, wait_for_prompt: bool) -> Result<String> {
    let mut buffer = [0; 1024];
    let mut total_data = String::new();

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => break, // EOF
            Ok(n) => {
                let chunk = String::from_utf8_lossy(&buffer[..n]);
                total_data.push_str(&chunk);
                if wait_for_prompt && total_data.contains("gql>") {
                    break;
                }
                if !wait_for_prompt {
                    break;
                }
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::WouldBlock
                    || e.kind() == std::io::ErrorKind::TimedOut
                {
                    break;
                }
                return Err(e.into());
            }
        }
    }
    Ok(total_data)
}
