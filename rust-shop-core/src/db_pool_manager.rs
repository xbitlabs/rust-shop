use std::any::Any;
use std::sync::Arc;
use hyper::{Body, Request};
use lazy_static::lazy_static;
use sqlx::{Error, MySql, MySqlPool, Pool, Transaction};
use crate::state::State;
use crate::app_config::load_mod_config;
use crate::extensions::Extensions;
use crate::RequestStateProvider;

#[derive(Debug,serde::Serialize, serde::Deserialize)]
pub struct MysqlConfig {
    pub host: String,
    pub port: u32,
    pub user: String,
    pub password: String,
    pub db: String,
    pub pool_min_idle: u64,
    pub pool_max_open: u64,
    pub timeout_seconds: u64,
}

lazy_static! {
    ///
    /// 全局配置
    ///
    pub static ref MYSQL_CONFIG: MysqlConfig = load_mod_config(String::from("db")).unwrap();
}

pub async fn get_connection_pool() -> Result<Pool<MySql>, Error> {
    println!("获取mysql连接池");
    let mysql_config = &MYSQL_CONFIG;
    let conn = format!("mysql://{}:{}@{}:{}/{}?useUnicode=true&characterEncoding=utf8&zeroDateTimeBehavior=convertToNull&useSSL=true&serverTimezone=Asia/Shanghai",
                       mysql_config.user,
                       mysql_config.password,
                       mysql_config.host,
                       mysql_config.port,
                       mysql_config.db);
    let pool = MySqlPool::connect(conn.as_str());
    pool.await
}

pub struct MysqlPoolManager<'a>{
    tran:Option<Transaction<'a,MySql>>,
    pool_state:Option<State<Pool<MySql>>>
}

impl <'a> MysqlPoolManager<'a> {
    pub fn new(pool_state:State<Pool<MySql>>) ->Self{
        MysqlPoolManager{
            pool_state : Some(pool_state),
            tran:None
        }
    }
    pub fn get_pool(&self) -> &Pool<MySql> {
        self.pool_state.as_ref().unwrap().as_ref()
    }
    pub async fn begin(&mut self)->anyhow::Result<&Transaction<'a,MySql>>{
        return if self.tran.is_some() {
            let tran = &self.tran.as_ref().unwrap();
            Ok(tran)
        } else {
            let tran = self.get_pool().begin().await?;
            self.tran = Some(tran);
            let tran = &self.tran.as_ref().unwrap();
            Ok(tran)
        }
    }
}

impl <'a> Drop for MysqlPoolManager<'a> {
    fn drop(&mut self) {
        println!("释放MysqlPoolManager");
    }
}


pub struct MysqlPoolStateProvider;

impl <'a> RequestStateProvider for  MysqlPoolStateProvider{
    fn get_state(&self, extensions: &Arc<Extensions>, req: &Request<Body>) -> Box<dyn Any + Send + Sync> {
        let pool_state : &State<Pool<MySql>> = extensions.get().unwrap();
        Box::new(MysqlPoolManager::new(pool_state.clone()))
    }
}
impl Drop for MysqlPoolStateProvider{
    fn drop(&mut self) {
        println!("释放MysqlPoolStateProvider");
    }
}