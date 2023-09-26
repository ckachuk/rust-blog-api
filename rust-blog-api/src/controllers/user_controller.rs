use rocket::{serde::json::Json, http::Status};
use sqlx::PgPool;

use crate::{validators::{CreateUserSchema, LoginSchema, UpdateUserPasswordSchema}, models::User, services::user_service::{sign_up_service, login_service, update_password_service}};


#[post("/signup", data = "<post>")]
pub async fn sign_up_controller(pool: &rocket::State<PgPool>, post: Json<CreateUserSchema>)-> Result<Json<User>, Status>{
    
    let new_user = sign_up_service(&pool, post).await;

    match new_user{
        Err(err) => {
            if err.to_string().contains("duplicate key value violates unique constraint"){
                return Err(Status::BadRequest)
            }
            Err(Status::InternalServerError)
        }, 
        _ => Ok(Json(new_user.unwrap()))
    }
} 


#[post("/login", data = "<post>")]
pub async fn login_controller(pool: &rocket::State<PgPool>, post: Json<LoginSchema>)-> Result<Json<String>, Status>{
    let query_result = login_service(pool, post).await;

    match query_result{
        Err(err) => {
            println!("{}",err);
            if err.to_string().contains("duplicate key value violates unique constraint"){
                return Err(Status::BadRequest)
            }
            if err.to_string().contains("invalid password"){
                return Err(Status::Unauthorized)
            }
            Err(Status::InternalServerError)
            
        }, 
        Ok(())=>{
           Ok(Json("valid".to_owned()))
        },
        
    }
}

#[post("/change_password", data = "<post>")]
pub async fn change_password(pool: &rocket::State<PgPool>, post: Json<UpdateUserPasswordSchema>)-> Result<Json<String>, Status>{

    if post.old_password == post.new_password{
        return Err(Status::BadRequest)
    }

    let user_modified = update_password_service(pool, post).await;
    
    match user_modified{
        Err(err) => {
            if err.to_string().contains("invalid password"){
                return Err(Status::BadRequest)
            }
            println!("{}",err);
            Err(Status::InternalServerError)
        }, 
        _ => Ok(Json(String::from("Password has been updated")))
    }

}
