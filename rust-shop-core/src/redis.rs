use crate::app_config::load_mod_config;

use lazy_static::lazy_static;
use log::error;
use redis::{
    Commands, Connection, ConnectionAddr, ConnectionInfo, RedisConnectionInfo, RedisError,
    RedisResult,
};
use serde::{Deserialize, Serialize};


pub extern crate r2d2;
pub extern crate redis;

use std::error;
use std::error::Error as _StdError;
use std::fmt;
use r2d2::PooledConnection;

use redis::ConnectionLike;

/// A unified enum of errors returned by redis::Client
#[derive(Debug)]
pub enum Error {
    /// A redis::RedisError
    Other(redis::RedisError),
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        #[allow(deprecated)] // `cause` is replaced by `Error:source` in 1.33
        match self.cause() {
            Some(cause) => write!(fmt, "{}: {}", self.description(), cause),
            None => write!(fmt, "{}", self.description()),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Other(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            Error::Other(ref err) => {
                #[allow(deprecated)] // `cause` is replaced by `Error:source` in 1.33
                err.cause()
            },
        }
    }
}

/// An `r2d2::ConnectionManager` for `redis::Client`s.
///
/// ## Example
///

/// ```
/// extern crate r2d2_redis;
///
/// use std::ops::DerefMut;
/// use std::thread;
///
/// use r2d2_redis::{r2d2, redis, RedisConnectionManager};
///
/// fn main() {
///     let manager = RedisConnectionManager::new("redis://localhost").unwrap();
///     let pool = r2d2::Pool::builder()
///         .build(manager)
///         .unwrap();
///
///     let mut handles = vec![];
///
///     for _i in 0..10i32 {
///         let pool = pool.clone();
///         handles.push(thread::spawn(move || {
///             let mut conn = pool.get().unwrap();
///             let reply = redis::cmd("PING").query::<String>(conn.deref_mut()).unwrap();
///             // Alternatively, without deref():
///             // let reply = redis::cmd("PING").query::<String>(&mut *conn).unwrap();
///             assert_eq!("PONG", reply);
///         }));
///     }
///
///     for h in handles {
///         h.join().unwrap();
///     }
/// }
/// ```
#[derive(Debug)]
pub struct RedisConnectionManager {
    connection_info: redis::ConnectionInfo,
}

impl RedisConnectionManager {
    /// Creates a new `RedisConnectionManager`.
    ///
    /// See `redis::Client::open` for a description of the parameter
    /// types.
    pub fn new<T: redis::IntoConnectionInfo>(
        params: T,
    ) -> Result<RedisConnectionManager, redis::RedisError> {
        Ok(RedisConnectionManager {
            connection_info: params.into_connection_info()?,
        })
    }
}

impl r2d2::ManageConnection for RedisConnectionManager {
    type Connection = redis::Connection;
    type Error = Error;

    fn connect(&self) -> Result<redis::Connection, Error> {
        match redis::Client::open(self.connection_info.clone()) {
            Ok(client) => client.get_connection().map_err(Error::Other),
            Err(err) => Err(Error::Other(err)),
        }
    }

    fn is_valid(&self, conn: &mut redis::Connection) -> Result<(), Error> {
        redis::cmd("PING").query(conn).map_err(Error::Other)
    }

    fn has_broken(&self, conn: &mut redis::Connection) -> bool {
        !conn.is_open()
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RedisConfig {
    pub db: i64,
    pub server: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
}
fn load_redis_config()->ConnectionInfo {
    let conn_info = ConnectionInfo {
        addr: ConnectionAddr::Tcp(REDIS_CONFIG.server.clone(), REDIS_CONFIG.port),
        redis: RedisConnectionInfo {
            db: REDIS_CONFIG.db,
            username: REDIS_CONFIG.username.clone(),
            password: REDIS_CONFIG.password.clone(),
        },
    };
    conn_info
}
lazy_static! {
    ///
    /// 全局配置
    ///
    pub static ref REDIS_CONFIG: RedisConfig = load_mod_config(String::from("redis")).unwrap();
    pub static ref REDIS_POOL:r2d2::Pool<RedisConnectionManager> = r2d2::Pool::builder()
        .build(RedisConnectionManager::new(load_redis_config()).unwrap())
        .unwrap();
}

pub async fn set<T: serde::Serialize>(key: &str, value: &T) -> RedisResult<()> {
    let mut conn = REDIS_POOL.get().unwrap();
    let json = serde_json::to_string(value);
    if json.is_ok() {
        println!("设置成功{}", key);
        conn.set(key, json.unwrap())
    } else {
        Err(RedisError::from(json.err().unwrap()))
    }
}
pub async fn get<T: for<'a> serde::Deserialize<'a>>(key: &str) -> RedisResult<T> {
    let mut conn = REDIS_POOL.get().unwrap();
    let result: RedisResult<String> = conn.get(key);
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
    let mut conn = REDIS_POOL.get().unwrap();
    let result: RedisResult<String> = conn.del(key);
    return match result {
        Ok(_) => true,
        Err(err) => {
            error!("删除redis key 失败：{}", err);
            false
        }
    };
}
