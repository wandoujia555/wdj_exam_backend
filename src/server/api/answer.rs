// // 通过paperId和answer存储回答
// #[post("/answer", data = "<answer>")]
// pub async fn answer(answer: Json<Answer>) -> Result<Json<Answer>, Error> {
//     let mut con = GLOBAL_REDIS
//         .lock()
//         .await
//         .get()
//         .expect("Failed to get Redis connection");

//     let result: Result<String, _> = con.get(format!("paper_{}", answer.paper_id));

//     let paper_result = match result {
//         Ok(data) if !data.is_empty() => {
//             let paper: Paper = serde_json::from_str(&data).unwrap();
//             let mut paper = paper;
//             let mut question_list = paper.content.get_mut(&answer.question_name).unwrap();
//             let question = question_list.content.get_mut(answer.question_index as usize).unwrap();
//             question.answer = answer.answer.clone();
//             let data = serde_json::to_string(&paper).unwrap();
//             con.set(format!("paper_{}", answer.paper_id), data).unwrap();
//             Ok(paper)
//         }
//         _ => Err(Error::NotFound),
//     };

//     match paper_result {
//         Ok(paper) => Ok(Json(answer.into_inner())),
//         Err(_) => Err(Error::NotFound),
//     }
// }