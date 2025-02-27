use std::{io::ErrorKind, pin::Pin};

use async_trait::async_trait;
use dubbo_registry_nacos::NacosRegistry;
use futures_util::{Stream, StreamExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

use dubbo::{
    codegen::*,
    config::RootConfig,
    extension,
    extension::registry_extension::RegistryExtension,
    logger::{
        tracing::{info, span},
        Level,
    },
    Dubbo,
};
use protos::{
    greeter_server::{register_server, Greeter},
    GreeterReply, GreeterRequest,
};
pub mod protos {
    #![allow(non_camel_case_types)]
    include!(concat!(env!("OUT_DIR"), "/org.apache.dubbo.sample.tri.rs"));
}

type ResponseStream =
    Pin<Box<dyn Stream<Item = Result<GreeterReply, dubbo::status::Status>> + Send>>;

#[tokio::main]
async fn main() {
    register_server(GreeterServerImpl {
        name: "greeter".to_string(),
    });

    // Dubbo::new().start().await;
    Dubbo::new()
        .with_config({
            let r = RootConfig::new();
            match r.load() {
                Ok(config) => config,
                Err(_err) => panic!("err: {:?}", _err), // response was droped
            }
        })
        .start()
        .await;
}

struct GreeterServerImpl {
    name: String,
}

impl Greeter for GreeterServerImpl {
    async fn greet(
        &self,
        request: Request<GreeterRequest>,
    ) -> Result<Response<GreeterReply>, dubbo::status::Status> {
        println!("GreeterServer::greet {:?}", request.metadata);

        Ok(Response::new(GreeterReply {
            message: "hello, dubbo-rust".to_string(),
        }))
    }
}