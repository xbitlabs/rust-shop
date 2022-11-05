#[derive(serde::Serialize, serde::Deserialize)]
pub struct AppConfig {
    pub server:Server,
    pub upload:Upload,
    pub static_file:StaticFile,
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
pub struct Server{
    pub port: u32
}