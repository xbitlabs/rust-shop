extern crate core;

use std::any::{Any, TypeId};
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::Infallible;
use std::fmt::{Display, Formatter};
use std::future::Future;
use std::io::Cursor;
use std::net::SocketAddr;
use std::ops::Deref;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Mutex;
use std::sync::{Arc, LockResult};
use std::time::Instant;

use anyhow::anyhow;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Local;
use chrono::NaiveDateTime;
use chrono::Utc;
use criterion::Criterion;
use http::{Method, Uri, Version};
use hyper::body::{Buf, Bytes};
use hyper::header::{HeaderValue, ToStrError};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, HeaderMap, Request, Response, StatusCode};
use lazy_static::lazy_static;
use log::{error, info};
use route_recognizer::{Params, Router as MethodRouter};
use schemars::_private::NoSerialize;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::encode::IsNull::No;
use sqlx::{Acquire, Error, MySql, MySqlPool, Pool, Transaction};
use time::OffsetDateTime;
use tokio::task_local;

use crate::extensions::Extensions;
use crate::extract::header::Header;
use crate::extract::path_variable::PathVariable;
use crate::extract::request_param::RequestParam;
use crate::router::{get_routers, register_route, Router};
use crate::security::UserDetails;
use crate::security::{AuthenticationProcessingFilter, SecurityConfig};
use crate::state::State;
use crate::EndpointResultCode::{AccessDenied, ClientError, ServerError, Unauthorized, SUCCESS};

pub mod app_config;
pub mod db;
pub mod entity;
pub mod extensions;
pub mod extract;
pub mod id_generator;
pub mod jwt;
pub mod jwt_service;
pub mod router;
pub mod security;
pub mod state;
pub mod wechat_service;
use crate::extract::json::body_to_bytes;
use http::request::Parts as HttpParts;
use once_cell::sync::Lazy;

pub static mut APP_EXTENSIONS: Lazy<Extensions> = Lazy::new(|| {
    let mut m: Extensions = Extensions::new();
    m
});

pub type BoxError = Box<dyn std::error::Error + Send + Sync>;

macro_rules! register_method {
    ($method_name: ident, $method_def: expr) => {
        pub fn $method_name(&mut self, path: impl AsRef<str>, handler: impl HTTPHandler) {
            self.register($method_def, path, handler)
        }
    };
}

pub struct RequestCtx {
    //pub parts: HttpParts,
    pub body: Body,
    pub router_params: Params,
    pub remote_addr: SocketAddr,
    pub query_params: HashMap<String, String>,
    pub headers: HashMap<String, Option<String>>,
    //pub request_states: Extensions,
    //pub current_user: Option<Box<dyn UserDetails + Send + Sync>>,
    method: Method,
    uri: Uri,
    version: Version,
    extensions: Extensions,
}

//处理http请求过程中状态保持
//pub type RequestStateProvider = dyn Fn() -> State<T> + Send + Sync;
pub trait RequestStateProvider {
    fn get_state(&self, req: &mut RequestCtx) -> Box<dyn Any + Send + Sync>;
    fn matches(&self, ctx: &mut RequestCtx) -> bool;
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum EndpointResultCode {
    SUCCESS,
    ServerError,
    ClientError,
    AccessDenied,
    Unauthorized,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct EndpointResult<T>
where
    T: Serialize,
{
    code: EndpointResultCode,
    msg: &'static str,
    payload: Option<T>,
}

impl<T: Serialize> EndpointResult<T> {
    pub fn new() -> EndpointResult<T> {
        EndpointResult {
            code: SUCCESS,
            msg: "",
            payload: None,
        }
    }
    pub fn set_code(&mut self, code: EndpointResultCode) {
        self.code = code;
    }
    pub fn set_msg(&mut self, msg: &'static str) {
        self.msg = msg;
    }
    pub fn set_payload(&mut self, payload: Option<T>) {
        self.payload = payload;
    }
    pub fn ok(msg: &'static str) -> Self {
        EndpointResult {
            code: SUCCESS,
            msg,
            payload: None,
        }
    }
    pub fn ok_with_payload(msg: &'static str, payload: T) -> Self {
        EndpointResult {
            code: SUCCESS,
            msg,
            payload: Some(payload),
        }
    }
    pub fn server_error(msg: &'static str) -> Self {
        EndpointResult {
            code: ServerError,
            msg,
            payload: Default::default(),
        }
    }
    pub fn server_error_with_payload(msg: &'static str, payload: T) -> Self {
        EndpointResult {
            code: ServerError,
            msg,
            payload: Some(payload),
        }
    }
    pub fn client_error(msg: &'static str) -> Self {
        EndpointResult {
            code: ClientError,
            msg,
            payload: None,
        }
    }
    pub fn client_error_with_payload(msg: &'static str, payload: T) -> Self {
        EndpointResult {
            code: ClientError,
            msg,
            payload: Some(payload),
        }
    }
    pub fn access_denied(msg: &'static str) -> Self {
        EndpointResult {
            code: AccessDenied,
            msg,
            payload: None,
        }
    }
    pub fn access_denied_with_payload(msg: &'static str, payload: T) -> Self {
        EndpointResult {
            code: AccessDenied,
            msg,
            payload: Some(payload),
        }
    }
    pub fn unauthorized(msg: &'static str) -> Self {
        EndpointResult {
            code: Unauthorized,
            msg,
            payload: None,
        }
    }
    pub fn unauthorized_with_payload(msg: &'static str, payload: T) -> Self {
        EndpointResult {
            code: Unauthorized,
            msg,
            payload: Some(payload),
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
    pub fn with_msg_and_payload<T>(
        msg: &'static str,
        payload: T,
        code: EndpointResultCode,
    ) -> Response<Body>
    where
        T: serde::Serialize,
    {
        let mut endpoint_result = EndpointResult::new();
        endpoint_result.set_msg(msg);
        endpoint_result.set_code(code);
        endpoint_result.set_payload(Some(payload));
        ResponseBuilder::with_endpoint_result(endpoint_result)
    }

    pub fn with_endpoint_result<T>(endpoint_result: EndpointResult<T>) -> Response<Body>
    where
        T: serde::Serialize,
    {
        let json = serde_json::to_string(&endpoint_result);
        let mut status = StatusCode::OK;
        match endpoint_result.code {
            ServerError => {
                status = StatusCode::INTERNAL_SERVER_ERROR;
            }
            SUCCESS => {
                status = StatusCode::OK;
            }
            ClientError => status = StatusCode::BAD_REQUEST,
            AccessDenied => status = StatusCode::FORBIDDEN,
            Unauthorized => status = StatusCode::UNAUTHORIZED,
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
    async fn handle(&self, mut ctx: RequestCtx) -> anyhow::Result<Response<Body>>;
}

type BoxHTTPHandler = Box<dyn HTTPHandler>;

#[async_trait::async_trait]
impl<F: Send + Sync + 'static, Fut> HTTPHandler for F
where
    F: Fn(RequestCtx) -> Fut,
    Fut: Future<Output = anyhow::Result<Response<Body>>> + Send + 'static,
{
    async fn handle(&self, mut ctx: RequestCtx) -> anyhow::Result<Response<Body>> {
        self(ctx).await
    }
}

#[async_trait::async_trait]
pub trait Filter: Send + Sync + 'static {
    async fn handle<'a>(
        &'a self,
        mut ctx: RequestCtx,
        next: Next<'a>,
    ) -> anyhow::Result<Response<Body>>;
    fn url_patterns(&self) -> String;
}

#[allow(missing_debug_implementations)]
pub struct Next<'a> {
    pub(crate) endpoint: &'a dyn HTTPHandler,
    pub(crate) next_filter: &'a [Arc<dyn Filter>],
}

impl<'a> Next<'a> {
    /// Asynchronously execute the remaining filter chain.
    pub async fn run(mut self, mut ctx: RequestCtx) -> anyhow::Result<Response<Body>> {
        if let Some((current, next)) = self.next_filter.split_first() {
            self.next_filter = next;
            current.handle(ctx, self).await
        } else {
            (self.endpoint).handle(ctx).await
        }
    }
}

pub struct RequestStateResolver;

impl RequestStateResolver {
    pub fn get<'a, T: 'a + 'static>(ctx: &'a mut RequestCtx) -> &'a T {
        let state: Option<&Box<dyn Any + Send + Sync>> = ctx.extensions.get();
        let state: Option<&T> = state.unwrap().downcast_ref();
        state.unwrap()
    }
}

pub struct AccessLogFilter;

#[async_trait::async_trait]
impl Filter for AccessLogFilter {
    async fn handle<'a>(
        &'a self,
        mut ctx: RequestCtx,
        next: Next<'a>,
    ) -> anyhow::Result<Response<Body>> {
        let start = Instant::now();
        let method = ctx.method.to_string();
        let path = ctx.uri.path().to_string();
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
    request_state_providers: Extensions,
    enable_security: bool,
}

impl Server {
    pub fn new() -> Self {
        let routers: &'static mut Router = get_routers();
        Server {
            router: routers,
            filters: Vec::new(),
            extensions: Extensions::new(),
            request_state_providers: Extensions::new(),
            enable_security: false,
        }
    }
    pub fn get_extension<T: 'static + Sync + Send>(&self) -> Option<&State<T>> {
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
        register_route(method.to_string(), path.as_ref().to_string(), handler);
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
    pub fn extension<U: 'static + Send + Sync>(&mut self, ext: U) {
        unsafe {
            APP_EXTENSIONS.insert(ext);
        }
    }
    pub fn request_state<U: 'static + Send + Sync>(&mut self, ext: U) {
        self.request_state_providers.insert(ext);
    }
    pub fn security_config(&mut self, security_config: SecurityConfig) {
        self.enable_security = security_config.is_enable_security();
        self.extensions.insert(State::new(security_config));
        if self.enable_security {
            self.filters
                .push(Arc::new(AuthenticationProcessingFilter {}))
        }
    }
    pub async fn run(self, addr: SocketAddr) -> anyhow::Result<()> {
        let Self {
            router,
            filters,
            request_state_providers,
            enable_security,
            ..
        } = self;
        let router = Arc::new(router);
        let filters = Arc::new(filters);
        //let extensions = Arc::new(extensions);
        let request_state_providers = Arc::new(request_state_providers);

        let make_svc = make_service_fn(|conn: &hyper::server::conn::AddrStream| {
            let remote_addr = conn.remote_addr();

            let router = router.clone();
            let filters = filters.clone();
            //let extensions = extensions.clone();
            let request_state_providers = request_state_providers.clone();

            async move {
                Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
                    let router = router.clone();
                    let filters = filters.clone();
                    //let extensions = extensions.clone();
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
                                Err(_) => &handle_not_found,
                            },
                            None => &handle_not_found,
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

                        /*        let mut request_states = Extensions::new();
                        for state_provider in request_state_providers.iter() {
                            let provider: Option<&Box<dyn RequestStateProvider + Sync + Send>> =
                                request_state_providers.get();
                            let state = provider.unwrap().get_state( &mut ctx);
                            request_states.insert(state);
                        }
                        let arc_request_states = Arc::new(request_states);*/

                        let mut request_states = Extensions::new();

                        let mut headers = HashMap::new();
                        for header in req.headers() {
                            let value = match header.1.to_str() {
                                Ok(val) => Some(val.to_string()),
                                Err(_) => None,
                            };
                            headers.insert(header.0.to_string(), value);
                        }
                        let (
                            http::request::Parts {
                                method,
                                uri,
                                version,
                                extensions,
                                ..
                            },
                            body,
                        ) = req.into_parts();

                        let mut ctx = RequestCtx {
                            //request: req,
                            router_params,
                            remote_addr,
                            query_params,
                            headers,
                            method,
                            uri,
                            version,
                            body,
                            extensions: Extensions::new(),
                        };

                        let resp_result = next.run(ctx).await;
                        match resp_result {
                            Ok(resp) => Ok::<_, anyhow::Error>(resp),
                            Err(error) => {
                                error!("处理请求异常{}：{}", url, error);
                                let endpoint_result: EndpointResult<String> =
                                    EndpointResult::server_error("内部服务器错误");
                                Ok::<_, anyhow::Error>(ResponseBuilder::with_endpoint_result(
                                    endpoint_result,
                                ))
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
}

impl Default for Server {
    fn default() -> Self {
        unsafe { Self::new() }
    }
}

pub async fn parse_request_json<T>(req: Request<Body>) -> anyhow::Result<T>
where
    for<'a> T: serde::Deserialize<'a>,
{
    let whole_body = hyper::body::aggregate(req).await?;
    // Decode as JSON...
    let mut read_result: Result<T, serde_json::Error> =
        serde_json::from_reader(whole_body.reader());
    match read_result {
        Ok(result) => Ok(result),
        Err(error) => Err(anyhow!(error)),
    }
}

pub async fn parse_form_params(req: &mut RequestCtx) -> HashMap<String, String> {
    let url = req.uri.to_string();
    let b = body_to_bytes(req.body.borrow_mut()).await;
    if b.is_err() {
        error!("解析form参数异常{}：{}", url, b.err().unwrap());
        return HashMap::new();
    }
    let params = form_urlencoded::parse(b.unwrap().as_ref())
        .into_owned()
        .collect::<HashMap<String, String>>();
    params
}
async fn handle_not_found(mut ctx: RequestCtx) -> anyhow::Result<Response<Body>> {
    Ok(ResponseBuilder::with_status(StatusCode::NOT_FOUND))
}
#[test]
fn test() {
    let now = Local::now();
    let str = now.format("%Y-%m-%d %H:%M:%S").to_string();

    print!("{}", str);
}
