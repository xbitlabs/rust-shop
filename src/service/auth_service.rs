use std::cell::RefCell;
use std::ops::Deref;
use std::string::ToString;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::SystemTime;
use time::{Duration, OffsetDateTime};
use chrono::Local;
use hyper::{Client, Method, Request, Uri};
use crate::config::load_config::APP_CONFIG;
use hyper::body::Buf;
use hyper_tls::HttpsConnector;
use lazy_static::lazy_static;
use sqlx::encode::IsNull::No;
use crate::entity::entity::User;
use crate::service::wechat_service::WeChatMiniAppService;
use jsonwebtoken::{Algorithm, DecodingKey, encode, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use crate::service::jwt_service::{DefaultJwtService};
use chrono::prelude::*;
use sqlx::{Error, MySql, Pool};
use crate::entity::entity::UserJwt;
use crate::{ID_GENERATOR, MysqlPoolManager};
use crate::core::{AccessToken, JwtService};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LoginResult{
    pub success:bool,
    pub msg:String,
    pub token:Option<AccessToken>,
}


pub struct AuthService<'a,'b>{
    wechat_service:WeChatMiniAppService,
    jwt_service: &'a (dyn JwtService  + Send + Sync),
    mysql_pool_manager: &'a MysqlPoolManager<'b>
}
impl <'a,'b> AuthService<'a,'b> {
    pub fn new(mysql_pool_manager:&'b MysqlPoolManager,jwt_service:&'a (dyn JwtService  + Send + Sync))->Self{
        AuthService{
            wechat_service:WeChatMiniAppService::new(),
            jwt_service,
            mysql_pool_manager
        }
    }
    pub async fn wechat_login(&self,js_code:String)->anyhow::Result<LoginResult> {
        let longin_response = self.wechat_service.login(js_code).await?;
        if longin_response.errcode.is_none() && longin_response.errmsg.is_none() {
            let pool = self.mysql_pool_manager.get_pool();
            let users = sqlx::query_as!(User,"select * from user where wx_open_id=?",
            longin_response.openid.as_ref())
                .fetch_all(pool).await?;
            if users.len() > 0 {
                let access_token = self.jwt_service.grant_access_token(users[0].id).await?;
                Ok(LoginResult {
                    success: true,
                    msg: "登录成功".to_string(),
                    token: Some(access_token)
                })
            } else {
                let id: i64 = ID_GENERATOR.lock().unwrap().real_time_generate();
                let rows_affected = sqlx::query!("insert into `user`(id,wx_open_id,created_time) values(?,?,?)",
                id,
                longin_response.openid,
                Local::now())
                    .execute(pool).await?
                    .rows_affected();

                let access_token = self.jwt_service.grant_access_token(id).await?;
                Ok(LoginResult {
                    success: true,
                    msg: "登录成功".to_string(),
                    token: Some(access_token)
                })
            }
        } else {
            Ok(LoginResult {
                success: false,
                msg: "登录失败".to_string(),
                token: None
            })
        }
    }
    pub async fn logout(&self,token:String)->anyhow::Result<bool>{
        self.jwt_service.remove_access_token(token).await
    }
    pub async fn refresh_token(&self,refresh_token:String)->anyhow::Result<AccessToken>{
        println!("当前线程id={:?}",thread::current().id());
        self.jwt_service.refresh_token(refresh_token).await
    }
}
