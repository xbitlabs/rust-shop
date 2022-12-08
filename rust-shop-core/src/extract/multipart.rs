use std::borrow::BorrowMut;
use std::convert::Infallible;
use std::fmt;
use std::pin::Pin;
use std::task::{Context, Poll};
use async_trait::async_trait;
use bytes::Bytes;
use futures::Stream;
use http::header::CONTENT_TYPE;
use http::{HeaderMap, StatusCode};
use hyper::Body;
use log::error;
use crate::extract::FromRequest;
use crate::extract::json::body_to_bytes;
use crate::RequestCtx;
use crate::response::into_response::IntoResponse;
use crate::response::Response;

#[derive(Debug)]
pub struct Multipart {
    inner: multer::Multipart<'static>,
}

#[async_trait]
impl FromRequest for Multipart
{
    type Rejection = MultipartError;

    async fn from_request(req: &mut RequestCtx) -> anyhow::Result<Self, Self::Rejection> {
        let boundary = parse_boundary(&req.headers).ok_or(MultipartError::from_multer(multer::Error::NoBoundary))?;
        let bytes = body_to_bytes(req.body.borrow_mut()).await;
        if bytes.is_err() {
            return Err(MultipartError::from_multer(multer::Error::StreamReadFailed(Box::new(bytes.err().unwrap()))));
        }
        let bytes = bytes.unwrap();
        let stream =
            futures_util::stream::once(async move { Result::<Bytes, Infallible>::Ok(bytes) });
        let multipart = multer::Multipart::new(stream, boundary);
        Ok(Self { inner: multipart })
    }
}

impl Multipart {
    /// Yields the next [`Field`] if available.
    pub async fn next_field(&mut self) -> Result<Option<Field<'_>>, MultipartError> {
        let field = self
            .inner
            .next_field()
            .await
            .map_err(MultipartError::from_multer)?;

        if let Some(field) = field {
            Ok(Some(Field {
                inner: field,
                _multipart: self,
            }))
        } else {
            Ok(None)
        }
    }
}

/// A single field in a multipart stream.
#[derive(Debug)]
pub struct Field<'a> {
    inner: multer::Field<'static>,
    // multer requires there to only be one live `multer::Field` at any point. This enforces that
    // statically, which multer does not do, it returns an error instead.
    _multipart: &'a mut Multipart,
}

impl<'a> Stream for Field<'a> {
    type Item = Result<Bytes, MultipartError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.inner)
            .poll_next(cx)
            .map_err(MultipartError::from_multer)
    }
}

impl<'a> Field<'a> {
    /// The field name found in the
    /// [`Content-Disposition`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Disposition)
    /// header.
    pub fn name(&self) -> Option<&str> {
        self.inner.name()
    }

    /// The file name found in the
    /// [`Content-Disposition`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Disposition)
    /// header.
    pub fn file_name(&self) -> Option<&str> {
        self.inner.file_name()
    }

    /// Get the [content type](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Type) of the field.
    pub fn content_type(&self) -> Option<&str> {
        self.inner.content_type().map(|m| m.as_ref())
    }

    /// Get a map of headers as [`HeaderMap`].
    pub fn headers(&self) -> &HeaderMap {
        self.inner.headers()
    }

    /// Get the full data of the field as [`Bytes`].
    pub async fn bytes(self) -> Result<Bytes, MultipartError> {
        self.inner
            .bytes()
            .await
            .map_err(MultipartError::from_multer)
    }

    /// Get the full field data as text.
    pub async fn text(self) -> Result<String, MultipartError> {
        self.inner.text().await.map_err(MultipartError::from_multer)
    }

    pub async fn chunk(&mut self) -> Result<Option<Bytes>, MultipartError> {
        self.inner
            .chunk()
            .await
            .map_err(MultipartError::from_multer)
    }
}

/// Errors associated with parsing `multipart/form-data` requests.
#[derive(Debug)]
pub struct MultipartError {
    source: multer::Error,
}

impl MultipartError {
    fn from_multer(multer: multer::Error) -> Self {
        Self { source: multer }
    }
}

impl fmt::Display for MultipartError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error parsing `multipart/form-data` request")
    }
}

impl std::error::Error for MultipartError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

fn parse_boundary(headers: &HeaderMap) -> Option<String> {
    let content_type = headers.get(CONTENT_TYPE)?.to_str().ok()?;
    multer::parse_boundary(content_type).ok()
}

impl IntoResponse for MultipartError{
    fn into_response(self) -> Response {
        let mut builder = Response::builder().status(StatusCode::BAD_REQUEST);
        error!("文件上传异常：{}",self);
        builder
            .body(Body::from(format!("无效请求：{}",self)))
            .unwrap()
            .into_response()
    }
}