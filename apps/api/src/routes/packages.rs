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
pub struct Package {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub inventory_item_id: Option<Uuid>,
    pub status: String,
    pub destination: String,
    pub nft_token: Option<String>,  // ✅ added NFT token support
    pub created_at: chrono::DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct NewPackage {
    pub owner_id: Uuid,
    pub inventory_item_id: Option<Uuid>,
    pub status: Option<String>,
    pub destination: String,
    #[serde(rename = "nft_token")] // ✅ maps from JSON key "nft_token"
    pub nft_token: Option<String>,
}

pub fn package_routes() -> Router {
    Router::new()
        .route("/packages", get(get_packages).post(create_package))
        .route("/packages/:id", get(get_package).put(update_package).delete(delete_package))
}

async fn get_packages(Extension(pool): Extension<PgPool>) -> Json<Vec<Package>> {
    let rows = sqlx::query_as::<_, Package>("SELECT * FROM packages")
        .fetch_all(&pool)
        .await
        .unwrap_or_default();
    Json(rows)
}

async fn get_package(
    Path(id): Path<Uuid>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Package>, StatusCode> {
    let row = sqlx::query_as::<_, Package>("SELECT * FROM packages WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match row {
        Some(pkg) => Ok(Json(pkg)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn create_package(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<NewPackage>,
) -> Result<Json<Package>, StatusCode> {
    let pkg = sqlx::query_as!(
        Package,
        r#"
        INSERT INTO packages (id, owner_id, inventory_item_id, status, destination, nft_token, created_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id, owner_id, inventory_item_id, status, destination, nft_token, created_at
        "#,
        Uuid::new_v4(),
        payload.owner_id,
        payload.inventory_item_id,
        payload.status.unwrap_or_else(|| "Pending".to_string()),
        payload.destination,
        payload.nft_token,
        Utc::now(),
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(pkg))
}

async fn update_package(
    Path(id): Path<Uuid>,
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<NewPackage>,
) -> Result<Json<Package>, StatusCode> {
    let pkg = sqlx::query_as!(
        Package,
        r#"
        UPDATE packages
        SET owner_id = $1, inventory_item_id = $2, status = $3, destination = $4, nft_token = $5
        WHERE id = $6
        RETURNING id, owner_id, inventory_item_id, status, destination, nft_token, created_at
        "#,
        payload.owner_id,
        payload.inventory_item_id,
        payload.status.unwrap_or_else(|| "Pending".to_string()),
        payload.destination,
        payload.nft_token,
        id,
    )
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match pkg {
        Some(pkg) => Ok(Json(pkg)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn delete_package(
    Path(id): Path<Uuid>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<&'static str>, StatusCode> {
    let result = sqlx::query!("DELETE FROM packages WHERE id = $1", id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(Json("Package deleted"))
}

