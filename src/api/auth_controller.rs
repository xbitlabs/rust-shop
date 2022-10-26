use std::any::Any;
use std::string::ToString;
use std::thread;
use hyper::StatusCode;
use lazy_static::lazy_static;
use log::error;
use crate::{MysqlPoolManager, MysqlPoolStateProvider, RequestCtx, ResponseBuilder, EndpointResult};
use crate::core::{RequestStateResolver, EndpointResultCode};
use crate::service::auth_service::AuthService;
use crate::entity::entity::UserJwt;

pub struct AuthController;

lazy_static! {
    static ref ACCESS_TOKEN: &'static str = "ACCESS_TOKEN";
    static ref REFRESH_TOKEN: &'static str = "REFRESH_TOKEN";
}

impl AuthController {
    pub async fn login(ctx:RequestCtx) ->anyhow::Result<hyper::Response<hyper::Body>> {
        let pool_manager : &MysqlPoolManager = RequestStateResolver::get(&ctx);

        let auth_service = AuthService::new(pool_manager);
        let js_code = ctx.query_params.get("js_code");
        if js_code.is_none() {
            let endpoint_result:EndpointResult<String> = EndpointResult::client_error("登录失败".to_string());
            error!("没有传入js_code参数，获取token失败");
            return Ok(ResponseBuilder::with_endpoint_result(&endpoint_result))
        }
        let login_result = auth_service.wechat_login(js_code.unwrap().to_string()).await?;

        return if login_result.success {
            let endpoint_result = EndpointResult::ok_with_payload(login_result.msg.as_str().to_string(), login_result);
            Ok(ResponseBuilder::with_endpoint_result(&endpoint_result))
        } else {
            let endpoint_result = EndpointResult::client_error_with_payload(login_result.msg.as_str().to_string(), login_result);
            Ok(ResponseBuilder::with_endpoint_result(&endpoint_result))
        }
    }
    pub async fn logout(ctx:RequestCtx) ->anyhow::Result<hyper::Response<hyper::Body>> {
        let pool_manager : &MysqlPoolManager = RequestStateResolver::get(&ctx);
        let auth_service = AuthService::new(pool_manager);
        let token_option = ctx.request.headers().get(ACCESS_TOKEN.to_string());

        if let Some(token) = token_option {
            let logout_result = auth_service.logout(token.to_str().unwrap().to_string()).await?;
            if logout_result {
                Ok(ResponseBuilder::with_text("退出成功".to_string(), EndpointResultCode::SUCCESS))
            } else {
                Ok(ResponseBuilder::with_text("退出失败".to_string(), EndpointResultCode::ClientError))
            }
        } else {
            Ok(ResponseBuilder::with_text("退出失败".to_string(), EndpointResultCode::ClientError))
        }
    }
    pub async fn refresh_token(ctx:RequestCtx) ->anyhow::Result<hyper::Response<hyper::Body>> {
        let pool_manager : &MysqlPoolManager = RequestStateResolver::get(&ctx);
        let auth_service = AuthService::new(pool_manager);
        let refresh_token_option = ctx.request.headers().get(REFRESH_TOKEN.to_string());

        if let Some(token) = refresh_token_option {
            let access_token = auth_service.refresh_token(token.to_str().unwrap().to_string()).await?;
            let endpoint_result = EndpointResult::ok_with_payload("刷新成功".to_string(), access_token);
            Ok(ResponseBuilder::with_endpoint_result(&endpoint_result))
        } else {
            Ok(ResponseBuilder::with_text("刷新token失败".to_string(), EndpointResultCode::ClientError))
        }
    }
}