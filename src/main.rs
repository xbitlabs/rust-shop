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
use std::collections::HashMap;
use std::convert::Infallible;
use std::env;
use std::fs::File;
use std::io::Read;
use std::net::SocketAddr;
use std::string::ToString;
use hyper::{Body, Request, Response, StatusCode};
use lazy_static::lazy_static;
use log::info;
use crate::api::index_controller::IndexController;
use snowflake::SnowflakeIdGenerator;
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;
use sqlx::{MySql, Pool};
use syn::__private::ToTokens;
use syn::{Item, ItemMod};
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


/*lazy_static! {
    static ref b : bool = register_route(String::from("post"),String::from("/test"),IndexController::index);
}*/

lazy_static! {
    static ref c : bool = register_route("get","/test2",StaticFileController::handle);
}

macro_rules! source_dir {
    ()=>{
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let mut path = path.to_str().unwrap().to_string();
        if cfg!(target_os = "windows")  {
            path.push_str("\\src");
        }else {
            path.push_str("/src");
        }
        path
    }
}

fn reg_route(){

}
#[tokio::main]
#[rust_shop_macro::rust_shop_app("/src")]
async fn main() ->anyhow::Result<()>{

    let mut file = File::open("D:\\项目\\rust-shop\\src\\api\\auth_controller.rs").expect("Unable to open file");

    let mut src = String::new();
    file.read_to_string(&mut src).expect("Unable to read file");

    let syntax = syn::parse_file(&src).expect("Unable to parse file");
    //println!("{:#?}", syntax);
    for item in syntax.items {
        //let item_mod = item as ItemMod;
        match item {
            Item::Mod(item_mod)=>{
                let mod_ident = item_mod.ident;
                if item_mod.content.is_some() {
                    //println!("{}",item_mod.content.unwrap().1.len());
                    let mod_content_items = item_mod.content.unwrap().1;
                    for mod_content_item in mod_content_items {
                        match mod_content_item {
                            Item::Fn(f)=>{

                            }
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        }

    }


    println!("{}",env!("CARGO_MANIFEST_DIR"));
    //println!("The map has {} entries.", *b);
    //println!("The map has {} entries.", *c);
    //lazy_static::initialize(&b);
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