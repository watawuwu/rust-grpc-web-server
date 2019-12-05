pub mod helloworld;
pub mod helloworld_grpc;

use std::thread;

use grpc::ServerHandlerContext;
use grpc::ServerRequestSingle;
use grpc::ServerResponseUnarySink;
use crate::helloworld::*;
use crate::helloworld_grpc::*;

struct GreeterImpl;

impl Greeter for GreeterImpl {
    fn say_hello(&self, _: ServerHandlerContext, req: ServerRequestSingle<HelloRequest>, resp: ServerResponseUnarySink<HelloReply>) -> grpc::Result<()> {
        let mut r = HelloReply::new();
        r.set_message(format!("Hello {}", req.message.get_name()));
        resp.finish(r)
    }
}

fn main() {
    let port = 50051;

    let mut server = grpc::ServerBuilder::<tls_api_native_tls::TlsAcceptor>::new();
    server.http.set_port(port);
    server.add_service(GreeterServer::new_service_def(GreeterImpl));
    let _server = server.build().expect("server");
    println!("greeter server started on port {}", port);

    loop {
        thread::park();
    }
}
