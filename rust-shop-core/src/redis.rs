use crate::app_config::load_mod_config;

use lazy_static::lazy_static;
use log::error;
use redis::{
    Commands, Connection, ConnectionAddr, ConnectionInfo, RedisConnectionInfo, RedisError,
    RedisResult,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RedisConfig {
    pub db: i64,
    pub server: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
}

lazy_static! {
    ///
    /// 全局配置
    ///
    pub static ref REDIS_CONFIG: RedisConfig = load_mod_config(String::from("redis")).unwrap();
}

fn connection() -> RedisResult<Connection> {
    let conn_info = ConnectionInfo {
        addr: ConnectionAddr::Tcp(REDIS_CONFIG.server.clone(), REDIS_CONFIG.port),
        redis: RedisConnectionInfo {
            db: REDIS_CONFIG.db,
            username: REDIS_CONFIG.username.clone(),
            password: REDIS_CONFIG.password.clone(),
        },
    };
    let client = redis::Client::open(conn_info)?;
    client.get_connection()
}
pub async fn set<T: serde::Serialize>(key: &str, value: &T) -> RedisResult<()> {
    let mut conn = connection();
    let json = serde_json::to_string(value);
    if json.is_ok() {
        println!("设置成功{}", key);
        conn?.set(key, json.unwrap())
    } else {
        Err(RedisError::from(json.err().unwrap()))
    }
}
pub async fn get<T: for<'a> serde::Deserialize<'a>>(key: &str) -> RedisResult<T> {
    let mut conn = connection();
    let result: RedisResult<String> = conn?.get(key);
    if result.is_ok() {
        let s = result.unwrap().clone();
        let mut deserializer = serde_json::Deserializer::from_str(s.as_str());
        let result: Result<T, serde_json::Error> = T::deserialize(&mut deserializer);
        if result.is_ok() {
            Ok(result.unwrap())
        } else {
            Err(RedisError::from(result.err().unwrap()))
        }
    } else {
        Err(RedisError::from(result.err().unwrap()))
    }
}
pub async fn remove(key: &str) -> bool {
    let mut conn = connection();
    if conn.is_err() {
        error!("获取redis链接失败：{}", conn.err().unwrap());
        return false;
    }
    let result: RedisResult<String> = conn.unwrap().del(key);
    return match result {
        Ok(_) => true,
        Err(err) => {
            error!("删除redis key 失败：{}", err);
            false
        }
    };
}
