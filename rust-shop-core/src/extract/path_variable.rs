use serde::de::DeserializeOwned;
use serde::Deserialize;

use crate::extract::ExtractError::FailedToDeserializeQueryString;
use crate::extract::{ExtractError, FromRequest};
use crate::RequestCtx;

#[derive(Debug, Clone, Copy, Default)]
pub struct PathVariable<T>(pub T);
