use async_trait::async_trait;

use hyper::{Body, StatusCode};
use log::error;
use thiserror::Error;

use crate::response::into_response::IntoResponse;
use crate::response::Response;
use crate::RequestCtx;

pub mod cookie;
pub mod extension;
pub mod form;
pub mod header;
pub mod json;
pub mod multipart;
pub mod path_variable;
pub mod query;
pub mod request_param;
pub mod request_state;

#[async_trait]
pub trait FromRequest: Sized {
    /// If the extractor fails it'll use this "rejection" type. A rejection is
    /// a kind of error that can be converted into a response.
    type Rejection: IntoResponse;
    /// Perform the extraction.
    async fn from_request(req: &mut RequestCtx) -> anyhow::Result<Self, Self::Rejection>;
}

#[derive(Error, Debug)]
pub enum ExtractError {
    #[error("无效的form content-type")]
    InvalidFormContentType,
    #[error("反系列化Url请求参数为对象失败：{0}")]
    FailedToDeserializeQueryString(String),
    #[error("反系列化Form参数为对象失败：{0}")]
    FailedToDeserializeFormData(String),
    #[error("未发现Json数据")]
    MissingJsonContentType,
    #[error("Json IO错误：{0}")]
    JsonIoError(String),
    #[error("Json对象类型映射错误：{0}")]
    JsonDataError(String),
    #[error("Json格式无效")]
    JsonSyntaxError,
    #[error("未知错误")]
    Unknown,
}

impl IntoResponse for ExtractError {
    fn into_response(self) -> Response {
        let mut builder = Response::builder().status(StatusCode::BAD_REQUEST);
        error!("提取请求参数异常：{}",self);
        builder
            .body(Body::from("无效请求"))
            .unwrap()
            .into_response()
    }
}

