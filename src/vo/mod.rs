use chrono::{DateTime, Utc};
use crate::entity::Sku;

#[derive( serde::Serialize, serde::Deserialize,Debug)]
pub struct Page<T> where T : serde::Serialize{
    pub page_size:i64,
    pub page_index:i64,
    pub page_count:i64,
    pub record_count:i64,
    pub items:Vec<T>
}

#[derive( serde::Serialize, serde::Deserialize,Debug)]
pub struct Product{
    pub id: i64,
    pub name: String,
    pub cover_image: String,
    pub pics: Vec<String>,
    pub video:Option<String>,
    pub description: String,
    pub status: String,
    pub created_time: DateTime<Utc>,
    pub last_modified_time: Option<DateTime<Utc>>,
    pub is_deleted:bool,
    pub product_category_ids:Vec<i64>,
    pub skus:Vec<Sku>,
}
