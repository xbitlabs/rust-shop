use std::any::{Any, TypeId};
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use hyper::{Body, Request, Response, StatusCode};
use std::collections::HashMap;
use std::convert::Infallible;
use std::fmt::{Display, Formatter};
use std::future::Future;
use std::net::SocketAddr;
use std::ops::Deref;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Instant;
use anyhow::anyhow;
use chrono::NaiveDateTime;
use criterion::Criterion;

use route_recognizer::{Params, Router as MethodRouter};

use hyper::service::{make_service_fn, service_fn};
use lazy_static::lazy_static;
use log::{error, info};
use serde::{Deserialize, Serialize};
use sqlx::{Acquire, Error, MySql, MySqlPool, Pool, Transaction};
use tokio::task_local;
use crate::config::load_config::APP_CONFIG;
use crate::core::EndpointResultCode::{AccessDenied, ClientError, ServerError, SUCCESS, Unauthorized};
use crate::extensions::Extensions;
use crate::MysqlPoolStateProvider;
use crate::state::State;
use rust_shop_std::FormParser;

macro_rules! register_method {
    ($method_name: ident, $method_def: expr) => {
        pub fn $method_name(&mut self, path: impl AsRef<str>, handler: impl HTTPHandler) {
            self.register($method_def, path, handler)
        }
    };
}

pub struct RequestCtx {
    pub request: Request<Body>,
    pub router_params: Params,
    pub remote_addr: SocketAddr,
    pub query_params: HashMap<String, String>,
    pub request_states: Extensions,
    pub current_user:Option<Box<dyn UserDetails + Send>>,
    pub extensions:Arc<Extensions>,
}

//处理http请求过程中状态保持
//pub type RequestStateProvider = dyn Fn() -> State<T> + Send + Sync;
pub trait RequestStateProvider{
    fn get_state(&self, extensions: &Arc<Extensions>, req: &Request<Body>) ->Box<dyn Any + Send + Sync>;
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum EndpointResultCode {
    SUCCESS,
    ServerError,
    ClientError,
    AccessDenied,
    Unauthorized
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EndpointResult<T>
    where T:
    Serialize
{
    code: EndpointResultCode,
    msg:String,
    payload:Option<T>
}

impl <T:Serialize> EndpointResult<T> {
    pub fn new()-> EndpointResult<T>{
        EndpointResult {
            code:SUCCESS,
            msg:"".to_string(),
            payload:None
        }
    }
    pub fn set_code(&mut self,code: EndpointResultCode){
        self.code = code;
    }
    pub fn set_msg(&mut self,msg:String){
        self.msg = msg;
    }
    pub fn set_payload(&mut self,payload:Option<T>){
        self.payload = payload;
    }
    pub fn ok(msg: String) ->Self{
        EndpointResult {
            code:SUCCESS,
            msg,
            payload:None
        }
    }
    pub fn ok_with_payload(msg: String, payload:T) ->Self{
        EndpointResult {
            code:SUCCESS,
            msg,
            payload:Some(payload)
        }
    }
    pub fn server_error(msg:String)->Self{
        EndpointResult {
            code:ServerError,
            msg,
            payload:Default::default()
        }
    }
    pub fn server_error_with_payload(msg:String,payload:T)->Self{
        EndpointResult {
            code:ServerError,
            msg,
            payload:Some(payload)
        }
    }
    pub fn client_error(msg:String)->Self{
        EndpointResult {
            code:ClientError,
            msg,
            payload:None
        }
    }
    pub fn client_error_with_payload(msg:String,payload:T)->Self{
        EndpointResult {
            code:ClientError,
            msg,
            payload:Some(payload)
        }
    }
    pub fn access_denied(msg:String)->Self{
        EndpointResult {
            code: AccessDenied,
            msg,
            payload:None
        }
    }
    pub fn access_denied_with_payload(msg:String,payload:T)->Self{
        EndpointResult {
            code: AccessDenied,
            msg,
            payload:Some(payload)
        }
    }
    pub fn unauthorized(msg:String)->Self{
        EndpointResult {
            code: Unauthorized,
            msg,
            payload:None
        }
    }
    pub fn unauthorized_with_payload(msg:String,payload:T)->Self{
        EndpointResult {
            code: Unauthorized,
            msg,
            payload:Some(payload)
        }
    }
}

pub struct ResponseBuilder;

impl ResponseBuilder {
    pub fn with_text(text: String, code: EndpointResultCode) -> Response<Body> {
        let mut endpoint_result = EndpointResult::new();
        endpoint_result.set_msg(text);
        endpoint_result.set_code(code);
        endpoint_result.set_payload(Some(""));

        ResponseBuilder::with_endpoint_result(&endpoint_result)
    }
    pub fn with_text_and_payload<T>(text: String, payload:T, code: EndpointResultCode) -> Response<Body>
        where T:
        serde::Serialize
    {
        let mut endpoint_result = EndpointResult::new();
        endpoint_result.set_msg(text);
        endpoint_result.set_code(code);
        endpoint_result.set_payload(Some(payload));
        ResponseBuilder::with_endpoint_result(&endpoint_result)
    }

    pub fn with_endpoint_result<T>(obj: &EndpointResult<T>) -> Response<Body>
        where T:
        serde::Serialize
    {
        let json = serde_json::to_string(obj);
        let mut status = StatusCode::OK;
        match obj.code {
            ServerError=>{
                status = StatusCode::INTERNAL_SERVER_ERROR;
            }
            SUCCESS =>{
                status = StatusCode::OK;
            }
            ClientError=>{
                status = StatusCode::BAD_REQUEST
            }
            AccessDenied=>{
                status = StatusCode::FORBIDDEN
            }
            Unauthorized=>{
                status = StatusCode::UNAUTHORIZED
            }
        }
        hyper::Response::builder()
            .header(
                "Content-type".parse::<hyper::header::HeaderName>().unwrap(),
                "application/json; charset=UTF-8"
                    .parse::<hyper::header::HeaderValue>()
                    .unwrap(),
            )
            .status(status)
            .body(hyper::Body::from(json.unwrap()))
            .unwrap()
    }
    pub fn with_status(status: StatusCode) -> Response<Body> {
        hyper::Response::builder()
            .status(status)
            .body(Body::empty())
            .unwrap()
    }
}

#[async_trait::async_trait]
pub trait HTTPHandler: Send + Sync + 'static {
    async fn handle(&self, ctx: RequestCtx) -> anyhow::Result<Response<Body>>;
}

type BoxHTTPHandler = Box<dyn HTTPHandler>;

#[async_trait::async_trait]
impl<F: Send + Sync + 'static, Fut> HTTPHandler for F
where
    F: Fn(RequestCtx) -> Fut,
    Fut: Future<Output = anyhow::Result<Response<Body>>> + Send + 'static,
{
    async fn handle(&self, ctx: RequestCtx) -> anyhow::Result<Response<Body>> {
        self(ctx).await
    }
}

type Router = HashMap<String, MethodRouter<BoxHTTPHandler>>;

#[async_trait::async_trait]
pub trait Filter: Send + Sync + 'static {
    async fn handle<'a>(&'a self, ctx: RequestCtx, next: Next<'a>) -> anyhow::Result<Response<Body>>;
}

#[allow(missing_debug_implementations)]
pub struct Next<'a> {
    pub(crate) endpoint: &'a dyn HTTPHandler,
    pub(crate) next_filter: &'a [Arc<dyn Filter>],
}

impl<'a> Next<'a> {
    /// Asynchronously execute the remaining filter chain.
    pub async fn run(mut self, ctx: RequestCtx) -> anyhow::Result<Response<Body>> {
        if let Some((current, next)) = self.next_filter.split_first() {
            self.next_filter = next;
            current.handle(ctx, self).await
        } else {
            (self.endpoint).handle(ctx).await
        }
    }
}

pub struct RequestStateResolver;

impl  RequestStateResolver {
    pub fn get<'a, T: 'a + 'static>(ctx: &'a RequestCtx) -> &'a T
    {
        let state: Option<&Box<dyn Any + Send + Sync>> = ctx.request_states.get();
        let state: Option<&T> = state.unwrap().downcast_ref();
        state.unwrap()
    }
}

pub struct AccessLogFilter;

#[async_trait::async_trait]
impl Filter for AccessLogFilter {
    async fn handle<'a>(&'a self, ctx: RequestCtx, next: Next<'a>) -> anyhow::Result<hyper::Response<Body>> {
        let start = Instant::now();
        let method = ctx.request.method().to_string();
        let path = ctx.request.uri().path().to_string();
        let remote_addr = ctx.remote_addr;

        let res = next.run(ctx).await;

        info!(
            "{} {:?} {}  {}ms",
            method,
            path,
            remote_addr,
            start.elapsed().as_millis()
        );
        res
    }
}

pub trait  LoadUserService{
    fn load_user_by_username(&self,username:String)-> dyn UserDetails;
}
//登录主体
pub trait Principal{
    fn get_name(&self)->&String;
}
pub struct DefaultPrincipal{
    name:String
}
impl DefaultPrincipal{
    pub fn new(name:String)->Self{
        DefaultPrincipal{
            name
        }
    }
}
impl Principal for DefaultPrincipal {
    fn get_name(&self) -> &String {
        &self.name
    }
}
pub struct Username{
    pub the_username:String,
}

impl Username {
    pub fn new(the_username:String)->Self{
        Username{
            the_username
        }
    }
}
impl Principal for Username{
    fn get_name(&self) -> &String {
        &self.the_username
    }
}

//登录凭证
pub trait Credentials{
    fn get_credentials(&self)->&String;
}
pub struct DefaultCredentials{
    credentials:String,
}

impl DefaultCredentials {
    pub fn new(credentials:String)->Self{
        DefaultCredentials{
            credentials
        }
    }
}

impl Credentials for DefaultCredentials {
    fn get_credentials(&self) -> &String {
        &self.credentials
    }
}
pub struct Password{
    pub the_password:String,
}

impl Password {
    pub fn new(the_password:String)->Self{
        Password{
            the_password
        }
    }
}
impl Credentials for Password {
    fn get_credentials(&self) -> &String {
        &self.the_password
    }
}
//登录凭证，如用户名、密码
pub trait AuthenticationToken{
    fn get_principal(&self)->Box<&dyn Principal>;
    fn get_credentials(&self)->Box<&dyn Credentials>;
}
//登录的用户信息
pub trait UserDetails{
    fn get_id(&self)->&i64;
    fn get_username(&self)->&String;
    fn get_authorities(&self)->&Vec<String>;
    fn is_enable(&self)->&bool;
}

//登录认证结果
pub trait Authentication {
    fn get_authentication_token(&self)->&(dyn AuthenticationToken);

    fn get_authorities(&self)->&Option<Vec<String>>;
    fn set_authorities(&mut self,authorities:Vec<String>);

    fn is_authenticated(&self)->&bool;
    fn set_authenticated(&mut self,is_authenticated:bool);

    fn set_details(&mut self,details:Box<dyn Any  + Send + Sync>);
    fn get_details(&self)-> &Box<dyn UserDetails  + Send + Sync>;
}
pub struct DefaultAuthenticationToken{
    principal:DefaultPrincipal,
    credentials:DefaultCredentials,
}

impl DefaultAuthenticationToken {
    pub fn new(principal:DefaultPrincipal,credentials:DefaultCredentials)->Self{
        DefaultAuthenticationToken{
            principal,
            credentials
        }
    }
}

impl AuthenticationToken for DefaultAuthenticationToken {
    fn get_principal(&self) -> Box<&dyn Principal> {
        Box::new(&self.principal)
    }

    fn get_credentials(&self) -> Box<&dyn Credentials> {
        Box::new(&self.credentials)
    }
}
pub struct DefaultAuthentication{
    authentication_token:DefaultAuthenticationToken,
    authorities:Option<Vec<String>>,
    authenticated:bool,
    details:Box<dyn UserDetails + Send + Sync>,
}
impl DefaultAuthentication{
    pub fn new(authentication_token:DefaultAuthenticationToken,
               authorities:Option<Vec<String>>,
               authenticated:bool,
               details:Box<dyn UserDetails + Send + Sync>)->Self{
        DefaultAuthentication{
            authentication_token,
            authorities,
            authenticated,
            details
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
        let details : Result<Box<DefaultUserDetails>, Box<dyn Any + Send + Sync>> = details.downcast();
        self.details = details.unwrap()
    }

    fn get_details(&self) -> &Box<dyn UserDetails + Send + Sync> {
        &self.details
    }
}
pub struct DefaultUserDetails{
    id:i64,
    username:String,
    authorities:Vec<String>,
    enable:bool,
}

impl DefaultUserDetails {
    pub fn new(id:i64,username:String,authorities:Vec<String>,enable:bool)->Self{
        DefaultUserDetails{
            id,
            username,
            authorities,
            enable
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
    fn get_authorities(&self) -> &Vec<String> {
        &self.authorities
    }
    fn is_enable(&self) -> &bool {
        &self.enable
    }
}

pub struct UsernamePasswordAuthenticationToken{
    username:Username,
    password:Password,
    /*pub authentication_token: Box<dyn AuthenticationToken>,*/
    details:Option<Box<dyn UserDetails + Send + Sync>>,
    authorities:Option<Vec<String>>,
    authenticated:bool,
}
impl UsernamePasswordAuthenticationToken{
    pub fn unauthenticated(username:String,password:String)->UsernamePasswordAuthenticationToken{
        UsernamePasswordAuthenticationToken{
            username:Username{
                the_username : username
            },
            password:Password{
                the_password:password
            },
            details: None,
            authorities: None,
            authenticated: false
        }
    }
    pub fn authenticated(username:String,password:String,authorities:Option<Vec<String>>)->UsernamePasswordAuthenticationToken{
        UsernamePasswordAuthenticationToken{
            username:Username{
                the_username : username
            },
            password:Password{
                the_password:password
            },
            details: None,
            authorities,
            authenticated: true
        }
    }
}
impl AuthenticationToken for UsernamePasswordAuthenticationToken{
    fn get_principal(&self) -> Box<&dyn Principal> {
        Box::new(&self.username)
    }

    fn get_credentials(&self) -> Box<&dyn Credentials> {
        Box::new(&self.password)
    }
}
impl Authentication for  UsernamePasswordAuthenticationToken{
    fn get_authentication_token(&self) -> &(dyn AuthenticationToken) {
        self
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

    fn set_authenticated(&mut self,is_authenticated: bool) {
        self.authenticated = is_authenticated;
    }

    fn set_details(&mut self, details:Box<dyn Any + Send + Sync>) {
        self.details = Some(details);
    }

    fn get_details(&self) -> &Box<dyn UserDetails  + Send + Sync> {
        &self.details
    }
}
#[async_trait::async_trait]
pub trait AuthenticationProvider{
    async fn authenticate(&self,authentication_token: Box<dyn AuthenticationToken + Send + Sync>) -> anyhow::Result<Box<dyn Authentication>>;
}
pub struct AuthenticationProviderManager{
    pub providers:HashMap<TypeId,Box<dyn AuthenticationProvider + Send + Sync + 'static>>
}

impl AuthenticationProviderManager {
    pub fn new()->Self{
        AuthenticationProviderManager{
            providers:HashMap::new(),
        }
    }
    pub fn add(&mut self, authentication_token_type_id:TypeId, provider: Box<dyn AuthenticationProvider + Send + Sync>){
        self.providers.entry(authentication_token_type_id).or_insert_with(||{
            provider
        });
    }
    pub fn get(&self,authentication_token_type_id:TypeId)->Option<&Box<dyn AuthenticationProvider + Send + Sync>>{
        self.providers.get(&authentication_token_type_id)
    }
}

pub struct AuthenticationProcessingFilter{

}
#[async_trait::async_trait]
impl Filter for AuthenticationProcessingFilter{
    async fn handle<'a>(&'a self, ctx: RequestCtx, next: Next<'a>) -> anyhow::Result<Response<Body>> {
        todo!()
    }
}
pub struct UsernamePasswordAuthenticationFilter{

}

#[derive(FormParser,serde::Deserialize,serde::Serialize,Debug)]
pub struct UsernamePassword{
    pub username:String,
    pub password:String,
}
#[async_trait::async_trait]
impl Filter for UsernamePasswordAuthenticationFilter{
    async fn handle<'a>(&'a self, ctx: RequestCtx, next: Next<'a>) -> anyhow::Result<Response<Body>> {
        let username_password_form_parser : UsernamePasswordFormParser = UsernamePassword::build_form_parser();
        let username_password:UsernamePassword = username_password_form_parser.parse(ctx.request).await?;
        let authentication_token = UsernamePasswordAuthenticationToken::unauthenticated(username_password.username,username_password.password);
        let auth_provider_manager : Option<&AuthenticationProviderManager> = ctx.extensions.get();
        if auth_provider_manager.is_none() {
            return Err(anyhow!("认证失败：找不到AuthenticationProviderManager"));
        }
        let auth_provider_manager = auth_provider_manager.unwrap();
        let auth_provider = auth_provider_manager.get(TypeId::of::<UsernamePasswordAuthenticationToken>());
        if auth_provider.is_none() {
            return Err(anyhow!("认证失败：找不到跟UsernamePasswordAuthenticationToken匹配的AuthenticationProvider"))
        }
        let auth_provider = auth_provider.unwrap();
        let auth = auth_provider.authenticate(Box::new(authentication_token)).await;

        todo!()
    }
}
#[async_trait::async_trait]
pub trait AuthenticationSuccessHandler{
    async fn handle<'a>(&'a self, ctx: RequestCtx, next: Next<'a>) -> anyhow::Result<Response<Body>>;
}
/*#[async_trait::async_trait]
impl Filter for AuthenticationSuccessHandler{
    async fn handle<'a>(&'a self, ctx: RequestCtx, next: Next<'a>) -> anyhow::Result<Response<Body>> {
        todo!()
    }
}*/
#[async_trait::async_trait]
pub trait  AuthenticationFailureHandler{
    async fn handle<'a>(&'a self, ctx: RequestCtx, next: Next<'a>) -> anyhow::Result<Response<Body>>;
}
/*#[async_trait::async_trait]
impl Filter for AuthenticationFailureHandler{
    async fn handle<'a>(&'a self, ctx: RequestCtx, next: Next<'a>) -> anyhow::Result<Response<Body>> {
        todo!()
    }
}*/
pub struct SecurityConfig{
    enable_security:bool,
    authentication_success_handler:Box<dyn AuthenticationSuccessHandler>,
    authentication_failure_handler:Box<dyn AuthenticationFailureHandler>,
    load_user_service:Box<dyn LoadUserService>,
    authentication_provider:Box<dyn AuthenticationProvider>
}
pub trait SecurityFilter{

}

pub struct MysqlPoolManager<'a>{
    tran:Option<Transaction<'a,MySql>>,
    pool_state:Option<State<Pool<MySql>>>
}

impl <'a> MysqlPoolManager<'a> {
    pub fn new(pool_state:State<Pool<MySql>>) ->Self{
        MysqlPoolManager{
            pool_state : Some(pool_state),
            tran:None
        }
    }
    pub fn get_pool(&self) -> &Pool<MySql> {
        self.pool_state.as_ref().unwrap().as_ref()
    }
    pub async fn begin(&mut self)->anyhow::Result<&Transaction<'a,MySql>>{
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
}

impl <'a> Drop for MysqlPoolManager<'a> {
    fn drop(&mut self) {
        println!("释放MysqlPoolManager");
    }
}

pub struct Server {
    router: Router,
    filters: Vec<Arc<dyn Filter>>,
    extensions: Extensions,
    request_state_providers:Extensions,
}

impl Server {
    pub fn new() -> Self {
        let mut extensions = Extensions::new();
        extensions.insert(State::new(AuthenticationProviderManager::new()));

        Server {
            router: HashMap::new(),
            filters: Vec::new(),
            extensions,
            request_state_providers : Extensions::new(),
        }
    }
    pub fn get_extension<T:'static + Sync + Send>(&self) ->Option<&State<T>>{
        self.extensions.get()
    }
    pub fn register(
        &mut self,
        method: impl ToString,
        path: impl AsRef<str>,
        handler: impl HTTPHandler,
    ) {
        let method = method.to_string().to_uppercase();
        self.router
            .entry(method)
            .or_insert_with(MethodRouter::new)
            .add(path.as_ref(), Box::new(handler));
    }

    register_method!(get, "GET");
    register_method!(head, "HEAD");
    register_method!(post, "POST");
    register_method!(put, "PUT");
    register_method!(delete, "DELETE");
    register_method!(connect, "CONNECT");
    register_method!(options, "OPTIONS");
    register_method!(trace, "TRACE");
    register_method!(patch, "PATCH");

    pub fn filter(&mut self, filter: impl Filter) {
        self.filters.push(Arc::new(filter));
    }
    pub fn extension<U: 'static + Send + Sync>(&mut self, ext: U)  {
        self.extensions.insert(ext);
    }
    pub fn request_state<U: 'static + Send + Sync>(&mut self, ext: U)  {
        self.request_state_providers.insert(ext);
    }
    pub async fn run(self, addr: SocketAddr) -> anyhow::Result<()> {
        let Self {
            router,
            filters,
            extensions,
            request_state_providers
        } = self;
        let router = Arc::new(router);
        let filters = Arc::new(filters);
        let extensions = Arc::new(extensions);
        let request_state_providers = Arc::new(request_state_providers);

        let make_svc = make_service_fn(|conn: &hyper::server::conn::AddrStream| {
            let remote_addr = conn.remote_addr();

            let router = router.clone();
            let filters = filters.clone();
            let extensions = extensions.clone();
            let request_state_providers = request_state_providers.clone();

                async move {
                Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
                    let router = router.clone();
                    let filters = filters.clone();
                    let extensions = extensions.clone();
                    let request_state_providers = request_state_providers.clone();

                    async move {
                        let method = &req.method().as_str().to_uppercase();

                        let mut router_params = Params::new();
                        let endpoint = match router.get(method) {
                            Some(router) => match router.recognize(req.uri().path()) {
                                Ok(m) => {
                                    router_params = m.params().clone();
                                    &***m.handler()
                                }
                                Err(_) => &Self::handle_not_found,
                            },
                            None => &Self::handle_not_found,
                        };

                        let next = Next {
                            endpoint,
                            next_filter: &**filters,
                        };

                        let mut query_params = HashMap::new();

                        if let Some(q) = req.uri().query() {
                            query_params = form_urlencoded::parse(q.as_bytes())
                                .into_owned()
                                .collect::<HashMap<String, String>>();
                        };

                        let url = req.uri().to_string();

                        let mut request_states = Extensions::new();

                        for state_provider in request_state_providers.iter() {
                            let provider : Option<&Box<dyn RequestStateProvider + Sync + Send>> = request_state_providers.get();
                            let state = provider.unwrap().get_state(&extensions, &req);
                            request_states.insert(state);
                        }
                        let ctx = RequestCtx {
                            request: req,
                            router_params,
                            remote_addr,
                            query_params,
                            request_states,
                            current_user:None,
                            extensions
                        };

                        let resp_result = next.run(ctx).await;
                        match resp_result {
                            Ok(resp)=>{
                                Ok::<_, anyhow::Error>(resp)
                            }
                            Err(error)=>{
                                error!("处理请求异常{}：{}",url, error);
                                let endpoint_result:EndpointResult<String> = EndpointResult::server_error("内部服务器错误".to_string());
                                Ok::<_, anyhow::Error>(ResponseBuilder::with_endpoint_result(&endpoint_result))
                            }
                        }

                    }
                }))
            }
        });

        let server = hyper::Server::bind(&addr).serve(make_svc);

        server
            .await
            .map_err(|e| anyhow!(format!("server run error: {:?}", e)))?;

        Ok(())
    }

    async fn handle_not_found(_ctx: RequestCtx) -> anyhow::Result<Response<Body>> {
        Ok(ResponseBuilder::with_status(StatusCode::NOT_FOUND))
    }
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}
use hyper::body::Buf;
use schemars::_private::NoSerialize;
use serde_json::Value;
use sqlx::encode::IsNull::No;

pub async fn parse_request_json<T>(req:Request<Body>)->anyhow::Result<T>
    where for<'a> T:
    serde::Deserialize<'a>
{
    let whole_body = hyper::body::aggregate(req).await?;
    // Decode as JSON...
    let mut read_result: Result<T, serde_json::Error> = serde_json::from_reader(whole_body.reader());
    match read_result {
        Ok(result) => {
            Ok(result)
        }
        Err(error) => {
            Err(anyhow!(error))
        }
    }
}
/*pub fn parse_request_params<T>(ctx:RequestCtx)->anyhow::Result<T>{
    Ok(())
}*/
pub async fn parse_form_params(req:Request<Body>)->HashMap<String,String>{
    let url = req.uri().to_string();
    let b = hyper::body::to_bytes(req).await;
    if b.is_err() {
        error!("解析form参数异常{}：{}",url,b.err().unwrap());
        return HashMap::new();
    }
    let params = form_urlencoded::parse(b.unwrap().as_ref())
        .into_owned()
        .collect::<HashMap<String, String>>();
    params
}
macro_rules! zoom_and_enhance {
    (struct $name:ident { $($fname:ident : $ftype:ty),* }) => {
     struct $name {
      $($fname : $ftype),*
     }

     impl $name {
      fn field_names() -> &'static [&'static str] {
       static NAMES: &'static [&'static str] = &[$(stringify!($fname)),*];
       NAMES
      }
     }
    }
}

zoom_and_enhance! {
    struct Export {
        first_name: String,
        last_name: String,
        gender: String,
        date_of_birth: String,
        address: String
    }
}

#[test]
fn test() {
    let map:HashMap<String,String> = HashMap::new();
    let value = map.get("key");
    if value.is_some() {
        let result = value.unwrap().parse::<i64>();
        if result.is_err() {
            let result = result.unwrap();
        }
    }
    let s  = "sss";
    println!("{:?}", Export::field_names());
}

