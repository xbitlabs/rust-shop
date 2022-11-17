use chrono::NaiveDateTime;

#[derive(sqlx::FromRow,serde::Serialize,serde::Deserialize,Debug)]
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
    pub status:String,
    #[serde(with = "rust_shop_core::entity::db_numeric_date")]
    pub created_time:NaiveDateTime,
    #[serde(with = "rust_shop_core::entity::db_numeric_date")]
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
    #[serde(with = "rust_shop_core::entity::db_numeric_date")]
    pub created_time:NaiveDateTime,
}
#[derive(sqlx::FromRow,serde::Serialize)]
pub struct OrderItem{
    pub id:i64,
    pub order_id:i64,
    pub product_id:i64,
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
    #[serde(with = "rust_shop_core::entity::db_numeric_date")]
    pub pay_time:NaiveDateTime
}
#[derive(sqlx::FromRow,serde::Serialize)]
pub struct ShoppingCart{
    pub id:i64,
    pub product_id:i64,
    pub sku_id:i64,
    pub quantity:i32,
    pub user_id:i64,
    #[serde(with = "rust_shop_core::entity::db_numeric_date")]
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
pub struct UserShippingAddress{
    pub id:i64,
    pub user_id:i64,
    pub recipient:String,
    pub phone_number:String,
    pub address:String,
    pub post_code:String,
    pub is_default:bool,
    #[serde(with = "rust_shop_core::entity::db_numeric_date")]
    pub created_time:NaiveDateTime,
}
#[derive(sqlx::FromRow,serde::Serialize)]
pub struct Promotion{
    pub id:i64,
}