use std::{io::ErrorKind, pin::Pin};
use actix_web::body::None;
use tokio::task;
use async_trait::async_trait;
use dubbo::codegen::{Request, Response};
use dubbo::Dubbo;
use futures_util::{Stream, StreamExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

use dotenv::dotenv;
use std::env;

pub mod protos {
    #![allow(non_camel_case_types)]
    include!(concat!("../", "/greeter.rs"));
}
use protos::{
    greeter_server::{register_server, Greeter},
    GreeterReply, GreeterRequest,
    LoginRequest, LoginReply,
};

// type ResponseStream =
//     Pin<Box<dyn Stream<Item = Result<GreeterReply, dubbo::status::Status>> + Send>>;
#[derive(Debug, Clone, Default)]
pub struct GreeterImpl {}

impl GreeterImpl {
    pub fn new() -> Self {
        GreeterImpl {}
    }
}

#[async_trait]
impl Greeter for GreeterImpl {
    async fn greet(&self, request: Request<GreeterRequest>) -> Result<Response<GreeterReply>, dubbo::status::Status> {
        println!("request: {:?}", request.into_inner());
        Ok(Response::new(GreeterReply{
            message: "hello dubbo-rust!".to_string(),
        }))
    }
    async fn authenticate(&self, request: Request<LoginRequest>) -> Result<Response<LoginReply>, dubbo::status::Status> {
        println!("request: {:?}", request.into_inner());
        Ok(Response::new(LoginReply{
            message: "hello dubbo-rust!".to_string(),
        }))
    }
}

// #[async_trait]
// impl Greeter for GreeterImpl {
    
// }


mod service;
mod api;
// #[tokio::main]
fn main() {
    // 初始化redis
    service::redis::redisi_init();
    // startServer();
    dotenv().ok();
    // let result = task::spawn_blocking(|| {
        
    // });
    service::mysql::get_per();
    startDubbo();

    // register_server(GreeterImpl::new());

    // Dubbo::new().start().await;
    // 默认算法是HS256，它使用共享机密。
    // let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret("secret".as_ref()))?;
}



#[tokio::main]
async fn startDubbo() {
    register_server(GreeterImpl::new());

    Dubbo::new().start().await;
    // 默认算法是HS256，它使用共享机密。
    // let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret("secret".as_ref()))?;
}