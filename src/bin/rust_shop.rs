#![feature(try_trait_v2)]

use std::net::SocketAddr;
use std::string::ToString;
use log::info;

use rust_shop_core::db::{mysql_connection_pool, SqlCommandExecutor};

use rust_shop_core::router::register_route;
use rust_shop_core::security::{
    AdminUserLoadService, AuthenticationFilter, AuthenticationProcessingFilter,
    SecurityInterceptor, Sha512PasswordEncoder, UsernamePasswordAuthenticationTokenResolver,
};
use rust_shop_core::security::{
    AuthenticationTokenResolver, AuthenticationTokenResolverFn, LoadUserService, LoadUserServiceFn,
    WebSecurityConfigurer,
};
use rust_shop_core::state::State;
use rust_shop_core::{AccessLogFilter, EndpointResult, RequestCtx, ResponseBuilder, Server};

use crate::api::demo_controller;
use crate::api::static_file_controller;
use crate::api::upload_controller;

use crate::config::load_config::APP_CONFIG;

pub mod api;
mod config;
pub mod entity;
mod filter;
mod request;
pub mod service;
pub mod utils;
mod vo;

fn load_user_service_fn<'r, 'a, 'b>(
    sql_command_executor: &'r mut SqlCommandExecutor<'a, 'b>,
) -> Box<dyn LoadUserService + Send + Sync + 'r> {
    AdminUserLoadService::new(sql_command_executor)
}

//https://crates.io/crates/message-io#app-list
//开源socket库
#[tokio::main]
#[rust_shop_macro::scan_route("/src")]
async fn main() -> anyhow::Result<()> {
    /*let mut file = File::open("D:\\项目\\rust-shop\\src\\api\\auth_controller.rs").expect("Unable to open file");

    let mut src = String::new();
    file.read_to_string(&mut src).expect("Unable to read file");

    let syntax = syn::parse_file(&src).expect("Unable to parse file");
    println!("{:#?}", syntax);*/

    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("booting up");

    let addr: SocketAddr = format!("127.0.0.1:{}", &APP_CONFIG.server.port)
        .parse()
        .unwrap();

    let mut srv = Server::new();

    srv.filter(AuthenticationFilter);
    srv.filter(AccessLogFilter);
    srv.filter(AuthenticationProcessingFilter);
    srv.filter(SecurityInterceptor);

    let conn_pool = mysql_connection_pool().await?;
    srv.extension(State::new(conn_pool.clone()));

    let mut security_config = WebSecurityConfigurer::new();
    security_config.enable_security(false);
    security_config.authentication_token_resolver(AuthenticationTokenResolverFn::from(Box::new(
        || -> Box<dyn AuthenticationTokenResolver + Send + Sync> {
            Box::new(UsernamePasswordAuthenticationTokenResolver {})
        },
    )));
    security_config.password_encoder(Box::new(Sha512PasswordEncoder));
    security_config.load_user_service(LoadUserServiceFn::from(Box::new(
        |_req: &mut RequestCtx| -> Box<
            dyn for<'r, 'c, 'd> Fn(
                &'r mut SqlCommandExecutor<'c, 'd>,
            ) -> Box<(dyn LoadUserService + Send + Sync + 'r)>
            + Send
            + Sync,
        > { Box::new(load_user_service_fn) },
    )));
    srv.security_config(security_config);
    srv.run(addr).await.unwrap();

    info!("server shutdown!");
    Ok(())
}
