from pathlib import Path
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


def main(server_addr):
    server = capnp.TwoPartyServer(server_addr, bootstrap=Server())
    print("Starting server...")
    server.run_forever()


if __name__ == '__main__':
    main(sys.argv[1])
