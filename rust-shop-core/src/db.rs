use std::any::Any;
use std::borrow::BorrowMut;
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;

use anyhow::anyhow;
use hyper::{Body, Request};
use lazy_static::lazy_static;
use sqlx::mysql::{MySqlArguments, MySqlPoolOptions, MySqlRow};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Acquire, Database, Error, MySql, MySqlPool, PgPool, Pool, Postgres, Transaction};

use crate::app_config::load_mod_config;
use crate::extensions::Extensions;
use crate::state::State;
use crate::RequestStateProvider;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DbConfig {
    pub url: String,
    pub idle_timeout: Option<u64>,
    pub max_connections: Option<u32>,
    pub min_connections: Option<u32>,
    pub max_lifetime: Option<u64>,
    pub acquire_timeout: Option<u64>,
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
        options =
            options.acquire_timeout(Duration::from_micros(db_config.acquire_timeout.unwrap()));
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
        options =
            options.acquire_timeout(Duration::from_micros(db_config.acquire_timeout.unwrap()));
    }
    if db_config.max_lifetime.is_some() {
        options = options.max_lifetime(Duration::from_micros(db_config.max_lifetime.unwrap()));
    }
    let pool = options.connect(db_config.url.as_str());
    pool.await
}

pub struct TransactionManager {
    tran: Transaction<'static, MySql>,
    rollback_only: bool,
}

impl TransactionManager {
    pub fn new(tran: Transaction<'static, MySql>) -> Self {
        TransactionManager {
            tran,
            rollback_only: false,
        }
    }
    pub async fn rollback(mut self) -> anyhow::Result<()> {
        self.tran.rollback().await?;
        Ok(())
    }
    pub async fn commit(mut self) -> anyhow::Result<()> {
        if self.rollback_only {
            return Err(anyhow!("current transaction support rollback only"));
        }
        self.tran.commit().await?;
        Ok(())
    }
    pub fn transaction(&mut self) -> &mut Transaction<'static, MySql> {
        self.tran.borrow_mut()
    }
    pub fn rollback_only(&mut self) {
        self.rollback_only = true;
    }
}

pub enum SqlCommandExecutor<'db, 'a> {
    WithTransaction(&'a mut TransactionManager),
    WithoutTransaction(&'db Pool<MySql>),
}

impl<'db, 'a> SqlCommandExecutor<'db, 'a> {
    pub async fn execute(&mut self, query: &str) -> anyhow::Result<u64> {
        return match self {
            Self::WithTransaction(ref mut tran_manager) => {
                let result = sqlx::query(query)
                    .execute(tran_manager.transaction())
                    .await?;
                Ok(result.rows_affected())
            }
            Self::WithoutTransaction(pool) => {
                let result = sqlx::query(query).execute(*pool).await?;
                Ok(result.rows_affected())
            }
        };
    }
    pub async fn execute_with(&mut self, query: &str, args: MySqlArguments) -> anyhow::Result<u64> {
        return match self {
            Self::WithTransaction(ref mut tran_manager) => {
                let result = sqlx::query_with(query, args)
                    .execute(tran_manager.transaction())
                    .await?;
                Ok(result.rows_affected())
            }
            Self::WithoutTransaction(pool) => {
                let result = sqlx::query_with(query, args).execute(*pool).await?;
                Ok(result.rows_affected())
            }
        };
    }
    pub async fn find_one<T>(&mut self, query: &str) -> anyhow::Result<T>
    where
        T: for<'r> sqlx::FromRow<'r, MySqlRow>,
        T: Send,
        T: Unpin,
    {
        return match self {
            Self::WithTransaction(ref mut tran_manager) => {
                let result: T = sqlx::query_as(query)
                    .fetch_one(tran_manager.transaction())
                    .await?;
                Ok(result)
            }
            Self::WithoutTransaction(pool) => {
                let result: T = sqlx::query_as(query).fetch_one(*pool).await?;
                Ok(result)
            }
        };
    }
    pub async fn find_one_with<T>(&mut self, query: &str, args: MySqlArguments) -> anyhow::Result<T>
    where
        T: for<'r> sqlx::FromRow<'r, MySqlRow>,
        T: Send,
        T: Unpin,
    {
        return match self {
            Self::WithTransaction(ref mut tran_manager) => {
                let result: T = sqlx::query_as_with(query, args)
                    .fetch_one(tran_manager.transaction())
                    .await?;
                Ok(result)
            }
            Self::WithoutTransaction(pool) => {
                let result: T = sqlx::query_as_with(query, args).fetch_one(*pool).await?;
                Ok(result)
            }
        };
    }
    pub async fn find_option<T>(&mut self, query: &str) -> anyhow::Result<Option<T>>
    where
        T: for<'r> sqlx::FromRow<'r, MySqlRow>,
        T: Send,
        T: Unpin,
    {
        return match self {
            Self::WithTransaction(ref mut tran_manager) => {
                let result: Option<T> = sqlx::query_as(query)
                    .fetch_optional(tran_manager.transaction())
                    .await?;
                Ok(result)
            }
            Self::WithoutTransaction(pool) => {
                let result: Option<T> = sqlx::query_as(query).fetch_optional(*pool).await?;
                Ok(result)
            }
        };
    }
    pub async fn find_option_with<T>(
        &mut self,
        query: &str,
        args: MySqlArguments,
    ) -> anyhow::Result<Option<T>>
    where
        T: for<'r> sqlx::FromRow<'r, MySqlRow>,
        T: Send,
        T: Unpin,
    {
        return match self {
            Self::WithTransaction(ref mut tran_manager) => {
                let result: Option<T> = sqlx::query_as_with(query, args)
                    .fetch_optional(tran_manager.transaction())
                    .await?;
                Ok(result)
            }
            Self::WithoutTransaction(pool) => {
                let result: Option<T> = sqlx::query_as_with(query, args)
                    .fetch_optional(*pool)
                    .await?;
                Ok(result)
            }
        };
    }
    pub async fn find_all<T>(&mut self, query: &str) -> anyhow::Result<Vec<T>>
    where
        T: for<'r> sqlx::FromRow<'r, MySqlRow>,
        T: Send,
        T: Unpin,
    {
        return match self {
            Self::WithTransaction(ref mut tran_manager) => {
                let result: Vec<T> = sqlx::query_as(query)
                    .fetch_all(tran_manager.transaction())
                    .await?;
                Ok(result)
            }
            Self::WithoutTransaction(pool) => {
                let result: Vec<T> = sqlx::query_as(query).fetch_all(*pool).await?;
                Ok(result)
            }
        };
    }
    pub async fn find_all_with<T>(
        &mut self,
        query: &str,
        args: MySqlArguments,
    ) -> anyhow::Result<Vec<T>>
    where
        T: for<'r> sqlx::FromRow<'r, MySqlRow>,
        T: Send,
        T: Unpin,
    {
        return match self {
            Self::WithTransaction(ref mut tran_manager) => {
                let result: Vec<T> = sqlx::query_as_with(query, args)
                    .fetch_all(tran_manager.transaction())
                    .await?;
                Ok(result)
            }
            Self::WithoutTransaction(pool) => {
                let result: Vec<T> = sqlx::query_as_with(query, args).fetch_all(*pool).await?;
                Ok(result)
            }
        };
    }
    pub async fn scalar_one<T>(&mut self, query: &str) -> anyhow::Result<T>
    where
        T: sqlx::Type<sqlx::MySql> + for<'r> sqlx::Decode<'r, sqlx::MySql>,
        T: Send,
        T: Unpin,
    {
        return match self {
            Self::WithTransaction(ref mut tran_manager) => {
                let result: T = sqlx::query_scalar(query)
                    .fetch_one(tran_manager.transaction())
                    .await?;
                Ok(result)
            }
            Self::WithoutTransaction(pool) => {
                let result: T = sqlx::query_scalar(query).fetch_one(*pool).await?;
                Ok(result)
            }
        };
    }
    pub async fn scalar_one_with<T>(
        &mut self,
        query: &str,
        args: MySqlArguments,
    ) -> anyhow::Result<T>
    where
        T: sqlx::Type<sqlx::MySql> + for<'r> sqlx::Decode<'r, sqlx::MySql>,
        T: Send,
        T: Unpin,
    {
        return match self {
            Self::WithTransaction(ref mut tran_manager) => {
                let result: T = sqlx::query_scalar_with(query, args)
                    .fetch_one(tran_manager.transaction())
                    .await?;
                Ok(result)
            }
            Self::WithoutTransaction(pool) => {
                let result: T = sqlx::query_scalar_with(query, args)
                    .fetch_one(*pool)
                    .await?;
                Ok(result)
            }
        };
    }
    pub async fn scalar_option<T>(&mut self, query: &str) -> anyhow::Result<Option<T>>
    where
        T: sqlx::Type<sqlx::MySql> + for<'r> sqlx::Decode<'r, sqlx::MySql>,
        T: Send,
        T: Unpin,
    {
        return match self {
            Self::WithTransaction(ref mut tran_manager) => {
                let result: Option<T> = sqlx::query_scalar(query)
                    .fetch_optional(tran_manager.transaction())
                    .await?;
                Ok(result)
            }
            Self::WithoutTransaction(pool) => {
                let result: Option<T> = sqlx::query_scalar(query).fetch_optional(*pool).await?;
                Ok(result)
            }
        };
    }
    pub async fn scalar_option_with<T>(
        &mut self,
        query: &str,
        args: MySqlArguments,
    ) -> anyhow::Result<Option<T>>
    where
        T: sqlx::Type<sqlx::MySql> + for<'r> sqlx::Decode<'r, sqlx::MySql>,
        T: Send,
        T: Unpin,
    {
        return match self {
            Self::WithTransaction(ref mut tran_manager) => {
                let result: Option<T> = sqlx::query_scalar_with(query, args)
                    .fetch_optional(tran_manager.transaction())
                    .await?;
                Ok(result)
            }
            Self::WithoutTransaction(pool) => {
                let result: Option<T> = sqlx::query_scalar_with(query, args)
                    .fetch_optional(*pool)
                    .await?;
                Ok(result)
            }
        };
    }
    pub async fn scalar_all<T>(&mut self, query: &str) -> anyhow::Result<Vec<T>>
    where
        T: sqlx::Type<sqlx::MySql> + for<'r> sqlx::Decode<'r, sqlx::MySql>,
        T: Send,
        T: Unpin,
    {
        return match self {
            Self::WithTransaction(ref mut tran_manager) => {
                let result: Vec<T> = sqlx::query_scalar(query)
                    .fetch_all(tran_manager.transaction())
                    .await?;
                Ok(result)
            }
            Self::WithoutTransaction(pool) => {
                let result: Vec<T> = sqlx::query_scalar(query).fetch_all(*pool).await?;
                Ok(result)
            }
        };
    }
    pub async fn scalar_all_with<T>(
        &mut self,
        query: &str,
        args: MySqlArguments,
    ) -> anyhow::Result<Vec<T>>
    where
        T: sqlx::Type<sqlx::MySql> + for<'r> sqlx::Decode<'r, sqlx::MySql>,
        T: Send,
        T: Unpin,
    {
        return match self {
            Self::WithTransaction(ref mut tran_manager) => {
                let result: Vec<T> = sqlx::query_scalar_with(query, args)
                    .fetch_all(tran_manager.transaction())
                    .await?;
                Ok(result)
            }
            Self::WithoutTransaction(pool) => {
                let result: Vec<T> = sqlx::query_scalar_with(query, args)
                    .fetch_all(*pool)
                    .await?;
                Ok(result)
            }
        };
    }
}

/*pub struct MysqlPoolStateProvider;

impl <'a> RequestStateProvider for  MysqlPoolStateProvider{
    fn get_state(&self, extensions: &Arc<Extensions>, req: &Request<Body>) -> Box<dyn Any + Send + Sync> {
        let pool_state : &State<Pool<MySql>> = extensions.get().unwrap();
        Box::new(State::new(DbPoolManager::new(pool_state.clone())))
    }
}*/
