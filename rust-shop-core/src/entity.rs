use chrono::NaiveDateTime;

pub mod db_numeric_date {
    //! Custom serialization of OffsetDateTime to conform with the JWT spec (RFC 7519 section 2, "Numeric Date")
    use chrono::NaiveDateTime;
    use serde::{self, Deserialize, Deserializer, Serializer};
    use time::OffsetDateTime;

    /// Serializes an OffsetDateTime to a Unix timestamp (milliseconds since 1970/1/1T00:00:00T)
    pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let timestamp = date.timestamp();
        serializer.serialize_i64(timestamp)
    }

    /// Attempts to deserialize an i64 and use as a Unix timestamp
    pub fn deserialize<'de>(deserializer: i64) -> anyhow::Result<NaiveDateTime> {
        Ok(NaiveDateTime::from_timestamp(deserializer, 0))
    }
}

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct User {
    pub id: i64,
    pub username: Option<String>,
    pub password: Option<String>,
    pub phone_number: Option<String>,
    pub is_phone_number_verified: Option<u8>,
    pub wx_open_id: Option<String>,
    pub enable: u8,
    #[serde(with = "db_numeric_date")]
    pub created_time: NaiveDateTime,
}

#[derive(sqlx::FromRow, serde::Serialize, Debug)]
pub struct UserJwt {
    pub id: i64,
    pub user_id: i64,
    pub token_id: String,
    pub access_token: String,
    pub refresh_token: String,
    #[serde(with = "db_numeric_date")]
    pub issue_time: NaiveDateTime,
}
