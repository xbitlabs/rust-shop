use std::borrow::BorrowMut;
use std::fmt::Error;
use sqlx::{MySql, MySqlPool};
use uuid::Uuid;
use rust_shop_core::db::{mysql_connection_pool, SqlCommandExecutor, TransactionManager};
use rust_shop_core::id_generator::ID_GENERATOR;
use crate::entity::entity::ProductCategory;
use chrono::Local;


pub struct ProductCategoryService<'a,'b>{
    sql_command_executor: &'b mut SqlCommandExecutor<'a,'b>
}
impl <'a,'b> ProductCategoryService<'a,'b> {
    pub fn new(sql_command_executor:&'b mut SqlCommandExecutor<'a,'b>) ->Self{
        ProductCategoryService{
            sql_command_executor
        }
    }
    pub async fn list_all_categories(&mut self)->anyhow::Result<Vec<ProductCategory>>  {
        let categories = self.sql_command_executor.find_all("SELECT * FROM product_category").await?;
        println!("查询到的数据有{}条",categories.len());
        Ok(categories)
    }
    pub async fn test_tran(&mut self) ->anyhow::Result<()>  {
      /*  let user_id = ID_GENERATOR.lock().unwrap().real_time_generate();
        let wx_open_id = Uuid::new_v4().to_string();
        let mut aag = self.mysql_pool_manager.tran();
        let mut aa: &mut &TransactionManager<MySql> = aag.borrow_mut();

        let  f = aa.tran();
        let rows_affected = sqlx::query!("insert into `user`(id,wx_open_id,created_time,enable) values(?,?,?,?)",user_id,wx_open_id,Local::now(),1)
            .execute(f).await?
            .rows_affected();*/
        Ok(())
    }
}