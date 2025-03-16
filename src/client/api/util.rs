use crate::protos::{self, AnswerPaper, AnswerReply, PaperRequest};
use dubbo::codegen::Request;
use once_cell::sync::Lazy;
use protos::{
    greeter_client::GreeterClient, GreeterReply, GreeterRequest, LoginReply, LoginRequest, Paper,
};
use std::sync::Arc;

static CLIENT: Lazy<GreeterClient> =
    Lazy::new(|| GreeterClient::new().with_uri("http://127.0.0.1:8888".to_string()));

async fn make_request() {
    let cli = &*CLIENT; // 共享并重用客户端

    // 使用 cli 发送请求
    // cli.some_method().await;
}

pub async fn authenticate(code: i32, password: String) -> Option<protos::login_reply::Message> {
    let mut cli = CLIENT.clone(); // 共享并重用客户端
    let resp: dubbo::codegen::Response<LoginReply> = cli
        .authenticate(Request::new(LoginRequest {
            code: code,
            password: password,
        }))
        .await
        .unwrap();
    let (_, msg) = resp.into_parts();
    // let a: Option<protos::login_reply::Message> = msg.message;
    return msg.message;
}
// 假设这是一个 Dubbo 客户端的同步调用
pub async fn call_dubbo_service() -> String {
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

// 假设这是一个 Dubbo 客户端的同步调用
pub async fn get_paper_by_id(id:i32) -> Result<Paper, dubbo::status::Status> {
    let mut cli = CLIENT.clone(); // 共享并重用客户端
    let resp: dubbo::codegen::Response<Paper> = cli
        .get_paper_by_id(Request::new(PaperRequest { id }))
        .await?;

    let (_, msg) = resp.into_parts();
    return Ok(msg);
}

pub async fn set_answer_by_id(id: AnswerPaper) -> Result<AnswerReply, dubbo::status::Status> {
    let mut cli = CLIENT.clone(); // 共享并重用客户端
    let resp: dubbo::codegen::Response<AnswerReply> = cli
        .set_answer_by_id(Request::new(id))
        .await?;

    let (_, msg) = resp.into_parts();
    return Ok(msg);
}





// 通过用户id获取试卷列表
pub async fn get_paper_list_by_user_id(){
    
}

// 通过试卷id获取试卷(带答案)
pub async fn get_paper_by_id_with_answer(){

}


// 通过试卷id和用户id获取用户答卷
pub async fn get_answer_by_paper_and_user(){

}

// 通过试卷id和用户id获取分数
pub async fn get_score_by_paper_and_user(){

}

// 通过试卷id和用户id获取评语(分页)
pub async fn get_comment_by_paper_and_user(){

}

