use chrono::NaiveDateTime;

mod db_numeric_date {
    //! Custom serialization of OffsetDateTime to conform with the JWT spec (RFC 7519 section 2, "Numeric Date")
    use chrono::NaiveDateTime;
    use serde::{self, Deserialize, Deserializer, Serializer};
    use time::OffsetDateTime;
    use crate::RustShopError;

    /// Serializes an OffsetDateTime to a Unix timestamp (milliseconds since 1970/1/1T00:00:00T)
    pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let timestamp = date.timestamp();
        serializer.serialize_i64(timestamp)
    }

    /// Attempts to deserialize an i64 and use as a Unix timestamp
    pub fn deserialize<'de>(deserializer: i64) -> Result<NaiveDateTime, RustShopError>
    {
        Ok(NaiveDateTime::from_timestamp(deserializer,0))
    }
}

#[derive(sqlx::FromRow,serde::Serialize)]
pub struct ProductCategory{
    pub id:i64,
    pub name:String,
    pub icon:Option<String>,
    pub pic:Option<String>,
    pub sort_index:i32,
}
#[derive(sqlx::FromRow,serde::Serialize)]
pub struct Product{
    pub id:i64,
    pub name:String,
    pub cover_image:String,
    pub category_id:i64,
    pub pics_and_video:String,
    pub description:String,
    #[serde(with = "db_numeric_date")]
    pub created_time:NaiveDateTime,
    #[serde(with = "db_numeric_date")]
    pub last_modified_time:NaiveDateTime,
}
#[derive(sqlx::FromRow,serde::Serialize)]
pub struct Order{
    pub id:i64,
    pub user_id:i64,
    pub logistics_status:Option<String>,
    pub pay_status:String,
    pub recipient:String,
    pub phone_number:String,
    pub address:String,
    pub post_code:String,
    pub remark:Option<String>,
    #[serde(with = "db_numeric_date")]
    pub created_time:NaiveDateTime,
}
#[derive(sqlx::FromRow,serde::Serialize)]
pub struct OrderItem{
    pub id:i64,
    pub order_id:i64,
    pub sku_id:i64,
    pub quantity:i32,
    pub price:f64
}
#[derive(sqlx::FromRow,serde::Serialize)]
pub struct PayLog{
    pub id:i64,
    pub order_id:i64,
    pub pay_request_info:Option<String>,
    pub pay_response:Option<String>,
    pub callback_infos:Option<String>,
    #[serde(with = "db_numeric_date")]
    pub pay_time:NaiveDateTime
}
#[derive(sqlx::FromRow,serde::Serialize)]
pub struct ShoppingCart{
    pub id:i64,
    pub sku_id:i64,
    pub quantity:i32,
    pub user_id:i64,
    #[serde(with = "db_numeric_date")]
    pub add_time:NaiveDateTime,
}
#[derive(sqlx::FromRow,serde::Serialize)]
pub struct Sku{
    pub id:i64,
    pub title:String,
    pub product_id:i64,
    pub price:f64,
    pub is_default:bool,
}
#[derive(sqlx::FromRow,serde::Serialize)]
pub struct User{
    pub id:i64,
    pub phone_number:Option<String>,
    pub password:Option<String>,
    pub wx_open_id:String,
    #[serde(with = "db_numeric_date")]
    pub created_time:NaiveDateTime,
}
#[derive(sqlx::FromRow,serde::Serialize)]
pub struct UserJwt{
    pub id:i64,
    pub user_id:i64,
    pub token_id:String,
    pub access_token:String,
    pub refresh_token:String,
    #[serde(with = "db_numeric_date")]
    pub issue_time:NaiveDateTime,
}
#[derive(sqlx::FromRow,serde::Serialize)]
pub struct UserShippingAddress{
    pub id:i64,
    pub user_id:i64,
    pub recipient:String,
    pub phone_number:String,
    pub address:String,
    pub post_code:String,
    pub is_default:bool,
    #[serde(with = "db_numeric_date")]
    pub created_time:NaiveDateTime,
}