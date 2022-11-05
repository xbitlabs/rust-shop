use schemars::schema::RootSchema;
use std::fs::read_to_string;
use lazy_static::lazy_static;
use rust_shop_core::app_config::{EnvConfig, load_conf};
use crate::config::env_config::{AppConfig};

lazy_static! {
    ///
    /// 全局配置
    ///
    pub static ref APP_CONFIG: AppConfig = load_conf().unwrap();
}


#[test]
fn test_load_env_conf_mysql() {
    let pro = load_conf();
    pro.as_ref().map(|a| {
        println!("mysqlConfig:{}", serde_json::to_string(&a.mysql).unwrap());
    });
}