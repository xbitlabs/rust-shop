use std::ops::{Deref, DerefMut};
use async_trait::async_trait;
use hyper::body::{Bytes, HttpBody};
use hyper::{Body, header, Response, StatusCode};
use hyper::header::HeaderValue;
use log::error;
use multer::bytes::{BufMut, BytesMut};
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::{BoxError, EndpointResult, RequestCtx, ResponseBuilder};
use crate::extract::{body_to_bytes, ExtractError, FromRequest, IntoResponse};
use crate::extract::ExtractError::{JsonDataError, JsonIoError, JsonSyntaxError, MissingJsonContentType};

#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(docsrs, doc(cfg(feature = "json")))]
pub struct Json<T>(pub T);

#[async_trait]
impl<T, B> FromRequest<B> for Json<T>
    where
        T: DeserializeOwned,
        B: HttpBody + Send,
        B::Data: Send,
        B::Error: Into<BoxError>,
{
    type Rejection = ExtractError;

    async fn from_request(ctx:RequestCtx) -> Result<Self, ExtractError> {
        if json_content_type(&ctx) {
            let bytes = body_to_bytes(ctx.request).await;
            if bytes.is_err() {
                return Err(JsonIoError)
            }
            let bytes = bytes.unwrap();
            let value = match serde_json::from_slice(&bytes) {
                Ok(value) => value,
                Err(err) => {
                    let rejection = match err.classify() {
                        serde_json::error::Category::Data => JsonDataError,
                        serde_json::error::Category::Syntax | serde_json::error::Category::Eof => {
                            JsonSyntaxError
                        }
                        serde_json::error::Category::Io => {
                            if cfg!(debug_assertions) {
                                // we don't use `serde_json::from_reader` and instead always buffer
                                // bodies first, so we shouldn't encounter any IO errors
                                unreachable!()
                            } else {
                                JsonSyntaxError
                            }
                        }
                    };
                    return Err(rejection);
                }
            };

            Ok(Json(value))
        } else {
            Err(MissingJsonContentType)
        }
    }
}

fn json_content_type(ctx:&RequestCtx) -> bool {
    let content_type = if let Some(content_type) = ctx.request.headers().get(header::CONTENT_TYPE) {
        content_type
    } else {
        return false;
    };

    let content_type = if let Ok(content_type) = content_type.to_str() {
        content_type
    } else {
        return false;
    };

    let mime = if let Ok(mime) = content_type.parse::<mime::Mime>() {
        mime
    } else {
        return false;
    };

    let is_json_content_type = mime.type_() == "application"
        && (mime.subtype() == "json" || mime.suffix().map_or(false, |name| name == "json"));

    is_json_content_type
}

impl<T> Deref for Json<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Json<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<T> for Json<T> {
    fn from(inner: T) -> Self {
        Self(inner)
    }
}

impl<T> IntoResponse for Json<T>
    where
        T: Serialize,
{
    fn into_response(self) -> Response<Body> {
        let mut buf = BytesMut::new().writer();
        match serde_json::to_writer(&mut buf, &self.0) {
            Ok(()) => {
                let mut builder = Response::builder()
                    .header(header::CONTENT_TYPE,HeaderValue::from_static(mime::APPLICATION_JSON.as_ref()))
                    .status(StatusCode::OK);
                builder.body(Body::from(buf.into_inner().freeze())).unwrap()
            }
            ,
            Err(err) => {
                error!("转换json数据为对象时异常：{}",err);
                let result : EndpointResult<String> = EndpointResult::client_error_with_payload("无效的JSON数据",String::from(""));
                ResponseBuilder::with_endpoint_result(result)
            }
        }
    }
}