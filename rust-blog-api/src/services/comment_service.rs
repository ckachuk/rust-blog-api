use sqlx::PgPool;
use uuid::Uuid;

use crate::{models::Comment, validators::CreateCommentSchema};

pub async fn get_comments_service(pool: &rocket::State<PgPool>, post_id: Uuid)-> Result<Vec<Comment>, anyhow::Error>{
    let comments = sqlx::query_as!(Comment, r#"SELECT * FROM comments WHERE post_id=$1"#, &post_id)
    .fetch_all(&**pool)
    .await?
    .iter()
    .map(|comment_row| Comment { pk_comment_id: comment_row.pk_comment_id, post_id: comment_row.post_id, user_id: comment_row.user_id, body: comment_row.body.clone(), create_at: comment_row.create_at, update_at: comment_row.update_at })
    .collect();
    
    Ok(comments)
}



pub async fn get_comment_service(pool: &rocket::State<PgPool>, comment_id: Uuid)-> Result<Comment, anyhow::Error>{
    let comment = sqlx::query_as!(Comment, r#"SELECT * FROM comments WHERE pk_comment_id=$1"#, &comment_id)
    .fetch_one(&**pool)
    .await?;
    
    Ok(comment)
}


pub async fn post_comment_service(pool: &rocket::State<PgPool>, post: CreateCommentSchema<'_>)->Result<Comment, anyhow::Error>{
    let comment_uuid: Uuid = Uuid::new_v4();
    
    let new_comment = sqlx::query_as!(Comment, r#"INSERT INTO comments(pk_comment_id, user_id, post_id, body) VALUES($1, $2, $3, $4) RETURNING *"#
    , comment_uuid, post.user_id, post.post_id, &post.body)
    .fetch_one(&**pool)
    .await?;

    Ok(new_comment)
}

pub async fn delete_comment_service(pool: &rocket::State<PgPool>, comment_id: Uuid)->Result<Comment, anyhow::Error>{
   let comment_deleted = sqlx::query_as!(Comment, r#"DELETE FROM comments WHERE pk_comment_id=$1 RETURNING *"#, comment_id)
   .fetch_one(&**pool)
   .await?;

    Ok(comment_deleted)
}
