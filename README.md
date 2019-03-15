# Cap'n Proto Playground

## Features

1. Made by former gRPC developer with better performance in mind
1. "Infinitely faster" than Protobuf (no serialization needed)
1. Data types: void, bool, int, float, string, bytes, list, structs, enums, **interfaces**, **generics**
1. Promise pipelining -- multiple RPC calls in one network round trip
1. No encryption or authentication

## Python

`pycapnp`: [GitHub](https://github.com/capnproto/pycapnp), [Docs](http://jparyani.github.io/pycapnp/)

It's a wrapper around C++ library which uses its own event loop.

#### Pycapnp installation on Windows:
1. You're gonna need:
    * Visual Studio 2017 with the following build tools:
        * Windows 10 SDK (10.0.17763.0)
        * Visual C++ Tools for CMake
    * CMake (version >= 3.1)
1. Download and unzip https://capnproto.org/capnproto-c++-win32-0.7.0.zip
1. Go to `capnproto-c++-0.7.0` directory
1. Run `cmake -G "Visual Studio 15 2017" -A x64 .`
1. Run Visual Studio **as admin** and open`Cap'n Proto.sln`
1. Select `Release` build type
1. Build `BUILD_ALL` solution
1. Build `INSTALL` solution
1. Create a new Python virtual environment (further referred to as `venv`)
1. Clone or download [Pycapnp](https://github.com/capnproto/pycapnp)
1. Modify `extensions` in `setup.py` (lines 138--143) to look like this:

    ```python
    extensions = [Extension("capnp.lib.capnp", ["capnp/lib/capnp.cpp"],
                            include_dirs=[".", "C:\Program Files (x86)\Cap'n Proto\include"],
                            library_dirs=["C:\Program Files (x86)\Cap'n Proto\lib"],
                            language='c++',
                            extra_compile_args=['--std=c++11'],
                            libraries=['capnpc', 'capnp-rpc', 'capnp', 'kj-async', 'kj', 'ws2_32', 'Advapi32'])]
    ```
1. Activate the virtual env and run `python setup.py install --force-system-libcapnp`
1. Good luck!

---

No code generation is needed, pycapnp does it on the fly.

To start non-encrypted server run:
```
python python/server.py 127.0.0.1 <PORT>
```
To start SSL server run:
```
python python/ssl_server.py 127.0.0.1 <PORT> ./keys/server.crt ./keys/server.key
```
To start non-encrypted client run:
```
python python/client.py 127.0.0.1 <PORT>
```
To start SSL client run:
```
python python/ssl_client.py 127.0.0.1 <PORT> ./keys/server.crt
```

## Rust

`capnproto-rust`: [GitHub](https://github.com/capnproto/capnproto-rust/), [Docs](https://docs.capnproto-rust.org/capnp/)

The library is future-based and tokio-compatible.

SSL support is provided by [tokio-tls](https://docs.rs/tokio-tls/0.2.1/tokio_tls/) or [tokio-openssl](https://docs.rs/tokio-openssl/0.3.0/tokio_openssl/).

To build the project you need OpenSSL installed. For Windows use [this distribution](https://slproweb.com/products/Win32OpenSSL.html).
Before running `cargo build` set `OPENSSL_DIR` variable to point to your OpenSSL installation directory.

To re-generate Rust definitions you need `capnp` binary ([install instruction](https://capnproto.org/install.html)) and `capnpc` crate installed. Once you have these run:
```
capnp compile -orust:./rust/helloworld/src/ helloworld.capnp
```
To start non-encrypted server run:
```
./rust/helloworld/target/debug/server.exe 127.0.0.1:<PORT>
```
To start native SSL server run:
```
./rust/helloworld/target/debug/native_ssl_server.exe 127.0.0.1:<PORT> ./keys/server.p12
```
To start OpenSSL server run:
```
./rust/helloworld/target/debug/openssl_server.exe 127.0.0.1:<PORT> ./keys/server.crt ./keys/server.key 
```
To start non-encrypted client run:
```
./rust/helloworld/target/debug/client.exe 127.0.0.1:<PORT>
```
To start native SSL client run:
```
./rust/helloworld/target/debug/client.exe 127.0.0.1:<PORT> ./keys/server.crt
```
