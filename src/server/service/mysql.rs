// 查找试卷
use mysql_async::{prelude::*, Pool, Row};
use std::{env, sync::Arc};
use tokio;

pub struct Database {
    pub pool: Pool,
}

impl Database {
    // 创建一个新的数据库连接池
    pub fn new(url: &str) -> Self {
        let pool = Pool::new(url);
        Database { pool }
    }

    // 执行查询并返回结果
    pub async fn query(&self, query: &str) -> Result<Vec<(i32, String)>, mysql_async::Error> {
        let mut conn = self.pool.get_conn().await?;
        let results: Vec<(i32, String)> = conn.query(query).await?;
        Ok(results)
    }

    // 插入数据
    pub async fn insert_user(pool: &Pool, name: &str) -> Result<i32, mysql_async::Error> {
        let mut conn = pool.get_conn().await?;
        let query = "INSERT INTO teacher (username) VALUES (?)";
        let result = conn.exec_drop(query, (name,)).await?;
        print!("{:?}", result);
        Ok(1)
        // Ok(result.last_insert_id().unwrap_or(0))
    }
    pub async fn get_conn(&self) -> Result<mysql_async::Conn, mysql_async::Error> {
        let conn = self.pool.get_conn().await?;
        Ok(conn)
    }
}

use once_cell::sync::Lazy;
use tokio::sync::Mutex;

pub static GLOBAL_DATA: Lazy<Arc<Mutex<Database>>> = Lazy::new(|| {
    let url = env::var("DATABASE_URL").expect("MY_VARIABLE is not set");
    Arc::new(Mutex::new(Database::new(url.as_str())))
});

#[tokio::main]
pub async fn get_per() {
    // let results: Vec<(i32, String)> = GLOBAL_DATA
    //     .lock()
    //     .await
    //     .query("SELECT id, name FROM teacher")
    //     .await
    //     .unwrap();

    // // let query = "SELECT id, name, email FROM users WHERE id = ?";
    // // let mut conn = pool.get_conn().await?;
    // // let result: Option<Row> = conn.exec_first(query, (user_id,)).await?;

    // // if let Some(row) = result {
    // //     let user = User {
    // //         id: row.get("id").unwrap(),
    // //         name: row.get("name").unwrap(),
    // //         email: row.get("email").unwrap(),
    // //     };
    // //     Ok(Some(user))
    // // } else {
    // //     Ok(None)
    // // }
    // // 数据库连接字符串
    // // let url = "mysql://root:123456@localhost/exam";

    // // // 创建连接池
    // // let pool = Pool::new(url);

    // // // 从连接池中获取连接
    // // let mut conn = pool.get_conn().await.unwrap();

    // // // 执行查询
    // // let results: Vec<(i32, String)> = conn.query("SELECT id, name FROM teacher").await.unwrap();

    // // // 处理查询结果
    // for (id, name) in results {
    //     println!("User ID: {}, Name: {}", id, name);
    // }
}

/*
    试卷:
    试卷名称
    题目数组:[题目类型ID]
    试卷分数
    考试时间
    开考时间
    是否随机题目顺序
*/

/*
    题目
    题目类型(多选，问答，填空，判断)
    题目内容
    参考答案(可能多个,如答案错误的情况)
    多题中随机抽取:boolean
*/

/*
    考生
    试卷数组:[试卷ID]


*/

// 题目类型
//
