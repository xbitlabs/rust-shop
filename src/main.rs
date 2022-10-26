
use std::net::SocketAddr;
use crate::{AccessLog, Middleware, Next, RequestCtx, Response, ResponseBuilder, Server};
use crate::controller::index_controller::IndexController;

pub struct  AuthMiddleware;

#[async_trait::async_trait]
impl Middleware for AuthMiddleware {
    async fn handle<'a>(&'a self, ctx: RequestCtx, next: Next<'a>) -> Response {
        ResponseBuilder::with_html("无权限")
    }
}

#[tokio::main]
async fn main() {
    let addr: SocketAddr = "127.0.0.1:5000".parse().unwrap();

    let mut srv = Server::new();

    srv.middleware(AccessLog);
    //srv.middleware(AuthMiddleware);

    srv.get("/", |_req| async move {
        hyper::Response::builder()
            .status(hyper::StatusCode::OK)
            .body(hyper::Body::from("Welcome!"))
            .unwrap()
    });
    srv.get("/hello/:name", hello);
    srv.post("/post", post);
    srv.post("/test",IndexController::index);
    srv.run(addr).await.unwrap();
}

async fn hello(ctx: RequestCtx) -> Response {
    let name = ctx.router_params.find("name").unwrap_or("world");

    ResponseBuilder::with_text(format!("Hello {}!", name))
}
async fn post(ctx: RequestCtx) -> Response {
    let name = ctx.router_params.find("name").unwrap_or("world");

    ResponseBuilder::with_text(format!("Hello {}!", name))
}