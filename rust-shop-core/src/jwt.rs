use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

pub mod jwt_numeric_date {
    //! Custom serialization of OffsetDateTime to conform with the JWT spec (RFC 7519 section 2, "Numeric Date")
    use serde::{self, Deserialize, Deserializer, Serialize, Serializer};
    use time::OffsetDateTime;

    /// Serializes an OffsetDateTime to a Unix timestamp (milliseconds since 1970/1/1T00:00:00T)
    pub fn serialize<S>(date: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let timestamp = date.unix_timestamp();
        serializer.serialize_i64(timestamp)
    }

    /// Attempts to deserialize an i64 and use as a Unix timestamp
    pub fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        OffsetDateTime::from_unix_timestamp(i64::deserialize(deserializer)?)
            .map_err(|_| serde::de::Error::custom("invalid Unix timestamp value"))
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Claims {
    pub token_id: String,
    //用户标识
    pub user_id: i64,
    pub sub: String,
    ///token颁发时间
    #[serde(with = "jwt_numeric_date")]
    pub iat: OffsetDateTime,
    ///失效时间
    #[serde(with = "jwt_numeric_date")]
    pub exp: OffsetDateTime,
}

impl Claims {
    pub fn new(
        token_id: String,
        user_id: i64,
        sub: String,
        iat: OffsetDateTime,
        exp: OffsetDateTime,
    ) -> Self {
        Claims {
            token_id,
            user_id,
            sub,
            iat,
            exp,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessToken {
    pub access_token: String,
    pub refresh_token: String,
    pub exp: i64,
}

#[async_trait::async_trait]
pub trait JwtService {
    async fn grant_access_token(&mut self, user_id: i64) -> anyhow::Result<AccessToken>;
    async fn decode_access_token(&self, access_token: String) -> anyhow::Result<Claims>;
    async fn decode_refresh_token(&self, refresh_token: String) -> anyhow::Result<Claims>;
    async fn refresh_token(&mut self, refresh_token: String) -> anyhow::Result<AccessToken>;
    async fn remove_access_token(&mut self, access_token: String) -> anyhow::Result<bool>;
}
