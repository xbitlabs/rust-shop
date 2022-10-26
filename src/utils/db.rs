use sqlx::{Error, MySql, MySqlPool, Pool};
use crate::APP_CONFIG;

pub async fn get_connection_pool() -> Result<Pool<MySql>, Error> {
    println!("获取mysql连接池");
    let app_config = &APP_CONFIG;
    let conn = format!("mysql://{}:{}@{}:{}/{}?useUnicode=true&characterEncoding=utf8&zeroDateTimeBehavior=convertToNull&useSSL=true&serverTimezone=Asia/Shanghai",
                       app_config.mysql.user,
                       app_config.mysql.password,
                       app_config.mysql.host,
                       app_config.mysql.port,
                       app_config.mysql.db);
    let pool = MySqlPool::connect(conn.as_str());
    pool.await
}