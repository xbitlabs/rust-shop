pub mod form;
pub mod json;
pub mod query;

use std::io;
use anyhow::anyhow;
use async_trait::async_trait;
use hyper::{Body, Error, Request, Response, StatusCode};
use hyper::body::Bytes;
use log::{error, log};
use thiserror::Error;
use crate::RequestCtx;

pub trait IntoResponse {
    /// Create a response.
    fn into_response(self) -> Response<Body>;
}

#[async_trait]
pub trait FromRequest<B>: Sized {
    /// If the extractor fails it'll use this "rejection" type. A rejection is
    /// a kind of error that can be converted into a response.
    type Rejection: IntoResponse;
    /// Perform the extraction.
    async fn from_request(req:RequestCtx) -> anyhow::Result<Self, Self::Rejection>;
}

#[derive(Error, Debug)]
pub enum ExtractError {
    #[error("无效的form content-type")]
    InvalidFormContentType,
    #[error("反系列化Url请求参数为对象失败")]
    FailedToDeserializeQueryString,
    #[error("反系列化Form参数为对象失败")]
    FailedToDeserializeFormData,
    #[error("未发现Json数据")]
    MissingJsonContentType,
    #[error("Json IO错误")]
    JsonIoError,
    #[error("Json对象类型映射错误")]
    JsonDataError,
    #[error("Json格式无效")]
    JsonSyntaxError,
    #[error("未知错误")]
    Unknown,
}

impl IntoResponse for ExtractError {
    fn into_response(self) -> Response<Body> {
        let mut builder = Response::builder()
            .status(StatusCode::BAD_REQUEST);
        error!("转换url参数/form表单为对象时异常");
        builder.body(Body::from("无效请求")).unwrap()
    }
}
async fn body_to_bytes(req:Request<Body>)->Result<Bytes,Error>{
    hyper::body::to_bytes(req).await
}