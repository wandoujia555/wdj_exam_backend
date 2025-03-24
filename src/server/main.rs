use actix_web::rt::task;
use async_trait::async_trait;
use dubbo::codegen::{Request, Response};
use dubbo::Dubbo;
use futures_util::FutureExt;
use protos::login_reply::{self, Message};
use service::query::paper::{
    get_answer_by_question_id, get_answer_by_user_id, get_answer_list_by_paper_id,
    get_user_exam_status, query_class_by_id, query_paper_by_id, query_paper_list_by_id,
    save_answer_by_user_id, set_answer_by_user_id, set_user_exam_status,
};
use service::query::user::{
    authenticate as user_authenticate, query_student_by_code, query_teacher_by_code,
};

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
        let user: LoginRequest = request.into_inner();

        let result = if user.login_type == 0 {
            query_student_by_code(user.code).await
        } else {
            query_teacher_by_code(user.code).await
        };
        let mut is_login = false;
        let mut name = String::new();
        if let Ok(value) = result {
            if let Some(value) = value {
                is_login = value.get_password() == user.password;
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
            login_type: 1,
        }))
    }
    async fn get_paper_by_id(
        &self,
        _request: Request<protos::PaperRequest>,
    ) -> Result<Response<protos::Paper>, dubbo::status::Status> {
        let result = match query_paper_by_id(_request.into_inner().id).await {
            Ok(value) => value,
            Err(_) => None,
        };
        if let Some(value) = result {
            return Ok(Response::new(value));
        } else {
            return Err(dubbo::status::Status::new(
                dubbo::status::Code::NotFound,
                "Not Found".to_string(),
            ));
        }
    }
    async fn get_paper_list_by_id(
        &self,
        _request: Request<protos::PaperRequest>,
    ) -> Result<Response<protos::PaperInfoList>, dubbo::status::Status> {
        let request = _request.into_inner();
        match query_paper_list_by_id(request.id).await {
            Some(paper_list) => {
                println!("paper_list{:?}", paper_list);
                return Ok(Response::new(paper_list));
            }
            None => {
                return Err(dubbo::status::Status::new(
                    dubbo::status::Code::NotFound,
                    "Not Found".to_string(),
                ));
            }
        };
    }
    async fn set_answer_by_id(
        &self,
        _request: Request<protos::AnswerPaper>,
    ) -> Result<Response<protos::AnswerReply>, dubbo::status::Status> {
        let request = _request.into_inner();
        let answer_type = request.answer_type;
        if answer_type == 1 {
            if let Err(e) =
                set_answer_by_user_id(request.paper_id, request.user_id, request.content).await
            {
                return Err(dubbo::status::Status::new(
                    dubbo::status::Code::NotFound,
                    "Not Save".to_string(),
                ));
            }
        } else if let Err(e) =
            save_answer_by_user_id(request.paper_id, request.user_id, request.content).await
        {
            return Err(dubbo::status::Status::new(
                dubbo::status::Code::NotFound,
                "Not Save".to_string(),
            ));
        }
        return Ok(Response::new(protos::AnswerReply { is_save: true }));
    }

    async fn get_answer_by_id(
        &self,
        _request: Request<protos::AnswerRequest>,
    ) -> Result<Response<protos::AnswerPaper>, dubbo::status::Status> {
        let request = _request.into_inner();
        let data = get_answer_by_user_id(request.paper_id, request.user_id).await;
        println!("data:{:?}", data);
        return data
            .map(|answer_paper| Response::new(answer_paper))
            .map_err(|_| {
                dubbo::status::Status::new(
                    dubbo::status::Code::Internal,
                    "Internal Error".to_string(),
                )
            });
    }
    async fn get_answer_by_question_id(
        &self,
        _request: Request<protos::QuestionRequest>,
    ) -> Result<Response<protos::QuestionReply>, dubbo::status::Status> {
        let request = _request.into_inner();
        let data = get_answer_by_question_id(request.id).await;
        return data
            .map(|question_reply| Response::new(question_reply))
            .map_err(|_| {
                dubbo::status::Status::new(
                    dubbo::status::Code::Internal,
                    "Internal Error".to_string(),
                )
            });
    }
    async fn get_answer_list_by_paper_id(
        &self,
        _request: Request<protos::AnswerListRequest>,
    ) -> Result<Response<protos::AnswerListReply>, dubbo::status::Status> {
        let request = _request.into_inner();
        println!("asdasd{}", request.paper_id);
        let data = get_answer_list_by_paper_id(request.paper_id).await;
        return data
            .map(|question_reply| Response::new(question_reply))
            .map_err(|_| {
                dubbo::status::Status::new(
                    dubbo::status::Code::Internal,
                    "Internal Error".to_string(),
                )
            });
    }
    async fn get_user_exam_status(
        &self,
        _request: Request<protos::PaperUserInfoRequest>,
    ) -> Result<Response<protos::PaperUserInfoReply>, dubbo::status::Status> {
        let request = _request.into_inner();
        println!("asdasd{}", request.paper_id);
        let data = get_user_exam_status(request.user_id, request.paper_id).await;
        return data
            .map(|question_reply| Response::new(question_reply))
            .map_err(|_| {
                dubbo::status::Status::new(
                    dubbo::status::Code::Internal,
                    "Internal Error".to_string(),
                )
            });
    }
    async fn set_user_exam_status(
        &self,
        _request: Request<protos::SetUserInfoRequest>,
    ) -> Result<Response<protos::SetUserInfoReply>, dubbo::status::Status> {
        let request = _request.into_inner();
        println!("asdasd{}", request.paper_id);
        let data = set_user_exam_status(request.user_id, request.paper_id, request.status).await;
        return data
            .map(|is_success| {
                Response::new(protos::SetUserInfoReply {
                    is_save: is_success,
                })
            })
            .map_err(|_| {
                dubbo::status::Status::new(
                    dubbo::status::Code::Internal,
                    "Internal Error".to_string(),
                )
            });
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
