use anyhow::anyhow;
use chrono::{Local, Utc};
use sqlx::Arguments;
use sqlx::mysql::MySqlArguments;
use rust_shop_core::db::{mysql_connection_pool, SqlCommandExecutor, TransactionManager};
use rust_shop_core::db::traits::Crud;
use rust_shop_core::id_generator::ID_GENERATOR;
use rust_shop_core::jwt::DefaultJwtService;
use crate::entity::Category;

pub struct CategoryService<'a, 'b> {
    sql_command_executor: &'b mut SqlCommandExecutor<'a, 'b>,
}

impl<'a, 'b> CategoryService<'a, 'b> {
    pub fn new(sql_command_executor: &'b mut SqlCommandExecutor<'a, 'b>) -> Self {
        CategoryService {
            sql_command_executor,
        }
    }
    pub async fn list_all_categories(&mut self) -> anyhow::Result<Vec<Category>> {
        let categories = self
            .sql_command_executor
            .find_all("SELECT * FROM category WHERE is_deleted = 0 ORDER BY sort_index ASC")
            .await?;
        Ok(categories)
    }
    pub async fn create(&mut self, category:&mut Category) ->anyhow::Result<bool>{
        category.created_time = Utc::now();
        category.is_deleted = false;
        let result =  category.create(self.sql_command_executor).await?;
        Ok(result)
    }
    pub async fn update(&mut self, category:&Category) ->anyhow::Result<bool>{
        let result = category.update(self.sql_command_executor).await?;
        Ok(result)
    }
    pub async fn delete_by_id(&mut self,id:i64)->anyhow::Result<bool> {
        let result = Category::delete_by_id(id, self.sql_command_executor).await?;
        Ok(result)
    }
    pub async fn mark_deleted(&mut self,id:i64)->anyhow::Result<bool>{
        let category = Category::select_by_id(id, self.sql_command_executor).await?;
        if category.is_some() {
            let mut category = category.unwrap();
            category.is_deleted = true;
            let result = category.update(self.sql_command_executor).await?;
            Ok(result)
        }else {
            Ok(false)
        }
    }
    pub async fn get_by_id(&mut self,id:i64)->anyhow::Result<Category>{
        let category = Category::select_by_id(id, self.sql_command_executor).await?;
        if category.is_some() {
            Ok(category.unwrap())
        }else {
            Err(anyhow!(format!("not found Category by id：{}",id)))
        }
    }
}
