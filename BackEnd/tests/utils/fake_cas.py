from http.server import HTTPServer, SimpleHTTPRequestHandler
import http.server
import os


PAYLOAD = b'''
 <cas:serviceResponse xmlns:cas="http://www.yale.edu/tp/cas">
  <cas:authenticationSuccess>
    <cas:user>12110000</cas:user>
    <cas:attributes>
      <cas:mail>12110000@mail.sustech.edu.cn</cas:mail>
      <cas:givenName>San</cas:givenName>
      <cas:cn>San Zhang</cas:cn>
      <cas:sn>Zhang</cas:sn>
    </cas:attributes>
  </cas:authenticationSuccess>
</cas:serviceResponse>
'''

class HTTPHandler(SimpleHTTPRequestHandler):
    """This handler uses server.base_path instead of always using os.getcwd()"""

    def do_GET(self):
        self.send_response(200, 'OK')
        self.send_header('Content-Length', len(PAYLOAD))
        self.end_headers()
        self.wfile.write(PAYLOAD)

httpd = HTTPServer(("127.0.0.1", 8089), HTTPHandler)
httpd.serve_forever()
