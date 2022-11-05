pub mod extensions;
pub mod state;
pub mod security;
pub mod router;
pub mod jwt;
pub mod db_pool_manager;
pub mod wechat_service;
pub mod app_config;
pub mod jwt_service;
pub mod entity;
pub mod id_generator;
mod extract;
mod response;

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
use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::Local;
use chrono::Utc;
use route_recognizer::{Params, Router as MethodRouter};
use hyper::service::{make_service_fn, service_fn};
use lazy_static::lazy_static;
use log::{error, info};
use serde::{Deserialize, Serialize};
use sqlx::{Acquire, Error, MySql, MySqlPool, Pool, Transaction};
use tokio::task_local;
use crate::extensions::Extensions;
use crate::state::State;
use std::sync::Mutex;
use crate::security::UserDetails;
use hyper::body::Buf;
use schemars::_private::NoSerialize;
use serde_json::Value;
use sqlx::encode::IsNull::No;
use time::OffsetDateTime;
use crate::EndpointResultCode::{AccessDenied, ClientError, ServerError, SUCCESS, Unauthorized};
use crate::router::Router;
use crate::security::{AuthenticationProcessingFilter, SecurityConfig};


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
    pub current_user:Option<Box<dyn UserDetails + Send + Sync>>,
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



#[async_trait::async_trait]
pub trait Filter: Send + Sync + 'static {
    async fn handle<'a>(&'a self, ctx: RequestCtx, next: Next<'a>) -> anyhow::Result<Response<Body>>;
    fn url_patterns(&self) ->String;
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
    async fn handle<'a>(&'a self, ctx: RequestCtx, next: Next<'a>) -> anyhow::Result<Response<Body>> {
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

    fn url_patterns(&self) -> String {
        todo!()
    }
}



pub struct Server {
    router: Router,
    filters: Vec<Arc<dyn Filter>>,
    extensions: Extensions,
    request_state_providers:Extensions,
    enable_security:bool,
}

impl Server {
    pub fn new() -> Self {
        Server {
            router: HashMap::new(),
            filters: Vec::new(),
            extensions : Extensions::new(),
            request_state_providers : Extensions::new(),
            enable_security:false,
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
    pub fn security_config(&mut self,security_config:SecurityConfig){
        self.enable_security = security_config.is_enable_security();
        self.extensions.insert(State::new(security_config));
        if self.enable_security {
            self.filters.push(Arc::new(AuthenticationProcessingFilter{}))
        }
    }
    pub async fn run(self, addr: SocketAddr) -> anyhow::Result<()> {
        let Self {
            router,
            filters,
            extensions,
            request_state_providers,
            enable_security
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
    let now = Local::now();
    let str = now.format("%Y-%m-%d %H:%M:%S").to_string();

    print!("{}",str);
}
