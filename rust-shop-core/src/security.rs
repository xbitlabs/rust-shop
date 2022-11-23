use std::any::Any;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::ops::DerefMut;
use std::sync::Arc;

use anyhow::anyhow;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Local;
use hyper::{Body, Request, Response};
use sqlx::mysql::MySqlArguments;
use sqlx::{Acquire, Arguments, MySql, Pool};

use rust_shop_macro::FormParser;

use crate::db::{SqlCommandExecutor, TransactionManager};
use crate::entity::User;
use crate::extensions::Extensions;
use crate::id_generator::ID_GENERATOR;
use crate::jwt::{AccessToken, JwtService};
use crate::jwt::DefaultJwtService;
use crate::state::State;
use crate::wechat::WeChatMiniAppService;
use crate::{parse_form_params, EndpointResult, Filter, Next, RequestCtx, ResponseBuilder, APP_EXTENSIONS};

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
        ctx:&mut RequestCtx,
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
        req:&mut RequestCtx,
    ) -> anyhow::Result<Box<dyn AuthenticationToken + Send + Sync>> {
        let params: HashMap<String, String> = parse_form_params(req).await;
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
    fn get_principal(&self) -> &(dyn Any + Send + Sync) {
        &self.principal
    }

    fn get_credentials(&self) -> &(dyn Any + Send + Sync) {
        &self.js_code
    }
}

pub struct WeChatMiniAppAuthenticationTokenResolver {}

#[async_trait::async_trait]
impl AuthenticationTokenResolver for WeChatMiniAppAuthenticationTokenResolver {
    async fn resolve(
        &self,
        req:&mut RequestCtx ,
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

pub struct WeChatUserService<'r,'a, 'b> {
    sql_command_executor: &'r mut SqlCommandExecutor<'a, 'b>,
}

impl<'r,'a, 'b> WeChatUserService<'r,'a, 'b> {
    pub fn new(
        sql_command_executor: &'r mut SqlCommandExecutor<'a, 'b>,
    ) -> Box<dyn LoadUserService + Send + Sync + 'r> {
        Box::new(WeChatUserService {
            sql_command_executor,
        })
    }
}

#[async_trait::async_trait]
impl<'r,'a, 'b> LoadUserService for WeChatUserService<'r,'a, 'b> {
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

//登录凭证，如用户名、密码
pub trait AuthenticationToken {
    fn get_principal(&self) -> &(dyn Any + Send + Sync);
    fn get_credentials(&self) -> &(dyn Any + Send + Sync);
}

//登录的用户信息
pub trait UserDetails {
    fn get_id(&self) -> &i64;
    fn get_username(&self) -> &String;
    fn get_password(&self) -> &String;
    fn get_authorities(&self) -> &Vec<String>;
    fn is_enable(&self) -> &bool;
}

//登录认证结果
pub trait Authentication {
    fn get_authentication_token(&self) -> &(dyn AuthenticationToken);

    fn get_authorities(&self) -> &Option<Vec<String>>;
    fn set_authorities(&mut self, authorities: Vec<String>);

    fn is_authenticated(&self) -> &bool;
    fn set_authenticated(&mut self, authenticated: bool);

    fn set_details(&mut self, details: Box<dyn Any + Send + Sync>);
    fn get_details(&self) -> &Box<dyn Any + Send + Sync>;
}

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
    fn get_principal(&self) -> &(dyn Any + Send + Sync) {
        &self.principal
    }

    fn get_credentials(&self) -> &(dyn Any + Send + Sync) {
        &self.credentials
    }
}

pub struct DefaultAuthentication {
    authentication_token: DefaultAuthenticationToken,
    authorities: Option<Vec<String>>,
    authenticated: bool,
    details: Box<dyn Any + Send + Sync>,
}

impl DefaultAuthentication {
    pub fn new(
        authentication_token: DefaultAuthenticationToken,
        authorities: Option<Vec<String>>,
        authenticated: bool,
        details: Box<dyn Any + Send + Sync>,
    ) -> Self {
        DefaultAuthentication {
            authentication_token,
            authorities,
            authenticated,
            details,
        }
    }
}

impl Authentication for DefaultAuthentication {
    fn get_authentication_token(&self) -> &(dyn AuthenticationToken) {
        &self.authentication_token
    }

    fn get_authorities(&self) -> &Option<Vec<String>> {
        &self.authorities
    }

    fn set_authorities(&mut self, authorities: Vec<String>) {
        self.authorities = Some(authorities)
    }

    fn is_authenticated(&self) -> &bool {
        &self.authenticated
    }

    fn set_authenticated(&mut self, is_authenticated: bool) {
        self.authenticated = is_authenticated
    }

    fn set_details(&mut self, details: Box<dyn Any + Send + Sync>) {
        let details: Result<Box<DefaultUserDetails>, Box<dyn Any + Send + Sync>> =
            details.downcast();
        self.details = details.unwrap()
    }

    fn get_details(&self) -> &Box<dyn Any + Send + Sync> {
        &self.details
    }
}

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
    fn get_principal(&self) -> &(dyn Any + Send + Sync) {
        &self.username
    }

    fn get_credentials(&self) -> &(dyn Any + Send + Sync) {
        &self.password
    }
}

pub struct DefaultLoadUserService<'r,'a, 'b> {
    sql_command_executor: &'r mut SqlCommandExecutor<'a, 'b>,
}

impl<'r,'a, 'b> DefaultLoadUserService<'r,'a, 'b> {
    pub fn new(
        sql_command_executor: &'r mut SqlCommandExecutor<'a, 'b>,
    ) -> Box<dyn LoadUserService + Send + Sync + 'r> {
        Box::new(DefaultLoadUserService {
            sql_command_executor,
        })
    }
}

#[async_trait::async_trait]
impl<'r,'a, 'b> LoadUserService for DefaultLoadUserService<'r,'a, 'b> {
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

pub struct DefaultAuthenticationProvider<'r,'a, 'b> {
    sql_command_executor: &'r mut SqlCommandExecutor<'a, 'b>,
}

impl<'r,'a, 'b> DefaultAuthenticationProvider<'r,'a, 'b> {
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
    authorities: Option<Vec<String>>,
    authenticated: bool,
    details: Box<dyn Any + Send + Sync>,
}

impl Authentication for UsernamePasswordAuthentication {
    fn get_authentication_token(&self) -> &(dyn AuthenticationToken) {
        &self.authentication_token
    }

    fn get_authorities(&self) -> &Option<Vec<String>> {
        &self.authorities
    }

    fn set_authorities(&mut self, authorities: Vec<String>) {
        self.authorities = Some(authorities)
    }

    fn is_authenticated(&self) -> &bool {
        &self.authenticated
    }

    fn set_authenticated(&mut self, authenticated: bool) {
        self.authenticated = authenticated;
    }

    fn set_details(&mut self, details: Box<dyn Any + Send + Sync>) {
        self.details = details;
    }

    fn get_details(&self) -> &Box<dyn Any + Send + Sync> {
        &self.details
    }
}

#[async_trait::async_trait]
impl<'r,'a, 'b> AuthenticationProvider for DefaultAuthenticationProvider<'r,'a, 'b> {
    async fn authenticate(
        &mut self,
        req:&mut RequestCtx,
        authentication_token: Box<dyn AuthenticationToken + Send + Sync>,
    ) -> anyhow::Result<Box<dyn Authentication + Send + Sync>> {

        let identify: Option<&String> = authentication_token.get_principal().downcast_ref();
        unsafe {
            let mut security_config: &mut SecurityConfig = APP_EXTENSIONS.get_mut::<SecurityConfig>().unwrap().borrow_mut();

            let identify = identify.unwrap();

            let load_user_service_fn =
                security_config.get_load_user_service()(req);
            let mut load_user_service = load_user_service_fn(self.sql_command_executor);
            let details = load_user_service.load_user(identify).await?;
            //手动释放load_user_service，不然无法第二次借用可变sql_command_executor
            drop(load_user_service);
            let user_details_checker: Box<dyn UserDetailsChecker + Send + Sync> = security_config
                .get_user_details_checker()(
                req
            );
            user_details_checker.check(&details).await?;
            drop(user_details_checker);
            self.additional_authentication_checks(
                req,
                &details,
                &authentication_token,
            )
                .await?;
            let mut authorities = vec![];
            for authority in details.get_authorities() {
                authorities.push(authority.to_string());
            }
            let password: Option<&String> = authentication_token.get_credentials().downcast_ref();
            let password = password.unwrap();
            let authentication: UsernamePasswordAuthentication = UsernamePasswordAuthentication {
                authentication_token: UsernamePasswordAuthenticationToken {
                    username: identify.to_string(),
                    password: password.to_string(),
                },
                authorities: Some(authorities),
                authenticated: true,
                details: Box::new(details),
            };
            Ok(Box::new(authentication))
        }
    }

    async fn additional_authentication_checks(
        &mut self,
        req:&mut RequestCtx,
        user_details: &Box<dyn UserDetails + Send + Sync>,
        authentication_token: &Box<dyn AuthenticationToken + Send + Sync>,
    ) -> anyhow::Result<()> {
        //let details : Box<DefaultUserDetails> = user_details.downcast().unwrap();
        unsafe {
            let security_config: & mut SecurityConfig = APP_EXTENSIONS.get_mut::<SecurityConfig>().unwrap();
            let password: Option<&String> = authentication_token.get_credentials().downcast_ref();
            let password = password.unwrap();
            let matches = security_config
                .get_password_encoder()
                .matches(password, user_details.get_password())?;
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
        req:&mut RequestCtx,
        authentication_token: Box<dyn AuthenticationToken + Send + Sync>,
    ) -> anyhow::Result<Box<dyn Authentication + Send + Sync>>;
    async fn additional_authentication_checks(
        &mut self,
        req:&mut RequestCtx,
        user_details: &Box<dyn UserDetails + Send + Sync>,
        authentication_token: &Box<dyn AuthenticationToken + Send + Sync>,
    ) -> anyhow::Result<()>;
}

pub trait AccessDecisionManager {
    //async fn decide(Authentication authentication, Object object, Collection<ConfigAttribute> configAttributes)
}

pub struct SecurityMetadataSource {}

pub enum Vote{
    AccessGranted,
    AccessDenied,
    AccessAbstain
}

pub trait AccessDecisionVoter {}

pub struct AuthenticationProcessingFilter {}

#[async_trait::async_trait]
impl Filter for AuthenticationProcessingFilter {
    async fn handle<'a>(
        &'a self,
        mut ctx:RequestCtx,
        next: Next<'a>,
    ) -> anyhow::Result<Response<Body>> {
        unsafe {
            let pool_state: Option<&State<Pool<MySql>>> = APP_EXTENSIONS.get();
            let pool = pool_state.unwrap().get_ref();
            let tran = pool.begin().await?;
            let mut tran_manager = TransactionManager::new(tran);
            let mut sql_command_executor = SqlCommandExecutor::WithTransaction(&mut tran_manager);

            let security_config: &mut SecurityConfig = APP_EXTENSIONS.get_mut::<SecurityConfig>().unwrap();
            //let request_states = Arc::new(ctx.request_states);
            //let request_states = Arc::clone(&request_states);

            let authentication_token_resolver = security_config.get_authentication_token_resolver()();
            let authentication_token = authentication_token_resolver.resolve(&mut ctx).await?;
            drop(authentication_token_resolver);

            let mut auth_provider = security_config.get_authentication_provider()(
                &mut ctx
            )(&mut sql_command_executor);
            let authentication = auth_provider
                .authenticate(
                    &mut ctx,
                    authentication_token,
                )
                .await;
            //手动释放load_user_service，不然无法第二次借用可变sql_command_executor
            drop(auth_provider);

            if authentication.is_ok() {
                let authentication = authentication.unwrap();
                let success_handler = security_config.get_authentication_success_handler();
                if success_handler.is_some() {
                    let result = success_handler.as_ref().unwrap()(
                        &mut ctx
                    )
                        .handle(authentication)
                        .await?;
                    Ok(result)
                } else {
                    let user_details: Option<&Box<dyn UserDetails + Send + Sync>> =
                        authentication.get_details().downcast_ref();
                    let mut jwt_service = security_config.get_jwt_service()(
                        &mut ctx
                    )(&mut sql_command_executor);
                    let access_token = jwt_service
                        .grant_access_token(*user_details.unwrap().get_id())
                        .await?;
                    let endpoint_result: EndpointResult<AccessToken> =
                        EndpointResult::ok_with_payload("", access_token);
                    Ok(ResponseBuilder::with_endpoint_result(endpoint_result))
                }
            } else {
                let fail_handler = security_config.get_authentication_failure_handler();
                if fail_handler.is_some() {
                    let result = fail_handler.as_ref().unwrap()(
                        &mut ctx
                    )
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

    fn url_patterns(&self) -> String {
        todo!()
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
        authentication: Box<dyn Authentication + Send + Sync>,
    ) -> anyhow::Result<Response<Body>>;
}

#[async_trait::async_trait]
pub trait AuthenticationFailureHandler {
    async fn handle(&self, error: anyhow::Error) -> anyhow::Result<Response<Body>>;
}

pub type AuthenticationTokenResolverFn = Box<
    dyn Fn()-> Box<dyn AuthenticationTokenResolver + Send + Sync>
        + Send
        + Sync,
>;
pub type JwtServiceFn = Box<
    dyn for<'a> Fn(
            &'a  mut RequestCtx
        ) -> Box<
            dyn for<'r,'c, 'd> Fn(
                    &'r mut SqlCommandExecutor<'c, 'd>,
                ) -> Box<dyn JwtService + Send + Sync + 'r>
                + Send
                + Sync,
        > + Send
        + Sync,
>;
pub type AuthenticationSuccessHandlerFn = Box<
    dyn for<'a> Fn(
            &'a  mut RequestCtx
        ) -> Box<dyn AuthenticationSuccessHandler + Send + Sync + 'a>
        + Send
        + Sync,
>;
pub type AuthenticationFailureHandlerFn = Box<
    dyn for<'a> Fn(
            &'a  mut RequestCtx
        ) -> Box<dyn AuthenticationFailureHandler + Send + Sync + 'a>
        + Send
        + Sync,
>;
pub type LoadUserServiceFn = Box<
    dyn for<'a> Fn(
            &'a  mut RequestCtx,
        ) -> Box<
            dyn for<'r,'c, 'd> Fn(
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
            dyn for<'r,'c, 'd> Fn(
                    &'r mut SqlCommandExecutor<'c, 'd>,
                )
                    -> Box<dyn AuthenticationProvider + Send + Sync + 'r>
                + Send
                + Sync,
        > + Send
        + Sync,
>;
pub type UserDetailsCheckerFn = Box<
    dyn for<'a> Fn(
            &'a  mut RequestCtx,
        ) -> Box<dyn UserDetailsChecker + Send + Sync + 'a>
        + Send
        + Sync,
>;


pub struct SecurityConfig {
    enable_security: bool,
    authentication_token_resolver: AuthenticationTokenResolverFn,
    jwt_service: JwtServiceFn,
    authentication_success_handler: Option<AuthenticationSuccessHandlerFn>,
    authentication_failure_handler: Option<AuthenticationFailureHandlerFn>,
    load_user_service: LoadUserServiceFn,
    authentication_provider: AuthenticationProviderFn,
    user_details_checker: UserDetailsCheckerFn,
    password_encoder: Box<dyn PasswordEncoder + Send + Sync>,
}

fn load_user_service_fn<'r,'a, 'b>(
    sql_command_executor: &'r mut SqlCommandExecutor<'a, 'b>,
) -> Box<dyn LoadUserService + Send + Sync + 'r> {
    DefaultLoadUserService::new(sql_command_executor)
}

fn jwt_service_fn<'r,'a, 'b>(
    sql_command_executor: &'r mut SqlCommandExecutor<'a, 'b>,
) -> Box<dyn JwtService + Send + Sync + 'r> {
    DefaultJwtService::new(sql_command_executor)
}

fn authentication_provider_fn<'r,'a, 'b>(
    sql_command_executor: &'r mut SqlCommandExecutor<'a, 'b>,
) -> Box<dyn AuthenticationProvider + Send + Sync + 'r> {
    DefaultAuthenticationProvider::new(sql_command_executor)
}

impl SecurityConfig {
    pub fn new() -> Self {
        SecurityConfig {
            enable_security: false,
            authentication_token_resolver: AuthenticationTokenResolverFn::from(Box::new(
                || -> Box<dyn AuthenticationTokenResolver + Send + Sync> {
                    Box::new(UsernamePasswordAuthenticationTokenResolver::new())
                },
            )),
            jwt_service: JwtServiceFn::from(Box::new(
                |req:& mut RequestCtx|
                 -> Box<
                    dyn for<'r,'c, 'd> Fn(
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
                |req:& mut RequestCtx|
                 -> Box<
                    dyn for<'r,'c, 'd> Fn(
                            &'r mut SqlCommandExecutor<'c, 'd>,
                        )
                            -> Box<(dyn LoadUserService + Send + Sync + 'r)>
                        + Send
                        + Sync,
                > { Box::new(load_user_service_fn) },
            )),
            authentication_provider: AuthenticationProviderFn::from(Box::new(
                |re:& mut RequestCtx|
                 -> Box<
                    dyn for<'r,'c, 'd> Fn(
                            &'r mut SqlCommandExecutor<'c, 'd>,
                        )
                            -> Box<(dyn AuthenticationProvider + Send + Sync + 'r)>
                        + Send
                        + Sync,
                > { Box::new(authentication_provider_fn) },
            )),
            user_details_checker: UserDetailsCheckerFn::from(Box::new(
                |req:& mut RequestCtx|
                 -> Box<dyn UserDetailsChecker + Send + Sync> {
                    Box::new(DefaultUserDetailsChecker::new())
                },
            )),
            password_encoder: Box::new(BcryptPasswordEncoder::new()),
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
}
