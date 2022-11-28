use http::Request;
use hyper::Body;
use crate::RequestCtx;

pub trait HandlerInterceptor{
    fn pre_handle(&self,request:&mut RequestCtx) ->bool;
    fn after_completion(&self,request:&mut RequestCtx, response:&mut Request<Body>);
}