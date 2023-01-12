use chrono::{DateTime, Utc};
use crate::entity;
use crate::entity::Sku;

#[derive(serde::Serialize, serde::Deserialize,Debug)]
pub struct Product{
    pub id: i64,
    pub name: String,
    pub cover_image: String,
    pub pics: Vec<String>,
    pub video:Option<String>,
    pub description: String,
    pub status: String,
    pub category_ids:Vec<i64>,
    pub skus:Vec<Sku>,
}
