use axum::{Router, Extension};
use sqlx::PgPool;
use std::net::SocketAddr;
use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use std::collections::HashMap;
use dotenvy::dotenv;

use crate::routes::commsec::{commsec_routes, init_commsec_state}; // ✅ added init_commsec_state
use crate::routes::auth::{auth_routes, AuthState};
use crate::routes::{user, inventory, packages};

mod routes;

/// Helper to load an env var with clear error messages
fn get_env_var(key: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| {
        eprintln!("❌ Missing environment variable: {}", key);
        std::process::exit(1);
    })
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // ✅ Load .env
    dotenv().ok();

    // ✅ Connect to DB
    let database_url = get_env_var("DATABASE_URL");
    let pool = PgPool::connect(&database_url).await?;

    // ✅ Setup OAuth clients
    let mut clients = HashMap::new();

    // Google
    clients.insert("google".to_string(),
        oauth2::basic::BasicClient::new(
            ClientId::new(get_env_var("GOOGLE_CLIENT_ID")),
            Some(ClientSecret::new(get_env_var("GOOGLE_CLIENT_SECRET"))),
            AuthUrl::new("https://accounts.google.com/o/oauth2/auth".to_string()).unwrap(),
            Some(TokenUrl::new("https://oauth2.googleapis.com/token".to_string()).unwrap()),
        )
        .set_redirect_uri(RedirectUrl::new(
            "http://127.0.0.1:3000/auth/callback/google".to_string()
        ).unwrap())
    );

    // GitHub
    clients.insert("github".to_string(),
        oauth2::basic::BasicClient::new(
            ClientId::new(get_env_var("GITHUB_CLIENT_ID")),
            Some(ClientSecret::new(get_env_var("GITHUB_CLIENT_SECRET"))),
            AuthUrl::new("https://github.com/login/oauth/authorize".to_string()).unwrap(),
            Some(TokenUrl::new("https://github.com/login/oauth/access_token".to_string()).unwrap()),
        )
        .set_redirect_uri(RedirectUrl::new(
            "http://127.0.0.1:3000/auth/callback/github".to_string()
        ).unwrap())
    );

    // Amazon
    clients.insert("amazon".to_string(),
        oauth2::basic::BasicClient::new(
            ClientId::new(get_env_var("AMAZON_CLIENT_ID")),
            Some(ClientSecret::new(get_env_var("AMAZON_CLIENT_SECRET"))),
            AuthUrl::new("https://www.amazon.com/ap/oa".to_string()).unwrap(),
            Some(TokenUrl::new("https://api.amazon.com/auth/o2/token".to_string()).unwrap()),
        )
        .set_redirect_uri(RedirectUrl::new(
            "http://127.0.0.1:3000/auth/callback/amazon".to_string()
        ).unwrap())
    );

    let auth_state = AuthState {
        clients,
        jwt_secret: get_env_var("JWT_SECRET"),
    };

    // ✅ Initialize CommSec state
    let commsec_state = init_commsec_state();

    // ✅ Register routes
    let app = Router::new()
        .merge(user::user_routes())
        .merge(inventory::inventory_routes())
        .merge(packages::package_routes())
        .merge(auth_routes(auth_state))
        .merge(commsec_routes(commsec_state)) // ✅ pass CommsecState here
        .layer(Extension(pool));

    // ✅ Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 API running at http://{}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;

    Ok(())
}

