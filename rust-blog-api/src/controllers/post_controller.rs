use rocket::{http::Status, serde::json::Json};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{validators::{CreatePostSchema, Token, UpdatePostSchema}, models::Post, services::{post_service::{create_post_service, get_posts_user_service, get_posts_service, get_post_service, update_post_service, delete_post_service}, category_service::get_category_service}};

#[post("/post", data="<post>")]
pub async fn create_post_controller(token_data: Token, pool: &rocket::State<PgPool>, post: Json<CreatePostSchema<'_>> )-> Result<Json<Post>, Status>{
    
    let category_query = get_category_service(pool, post.category_id).await;
   
    let category = match category_query{
        Ok(cat) => cat,
        Err(err) => {
            println!("{}", err);
           return Err(Status::InternalServerError)
        },
    };
   
    let user_id = Uuid::parse_str(&token_data.to_string().as_str());
    
    let user_uuid = match user_id{
        Ok(user_uuid) => user_uuid,
        Err(err) => {
            println!("{}", err);
           return Err(Status::InternalServerError)
        },
    };
 
    let new_post = CreatePostSchema{ title: &post.title, body: &post.body, user_id: Some(user_uuid), category_id: category.pk_category_id};
    let post = create_post_service(pool, new_post).await;
    match post{
        Err(err) => {
            println!("{}", err);
            Err(Status::InternalServerError)
        },
        _ => Ok(Json(post.unwrap())),
    }
}


#[get("/post/user")]
pub async fn get_user_posts_controller(token_data: Token, pool: &rocket::State<PgPool>)-> Result<Json<Vec<Post>>, Status>{
  
    let user_id = Uuid::parse_str(&token_data.to_string().as_str());
    
    let user_uuid = match user_id{
        Ok(user_uuid) => user_uuid,
        Err(err) => {
            println!("{}", err);
           return Err(Status::InternalServerError)
        },
    };
    let posts_query = get_posts_user_service(pool, user_uuid).await;
   
    match posts_query{
        Ok(p) => Ok(Json(p)),
        Err(err) => {
            println!("{}", err);
           return Err(Status::InternalServerError)
        },  
    }
}

#[get("/post")]
pub async fn get_posts_controller(_token_data: Token, pool: &rocket::State<PgPool>)-> Result<Json<Vec<Post>>, Status>{
    let posts_query = get_posts_service(pool).await;
   
    match posts_query{
        Ok(p) => Ok(Json(p)),
        Err(err) => {
            println!("{}", err);
           return Err(Status::InternalServerError)
        },  
    }
}


#[put("/post", data="<post>")]
pub async fn update_post_controller(token_data: Token, pool: &rocket::State<PgPool>, post: Json<UpdatePostSchema<'_>> )-> Result<Json<Post>, Status>{
    
    let category_query = get_category_service(pool, post.category_id).await;
   
    let category = match category_query{
        Ok(cat) => cat,
        Err(err) => {
            println!("{}", err);
           return Err(Status::InternalServerError)
        },
    };
   
    let user_id = Uuid::parse_str(&token_data.to_string().as_str());
    
    let user_uuid = match user_id{
        Ok(user_uuid) => user_uuid,
        Err(err) => {
            println!("{}", err);
           return Err(Status::InternalServerError)
        },
    };

    let post_query = get_post_service(pool, post.post_id).await;
    match post_query{
        Ok(post) => {
            if post.user_id == user_uuid{
                post
            }else{
                return Err(Status::BadRequest)
            }
        },
        Err(err) => {
            println!("{}", err);
            return Err(Status::InternalServerError)
        },
    };
    
    let post_update = UpdatePostSchema{ post_id: post.post_id, title: &post.title, body: &post.body, category_id: category.pk_category_id};
    
    let post = update_post_service(pool, post_update).await;
    match post{
        Err(err) => {
            println!("{}", err);
            Err(Status::InternalServerError)
        },
        _ => Ok(Json(post.unwrap())),
    }
}

#[delete("/post/<post_id>")]
pub async fn delete_post_controller(pool: &rocket::State<PgPool>, post_id: String)->Result<Json<Post>,Status>{
    let post_uuid_query = Uuid::parse_str(post_id.as_str());

    let post_uuid = match post_uuid_query{
        Ok(p_uuid) => p_uuid,
        Err(err) => {
            println!("{}", err);
           return Err(Status::InternalServerError)
        },
    }; 
    let post_deleted = delete_post_service(pool, post_uuid).await;
    match post_deleted{
        Ok(post) => Ok(Json(post)),
        Err(err) => {
            println!("{}", err);
           return Err(Status::InternalServerError)
        },
    }
}

