use std::collections::{BTreeMap, HashMap};
use std::error::Error;
use http::{header, HeaderValue, Response, StatusCode};
use hyper::Body;
use lazy_static::lazy_static;
use serde::Serialize;
use serde_json::{to_value, Value};
use crate::extract::IntoResponse;
use tera::{Context, Result, Tera, try_get_value};
use crate::ResponseBuilder;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec!["html", ".sql"]);
        tera.register_filter("do_nothing", do_nothing_filter);
        tera
    };
}
pub fn do_nothing_filter(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
    let s = try_get_value!("do_nothing_filter", "value", String, value);
    Ok(to_value(&s).unwrap())
}

pub struct ModelAndView{
    view:String,
    models:Context,
}

impl ModelAndView {
    pub fn new(view:String)->Self{
        ModelAndView{
            view,
            models: Context::new()
        }
    }
    pub fn insert<T: Serialize + ?Sized, S: Into<String>>(&mut self, key: S, val: &T) {
        self.models.insert(key.into(), val);
    }
    pub fn get(&self, index: &str) -> Option<&Value> {
        self.models.get(index)
    }
    pub fn remove(&mut self, index: &str) -> Option<Value> {
        self.models.remove(index)
    }
    pub fn contains_key(&self, index: &str) -> bool {
        self.models.contains_key(index)
    }
}

impl IntoResponse for ModelAndView {
    fn into_response(self) -> Response<Body> {
        return match TEMPLATES.render(&self.view, &self.models) {
            Ok(result) => {
                ResponseBuilder::with_status_and_html(StatusCode::OK,result)
            },
            Err(e) => {
                let mut result = String::from(format!("Error: {}\r\n", e));

                let mut cause = e.source();
                while let Some(e) = cause {
                    result = result + &*format!("Reason: {}\r\n", e);
                    cause = e.source();
                }
                ResponseBuilder::with_status_and_html(StatusCode::INTERNAL_SERVER_ERROR,result)
            }
        };
    }
}