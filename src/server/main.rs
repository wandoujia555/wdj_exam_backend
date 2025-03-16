use actix_web::rt::task;
use async_trait::async_trait;
use dubbo::codegen::{Request, Response};
use dubbo::Dubbo;
use protos::login_reply::{self, Message};
use service::query::paper::{query_paper_list_by_id, query_paper_by_id};
use service::query::user::query_student_by_code;

use dotenv::dotenv;
use std::{env, result};

pub mod protos {
    #![allow(non_camel_case_types)]
    include!(concat!("../", "/greeter.rs"));
}
use protos::{
    greeter_server::{register_server, Greeter},
    GreeterReply, GreeterRequest, LoginReply, LoginRequest, Paper, PaperRequest,
};
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
        // req.path() == "/login"
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
                println!("{:?}", value);
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
    async fn get_paper_by_id(
        &self,
        _request: Request<protos::PaperRequest>,
    ) -> Result<Response<protos::Paper>, dubbo::status::Status> {

        query_paper_list_by_id().await;

        let result = match query_paper_by_id(_request.into_inner().id).await {
            Ok(value) => value,
            Err(_) => None,
        };
        if let Some(value) = result {
            return Ok(Response::new(value));
        }else {
            return Err(dubbo::status::Status::new(dubbo::status::Code::NotFound, "Not Found".to_string()));
        }
    }
}

mod api;
mod service;

fn main() {
    // async_std::task::block_on(query_paper_by_id(1));
    // 初始化redis

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
use std::thread;
#[tokio::main]
async fn start_dubbo() {
    register_server(GreeterImpl::new());
    service::redis::redisi_init().await;

    Dubbo::new().start().await;
    // 默认算法是HS256，它使用共享机密。
    // let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret("secret".as_ref()))?;
}
