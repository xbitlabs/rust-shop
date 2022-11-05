use std::ops::Deref;
use hyper::body::HttpBody;
use hyper::Method;
use serde::de::DeserializeOwned;
use crate::RequestCtx;

#[derive(Debug, Clone, Copy, Default)]
pub struct Form<T>(pub T);

impl<T> Deref for Form<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[async_trait]
impl<T, B> FromRequest<B> for Form<T>
    where
        T: DeserializeOwned,
        B: HttpBody + Send,
        B::Data: Send,
        B::Error: Into<BoxError>,
{
    type Rejection = FormRejection;

    async fn from_request(req: &mut RequestCtx) -> Result<Self, Self::Rejection> {
        if req.method() == Method::GET {
            let query = req.request.uri().query().unwrap_or_default();
            let value = serde_html_form::from_str(query)
                .map_err(FailedToDeserializeQueryString::__private_new::<(), _>)?;
            Ok(Form(value))
        } else {
            if !has_content_type(req, &mime::APPLICATION_WWW_FORM_URLENCODED) {
                return Err(InvalidFormContentType::default().into());
            }

            let bytes = Bytes::from_request(req).await?;
            let value = serde_html_form::from_bytes(&bytes)
                .map_err(FailedToDeserializeQueryString::__private_new::<(), _>)?;

            Ok(Form(value))
        }
    }
}

// this is duplicated in `axum/src/extract/mod.rs`
fn has_content_type<B>(req: &RequestParts<B>, expected_content_type: &mime::Mime) -> bool {
    let content_type = if let Some(content_type) = req.headers().get(header::CONTENT_TYPE) {
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