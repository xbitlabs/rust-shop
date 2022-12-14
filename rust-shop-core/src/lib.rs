extern crate core;

use std::any::Any;
use std::borrow::BorrowMut;

use std::collections::HashMap;
use std::convert::Infallible;

use std::future::Future;

use std::net::SocketAddr;

use std::sync::Arc;
use std::time::Instant;

use anyhow::anyhow;

use chrono::Local;

use http::{header, Method, Uri, Version};
use hyper::body::Buf;
use hyper::header::HeaderValue;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, HeaderMap, Request, StatusCode};

use log::{error, info};
use route_recognizer::Params;

use serde::Serialize;

use crate::extensions::Extensions;

use crate::router::{get_routers, register_route, Router};
use crate::security::{Authentication, DefaultAuthentication, req_matches};
use crate::security::{AuthenticationProcessingFilter, WebSecurityConfigurer};
use crate::state::State;
use crate::EndpointResultCode::{AccessDenied, ClientError, ServerError, SUCCESS, Unauthorized};

pub mod app_config;
mod application_context;
mod dispatcher;
pub mod entity;
pub mod extensions;
pub mod extract;
pub mod handler_interceptor;
pub mod id_generator;
pub mod jwt;
pub mod memory_cache;
pub mod mode_and_view;
pub mod redis;
pub mod router;
pub mod security;
pub mod serde_utils;
pub mod session;
pub mod state;
pub mod wechat;

#[macro_use]
pub(crate) mod macros;

pub mod body;
pub mod response;

pub mod error;
pub mod db;

pub use self::error::Error;

use crate::application_context::APPLICATION_CONTEXT;
use crate::extract::cookie::CookieJar;
use crate::extract::json::body_to_bytes;
use crate::extract::FromRequest;
use crate::response::into_response::IntoResponse;
use crate::response::Response;
use crate::session::{DefaultSession, DefaultSessionManager, Session, SessionManager};
use futures::executor::block_on;
use futures_util::future::BoxFuture;
use futures_util::FutureExt;
use http_body::Empty;
use once_cell::sync::Lazy;

pub static mut APP_EXTENSIONS: Lazy<Extensions> = Lazy::new(|| {
    let mut extensions: Extensions = Extensions::new();
    extensions
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
    pub headers: HeaderMap,
    //pub request_states: Extensions,
    //pub current_user: Option<Box<dyn UserDetails + Send + Sync>>,
    pub method: Method,
    pub uri: Uri,
    pub version: Version,
    pub extensions: http::Extensions,
    pub authentication: Box<(dyn Authentication + Sync + Send)>,
    pub cookies: CookieJar,
    pub session: DefaultSession,
}

impl RequestCtx {
    pub fn extensions_mut(&mut self) -> &mut http::Extensions {
        self.extensions.borrow_mut()
    }
    pub fn header_mut(&mut self)->&mut HeaderMap{
        self.headers.borrow_mut()
    }
    pub fn set_authentication(&mut self, authentication: Box<(dyn Authentication + Sync + Send)>) {
        self.authentication = authentication;
    }
}

#[async_trait::async_trait]
impl Drop for RequestCtx {
    fn drop(&mut self) {
        unsafe {
            block_on(APPLICATION_CONTEXT.session_manager.save_session(self));
            println!("????????????");
        }
    }
}
impl Into<Request<Body>> for &mut RequestCtx{
    fn into(mut self) -> Request<Body> {
        let method = self.method.clone();
        let mut req = Request::new(Body::empty());
        *req.method_mut() = method;
        *req.version_mut() = self.version;
        *req.uri_mut() = self.uri.clone();
        *req.headers_mut() = std::mem::take(self.header_mut());
        *req.extensions_mut() = std::mem::take(self.extensions_mut());
        req
    }
}
//??????http???????????????????????????
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
    NotFound
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

impl <T> IntoResponse for EndpointResult<T> where T:Serialize {
    fn into_response(self) -> Response {
        ResponseBuilder::with_endpoint_result(self)
    }
}

pub struct ResponseBuilder;

impl ResponseBuilder {
    pub fn with_msg(msg: &'static str, code: EndpointResultCode) -> Response {
        let mut endpoint_result = EndpointResult::new();
        endpoint_result.set_msg(msg);
        endpoint_result.set_code(code);
        endpoint_result.set_payload(Some(""));

        ResponseBuilder::with_endpoint_result(endpoint_result).into_response()
    }
    pub fn with_msg_and_payload<T>(
        msg: &'static str,
        payload: T,
        code: EndpointResultCode,
    ) -> Response
    where
        T: serde::Serialize,
    {
        let mut endpoint_result = EndpointResult::new();
        endpoint_result.set_msg(msg);
        endpoint_result.set_code(code);
        endpoint_result.set_payload(Some(payload));
        ResponseBuilder::with_endpoint_result(endpoint_result).into_response()
    }

    pub fn with_endpoint_result<T>(endpoint_result: EndpointResult<T>) -> Response
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
            NotFound=> status = StatusCode::NOT_FOUND
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
            .into_response()
    }
    pub fn with_status(status: StatusCode) -> Response {
        hyper::Response::builder()
            .status(status)
            .body(Body::empty())
            .unwrap()
            .into_response()
    }
    pub fn with_status_and_html(status: StatusCode, html: String) -> Response {
        hyper::Response::builder()
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_static(mime::TEXT_HTML.as_ref()),
            )
            .status(status)
            .body(Body::from(html))
            .unwrap()
            .into_response()
    }
}

#[async_trait::async_trait]
pub trait HTTPHandler: Send + Sync + 'static {
    async fn handle(&self, mut ctx: RequestCtx) -> anyhow::Result<Response>;
}

type BoxHTTPHandler = Box<dyn HTTPHandler>;

#[async_trait::async_trait]
impl<F: Send + Sync + 'static, Fut> HTTPHandler for F
where
    F: Fn(RequestCtx) -> Fut,
    Fut: Future<Output = anyhow::Result<Response>> + Send + 'static,
{
    async fn handle(&self, mut ctx: RequestCtx) -> anyhow::Result<Response> {
        self(ctx).await
    }
}

#[async_trait::async_trait]
pub trait Filter: Send + Sync + 'static {
    async fn handle<'a>(&'a self, mut ctx: RequestCtx, next: Next<'a>) -> anyhow::Result<Response>;
    fn url_patterns(&self) -> &'static str;
    fn order(&self) -> u64;
    fn name(&self) -> &'static str;
}

#[allow(missing_debug_implementations)]
pub struct Next<'a> {
    pub(crate) endpoint: &'a dyn HTTPHandler,
    pub(crate) next_filter: &'a [Arc<dyn Filter>],
}

impl<'a> Next<'a> {
    /// Asynchronously execute the remaining filter chain.
    pub async fn run(mut self, mut ctx: RequestCtx) -> anyhow::Result<Response> {
        if let Some((current, next)) = self.next_filter.split_first() {
            if req_matches(&ctx, &current.url_patterns()) {
                info!(
                    "filter?????????`{}`???????????????`{}`??????????????????filter???{}",
                    current.url_patterns(),
                    ctx.uri,
                    current.name()
                );
                self.next_filter = next;
                current.handle(ctx, self).await
            } else {
                info!(
                    "filter?????????`{}`???????????????`{}`????????????filter `{}`????????????",
                    current.url_patterns(),
                    ctx.uri,
                    current.name()
                );
                self.skip_current_and_exec_next(next, ctx).await
            }
        } else {
            (self.endpoint).handle(ctx).await
        }
    }
    fn skip_current_and_exec_next(
        mut self,
        chain: &'a [Arc<dyn Filter>],
        mut ctx: RequestCtx,
    ) -> BoxFuture<anyhow::Result<Response>> {
        async move {
            if let Some((current, next)) = chain.split_first() {
                if req_matches(&ctx, &current.url_patterns()) {
                    info!(
                        "filter?????????`{}`???????????????`{}`??????????????????filter???{}",
                        current.url_patterns(),
                        ctx.uri,
                        current.name()
                    );
                    self.next_filter = next;
                    current.handle(ctx, self).await
                } else {
                    info!(
                        "filter?????????`{}`???????????????`{}`????????????filter `{}`????????????",
                        current.url_patterns(),
                        ctx.uri,
                        current.name()
                    );
                    self.skip_current_and_exec_next(next, ctx).await
                }
            } else {
                (self.endpoint).handle(ctx).await
            }
        }
        .boxed()
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
    async fn handle<'a>(&'a self, mut ctx: RequestCtx, next: Next<'a>) -> anyhow::Result<Response> {
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

    fn url_patterns(&self) -> &'static str {
        "/*"
    }

    fn order(&self) -> u64 {
        todo!()
    }

    fn name(&self) -> &'static str {
        "AccessLogFilter"
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
    pub fn security_config(&mut self, security_config: WebSecurityConfigurer) {
        self.enable_security = security_config.is_enable_security();
        unsafe {
            APP_EXTENSIONS.insert(security_config);
        }
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
        let request_state_providers = Arc::new(request_state_providers);

        let make_svc = make_service_fn(|conn: &hyper::server::conn::AddrStream| {
            let remote_addr = conn.remote_addr();

            let router = router.clone();
            let filters = filters.clone();
            let request_state_providers = request_state_providers.clone();

            async move {
                Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
                    let router = router.clone();
                    let filters = filters.clone();
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
                        let (
                            http::request::Parts {
                                method,
                                uri,
                                version,
                                extensions,
                                headers,
                                ..
                            },
                            body,
                        ) = req.into_parts();

                        let mut ctx = RequestCtx {
                            router_params,
                            remote_addr,
                            query_params,
                            headers,
                            method,
                            uri,
                            version,
                            body,
                            extensions,
                            authentication: Box::new(DefaultAuthentication::default()),
                            cookies: CookieJar::default(),
                            session: DefaultSession::default(),
                        };
                        let cookies = CookieJar::from_request(&mut ctx).await?;
                        ctx.cookies = cookies;

                        let mut is_new_session = false;
                        let mut session_id = String::from("session_id=");
                        unsafe {
                            let mut session = APPLICATION_CONTEXT
                                .session_manager
                                .session_for_request(&ctx)
                                .await;
                            session.set_last_activity(Local::now().timestamp_millis());
                            is_new_session = session.is_new();
                            session_id = session_id
                                + session.get_session_id().to_string().as_str()
                                + "; Path=/; Max-Age=2592000";
                            ctx.session = session;
                        }

                        let resp_result = next.run(ctx).await;
                        /*unsafe {
                            APPLICATION_CONTEXT.session_manager.save_session(&mut ctx);
                        }*/
                        match resp_result {
                            Ok(mut resp) => {
                                if is_new_session {
                                    let session = session_id.parse().unwrap();
                                    resp.headers_mut().insert("Set-Cookie", session);
                                }
                                Ok::<_, anyhow::Error>(resp)
                            }
                            Err(error) => {
                                let mut error_details = String::from(format!("Error: {}\r\n", error));

                                let mut cause = error.source();
                                while let Some(e) = cause {
                                    error_details = error_details + &*format!("Reason: {}\r\n", e);
                                    cause = e.source();
                                }
                                error!("??????????????????{}???{}", url, error_details);

                                let endpoint_result: EndpointResult<String> =
                                    EndpointResult::server_error("?????????????????????");
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
        Self::new()
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
        error!("??????form????????????{}???{}", url, b.err().unwrap());
        return HashMap::new();
    }
    let params = form_urlencoded::parse(b.unwrap().as_ref())
        .into_owned()
        .collect::<HashMap<String, String>>();
    params
}
async fn handle_not_found(mut ctx: RequestCtx) -> anyhow::Result<Response> {
    Ok(ResponseBuilder::with_status(StatusCode::NOT_FOUND))
}
#[test]
fn test() {
    let now = Local::now();
    let str = now.format("%Y-%m-%d %H:%M:%S").to_string();

    print!("{}", str);
}
