pub mod routes;

use axum::{Router, Extension};
use sqlx::PgPool;

/// Shared app router — used by both main.rs and tests
pub fn app_routes(pool: PgPool) -> Router {
    Router::new()
        .merge(routes::user::user_routes())
        .merge(routes::inventory::inventory_routes())
        .merge(routes::packages::package_routes())
        .layer(Extension(pool))
}

pub async fn init_db_pool() -> PgPool {
    let url = std::env::var("DATABASE_TEST_URL")
        .or_else(|_| std::env::var("DATABASE_URL"))
        .expect("DATABASE_URL or DATABASE_TEST_URL must be set");
    PgPool::connect(&url).await.expect("Failed to connect to DB")
}

