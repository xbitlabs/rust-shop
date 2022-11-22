use std::sync::Mutex;

use snowflake::SnowflakeIdGenerator;

lazy_static::lazy_static! {
    pub static ref ID_GENERATOR : Mutex<SnowflakeIdGenerator> = Mutex::new(SnowflakeIdGenerator::new(1, 1));
}
