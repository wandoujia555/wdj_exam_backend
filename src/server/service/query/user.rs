
use serde::Deserialize;
use crate::database::GLOBAL_DATA;
#[derive(Deserialize, Debug)]
struct Student {
    code: i32,
    name: String,
    password: String,
}
async fn query_student_by_id(student_code: i32) -> Result<Option<Student>, mysql_async::Error> {
    

    let global_data = GLOBAL_DATA.lock().unwrap();
    let query = "SELECT code, name, password FROM student WHERE code = ?";
    let mut conn = global_data.pool.get_conn().await?;
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