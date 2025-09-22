use dotenvy::dotenv;
use std::env;
use sqlx::PgPool;

use db::queries::{
    create_user, get_users,
    create_inventory, get_inventory,
    create_package, get_packages,
};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await?;

    // 1. Insert a test user
    let user = create_user(&pool, "alice", "alice@tidasone.com").await?;
    println!("👤 Inserted user: {:?}", user);

    // 2. Insert inventory
    let inventory = create_inventory(
        &pool, user.id, "Quantum Drive", Some("Prototype FTL engine"),
        1, Some("Hangar 42"), None
    ).await?;
    println!("📦 Inserted inventory: {:?}", inventory);

    // 3. Insert package
    let package = create_package(&pool, user.id, Some(inventory.id), "Mars Base Alpha").await?;
    println!("🚀 Inserted package: {:?}", package);

    // 4. Fetch back everything
    println!("👥 Users: {:?}", get_users(&pool).await?);
    println!("📦 Inventory: {:?}", get_inventory(&pool).await?);
    println!("📦 Packages: {:?}", get_packages(&pool).await?);

    Ok(())
}

