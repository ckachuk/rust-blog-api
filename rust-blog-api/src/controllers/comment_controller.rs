use rocket::{serde::json::Json, http::Status};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{validators::CreateCommentSchema, models::Comment, services::comment_service::{post_comment_service, get_comment_service, get_comments_service, delete_comment_service}};

#[post("/post/<post_id>/comment", data="<post>")]
pub async fn create_comment_controller(pool: &rocket::State<PgPool>,post_id: String, post: Json<CreateCommentSchema<'_>>)->Result<Json<Comment>,Status>{
    let post_uuid_query = Uuid::parse_str(post_id.as_str());
    
    let post_uuid = match post_uuid_query{
        Ok(p_uuid) => p_uuid,
        Err(err) => {
            println!("{}", err);
           return Err(Status::InternalServerError)
        },
    }; 
    let new_comment = CreateCommentSchema{post_id: Some(post_uuid), user_id: post.user_id, body: post.body};

    let post_comment = post_comment_service(pool, new_comment).await;

    match post_comment{
        Ok(n_comment)=> Ok(Json(n_comment)),
        Err(err)=>{
            println!("{}", err);
            Err(Status::InternalServerError)
        }
    }
}


#[get("/comment/<comment_id>")]
pub async fn get_comment_controller(pool: &rocket::State<PgPool>, comment_id: String)->Result<Json<Comment>,Status>{
    let comment_uuid_query = Uuid::parse_str(comment_id.as_str());
    
    let comment_uuid = match comment_uuid_query{
        Ok(c_uuid) => c_uuid,
        Err(err) => {
            println!("{}", err);
           return Err(Status::InternalServerError)
        },
    }; 
    let comment_query = get_comment_service(pool, comment_uuid).await;
    match comment_query{
        Ok(comment) => Ok(Json(comment)),
        Err(err) => {
            println!("{}", err);
           return Err(Status::InternalServerError)
        },
    }
}


#[get("/post/<post_id>/comment")]
pub async fn get_comments_controller(pool: &rocket::State<PgPool>, post_id: String)->Result<Json<Vec<Comment>>,Status>{
    let post_uuid_query = Uuid::parse_str(post_id.as_str());
    
    let post_uuid = match post_uuid_query{
        Ok(p_uuid) => p_uuid,
        Err(err) => {
            println!("{}", err);
           return Err(Status::InternalServerError)
        },
    }; 
    let comments_query = get_comments_service(pool, post_uuid).await;
    match comments_query{
        Ok(comments) => Ok(Json(comments)),
        Err(err) => {
            println!("{}", err);
           return Err(Status::InternalServerError)
        },
    }
}

#[delete("/comment/<comment_id>")]
pub async fn delete_comment_controller(pool: &rocket::State<PgPool>, comment_id: String)->Result<Json<Comment>,Status>{
    let comment_uuid_query = Uuid::parse_str(comment_id.as_str());

    let comment_uuid = match comment_uuid_query{
        Ok(c_uuid) => c_uuid,
        Err(err) => {
            println!("{}", err);
           return Err(Status::InternalServerError)
        },
    }; 
    let comment_deleted = delete_comment_service(pool, comment_uuid).await;
    match comment_deleted{
        Ok(comment) => Ok(Json(comment)),
        Err(err) => {
            println!("{}", err);
           return Err(Status::InternalServerError)
        },
    }
}


