use sqlx::Arguments;
use sqlx::mysql::MySqlArguments;
use rust_shop_core::db::{mysql_connection_pool, SqlCommandExecutor, TransactionManager};
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
            .find_all("SELECT * FROM product_category ORDER BY sort_index ASC")
            .await?;
        Ok(categories)
    }
    pub async fn create(&mut self,category:&ProductCategory)->anyhow::Result<bool>{
        let mut args = MySqlArguments::default();
        let id: i64 = ID_GENERATOR.lock().unwrap().real_time_generate();
        args.add(id);
        args.add(category.name.clone());
        args.add(category.icon.clone());
        args.add(category.pic.clone());
        args.add(category.sort_index);
        let result = self.sql_command_executor.execute_with("INSERT INTO ProductCategory(id,name,icon,pic,sort_index) VALUES(?,?,?,?,?);",args).await?;
        Ok(result > 0)
    }
    pub async fn update(&mut self,category:&ProductCategory)->anyhow::Result<bool>{
        let mut args = MySqlArguments::default();
        args.add(category.name.clone());
        args.add(category.icon.clone());
        args.add(category.pic.clone());
        args.add(category.sort_index);
        args.add(category.id);
        let result = self.sql_command_executor.execute_with("UPDATE ProductCategory SET name=?,icon=?,pic=?,sort_index=? WHERE id=?",args).await?;
        Ok(result > 0)
    }
    pub async fn delete_by_id(&mut self,id:i64)->anyhow::Result<bool> {
        let mut args = MySqlArguments::default();
        args.add(id);
        let result = self.sql_command_executor.execute_with("DELETE FROM ProductCategory WHERE id=?", args).await?;
        Ok(result > 0)
    }
}
