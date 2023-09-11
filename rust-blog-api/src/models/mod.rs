use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[allow(non_snake_case)]
pub struct Category{
    pub pk_category_id: Uuid,
    pub category_name: String 
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[allow(non_snake_case)]
pub struct Credential{
    pub credential_id: Uuid,
    pub is_author: bool,
    pub is_admin: bool,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[allow(non_snake_case)]
pub struct User{
    pub user_id: Uuid,
    pub credential_id: Uuid,
    pub username: String,
    pub password: String,
    pub fullname: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updateAt")]
    pub update_at: Option<chrono::DateTime<chrono::Utc>>
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[allow(non_snake_case)]
pub struct Post{
    pub post_id: Uuid,
    pub user_id: Uuid,
    pub category_id: Uuid,
    pub title: String,
    pub body: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updateAt")]
    pub update_at: Option<chrono::DateTime<chrono::Utc>>
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[allow(non_snake_case)]
pub struct Comment{
    pub comment_id: Uuid,
    pub post_id: Uuid,
    pub user_id: Uuid,
    pub body: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updateAt")]
    pub update_at: Option<chrono::DateTime<chrono::Utc>>
}