

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
    use anyhow::anyhow;
    use std::convert::Infallible;
    use rust_shop_core::extract::form::Form;
    use rust_shop_core::extract::header::Header;
    use rust_shop_core::extract::query::Query;

    #[derive(serde::Serialize,serde::Deserialize,Debug)]
    pub struct User{
        pub id:u32,
        pub name:String
    }

    #[route("POST","/user/:id")]
    pub async fn test(Header(token):Header<Option<String>>,
                      PathVariable(id):PathVariable<Option<u32>>,
                      RequestParam(name):RequestParam<Option<String>>,
                      Form(user):Form<User>)->anyhow::Result<Json<User>>{
        let u = User{
            id:id.unwrap(),
            name:name.unwrap()
        };
        if token.is_none() {
            println!("token={}","None");
        }else {
            println!("token={}",token.unwrap());
        }
        println!("{:?}",u);

        Ok(Json(u))
    }
    #[route("POST","/user")]
    pub async fn test_handler11(ctx:RequestCtx)->anyhow::Result<Json<User>>{
        Ok(Json(User{
            id: 0,
            name: "pgg".to_string()
        }))
    }
}