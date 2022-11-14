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
pub mod extract;


use std::any::{Any, TypeId};
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use hyper::{Body, HeaderMap, http, Method, Request, Response, StatusCode, Uri, Version};
use std::collections::HashMap;
use std::convert::Infallible;
use std::fmt::{Display, Formatter};
use std::future::Future;
use std::net::SocketAddr;
use std::ops::Deref;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::{Arc, LockResult};
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
use crate::extract::FromRequest;
use crate::router::{get_routers, register_route, Router};
use crate::security::{AuthenticationProcessingFilter, SecurityConfig};

pub type BoxError = Box<dyn std::error::Error + Send + Sync>;

macro_rules! register_method {
    ($method_name: ident, $method_def: expr) => {
        pub fn $method_name(&mut self, path: impl AsRef<str>, handler: impl HTTPHandler) {
            self.register($method_def, path, handler)
        }
    };
}

pub struct RequestCtx {
    //pub request: Request<Body>,
    pub router_params: Params,
    pub remote_addr: SocketAddr,
    pub query_params: HashMap<String, String>,
    pub extensions: http::Extensions,
    pub current_user:Option<Box<dyn UserDetails + Send + Sync>>,
    pub app_extensions:Arc<Extensions>,
    method: Method,
    uri: Uri,
    version: Version,
    headers: HeaderMap,
    body: Option<Body>,
}

impl RequestCtx {
    /// Create a new `RequestParts`.
    ///
    /// You generally shouldn't need to construct this type yourself, unless
    /// using extractors outside of axum for example to implement a
    /// [`tower::Service`].
    ///
    /// [`tower::Service`]: https://docs.rs/tower/lastest/tower/trait.Service.html
    pub fn new(remote_addr: SocketAddr,
               req: Request<Body>,
               query_params: HashMap<String, String>,
               router_parameter:Params,
               app_extensions:Arc<Extensions>) -> Self {
        let (
            http::request::Parts {
                method,
                uri,
                version,
                headers,
                extensions,
                ..
            },
            body,
        ) = req.into_parts();

        RequestCtx {
            router_params: router_parameter,
            remote_addr,
            query_params,
            extensions,
            current_user: None,
            app_extensions,
            method,
            uri,
            version,
            headers,
            body: Some(body),
        }
    }

    pub async fn extract<E: FromRequest>(self) -> anyhow::Result<E, E::Rejection> {
        E::from_request(self).await
    }

    /// Convert this `RequestParts` back into a [`Request`].
    ///
    /// Fails if The request body has been extracted, that is [`take_body`] has
    /// been called.
    ///
    /// [`take_body`]: RequestParts::take_body
    pub fn try_into_request(&self) -> anyhow::Result<Request<Body>> {
        let Self {
            method,
            uri,
            version,
            headers,
            extensions,
            mut body, ..
        } = self;

        let mut req = if let Some(body) = body.take() {
            Request::new(body)
        } else {
            return Err(anyhow!("request body has been extracted"));
        };

        *req.method_mut() = method.clone();
        *req.uri_mut() = uri.clone();
        *req.version_mut() = version.clone();
        *req.headers_mut() = headers.clone();
        *req.extensions_mut() = extensions.clone();

        Ok(req)
    }

    /// Gets a reference to the request method.
    pub fn method(&self) -> &Method {
        &self.method
    }

    /// Gets a mutable reference to the request method.
    pub fn method_mut(&mut self) -> &mut Method {
        &mut self.method
    }

    /// Gets a reference to the request URI.
    pub fn uri(&self) -> &Uri {
        &self.uri
    }

    /// Gets a mutable reference to the request URI.
    pub fn uri_mut(&mut self) -> &mut Uri {
        &mut self.uri
    }

    /// Get the request HTTP version.
    pub fn version(&self) -> Version {
        self.version
    }

    /// Gets a mutable reference to the request HTTP version.
    pub fn version_mut(&mut self) -> &mut Version {
        &mut self.version
    }

    /// Gets a reference to the request headers.
    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    /// Gets a mutable reference to the request headers.
    pub fn headers_mut(&mut self) -> &mut HeaderMap {
        &mut self.headers
    }

    /// Gets a reference to the request extensions.
    pub fn extensions(&self) -> &hyper::http::Extensions {
        &self.extensions
    }

    /// Gets a mutable reference to the request extensions.
    pub fn extensions_mut(&mut self) -> &mut hyper::http::Extensions {
        &mut self.extensions
    }

    /// Gets a reference to the request body.
    ///
    /// Returns `None` if the body has been taken by another extractor.
    pub fn body(&self) -> Option<&Body> {
        self.body.as_ref()
    }

    /// Gets a mutable reference to the request body.
    ///
    /// Returns `None` if the body has been taken by another extractor.
    // this returns `&mut Option<B>` rather than `Option<&mut B>` such that users can use it to set the body.
    pub fn body_mut(&mut self) -> &mut Option<Body> {
        &mut self.body
    }

    /// Takes the body out of the request, leaving a `None` in its place.
    pub fn take_body(&mut self) -> Option<Body> {
        self.body.take()
    }
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
    msg:&'static str,
    payload:Option<T>
}

impl <T:Serialize> EndpointResult<T> {
    pub fn new()-> EndpointResult<T>{
        EndpointResult {
            code:SUCCESS,
            msg:"",
            payload:None
        }
    }
    pub fn set_code(&mut self,code: EndpointResultCode){
        self.code = code;
    }
    pub fn set_msg(&mut self,msg:&'static str){
        self.msg = msg;
    }
    pub fn set_payload(&mut self,payload:Option<T>){
        self.payload = payload;
    }
    pub fn ok(msg: &'static str) ->Self{
        EndpointResult {
            code:SUCCESS,
            msg,
            payload:None
        }
    }
    pub fn ok_with_payload(msg: &'static str, payload:T) ->Self{
        EndpointResult {
            code:SUCCESS,
            msg,
            payload:Some(payload)
        }
    }
    pub fn server_error(msg: &'static str)->Self{
        EndpointResult {
            code:ServerError,
            msg,
            payload:Default::default()
        }
    }
    pub fn server_error_with_payload(msg: &'static str,payload:T)->Self{
        EndpointResult {
            code:ServerError,
            msg,
            payload:Some(payload)
        }
    }
    pub fn client_error(msg: &'static str)->Self{
        EndpointResult {
            code:ClientError,
            msg,
            payload:None
        }
    }
    pub fn client_error_with_payload(msg: &'static str,payload:T)->Self{
        EndpointResult {
            code:ClientError,
            msg,
            payload:Some(payload)
        }
    }
    pub fn access_denied(msg: &'static str)->Self{
        EndpointResult {
            code: AccessDenied,
            msg,
            payload:None
        }
    }
    pub fn access_denied_with_payload(msg: &'static str,payload:T)->Self{
        EndpointResult {
            code: AccessDenied,
            msg,
            payload:Some(payload)
        }
    }
    pub fn unauthorized(msg: &'static str)->Self{
        EndpointResult {
            code: Unauthorized,
            msg,
            payload:None
        }
    }
    pub fn unauthorized_with_payload(msg: &'static str,payload:T)->Self{
        EndpointResult {
            code: Unauthorized,
            msg,
            payload:Some(payload)
        }
    }
}

pub struct ResponseBuilder;

impl ResponseBuilder {
    pub fn with_msg(msg: &'static str, code: EndpointResultCode) -> Response<Body> {
        let mut endpoint_result = EndpointResult::new();
        endpoint_result.set_msg(msg);
        endpoint_result.set_code(code);
        endpoint_result.set_payload(Some(""));

        ResponseBuilder::with_endpoint_result(endpoint_result)
    }
    pub fn with_msg_and_payload<T>(msg: &'static str, payload:T, code: EndpointResultCode) -> Response<Body>
        where T:
        serde::Serialize
    {
        let mut endpoint_result = EndpointResult::new();
        endpoint_result.set_msg(msg);
        endpoint_result.set_code(code);
        endpoint_result.set_payload(Some(payload));
        ResponseBuilder::with_endpoint_result(endpoint_result)
    }

    pub fn with_endpoint_result<T>(endpoint_result: EndpointResult<T>) -> Response<Body>
        where T:
        serde::Serialize
    {
        let json = serde_json::to_string(&endpoint_result);
        let mut status = StatusCode::OK;
        match endpoint_result.code {
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
    async fn handle(&self, ctx:RequestCtx) -> anyhow::Result<Response<Body>>;
}

type BoxHTTPHandler = Box<dyn HTTPHandler>;

#[async_trait::async_trait]
impl<F: Send + Sync + 'static, Fut> HTTPHandler for F
    where
        F: Fn(RequestCtx) -> Fut,
        Fut: Future<Output = anyhow::Result<Response<Body>>> + Send + 'static,
{
    async fn handle(&self, ctx:RequestCtx) -> anyhow::Result<Response<Body>> {
        self(ctx).await
    }
}



#[async_trait::async_trait]
pub trait Filter: Send + Sync + 'static {
    async fn handle<'a>(&'a self, ctx:RequestCtx, next: Next<'a>) -> anyhow::Result<Response<Body>>;
    fn url_patterns(&self) ->String;
}

#[allow(missing_debug_implementations)]
pub struct Next<'a> {
    pub(crate) endpoint: &'a dyn HTTPHandler,
    pub(crate) next_filter: &'a [Arc<dyn Filter>],
}

impl<'a> Next<'a> {
    /// Asynchronously execute the remaining filter chain.
    pub async fn run(mut self, ctx:RequestCtx) -> anyhow::Result<Response<Body>> {
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
        let state: Option<&Box<dyn Any + Send + Sync>> = ctx.extensions.get();
        let state: Option<&T> = state.unwrap().downcast_ref();
        state.unwrap()
    }
}

pub struct AccessLogFilter;

#[async_trait::async_trait]
impl Filter for AccessLogFilter {
    async fn handle<'a>(&'a self, ctx:RequestCtx, next: Next<'a>) -> anyhow::Result<Response<Body>> {
        let start = Instant::now();
        let method = ctx.method().to_string();
        let path = ctx.uri().path().to_string();
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
    router: &'static mut Router,
    filters: Vec<Arc<dyn Filter>>,
    extensions: Extensions,
    request_state_providers:Extensions,
    enable_security:bool,
}

impl Server {
    pub fn new() -> Self {
        let routers:&'static mut Router = get_routers();
        Server {
            router:routers,
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
        /*self.router
            .entry(method)
            .or_insert_with(MethodRouter::new)
            .add(path.as_ref(), Box::new(handler));*/
        register_route(method.as_str(),path.as_ref(),handler);
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

                        let mut request_states = http::Extensions::new();

                        for state_provider in request_state_providers.iter() {
                            let provider : Option<&Box<dyn RequestStateProvider + Sync + Send>> = request_state_providers.get();
                            let state = provider.unwrap().get_state(&extensions, &req);
                            request_states.insert(state);
                        }
                        let ctx = RequestCtx::new(remote_addr,req,query_params,router_params,extensions);

                        let resp_result = next.run(ctx).await;
                        match resp_result {
                            Ok(resp)=>{
                                Ok::<_, anyhow::Error>(resp)
                            }
                            Err(error)=>{
                                error!("处理请求异常{}：{}",url, error);
                                let endpoint_result:EndpointResult<String> = EndpointResult::server_error("内部服务器错误");
                                Ok::<_, anyhow::Error>(ResponseBuilder::with_endpoint_result(endpoint_result))
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

    async fn handle_not_found(_ctx:RequestCtx) -> anyhow::Result<Response<Body>> {
        Ok(ResponseBuilder::with_status(StatusCode::NOT_FOUND))
    }
}

impl Default for Server {
    fn default() -> Self {
        unsafe {
            Self::new()
        }
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

#[test]
fn test() {
    let now = Local::now();
    let str = now.format("%Y-%m-%d %H:%M:%S").to_string();

    print!("{}",str);
}

