
use std::borrow::BorrowMut;
use std::ops::Deref;
use anyhow::anyhow;

use hyper::body::{Bytes, HttpBody};
use hyper::{header, Body, Error, Method, Request};
use serde::de::DeserializeOwned;
use serde::Deserialize;

use crate::extract::{ ExtractError, FromRequest};
use crate::{BoxError, RequestCtx};
use crate::extract::json::body_to_bytes;


#[derive(Debug, Clone, Copy, Default)]
pub struct Form<T>(pub T);

impl<T> Deref for Form<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[async_trait::async_trait]
impl<T> FromRequest for Form<T>
where
    T: for<'a> Deserialize<'a>,
{
    type Rejection = ExtractError;

    async fn from_request(ctx:&mut RequestCtx) -> anyhow::Result<Form<T>, ExtractError> {
        if ctx.method == Method::GET {
            let query = ctx.uri.query().unwrap_or_default();
            let value = serde_html_form::from_str(query)
                .map_err(|err| ExtractError::FailedToDeserializeQueryString(err.to_string()))?;
            Ok(Form(value))
        } else {
            if !has_content_type(&ctx, &mime::APPLICATION_WWW_FORM_URLENCODED) {
                return Err(ExtractError::InvalidFormContentType);
            }

            let bytes = body_to_bytes(ctx.body.borrow_mut()).await;
            if bytes.is_err() {
                return Err(ExtractError::FailedToDeserializeFormData(bytes.err().unwrap().to_string()));
            }
            let bytes = bytes.unwrap();
            let value = serde_html_form::from_bytes(&bytes);
            if value.is_ok() {
                Ok(Form(value.unwrap()))
            }else {
                Err(ExtractError::FailedToDeserializeFormData(value.err().unwrap().to_string()))
            }

        }
    }
}

// this is duplicated in `axum/src/extract/mod.rs`
fn has_content_type(ctx: &RequestCtx, expected_content_type: &mime::Mime) -> bool {
    let content_type = ctx.headers.get(header::CONTENT_TYPE.as_str());
    if content_type.is_none() {
        return false;
    }
    let content_type = content_type.as_ref().unwrap();
    if content_type.is_none() {
        return false;
    }
    let content_type = content_type.as_ref().unwrap();
    content_type.starts_with(expected_content_type.as_ref())
}
