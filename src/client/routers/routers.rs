use crate::{api::util::{self, authenticate, call_dubbo_service, get_paper_by_id}, protos::Paper};
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
    // println!("{}",form.name);
    // let response = format!("Name: {}, Age: {}", form.name, form.age);
    // HttpResponse::Ok().body(response)
    // let login_data = data.into_inner(); // 反序列化 JSON 数据
    let auth = authenticate(data.code, data.password.clone()).await;

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
async fn test(data:web::Json<PaperRequest>) -> impl Responder {
    let result = get_paper_by_id(data.id).await;
    match result {
        Ok(paper) => {

            // serde_json::to_string(&paper).unwrap();
            // let paper_bytes = paper.encode_to_vec();
            // serde_json::to_string(&paper).unwrap();
            HttpResponse::Ok().json(paper)
        },
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
        .route("/test", web::post().to(test));
}
