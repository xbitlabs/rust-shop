use std::collections::HashMap;
use rust_shop_core::extract::json::Json;
use rust_shop_core::{HTTPHandler, RequestCtx};
use crate::test1::User;
//use crate::test1::add;
/*static mut ROUTER : HashMap<String,Box<dyn HTTPHandler>> = HashMap::new();
unsafe fn reg(method:String, handler:Box<dyn HTTPHandler>) ->bool{
    ROUTER.entry(method)
        .or_insert_with(||handler);
    return true;
}

static v:bool = unsafe { reg(String::from("post"), ) };*/

pub mod test1 {
    #[derive(serde::Serialize,serde::Deserialize)]
    pub struct User{
        pub name:String,
        pub age:u32,
    }

    use rust_shop_core::extract::json::Json;
    use rust_shop_macro::route;

    #[route("post", "/")]
    pub async fn add(Json(payload): Json<User>) -> String {
        String::from("hello")
    }

    #[route("post", "/")]
    pub async fn update(Json(payload): Json<User>) -> String {
        String::from("hello")
    }
}


fn main() {
    //add(Json::from_request());

    println!("Hello, world!{}",std::any::type_name::<Json<User>>());
}
