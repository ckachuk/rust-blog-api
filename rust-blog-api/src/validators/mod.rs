use std::fmt;
use jsonwebtoken::{decode, DecodingKey,  Validation, Algorithm};

use rocket::{request::FromRequest, request::Outcome, http::Status, request::Request};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateCategorySchema{
    pub name: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserSchema{
    pub username: String,
    pub password: String,
    pub fullname: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginSchema<'a>{
    pub username: &'a str,
    pub password: &'a str
}


#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateUserPasswordSchema<'a>{
    pub username: &'a str,
    pub old_password: &'a str,
    pub new_password: &'a str
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePostSchema<'a>{
    pub title: &'a str,
    pub body: &'a str,
    pub user_id: Option<Uuid>,
    pub category_id: Uuid 
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdatePostSchema<'a>{
    pub post_id: Uuid,
    pub title: &'a str,
    pub body: &'a str,
    pub category_id: Uuid 
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateCommentSchema<'a>{
    pub post_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub body: &'a str,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct TokenClaims{
    pub exp: usize,
    pub sub: String,
}


pub struct Token(String);

#[derive(Debug)]
pub enum ApiTokenError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = ApiTokenError;
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let secret = std::env::var("TOKEN_SECRET");
        if secret.is_err(){
            return Outcome::Failure((Status::InternalServerError, ApiTokenError::Invalid));
        }
        
        let token: Option<_> = req.headers().get_one("token");
        match token {
            Some(token) => {
                let token_decoded = decode::<TokenClaims>(&token, &DecodingKey::from_secret(secret.unwrap().as_bytes()), &Validation::new(Algorithm::HS512));
                
                if token_decoded.is_err(){
                    return Outcome::Failure((Status::Unauthorized, ApiTokenError::Invalid));
                }
                let user_id = token_decoded.unwrap().claims.sub;
                
                Outcome::Success(Token(user_id))
            }
            None => Outcome::Failure((Status::Unauthorized, ApiTokenError::Missing)),
        }
    }
}

impl fmt::Display for Token {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.0)
    }
}