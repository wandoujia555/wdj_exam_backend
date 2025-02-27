use crate::service;
use crate::service::query::user as user_query;
pub async fn authenticate(student_code:i32,password:String){
    let student = user_query::query_student_by_code(32).await;
    if student.is_err() {
        return println!("error auth");
    }
    println!("{:?}",student);
}