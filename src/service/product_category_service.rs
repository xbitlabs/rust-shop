use std::fmt::Error;
use sqlx::MySqlPool;
use rust_shop_core::db_pool_manager::get_connection_pool;
use crate::entity::entity::ProductCategory;
use crate::{MysqlPoolManager};


pub struct ProductCategoryService<'a,'b>{
    mysql_pool_manager: &'a MysqlPoolManager<'b>
}
impl <'a,'b> ProductCategoryService<'a,'b> {
    pub fn new(mysql_pool_manager:&'b MysqlPoolManager)->Self{
        ProductCategoryService{
            mysql_pool_manager
        }
    }
    pub async fn list_all_categories()->anyhow::Result<Vec<ProductCategory>>  {
        let pool = get_connection_pool().await?;
        let categories = sqlx::query_as!(ProductCategory,"SELECT * FROM product_category")
            .fetch_all(&pool).await?;
        println!("查询到的数据有{}条",categories.len());
        Ok(categories)
    }
}