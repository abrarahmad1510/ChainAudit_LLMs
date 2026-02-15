from http.server import HTTPServer, BaseHTTPRequestHandler
import json
import time

class LLMHandler(BaseHTTPRequestHandler):
    def do_POST(self):
        content_length = int(self.headers.get('Content-Length', 0))
        post_data = self.rfile.read(content_length)
        print("Received:", post_data)
        time.sleep(0.05)  # simulate inference
        self.send_response(200)
        self.send_header('Content-Type', 'application/json')
        self.end_headers()
        response = {"response": "This is a mock LLM response."}
        self.wfile.write(json.dumps(response).encode())

    def do_GET(self):
        self.do_POST()

if __name__ == '__main__':
    server = HTTPServer(('0.0.0.0', 8080), LLMHandler)
    print("Mock LLM running on port 8080")
    server.serve_forever()
