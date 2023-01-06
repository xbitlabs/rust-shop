use anyhow::anyhow;
use chrono::{Local, Utc};
use sqlx::Arguments;
use sqlx::mysql::MySqlArguments;
use rust_shop_core::db::{mysql_connection_pool, SqlCommandExecutor, TransactionManager};
use rust_shop_core::db::traits::Crud;
use rust_shop_core::id_generator::ID_GENERATOR;
use rust_shop_core::jwt::DefaultJwtService;
use crate::entity::ProductCategory;

pub struct ProductCategoryService<'a, 'b> {
    sql_command_executor: &'b mut SqlCommandExecutor<'a, 'b>,
}

impl<'a, 'b> ProductCategoryService<'a, 'b> {
    pub fn new(sql_command_executor: &'b mut SqlCommandExecutor<'a, 'b>) -> Self {
        ProductCategoryService {
            sql_command_executor,
        }
    }
    pub async fn list_all_categories(&mut self) -> anyhow::Result<Vec<ProductCategory>> {
        let categories = self
            .sql_command_executor
            .find_all("SELECT * FROM product_category WHERE is_deleted = 0 ORDER BY sort_index ASC")
            .await?;
        Ok(categories)
    }
    pub async fn create(&mut self,category:&mut ProductCategory)->anyhow::Result<bool>{
        category.created_time = Utc::now();
        category.is_deleted = false;
        let result =  category.create(self.sql_command_executor).await?;
        Ok(result)
    }
    pub async fn update(&mut self,category:&ProductCategory)->anyhow::Result<bool>{
        let result = category.update(self.sql_command_executor).await?;
        Ok(result)
    }
    pub async fn delete_by_id(&mut self,id:i64)->anyhow::Result<bool> {
        let result = ProductCategory::delete_by_id(id, self.sql_command_executor).await?;
        Ok(result)
    }
    pub async fn mark_deleted(&mut self,id:i64)->anyhow::Result<bool>{
        let category = ProductCategory::select_by_id(id,self.sql_command_executor).await?;
        if category.is_some() {
            let mut category = category.unwrap();
            category.is_deleted = true;
            let result = category.update(self.sql_command_executor).await?;
            Ok(result)
        }else {
            Ok(false)
        }
    }
    pub async fn get_by_id(&mut self,id:i64)->anyhow::Result<ProductCategory>{
        let category = ProductCategory::select_by_id(id,self.sql_command_executor).await?;
        if category.is_some() {
            Ok(category.unwrap())
        }else {
            Err(anyhow!(format!("not found ProductCategory by id：{}",id)))
        }
    }
}
