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
use hyper::{Body, Request, Response, StatusCode};
use lazy_static::lazy_static;
use log::info;
use crate::api::index_controller::IndexController;
use snowflake::SnowflakeIdGenerator;
use std::sync::{Arc, Mutex};
use sqlx::{MySql, Pool};
use rust_shop_core::{AccessLogFilter, EndpointResult, Filter, Next, RequestCtx, RequestStateProvider, ResponseBuilder, Server};
use rust_shop_core::db_pool_manager::{get_connection_pool, MysqlPoolManager};
use rust_shop_core::extensions::Extensions;
use rust_shop_core::extract::{FromRequest, IntoResponse};
use rust_shop_core::extract::json::Json;
use rust_shop_core::router::ROUTER;
use rust_shop_core::security::{AuthenticationTokenResolver, AuthenticationTokenResolverFn, LoadUserService, LoadUserServiceFn, SecurityConfig, WeChatMiniAppAuthenticationTokenResolver, WeChatUserService};
use rust_shop_core::state::State;
use rust_shop_core::router::register_route;
use rust_shop_core::security::NopPasswordEncoder;


pub struct  AuthFilter;

#[async_trait::async_trait]
impl Filter for AuthFilter {
    async fn handle<'a>(&'a self, ctx: RequestCtx, next: Next<'a>) -> anyhow::Result<hyper::Response<hyper::Body>> {
        let endpoint_result:EndpointResult<String> = EndpointResult::server_error("无权限");
        Ok(ResponseBuilder::with_endpoint_result(endpoint_result))
    }

    fn url_patterns(&self) -> String {
        todo!()
    }
}


//use crate::api::auth_controller::AuthController;
use crate::api::static_file_controller::StaticFileController;
use crate::api::upload_controller::UploadController;
use crate::config::load_config::APP_CONFIG;

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


lazy_static! {
    static ref b : bool = register_route(String::from("post"),String::from("/test"),IndexController::index);
}

lazy_static! {
    static ref c : bool = register_route(String::from("get"),String::from("/test2"),StaticFileController::handle);
}

#[tokio::main]
async fn main() ->anyhow::Result<()>{

    println!("Hello, world!{}",std::any::type_name::<Json<User>>());
    //println!("The map has {} entries.", *b);
    //println!("The map has {} entries.", *c);
    lazy_static::initialize(&b);
    lazy_static::initialize(&c);

    println!("len = {}",ROUTER.lock().unwrap().len());

    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("booting up");

    let addr: SocketAddr = format!("127.0.0.1:{}",&APP_CONFIG.server.port).parse().unwrap();

    let mut srv = Server::new();

    srv.filter(AccessLogFilter);

    let conn_pool = get_connection_pool().await?;
    srv.extension(State::new(conn_pool.clone()));

    let mut security_config = SecurityConfig::new();
    security_config.enable_security(true);
    security_config.authentication_token_resolver(
        AuthenticationTokenResolverFn::from(
        Box::new(|request_states: &Arc<Extensions>, app_extensions: &Arc<Extensions>| -> Box<dyn AuthenticationTokenResolver + Send + Sync>{
            Box::new(WeChatMiniAppAuthenticationTokenResolver{})
        })));
    security_config.password_encoder(Box::new(NopPasswordEncoder{}));
    security_config.load_user_service(
        LoadUserServiceFn::from(
            Box::new(|request_states: &Arc<Extensions>, app_extensions: &Arc<Extensions>| -> Box<dyn LoadUserService + Send + Sync>{
                let state: Option<&Box<dyn Any + Send + Sync>> = request_states.get();
                let state: Option<&MysqlPoolManager> = state.unwrap().downcast_ref();
                let pool = state.unwrap();
                Box::new(WeChatUserService::new(pool))
            }))
    );
    srv.security_config(security_config);

    let mysql_pool_state_provider : Box<dyn RequestStateProvider + Sync + Send> = Box::new(MysqlPoolStateProvider);
    srv.request_state(mysql_pool_state_provider);

    srv.post("/", IndexController::index);
    //登录
    //srv.post("/login", AuthController::login);
    //srv.post("/logout", AuthController::logout);
    //srv.post("/refresh_token",AuthController::refresh_token);
    //上传
    srv.post("/upload",UploadController::upload);
    //静态文件
    srv.get("/static/:day/:file",StaticFileController::handle);
    srv.run(addr).await.unwrap();

    info!("server shutdown!");
    Ok(())
}

#[derive(serde::Serialize,serde::Deserialize)]
pub struct User{
    pub username:String,
    pub age:u32
}

pub async fn create_user(Json(user):Json<User>)->anyhow::Result<Json<EndpointResult<&'static str>>>{
    Ok(Json(EndpointResult::ok_with_payload("新增成功","")))
}
pub async fn create_user_handler(ctx:RequestCtx)->anyhow::Result<Response<Body>>{
    let extract_result = Json::from_request(ctx).await?;
    let result = create_user(extract_result).await?;
    Ok(result.into_response())
}