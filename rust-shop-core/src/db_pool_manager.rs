use std::any::Any;
use std::borrow::BorrowMut;
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;
use anyhow::anyhow;
use hyper::{Body, Request};
use lazy_static::lazy_static;
use sqlx::{Database, Error, MySql, MySqlPool, PgPool, Pool, Postgres, Transaction};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::postgres::PgPoolOptions;
use crate::state::State;
use crate::app_config::load_mod_config;
use crate::extensions::Extensions;
use crate::RequestStateProvider;

#[derive(Debug,serde::Serialize, serde::Deserialize)]
pub struct DbConfig {
    pub url: String,
    pub idle_timeout:Option<u64>,
    pub max_connections:Option<u32>,
    pub min_connections:Option<u32>,
    pub max_lifetime:Option<u64>,
    pub acquire_timeout:Option<u64>,
}

lazy_static! {
    ///
    /// 全局配置
    ///
    pub static ref DB_CONFIG: DbConfig = load_mod_config(String::from("db")).unwrap();
}

pub async fn mysql_connection_pool() -> Result<Pool<MySql>, Error> {
    let db_config = &DB_CONFIG;
    let mut options = MySqlPoolOptions::new();
    if db_config.idle_timeout.is_some() {
        options = options.idle_timeout(Duration::from_micros(db_config.idle_timeout.unwrap()));
    }
    if db_config.max_connections.is_some() {
        options = options.max_connections(db_config.max_connections.unwrap());
    }
    if db_config.min_connections.is_some() {
        options = options.min_connections(db_config.min_connections.unwrap());
    }
    if db_config.acquire_timeout.is_some() {
        options = options.acquire_timeout(Duration::from_micros(db_config.acquire_timeout.unwrap()));
    }
    if db_config.max_lifetime.is_some() {
        options = options.max_lifetime(Duration::from_micros(db_config.max_lifetime.unwrap()));
    }
    let pool = options.connect(db_config.url.as_str());
    pool.await
}
pub async fn postgres_connection_pool() -> Result<Pool<Postgres>, Error> {
    let db_config = &DB_CONFIG;
    let mut options = PgPoolOptions::new();
    if db_config.idle_timeout.is_some() {
        options = options.idle_timeout(Duration::from_micros(db_config.idle_timeout.unwrap()));
    }
    if db_config.max_connections.is_some() {
        options = options.max_connections(db_config.max_connections.unwrap());
    }
    if db_config.min_connections.is_some() {
        options = options.min_connections(db_config.min_connections.unwrap());
    }
    if db_config.acquire_timeout.is_some() {
        options = options.acquire_timeout(Duration::from_micros(db_config.acquire_timeout.unwrap()));
    }
    if db_config.max_lifetime.is_some() {
        options = options.max_lifetime(Duration::from_micros(db_config.max_lifetime.unwrap()));
    }
    let pool = options.connect(db_config.url.as_str());
    pool.await
}

impl <'a> Deref for DbPoolManager<'a,MySql> {
    type Target = Option<Transaction<'a,MySql>>;

    fn deref(&self) -> &Self::Target {
        &self.tran
    }
}

pub struct DbPoolManager<'a,Db:Database>{
    tran:Option<Transaction<'a,Db>>,
    pool_state:Option<State<Pool<Db>>>
}

impl <'a,Db:Database> DbPoolManager<'a,Db> {
    pub fn new(pool_state:State<Pool<Db>>) ->Self{
        DbPoolManager {
            pool_state : Some(pool_state),
            tran:None
        }
    }
    pub fn get_pool(&self) -> &Pool<Db> {
        self.pool_state.as_ref().unwrap().as_ref()
    }
    pub async fn begin(&mut self)->anyhow::Result<&Transaction<'a,Db>>{
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

    pub fn use_tran(&self)->bool{
        return self.tran.is_some()
    }
}



pub struct MysqlPoolStateProvider;

impl <'a> RequestStateProvider for  MysqlPoolStateProvider{
    fn get_state(&self, extensions: &Arc<Extensions>, req: &Request<Body>) -> Box<dyn Any + Send + Sync> {
        let pool_state : &State<Pool<MySql>> = extensions.get().unwrap();
        Box::new(State::new(DbPoolManager::new(pool_state.clone())))
    }
}
