pub mod helloworld;
pub mod helloworld_grpc;

use std::thread;

use crate::helloworld_grpc::*;
use crate::helloworld::*;

struct GreeterImpl;

impl Greeter for GreeterImpl {
    fn say_hello(&self, _m: grpc::RequestOptions, req: HelloRequest) -> grpc::SingleResponse<HelloReply> {
        let mut r = HelloReply::new();
        r.set_message(format!("Hello {}", req.get_name()));
        grpc::SingleResponse::completed(r)
    }
}

fn main() {
    let port = 50051;

    let mut server = grpc::ServerBuilder::<tls_api_native_tls::TlsAcceptor>::new();
    server.http.set_port(port);
    server.add_service(GreeterServer::new_service_def(GreeterImpl));
    server.http.set_cpu_pool_threads(12);

    let _server = server.build().expect("server");
    println!("greeter server started on port {}", port);
    loop {
        thread::park();
    }
}
