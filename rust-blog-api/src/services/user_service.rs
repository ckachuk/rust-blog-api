
use chrono::Utc;
use rocket::serde::json::Json;
use sqlx::PgPool;
use uuid::Uuid;
use jsonwebtoken::{encode, Algorithm,  EncodingKey, Header};
use crate::{models::{User, Credential}, validators::{CreateUserSchema, LoginSchema, UpdateUserPasswordSchema, TokenClaims}};
use argon2;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString, Error
    },
    Argon2
};


pub async fn sign_up_service(pool: &rocket::State<PgPool>, post: Json<CreateUserSchema>)-> Result<User, anyhow::Error>{
    let user_uuid: Uuid = Uuid::new_v4();
    let credential = create_credentials_service(pool).await?;
    let password_hashed = hash_password_service(post.password.to_string()).await?;
    let datetime = Utc::now();
    let user_data = User{
        pk_user_id: user_uuid,
        credential_id: credential.pk_credential_id,
        password: password_hashed,
        username: post.username.to_string(),
        fullname: post.fullname.to_string(),
        token: Some("".to_owned()),
        create_at: Some(datetime),
        update_at: Some(datetime)
    };
    let query_result: User = sqlx::query_as!(User, r#"INSERT INTO users(pk_user_id, credential_id, username, password, fullname) VALUES($1, $2, $3, $4, $5) RETURNING *"#,
    user_data.pk_user_id, user_data.credential_id, user_data.username, user_data.password, user_data.fullname).fetch_one(&**pool).await?;

    Ok(query_result)
    
}


async fn create_credentials_service(pool: &rocket::State<PgPool>)-> Result<Credential, sqlx::Error>{
    let credential_uuid: Uuid = Uuid::new_v4();
    
    let query_result: Credential = sqlx::query_as!(Credential, r#"INSERT INTO credentials(pk_credential_id, is_author, is_admin) VALUES($1, $2, $3) RETURNING pk_credential_id, is_author, is_admin"#, credential_uuid, Some(false), Some(false))
    .fetch_one(&**pool)
    .await?;

    
    Ok(Credential{pk_credential_id: query_result.pk_credential_id, is_author: query_result.is_author, is_admin: query_result.is_admin})
}


async fn hash_password_service(password: String)-> Result<String, Error>{
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?.to_string();

    Ok(password_hash) 
}


pub async fn update_password_service(pool: &rocket::State<PgPool>, post: Json<UpdateUserPasswordSchema<'_>>) -> Result<User, anyhow::Error>{
    
    verify_password(&post.username, &post.old_password, pool).await?;

    let new_password_parsed_hash = hash_password_service(post.new_password.to_string()).await?;

    let user_updated = sqlx:: query_as!(User, r#"UPDATE users SET password=$1 WHERE users.username=$2 RETURNING *"#, new_password_parsed_hash, &post.username)
    .fetch_one(&**pool)
    .await?;

    Ok(user_updated)
}


pub async fn login_service(pool: &rocket::State<PgPool>, post: Json<LoginSchema<'_>>)-> Result<String, anyhow::Error>{
    let user = verify_password(&post.username, &post.password, pool).await?;
   
    let token_claims = TokenClaims{ sub: user.pk_user_id.to_string(), exp: 10000000000};
    let token_headers =  Header { kid: Some("signing_key".to_owned()), alg: Algorithm::HS512, ..Default::default() };
    let secret = std::env::var("TOKEN_SECRET")?;
    let token = encode(&token_headers, &token_claims, &EncodingKey::from_secret(secret.as_bytes()))?;

    sqlx:: query_as!(User, r#"UPDATE users SET token=$1 WHERE users.username=$2 RETURNING *"#, &token, &post.username)
    .fetch_one(&**pool)
    .await?;

    Ok(token)
}

pub async fn verify_password(username: &str, user_password: &str, pool: &rocket::State<PgPool>)-> Result<User, anyhow::Error>{
    let user = sqlx::query_as!(User, r#"SELECT * FROM users WHERE users.username=$1"#, &username)
    .fetch_one(&**pool)
    .await?;

    let parsed_hash = PasswordHash::new(&user.password)?;
    
    Argon2::default().verify_password(user_password.as_bytes(), &parsed_hash)?;
   
    Ok(user)
}