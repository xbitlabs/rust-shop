use std::fmt::Error;
use sqlx::{MySql, MySqlPool};
use rust_shop_core::db_pool_manager::mysql_connection_pool;
use crate::entity::entity::ProductCategory;
use crate::{DbPoolManager};


pub struct ProductCategoryService<'a,'b>{
    mysql_pool_manager: &'a DbPoolManager<'b,MySql>
}
impl <'a,'b> ProductCategoryService<'a,'b> {
    pub fn new(mysql_pool_manager:&'b DbPoolManager<MySql>) ->Self{
        ProductCategoryService{
            mysql_pool_manager
        }
    }
    pub async fn list_all_categories()->anyhow::Result<Vec<ProductCategory>>  {
        let pool = mysql_connection_pool().await?;
        let categories = sqlx::query_as!(ProductCategory,"SELECT * FROM product_category")
            .fetch_all(&pool).await?;
        println!("查询到的数据有{}条",categories.len());
        Ok(categories)
    }
}