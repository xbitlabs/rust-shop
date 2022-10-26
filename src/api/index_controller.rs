
use crate::{RequestCtx, Response, ResponseBuilder};
use crate::service::product_category_service::ProductCategoryService;

pub struct IndexController;
impl IndexController {
    pub async fn index(ctx: RequestCtx) -> Response {
        let name = ctx.router_params.find("name").unwrap_or("world");
        let rows = ProductCategoryService::list_all_categories().await;
        ResponseBuilder::with_text(format!("Hello {}!", name))
    }
}
