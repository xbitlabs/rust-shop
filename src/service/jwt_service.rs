use std::collections::HashMap;
use std::thread;
use anyhow::anyhow;
use chrono::NaiveDateTime;
use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, TokenData, Validation};
use log::info;
use sqlx::Executor;
use time::{Duration, OffsetDateTime};
use uuid::Uuid;
use crate::config::load_config::APP_CONFIG;
use serde::Serialize;
use serde::Deserialize;
use crate::{ID_GENERATOR, MysqlPoolManager};
use crate::entity::entity::UserJwt;

mod jwt_numeric_date {
    //! Custom serialization of OffsetDateTime to conform with the JWT spec (RFC 7519 section 2, "Numeric Date")
    use serde::{self, Deserialize, Deserializer, Serializer};
    use time::OffsetDateTime;

    /// Serializes an OffsetDateTime to a Unix timestamp (milliseconds since 1970/1/1T00:00:00T)
    pub fn serialize<S>(date: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let timestamp = date.unix_timestamp();
        serializer.serialize_i64(timestamp)
    }

    /// Attempts to deserialize an i64 and use as a Unix timestamp
    pub fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
        where
            D: Deserializer<'de>,
    {
        OffsetDateTime::from_unix_timestamp(i64::deserialize(deserializer)?)
            .map_err(|_| serde::de::Error::custom("invalid Unix timestamp value"))
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Claims {
    pub token_id:String,
    //用户标识
    pub user_id:i64,
    pub sub: String,
    ///token颁发时间
    #[serde(with = "jwt_numeric_date")]
    pub iat: OffsetDateTime,
    ///失效时间
    #[serde(with = "jwt_numeric_date")]
    pub exp: OffsetDateTime,
}
impl Claims{
    pub fn new(token_id:String,user_id:i64,sub:String,iat: OffsetDateTime,exp: OffsetDateTime)->Self{
        Claims{
            token_id,
            user_id,
            sub,
            iat,
            exp
        }
    }
}
pub struct JwtConfig{
    pub access_token_validity:i64,
    pub refresh_token_validity:i64,
    pub sub:String,
}
impl JwtConfig {
    pub fn new(access_token_validity:i64,refresh_token_validity:i64,sub:String)->Self{
        JwtConfig{
            access_token_validity,
            refresh_token_validity,
            sub
        }
    }
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessToken{
    pub access_token:String,
    pub refresh_token:String,
    pub exp:i64,
}



pub struct RustShopJwtService<'a,'b>{
    mysql_pool_manager: &'a MysqlPoolManager<'b>
}

impl <'a,'b> RustShopJwtService<'a,'b>{
    pub fn new(mysql_pool_manager:&'b MysqlPoolManager)->Self{
        RustShopJwtService{
            mysql_pool_manager
        }
    }
    pub async fn grant_access_token(&self,user_id: i64) -> anyhow::Result<AccessToken> {
        let jwt_config = self.get_jwt_config().await?;
        let iat = OffsetDateTime::now_utc();
        let access_token_exp = iat + Duration::days(jwt_config.access_token_validity / 60 / 60 / 24);
        let refresh_token_exp = iat + Duration::days(jwt_config.refresh_token_validity / 60 / 60 / 24);
        let token_id = Uuid::new_v4().to_string();
        let access_token_claims = Claims{
            token_id : token_id.as_str().to_string(),
            user_id,
            sub : APP_CONFIG.jwt.sub.as_str().to_string(),
            iat,
            exp: access_token_exp
        };
        let refresh_token_claims = Claims{
            token_id : token_id.as_str().to_string(),
            user_id,
            sub : APP_CONFIG.jwt.sub.as_str().to_string(),
            iat,
            exp: refresh_token_exp
        };
        let access_token =  encode(&Header::default(), &access_token_claims, &EncodingKey::from_secret(APP_CONFIG.jwt.secret.as_ref()))?;
        let refresh_token = encode(&Header::default(), &refresh_token_claims, &EncodingKey::from_secret(APP_CONFIG.jwt.secret.as_ref()))?;

        let conn_pool = self.mysql_pool_manager.get_pool();
        let user_jwt_id = ID_GENERATOR.lock().unwrap().real_time_generate();
        sqlx::query!(r#"insert into user_jwt(id,user_id,token_id,access_token,refresh_token,issue_time) values(?,?,?,?,?,?)"#,
                    user_jwt_id,
                    user_id,
                    token_id.as_str().to_string(),
                    access_token.as_str().to_string(),
                    refresh_token.as_str().to_string(),
                    NaiveDateTime::from_timestamp(iat.unix_timestamp(),0),
             )
            .execute(conn_pool).await?
            .rows_affected();
        Ok(AccessToken{
            access_token,
            refresh_token,
            exp:jwt_config.access_token_validity,
        })
    }

    pub async fn decode_access_token(&self,access_token: String) -> anyhow::Result<Claims> {
        let access_token_claims:TokenData<Claims> = decode(access_token.as_str(),
                                                           &DecodingKey::from_secret(APP_CONFIG.jwt.secret.as_ref()),
                                                           &Validation::new(Algorithm::HS256))?;
        Ok(access_token_claims.claims)
    }

    pub async fn decode_refresh_token(&self,refresh_token: String) -> anyhow::Result<Claims> {
        let token_data:TokenData<Claims> = decode(refresh_token.as_str(),
                                                           &DecodingKey::from_secret(APP_CONFIG.jwt.secret.as_ref()),
                                                           &Validation::new(Algorithm::HS256))?;
        Ok(token_data.claims)
    }

    pub async fn refresh_token(&self,refresh_token: String) -> anyhow::Result<AccessToken> {
        println!("当前线程id={:?}",thread::current().id());
        let decode_refresh_token = self.decode_refresh_token(refresh_token).await?;

        let conn_pool = self.mysql_pool_manager.get_pool();

        let user_jwt_iter = sqlx::query_as!(UserJwt,"select * from user_jwt where user_id=? and token_id=?",
                decode_refresh_token.user_id,
                decode_refresh_token.token_id)
            .fetch_all(conn_pool)
            .await?;
        if user_jwt_iter.len() == 0 {
            return Err(anyhow!("refresh_token不存在"));
        }
        let rows_affected = sqlx::query!("delete from user_jwt where user_id=? and token_id=?",
                decode_refresh_token.user_id,
                decode_refresh_token.token_id)
            .execute(conn_pool)
            .await?
            .rows_affected();
        info!("refresh_token删除旧token影响行数：{}",rows_affected);
        self.grant_access_token(decode_refresh_token.user_id).await
    }

    pub async fn get_jwt_config(&self,) -> anyhow::Result<JwtConfig> {
        Ok(JwtConfig{
            access_token_validity: APP_CONFIG.jwt.access_token_validity,
            refresh_token_validity: APP_CONFIG.jwt.refresh_token_validity,
            sub: APP_CONFIG.jwt.sub.as_str().to_string(),
        })
    }

    pub async fn remove_access_token(&self, access_token: String) -> anyhow::Result<bool> {
        let decode_access_token = self.decode_access_token(access_token).await?;
        let pool = self.mysql_pool_manager.get_pool();
        let rows_affected = sqlx::query!("delete from user_jwt where user_id=? and token_id=?",decode_access_token.user_id,decode_access_token.token_id)
            .execute(pool)
            .await?
            .rows_affected();
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