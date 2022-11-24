use crate::app_config::load_mod_config;
use crate::db::SqlCommandExecutor;
use crate::entity::UserJwt;
use crate::id_generator::ID_GENERATOR;
use anyhow::anyhow;
use chrono::NaiveDateTime;
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use lazy_static::lazy_static;
use log::info;
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlArguments;
use sqlx::{Arguments, Database, Executor, MySql};
use std::thread;
use time::Duration;
use time::OffsetDateTime;
use uuid::Uuid;

pub mod jwt_date_format {
    //! Custom serialization of OffsetDateTime to conform with the JWT spec (RFC 7519 section 2, "Numeric Date")
    use serde::{self, Deserialize, Deserializer, Serialize, Serializer};
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
    pub token_id: String,
    //用户标识
    pub user_id: i64,
    pub sub: String,
    ///token颁发时间
    #[serde(with = "jwt_date_format")]
    pub iat: OffsetDateTime,
    ///失效时间
    #[serde(with = "jwt_date_format")]
    pub exp: OffsetDateTime,
}

impl Claims {
    pub fn new(
        token_id: String,
        user_id: i64,
        sub: String,
        iat: OffsetDateTime,
        exp: OffsetDateTime,
    ) -> Self {
        Claims {
            token_id,
            user_id,
            sub,
            iat,
            exp,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessToken {
    pub access_token: String,
    pub refresh_token: String,
    pub exp: i64,
}

#[async_trait::async_trait]
pub trait JwtService {
    async fn grant_access_token(&mut self, user_id: i64) -> anyhow::Result<AccessToken>;
    async fn decode_access_token(&self, access_token: String) -> anyhow::Result<Claims>;
    async fn decode_refresh_token(&self, refresh_token: String) -> anyhow::Result<Claims>;
    async fn refresh_token(&mut self, refresh_token: String) -> anyhow::Result<AccessToken>;
    async fn remove_access_token(&mut self, access_token: String) -> anyhow::Result<bool>;
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct JwtConfig {
    pub secret: String,
    pub sub: String,
    pub access_token_validity: i64,
    pub refresh_token_validity: i64,
}

lazy_static! {
    ///
    /// 全局配置
    ///
    pub static ref JWT_CONFIG: JwtConfig = load_mod_config(String::from("jwt")).unwrap();
}

pub struct DefaultJwtService<'r, 'a, 'b> {
    sql_command_executor: &'r mut SqlCommandExecutor<'a, 'b>,
}

impl<'r, 'a, 'b> DefaultJwtService<'r, 'a, 'b> {
    pub fn new(
        sql_command_executor: &'r mut SqlCommandExecutor<'a, 'b>,
    ) -> Box<dyn JwtService + Send + Sync + 'r> {
        Box::new(DefaultJwtService {
            sql_command_executor,
        })
    }
}

#[async_trait::async_trait]
impl<'r, 'a, 'b> JwtService for DefaultJwtService<'r, 'a, 'b> {
    async fn grant_access_token(&mut self, user_id: i64) -> anyhow::Result<AccessToken> {
        let jwt_config = &JWT_CONFIG;
        let iat = OffsetDateTime::now_utc();
        let access_token_exp =
            iat + Duration::days(jwt_config.access_token_validity / 60 / 60 / 24);
        let refresh_token_exp =
            iat + Duration::days(jwt_config.refresh_token_validity / 60 / 60 / 24);
        let token_id = Uuid::new_v4().to_string();
        let access_token_claims = Claims {
            token_id: token_id.as_str().to_string(),
            user_id,
            sub: JWT_CONFIG.sub.as_str().to_string(),
            iat,
            exp: access_token_exp,
        };
        let refresh_token_claims = Claims {
            token_id: token_id.as_str().to_string(),
            user_id,
            sub: JWT_CONFIG.sub.as_str().to_string(),
            iat,
            exp: refresh_token_exp,
        };
        let access_token = encode(
            &Header::default(),
            &access_token_claims,
            &EncodingKey::from_secret(JWT_CONFIG.secret.as_ref()),
        )?;
        let refresh_token = encode(
            &Header::default(),
            &refresh_token_claims,
            &EncodingKey::from_secret(JWT_CONFIG.secret.as_ref()),
        )?;

        //let conn_pool = self.mysql_pool_manager.get_pool();
        let user_jwt_id = ID_GENERATOR.lock().unwrap().real_time_generate();
        let mut args = MySqlArguments::default();
        args.add(user_jwt_id);
        args.add(user_id);
        args.add(token_id.as_str().to_string());
        args.add(access_token.as_str().to_string());
        args.add(refresh_token.as_str().to_string());
        args.add(NaiveDateTime::from_timestamp(iat.unix_timestamp(), 0));

        let rows_affected = self.sql_command_executor.execute_with("insert into user_jwt(id,user_id,token_id,access_token,refresh_token,issue_time) values(?,?,?,?,?,?)", args).await?;
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
            Ok(AccessToken {
                access_token,
                refresh_token,
                exp: jwt_config.access_token_validity,
            })
        } else {
            Err(anyhow!("grant_access_token failed!"))
        };
    }

    async fn decode_access_token(&self, access_token: String) -> anyhow::Result<Claims> {
        let access_token_claims: TokenData<Claims> = decode(
            access_token.as_str(),
            &DecodingKey::from_secret(JWT_CONFIG.secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        )?;
        Ok(access_token_claims.claims)
    }

    async fn decode_refresh_token(&self, refresh_token: String) -> anyhow::Result<Claims> {
        let token_data: TokenData<Claims> = decode(
            refresh_token.as_str(),
            &DecodingKey::from_secret(JWT_CONFIG.secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        )?;
        Ok(token_data.claims)
    }

    async fn refresh_token(&mut self, refresh_token: String) -> anyhow::Result<AccessToken> {
        println!("当前线程id={:?}", thread::current().id());
        let decode_refresh_token = self.decode_refresh_token(refresh_token).await?;

        //let conn_pool = self.mysql_pool_manager.get_pool();
        let mut args = MySqlArguments::default();
        args.add(decode_refresh_token.user_id);
        args.add(decode_refresh_token.token_id.clone());

        let user_jwt_iter: Option<UserJwt> = self
            .sql_command_executor
            .find_option_with(
                "select * from user_jwt where user_id=? and token_id=?",
                args,
            )
            .await?;

        if user_jwt_iter.is_none() {
            return Err(anyhow!("refresh_token不存在"));
        }

        let mut args = MySqlArguments::default();
        args.add(decode_refresh_token.user_id);
        args.add(decode_refresh_token.token_id);

        let rows_affected = self
            .sql_command_executor
            .execute_with("delete from user_jwt where user_id=? and token_id=?", args)
            .await?;
        info!("refresh_token删除旧token影响行数：{}", rows_affected);
        self.grant_access_token(decode_refresh_token.user_id).await
    }

    async fn remove_access_token(&mut self, access_token: String) -> anyhow::Result<bool> {
        let decode_access_token = self.decode_access_token(access_token).await?;
        //let pool = self.mysql_pool_manager.get_pool();
        let mut args = MySqlArguments::default();
        args.add(decode_access_token.user_id);
        args.add(decode_access_token.token_id);

        let rows_affected = self
            .sql_command_executor
            .execute_with("delete from user_jwt where user_id=? and token_id=?", args)
            .await?;
        Ok(rows_affected > 0)
    }
}
macro_rules! aw {
    ($e:expr) => {
        tokio_test::block_on($e)
    };
}
