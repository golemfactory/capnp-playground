from pathlib import Path
import socket
import ssl
import sys

import capnp


capnp_path = Path(__file__).parent / '..' / 'helloworld.capnp'
HelloWorld = capnp.load(str(capnp_path)).HelloWorld


class Server(HelloWorld.Server):

    def sayHello(self, request, **_):
        name = request.name
        print(f"Received message from {name}.")
        message = f"Hello, {name}!"
        return HelloWorld.HelloReply.new_message(message=message)


def main(host, port, server_cert, server_key):
    context = ssl.create_default_context(ssl.Purpose.CLIENT_AUTH)
    context.load_cert_chain(server_cert, server_key)
    sock = socket.socket()
    sock.bind((host, int(port)))
    sock.listen()

    print("Starting server...")
    while True:
        conn, _ = sock.accept()
        conn.setsockopt(socket.IPPROTO_TCP, socket.TCP_NODELAY, 1)
        try:
            ssl_conn = context.wrap_socket(conn, server_side=True)
            ssl_conn.setblocking(False)
            server = capnp.TwoPartyServer(ssl_conn, bootstrap=Server())
            server.on_disconnect().wait()
            ssl_conn.close()
        except ssl.SSLError as e:
            print(e)


if __name__ == '__main__':
    main(*sys.argv[1:])
