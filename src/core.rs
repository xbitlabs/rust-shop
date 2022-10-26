pub mod extensions;
pub mod state;
pub mod utils;

use hyper::{Body, StatusCode};
use std::collections::HashMap;
use std::convert::Infallible;
use std::fmt::{Display, Formatter};
use std::future::Future;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Instant;

use route_recognizer::{Params, Router as MethodRouter};

use hyper::service::{make_service_fn, service_fn};
use log::{error, info};
use sqlx::{Acquire, MySql, Pool, Transaction};
use tokio::task_local;
use crate::extensions::Extensions;
use crate::RustShopResponseCode::{AccessDenied, ClientError, ServerError, SUCCESS, Unauthorized};
use crate::state::State;
use crate::utils::db::get_connection_pool;

macro_rules! register_method {
    ($method_name: ident, $method_def: expr) => {
        pub fn $method_name(&mut self, path: impl AsRef<str>, handler: impl HTTPHandler) {
            self.register($method_def, path, handler)
        }
    };
}

#[derive(Debug)]
pub struct RustShopError(String);

impl RustShopError {
    pub fn new(msg: impl ToString) -> Self {
        RustShopError(msg.to_string())
    }
}


impl From<sqlx::Error> for RustShopError {
    fn from(error: sqlx::Error) -> Self {
        RustShopError::new(error)
    }
}

impl From<serde_json::Error> for RustShopError {
    fn from(error: serde_json::Error) -> Self {
        RustShopError::new(error)
    }
}
impl From<hyper::Error> for RustShopError{
    fn from(error: hyper::Error) -> Self {
        RustShopError::new(error)
    }
}

impl Display for RustShopError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}", self)
    }
}

impl std::error::Error for RustShopError{

}

impl From<jsonwebtoken::errors::Error> for RustShopError{
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        RustShopError::new(error)
    }
}


pub struct RequestCtx {
    pub request: hyper::Request<Body>,
    pub router_params: Params,
    pub remote_addr: SocketAddr,
    pub query_params: HashMap<String, String>,
    pub request_state: State<MysqlPoolManager>,
}


#[derive(serde::Serialize, serde::Deserialize)]
pub enum RustShopResponseCode {
    SUCCESS,
    ServerError,
    ClientError,
    AccessDenied,
    Unauthorized
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RustShopResponse<T>
    where T:
    serde::Serialize
{
    code: RustShopResponseCode,
    msg:String,
    payload:Option<T>
}

impl <T:serde::Serialize> RustShopResponse<T> {
    pub fn new()-> RustShopResponse<T>{
        RustShopResponse {
            code:SUCCESS,
            msg:"".to_string(),
            payload:None
        }
    }
    pub fn set_code(&mut self,code: RustShopResponseCode){
        self.code = code;
    }
    pub fn set_msg(&mut self,msg:String){
        self.msg = msg;
    }
    pub fn set_payload(&mut self,payload:Option<T>){
        self.payload = payload;
    }
    pub fn ok(msg: String, payload:T) ->Self{
        RustShopResponse {
            code:SUCCESS,
            msg,
            payload:Some(payload)
        }
    }
    pub fn server_error(msg:String,payload:T)->Self{
        RustShopResponse {
            code:ServerError,
            msg,
            payload:Some(payload)
        }
    }
    pub fn client_error(msg:String,payload:T)->Self{
        RustShopResponse {
            code:ClientError,
            msg,
            payload:Some(payload)
        }
    }
    pub fn access_denied(msg:String,payload:T)->Self{
        RustShopResponse {
            code: AccessDenied,
            msg,
            payload:Some(payload)
        }
    }
    pub fn unauthorized(msg:String,payload:T)->Self{
        RustShopResponse {
            code: Unauthorized,
            msg,
            payload:Some(payload)
        }
    }
}

pub struct ResponseBuilder;

impl ResponseBuilder {
    pub fn with_text(text: String,code: RustShopResponseCode) -> hyper::Response<Body> {
        let mut rust_shop_response = RustShopResponse::new();
        rust_shop_response.set_msg(text);
        rust_shop_response.set_code(code);
        rust_shop_response.set_payload(Some(""));

        ResponseBuilder::with_rust_shop_response(&rust_shop_response)
    }
    pub fn with_text_and_payload<T>(text: String,payload:T,code: RustShopResponseCode) -> hyper::Response<Body>
        where T:
        serde::Serialize
    {
        let mut rust_shop_response = RustShopResponse::new();
        rust_shop_response.set_msg(text);
        rust_shop_response.set_code(code);
        rust_shop_response.set_payload(Some(payload));
        ResponseBuilder::with_rust_shop_response(&rust_shop_response)
    }

    pub fn with_rust_shop_response<T>(obj: &RustShopResponse<T>) -> hyper::Response<Body>
        where T:
        serde::Serialize
    {
        let json = serde_json::to_string(obj);
        let mut status = hyper::StatusCode::OK;
        match obj.code {
            ServerError=>{
                status = hyper::StatusCode::INTERNAL_SERVER_ERROR;
            }
            SUCCESS =>{
                status = hyper::StatusCode::OK;
            }
            ClientError=>{
                status = hyper::StatusCode::BAD_REQUEST
            }
            AccessDenied=>{
                status = hyper::StatusCode::FORBIDDEN
            }
            Unauthorized=>{
                status = hyper::StatusCode::UNAUTHORIZED
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
    pub fn with_status(status: StatusCode) -> hyper::Response<Body> {
        hyper::Response::builder()
            .status(status)
            .body(Body::empty())
            .unwrap()
    }
}

#[async_trait::async_trait]
pub trait HTTPHandler: Send + Sync + 'static {
    async fn handle(&self, ctx: RequestCtx) -> Result<hyper::Response<Body>,RustShopError>;
}

type BoxHTTPHandler = Box<dyn HTTPHandler>;

#[async_trait::async_trait]
impl<F: Send + Sync + 'static, Fut> HTTPHandler for F
where
    F: Fn(RequestCtx) -> Fut,
    Fut: Future<Output = Result<hyper::Response<Body>,RustShopError>> + Send + 'static,
{
    async fn handle(&self, ctx: RequestCtx) -> Result<hyper::Response<Body>,RustShopError> {
        self(ctx).await
    }
}

type Router = HashMap<String, MethodRouter<BoxHTTPHandler>>;

#[async_trait::async_trait]
pub trait Filter: Send + Sync + 'static {
    async fn handle<'a>(&'a self, ctx: RequestCtx, next: Next<'a>) -> Result<hyper::Response<Body>,RustShopError>;
}

#[allow(missing_debug_implementations)]
pub struct Next<'a> {
    pub(crate) endpoint: &'a dyn HTTPHandler,
    pub(crate) next_filter: &'a [Arc<dyn Filter>],
}

impl<'a> Next<'a> {
    /// Asynchronously execute the remaining filter chain.
    pub async fn run(mut self, ctx: RequestCtx) -> Result<hyper::Response<Body>,RustShopError> {
        if let Some((current, next)) = self.next_filter.split_first() {
            self.next_filter = next;
            current.handle(ctx, self).await
        } else {
            (self.endpoint).handle(ctx).await
        }
    }
}

pub struct AccessLogFilter;

#[async_trait::async_trait]
impl Filter for AccessLogFilter {
    async fn handle<'a>(&'a self, ctx: RequestCtx, next: Next<'a>) -> Result<hyper::Response<Body>,RustShopError> {
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

pub struct MysqlPoolManager{
    tran:Option<Transaction<'static,MySql>>,
    pool_state:&'static State<Pool<MySql>>
}

impl MysqlPoolManager {
    pub fn new(pool_state:&'static State<Pool<MySql>>)->Self{
        MysqlPoolManager{
            pool_state,
            tran:None
        }
    }
    pub fn get_pool(&self) -> &Pool<MySql> {
        self.pool_state.get_ref()
    }
    pub async fn begin(&mut self)->Result<&Transaction<'static,MySql>,RustShopError>{
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

pub struct Server {
    router: Router,
    filters: Vec<Arc<dyn Filter>>,
    extensions: Extensions,
}

impl Server {
    pub fn new() -> Self {
        Server {
            router: HashMap::new(),
            filters: Vec::new(),
            extensions: Extensions::new(),
        }
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
    pub fn app_data<U: 'static>(&mut self, ext: U)  {
        self.extensions.insert(ext);
    }
    pub async fn run(self, addr: SocketAddr) -> Result<(), RustShopError> {
        let Self {
            router,
            filters,
            extensions
        } = self;

        let router = Arc::new(router);
        let filters = Arc::new(filters);

        let make_svc = make_service_fn(|conn: &hyper::server::conn::AddrStream| {
            let router = router.clone();
            let filters = filters.clone();
            let remote_addr = conn.remote_addr();

            async move {
                Ok::<_, Infallible>(service_fn(move |req: hyper::Request<Body>| {
                    let router = router.clone();
                    let filters = filters.clone();

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
                            next_filter: &filters,
                        };

                        let mut query_params = HashMap::new();

                        if let Some(q) = req.uri().query() {
                            query_params = form_urlencoded::parse(q.as_bytes())
                                .into_owned()
                                .collect::<HashMap<String, String>>();
                        };
                        let url = req.uri().to_string();
                        let request_state = Extensions::new();
                        //request_state.insert(State::new())
                        let pool = get_connection_pool().await?;
                        let ctx = RequestCtx {
                            request: req,
                            router_params,
                            remote_addr,
                            query_params,
                            request_state : State::new(MysqlPoolManager::new(&pool))
                        };
                        let resp_result = next.run(ctx).await;
                        match resp_result {
                            Ok(resp)=>{
                                Ok::<_, RustShopError>(resp)
                            }
                            Err(error)=>{
                                error!("处理请求异常{}：{}",url, error.0);
                                let rust_shop_response = RustShopResponse::server_error("内部服务器错误".to_string(),"");
                                Ok::<_, RustShopError>(ResponseBuilder::with_rust_shop_response(&rust_shop_response))
                            }
                        }

                    }
                }))
            }
        });

        let server = hyper::Server::bind(&addr).serve(make_svc);

        server
            .await
            .map_err(|e| RustShopError::new(format!("server run error: {:?}", e)))?;

        Ok(())
    }

    async fn handle_not_found(_ctx: RequestCtx) -> Result<hyper::Response<Body>,RustShopError> {
        Ok(ResponseBuilder::with_status(hyper::StatusCode::NOT_FOUND))
    }
    /*    async fn parse_form_params(body: Body) ->HashMap<String,String>{
        let b = hyper::body::to_bytes(body).await;
        match b {
            Ok(bytes)=>{
                form_urlencoded::parse(bytes.as_ref())
                    .into_owned()
                    .collect::<HashMap<String, String>>()
            }
            Err(e)=>{
                HashMap::new()
            }
        }

    }*/
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}
