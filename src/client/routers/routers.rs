use actix_web::{web, App, HttpResponse, HttpServer, Responder};

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
    HttpResponse::Ok().body("Hello, world!")
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

use serde::{Deserialize, Serialize};

use crate::middleware::auth::{generate_jwt, User};
// #[derive(Deserialize, Serialize)]
// pub struct LoginData {
//     code: String,
//     password: String,
// }

#[derive(Serialize)]
struct JwtResponse {
    token: String,
}

async fn post_login(data: web::Json<User>) -> impl Responder {
    // println!("{}",form.name);
    // let response = format!("Name: {}, Age: {}", form.name, form.age);
    // HttpResponse::Ok().body(response)

    // let login_data = data.into_inner(); // 反序列化 JSON 数据

    let login_data =  generate_jwt(&data);

    if login_data.is_err() {
        return  HttpResponse::Ok().body("create jwt error")
    }
    let login_data_value = JwtResponse {
        token: login_data.unwrap(),
    };
    
    HttpResponse::Ok().json(login_data_value) // 返回解析后的数据作为 JSON 响应
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(index))
        .route("/about", web::get().to(about))
        .route("/contact", web::get().to(contact))
        .route("/submit", web::post().to(submit))
        .route("/login", web::post().to(post_login))
        .route("/login", web::get().to(login));
}
