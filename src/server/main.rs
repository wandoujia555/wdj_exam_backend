use actix_web::body::None;
use api::login::authenticate;
use async_trait::async_trait;
use dubbo::codegen::{Request, Response};
use dubbo::Dubbo;
use futures_util::{Stream, StreamExt};
use protos::login_reply::{self, Message};
use service::mysql::GLOBAL_DATA;
use service::query::user::query_student_by_code;
use std::{io::ErrorKind, pin::Pin};
use tokio::sync::mpsc;
use tokio::task;
use tokio_stream::wrappers::ReceiverStream;

use dotenv::dotenv;
use std::{env, result};

pub mod protos {
    #![allow(non_camel_case_types)]
    include!(concat!("../", "/greeter.rs"));
}
use protos::{
    greeter_server::{register_server, Greeter},
    GreeterReply, GreeterRequest, LoginReply, LoginRequest,
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
    async fn greet(
        &self,
        request: Request<GreeterRequest>,
    ) -> Result<Response<GreeterReply>, dubbo::status::Status> {
        println!("request: {:?}", request.into_inner());
        Ok(Response::new(GreeterReply {
            message: "hello dubbo-rust!".to_string(),
        }))
    }
    async fn authenticate(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginReply>, dubbo::status::Status> {
        let studenta: LoginRequest = request.into_inner();
        let result = query_student_by_code(studenta.code).await;
        let mut is_login = false;
        let mut name = String::new();
        if let Ok(value) = result {
            if let Some(value) = value {
                is_login = value.get_password() == studenta.password;
                println!("{:?}",value);
                name = value.get_name().to_string();
            }
        }
        // authenticate(studenta.code, studenta.password).await;
        Ok(Response::new(LoginReply {
            message: if is_login {
                Some(Message::Name(name))
            } else {
                Some(Message::IsLogin(is_login))
            },
        }))
    }
}

// #[async_trait]
// impl Greeter for GreeterImpl {

// }

mod api;
mod service;
// #[tokio::main]
fn main() {
    // 初始化redis
    service::redis::redisi_init();
    // startServer();
    dotenv().ok();
    // let result = task::spawn_blocking(|| {

    // });
    service::mysql::get_per();
    start_dubbo();

    // register_server(GreeterImpl::new());

    // Dubbo::new().start().await;
    // 默认算法是HS256，它使用共享机密。
    // let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret("secret".as_ref()))?;
}

#[tokio::main]
async fn start_dubbo() {
    register_server(GreeterImpl::new());

    Dubbo::new().start().await;
    // 默认算法是HS256，它使用共享机密。
    // let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret("secret".as_ref()))?;
}
