use axum::{
    extract::Path,
    routing::{get, post, put, delete},
    Router, Json, Extension,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::{Utc, DateTime};
use axum::http::StatusCode;

use crate::routes::auth_middleware::AuthenticatedUser; // ✅ import middleware

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct InventoryItem {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub quantity: i32,
    pub location: Option<String>,
    pub token_id: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct NewInventoryItem {
    pub owner_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub quantity: i32,
    pub location: Option<String>,
    pub token_id: Option<String>,
}

pub fn inventory_routes() -> Router {
    Router::new()
        .route(
            "/inventory",
            get(get_inventory).post(create_inventory_item),
        )
        .route(
            "/inventory/:id",
            get(get_inventory_item)
                .put(update_inventory_item)
                .delete(delete_inventory_item),
        )
}

/// List all inventory items (protected)
async fn get_inventory(
    AuthenticatedUser(user): AuthenticatedUser, // ✅ now requires valid JWT
    Extension(pool): Extension<PgPool>,
) -> Json<Vec<InventoryItem>> {
    println!("🔐 Authenticated user: {}", user.sub);

    let items = sqlx::query_as::<_, InventoryItem>("SELECT * FROM inventory")
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

    Json(items)
}

async fn get_inventory_item(
    AuthenticatedUser(_user): AuthenticatedUser,
    Path(id): Path<Uuid>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<InventoryItem>, StatusCode> {
    let item = sqlx::query_as::<_, InventoryItem>("SELECT * FROM inventory WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match item {
        Some(i) => Ok(Json(i)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn create_inventory_item(
    AuthenticatedUser(user): AuthenticatedUser,
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<NewInventoryItem>,
) -> Result<Json<InventoryItem>, StatusCode> {
    println!("🛠 Creating item for {}", user.sub);

    let item = sqlx::query_as!(
        InventoryItem,
        r#"
        INSERT INTO inventory (id, owner_id, name, description, quantity, location, token_id, created_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id, owner_id, name, description, quantity, location, token_id, created_at
        "#,
        Uuid::new_v4(),
        payload.owner_id,
        payload.name,
        payload.description,
        payload.quantity,
        payload.location,
        payload.token_id,
        Utc::now()
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(item))
}

async fn update_inventory_item(
    AuthenticatedUser(_user): AuthenticatedUser,
    Path(id): Path<Uuid>,
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<NewInventoryItem>,
) -> Result<Json<InventoryItem>, StatusCode> {
    let item = sqlx::query_as!(
        InventoryItem,
        r#"
        UPDATE inventory
        SET owner_id = $1, name = $2, description = $3, quantity = $4, location = $5, token_id = $6
        WHERE id = $7
        RETURNING id, owner_id, name, description, quantity, location, token_id, created_at
        "#,
        payload.owner_id,
        payload.name,
        payload.description,
        payload.quantity,
        payload.location,
        payload.token_id,
        id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match item {
        Some(i) => Ok(Json(i)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn delete_inventory_item(
    AuthenticatedUser(_user): AuthenticatedUser,
    Path(id): Path<Uuid>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<&'static str>, StatusCode> {
    let rows_affected = sqlx::query!("DELETE FROM inventory WHERE id = $1", id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .rows_affected();

    if rows_affected == 0 {
        Err(StatusCode::NOT_FOUND)
    } else {
        Ok(Json("Item deleted"))
    }
}

