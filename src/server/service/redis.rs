use r2d2::{ManageConnection, Pool};
use redis::{Client, Commands, RedisResult};
use std::time::Duration;
pub struct RedisManager {
    client: Client,
}
impl RedisManager {
    fn new() -> Self {
        RedisManager {
            client: Client::open("redis://127.0.0.1/").expect("Invalid Redis URL"),
        }
    }
}
impl ManageConnection for RedisManager {
    type Connection = redis::Connection;
    type Error = redis::RedisError;

    fn connect(&self) -> Result<Self::Connection, Self::Error> {
        self.client.get_connection()
    }

    fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        conn.ping()?;
        Ok(())
    }

    fn has_broken(&self, conn: &mut Self::Connection) -> bool {
        conn.get::<_, String>("ping").is_err()
    }
}
use once_cell::sync::Lazy;
use std::sync::Mutex;
type MyPool = Pool<RedisManager>;
pub static GLOBAL_REDIS: Lazy<Mutex<MyPool>> = Lazy::new(|| {
    Mutex::new(
        Pool::builder()
            .max_size(10)
            .connection_timeout(Duration::new(5, 0))
            .build(RedisManager::new())
            .expect("Failed to create Redis pool"),
    )
});

#[tokio::main]
pub async fn redisi_init() -> RedisResult<()> {
    // let manager = RedisManager::new();
    let mut con = GLOBAL_REDIS
        .lock()
        .unwrap()
        .get()
        .expect("Failed to get Redis connection");
    // let pool: Pool<RedisManager> = Pool::builder()
    //     .max_size(10)
    //     .connection_timeout(Duration::new(5, 0))
    //     .build(manager)
    //     .expect("Failed to create Redis pool");

    // let mut con: r2d2::PooledConnection<RedisManager> = pool.get().expect("Failed to get Redis connection");

    let _: () = con.set("my_key", 42)?;
    let result: i32 = con.get("my_key")?;
    println!("The value of 'my_key' is: {}", result);

    Ok(())
}

// pub fn redisi_init() -> RedisResult<()> {
//     // 连接到 Redis 服务器
//     let client = redis::Client::open("redis://127.0.0.1/")?;
//     let mut con = client.get_connection()?; // 获取一个同步连接

//     // 设置一个键值对
//     let _: () = con.set("my_key", 42)?;

//     // 获取该键值对
//     let result: i32 = con.get("my_key")?;
//     println!("The value of 'my_key' is: {}", result);

//     Ok(())
// }
