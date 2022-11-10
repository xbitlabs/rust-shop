use std::cell::RefCell;
use std::collections::HashMap;
use lazy_static::lazy_static;
use route_recognizer::{Params, Router as MethodRouter};
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;

use crate::{BoxHTTPHandler, HTTPHandler};

pub type Router = HashMap<String, MethodRouter<BoxHTTPHandler>>;

pub static mut GLOBAL_ROUTER: Lazy<Router> = Lazy::new(|| {
    let mut m:Router = HashMap::new();
    m
});

lazy_static! {
    ///
    /// 全局配置
    ///
    pub static ref ROUTER: Mutex<Router> = Mutex::new(HashMap::new());
}


pub fn register_route(method:&'static str, path:&'static str, handler: impl HTTPHandler) ->bool{
    /*let lock_result = ROUTER.lock();
    match lock_result {
        Ok(mut result)=>{
            result.entry(method.to_string())
                .or_insert_with(MethodRouter::new)
                .add(path.as_ref(), Box::new(handler));
            true
        }
        _=>{
            false
        }
    }*/
    unsafe {
        GLOBAL_ROUTER.entry(method.to_string())
            .or_insert_with(MethodRouter::new)
            .add(path.as_ref(), Box::new(handler));
    }

    true
}