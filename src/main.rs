#![feature(try_trait_v2)]

pub mod api;
pub mod service;
pub mod entity;
pub mod utils;
mod vo;
mod filter;
mod request;
mod config;
mod core;
mod state;
mod extensions;


use std::any::Any;
use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, StatusCode};
use lazy_static::lazy_static;
use log::info;
use crate::core::{AccessLogFilter, Filter, MysqlPoolManager, Next, RequestCtx, RequestStateProvider, ResponseBuilder, EndpointResult, Server};
use crate::api::index_controller::IndexController;
use snowflake::SnowflakeIdGenerator;
use std::sync::{Arc, Mutex};
use sqlx::{MySql, Pool};



lazy_static::lazy_static! {
    pub static ref ID_GENERATOR : Mutex<SnowflakeIdGenerator> = Mutex::new(SnowflakeIdGenerator::new(1, 1));
}

pub struct  AuthFilter;

#[async_trait::async_trait]
impl Filter for AuthFilter {
    async fn handle<'a>(&'a self, ctx: RequestCtx, next: Next<'a>) -> anyhow::Result<hyper::Response<hyper::Body>> {
        let endpoint_result:EndpointResult<String> = EndpointResult::server_error("无权限".to_string());
        Ok(ResponseBuilder::with_endpoint_result(&endpoint_result))
    }
}


use crate::api::auth_controller::AuthController;
use crate::api::static_file_controller::StaticFileController;
use crate::api::upload_controller::UploadController;
use crate::config::load_config::{APP_CONFIG, load_conf};
use crate::extensions::Extensions;
use crate::state::State;
use crate::utils::db::get_connection_pool;

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

#[tokio::main]
async fn main() ->anyhow::Result<()>{

    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("booting up");

    let addr: SocketAddr = format!("127.0.0.1:{}",&APP_CONFIG.server.port).parse().unwrap();

    let mut srv = Server::new();

    srv.filter(AccessLogFilter);

    let conn_pool = get_connection_pool().await?;
    srv.extension(State::new(conn_pool.clone()));

    let mysql_pool_state_provider : Box<dyn RequestStateProvider + Sync + Send> = Box::new(MysqlPoolStateProvider);
    srv.request_state(mysql_pool_state_provider);

    srv.post("/", IndexController::index);
    //登录
    srv.post("/login", AuthController::login);
    srv.post("/logout", AuthController::logout);
    srv.post("/refresh_token",AuthController::refresh_token);
    //上传
    srv.post("/upload",UploadController::upload);
    //静态文件
    srv.get("/static/:day/:file",StaticFileController::handle);
    srv.run(addr).await.unwrap();

    info!("server shutdown!");
    Ok(())
}
