use std::collections::HashMap;
use rust_shop_core::extract::json::Json;
use rust_shop_core::{HTTPHandler, RequestCtx};
use rust_shop_core::router::register_route;
use crate::test1::User;

pub mod test1 {
    #[derive(serde::Serialize,serde::Deserialize)]
    pub struct User{
        pub name:String,
        pub age:u32,
    }

    use rust_shop_core::extract::json::Json;
    use rust_shop_macro::route;
    use rust_shop_core::router::register_route;
    use lazy_static::lazy_static;
    use rust_shop_core::RequestCtx;
    use hyper::Response;
    use hyper::Body;
    use rust_shop_core::extract::FromRequest;
    use rust_shop_core::extract::IntoResponse;

    #[route("post", "/")]
    pub async fn add(Json(payload): Json<User>) -> anyhow::Result<Json<User>> {
        Ok(Json(payload))
    }

    #[route("post", "/")]
    pub async fn update(Json(payload): Json<User>) -> anyhow::Result<Json<User>> {
        Ok(Json(payload))
    }
}


fn main() {
    println!("Hello, world!{}",std::any::type_name::<Json<User>>());
}

