use std::collections::HashMap;

use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::{protos::{Paper, Question, QuestionList, QuestionType}, service::{mysql::GLOBAL_DATA, redis::GLOBAL_REDIS}};
use redis::Commands;
use mysql_async::{prelude::*, Row};
use prost::Message;
// #[derive(Serialize, Deserialize, Debug)]
// enum QuestionType {
//     Choice,
//     Selection,
//     Interlocution,
//     Judge,
// }

// 题目内容
// #[derive(Serialize, Deserialize, Debug)]
// pub struct Question {
//     id: i32,
//     question_type: QuestionType,
//     name: String,
//     content: String,
//     score: i32,
// }

// 大题类型
// #[derive(Serialize, Deserialize, Debug)]
// pub struct QuestionList {
//     id: i32,
//     question_type: QuestionType,
//     name: String,
//     content: Vec<Question>,
//     question_num: i32,
//     total_score: i32,
// }
// impl QuestionList {
//     pub fn push(&mut self, value: Question) {
//         self.total_score += value.score;
//         self.question_num += 1;
//         self.content.push(value);
//     }
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Paper {
//     id: i32,
//     name: String,
//     question_num: i32,
//     mintues: i32,
//     status: i32,
//     created_time: i32,
//     updata_time: i32,
//     content: HashMap<String, QuestionList>,
// }



pub trait QuestionListExt {
    fn push(&mut self, value: Question);
}
impl QuestionListExt for QuestionList {
    fn push(&mut self, value: Question) {
        self.total_score += value.score;
        self.question_num += 1;
        self.content.push(value);
    }
}



// 通过试卷id获取试卷
pub async fn query_paper_by_id(id: i32) -> Result<Option<Paper>, mysql_async::Error> {

    let mut con = GLOBAL_REDIS
        .lock()
        .await
        .get()
        .expect("Failed to get Redis connection");

    let result: Result<String, _> = con.get(format!("paper_{}", id));

    let paper_result = match result {
        Ok(data) if !data.is_empty() => {
            // match serde_json::from_str::<Paper>(&data) {
            //     Ok(paper) => Ok(Some(paper)),
            //     Err(e) => Err(Box::new(e)),
            // }
            match Paper::decode(data.as_bytes()) {
                Ok(paper) => Ok(Some(paper)),
                Err(e) => Err(Box::new(e)),
            }
        }
        Ok(_) => Ok(None),
        Err(_) => Ok(None),
    };
    if let Ok(Some(paper)) = paper_result {
        println!("{:?}", paper);
        return Ok(Some(paper));
    }
    let query: &str = "SELECT
            p.id AS paper_id,
            p.duration as paper_minutes,
            p.name AS paper_name,
            p.state AS paper_status,
            UNIX_TIMESTAMP(p.updateTime) AS paper_updateTime,
            UNIX_TIMESTAMP(p.createTime) AS paper_createTime,
            UNIX_TIMESTAMP(p.startTime) AS paper_startTime,
            ql.id AS questionList_id,
            ql.name AS questionList_name,
            q.id AS question_id,
            q.content AS question_content,
            q.choose AS question_choose,
            q.score AS question_score
        FROM
            paper p
        LEFT JOIN
            paper_questionList pq ON p.id = pq.paper_id
        LEFT JOIN
            questionList ql ON pq.questionList_id = ql.id and ql.is_deleted = 1
        LEFT JOIN
            questionList_question qlq ON ql.id = qlq.questionList_id
        LEFT JOIN
            question q ON qlq.question_id = q.id  and q.is_deleted = 1
        WHERE
            p.id = ? and p.is_deleted = 1;";
    let global_data = GLOBAL_DATA.lock().await;
    // let query = "SELECT id, name, question, mintues, status FROM paper WHERE id = ?";
    let mut conn = global_data.get_conn().await?;
    let results: Vec<Row> = conn.exec(query, (id,)).await?;
    if results[0].is_empty() {
        return Ok(None);
    }

    if results[0].get::<i64, _>("paper_startTime").unwrap() > Utc::now().timestamp() {
        return Ok(None);
    }
    let mut paper = Paper {
        id: results[0].get("paper_id").unwrap(),
        name: results[0].get("paper_name").unwrap(),
        question_num: 0,
        minutes: results[0].get("paper_minutes").unwrap(),
        status: results[0].get("paper_status").unwrap(),
        created_time: results[0].get("paper_createTime").unwrap(),
        update_time: results[0].get("paper_updateTime").unwrap(),
        content: HashMap::<String, QuestionList>::new(),
    };
    for row in results {
        let question_list_name = match row.get::<Option<String>, _>("questionList_name") {
            Some(Some(value)) => value,
            Some(None) => continue,
            None => continue,
        };
        let question = match row.get::<Option<i32>, _>("question_id") {
            Some(Some(value)) => Question {
                id: value,
                name: row.get("question_content").unwrap(),
                question_type: QuestionType::Choice as i32,
                content: row.get("question_choose").unwrap(),
                score: row.get("question_score").unwrap(),
            },
            Some(None) => continue,
            None => continue,
        };

        if paper.content.contains_key(&question_list_name) {
            paper
                .content
                .get_mut(&question_list_name)
                .unwrap()
                .push(question);
        } else {
            paper.content.insert(
                question_list_name.clone(),
                QuestionList {
                    id: row.get("questionList_id").unwrap(),
                    question_type: QuestionType::Choice as i32,
                    name: question_list_name.clone(),
                    content: Vec::<Question>::new(),
                    question_num: 0,
                    total_score: 0,
                },
            );
        }
    }
    println!("{:?}", paper);
    let _: () = con.set(format!("paper_{}", id), Paper::encode_to_vec(&paper)).unwrap();
    Ok(Some(paper))
}

// 考试内容
pub struct PaperContent {
    id: i32,
    name: String,
    question_num: i32,
    mintues: i32,
    status: i32,
    created_time: i32,
}
impl PaperContent {
    pub fn get_password(&self) -> &i32 {
        &self.question_num
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

// SELECT
//     e.employee_id,
//     e.name AS employee_name,
//     d.department_id,
//     d.department_name
// FROM
//     employees e
// INNER JOIN
//     employee_department ed
// ON
//     e.employee_id = ed.employee_id
// INNER JOIN
//     departments d
// ON
//     ed.department_id = d.department_id
// WHERE
//     e.employee_id = 123;

/*  */
