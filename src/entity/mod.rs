use chrono::{DateTime, Local, NaiveDateTime, Utc};
use serde::{Deserializer, Serializer};
use rust_shop_macro::SqlxCrud;
use sqlx::MySql;
use sqlx::mysql::MySqlArguments;

#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize, Debug,SqlxCrud)]
pub struct Category {
    #[serde(with = "rust_shop_core::serde_utils::long_format")]
    pub id: i64,
    pub name: String,
    #[serde(with = "rust_shop_core::serde_utils::option_long_format")]
    pub parent_id:Option<i64>,
    pub icon: Option<String>,
    pub pic: Option<String>,
    pub sort_index: i32,
    #[serde(with = "rust_shop_core::serde_utils::date_format")]
    pub created_time: DateTime<Utc>,
    pub is_deleted:bool,
}

#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize,Debug,SqlxCrud)]
pub struct Product {
    pub id: i64,
    pub name: String,
    pub cover_image: String,
    pub pics: String,
    pub video:Option<String>,
    pub description: String,
    pub status: String,
    #[serde(with = "rust_shop_core::serde_utils::date_format")]
    pub created_time: DateTime<Utc>,
    pub last_modified_time: Option<DateTime<Utc>>,
    pub is_deleted:bool,
}

#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize,Debug)]
pub struct ProductCategoryMapping{
    pub product_id:i64,
    pub product_category_id:i64
}

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct Order {
    pub id: i64,
    pub user_id: i64,
    pub logistics_status: Option<String>,
    pub pay_status: String,
    pub recipient: String,
    pub phone_number: String,
    pub address: String,
    pub post_code: String,
    pub remark: Option<String>,
    #[serde(with = "rust_shop_core::serde_utils::date_format")]
    pub created_time: DateTime<Utc>,
}

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct OrderItem {
    pub id: i64,
    pub order_id: i64,
    pub product_id: i64,
    pub sku_id: i64,
    pub quantity: i32,
    pub price: f64,
}

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct PayLog {
    pub id: i64,
    pub order_id: i64,
    pub pay_request_info: Option<String>,
    pub pay_response: Option<String>,
    pub callback_infos: Option<String>,
    #[serde(with = "rust_shop_core::serde_utils::date_format")]
    pub pay_time: DateTime<Utc>,
}

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct ShoppingCart {
    pub id: i64,
    pub product_id: i64,
    pub sku_id: i64,
    pub quantity: i32,
    pub user_id: i64,
    #[serde(with = "rust_shop_core::serde_utils::date_format")]
    pub add_time: DateTime<Utc>,
}

#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize,Debug,SqlxCrud)]
pub struct Sku {
    #[serde(with = "rust_shop_core::serde_utils::long_format")]
    pub id: i64,
    pub title: String,
    #[serde(with = "rust_shop_core::serde_utils::long_format")]
    pub product_id: i64,
    pub price: bigdecimal::BigDecimal,
    pub is_default: bool,
    pub is_deleted:bool,
}


#[derive(sqlx::FromRow, serde::Serialize)]
pub struct UserShippingAddress {
    pub id: i64,
    pub user_id: i64,
    pub recipient: String,
    pub phone_number: String,
    pub address: String,
    pub post_code: String,
    pub is_default: bool,
    #[serde(with = "rust_shop_core::serde_utils::date_format")]
    pub created_time: DateTime<Utc>,
}

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct Promotion {
    pub id: i64,
}
