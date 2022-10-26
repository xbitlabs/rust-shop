#[derive(serde::Serialize, serde::Deserialize)]
pub struct AppConfig {
    pub server:Server,
    pub mysql: Mysql,
    pub upload:Upload,
    pub static_file:StaticFile,
    pub wechat:Wechat,
    pub jwt:Jwt,
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Profiles {
    pub active: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct EnvConfig {
    pub profiles: Profiles,
}

#[derive(Debug,serde::Serialize, serde::Deserialize)]
pub struct Mysql {
    pub host: String,
    pub port: u32,
    pub user: String,
    pub password: String,
    pub db: String,
    pub pool_min_idle: u64,
    pub pool_max_open: u64,
    pub timeout_seconds: u64,
}
#[derive(Debug,serde::Serialize, serde::Deserialize)]
pub struct Upload {
    pub save_path: String,
}
#[derive(Debug,serde::Serialize, serde::Deserialize)]
pub struct StaticFile{
    pub virtual_path:String
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Wechat{
    pub app_id:String,
    pub app_secret:String,
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Jwt{
    pub secret:String,
    pub sub:String,
    pub access_token_validity:i64,
    pub refresh_token_validity:i64
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Server{
    pub port: u32
}