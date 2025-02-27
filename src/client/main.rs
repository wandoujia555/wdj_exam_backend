pub mod protos {
    #![allow(non_camel_case_types)]
    include!(concat!("../", "/greeter.rs"));
}
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dubbo::{codegen::Request, triple::transport::router};
use protos::{greeter_client::GreeterClient, GreeterReply, GreeterRequest, LoginReply, LoginRequest};
use tokio::task;

use dotenv::dotenv;
use std::env;

// mod jwt;
mod routers;
fn main() {
    dotenv().ok(); // 加载 .env 文件
                   // JWT_SECRET_KEY
    match env::var("MY_VARIABLE") {
        Ok(value) => println!("MY_VARIABLE is set to: {}", value),
        Err(_) => println!("MY_VARIABLE is not set"),
    }

    let data = tokio::runtime::Runtime::new().unwrap().block_on(async {
        handler().await;
    });
    start_server();
}

// 假设这是一个 Dubbo 客户端的同步调用
#[tokio::main]
async fn call_dubbo_service() -> String {
    let mut cli = GreeterClient::new().with_uri("http://127.0.0.1:8888".to_string());
    let resp: dubbo::codegen::Response<GreeterReply> = cli
        .greet(Request::new(GreeterRequest {
            name: "hello, I'm client".to_string(),
        }))
        .await
        .unwrap();

    let (_, msg) = resp.into_parts();
    println!("response: {:?}", msg);
    return msg.message;
}

// 异步处理器，调用 Dubbo 服务
async fn handler() -> impl Responder {
    // match task::spawn(call_dubbo_service()).await {
    //     Ok(result) => {
    //         println!("异步任务返回的结果: {}", result);
    //         // 在这里执行任务完成后的操作
    //         println!("执行任务完成后的操作");
    //     }
    //     Err(e) => {
    //         eprintln!("异步任务发生错误: {:?}", e);
    //     }
    // };

    let result = task::spawn_blocking(|| call_dubbo_service()).await.unwrap();

    HttpResponse::Ok().body(result)
}
mod middleware;
use actix_cors::Cors;
#[actix_web::main] // 使用 actix 的异步运行时
async fn start_server() -> std::io::Result<()> {
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let port: u16 = port.parse().expect("PORT must be a valid port number");

    println!("Starting server on port {}", port);

    // 启动 HTTP 服务器，监听 port 端口
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::auth::Auth)
            .wrap(
                Cors::default()
                    // .allowed_origin("http://localhost:3000")
                    .allow_any_origin() // 允许的来源，通常是前端应用的地址
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"]) // 允许的 HTTP 方法
                    // .allowed_headers(vec!["Content-Type", "Authorization"]) // 允许的请求头
                    .allow_any_header()
                    .max_age(3600),
            )
            .configure(routers::routers::configure_routes)
    })
    .bind(("localhost", port))? // 绑定地址和端口
    .run()
    .await
}
