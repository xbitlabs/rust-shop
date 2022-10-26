use std::any::Any;
use crate::{RequestCtx, ResponseBuilder, EndpointResult, StatusCode};
use crate::core::parse_request_json;
use crate::service::product_category_service::ProductCategoryService;
use anyhow::anyhow;
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use crate::core::parse_form_params;
use rust_shop_std::FormParser;
use hyper::Body;
use hyper::Request;

#[derive(FormParser,serde::Deserialize,serde::Serialize,Debug)]
pub struct Test{
    pub name:Option<String>,
    pub age:Option<u32>,
    pub address:Option<String>,
    pub time:Option<NaiveDateTime>,
    pub date:Option<NaiveDate>,
    pub a:Option<bool>,
    pub b:Option<i32>,
    pub c:Option<u32>,
    pub d:Option<i64>,
    pub e:Option<u64>,
    pub f:Option<f64>,
    pub h:Option<f32>,
    pub i:Option<isize>,
    pub j:Option<usize>,
    pub k:Option<i8>,
    pub l:Option<u8>,
    pub m:Option<i16>,
    pub n:Option<u16>,
    pub o:Option<i128>,
    pub p:Option<u128>,
}

pub struct IndexController;
impl IndexController {
    pub async fn index(ctx: RequestCtx) -> anyhow::Result<hyper::Response<hyper::Body>> {
        let query = ctx.query_params.get("query");
        if query.is_some() {
            println!("query = {}",query.unwrap());
        }
        let test_form_parser : TestFormParser = Test::build_form_parser();
        let test:Test = test_form_parser.parse(ctx.request).await?;
        println!("{:?}",test);
        //let name = ctx.router_params.find("name").unwrap_or("world");
        let rows = ProductCategoryService::list_all_categories().await;
        match rows {
            Ok(data)=>{
                let endpoint_result = EndpointResult::ok_with_payload("".to_string(), data);
                Ok(ResponseBuilder::with_endpoint_result(&endpoint_result))
            }
            Err(e)=>{
                let endpoint_result:EndpointResult<String> = EndpointResult::server_error("内部服务器错误".to_string());
                Ok(ResponseBuilder::with_endpoint_result(&endpoint_result))
            }
        }

    }

}
#[test]
fn test(){
    let dd = NaiveDateTime::parse_from_str("2014-11-28 12:00:09", "%Y-%m-%d %H:%M:%S");
    let date = Utc::now().date_naive();

    if dd.is_err(){
        println!("{:?}",dd);
    }else {
        println!("{:?}",dd.unwrap());
        println!("{:?}",date);
    }
}