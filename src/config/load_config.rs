use std::fs::read_to_string;

use lazy_static::lazy_static;
use schemars::schema::RootSchema;

use rust_shop_core::app_config::{load_conf, EnvConfig};

use crate::config::env_config::AppConfig;

lazy_static! {
    ///
    /// 全局配置
    ///
    pub static ref APP_CONFIG: AppConfig = load_conf().unwrap();
}
