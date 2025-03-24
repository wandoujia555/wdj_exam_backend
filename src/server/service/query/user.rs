use dubbo::codegen::{Request, Response};
use serde::Deserialize;

use crate::{
    protos::{login_reply::Message, LoginReply, LoginRequest},
    service::mysql::GLOBAL_DATA,
};
use mysql_async::{prelude::*, Pool, Row};

#[derive(Deserialize, Debug)]
pub struct User {
    code: i32,
    name: String,
    password: String,
    privileges: i32,
}
impl User {
    pub fn get_password(&self) -> &str {
        &self.password // 只允许读取密码，不暴露字段本身
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
}
pub async fn query_student_by_code(student_code: i32) -> Result<Option<User>, mysql_async::Error> {
    let global_data = GLOBAL_DATA.lock().await;
    let query = "SELECT code, name, password FROM student WHERE code = ?";
    let mut conn = global_data.get_conn().await?;
    let result: Option<Row> = conn.exec_first(query, (student_code,)).await?;

    if let Some(row) = result {
        let student = User {
            code: row.get("code").unwrap(),
            name: row.get("name").unwrap(),
            password: row.get("password").unwrap(),
            privileges: 0,
        };
        Ok(Some(student))
    } else {
        Ok(None)
    }
}

pub async fn query_teacher_by_code(teacher_code: i32) -> Result<Option<User>, mysql_async::Error> {
    let global_data = GLOBAL_DATA.lock().await;
    let query = "SELECT code, name, password FROM teacher WHERE code = ?";
    let mut conn = global_data.get_conn().await?;
    let result: Option<Row> = conn.exec_first(query, (teacher_code,)).await?;

    if let Some(row) = result {
        let student = User {
            code: row.get("code").unwrap(),
            name: row.get("name").unwrap(),
            password: row.get("password").unwrap(),
            privileges: 1,
        };
        Ok(Some(student))
    } else {
        Ok(None)
    }
}

pub async fn authenticate(
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
