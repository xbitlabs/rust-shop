use crate::app_config::load_mod_config;
use crate::security::{DefaultAuthenticationToken, DefaultSecurityContext, DefaultUserDetails};
use crate::DefaultAuthentication;
use anyhow::anyhow;
use lazy_static::lazy_static;
use log::error;
use redis::{
    Commands, Connection, ConnectionAddr, ConnectionInfo, JsonCommands, RedisConnectionInfo,
    RedisError, RedisResult,
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
macro_rules! aw {
    ($e:expr) => {
        tokio_test::block_on($e)
    };
}
/*
#[test]
fn test_wechat_api(){
    let api = WeChatMiniAppService::new();
    let result = api.get_access_token();
    let result1 = aw!(result);
    println!("{:?}",result1);
}*/
#[test]
fn test() {
    let token = DefaultAuthenticationToken::new("pgg".parse().unwrap(), "168".to_string());
    let user = DefaultUserDetails::new(
        1,
        "pgg".to_string(),
        "168".to_string(),
        vec!["add".to_string(), "update".to_string()],
        true,
    );
    let auth = DefaultAuthentication::new(
        token,
        vec!["add".to_string(), "update".to_string()],
        true,
        Box::new(user),
    );
    let context = DefaultSecurityContext::new(auth);
    let result = aw!(set("test", &context));
    let context: RedisResult<DefaultSecurityContext> = aw!(get("test"));
    if context.is_ok() {
        let context = context.unwrap();
        println!("{}", serde_json::to_string(&context).unwrap());
        println!("ok");
    } else {
        println!("{:?}", context.err().unwrap());
    }
    println!("{:?}", result);
}
