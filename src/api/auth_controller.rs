

pub mod AuthController {
    use std::any::Any;
    use std::string::ToString;
    use std::thread;
    use rust_shop_core::{EndpointResult, ResponseBuilder};
    use rust_shop_core::extract::json::Json;
    use rust_shop_macro::route;
    use lazy_static::lazy_static;
    use rust_shop_core::RequestCtx;
    use hyper::Response;
    use hyper::Body;
    use rust_shop_core::router::register_route;
    use rust_shop_core::extract::FromRequest;
    use rust_shop_core::extract::IntoResponse;
    use rust_shop_core::extract::path_variable::PathVariable;
    use rust_shop_core::extract::request_param::RequestParam;
    use crate::StatusCode;

    #[derive(serde::Serialize,serde::Deserialize)]
    pub struct User{
        pub name:String
    }


    #[route("POST","/user/login")]
    pub async fn login(Json(user):Json<User>) ->anyhow::Result<Json<User>> {
       Ok(Json(user))
    }
    #[route("POST","/user/create")]
    pub async fn create(Json(user):Json<User>) ->anyhow::Result<Json<User>> {
        Ok(Json(user))
    }
    #[route("POST","/user/del")]
    pub async fn del(ctx:RequestCtx) ->anyhow::Result<Response<Body>> {
        let parts = ctx.request.into_parts();
        Ok(ResponseBuilder::with_status(StatusCode::OK))
    }
    pub async fn test(PathVariable(id):PathVariable<u32>,RequestParam(name):RequestParam<String>,Json(user):Json<User>)->anyhow::Result<Json<User>>{
        Ok(Json(user))
    }
    pub async fn test_main(ctx:RequestCtx)->anyhow::Result<Response<Body>>{
        let id : PathVariable<u32> = PathVariable(ctx.router_params.find("id").unwrap().parse().unwrap());
        let name:RequestParam<String> = RequestParam(ctx.query_params.get("name").unwrap().to_string());
        let user:Json<User> = Json::from_request(ctx).await?;
        let result = test(id,name,user).await?;
        Ok(result.into_response())
    }
}