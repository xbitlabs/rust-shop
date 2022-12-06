use crate::RequestCtx;
use http::Request;
use hyper::Body;

pub trait HandlerInterceptor {
    fn pre_handle(&self, request: RequestCtx) -> bool;
    fn after_completion(&self, request: RequestCtx, response: &mut Request<Body>);
}
