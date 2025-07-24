#!/usr/bin/env python3
"""
Simple HTTP server for serving the USV Telemetry Dashboard
"""

import http.server
import socketserver
import os
import sys
import socket
from pathlib import Path

# Configuration
DIRECTORY = Path(__file__).parent

class CORSRequestHandler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=DIRECTORY, **kwargs)
    
    def end_headers(self):
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', 'Content-Type')
        self.send_header('Cross-Origin-Embedder-Policy', 'require-corp')
        self.send_header('Cross-Origin-Opener-Policy', 'same-origin')
        super().end_headers()
    
    def guess_type(self, path):
        mimetype = super().guess_type(path)
        if path.endswith('.wasm'):
            return 'application/wasm'
        return mimetype

def find_free_port(start_port=3000, max_attempts=100):
    """Find a free port starting from start_port"""
    for port in range(start_port, start_port + max_attempts):
        try:
            with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
                s.bind(('', port))
                return port
        except OSError:
            continue
    raise RuntimeError(f"Could not find a free port in range {start_port}-{start_port + max_attempts}")

def main():
    print(f"🚤 USV Telemetry Dashboard Server")
    print(f"📁 Serving directory: {DIRECTORY}")
    
    # Find a free port
    try:
        PORT = find_free_port()
        print(f"🌐 Server running at: http://localhost:{PORT}")
        print(f"🎯 Dashboard URL: http://localhost:{PORT}")
    except RuntimeError as e:
        print(f"❌ Error: {e}")
        sys.exit(1)
    
    print(f"\n📋 Files being served:")
    
    # List key files
    for file in ['index.html', 'assets/styles.css', 'target/wasm32-unknown-unknown/release/usv_telemetry.wasm', 'favicon.svg', 'favicon.ico']:
        if (DIRECTORY / file).exists():
            print(f"   ✅ {file}")
        else:
            print(f"   ❌ {file} (missing)")
    
    print(f"\n🔄 Starting server...")
    print(f"💡 Press Ctrl+C to stop the server\n")
    
    try:
        with socketserver.TCPServer(("", PORT), CORSRequestHandler) as httpd:
            print(f"✅ Server successfully started on port {PORT}")
            httpd.serve_forever()
    except KeyboardInterrupt:
        print(f"\n🛑 Server stopped.")
        sys.exit(0)
    except Exception as e:
        print(f"❌ Server error: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()