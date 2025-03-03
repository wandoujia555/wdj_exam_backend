
use serde::Deserialize;

use crate::service::mysql::GLOBAL_DATA;
use mysql_async::{Pool, Row, prelude::*};

#[derive(Deserialize, Debug)]
pub struct Student {
    code: i32,
    name: String,
    password: String,
}
impl Student {
    pub fn get_password(&self) -> &str {
        &self.password  // 只允许读取密码，不暴露字段本身
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
}
pub async fn query_student_by_code(student_code: i32) -> Result<Option<Student>, mysql_async::Error> {
    let global_data = GLOBAL_DATA.lock().await;
    let query = "SELECT code, name, password FROM student WHERE code = ?";
    let mut conn = global_data.get_conn().await?;
    let result: Option<Row> = conn.exec_first(query, (student_code,)).await?;

    if let Some(row) = result {
        let student = Student {
            code: row.get("code").unwrap(),
            name: row.get("name").unwrap(),
            password: row.get("password").unwrap(),
        };
        Ok(Some(student))
    } else {
        Ok(None)
    }
}

