#!/usr/bin/env python3
"""
HTTP-to-HTTPS Proxy for ThingOS Development

Runs on the host and accepts requests like:
  GET /?url=https://example.com/path

Returns the HTTPS content over plain HTTP so the guest OS
(which lacks TLS) can fetch web content.

The guest accesses this via QEMU's user-mode networking at:
  http://10.0.2.2:8080/?url=https://csszengarden.com/

Usage:
  python3 scripts/https_proxy.py [port]
  Default port: 8080
"""

import http.server
import socketserver
import urllib.request
import urllib.parse
import urllib.error
import ssl
import sys

PORT = int(sys.argv[1]) if len(sys.argv) > 1 else 8080

class ProxyHandler(http.server.BaseHTTPRequestHandler):
    def do_GET(self):
        # Parse query string
        parsed = urllib.parse.urlparse(self.path)
        params = urllib.parse.parse_qs(parsed.query)
        
        target_url = params.get('url', [None])[0]
        
        if not target_url:
            self.send_error(400, "Missing 'url' parameter")
            return
        
        print(f"[PROXY] Fetching: {target_url}")
        
        try:
            # Create SSL context that accepts certificates
            ctx = ssl.create_default_context()
            
            # Fetch the URL
            req = urllib.request.Request(
                target_url,
                headers={
                    'User-Agent': 'ThingOS-Proxy/1.0',
                    'Accept': '*/*',
                }
            )
            
            with urllib.request.urlopen(req, context=ctx, timeout=30) as response:
                content = response.read()
                content_type = response.headers.get('Content-Type', 'application/octet-stream')
                
                self.send_response(200)
                self.send_header('Content-Type', content_type)
                self.send_header('Content-Length', len(content))
                self.send_header('X-Original-URL', target_url)
                self.end_headers()
                self.wfile.write(content)
                
                print(f"[PROXY] OK: {len(content)} bytes ({content_type})")
                
        except urllib.error.HTTPError as e:
            print(f"[PROXY] HTTP Error {e.code}: {target_url}")
            self.send_error(e.code, str(e.reason))
            
        except urllib.error.URLError as e:
            print(f"[PROXY] URL Error: {e.reason}")
            self.send_error(502, f"Proxy error: {e.reason}")
            
        except Exception as e:
            print(f"[PROXY] Error: {e}")
            self.send_error(500, str(e))
    
    def log_message(self, format, *args):
        # Suppress default logging (we do our own)
        pass

def main():
    # Bind to all interfaces so QEMU guest can reach us
    with socketserver.TCPServer(("0.0.0.0", PORT), ProxyHandler) as httpd:
        print(f"[PROXY] HTTP-to-HTTPS Proxy running on port {PORT}")
        print(f"[PROXY] Guest should use: http://10.0.2.2:{PORT}/?url=https://...")
        print(f"[PROXY] Press Ctrl+C to stop")
        try:
            httpd.serve_forever()
        except KeyboardInterrupt:
            print("\n[PROXY] Shutting down...")

if __name__ == "__main__":
    main()
