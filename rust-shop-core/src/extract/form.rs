use std::ops::Deref;
use hyper::body::{Bytes, HttpBody};
use hyper::{Body, Error, header, Method, Request};
use serde::de::DeserializeOwned;
use crate::{BoxError, RequestCtx};
use crate::extract::{body_to_bytes, ExtractError, FromRequest};

#[derive(Debug, Clone, Copy, Default)]
pub struct Form<T>(pub T);

impl<T> Deref for Form<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[async_trait::async_trait]
impl<T, B> FromRequest<B> for Form<T>
    where
        T: DeserializeOwned,
        B: HttpBody + Send,
        B::Data: Send,
        B::Error: Into<BoxError>,
{
    type Rejection = ExtractError;

    async fn from_request(ctx:RequestCtx) -> anyhow::Result<Form<T>,ExtractError> {
        if ctx.request.method() == Method::GET {
            let query = ctx.request.uri().query().unwrap_or_default();
            let value = serde_html_form::from_str(query)
                .map_err(|_|ExtractError::FailedToDeserializeQueryString)?;
            Ok(Form(value))
        } else {
            if !has_content_type(&ctx, &mime::APPLICATION_WWW_FORM_URLENCODED) {
                return Err(ExtractError::InvalidFormContentType);
            }

            let bytes =  body_to_bytes(ctx.request).await;
            if bytes.is_err() {
                return Err(ExtractError::FailedToDeserializeFormData);
            }
            let bytes = bytes.unwrap();
            let value = serde_html_form::from_bytes(&bytes)
                .map_err(|_|ExtractError::FailedToDeserializeFormData)?;

            Ok(Form(value))
        }
    }
}

// this is duplicated in `axum/src/extract/mod.rs`
fn has_content_type(ctx:&RequestCtx, expected_content_type: &mime::Mime) -> bool {
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

    content_type.starts_with(expected_content_type.as_ref())
}