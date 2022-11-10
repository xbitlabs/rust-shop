

pub mod AuthController {
    use std::any::Any;
    use std::string::ToString;
    use std::thread;
    use rust_shop_core::EndpointResult;
    use rust_shop_core::extract::json::Json;
    use rust_shop_macro::route;
    use lazy_static::lazy_static;
    use rust_shop_core::RequestCtx;
    use hyper::Response;
    use hyper::Body;
    use rust_shop_core::router::register_route;
    use rust_shop_core::extract::FromRequest;
    use rust_shop_core::extract::IntoResponse;

    #[derive(serde::Serialize,serde::Deserialize)]
    pub struct User{
        pub name:String
    }


    #[route("post","/user/login")]
    pub async fn login(Json(user):Json<User>) ->anyhow::Result<Json<EndpointResult<bool>>> {
       Ok(Json(EndpointResult::ok_with_payload("",true)))
    }
    #[route("post","/user/create")]
    pub async fn create(Json(user):Json<User>) ->anyhow::Result<Json<EndpointResult<bool>>> {
        Ok(Json(EndpointResult::ok_with_payload("",true)))
    }
}