use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use oauth2::{AuthorizationCode, CsrfToken, TokenResponse};
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;
use jsonwebtoken::{encode, EncodingKey, Header};

#[derive(Clone)]
pub struct AuthState {
    pub clients: HashMap<String, oauth2::basic::BasicClient>,
    pub jwt_secret: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    pub code: String,
    pub state: String,
}

#[derive(serde::Serialize)]
struct Claims {
    sub: String,     // subject (user id or email)
    exp: usize,      // expiration timestamp
    provider: String,
}

#[derive(serde::Serialize)]
struct AuthResponse {
    token: String,
}

/// Expected minimal fields from userinfo endpoints
#[derive(Debug, Deserialize)]
struct UserInfo {
    // Google / Amazon
    sub: Option<String>,
    email: Option<String>,

    // GitHub
    id: Option<u64>,
    login: Option<String>,
    name: Option<String>,
}

pub fn auth_routes(state: AuthState) -> axum::Router {
    use axum::routing::get;
    axum::Router::new()
        .route("/auth/login/:provider", get(login_handler))
        .route("/auth/callback/:provider", get(callback_handler))
        .with_state(state)
}

/// Start OAuth login flow
async fn login_handler(
    State(state): State<AuthState>,
    Path(provider): Path<String>,
) -> Result<(StatusCode, [(String, String); 1]), (StatusCode, String)> {
    if let Some(client) = state.clients.get(&provider) {
        let (auth_url, _csrf_token) = client
            .authorize_url(CsrfToken::new_random)
            .add_scope(oauth2::Scope::new("openid".to_string()))
            .add_scope(oauth2::Scope::new("email".to_string()))
            .add_scope(oauth2::Scope::new("profile".to_string()))
            .url();

        Ok((
            StatusCode::SEE_OTHER,
            [("Location".to_string(), auth_url.to_string())],
        ))
    } else {
        Err((StatusCode::BAD_REQUEST, "Unknown provider".to_string()))
    }
}

/// Handle OAuth callback, exchange code for token, fetch user info, issue JWT
async fn callback_handler(
    State(state): State<AuthState>,
    Path(provider): Path<String>,
    Query(query): Query<AuthRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let client = state
        .clients
        .get(&provider)
        .ok_or((StatusCode::BAD_REQUEST, "Unknown provider".to_string()))?;

    let token_result = client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(oauth2::reqwest::async_http_client)
        .await;

    let token = match token_result {
        Ok(tok) => tok,
        Err(err) => {
            eprintln!("OAuth error: {:?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Token exchange failed".to_string(),
            ));
        }
    };

    // 🔑 Use access_token to fetch user profile
    let access_token = token.access_token().secret();
    let userinfo_url = match provider.as_str() {
        "google" => "https://www.googleapis.com/oauth2/v3/userinfo",
        "github" => "https://api.github.com/user",
        "amazon" => "https://api.amazon.com/user/profile",
        _ => return Err((StatusCode::BAD_REQUEST, "Unknown provider".to_string())),
    };

    let client = reqwest::Client::new();
    let userinfo_res = client
        .get(userinfo_url)
        .bearer_auth(access_token)
        .header("User-Agent", "TIDasONE-App") // GitHub requires UA
        .send()
        .await
        .map_err(|_| (StatusCode::BAD_GATEWAY, "Failed to call userinfo endpoint".to_string()))?;

    if !userinfo_res.status().is_success() {
        return Err((
            StatusCode::BAD_GATEWAY,
            format!("Userinfo request failed: {}", userinfo_res.status()),
        ));
    }

    let userinfo: UserInfo = userinfo_res
        .json()
        .await
        .map_err(|_| (StatusCode::BAD_GATEWAY, "Failed to parse userinfo".to_string()))?;

    // ✅ pick best identifier depending on provider
    let subject = userinfo
        .email
        .or_else(|| userinfo.login.clone())
        .or_else(|| userinfo.name.clone())
        .or_else(|| userinfo.sub.clone())
        .or_else(|| userinfo.id.map(|id| id.to_string()))
        .unwrap_or_else(|| "unknown".to_string());

    // mint JWT
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(1))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: subject,
        exp: expiration,
        provider: provider.clone(),
    };

    let jwt = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_secret.as_bytes()),
    )
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to encode JWT".to_string(),
        )
    })?;

    Ok(Json(AuthResponse { token: jwt }))
}

