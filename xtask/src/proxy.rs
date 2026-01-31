//! HTTPS Reverse Proxy for ThingOS Development
//!
//! Wraps the guest's HTTP server (port 8888) behind a self-signed HTTPS server
//! so host browsers can access it without mixed-content warnings.

use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use crate::common::project_root;

/// Run the HTTPS reverse proxy
pub fn https_proxy(https_port: u16, target_port: u16) -> Result<()> {
    let cert_dir = project_root().join("target").join(".certs");
    let cert_file = cert_dir.join("server.crt");
    let key_file = cert_dir.join("server.key");

    // Ensure certificates exist
    ensure_certificates(&cert_dir, &cert_file, &key_file)?;

    println!("[HTTPS-PROXY] Starting HTTPS reverse proxy...");
    println!("[HTTPS-PROXY] HTTPS port: {}", https_port);
    println!("[HTTPS-PROXY] Target: http://localhost:{}", target_port);
    println!(
        "[HTTPS-PROXY] Browser access: https://localhost:{}/",
        https_port
    );
    println!("[HTTPS-PROXY] Note: Accept the self-signed certificate warning in your browser");
    println!("[HTTPS-PROXY] Press Ctrl+C to stop");

    // Use openssl s_server as a simple HTTPS frontend
    // This is simpler than implementing TLS in pure Rust without pulling in large deps
    run_openssl_proxy(&cert_file, &key_file, https_port, target_port)
}

fn ensure_certificates(cert_dir: &PathBuf, cert_file: &PathBuf, key_file: &PathBuf) -> Result<()> {
    if cert_file.exists() && key_file.exists() {
        println!(
            "[HTTPS-PROXY] Using existing certificates in {}",
            cert_dir.display()
        );
        return Ok(());
    }

    fs::create_dir_all(cert_dir).context("Failed to create certificate directory")?;

    println!("[HTTPS-PROXY] Generating self-signed certificates...");

    let status = Command::new("openssl")
        .args([
            "req",
            "-x509",
            "-newkey",
            "rsa:2048",
            "-keyout",
            key_file.to_str().unwrap(),
            "-out",
            cert_file.to_str().unwrap(),
            "-days",
            "365",
            "-nodes",
            "-subj",
            "/CN=localhost/O=ThingOS-Dev",
            "-addext",
            "subjectAltName=DNS:localhost,IP:127.0.0.1",
        ])
        .status()
        .context("Failed to run openssl")?;

    if !status.success() {
        anyhow::bail!("Failed to generate certificates");
    }

    println!(
        "[HTTPS-PROXY] Generated certificates in {}",
        cert_dir.display()
    );
    Ok(())
}

/// Use a simple TCP proxy with openssl s_server for TLS termination
fn run_openssl_proxy(
    cert_file: &PathBuf,
    key_file: &PathBuf,
    https_port: u16,
    target_port: u16,
) -> Result<()> {
    // We'll use a custom simple proxy approach:
    // Listen on HTTPS, for each connection, spawn openssl s_client to handle TLS,
    // then forward to the backend

    // Actually, the simplest approach is to use socat or a simple Rust TLS impl
    // But to keep dependencies minimal, we'll use openssl s_server with -WWW mode
    // However that serves files, not proxies.

    // Alternative: use stunnel-like approach with openssl s_server piping to nc
    // Let's use a simpler Python one-liner approach via subprocess

    // For robustness, let's just call the Python script inline
    let python_script = r#"
import http.server
import ssl
import subprocess

class ReverseProxyHandler(http.server.BaseHTTPRequestHandler):
    protocol_version = 'HTTP/1.1'
    
    def proxy_request(self, method):
        url = f'http://localhost:{TARGET_PORT}{self.path}'
        try:
            # Use curl to fetch from backend - this is proven to work
            result = subprocess.run(
                ['curl', '-s', '-i', '-X', method, '--max-time', '10', url],
                capture_output=True,
                timeout=15
            )
            
            if result.returncode != 0:
                self.send_error(502, f'Backend error (curl returned {result.returncode})')
                return
            
            # Parse curl output (headers + body separated by \r\n\r\n)
            output = result.stdout
            header_end = output.find(b'\r\n\r\n')
            if header_end == -1:
                header_end = output.find(b'\n\n')  # fallback
                sep_len = 2
            else:
                sep_len = 4
            
            if header_end == -1:
                # No headers found, treat entire output as body
                self.send_response(200)
                self.send_header('Content-Type', 'text/plain')
                self.send_header('Content-Length', len(output))
                self.end_headers()
                self.wfile.write(output)
                return
            
            headers_raw = output[:header_end].decode('utf-8', errors='replace')
            body = output[header_end + sep_len:]
            
            # Parse status line
            lines = headers_raw.split('\r\n') if '\r\n' in headers_raw else headers_raw.split('\n')
            status_line = lines[0] if lines else 'HTTP/1.1 200 OK'
            parts = status_line.split(' ', 2)
            status_code = int(parts[1]) if len(parts) > 1 else 200
            
            self.send_response(status_code)
            
            # Forward headers
            for line in lines[1:]:
                if ':' in line:
                    key, val = line.split(':', 1)
                    key = key.strip().lower()
                    if key not in ('transfer-encoding', 'connection', 'content-length'):
                        self.send_header(key, val.strip())
            
            self.send_header('Content-Length', len(body))
            self.end_headers()
            self.wfile.write(body)
            
        except subprocess.TimeoutExpired:
            self.send_error(504, 'Backend timeout')
        except Exception as e:
            print(f'[HTTPS-PROXY] Error: {e}')
            self.send_error(500, str(e))
    
    def do_GET(self): self.proxy_request('GET')
    def do_POST(self): self.proxy_request('POST')
    def do_PUT(self): self.proxy_request('PUT')
    def do_DELETE(self): self.proxy_request('DELETE')
    def do_HEAD(self): self.proxy_request('HEAD')
    def do_OPTIONS(self): self.proxy_request('OPTIONS')
    def log_message(self, format, *args): print(f'[HTTPS-PROXY] {args[0]}')

server_address = ('0.0.0.0', HTTPS_PORT)
httpd = http.server.HTTPServer(server_address, ReverseProxyHandler)
context = ssl.SSLContext(ssl.PROTOCOL_TLS_SERVER)
context.load_cert_chain(CERT_FILE, KEY_FILE)
httpd.socket = context.wrap_socket(httpd.socket, server_side=True)
print('[HTTPS-PROXY] Server ready')
httpd.serve_forever()
"#;

    // Substitute values
    let script = python_script
        .replace("HTTPS_PORT", &https_port.to_string())
        .replace("TARGET_PORT", &target_port.to_string())
        .replace("CERT_FILE", &format!("\"{}\"", cert_file.display()))
        .replace("KEY_FILE", &format!("\"{}\"", key_file.display()));

    // Run the Python script
    let status = Command::new("python3")
        .arg("-c")
        .arg(&script)
        .status()
        .context("Failed to run Python HTTPS proxy")?;

    if !status.success() {
        anyhow::bail!("HTTPS proxy exited with error");
    }

    Ok(())
}
