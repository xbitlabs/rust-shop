use std::fs::read_to_string;

use schemars::schema::RootSchema;
use serde_json::to_string_pretty;
use serde_yaml::from_str as yaml_from_str;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Profiles {
    pub active: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct EnvConfig {
    pub profiles: Profiles,
}

///
/// 加载环境配置
///
///
pub fn load_env_conf() -> Option<EnvConfig> {
    let schema = yaml_from_str::<RootSchema>(
        &read_to_string("application.yml").expect("Error loading configuration file resources/application.yml, please check the configuration!"),
    );
    return match schema {
        Ok(json) => {
            let data = to_string_pretty(&json).expect("config/application.yml file data error！");
            let p: EnvConfig =
                yaml_from_str(&*data).expect("Failed to transfer JSON data to EnvConfig object！");
            return Some(p);
        }
        Err(err) => {
            println!("{}", err);
            None
        }
    };
}

///
/// 根据环境配置加载全局配置
/// action  dev 开始环境 test 测试环境 prod 生产环境
///
pub fn load_app_config<TConfig: for<'a> serde::Deserialize<'a>>(action: String) -> Option<TConfig> {
    let path = format!("application-{}.yml", &action);
    let schema = yaml_from_str::<RootSchema>(&read_to_string(&path).unwrap_or_else(|_| {
        panic!(
            "Error loading configuration file {}, please check the configuration!",
            &path
        )
    }));
    return match schema {
        Ok(json) => {
            let data = to_string_pretty(&json).unwrap_or_else(|_| {
                panic!(
                    "{} file data error！, please check the configuration!",
                    path
                )
            });
            let p = yaml_from_str(&*data)
                .expect("Failed to transfer JSON data to BriefProConfig object！");
            return Some(p);
        }
        Err(err) => {
            println!("{}", err);
            None
        }
    };
}

///
/// 根据环境配置加载全局配置
/// action  dev 开始环境 test 测试环境 prod 生产环境
///
pub fn load_mod_config<TConfig: for<'a> serde::Deserialize<'a>>(
    mod_config: String,
) -> Option<TConfig> {
    let path = format!("{}.yml", &mod_config);
    let schema = yaml_from_str::<RootSchema>(&read_to_string(&path).unwrap_or_else(|_| {
        panic!(
            "Error loading configuration file {}, please check the configuration!",
            &path
        )
    }));
    return match schema {
        Ok(json) => {
            let data = to_string_pretty(&json).unwrap_or_else(|_| {
                panic!(
                    "{} file data error！, please check the configuration!",
                    path
                )
            });
            let p = yaml_from_str(&*data)
                .expect("Failed to transfer JSON data to BriefProConfig object！");
            return Some(p);
        }
        Err(err) => {
            println!("{}", err);
            None
        }
    };
}

///
/// 先加载环境配置 在根据当前加载的环境 去加载相应的信息
///
pub fn load_conf<TConfig: for<'a> serde::Deserialize<'a>>() -> Option<TConfig> {
    println!("{}", "Load profile");
    if let Some(init) = load_env_conf() {
        println!("{}", init.profiles.active);
        return load_app_config(init.profiles.active);
    }
    None
}
