use axum::{
    routing::{get, post, put, delete},
    Router, Json, Extension,
    extract::Path,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub nft_token_id: Option<String>,
    pub identity_hash: Option<String>,
    pub created_at: chrono::DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub nft_token_id: Option<String>,
    pub identity_hash: Option<String>,
}

pub fn user_routes() -> Router {
    Router::new()
        .route("/users", get(get_users).post(create_user))
        .route("/users/:id", get(get_user).put(update_user).delete(delete_user))
}

pub async fn get_users(Extension(pool): Extension<PgPool>) -> Json<Vec<User>> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY created_at DESC")
        .fetch_all(&pool)
        .await
        .unwrap_or_default();
    Json(users)
}

pub async fn get_user(
    Path(id): Path<Uuid>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<User>, StatusCode> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match user {
        Some(u) => Ok(Json(u)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn create_user(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<NewUser>,
) -> Result<Json<User>, StatusCode> {
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (id, username, email, nft_token_id, identity_hash, created_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, username, email, nft_token_id, identity_hash, created_at
        "#,
        Uuid::new_v4(),
        payload.username,
        payload.email,
        payload.nft_token_id,
        payload.identity_hash,
        Utc::now()
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(user))
}

pub async fn update_user(
    Path(id): Path<Uuid>,
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<NewUser>,
) -> Result<Json<User>, StatusCode> {
    let user = sqlx::query_as!(
        User,
        r#"
        UPDATE users
        SET username = $1, email = $2, nft_token_id = $3, identity_hash = $4
        WHERE id = $5
        RETURNING id, username, email, nft_token_id, identity_hash, created_at
        "#,
        payload.username,
        payload.email,
        payload.nft_token_id,
        payload.identity_hash,
        id,
    )
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match user {
        Some(u) => Ok(Json(u)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn delete_user(
    Path(id): Path<Uuid>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<&'static str>, StatusCode> {
    let rows_affected = sqlx::query!("DELETE FROM users WHERE id = $1", id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .rows_affected();

    if rows_affected == 0 {
        Err(StatusCode::NOT_FOUND)
    } else {
        Ok(Json("User deleted"))
    }
}

