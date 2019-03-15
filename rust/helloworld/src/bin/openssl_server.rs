extern crate capnp;
#[macro_use]
extern crate capnp_rpc;
extern crate openssl;
extern crate tokio;
extern crate tokio_io;
extern crate tokio_openssl;
extern crate helloworld;

use std::env;
use std::io::BufWriter;
use std::net::SocketAddr;

use capnp::capability::Promise;
use capnp::Error;
use capnp::message::HeapAllocator;

use capnp_rpc::{ImbuedMessageBuilder, RpcSystem, Server};
use capnp_rpc::rpc_twoparty_capnp::Side;
use capnp_rpc::twoparty::VatNetwork;

use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use tokio::io::AsyncRead;
use tokio::net::TcpListener;
use tokio::prelude::{Future, Stream};
use tokio::runtime::current_thread;

use tokio_openssl::SslAcceptorExt;

use helloworld::{Server as SayHelloServer, SayHelloParams, SayHelloResults, ToClient};
use helloworld::hello_reply::Builder as ReplyBuilder;

struct ServerImpl {
    builder: ImbuedMessageBuilder<HeapAllocator>
}

impl ServerImpl {
    fn new() -> Self {
        ServerImpl {
            builder: ImbuedMessageBuilder::new(HeapAllocator::new())
        }
    }
}

impl SayHelloServer for ServerImpl {

    fn say_hello(&mut self, params: SayHelloParams, mut results: SayHelloResults) -> Promise<(), Error> {
        // Get name from request
        let request = pry!(pry!(params.get()).get_request());
        let name = pry!(request.get_name());
        println!("Received message from {}.", name);

        // Build reply
        let message = format!("Hello, {}!", name);
        let mut reply_builder = pry!(self.builder.get_root::<ReplyBuilder>());
        reply_builder.set_message(&message);

        // 'Return' reply
        results.get().set_reply(reply_builder.into_reader()).unwrap();
        Promise::ok(())
    }

}

fn main() {
    let args: Vec<String> = env::args().collect();

    // TCP listener
    let addr = args[1].parse::<SocketAddr>().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();

    // TLS acceptor
    let mut acceptor_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    acceptor_builder.set_certificate_file(&args[2], SslFiletype::PEM).unwrap();
    acceptor_builder.set_private_key_file(&args[3], SslFiletype::PEM).unwrap();
    acceptor_builder.check_private_key().unwrap();
    let acceptor = acceptor_builder.build();

    // RPC client
    let client = ToClient::new(ServerImpl::new()).into_client::<Server>();

    println!("Starting server...");
    current_thread::block_on_all(listener.incoming().for_each(move |stream| {
        stream.set_nodelay(true)?;
        let client = client.clone();

        let tls_accept = acceptor
            .accept_async(stream)
            .and_then(move |stream| {
                let (reader, writer) = stream.split();
                let writer = BufWriter::new(writer);

                let network = VatNetwork::new(
                    reader,
                    writer,
                    Side::Server,
                    Default::default()
                );
                let rpc_system = RpcSystem::new(
                    Box::new(network),
                    Some(client.client)
                );

                current_thread::spawn(rpc_system.map_err(|e| eprintln!("{:?}", e)));
                Ok(())
            });

        current_thread::spawn(tls_accept.map_err(|e| eprintln!("{:?}", e)));
        Ok(())
    })).unwrap();
}
