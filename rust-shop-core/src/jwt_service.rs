use std::collections::HashMap;
use std::thread;
use anyhow::anyhow;
use chrono::NaiveDateTime;
use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, TokenData, Validation};
use lazy_static::lazy_static;
use log::info;
use sqlx::{Arguments, Database, Executor, MySql};
use time::{Duration, OffsetDateTime};
use uuid::Uuid;
use serde::Serialize;
use serde::Deserialize;
use sqlx::mysql::MySqlArguments;
use crate::jwt::{AccessToken, Claims, JwtService};
use crate::app_config::load_mod_config;
use crate::db::SqlCommandExecutor;
use crate::id_generator::ID_GENERATOR;
use crate::entity::UserJwt;
use crate::security::LoadUserService;


#[derive(serde::Serialize, serde::Deserialize)]
pub struct JwtConfig{
    pub secret:String,
    pub sub:String,
    pub access_token_validity:i64,
    pub refresh_token_validity:i64
}


lazy_static! {
    ///
    /// 全局配置
    ///
    pub static ref JWT_CONFIG: JwtConfig = load_mod_config(String::from("jwt")).unwrap();
}

pub struct DefaultJwtService<'a,'b>{
    sql_command_executor: &'b mut SqlCommandExecutor<'a,'b>
}

impl <'a,'b> DefaultJwtService<'a,'b> {
    pub fn new(sql_command_executor: &'b mut SqlCommandExecutor<'a,'b>) -> Box<dyn JwtService + Send + Sync + 'b> {
        Box::new(DefaultJwtService {
            sql_command_executor
        })
    }
}

#[async_trait::async_trait]
impl <'a,'b> JwtService for DefaultJwtService<'a,'b> {
    async fn grant_access_token(&mut self, user_id: i64) -> anyhow::Result<AccessToken> {
        let jwt_config = &JWT_CONFIG;
        let iat = OffsetDateTime::now_utc();
        let access_token_exp = iat + Duration::days(jwt_config.access_token_validity / 60 / 60 / 24);
        let refresh_token_exp = iat + Duration::days(jwt_config.refresh_token_validity / 60 / 60 / 24);
        let token_id = Uuid::new_v4().to_string();
        let access_token_claims = Claims{
            token_id : token_id.as_str().to_string(),
            user_id,
            sub : JWT_CONFIG.sub.as_str().to_string(),
            iat,
            exp: access_token_exp
        };
        let refresh_token_claims = Claims{
            token_id : token_id.as_str().to_string(),
            user_id,
            sub : JWT_CONFIG.sub.as_str().to_string(),
            iat,
            exp: refresh_token_exp
        };
        let access_token =  encode(&Header::default(), &access_token_claims, &EncodingKey::from_secret(JWT_CONFIG.secret.as_ref()))?;
        let refresh_token = encode(&Header::default(), &refresh_token_claims, &EncodingKey::from_secret(JWT_CONFIG.secret.as_ref()))?;

        //let conn_pool = self.mysql_pool_manager.get_pool();
        let user_jwt_id = ID_GENERATOR.lock().unwrap().real_time_generate();
        let mut args = MySqlArguments::default();
        args.add(user_jwt_id);
        args.add(user_id);
        args.add(token_id.as_str().to_string());
        args.add(access_token.as_str().to_string());
        args.add(refresh_token.as_str().to_string());
        args.add(NaiveDateTime::from_timestamp(iat.unix_timestamp(),0),);

        let rows_affected = self.sql_command_executor.execute_with("insert into user_jwt(id,user_id,token_id,access_token,refresh_token,issue_time) values(?,?,?,?,?,?)",args).await?;
        /*sqlx::query!(r#"insert into user_jwt(id,user_id,token_id,access_token,refresh_token,issue_time) values(?,?,?,?,?,?)"#,
                    user_jwt_id,
                    user_id,
                    token_id.as_str().to_string(),
                    access_token.as_str().to_string(),
                    refresh_token.as_str().to_string(),
                    NaiveDateTime::from_timestamp(iat.unix_timestamp(),0),
             )
            .execute(conn_pool).await?
            .rows_affected();*/
        return if rows_affected > 0 {
            Ok(AccessToken{
                access_token,
                refresh_token,
                exp:jwt_config.access_token_validity,
            })
        }else {
            Err(anyhow!("grant_access_token failed!"))
        }

    }

    async fn decode_access_token(&self,access_token: String) -> anyhow::Result<Claims> {
        let access_token_claims:TokenData<Claims> = decode(access_token.as_str(),
                                                           &DecodingKey::from_secret(JWT_CONFIG.secret.as_ref()),
                                                           &Validation::new(Algorithm::HS256))?;
        Ok(access_token_claims.claims)
    }

    async fn decode_refresh_token(&self,refresh_token: String) -> anyhow::Result<Claims> {
        let token_data:TokenData<Claims> = decode(refresh_token.as_str(),
                                                           &DecodingKey::from_secret(JWT_CONFIG.secret.as_ref()),
                                                           &Validation::new(Algorithm::HS256))?;
        Ok(token_data.claims)
    }

    async fn refresh_token(&mut self, refresh_token: String) -> anyhow::Result<AccessToken> {
        println!("当前线程id={:?}",thread::current().id());
        let decode_refresh_token = self.decode_refresh_token(refresh_token).await?;

        //let conn_pool = self.mysql_pool_manager.get_pool();
        let mut args = MySqlArguments::default();
        args.add(decode_refresh_token.user_id);
        args.add(decode_refresh_token.token_id.clone());

        let user_jwt_iter:Option<UserJwt> = self.sql_command_executor.find_option_with("select * from user_jwt where user_id=? and token_id=?",
                args).await?;

        if user_jwt_iter.is_none() {
            return Err(anyhow!("refresh_token不存在"));
        }

        let mut args = MySqlArguments::default();
        args.add(decode_refresh_token.user_id);
        args.add(decode_refresh_token.token_id);

        let rows_affected = self.sql_command_executor.execute_with("delete from user_jwt where user_id=? and token_id=?",
                args).await?;
        info!("refresh_token删除旧token影响行数：{}",rows_affected);
        self.grant_access_token(decode_refresh_token.user_id).await
    }



    async fn remove_access_token(&mut self, access_token: String) -> anyhow::Result<bool> {
        let decode_access_token = self.decode_access_token(access_token).await?;
        //let pool = self.mysql_pool_manager.get_pool();
        let mut args = MySqlArguments::default();
        args.add(decode_access_token.user_id);
        args.add(decode_access_token.token_id);

        let rows_affected = self.sql_command_executor.execute_with("delete from user_jwt where user_id=? and token_id=?",args)
            .await?;
        Ok(rows_affected > 0)
    }
}
macro_rules! aw {
  ($e:expr) => {
      tokio_test::block_on($e)
  };
}
/*#[test]
fn test_jwt()->Result<(),RustShopError>{
    let jwt_service = RustShopJwtService::new();
    let token_aw = jwt_service.grant_access_token(1111);
    let token_obj = aw!(token_aw)?;
    let token = &token_obj.access_token;
    let c =  aw!(jwt_service.decode_access_token(token.to_string()))?;
    //let ref_token_str = &token_obj.refresh_token;
    //let ref_token = aw!(jwt_service.refresh_token(ref_token_str.to_string()))?;
    //println!("{:?}",ref_token);
    //assert_eq!(c.user_id, 1111);
   // println!("user_id={}",c.user_id);
    //println!("{:?}",token_obj);
    //let res = aw!(jwt_service.remove_access_token(ref_token.access_token))?;
    //if res {
    //    println!("{}","删除成功");
    //}
    Ok(())
}*/