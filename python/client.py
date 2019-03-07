from pathlib import Path
import sys

import capnp


capnp_path = Path(__file__).parent / '..' / 'helloworld.capnp'
HelloWorld = capnp.load(str(capnp_path)).HelloWorld


def main(server_addr):
    client = capnp.TwoPartyClient(server_addr)
    helloworld = client.bootstrap().cast_as(HelloWorld)
    request = HelloWorld.HelloRequest.new_message(name="World")
    print("Sending request...")
    result = helloworld.sayHello(request=request).wait()
    print(f"Got reply: {result.reply.message}")


if __name__ == '__main__':
    main(sys.argv[1])
