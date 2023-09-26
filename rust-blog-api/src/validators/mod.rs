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
pub struct LoginSchema{
    pub user_id: Uuid,
    pub username: String,
    pub password: String
}


#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateUserPasswordSchema{
    pub user_id: Uuid,
    pub old_password: String,
    pub new_password: String
}