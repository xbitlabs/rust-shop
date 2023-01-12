pub mod product_controller{
    use anyhow::anyhow;
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
    use rust_shop_core::extract::query::Query;
    use rust_shop_core::response::into_response::IntoResponse;
    use crate::vo::Page;

    #[route("POST", "/product")]
    pub async fn create<'db,'a>(Json(mut product):Json<dto::Product>, sql_exe: &'a mut SqlCommandExecutor<'db, 'a>,) -> anyhow::Result<EndpointResult<&'static str>>{
        let mut product_service = ProductService::new(sql_exe);
        let result = product_service.create(&mut product).await?;
        if result {
            Ok(EndpointResult::ok_with_payload("新增商品成功",""))
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
}