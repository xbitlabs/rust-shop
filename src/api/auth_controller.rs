

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
        Ok(ResponseBuilder::with_status(StatusCode::OK))
    }
}