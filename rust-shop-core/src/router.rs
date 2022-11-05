use std::collections::HashMap;
use lazy_static::lazy_static;
use route_recognizer::{Params, Router as MethodRouter};
use std::sync::Mutex;
use crate::{BoxHTTPHandler, HTTPHandler};

pub type Router = HashMap<String, MethodRouter<BoxHTTPHandler>>;


lazy_static! {
    ///
    /// 全局配置
    ///
    pub static ref ROUTER: Mutex<Router> = Mutex::new(HashMap::new());
}


pub fn register_route(method:String,path:String,handler: impl HTTPHandler)->bool{
    let lock_result = ROUTER.lock();
    match lock_result {
        Ok(mut result)=>{
            result.entry(method)
                .or_insert_with(MethodRouter::new)
                .add(path.as_ref(), Box::new(handler));
            true
        }
        _=>{
            false
        }
    }
}