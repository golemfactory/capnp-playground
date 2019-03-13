from pathlib import Path
import socket
import ssl
import sys

import capnp


capnp_path = Path(__file__).parent / '..' / 'helloworld.capnp'
HelloWorld = capnp.load(str(capnp_path)).HelloWorld


def main(server_host, server_port, server_cert):
    context = ssl.create_default_context(ssl.Purpose.SERVER_AUTH)
    context.load_verify_locations(server_cert)

    sock = socket.socket()
    sock.connect((server_host, int(server_port)))
    sock.setsockopt(socket.IPPROTO_TCP, socket.TCP_NODELAY, 1)
    conn = context.wrap_socket(sock, server_hostname="localhost")
    conn.setblocking(False)

    client = capnp.TwoPartyClient(conn)
    helloworld = client.bootstrap().cast_as(HelloWorld)
    request = HelloWorld.HelloRequest.new_message(name="World")
    print("Sending request...")
    result = helloworld.sayHello(request=request).wait()
    print(f"Got reply: {result.reply.message}")


if __name__ == '__main__':
    main(*sys.argv[1:])
