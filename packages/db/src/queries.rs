use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{User, Inventory, Package};

//
// ─── USERS ────────────────────────────────────────────────────────────────
//

// Create
pub async fn create_user(pool: &PgPool, username: &str, email: &str) -> sqlx::Result<User> {
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (id, username, email)
        VALUES ($1, $2, $3)
        RETURNING id, username, email, nft_token_id, identity_hash, created_at
        "#,
        Uuid::new_v4(),
        username,
        email
    )
    .fetch_one(pool)
    .await?;
    Ok(user)
}

// Read
pub async fn get_users(pool: &PgPool) -> sqlx::Result<Vec<User>> {
    let users = sqlx::query_as!(User, "SELECT * FROM users ORDER BY created_at DESC")
        .fetch_all(pool)
        .await?;
    Ok(users)
}

// Update
pub async fn update_user_email(pool: &PgPool, user_id: Uuid, new_email: &str) -> sqlx::Result<User> {
    let user = sqlx::query_as!(
        User,
        r#"
        UPDATE users
        SET email = $2
        WHERE id = $1
        RETURNING id, username, email, nft_token_id, identity_hash, created_at
        "#,
        user_id,
        new_email
    )
    .fetch_one(pool)
    .await?;
    Ok(user)
}

// Delete
pub async fn delete_user(pool: &PgPool, user_id: Uuid) -> sqlx::Result<u64> {
    let rows_affected = sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
        .execute(pool)
        .await?
        .rows_affected();
    Ok(rows_affected)
}

//
// ─── INVENTORY ────────────────────────────────────────────────────────────────
//

pub async fn create_inventory(
    pool: &PgPool,
    owner_id: Uuid,
    name: &str,
    description: Option<&str>,
    quantity: i32,
    location: Option<&str>,
    token_id: Option<&str>,
) -> sqlx::Result<Inventory> {
    let inventory = sqlx::query_as!(
        Inventory,
        r#"
        INSERT INTO inventory (id, owner_id, name, description, quantity, location, token_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id, owner_id, name, description, quantity, location, token_id, created_at
        "#,
        Uuid::new_v4(),
        owner_id,
        name,
        description,
        quantity,
        location,
        token_id
    )
    .fetch_one(pool)
    .await?;
    Ok(inventory)
}

pub async fn get_inventory(pool: &PgPool) -> sqlx::Result<Vec<Inventory>> {
    let items = sqlx::query_as!(Inventory, "SELECT * FROM inventory ORDER BY created_at DESC")
        .fetch_all(pool)
        .await?;
    Ok(items)
}

pub async fn update_inventory_quantity(
    pool: &PgPool,
    inventory_id: Uuid,
    new_quantity: i32,
) -> sqlx::Result<Inventory> {
    let item = sqlx::query_as!(
        Inventory,
        r#"
        UPDATE inventory
        SET quantity = $2
        WHERE id = $1
        RETURNING id, owner_id, name, description, quantity, location, token_id, created_at
        "#,
        inventory_id,
        new_quantity
    )
    .fetch_one(pool)
    .await?;
    Ok(item)
}

pub async fn delete_inventory(pool: &PgPool, inventory_id: Uuid) -> sqlx::Result<u64> {
    let rows_affected = sqlx::query!("DELETE FROM inventory WHERE id = $1", inventory_id)
        .execute(pool)
        .await?
        .rows_affected();
    Ok(rows_affected)
}

//
// ─── PACKAGES ────────────────────────────────────────────────────────────────
//

pub async fn create_package(
    pool: &PgPool,
    owner_id: Uuid,
    inventory_item_id: Option<Uuid>,
    destination: &str,
) -> sqlx::Result<Package> {
    let package = sqlx::query_as!(
        Package,
        r#"
        INSERT INTO packages (id, owner_id, inventory_item_id, destination)
        VALUES ($1, $2, $3, $4)
        RETURNING id, owner_id, inventory_item_id, status, destination, nft_token, created_at
        "#,
        Uuid::new_v4(),
        owner_id,
        inventory_item_id,
        destination
    )
    .fetch_one(pool)
    .await?;
    Ok(package)
}

pub async fn get_packages(pool: &PgPool) -> sqlx::Result<Vec<Package>> {
    let packages = sqlx::query_as!(Package, "SELECT * FROM packages ORDER BY created_at DESC")
        .fetch_all(pool)
        .await?;
    Ok(packages)
}

pub async fn update_package_status(
    pool: &PgPool,
    package_id: Uuid,
    new_status: &str,
) -> sqlx::Result<Package> {
    let package = sqlx::query_as!(
        Package,
        r#"
        UPDATE packages
        SET status = $2
        WHERE id = $1
        RETURNING id, owner_id, inventory_item_id, status, destination, nft_token, created_at
        "#,
        package_id,
        new_status
    )
    .fetch_one(pool)
    .await?;
    Ok(package)
}

pub async fn delete_package(pool: &PgPool, package_id: Uuid) -> sqlx::Result<u64> {
    let rows_affected = sqlx::query!("DELETE FROM packages WHERE id = $1", package_id)
        .execute(pool)
        .await?
        .rows_affected();
    Ok(rows_affected)
}

