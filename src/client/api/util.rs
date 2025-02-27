use dubbo::{codegen::Request};
use crate::protos;
use protos::{greeter_client::GreeterClient, GreeterReply, GreeterRequest, LoginReply, LoginRequest};
use once_cell::sync::Lazy;
use std::sync::Arc;


static CLIENT: Lazy<GreeterClient> =
    Lazy::new(|| GreeterClient::new().with_uri("http://127.0.0.1:8888".to_string()));

async fn make_request() {
    let cli = &*CLIENT; // 共享并重用客户端

    // 使用 cli 发送请求
    // cli.some_method().await;
}
// #[tokio::main]
// async fn call_dubbo_service() -> String {
//     let mut cli = GreeterClient::new().with_uri("http://127.0.0.1:8888".to_string());
//     let resp: dubbo::codegen::Response<GreeterReply> = cli
//         .greet(Request::new(GreeterRequest {
//             name: "hello, I'm client".to_string(),
//         }))
//         .await
//         .unwrap();

//     let (_, msg) = resp.into_parts();
//     println!("response: {:?}", msg);
//     return msg.message;
// }

// 验证登录是否成功

pub async fn authenticate(code: i32, password: String) -> bool {
    let mut cli = CLIENT.clone(); // 共享并重用客户端
    let resp: dubbo::codegen::Response<LoginReply> = cli
        .authenticate(Request::new(LoginRequest {
            code: code,
            password: password,
        }))
        .await
        .unwrap();
    let (_, msg) = resp.into_parts();
    return msg.message;
}
// 假设这是一个 Dubbo 客户端的同步调用
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
