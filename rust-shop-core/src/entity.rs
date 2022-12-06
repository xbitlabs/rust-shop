use chrono::{DateTime, Utc};

#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize, Debug)]
pub struct User {
    pub id: i64,
    pub username: Option<String>,
    pub password: Option<String>,
    pub phone_number: Option<String>,
    pub is_phone_number_verified: Option<u8>,
    pub wx_open_id: Option<String>,
    pub enable: u8,
    #[serde(with = "crate::serde_utils::date_format")]
    pub created_time: DateTime<Utc>,
}

#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize, Debug)]
pub struct UserJwt {
    pub id: i64,
    pub user_id: i64,
    pub token_id: String,
    pub access_token: String,
    pub refresh_token: String,
    #[serde(with = "crate::serde_utils::date_format")]
    pub issue_time: DateTime<Utc>,
}
#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize, Debug)]
pub struct AdminPermission {
    pub id: i64,
    pub admin_permission_group_id: i64,
    pub title: String,
    pub code: String,
    pub url: String,
}
#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize, Debug)]
pub struct AdminPermissionGroup {
    pub id: i64,
    pub name: String,
    pub parent_id: Option<i64>,
}
#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize, Debug)]
pub struct AdminRole {
    pub id: i64,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
}
#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize, Debug)]
pub struct AdminRolePermission {
    pub admin_role_id: i64,
    pub admin_permission_id: i64,
}
#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize, Debug)]
pub struct AdminUser {
    pub id: i64,
    pub username: String,
    pub password: String,
    #[serde(with = "crate::serde_utils::date_format")]
    pub created_time: DateTime<Utc>,
}
#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize, Debug)]
pub struct AdminUserRole {
    pub admin_user_id: i64,
    pub admin_role_id: i64,
}
