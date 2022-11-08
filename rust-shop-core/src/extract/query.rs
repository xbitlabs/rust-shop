use serde::de::DeserializeOwned;
use crate::extract::ExtractError::FailedToDeserializeQueryString;
use crate::extract::{ExtractError, FromRequest};
use crate::RequestCtx;

#[derive(Debug, Clone, Copy, Default)]
pub struct Query<T>(pub T);

#[async_trait::async_trait]
impl<T, B> FromRequest<B> for Query<T>
    where
        T: DeserializeOwned,
        B: Send,
{
    type Rejection = ExtractError;

    async fn from_request(ctx:RequestCtx) -> Result<Self, ExtractError> {
        let query = ctx.request.uri().query().unwrap_or_default();
        let value = serde_html_form::from_str(query)
            .map_err(|_|FailedToDeserializeQueryString)?;
        Ok(Query(value))
    }
}