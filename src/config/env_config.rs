
#[derive(Serialize, Deserialize)]
pub struct Profiles {
    pub active: String,
}

#[derive(Serialize, Deserialize)]
pub struct EnvConfig {
    pub profiles: Profiles,
}

#[derive(Debug,Serialize, Deserialize)]
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