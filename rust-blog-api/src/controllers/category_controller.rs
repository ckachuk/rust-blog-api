
use rocket::http::Status;
use rocket::serde::json::Json;
use sqlx::{self, PgPool};
use uuid::Uuid;
use crate::models::Category;
use crate::services::category_service;
use crate::validators::CreateCategorySchema;

#[get("/category")]
pub async fn get_categories_controller(pool: &rocket::State<PgPool>) -> Result<Json<Vec<Category>>, Status>{
    let query_result: Result<Vec<Category>, sqlx::Error> = category_service::get_categories(pool).await;
    
    if query_result.is_err(){
        return Err(Status::InternalServerError);
    }
    let categories: Vec<Category> = query_result.unwrap();

    let json_response: Json<Vec<Category>> = Json(categories);

    Ok(json_response)
}   


#[get("/category/<category_id>")]
pub async fn get_category_controller(pool: &rocket::State<PgPool>, category_id: String) -> Result<Json<Category>, Status>{
    let category_uuid = Uuid::parse_str(category_id.as_str());

    
    let query_result: Result<Category, sqlx::Error> = category_service::get_category_service(pool, category_uuid.unwrap()).await;
    
    if query_result.is_err(){
        return Err(Status::InternalServerError);
    }
    let category: Category = query_result.unwrap();

    let json_response: Json<Category> = Json(category);

    Ok(json_response)
}  

#[post("/category", data = "<post>")]
pub async fn create_category_controller(pool: &rocket::State<PgPool>, post: Json<CreateCategorySchema>)->  Result<Json<Category>, Status> {
  
    let category: Result<crate::models::Category, sqlx::Error> = category_service::create_category_service(pool, post).await;
    match category {
        Err(err) => {
            if err.to_string().contains("duplicate key value violates unique constraint"){
                return Err(Status::BadRequest)
            }
            Err(Status::InternalServerError)
        },
        _ => Ok(Json(category.unwrap())),
    }
}

#[put("/category/<category_id>", data = "<post>")]
pub async fn update_category_controller(pool: &rocket::State<PgPool>, post: Json<CreateCategorySchema>, category_id: String)->  Result<Json<Category>, Status> {
    let category_uuid = Uuid::parse_str(category_id.as_str());

    let category: Result<crate::models::Category, sqlx::Error> = category_service::update_category_service(pool, post, category_uuid.unwrap()).await;
    match category {
        Err(err) => {
            if err.to_string().contains("duplicate key value violates unique constraint"){
                return Err(Status::BadRequest)
            }
            Err(Status::InternalServerError)
        },
        _ => Ok(Json(category.unwrap())),
    }
}


#[delete("/category/<category_id>")]
pub async fn delete_categoy_controller(pool: &rocket::State<PgPool>, category_id: String) -> Result<Json<Category>, Status>{
    let category_uuid = Uuid::parse_str(category_id.as_str());

    
    let query_result: Result<Category, sqlx::Error> = category_service::delete_category_service(pool, category_uuid.unwrap()).await;
    
    if query_result.is_err(){
        return Err(Status::InternalServerError);
    }
    let category: Category = query_result.unwrap();

    let json_response: Json<Category> = Json(category);

    Ok(json_response)
}