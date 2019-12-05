use std::io::Read;
use std::sync::Arc;
use std::{io, thread};

use futures::sync::oneshot;
use futures::Future;
use grpcio::{ChannelBuilder, Environment, ResourceQuota, RpcContext, ServerBuilder, UnarySink};

use protos::helloworld::*;
use protos::helloworld_grpc::*;

#[derive(Clone)]
struct GreeterService;

impl Greeter for GreeterService {
    fn say_hello(&mut self, ctx: RpcContext, req: HelloRequest, sink: UnarySink<HelloReply>) {
        let msg = format!("Hello {}", req.get_name());
        let mut resp = HelloReply::new();
        resp.set_message(msg);
        let f = sink
            .success(resp)
            .map_err(move |e| println!("failed to reply {:?}: {:?}", req, e));
        ctx.spawn(f)
    }
}

fn main() {
    let env = Arc::new(Environment::new(1));
    let service = create_greeter(GreeterService);
    let quota = ResourceQuota::new(Some("HelloServerQuota")).resize_memory(1024 * 1024);
    let ch_builder = ChannelBuilder::new(env.clone()).resource_quota(quota);
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("0.0.0.0", 50_051)
        .channel_args(ch_builder.build_args())
        .build()
        .unwrap();
    server.start();
    for &(ref host, port) in server.bind_addrs() {
        println!("listening on {}:{}", host, port);
    }
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        println!("Press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    let _ = rx.wait();
    let _ = server.shutdown().wait();
}
