pub mod product_controller{
    use anyhow::anyhow;
    use chrono::{Local, TimeZone, Utc};
    use rust_shop_core::db::SqlCommandExecutor;
    use rust_shop_core::EndpointResult;
    use rust_shop_core::extract::json::Json;
    use crate::{dto, qo, vo};
    use crate::service::product_service::ProductService;
    use rust_shop_macro::route;
    use rust_shop_core::RequestCtx;
    use rust_shop_core::response::Response;
    use rust_shop_core::state::State;
    use sqlx::Pool;
    use sqlx::MySql;
    use rust_shop_core::APP_EXTENSIONS;
    use rust_shop_core::extract::FromRequest;
    use rust_shop_core::extract::path_variable::PathVariable;
    use rust_shop_core::extract::query::Query;
    use rust_shop_core::response::into_response::IntoResponse;
    use crate::vo::Page;
    use rust_shop_core::db::TransactionManager;
    use rust_shop_core::extract::request_param::RequestParam;

    #[route("POST", "/product")]
    pub async fn create<'db,'a>(Json(mut product):Json<dto::Product>, sql_exe_with_tran: &'a mut SqlCommandExecutor<'db, 'a>,) -> anyhow::Result<EndpointResult<&'static str>>{
        if product.name.is_empty() {
            return Err(anyhow!("商品名称不能为空"));
        }
        let mut product_service = ProductService::new(sql_exe_with_tran);
        let result = product_service.create(&mut product).await?;
        if result {
            Ok(EndpointResult::ok("新增商品成功"))
        }else {
            Err(anyhow!("新增商品失败"))
        }
    }
    #[route("GET", "/product/page")]
    pub async fn page<'db,'a>(Query(mut page_query):Query<qo::ProductPageQueryRequest>, sql_exe: &'a mut SqlCommandExecutor<'db, 'a>,) -> anyhow::Result<EndpointResult<Page<vo::Product>>>{
        let mut product_service = ProductService::new(sql_exe);
        let result = product_service.page(&page_query).await?;
        Ok(EndpointResult::ok_with_payload("",result))
    }
    #[route("PUT", "/product")]
    pub async fn update<'db,'a>(Json(mut product):Json<dto::Product>,sql_exe_with_tran: &'a mut SqlCommandExecutor<'db, 'a>,)->anyhow::Result<EndpointResult<&'static str>>{
        let mut product_service = ProductService::new(sql_exe_with_tran);
        let result = product_service.update(&mut product).await?;
        if result {
            Ok(EndpointResult::ok("商品更新成功"))
        }else {
            Err(anyhow!("商品更新失败"))
        }
    }
    #[route("GET", "/product/:id")]
    pub async fn get_by_id<'db,'a>(PathVariable(id):PathVariable<i64>,sql_exe: &'a mut SqlCommandExecutor<'db, 'a>,)->anyhow::Result<EndpointResult<vo::Product>>{
        let mut product_service = ProductService::new(sql_exe);
        let product = product_service.get_by_id(id).await?;
        Ok(EndpointResult::ok_with_payload("",product))
    }
    #[route("DELETE", "/product/:id")]
    pub async fn delete_by_id<'db,'a>(PathVariable(id):PathVariable<i64>,sql_exe: &'a mut SqlCommandExecutor<'db, 'a>,)->anyhow::Result<EndpointResult<vo::Product>>{
        let mut product_service = ProductService::new(sql_exe);
        let result = product_service.mark_deleted(id).await?;
        if result {
            Ok(EndpointResult::ok("商品删除成功"))
        }else {
            Err(anyhow!("删除商品失败"))
        }
    }
    #[route("PUT", "/product/put_on_sale")]
    pub async fn put_product_on_sale<'db,'a>(RequestParam(id):RequestParam<i64>,sql_exe: &'a mut SqlCommandExecutor<'db, 'a>,)->anyhow::Result<EndpointResult<&'static str>>{
        let mut product_service = ProductService::new(sql_exe);
        let result = product_service.put_product_on_sale(id).await?;
        if result {
            Ok(EndpointResult::ok("商品上架成功"))
        }else {
            Err(anyhow!("上架商品失败"))
        }
    }
    #[route("PUT", "/product/remove_from_shelves")]
    pub async fn remove_from_shelves<'db,'a>(RequestParam(id):RequestParam<i64>,sql_exe: &'a mut SqlCommandExecutor<'db, 'a>,)->anyhow::Result<EndpointResult<&'static str>>{
        let mut product_service = ProductService::new(sql_exe);
        let result = product_service.remove_from_shelves(id).await?;
        if result {
            Ok(EndpointResult::ok("商品下架成功"))
        }else {
            Err(anyhow!("下架商品失败"))
        }
    }
}