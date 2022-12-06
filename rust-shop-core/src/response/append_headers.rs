use crate::response::into_response::IntoResponse;
use crate::response::into_response_parts::{IntoResponseParts, ResponseParts, TryIntoHeaderError};
use crate::response::Response;
use http::header::HeaderName;
use http::HeaderValue;

use std::fmt;

#[derive(Debug)]
pub struct AppendHeaders<K, V, const N: usize>(pub [(K, V); N]);

impl<K, V, const N: usize> IntoResponse for AppendHeaders<K, V, N>
where
    K: TryInto<HeaderName>,
    K::Error: fmt::Display,
    V: TryInto<HeaderValue>,
    V::Error: fmt::Display,
{
    fn into_response(self) -> Response {
        (self, ()).into_response()
    }
}

impl<K, V, const N: usize> IntoResponseParts for AppendHeaders<K, V, N>
where
    K: TryInto<HeaderName>,
    K::Error: fmt::Display,
    V: TryInto<HeaderValue>,
    V::Error: fmt::Display,
{
    type Error = TryIntoHeaderError<K::Error, V::Error>;

    fn into_response_parts(self, mut res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        for (key, value) in self.0 {
            let key = key.try_into().map_err(TryIntoHeaderError::key)?;
            let value = value.try_into().map_err(TryIntoHeaderError::value)?;
            res.headers_mut().append(key, value);
        }

        Ok(res)
    }
}
