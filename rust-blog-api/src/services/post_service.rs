use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{models::Post, validators::{CreatePostSchema, UpdatePostSchema}};


pub async fn get_posts_service(pool: &rocket::State<PgPool>)-> Result<Vec<Post>, anyhow::Error>{
    let posts: Vec<Post>= sqlx::query_as!(Post, r#"SELECT * FROM posts"#)
    .fetch_all(&**pool)
    .await?
    .iter()
    .map(|post_row| Post{
        pk_post_id: post_row.pk_post_id,
        user_id: post_row.user_id,
        category_id: post_row.category_id,
        title: post_row.title.clone(),
        body: post_row.body.clone(),
        create_at: post_row.create_at,
        update_at: post_row.update_at,
    })
    .collect();

    Ok(posts)
}

pub async fn get_posts_user_service(pool: &rocket::State<PgPool>, user_id: Uuid)-> Result<Vec<Post>, anyhow::Error>{
    let posts: Vec<Post>= sqlx::query_as!(Post, r#"SELECT * FROM posts WHERE user_id=$1"#, &user_id)
    .fetch_all(&**pool)
    .await?
    .iter()
    .map(|post_row| Post{
        pk_post_id: post_row.pk_post_id,
        user_id: post_row.user_id,
        category_id: post_row.category_id,
        title: post_row.title.clone(),
        body: post_row.body.clone(),
        create_at: post_row.create_at,
        update_at: post_row.update_at,
    })
    .collect();

    Ok(posts)
}

pub async fn get_post_service(pool: &rocket::State<PgPool>, post_id: Uuid)-> Result<Post, anyhow::Error>{
    let post: Post= sqlx::query_as!(Post, r#"SELECT * FROM posts WHERE pk_post_id=$1"#, &post_id)
    .fetch_one(&**pool)
    .await?;

    Ok(post)
}

pub async fn create_post_service(pool: &rocket::State<PgPool>, post: CreatePostSchema<'_>)-> Result<Post, anyhow::Error>{
    let post_uuid: Uuid = Uuid::new_v4();
    let datetime = Utc::now();
    let new_post: Post = sqlx::query_as!(Post, r#"INSERT INTO posts(pk_post_id, title, body, user_id, category_id) VALUES($1, $2, $3, $4, $5) RETURNING *"#, post_uuid, post.title, post.body, post.user_id, post.category_id)
    .fetch_one(&**pool)
    .await?;

    Ok(Post{pk_post_id: new_post.pk_post_id, user_id: new_post.user_id, category_id: new_post.category_id, title: new_post.title, body: new_post.body, create_at: Some(datetime), update_at:Some(datetime)})
}

pub async fn update_post_service(pool: &rocket::State<PgPool>, post: UpdatePostSchema<'_>) -> Result<Post, anyhow::Error>{
    let datetime = Utc::now();
    let post_updated: Post = sqlx::query_as!(Post, r#"UPDATE posts SET  title=$1, body=$2, category_id=$3 WHERE pk_post_id=$4 RETURNING *"#,  post.title, post.body, post.category_id, post.post_id)
    .fetch_one(&**pool)
    .await?;

    Ok(Post{pk_post_id: post_updated.pk_post_id, user_id: post_updated.user_id, category_id: post_updated.category_id, title: post_updated.title, body: post_updated.body, create_at: Some(datetime), update_at:Some(datetime)})
}

pub async fn delete_post_service(pool: &rocket::State<PgPool>, post_id: Uuid)-> Result<Post, anyhow::Error>{
    let post_deleted: Post =sqlx::query_as!(Post, r#"DELETE FROM posts WHERE pk_post_id=$1 RETURNING *"#, post_id)
    .fetch_one(&**pool)
    .await?;

    Ok(post_deleted)
}