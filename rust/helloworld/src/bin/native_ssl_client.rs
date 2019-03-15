extern crate capnp;
#[macro_use]
extern crate capnp_rpc;
extern crate native_tls;
extern crate tokio;
extern crate tokio_tls;
extern crate helloworld;

use std::env;
use std::fs::read;
use std::io::{BufWriter, Error as IOError, ErrorKind as IOErrorKind};
use std::net::SocketAddr;

use capnp::capability::Promise;

use capnp_rpc::RpcSystem;
use capnp_rpc::rpc_twoparty_capnp::Side;
use capnp_rpc::twoparty::VatNetwork;

use native_tls::{Certificate, TlsConnector as NativeConnector};

use tokio::io::AsyncRead;
use tokio::net::TcpStream;
use tokio::prelude::Future;
use tokio::runtime::current_thread;

use tokio_tls::TlsConnector as TokioConnector;

use helloworld::Client;

fn main () {
    let args: Vec<String> = env::args().collect();
    let addr = args[1].parse::<SocketAddr>().unwrap();


    let cert = read(&args[2]).unwrap();
    let cert = Certificate::from_pem(cert.as_slice()).unwrap();
    let connector = NativeConnector::builder()
        .add_root_certificate(cert)
        .build().unwrap();
    let connector = TokioConnector::from(connector);

    // Connect to server
    let mut runtime = current_thread::Runtime::new().unwrap();
    let stream = runtime.block_on(TcpStream::connect(&addr).and_then(|stream| {
        stream.set_nodelay(true).unwrap();
        connector.connect("localhost", stream)
            .map_err(|e| IOError::new(IOErrorKind::Other, e))
    })).unwrap();

    // Boilerplate
    let (reader, writer) = stream.split();
    let writer = BufWriter::new(writer);
    let network = VatNetwork::new(
        reader,
        writer,
        Side::Client,
        Default::default()
    );
    let mut rpc_system = RpcSystem::new(Box::new(network), None);
    let client: Client = rpc_system.bootstrap(Side::Server);
    runtime.spawn(rpc_system.map_err(|e| eprintln!("{:?}", e)));

    // Prepare request
    let mut request = client.say_hello_request();
    request.get().init_request().set_name("World");

    // Send request and wait for response
    println!("Sending request...");
    runtime.block_on(request.send().promise.and_then(|response| {
        let reply = pry!(pry!(response.get()).get_reply());
        let message = pry!(reply.get_message());
        println!("Got reply: \"{}\"", message);
        Promise::ok(())
    })).unwrap();
}
