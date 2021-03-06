from pathlib import Path
import socket
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


def main(host, port):
    sock = socket.socket()
    sock.bind((host, int(port)))
    sock.listen()

    print("Starting server...")
    while True:
        conn, _ = sock.accept()
        conn.setsockopt(socket.IPPROTO_TCP, socket.TCP_NODELAY, 1)
        conn.setblocking(False)
        server = capnp.TwoPartyServer(conn, bootstrap=Server())
        server.on_disconnect().wait()
        conn.close()


if __name__ == '__main__':
    main(*sys.argv[1:])
