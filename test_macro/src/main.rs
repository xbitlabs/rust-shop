use axum::extract::FromRequest;
use axum::Json;
use crate::test1::add;

pub mod test1 {

    pub struct User{
        pub name:String,
        pub age:u32,
    }

    use axum::Json;
    use axum_route::route;
    use axum::routing::{get,post};

    #[route(post, "/")]
    pub async fn add(Json(payload): Json<User>) -> String {
        String::from("hello")
    }

    #[route(post, "/")]
    pub async fn update(Json(payload): Json<User>) -> String {
        String::from("hello")
    }
}



fn main() {
    //add(Json::from_request());
    println!("Hello, world!");
}
