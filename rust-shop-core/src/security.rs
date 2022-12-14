use std::any::Any;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::fmt::Display;

use std::string::ToString;

use anyhow::anyhow;
use base64ct::{Base64, Encoding};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Local;
use erased_serde::serialize_trait_object;

use lazy_static::lazy_static;
use log::{error, info};

use redis::RedisResult;
use serde::Deserializer;
use sha2::{Digest, Sha512};
use sqlx::mysql::MySqlArguments;
use sqlx::{Arguments, MySql, Pool};
use thiserror::Error;
use url::Url;
use urlpattern::{UrlPattern, UrlPatternInit, UrlPatternMatchInput};

use crate::app_config::load_mod_config;
use crate::db::{SqlCommandExecutor, TransactionManager};
use crate::entity::{AdminPermission, AdminRole, AdminUser, User};

use crate::id_generator::ID_GENERATOR;
use crate::jwt::{decode_access_token, DefaultJwtService};
use crate::jwt::{AccessToken, JwtService};

use crate::response::Response;
use crate::state::State;
use crate::wechat::WeChatMiniAppService;
use crate::{
    parse_form_params, EndpointResult, Filter, Next, RequestCtx, ResponseBuilder, APP_EXTENSIONS,
};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SecurityConfig {
    allow_if_all_abstain_decisions: Option<bool>,
    intercept_url_patterns: Option<String>,
    security_context_storage_strategy: Option<String>,
    security_metadata_source_storage_key: String,
    cookie_based: bool,
    password_salt: String,
}

lazy_static! {
    ///
    /// 全局配置
    ///
    pub static ref SECURITY_CONFIG: SecurityConfig = load_mod_config(String::from("security")).unwrap();
}

#[async_trait::async_trait]
pub trait LoadUserService {
    async fn load_user(
        &mut self,
        identity: &String,
    ) -> anyhow::Result<Box<dyn UserDetails + Send + Sync>>;
}

//获取登陆凭证
#[async_trait::async_trait]
pub trait AuthenticationTokenResolver {
    async fn resolve(
        &self,
        ctx: &mut RequestCtx,
    ) -> anyhow::Result<Box<dyn AuthenticationToken + Send + Sync>>;
}

pub struct UsernamePasswordAuthenticationTokenResolver {}

impl UsernamePasswordAuthenticationTokenResolver {
    pub fn new() -> Self {
        UsernamePasswordAuthenticationTokenResolver {}
    }
}

#[async_trait::async_trait]
impl AuthenticationTokenResolver for UsernamePasswordAuthenticationTokenResolver {
    async fn resolve(
        &self,
        ctx: &mut RequestCtx,
    ) -> anyhow::Result<Box<dyn AuthenticationToken + Send + Sync>> {
        let params: HashMap<String, String> = parse_form_params(ctx).await;
        let username = params.get("username");
        if username.is_none() {
            return Err(anyhow!("必须传入username字段"));
        }
        let password = params.get("password");
        if password.is_none() {
            return Err(anyhow!("必须传入password字段"));
        }
        Ok(Box::new(UsernamePasswordAuthenticationToken::new(
            username.unwrap().to_string(),
            password.unwrap().to_string(),
        )))
    }
}

pub struct WeChatMiniAppAuthenticationToken {
    pub js_code: String,
    pub principal: String,
}

impl AuthenticationToken for WeChatMiniAppAuthenticationToken {
    fn get_principal(&self) -> &(dyn Display + Sync + Send) {
        &self.principal
    }
    fn get_credentials(&self) -> &(dyn Display + Sync + Send) {
        &self.js_code
    }
}

pub struct WeChatMiniAppAuthenticationTokenResolver {}

#[async_trait::async_trait]
impl AuthenticationTokenResolver for WeChatMiniAppAuthenticationTokenResolver {
    async fn resolve(
        &self,
        req: &mut RequestCtx,
    ) -> anyhow::Result<Box<dyn AuthenticationToken + Send + Sync>> {
        let params: HashMap<String, String> = parse_form_params(req).await;
        let js_code = params.get("jsCode");
        if js_code.is_none() {
            return Err(anyhow!("必须传入jsCode字段"));
        } else {
            let js_code = js_code.unwrap().trim().to_string();
            if js_code.is_empty() {
                return Err(anyhow!("jsCode不能为空"));
            } else {
                let wechat_service = WeChatMiniAppService::new();
                let login_result = wechat_service.login(js_code.clone()).await?;
                if login_result.errcode.is_none() && login_result.errmsg.is_none() {
                    Ok(Box::new(WeChatMiniAppAuthenticationToken {
                        js_code,
                        principal: login_result.openid.unwrap(),
                    }))
                } else {
                    return Err(anyhow!(login_result.errmsg.unwrap()));
                }
            }
        }
    }
}

pub struct WeChatUserService<'r, 'a, 'b> {
    sql_command_executor: &'r mut SqlCommandExecutor<'a, 'b>,
}

impl<'r, 'a, 'b> WeChatUserService<'r, 'a, 'b> {
    pub fn new(
        sql_command_executor: &'r mut SqlCommandExecutor<'a, 'b>,
    ) -> Box<dyn LoadUserService + Send + Sync + 'r> {
        Box::new(WeChatUserService {
            sql_command_executor,
        })
    }
}

#[async_trait::async_trait]
impl<'r, 'a, 'b> LoadUserService for WeChatUserService<'r, 'a, 'b> {
    async fn load_user(
        &mut self,
        identity: &String,
    ) -> anyhow::Result<Box<dyn UserDetails + Send + Sync>> {
        let mut args = MySqlArguments::default();
        args.add(identity.to_string());
        let user: Option<User> = self
            .sql_command_executor
            .find_option_with("select * from user where wx_open_id=?", args)
            .await?;
        if user.is_some() {
            let user = user.unwrap();
            let username: String = if user.username.is_some() {
                user.username.unwrap()
            } else {
                String::from("")
            };
            let password: String = if user.password.is_some() {
                user.password.unwrap()
            } else {
                String::from("")
            };
            Ok(Box::new(DefaultUserDetails {
                id: user.id,
                username,
                password,
                authorities: vec![],
                enable: true,
            }))
        } else {
            let id: i64 = ID_GENERATOR.lock().unwrap().real_time_generate();
            let mut args = MySqlArguments::default();
            args.add(id);
            args.add(identity.to_string());
            args.add(Local::now());
            args.add(1);
            let rows_affected = self
                .sql_command_executor
                .execute_with(
                    "insert into `user`(id,wx_open_id,created_time,enable) values(?,?,?,?)",
                    args,
                )
                .await?;
            if rows_affected > 0 {
                Ok(Box::new(DefaultUserDetails {
                    id,
                    username: String::from(""),
                    password: String::from(""),
                    authorities: vec![],
                    enable: true,
                }))
            } else {
                return Err(anyhow!("保存微信用户信息失败"));
            }
        }
    }
}

pub struct AdminUserLoadService<'r, 'a, 'b> {
    sql_command_executor: &'r mut SqlCommandExecutor<'a, 'b>,
}

impl<'r, 'a, 'b> AdminUserLoadService<'r, 'a, 'b> {
    pub fn new(
        sql_command_executor: &'r mut SqlCommandExecutor<'a, 'b>,
    ) -> Box<dyn LoadUserService + Send + Sync + 'r> {
        Box::new(AdminUserLoadService {
            sql_command_executor,
        })
    }
}
#[async_trait::async_trait]
impl<'r, 'a, 'b> LoadUserService for AdminUserLoadService<'r, 'a, 'b> {
    async fn load_user(
        &mut self,
        identity: &String,
    ) -> anyhow::Result<Box<dyn UserDetails + Send + Sync>> {
        let start = Local::now().timestamp_millis();
        let mut args = MySqlArguments::default();
        args.add(identity.to_string());
        let user: Option<AdminUser> = self
            .sql_command_executor
            .find_option_with("select * from admin_user where username=?", args)
            .await?;
        let end = Local::now().timestamp_millis();
        println!(
            "select * from admin_user where username=? 耗时：{}",
            end - start
        );
        if user.is_some() {
            let start = Local::now().timestamp_millis();
            let user = user.unwrap();
            let mut args = MySqlArguments::default();
            args.add(user.id);
            let admin_user_roles: Vec<AdminRole> = self
                .sql_command_executor
                .find_all_with(
                    r#"SELECT
                            r.*
                        FROM
                            admin_role r
                            JOIN admin_user_role ur ON r.id = ur.admin_role_id
                        WHERE
                            ur.admin_user_id = ?"#,
                    args,
                )
                .await?;
            let mut authorities = vec![];
            for admin_user_role in admin_user_roles {
                authorities.push(admin_user_role.code);
            }
            let end = Local::now().timestamp_millis();
            println!("查询角色耗时：{}", end - start);
            Ok(Box::new(DefaultUserDetails {
                id: user.id,
                username: user.username,
                password: user.password,
                authorities,
                enable: true,
            }))
        } else {
            Err(anyhow!("用户不存在"))
        }
    }
}

//登录凭证，如用户名、密码
pub trait AuthenticationToken {
    fn get_principal(&self) -> &(dyn Display + Send + Sync);
    fn get_credentials(&self) -> &(dyn Display + Send + Sync);
}

//登录的用户信息
pub trait UserDetails: erased_serde::Serialize {
    fn get_id(&self) -> &i64;
    fn get_username(&self) -> &String;
    fn get_password(&self) -> &String;
    fn get_authorities(&self) -> &Vec<String>;
    fn is_enable(&self) -> &bool;
    fn as_any(&self) -> Box<dyn Any + '_> {
        Box::new(self)
    }
}
serialize_trait_object!(UserDetails);

//登录认证结果
pub trait Authentication {
    fn get_authentication_token(&self) -> &(dyn AuthenticationToken);

    fn get_authorities(&self) -> &Vec<String>;
    fn set_authorities(&mut self, authorities: Vec<String>);

    fn is_authenticated(&self) -> &bool;
    fn set_authenticated(&mut self, authenticated: bool);

    fn set_details(&mut self, details: Box<dyn UserDetails + Send + Sync>);
    fn get_details(&self) -> &Box<(dyn UserDetails + Send + Sync)>;
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct DefaultAuthenticationToken {
    principal: String,
    credentials: String,
}

impl DefaultAuthenticationToken {
    pub fn new(principal: String, credentials: String) -> Self {
        DefaultAuthenticationToken {
            principal,
            credentials,
        }
    }
}

impl AuthenticationToken for DefaultAuthenticationToken {
    fn get_principal(&self) -> &(dyn Display + Send + Sync) {
        &self.principal
    }

    fn get_credentials(&self) -> &(dyn Display + Send + Sync) {
        &self.credentials
    }
}

#[derive(serde::Serialize)]
pub struct DefaultAuthentication {
    authentication_token: DefaultAuthenticationToken,
    authorities: Vec<String>,
    authenticated: bool,
    details: Box<dyn UserDetails + Send + Sync>,
}

impl DefaultAuthentication {
    pub fn new(
        authentication_token: DefaultAuthenticationToken,
        authorities: Vec<String>,
        authenticated: bool,
        details: Box<dyn UserDetails + Send + Sync>,
    ) -> Self {
        DefaultAuthentication {
            authentication_token,
            authorities,
            authenticated,
            details,
        }
    }
}

impl From<&(dyn Authentication + Send + Sync)> for DefaultAuthentication {
    fn from(value: &(dyn Authentication + Send + Sync)) -> Self {
        let principal = value.get_authentication_token().get_principal().to_string();
        let credentials = value
            .get_authentication_token()
            .get_credentials()
            .to_string();
        let mut authorities = vec![];
        for authority in value.get_authorities() {
            authorities.push(authority.to_string());
        }
        let mut user_authorities = vec![];
        for authority in value.get_details().get_authorities() {
            user_authorities.push(authority.to_string());
        }
        let mut user = DefaultUserDetails::new(
            value.get_details().get_id().clone(),
            value.get_details().get_username().to_string(),
            value.get_details().get_password().to_string(),
            user_authorities,
            value.get_details().is_enable().clone(),
        );
        DefaultAuthentication {
            authentication_token: DefaultAuthenticationToken {
                principal,
                credentials,
            },
            authorities,
            authenticated: value.is_authenticated().clone(),
            details: Box::new(user),
        }
    }
}
impl Authentication for DefaultAuthentication {
    fn get_authentication_token(&self) -> &(dyn AuthenticationToken) {
        &self.authentication_token
    }

    fn get_authorities(&self) -> &Vec<String> {
        &self.authorities
    }

    fn set_authorities(&mut self, authorities: Vec<String>) {
        self.authorities = authorities
    }

    fn is_authenticated(&self) -> &bool {
        &self.authenticated
    }

    fn set_authenticated(&mut self, is_authenticated: bool) {
        self.authenticated = is_authenticated
    }

    fn set_details(&mut self, details: Box<(dyn UserDetails + Send + Sync)>) {
        self.details = details;
    }

    fn get_details(&self) -> &Box<(dyn UserDetails + Send + Sync)> {
        &self.details
    }
}

impl Default for DefaultAuthentication {
    fn default() -> Self {
        DefaultAuthentication {
            authentication_token: DefaultAuthenticationToken {
                principal: ANONYMOUS.to_string(),
                credentials: "".to_string(),
            },
            authorities: vec![],
            authenticated: false,
            details: Box::new(DefaultUserDetails {
                id: 0,
                username: ANONYMOUS.to_string(),
                password: "".to_string(),
                authorities: vec![],
                enable: false,
            }),
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct DefaultUserDetails {
    id: i64,
    username: String,
    password: String,
    authorities: Vec<String>,
    enable: bool,
}

impl DefaultUserDetails {
    pub fn new(
        id: i64,
        username: String,
        password: String,
        authorities: Vec<String>,
        enable: bool,
    ) -> Self {
        DefaultUserDetails {
            id,
            username,
            password,
            authorities,
            enable,
        }
    }
    pub fn default() -> DefaultUserDetails {
        DefaultUserDetails {
            id: 0,
            username: "".to_string(),
            password: "".to_string(),
            authorities: vec![],
            enable: false,
        }
    }
}

impl UserDetails for DefaultUserDetails {
    fn get_id(&self) -> &i64 {
        &self.id
    }
    fn get_username(&self) -> &String {
        &self.username
    }

    fn get_password(&self) -> &String {
        &self.password
    }

    fn get_authorities(&self) -> &Vec<String> {
        &self.authorities
    }
    fn is_enable(&self) -> &bool {
        &self.enable
    }
}

pub struct UsernamePasswordAuthenticationToken {
    username: String,
    password: String,
}

impl UsernamePasswordAuthenticationToken {
    pub fn new(username: String, password: String) -> UsernamePasswordAuthenticationToken {
        UsernamePasswordAuthenticationToken { username, password }
    }
}

impl AuthenticationToken for UsernamePasswordAuthenticationToken {
    fn get_principal(&self) -> &(dyn Display + Send + Sync) {
        &self.username
    }

    fn get_credentials(&self) -> &(dyn Display + Send + Sync) {
        &self.password
    }
}

pub struct DefaultLoadUserService<'r, 'a, 'b> {
    sql_command_executor: &'r mut SqlCommandExecutor<'a, 'b>,
}

impl<'r, 'a, 'b> DefaultLoadUserService<'r, 'a, 'b> {
    pub fn new(
        sql_command_executor: &'r mut SqlCommandExecutor<'a, 'b>,
    ) -> Box<dyn LoadUserService + Send + Sync + 'r> {
        Box::new(DefaultLoadUserService {
            sql_command_executor,
        })
    }
}

#[async_trait::async_trait]
impl<'r, 'a, 'b> LoadUserService for DefaultLoadUserService<'r, 'a, 'b> {
    async fn load_user(
        &mut self,
        username: &String,
    ) -> anyhow::Result<Box<dyn UserDetails + Send + Sync>> {
        let mut args = MySqlArguments::default();
        args.add(username);
        let user: Option<User> = self
            .sql_command_executor
            .find_option_with("select * from `user` where username=?", args)
            .await?;
        if user.is_some() {
            let user = user.unwrap();
            let user_details = DefaultUserDetails {
                id: user.id,
                username: user.username.unwrap(),
                password: user.password.unwrap(),
                authorities: vec![],
                enable: user.enable == 1,
            };
            Ok(Box::new(user_details))
        } else {
            Err(anyhow!(format!("user '{}' not exists!", username)))
        }
    }
}

pub struct DefaultAuthenticationProvider<'r, 'a, 'b> {
    sql_command_executor: &'r mut SqlCommandExecutor<'a, 'b>,
}

impl<'r, 'a, 'b> DefaultAuthenticationProvider<'r, 'a, 'b> {
    pub fn new(
        sql_command_executor: &'r mut SqlCommandExecutor<'a, 'b>,
    ) -> Box<dyn AuthenticationProvider + Send + Sync + 'r> {
        Box::new(DefaultAuthenticationProvider {
            sql_command_executor,
        })
    }
}

pub struct UsernamePasswordAuthentication {
    authentication_token: UsernamePasswordAuthenticationToken,
    authorities: Vec<String>,
    authenticated: bool,
    details: Box<dyn UserDetails + Send + Sync>,
}

impl Authentication for UsernamePasswordAuthentication {
    fn get_authentication_token(&self) -> &(dyn AuthenticationToken) {
        &self.authentication_token
    }

    fn get_authorities(&self) -> &Vec<String> {
        &self.authorities
    }

    fn set_authorities(&mut self, authorities: Vec<String>) {
        self.authorities = authorities
    }

    fn is_authenticated(&self) -> &bool {
        &self.authenticated
    }

    fn set_authenticated(&mut self, authenticated: bool) {
        self.authenticated = authenticated;
    }

    fn set_details(&mut self, details: Box<(dyn UserDetails + Send + Sync)>) {
        self.details = details;
    }

    fn get_details(&self) -> &Box<(dyn UserDetails + Send + Sync)> {
        &self.details
    }
}

#[async_trait::async_trait]
impl<'r, 'a, 'b> AuthenticationProvider for DefaultAuthenticationProvider<'r, 'a, 'b> {
    async fn authenticate(
        &mut self,
        req: &mut RequestCtx,
        authentication_token: Box<dyn AuthenticationToken + Send + Sync>,
    ) -> anyhow::Result<Box<dyn Authentication + Send + Sync>> {
        let identify: String = authentication_token.get_principal().to_string();
        unsafe {
            let mut security_config: &mut WebSecurityConfigurer = APP_EXTENSIONS
                .get_mut::<WebSecurityConfigurer>()
                .unwrap()
                .borrow_mut();

            //let identify = identify.unwrap();

            let load_user_service_fn = security_config.get_load_user_service()(req);
            let mut load_user_service = load_user_service_fn(self.sql_command_executor);
            let details = load_user_service.load_user(&identify).await?;
            //手动释放load_user_service，不然无法第二次借用可变sql_command_executor
            drop(load_user_service);
            let user_details_checker: Box<dyn UserDetailsChecker + Send + Sync> =
                security_config.get_user_details_checker()(req);
            user_details_checker.check(&details).await?;
            drop(user_details_checker);
            self.additional_authentication_checks(req, &details, &authentication_token)
                .await?;
            let mut authorities = vec![];
            for authority in details.get_authorities() {
                authorities.push(authority.to_string());
            }
            let password: String = authentication_token.get_credentials().to_string();
            let authentication: UsernamePasswordAuthentication = UsernamePasswordAuthentication {
                authentication_token: UsernamePasswordAuthenticationToken {
                    username: identify.to_string(),
                    password: password.to_string(),
                },
                authorities,
                authenticated: true,
                details,
            };
            Ok(Box::new(authentication))
        }
    }

    async fn additional_authentication_checks(
        &mut self,
        req: &mut RequestCtx,
        user_details: &Box<dyn UserDetails + Send + Sync>,
        authentication_token: &Box<dyn AuthenticationToken + Send + Sync>,
    ) -> anyhow::Result<()> {
        //let details : Box<DefaultUserDetails> = user_details.downcast().unwrap();
        unsafe {
            let security_config: &mut WebSecurityConfigurer =
                APP_EXTENSIONS.get_mut::<WebSecurityConfigurer>().unwrap();
            let password: String = authentication_token.get_credentials().to_string();
            let matches = security_config
                .get_password_encoder()
                .matches(&password, user_details.get_password())?;
            return if matches {
                Ok(())
            } else {
                Err(anyhow!("无效密码"))
            };
        }
    }
}

pub trait PasswordEncoder {
    fn encode(&self, raw_password: &String) -> anyhow::Result<String>;
    fn matches(&self, raw_password: &String, encoded_password: &String) -> anyhow::Result<bool>;
}

pub struct BcryptPasswordEncoder {}

impl BcryptPasswordEncoder {
    pub fn new() -> Self {
        BcryptPasswordEncoder {}
    }
}

impl PasswordEncoder for BcryptPasswordEncoder {
    fn encode(&self, raw_password: &String) -> anyhow::Result<String> {
        let hashed = hash(raw_password, DEFAULT_COST)?;
        return Ok(hashed);
    }

    fn matches(&self, raw_password: &String, encoded_password: &String) -> anyhow::Result<bool> {
        let valid = verify(raw_password, encoded_password);
        return if valid.is_ok() {
            if valid.unwrap() {
                Ok(true)
            } else {
                Err(anyhow!("密码错误"))
            }
        } else {
            println!("{:?}", valid);
            Err(anyhow!("密码格式无效"))
        };
    }
}

pub struct NopPasswordEncoder;

impl PasswordEncoder for NopPasswordEncoder {
    fn encode(&self, raw_password: &String) -> anyhow::Result<String> {
        Ok(raw_password.to_string())
    }

    fn matches(&self, _: &String, _: &String) -> anyhow::Result<bool> {
        Ok(true)
    }
}
pub struct Sha512PasswordEncoder;

impl PasswordEncoder for Sha512PasswordEncoder {
    fn encode(&self, raw_password: &String) -> anyhow::Result<String> {
        let mut buf = [0u8; 64];
        hash_password::<Sha512>(
            raw_password.as_ref(),
            SECURITY_CONFIG.password_salt.as_ref(),
            &mut buf,
        );
        let base64_hash = Base64::encode_string(&buf);
        Ok(base64_hash)
    }

    fn matches(&self, raw_password: &String, encoded_password: &String) -> anyhow::Result<bool> {
        let encode = self.encode(raw_password)?;
        Ok(encode == encoded_password.to_string())
    }
}

#[async_trait::async_trait]
pub trait UserDetailsChecker {
    async fn check(&self, details: &Box<(dyn UserDetails + Send + Sync)>) -> anyhow::Result<()>;
}

pub struct DefaultUserDetailsChecker {}

impl DefaultUserDetailsChecker {
    pub fn new() -> Self {
        DefaultUserDetailsChecker {}
    }
}

#[async_trait::async_trait]
impl UserDetailsChecker for DefaultUserDetailsChecker {
    async fn check(&self, details: &Box<(dyn UserDetails + Send + Sync)>) -> anyhow::Result<()> {
        if !details.is_enable() {
            Err(anyhow!(format!(
                "账号{}已被禁止登陆",
                details.get_username()
            )))
        } else {
            Ok(())
        }
    }
}

#[async_trait::async_trait]
pub trait AuthenticationProvider {
    async fn authenticate(
        &mut self,
        req: &mut RequestCtx,
        authentication_token: Box<dyn AuthenticationToken + Send + Sync>,
    ) -> anyhow::Result<Box<dyn Authentication + Send + Sync>>;
    async fn additional_authentication_checks(
        &mut self,
        req: &mut RequestCtx,
        user_details: &Box<dyn UserDetails + Send + Sync>,
        authentication_token: &Box<dyn AuthenticationToken + Send + Sync>,
    ) -> anyhow::Result<()>;
}

#[derive(Error, Debug)]
pub enum SecurityError {
    #[error("访问被拒绝")]
    AccessDeniedException,
}
#[async_trait::async_trait]
pub trait AccessDecisionManager {
    async fn decide(
        &self,
        authentication: &(dyn Authentication + Send + Sync),
        object: &(dyn Any + Send + Sync),
        config_attributes: &Vec<Box<dyn ConfigAttribute + Send + Sync>>,
    ) -> anyhow::Result<(), SecurityError>;
    async fn check_allow_if_all_abstain_decisions(&self) -> anyhow::Result<(), SecurityError> {
        if SECURITY_CONFIG.allow_if_all_abstain_decisions.is_some()
            && !SECURITY_CONFIG.allow_if_all_abstain_decisions.unwrap()
        {
            Err(SecurityError::AccessDeniedException)
        } else {
            Ok(())
        }
    }
}
pub struct AffirmativeBased {
    decision_voters: Vec<Box<dyn AccessDecisionVoter + Send + Sync>>,
}
#[async_trait::async_trait]
impl AccessDecisionManager for AffirmativeBased {
    async fn decide(
        &self,
        authentication: &(dyn Authentication + Send + Sync),
        object: &(dyn Any + Send + Sync),
        config_attributes: &Vec<Box<dyn ConfigAttribute + Send + Sync>>,
    ) -> anyhow::Result<(), SecurityError> {
        let mut deny = 0;
        let decision_voters = self.decision_voters.iter();
        for decision_voter in decision_voters {
            let result = decision_voter.vote(authentication, object, config_attributes);
            match result {
                Vote::AccessDenied => {
                    deny = deny + 1;
                }
                Vote::AccessAbstain => {}
                Vote::AccessGranted => {
                    return Ok(());
                }
            }
        }
        if deny > 0 {
            Err(SecurityError::AccessDeniedException)
        } else {
            self.check_allow_if_all_abstain_decisions().await
        }
    }
}
//角色
pub trait ConfigAttribute: erased_serde::Serialize {
    fn get_attribute(&self) -> &String;
}
serialize_trait_object!(ConfigAttribute);
#[derive(serde::Serialize)]
pub struct DefaultConfigAttribute {
    pub attribute: String,
}

impl ConfigAttribute for DefaultConfigAttribute {
    fn get_attribute(&self) -> &String {
        &self.attribute
    }
}
#[derive(serde::Serialize)]
pub struct SecurityMetadataSource {
    //请求的接口url到角色的映射关系
    request_map: HashMap<String, Vec<Box<dyn ConfigAttribute + Send + Sync>>>,
}

impl SecurityMetadataSource {
    pub fn request_map(&self) -> &HashMap<String, Vec<Box<dyn ConfigAttribute + Send + Sync>>> {
        &self.request_map
    }
}
impl<'de> serde::Deserialize<'de> for SecurityMetadataSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let map = serde_json::Map::deserialize(deserializer)?;
        let mut req_map: HashMap<String, Vec<Box<dyn ConfigAttribute + Send + Sync>>> =
            HashMap::new();

        let req_map_field = map.get("request_map").unwrap();
        for item in req_map_field.as_object().unwrap() {
            let mut roles: Vec<Box<dyn ConfigAttribute + Send + Sync>> = vec![];
            let attrs = item.1.as_array().unwrap();
            for attr in attrs {
                roles.push(Box::new(DefaultConfigAttribute {
                    attribute: attr.get("attribute").unwrap().as_str().unwrap().to_string(),
                }));
            }
            req_map.insert(item.0.to_string(), roles);
        }
        let mut metadata = SecurityMetadataSource {
            request_map: req_map,
        };
        Ok(metadata)
    }
}
#[async_trait::async_trait]
pub trait SecurityMetadataSourceProvider {
    async fn security_metadata_source(&self) -> SecurityMetadataSource;
}

pub struct DefaultSecurityMetadataSourceProvider;

#[async_trait::async_trait]
impl SecurityMetadataSourceProvider for DefaultSecurityMetadataSourceProvider {
    async fn security_metadata_source(&self) -> SecurityMetadataSource {
        unsafe {
            let metadata_source: RedisResult<SecurityMetadataSource> = crate::redis::get(
                SECURITY_CONFIG
                    .security_metadata_source_storage_key
                    .clone()
                    .as_str(),
            )
            .await;
            if metadata_source.is_ok() {
                return metadata_source.unwrap();
            } else {
                let pool_state: Option<&State<Pool<MySql>>> = APP_EXTENSIONS.get();
                let pool = pool_state.unwrap().get_ref();
                let mut sql_command_executor = SqlCommandExecutor::WithoutTransaction(pool);
                let permission: anyhow::Result<Vec<AdminPermission>> = sql_command_executor
                    .find_all("select * from admin_permission")
                    .await;
                if permission.is_ok() {
                    let permissions = permission.unwrap();
                    let mut req_map: HashMap<String, Vec<Box<dyn ConfigAttribute + Send + Sync>>> =
                        HashMap::new();
                    for permission in permissions {
                        let mut args = MySqlArguments::default();
                        args.add(permission.url.clone());
                        let roles: anyhow::Result<Vec<AdminRole>> = sql_command_executor
                            .find_all_with(
                                r#"SELECT
                                            r.*
                                        FROM
                                            admin_role r
                                            JOIN admin_role_permission rp ON r.id = rp.admin_role_id
                                            JOIN admin_permission p ON rp.admin_permission_id = p.id
                                        where p.url = ?"#,
                                args,
                            )
                            .await;
                        if roles.is_ok() {
                            let roles = roles.unwrap();
                            let mut config_attrs: Vec<Box<dyn ConfigAttribute + Send + Sync>> =
                                vec![];
                            for role in roles {
                                config_attrs.push(Box::new(DefaultConfigAttribute {
                                    attribute: role.code,
                                }));
                            }
                            req_map.insert(permission.url.clone(), config_attrs);
                        }
                    }
                    let metadata_source = SecurityMetadataSource {
                        request_map: req_map,
                    };
                    let storage_result = crate::redis::set(
                        SECURITY_CONFIG
                            .security_metadata_source_storage_key
                            .clone()
                            .as_str(),
                        &metadata_source,
                    )
                    .await;
                    if storage_result.is_err() {
                        error!(
                            "security_metadata_source 存储失败：{}",
                            storage_result.err().unwrap()
                        );
                    }
                    metadata_source
                } else {
                    error!("查询异常admin_permission：{}", permission.err().unwrap());
                    SecurityMetadataSource {
                        request_map: HashMap::new(),
                    }
                }
            }
        }
    }
}

#[derive(PartialEq)]
pub enum Vote {
    AccessGranted,
    AccessDenied,
    AccessAbstain,
}

pub trait AccessDecisionVoter {
    fn supports(&self, attribute: &Box<dyn ConfigAttribute + Send + Sync>) -> bool;
    fn vote(
        &self,
        authentication: &(dyn Authentication + Send + Sync),
        object: &(dyn Any + Send + Sync),
        attributes: &Vec<Box<dyn ConfigAttribute + Send + Sync>>,
    ) -> Vote;
}
pub struct RoleVoter;

impl AccessDecisionVoter for RoleVoter {
    fn supports(&self, attribute: &Box<dyn ConfigAttribute + Send + Sync>) -> bool {
        true
    }
    fn vote(
        &self,
        authentication: &(dyn Authentication + Send + Sync),
        object: &(dyn Any + Send + Sync),
        attributes: &Vec<Box<dyn ConfigAttribute + Send + Sync>>,
    ) -> Vote {
        if !authentication.is_authenticated() {
            return Vote::AccessDenied;
        }
        let mut vote = Vote::AccessAbstain;
        //当前用户拥有的角色
        let authorities = authentication.get_authorities();
        for authority in authorities {
            println!("当前用户拥有的角色{}", authority);
        }
        //访问当前资源所需要的角色
        for attribute in attributes {
            if self.supports(attribute) {
                vote = Vote::AccessDenied;
                for authority in authorities {
                    println!(
                        "attribute.get_attribute().to_string()={}",
                        attribute.get_attribute().to_string()
                    );
                    println!("authority.to_string()={}", authority.to_string());
                    if attribute.get_attribute().to_string() == authority.to_string() {
                        vote = Vote::AccessGranted;
                        break;
                    }
                }
                if vote == Vote::AccessDenied {
                    return Vote::AccessDenied;
                }
            }
        }
        return vote;
    }
}
pub(crate) fn req_matches(req: &RequestCtx, url_patterns: &str) -> bool {
    let init = UrlPatternInit {
        pathname: Some(url_patterns.to_string()),
        ..Default::default()
    };
    let pattern = <UrlPattern>::parse(init);
    if pattern.is_ok() {
        let pattern = pattern.unwrap();
        // Match the pattern against a URL.
        println!("req uri = {}", req.uri.to_string());
        let url = ("http://127.0.0.1".to_string() + &*req.uri.to_string()).parse::<Url>();
        if url.is_ok() {
            let url = url.unwrap();
            let result = pattern.exec(UrlPatternMatchInput::Url(url));
            if result.is_ok() {
                let result = result.unwrap();
                if result.is_some() {
                    return true;
                }
            }
        }
    }
    return false;
}
pub struct SecurityInterceptor;
#[async_trait::async_trait]
impl Filter for SecurityInterceptor {
    async fn handle<'a>(&'a self, mut ctx: RequestCtx, next: Next<'a>) -> anyhow::Result<Response> {
        unsafe {
            let security_config: &mut WebSecurityConfigurer =
                APP_EXTENSIONS.get_mut::<WebSecurityConfigurer>().unwrap();
            let access_decision_manager = security_config.get_access_decision_manager().as_ref();
            let security_metadata_source_provider = security_config
                .get_security_metadata_source_provider()
                .as_ref();
            let metadata = security_metadata_source_provider
                .security_metadata_source()
                .await;
            let req_map = metadata.request_map();
            let uri_req_map = req_map.get(ctx.uri.path());
            if uri_req_map.is_some() {
                println!("uri = {}", ctx.uri.path());
                let uri_req_map = uri_req_map.unwrap();
                println!("{}", "找到uri_req_map");
                access_decision_manager
                    .decide(ctx.authentication.as_ref(), &ctx, uri_req_map)
                    .await?;
                return next.run(ctx).await;
            } else {
                for (url, attrs) in req_map {
                    if req_matches(&ctx, url) {
                        println!("uri = {}", ctx.uri.path());
                        println!("{}", "找到uri_req_map");
                        access_decision_manager
                            .decide(ctx.authentication.as_ref(), &ctx, attrs)
                            .await?;
                        return next.run(ctx).await;
                    }
                }
            }
            info!("访问`{}`被拒绝：{}", ctx.uri.to_string(), "没有权限");
            return Err(anyhow!("没有权限"));
        }
    }

    fn url_patterns(&self) -> &'static str {
        if SECURITY_CONFIG.intercept_url_patterns.is_none() {
            panic!("intercept_url_patterns尚未配置");
        } else {
            SECURITY_CONFIG
                .intercept_url_patterns
                .as_ref()
                .unwrap()
                .as_str()
        }
    }

    fn order(&self) -> u64 {
        todo!()
    }

    fn name(&self) -> &'static str {
        "SecurityInterceptor"
    }
}

pub struct AuthenticationProcessingFilter;

#[async_trait::async_trait]
impl Filter for AuthenticationProcessingFilter {
    async fn handle<'a>(&'a self, mut ctx: RequestCtx, next: Next<'a>) -> anyhow::Result<Response> {
        unsafe {
            let pool_state: Option<&State<Pool<MySql>>> = APP_EXTENSIONS.get();
            let pool = pool_state.unwrap().get_ref();
            let tran = pool.begin().await?;
            let mut tran_manager = TransactionManager::new(tran);
            let mut sql_command_executor = SqlCommandExecutor::WithTransaction(&mut tran_manager);

            let security_config: &mut WebSecurityConfigurer =
                APP_EXTENSIONS.get_mut::<WebSecurityConfigurer>().unwrap();

            let authentication_token_resolver =
                security_config.get_authentication_token_resolver()();
            let authentication_token = authentication_token_resolver.resolve(&mut ctx).await?;
            drop(authentication_token_resolver);

            let start = Local::now().timestamp_millis();
            let mut auth_provider =
                security_config.get_authentication_provider()(&mut ctx)(&mut sql_command_executor);
            let authentication = auth_provider
                .authenticate(&mut ctx, authentication_token)
                .await;
            let end = Local::now().timestamp_millis();
            println!("查询用户耗时{}", end - start);
            //手动释放load_user_service，不然无法第二次借用可变sql_command_executor
            drop(auth_provider);

            if authentication.is_ok() {
                let authentication = authentication.unwrap();

                let success_handler = security_config.get_authentication_success_handler();
                if success_handler.is_some() {
                    let success_handler = success_handler.as_ref().unwrap();
                    let mut result = success_handler(&mut ctx).handle(&authentication).await?;
                    ctx.authentication = authentication;
                    let auth = DefaultAuthentication::from(ctx.authentication.as_ref());
                    SecurityContextHolder::set_context(
                        &ctx,
                        DefaultSecurityContext {
                            authentication: auth,
                        },
                    )
                    .await;
                    Ok(result)
                } else {
                    let user_details: &Box<dyn UserDetails + Send + Sync> =
                        authentication.get_details();
                    let mut jwt_service =
                        security_config.get_jwt_service()(&mut ctx)(&mut sql_command_executor);
                    let start = Local::now().timestamp_millis();
                    let access_token = jwt_service
                        .grant_access_token(*user_details.get_id())
                        .await?;
                    let end = Local::now().timestamp_millis();
                    println!("生成token耗时{}", end - start);
                    drop(jwt_service);
                    drop(sql_command_executor);
                    let endpoint_result: EndpointResult<AccessToken> =
                        EndpointResult::ok_with_payload("", access_token);
                    let mut result = ResponseBuilder::with_endpoint_result(endpoint_result);
                    ctx.authentication = authentication;
                    let auth = DefaultAuthentication::from(ctx.authentication.as_ref());
                    SecurityContextHolder::set_context(
                        &ctx,
                        DefaultSecurityContext {
                            authentication: auth,
                        },
                    )
                    .await;
                    tran_manager.commit().await?;

                    Ok(result)
                }
            } else {
                let fail_handler = security_config.get_authentication_failure_handler();
                if fail_handler.is_some() {
                    let result = fail_handler.as_ref().unwrap()(&mut ctx)
                        .handle(authentication.err().unwrap())
                        .await?;
                    Ok(result)
                } else {
                    let endpoint_result: EndpointResult<AccessToken> =
                        EndpointResult::unauthorized("登录凭证无效");
                    Ok(ResponseBuilder::with_endpoint_result(endpoint_result))
                }
            }
        }
    }

    fn url_patterns(&self) -> &'static str {
        "/login"
    }

    fn order(&self) -> u64 {
        todo!()
    }

    fn name(&self) -> &'static str {
        "AuthenticationProcessingFilter"
    }
}

pub struct UsernamePasswordAuthenticationFilter {}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct UsernamePassword {
    pub username: String,
    pub password: String,
}

#[async_trait::async_trait]
pub trait AuthenticationSuccessHandler {
    async fn handle(
        &self,
        authentication: &Box<dyn Authentication + Send + Sync>,
    ) -> anyhow::Result<Response>;
}

#[async_trait::async_trait]
pub trait AuthenticationFailureHandler {
    async fn handle(&self, error: anyhow::Error) -> anyhow::Result<Response>;
}

pub type AuthenticationTokenResolverFn =
    Box<dyn Fn() -> Box<dyn AuthenticationTokenResolver + Send + Sync> + Send + Sync>;
pub type JwtServiceFn = Box<
    dyn for<'a> Fn(
            &'a mut RequestCtx,
        ) -> Box<
            dyn for<'r, 'c, 'd> Fn(
                    &'r mut SqlCommandExecutor<'c, 'd>,
                ) -> Box<dyn JwtService + Send + Sync + 'r>
                + Send
                + Sync,
        > + Send
        + Sync,
>;
pub type AuthenticationSuccessHandlerFn = Box<
    dyn for<'a> Fn(&'a mut RequestCtx) -> Box<dyn AuthenticationSuccessHandler + Send + Sync + 'a>
        + Send
        + Sync,
>;
pub type AuthenticationFailureHandlerFn = Box<
    dyn for<'a> Fn(&'a mut RequestCtx) -> Box<dyn AuthenticationFailureHandler + Send + Sync + 'a>
        + Send
        + Sync,
>;
pub type LoadUserServiceFn = Box<
    dyn for<'a> Fn(
            &'a mut RequestCtx,
        ) -> Box<
            dyn for<'r, 'c, 'd> Fn(
                    &'r mut SqlCommandExecutor<'c, 'd>,
                ) -> Box<dyn LoadUserService + Send + Sync + 'r>
                + Send
                + Sync,
        > + Send
        + Sync,
>;
pub type AuthenticationProviderFn = Box<
    dyn for<'a> Fn(
            &'a mut RequestCtx,
        ) -> Box<
            dyn for<'r, 'c, 'd> Fn(
                    &'r mut SqlCommandExecutor<'c, 'd>,
                )
                    -> Box<dyn AuthenticationProvider + Send + Sync + 'r>
                + Send
                + Sync,
        > + Send
        + Sync,
>;
pub type UserDetailsCheckerFn = Box<
    dyn for<'a> Fn(&'a mut RequestCtx) -> Box<dyn UserDetailsChecker + Send + Sync + 'a>
        + Send
        + Sync,
>;

pub struct WebSecurityConfigurer {
    enable_security: bool,
    authentication_token_resolver: AuthenticationTokenResolverFn,
    jwt_service: JwtServiceFn,
    authentication_success_handler: Option<AuthenticationSuccessHandlerFn>,
    authentication_failure_handler: Option<AuthenticationFailureHandlerFn>,
    load_user_service: LoadUserServiceFn,
    authentication_provider: AuthenticationProviderFn,
    user_details_checker: UserDetailsCheckerFn,
    password_encoder: Box<dyn PasswordEncoder + Send + Sync>,
    security_metadata_source_provider: Box<dyn SecurityMetadataSourceProvider + Send + Sync>,
    //access_decision_voter: Option<Box<dyn AccessDecisionVoter + Send + Sync>>,
    access_decision_manager: Box<dyn AccessDecisionManager + Send + Sync>,
}

fn load_user_service_fn<'r, 'a, 'b>(
    sql_command_executor: &'r mut SqlCommandExecutor<'a, 'b>,
) -> Box<dyn LoadUserService + Send + Sync + 'r> {
    DefaultLoadUserService::new(sql_command_executor)
}

fn jwt_service_fn<'r, 'a, 'b>(
    sql_command_executor: &'r mut SqlCommandExecutor<'a, 'b>,
) -> Box<dyn JwtService + Send + Sync + 'r> {
    DefaultJwtService::new(sql_command_executor)
}

fn authentication_provider_fn<'r, 'a, 'b>(
    sql_command_executor: &'r mut SqlCommandExecutor<'a, 'b>,
) -> Box<dyn AuthenticationProvider + Send + Sync + 'r> {
    DefaultAuthenticationProvider::new(sql_command_executor)
}

impl WebSecurityConfigurer {
    pub fn new() -> Self {
        WebSecurityConfigurer {
            enable_security: false,
            authentication_token_resolver: AuthenticationTokenResolverFn::from(Box::new(
                || -> Box<dyn AuthenticationTokenResolver + Send + Sync> {
                    Box::new(UsernamePasswordAuthenticationTokenResolver::new())
                },
            )),
            jwt_service: JwtServiceFn::from(Box::new(
                |req: &mut RequestCtx| -> Box<
                    dyn for<'r, 'c, 'd> Fn(
                            &'r mut SqlCommandExecutor<'c, 'd>,
                        )
                            -> Box<(dyn JwtService + Send + Sync + 'r)>
                        + Send
                        + Sync,
                > { Box::new(jwt_service_fn) },
            )),
            authentication_success_handler: None,
            authentication_failure_handler: None,
            load_user_service: LoadUserServiceFn::from(Box::new(
                |req: &mut RequestCtx| -> Box<
                    dyn for<'r, 'c, 'd> Fn(
                            &'r mut SqlCommandExecutor<'c, 'd>,
                        )
                            -> Box<(dyn LoadUserService + Send + Sync + 'r)>
                        + Send
                        + Sync,
                > { Box::new(load_user_service_fn) },
            )),
            authentication_provider: AuthenticationProviderFn::from(Box::new(
                |re: &mut RequestCtx| -> Box<
                    dyn for<'r, 'c, 'd> Fn(
                            &'r mut SqlCommandExecutor<'c, 'd>,
                        )
                            -> Box<(dyn AuthenticationProvider + Send + Sync + 'r)>
                        + Send
                        + Sync,
                > { Box::new(authentication_provider_fn) },
            )),
            user_details_checker: UserDetailsCheckerFn::from(Box::new(
                |req: &mut RequestCtx| -> Box<dyn UserDetailsChecker + Send + Sync> {
                    Box::new(DefaultUserDetailsChecker::new())
                },
            )),
            password_encoder: Box::new(BcryptPasswordEncoder::new()),
            security_metadata_source_provider: Box::new(DefaultSecurityMetadataSourceProvider),
            //access_decision_voter: Some(Box::new(RoleVoter)),
            access_decision_manager: Box::new(AffirmativeBased {
                decision_voters: vec![Box::new(RoleVoter)],
            }),
        }
    }
    pub fn enable_security(&mut self, enable_security: bool) -> &Self {
        self.enable_security = enable_security;
        self
    }
    pub fn authentication_token_resolver(
        &mut self,
        authentication_token_resolver: AuthenticationTokenResolverFn,
    ) -> &Self {
        self.authentication_token_resolver = authentication_token_resolver;
        self
    }
    pub fn jwt_service(&mut self, jwt_service: JwtServiceFn) -> &Self {
        self.jwt_service = jwt_service;
        self
    }
    pub fn authentication_success_handler(
        &mut self,
        authentication_success_handler: AuthenticationSuccessHandlerFn,
    ) -> &Self {
        self.authentication_success_handler = Some(authentication_success_handler);
        self
    }
    pub fn authentication_failure_handler(
        &mut self,
        authentication_failure_handler: AuthenticationFailureHandlerFn,
    ) -> &Self {
        self.authentication_failure_handler = Some(authentication_failure_handler);
        self
    }
    pub fn load_user_service(&mut self, load_user_service: LoadUserServiceFn) -> &Self {
        self.load_user_service = load_user_service;
        self
    }
    pub fn authentication_provider(
        &mut self,
        authentication_provider: AuthenticationProviderFn,
    ) -> &Self {
        self.authentication_provider = authentication_provider;
        self
    }
    pub fn user_details_checker(&mut self, user_details_checker: UserDetailsCheckerFn) -> &Self {
        self.user_details_checker = user_details_checker;
        self
    }
    pub fn password_encoder(
        &mut self,
        password_encoder: Box<dyn PasswordEncoder + Send + Sync>,
    ) -> &Self {
        self.password_encoder = password_encoder;
        self
    }

    pub fn is_enable_security(&self) -> bool {
        self.enable_security
    }
    pub fn get_authentication_token_resolver(&self) -> &AuthenticationTokenResolverFn {
        &self.authentication_token_resolver
    }
    pub fn get_jwt_service(&self) -> &JwtServiceFn {
        &self.jwt_service
    }
    pub fn get_authentication_success_handler(&self) -> &Option<AuthenticationSuccessHandlerFn> {
        &self.authentication_success_handler
    }
    pub fn get_authentication_failure_handler(&self) -> &Option<AuthenticationFailureHandlerFn> {
        &self.authentication_failure_handler
    }
    pub fn get_load_user_service(&mut self) -> &LoadUserServiceFn {
        &self.load_user_service
    }
    pub fn get_authentication_provider(&self) -> &AuthenticationProviderFn {
        &self.authentication_provider
    }
    pub fn get_user_details_checker(&self) -> &UserDetailsCheckerFn {
        &self.user_details_checker
    }
    pub fn get_password_encoder(&self) -> &Box<dyn PasswordEncoder + Send + Sync> {
        &self.password_encoder
    }
    //security_metadata_source_provider   access_decision_voter   access_decision_manager
    pub fn security_metadata_source_provider(
        &mut self,
        security_metadata_source_provider: Box<dyn SecurityMetadataSourceProvider + Send + Sync>,
    ) {
        self.security_metadata_source_provider = security_metadata_source_provider;
    }
    /*    pub fn access_decision_voter(
        &mut self,
        access_decision_voter: Box<dyn AccessDecisionVoter + Send + Sync>,
    ) {
        self.access_decision_voter = Some(access_decision_voter);
    }*/
    pub fn access_decision_manager(
        &mut self,
        access_decision_manager: Box<dyn AccessDecisionManager + Send + Sync>,
    ) {
        self.access_decision_manager = access_decision_manager;
    }
    pub fn get_security_metadata_source_provider(
        &self,
    ) -> &Box<dyn SecurityMetadataSourceProvider + Send + Sync> {
        &self.security_metadata_source_provider
    }
    /*   pub fn get_access_decision_voter(&self) -> &Option<Box<dyn AccessDecisionVoter + Send + Sync>> {
        &self.access_decision_voter
    }*/
    pub fn get_access_decision_manager(&self) -> &Box<dyn AccessDecisionManager + Send + Sync> {
        &self.access_decision_manager
    }
}
pub trait SecurityContext<T>
where
    T: Authentication + Send + Sync,
{
    fn get_authentication(&self) -> &T;
}
#[derive(serde::Serialize)]
pub struct DefaultSecurityContext {
    authentication: DefaultAuthentication,
}
impl DefaultSecurityContext {
    pub fn new(authentication: DefaultAuthentication) -> Self {
        DefaultSecurityContext { authentication }
    }
}

impl Default for DefaultSecurityContext {
    fn default() -> Self {
        DefaultSecurityContext {
            authentication: Default::default(),
        }
    }
}

impl<'de> serde::Deserialize<'de> for DefaultSecurityContext {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let map = serde_json::Map::deserialize(deserializer)?;

        let authentication_field = map.get("authentication").unwrap();
        let details_field = authentication_field.get("details").unwrap();
        let authentication_token_field = authentication_field.get("authentication_token").unwrap();
        let authorities_field = authentication_field
            .get("authorities")
            .unwrap()
            .as_array()
            .unwrap();

        let mut token = DefaultAuthenticationToken {
            principal: "".to_string(),
            credentials: "".to_string(),
        };
        token.principal = authentication_token_field
            .get("principal")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        token.credentials = authentication_token_field
            .get("credentials")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();

        let mut user = DefaultUserDetails::default();
        user.id = details_field.get("id").unwrap().as_i64().unwrap();
        let mut authorities: Vec<String> = vec![];
        for item in details_field
            .get("authorities")
            .unwrap()
            .as_array()
            .unwrap()
        {
            authorities.push(item.as_str().unwrap().to_string());
        }
        user.authorities = authorities;
        user.enable = details_field.get("enable").unwrap().as_bool().unwrap();
        user.password = details_field
            .get("password")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        user.username = details_field
            .get("username")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();

        let mut authorities: Vec<String> = vec![];
        for item in authorities_field {
            authorities.push(item.as_str().unwrap().to_string());
        }

        let mut authentication = DefaultAuthentication {
            authentication_token: token,
            authorities,
            authenticated: authentication_field
                .get("authenticated")
                .unwrap()
                .as_bool()
                .unwrap(),
            details: Box::new(user),
        };
        let context = DefaultSecurityContext { authentication };
        Ok(context)
    }
}

impl SecurityContext<DefaultAuthentication> for DefaultSecurityContext {
    fn get_authentication(&self) -> &DefaultAuthentication {
        &self.authentication
    }
}
#[async_trait::async_trait]
pub trait SecurityContextHolderStrategy<T, A>
where
    T: SecurityContext<A> + Send + Sync,
    A: Authentication + Send + Sync,
{
    async fn get_context(&self, user_request_identity: String) -> T;
    async fn set_context(&mut self, user_request_identity: String, security_context: T);
    fn create_empty_context(&self) -> T;
}
pub struct LocalCacheSecurityContextHolderStrategy;

#[async_trait::async_trait]
impl<T, A> SecurityContextHolderStrategy<T, A> for LocalCacheSecurityContextHolderStrategy
where
    for<'a> T:
        SecurityContext<A> + Send + Sync + serde::Deserialize<'a> + serde::Serialize + Default + 'a,
    A: Authentication + Send + Sync,
{
    async fn get_context(&self, session_id: String) -> T {
        let key = format!("security_context:{0}", session_id);
        let result: RedisResult<T> = crate::redis::get(&*key).await;
        if result.is_ok() {
            result.unwrap()
        } else {
            self.create_empty_context()
        }
    }

    async fn set_context(&mut self, session_id: String, security_context: T) {
        let key = format!("security_context:{0}", session_id);
        let result = crate::redis::set(&*key, &security_context).await;
        if result.is_ok() {
            info!("设置security_context缓存`{}`成功", session_id);
        } else {
            error!(
                "设置security_context缓存`{}`异常：{}",
                session_id,
                result.err().unwrap()
            );
        }
    }

    fn create_empty_context(&self) -> T {
        T::default()
    }
}
pub const DEFAULT_STRATEGY_NAME: &'static str = "redis";
pub const SESSION_ID: &'static str = "session_id";

pub struct SecurityContextHolder;

impl SecurityContextHolder {
    pub async fn get_context<T, A>(req: &RequestCtx) -> T
    where
        for<'a> T: SecurityContext<A>
            + Send
            + Sync
            + Default
            + serde::Serialize
            + serde::Deserialize<'a>
            + 'a,
        A: Authentication + Send + Sync,
    {
        let session_id = SecurityContextHolder::session_id(req).await;
        if SECURITY_CONFIG.security_context_storage_strategy.is_none() {
            SecurityContextHolder::default_security_context(session_id).await
        } else {
            let security_context_storage_strategy = SECURITY_CONFIG
                .security_context_storage_strategy
                .as_ref()
                .unwrap()
                .clone();
            if security_context_storage_strategy == DEFAULT_STRATEGY_NAME.to_string() {
                SecurityContextHolder::default_security_context(session_id).await
            } else {
                panic!(
                    "security_context_storage_strategy `{}`尚未实现",
                    security_context_storage_strategy
                );
            }
        }
    }
    pub async fn default_security_context<T, A>(session_id: String) -> T
    where
        for<'a> T: SecurityContext<A>
            + Send
            + Sync
            + Default
            + serde::Serialize
            + serde::Deserialize<'a>
            + 'a,
        A: Authentication + Send + Sync,
    {
        let security_context_holder_strategy = LocalCacheSecurityContextHolderStrategy;
        security_context_holder_strategy
            .get_context(session_id)
            .await
    }
    pub async fn set_context<T, A>(req: &RequestCtx, security_context: T)
    where
        for<'a> T: SecurityContext<A>
            + Send
            + Sync
            + Default
            + serde::Serialize
            + serde::Deserialize<'a>
            + 'a,
        A: Authentication + Send + Sync,
    {
        let session_id = SecurityContextHolder::session_id(req).await;
        let mut security_context_holder_strategy = LocalCacheSecurityContextHolderStrategy;
        security_context_holder_strategy
            .set_context(session_id, security_context)
            .await
    }
    async fn session_id(req: &RequestCtx) -> String {
        return if SECURITY_CONFIG.cookie_based {
            let session_id = req.headers.get(SESSION_ID);
            if session_id.is_some() {
                let session_id = session_id.unwrap();
                let session_id = session_id.to_str();
                if session_id.is_ok() {
                    let session_id = session_id.unwrap();
                    if !session_id.is_empty() {
                        return session_id.to_string();
                    }
                }
            }
            ANONYMOUS.to_string()
        } else {
            if *req.authentication.is_authenticated() {
                req.authentication.get_details().get_id().to_string()
            } else {
                let access_token = req.headers.get("access_token");
                return if access_token.is_some() {
                    let access_token = access_token.unwrap();
                    let access_token = access_token.to_str();
                    if access_token.is_ok() {
                        let access_token = access_token.unwrap();
                        let claims = decode_access_token(access_token.to_string()).await;
                        if claims.is_ok() {
                            let claims = claims.unwrap();
                            return claims.user_id.to_string();
                        }
                    }
                    ANONYMOUS.to_string()
                } else {
                    ANONYMOUS.to_string()
                };
            }
        };
    }
}
const ANONYMOUS: &'static str = "anonymous";

pub struct AuthenticationFilter;
#[async_trait::async_trait]
impl Filter for AuthenticationFilter {
    async fn handle<'a>(&'a self, mut ctx: RequestCtx, next: Next<'a>) -> anyhow::Result<Response> {
        let context: DefaultSecurityContext = SecurityContextHolder::get_context(&ctx).await;
        ctx.authentication = Box::new(context.authentication);
        next.run(ctx).await
    }

    fn url_patterns(&self) -> &'static str {
        "/*"
    }

    fn order(&self) -> u64 {
        todo!()
    }

    fn name(&self) -> &'static str {
        "AuthenticationFilter"
    }
}

// Toy example, do not use it in practice!
// Instead use crates from: https://github.com/RustCrypto/password-hashing
fn hash_password<D: Digest>(password: &str, salt: &str, output: &mut [u8]) {
    let mut hasher = D::new();
    hasher.update(password.as_bytes());
    hasher.update(b"$");
    hasher.update(salt.as_bytes());
    output.copy_from_slice(&hasher.finalize())
}

#[test]
fn test() {
    let start = Local::now().timestamp_millis();
    let mut buf1 = [0u8; 64];
    hash_password::<Sha512>("123456", "123456", &mut buf1);

    let base64_hash = Base64::encode_string(&buf1);
    println!("Base64-encoded hash: {}", base64_hash);

    let end = Local::now().timestamp_millis();
    println!("耗时：{}", end - start);

    let init = UrlPatternInit {
        pathname: Some("/login".to_owned()),
        ..Default::default()
    };
    let pattern = <UrlPattern>::parse(init).unwrap();

    // Match the pattern against a URL.
    let url = "https://example.com/login".parse().unwrap();
    let result = pattern
        .exec(UrlPatternMatchInput::Url(url))
        .unwrap()
        .unwrap();

    println!("{:?}", result.pathname);
    let p = BcryptPasswordEncoder::new();
    let s = "123456".to_string();
    println!("{}", p.encode(&s).unwrap())
}
