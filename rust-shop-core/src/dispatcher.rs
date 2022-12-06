use crate::handler_interceptor::HandlerInterceptor;
use crate::{Next, RequestCtx};
use http::{Request, Response};
use hyper::Body;

pub struct Dispatcher {}

impl Dispatcher {}
/*impl HandlerInterceptor for Dispatcher{
    fn pre_handle(&self, request:RequestCtx) -> anyhow::Result<Response<Body>> {
        todo!()
    }

    fn after_completion(&self, request:RequestCtx, response: &mut Request<Body>) {
        todo!()
    }
}*/
