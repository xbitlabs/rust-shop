pub mod category_controller{
    use anyhow::anyhow;
    use rust_shop_core::db::SqlCommandExecutor;
    use rust_shop_core::EndpointResult;
    use rust_shop_core::extract::json::Json;
    use crate::entity::{Category, Product};
    use crate::service::category_service::CategoryService;
    use rust_shop_macro::route;
    use rust_shop_core::RequestCtx;
    use rust_shop_core::response::Response;
    use rust_shop_core::state::State;
    use sqlx::Pool;
    use sqlx::MySql;
    use rust_shop_core::APP_EXTENSIONS;
    use rust_shop_core::extract::FromRequest;
    use rust_shop_core::extract::path_variable::PathVariable;
    use rust_shop_core::response::into_response::IntoResponse;

    #[route("POST", "/category")]
    pub async fn create<'db,'a>(Json(mut category):Json<Category>, sql_exe: &'a mut SqlCommandExecutor<'db, 'a>,)->anyhow::Result<EndpointResult<&'static str>>{
        let mut category_service = CategoryService::new(sql_exe);
        let result = category_service.create(&mut category).await?;
        if result {
            Ok(EndpointResult::ok("新增商品分类成功"))
        }else {
            Err(anyhow!("新增商品分类失败"))
        }
    }
    #[route("PUT", "/category")]
    pub async fn update<'db,'a>(Json(mut category):Json<Category>, sql_exe: &'a mut SqlCommandExecutor<'db, 'a>,)->anyhow::Result<EndpointResult<&'static str>>{
        let mut category_service = CategoryService::new(sql_exe);
        let result = category_service.update(&mut category).await?;
        if result {
            Ok(EndpointResult::ok("更新商品分类成功"))
        }else {
            Err(anyhow!("更新商品分类失败"))
        }
    }
    #[route("DELETE", "/category/:id")]
    pub async fn delete_by_id<'db,'a>(PathVariable(id):PathVariable<i64>, sql_exe: &'a mut SqlCommandExecutor<'db, 'a>,)->anyhow::Result<EndpointResult<&'static str>>{
        let mut category_service = CategoryService::new(sql_exe);
        let result = category_service.mark_deleted(id).await?;
        if result {
            Ok(EndpointResult::ok("删除商品分类成功"))
        }else {
            Err(anyhow!("删除商品分类失败"))
        }
    }
    #[route("GET", "/category/:id")]
    pub async fn get_by_id<'db,'a>(PathVariable(id):PathVariable<i64>, sql_exe: &'a mut SqlCommandExecutor<'db, 'a>,)->anyhow::Result<EndpointResult<Category>>{
        let mut category_service = CategoryService::new(sql_exe);
        let result = category_service.get_by_id(id).await?;
        Ok(EndpointResult::ok_with_payload("",result))
    }
    #[route("GET", "/category/list")]
    pub async fn list_all<'db,'a>(sql_exe: &'a mut SqlCommandExecutor<'db, 'a>,)->anyhow::Result<EndpointResult<Vec<Category>>>{
        let mut category_service = CategoryService::new(sql_exe);
        let result = category_service.list_all_categories().await?;
        Ok(EndpointResult::ok_with_payload("",result))
    }
}