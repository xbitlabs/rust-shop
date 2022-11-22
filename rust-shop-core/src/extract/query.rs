use serde::de::DeserializeOwned;
use serde::Deserialize;

use crate::extract::ExtractError::FailedToDeserializeQueryString;
use crate::extract::{ExtractError, FromRequest};
use crate::RequestCtx;

#[derive(Debug, Clone, Copy, Default)]
pub struct Query<T>(pub T);

#[async_trait::async_trait]
impl<T> FromRequest for Query<T>
where
    T: for<'a> Deserialize<'a>,
{
    type Rejection = ExtractError;

    async fn from_request(ctx:&mut RequestCtx) -> Result<Self, ExtractError> {
        let query = ctx.parts.uri.query().unwrap_or_default();
        let value = serde_html_form::from_str(query).map_err(|_| FailedToDeserializeQueryString)?;
        Ok(Query(value))
    }
}
