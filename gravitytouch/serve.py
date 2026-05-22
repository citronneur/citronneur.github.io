"""
Tiny dev server for the WASM build.
Serves web/ with the MIME type for .wasm files set correctly.
Usage: python serve.py [port]   (default port 8080)
"""
import http.server
import sys
import webbrowser
from pathlib import Path

PORT = int(sys.argv[1]) if len(sys.argv) > 1 else 8080
WEB_DIR = Path(__file__).parent


class Handler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=str(WEB_DIR), **kwargs)

    def guess_type(self, path):
        if str(path).endswith(".wasm"):
            return "application/wasm"
        return super().guess_type(path)

    def log_message(self, fmt, *args):
        print(fmt % args)


print(f"Serving http://localhost:{PORT}/ from {WEB_DIR}")
webbrowser.open(f"http://localhost:{PORT}/")
http.server.HTTPServer(("", PORT), Handler).serve_forever()
