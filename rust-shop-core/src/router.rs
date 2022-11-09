use std::cell::RefCell;
use std::collections::HashMap;
use lazy_static::lazy_static;
use route_recognizer::{Params, Router as MethodRouter};
use std::sync::{Arc, Mutex};
use crate::{BoxHTTPHandler, HTTPHandler};

pub type Router = HashMap<String, MethodRouter<BoxHTTPHandler>>;

/*const  ROUTER1 : Arc<Router> = Arc::new(HashMap::with_capacity(10));
 fn register_route1(method:String, path:String, handler: impl HTTPHandler) ->bool{
    ROUTER1.entry(method)
        .or_insert_with(MethodRouter::new)
        .add(path.as_ref(), Box::new(handler));
    return true;
}*/

//static v:bool = unsafe { reg(String::from("post"), ) };


lazy_static! {
    ///
    /// 全局配置
    ///
    pub static ref ROUTER: Mutex<Router> = Mutex::new(HashMap::new());
}


pub fn register_route(method:&'static str,path:&'static str,handler: impl HTTPHandler)->bool{
    let lock_result = ROUTER.lock();
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
    }
}