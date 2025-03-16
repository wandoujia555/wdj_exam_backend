use std::collections::HashMap;

use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::{
    protos::{Paper, Question, QuestionList, QuestionType},
    service::{mysql::GLOBAL_DATA, redis::GLOBAL_REDIS},
};
use mysql_async::{prelude::*, Params, Row, Value};
use prost::Message;
use redis::Commands;

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
    drop(global_data);
    if results.is_empty() {
        return Ok(None);
    }

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
    let _: () = con
        .set(format!("paper_{}", id), Paper::encode_to_vec(&paper))
        .unwrap();
    Ok(Some(paper))
}

// 通过用户id查询试卷列表 (分页)
pub async fn query_paper_list_by_id() {
    println!("1");
    let mut con = GLOBAL_REDIS
        .lock()
        .await
        .get()
        .expect("Failed to get Redis connection");
    let class = query_class_by_id(2).await;

    // let paperList = query_paperList_by_class(1);
}

// 通过用户id查询class
pub async fn query_class_by_id(id:i32) -> Result<Option<i32>, mysql_async::Error>  {
    // let mut con = GLOBAL_REDIS
    //     .lock()
    //     .await
    //     .get()
    //     .expect("Failed to get Redis connection");
    let query: &str = "SELECT 
            class.id
        from
            class
        JOIN
            class_stu cs on class.id = cs.class_id
        JOIN
            student s on cs.student_id = s.id
        WHERE
            s.id = ? and class.is_deleted = 1;";
    // println!("aa{:?}",results);
    let global_data = GLOBAL_DATA.lock().await;
    let mut conn = global_data.get_conn().await?;
    let results: Vec<Row> = conn.exec(query, (id,)).await?;
    drop(global_data);
    if results.is_empty() {
        return Ok(None);
    }

    let mut class_ids: Vec<i32> = Vec::new();

    for row in results {
        class_ids.push(row.get("id").unwrap());
    }
    println!("aa{:?}",class_ids);
    let _ = query_paper_list_by_class(class_ids).await;

    return Ok(None);
    // if results.is_empty() {
    //     return Ok(None);
    // }

}

// 通过class获取试卷列表
pub async fn query_paper_list_by_class(ids: Vec<i32>) -> Result<Option<Paper>, mysql_async::Error> {
    // let mut con = GLOBAL_REDIS
    //     .lock()
    //     .await
    //     .get()
    //     .expect("Failed to get Redis connection");

    // let result: Result<String, _> = con.get(format!("paper_{:?}", ids));

    // let paper_result = match result {
    //     Ok(data) if !data.is_empty() => {
    //         // match serde_json::from_str::<Paper>(&data) {
    //         //     Ok(paper) => Ok(Some(paper)),
    //         //     Err(e) => Err(Box::new(e)),
    //         // }
    //         match Paper::decode(data.as_bytes()) {
    //             Ok(paper) => Ok(Some(paper)),
    //             Err(e) => Err(Box::new(e)),
    //         }
    //     }
    //     Ok(_) => Ok(None),
    //     Err(_) => Ok(None),
    // };
    
    
    // if let Ok(Some(paper)) = paper_result {
    //     println!("{:?}", paper);
    //     return Ok(Some(paper));
    // }
    
    
    let query = format!("SELECT paper.id,paper.name,paper.desc,paper.startTime,paper.createTime,paper.updateTime,paper.duration,paper.total
        FROM paper
                JOIN paper_class pc on paper.id = pc.paper_id
                JOIN class c on pc.class_id = c.id
        WHERE c.id in ({}) and paper.is_deleted = 1", ids.iter().map(|_| "?").collect::<Vec<_>>().join(", "));

    let global_data = GLOBAL_DATA.lock().await;
    let mut conn = global_data.get_conn().await?;

    let params = Params::Positional(ids.iter().map(|id| Value::from(*id)).collect());
    let results: Vec<Row> = conn.exec(query,params).await?;
    drop(global_data);

    if results.is_empty() {
        return  Ok(None);
    }



    Ok(None)
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

// 用户:
// 发送答案

// 查看试卷列表
// 可选考试
// 添加评论
// 查看评论

// 教师
// 新建试卷
// 新建大题
// 新建题目
// 获取答案
// 添加评论
// 查看评论
// 导出excel分数
