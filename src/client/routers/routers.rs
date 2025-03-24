use crate::{
    api::util::{
        self, authenticate, call_dubbo_service, get_answer_by_id, get_answer_by_question_id, get_answer_list_by_paper_id, get_paper_by_id, get_paper_list_by_user_id, set_answer_by_id
    },
    protos::{AnswerListRequest, AnswerPaper, AnswerRequest, Paper, QuestionRequest},
};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use chrono::Utc;
use prost::Message;
use serde_json::to_string;
// /// 处理根路径的 GET 请求
// async fn index() -> impl Responder {
//     HttpResponse::Ok().body("Hello, world!")
// }

// /// 处理表单提交的 POST 请求
// async fn submit_form(form: web::Form<FormData>) -> impl Responder {
//     HttpResponse::Ok().body(format!("Welcome, {}!", form.name))
// }

// fn configure_routes(cfg: &mut web::ServiceConfig) {
//     cfg.service(web::resource("/").route(web::get().to(index)))
//         .service(web::resource("/submit").route(web::post().to(submit_form)));
// }
async fn index() -> impl Responder {
    println!("{}", Utc::now().timestamp());
    let res = call_dubbo_service().await;
    println!("{}", Utc::now().timestamp());
    HttpResponse::Ok().body(res)
}

async fn about() -> impl Responder {
    HttpResponse::Ok().body("This is the about page.")
}

async fn contact() -> impl Responder {
    HttpResponse::Ok().body("Contact us at contact@example.com")
}

#[derive(Serialize)]
struct SubmitResponse {
    text: String,
}

async fn submit() -> impl Responder {
    let data = SubmitResponse {
        text: "asdasd".to_string(),
    };

    HttpResponse::Ok().json(data) // 返回解析后的数据作为 JSON 响应

    // HttpResponse::Ok().body("Form submitted successfully!")
}

async fn login() -> impl Responder {
    println!("asdasd");
    HttpResponse::Ok().body("Form submasdasdaditted successfully!")
}

use crate::middleware::auth::{generate_jwt, User};
use serde::{Deserialize, Serialize};
// #[derive(Deserialize, Serialize)]
// pub struct LoginData {
//     code: String,
//     password: String,
// }

#[derive(Serialize)]
struct JwtResponse {
    token: String,
    name: String,
}
use crate::protos::login_reply;
async fn post_login(data: web::Json<User>) -> impl Responder {
    println!("{:?}",data);
    // let response = format!("Name: {}, Age: {}", form.name, form.age);
    // HttpResponse::Ok().body(response)
    // let login_data = data.into_inner(); // 反序列化 JSON 数据
    let auth = authenticate(data.code, data.password.clone(), data.login_type).await;
    
    match auth {
        Some(login_reply::Message::IsLogin(is_login)) => HttpResponse::Ok().json(false),
        Some(login_reply::Message::Name(name)) => {
            let login_data = generate_jwt(&data);

            if login_data.is_err() {
                return HttpResponse::Ok().body("create jwt error");
            }

            let login_data_value = JwtResponse {
                token: login_data.unwrap(),
                name: name,
            };

            HttpResponse::Ok().json(login_data_value) // 返回解析后的数据作为 JSON 响应
        }
        None => {
            HttpResponse::Ok().json(false) // 返回解析后的数据作为 JSON 响应
        }
    }
}

#[derive(Serialize, Deserialize)]
struct PaperRequest {
    id: i32,
}
async fn paper(data: web::Json<PaperRequest>) -> impl Responder {
    let result = get_paper_by_id(data.id).await;
    match result {
        Ok(paper) => {
            // serde_json::to_string(&paper).unwrap();
            // let paper_bytes = paper.encode_to_vec();
            // serde_json::to_string(&paper).unwrap();
            HttpResponse::Ok().json(paper)
        }
        Err(_) => HttpResponse::Ok().json(false),
    }
}

#[derive(Serialize, Deserialize)]
struct AnswerPaperUser {
    id: i32,
}
async fn set_answer(data: web::Json<AnswerPaper>) -> impl Responder {
    println!("-{:?}", data);
    let result = set_answer_by_id(data.into_inner()).await;
    match result {
        Ok(paper) => {
            // serde_json::to_string(&paper).unwrap();
            // let paper_bytes = paper.encode_to_vec();
            // serde_json::to_string(&paper).unwrap();
            HttpResponse::Ok().json(paper)
        }
        Err(_) => HttpResponse::Ok().json(false),
    }
}

async fn get_answer(data: web::Json<AnswerRequest>) -> impl Responder {
    println!("-{:?}", data);
    let result = get_answer_by_id(data.into_inner()).await;
    match result {
        Ok(paper) => {
            // serde_json::to_string(&paper).unwrap();
            // let paper_bytes = paper.encode_to_vec();
            // serde_json::to_string(&paper).unwrap();
            HttpResponse::Ok().json(paper)
        }
        Err(_) => HttpResponse::Ok().json(false),
    }
}
#[derive(Serialize, Deserialize)]
struct PaperListReuest {
    id: i32,
}
async fn get_paper_list_by_id(data: web::Json<PaperListReuest>) -> impl Responder {
    let result = get_paper_list_by_user_id(data.id).await;
    match result {
        Ok(list) => return HttpResponse::Ok().json(list),
        Err(_) => HttpResponse::Ok().json(false),
    }
}

async fn get_answer_by_question_id_handler(data: web::Json<QuestionRequest>) -> impl Responder {
    let result = get_answer_by_question_id(data.into_inner()).await;
    match result {
        Ok(list) => return HttpResponse::Ok().json(list),
        Err(_) => HttpResponse::Ok().json(false),
    }
}


async fn get_answer_list_by_paper_id_handler(data: web::Json<AnswerListRequest>) -> impl Responder {
    let result = get_answer_list_by_paper_id(data.into_inner()).await;
    match result {
        Ok(list) => return HttpResponse::Ok().json(list),
        Err(_) => HttpResponse::Ok().json(false),
    }
}


pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(index))
        .route("/about", web::get().to(about))
        .route("/contact", web::get().to(contact))
        .route("/submit", web::post().to(submit))
        .route("/login", web::post().to(post_login))
        .route("/login", web::get().to(login))
        .route("/setAnswer", web::post().to(set_answer))
        .route("/getPaperList", web::post().to(get_paper_list_by_id))
        .route("/paper", web::post().to(paper))
        .route("/test", web::post().to(get_paper_list_by_id))
        .route("/getQuestion", web::post().to(get_answer_by_question_id_handler))
        .route("/getAnswer", web::post().to(get_answer));
}
