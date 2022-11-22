use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use route_recognizer::{Params, Router as MethodRouter};

use crate::{BoxHTTPHandler, HTTPHandler};

pub type Router = HashMap<String, MethodRouter<BoxHTTPHandler>>;

static mut GLOBAL_ROUTER: Lazy<Router> = Lazy::new(|| {
    let mut m: Router = HashMap::new();
    m
});

pub fn get_routers() -> &'static mut Router {
    unsafe { &mut GLOBAL_ROUTER }
}

pub fn register_route<'a>(method: String, path: String, handler: impl HTTPHandler) -> bool {
    static MUTEX: Mutex<()> = Mutex::new(());
    let lock_result = MUTEX.lock();
    match lock_result {
        Ok(_) => unsafe {
            GLOBAL_ROUTER
                .entry(method.to_string())
                .or_insert_with(MethodRouter::new)
                .add(path.as_ref(), Box::new(handler));
        },
        Err(e) => {
            panic!("注册路由异常：{}", e);
        }
    }
    true
}
