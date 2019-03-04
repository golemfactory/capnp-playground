extern crate capnp;
#[macro_use]
extern crate capnp_rpc;
extern crate tokio;
extern crate helloworld;

use std::env;
use std::io::BufWriter;
use std::net::SocketAddr;

use capnp::capability::Promise;

use capnp_rpc::RpcSystem;
use capnp_rpc::rpc_twoparty_capnp::Side;
use capnp_rpc::twoparty::VatNetwork;

use tokio::io::AsyncRead;
use tokio::net::TcpStream;
use tokio::prelude::Future;
use tokio::runtime::current_thread;

use helloworld::Client;

fn main () {
    let args: Vec<String> = env::args().collect();
    let addr = args[1].parse::<SocketAddr>().unwrap();

    // Connect to server
    let mut runtime = current_thread::Runtime::new().unwrap();
    let stream = runtime.block_on(TcpStream::connect(&addr)).unwrap();

    // Boilerplate
    stream.set_nodelay(true).unwrap();
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
