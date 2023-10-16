use rocket::serde::json::Json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{models::Category, validators::CreateCategorySchema};



pub async fn create_category_service(pool: &rocket::State<PgPool>, post: Json<CreateCategorySchema>)->   Result<Category, sqlx::Error >{
    let category_uuid: Uuid = Uuid::new_v4();
    let new_category= sqlx::query_as!(Category, r#"INSERT INTO categories(pk_category_id, category_name) VALUES ($1, $2) RETURNING pk_category_id, category_name"#, &category_uuid, &post.name)
        .fetch_one(&**pool)
        .await?;
   
    Ok(Category{pk_category_id: new_category.pk_category_id, category_name: new_category.category_name,})    
}


pub async fn get_categories(pool: &rocket::State<PgPool>) -> Result<Vec<Category>, sqlx::Error>{
    let categories: Vec<Category> = sqlx::query_as!(Category, r#"SELECT * FROM categories"#).fetch_all(&**pool)
    .await?
    .iter()
    .map(|category_row| Category{
        pk_category_id: category_row.pk_category_id,
        category_name: category_row.category_name.clone()
    })
    .collect();

    Ok(categories)
}


pub async fn get_category_service(pool: &rocket::State<PgPool>, category_id: Uuid) -> Result<Category, sqlx::Error>{
    let category: Category = sqlx::query_as!(Category, r#"SELECT * FROM categories WHERE categories.pk_category_id=$1"#, category_id)
    .fetch_one(&**pool)
    .await?;

    Ok(Category{pk_category_id: category.pk_category_id, category_name: category.category_name,})
}


pub async fn delete_category_service(pool: &rocket::State<PgPool>, category_id: Uuid) -> Result<Category, sqlx::Error>{
    let category_deleted = sqlx::query_as!(Category, r#"DELETE FROM categories WHERE categories.pk_category_id=$1 RETURNING pk_category_id, category_name"#, category_id)
    .fetch_one(&**pool)
    .await?;

    Ok(Category{pk_category_id: category_deleted.pk_category_id, category_name: category_deleted.category_name})
}


pub async fn update_category_service(pool: &rocket::State<PgPool>, post: Json<CreateCategorySchema>, category_id: Uuid) -> Result<Category, sqlx::Error>{
    let category_updated = sqlx::query_as!(Category, r#"UPDATE categories SET category_name=$1 WHERE pk_category_id=$2 RETURNING pk_category_id, category_name"#, post.name, category_id)
    .fetch_one(&**pool)
    .await?;

    Ok(Category{pk_category_id: category_updated.pk_category_id, category_name: category_updated.category_name })
}