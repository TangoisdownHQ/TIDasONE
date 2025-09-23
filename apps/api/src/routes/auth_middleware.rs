use axum::async_trait;
use axum::http::{request::Parts, StatusCode};
use axum_extra::{
    extract::TypedHeader,
    headers::{authorization::Bearer, Authorization},
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Claims {
    pub sub: String,     // user ID or email
    pub exp: usize,      // expiration timestamp
    pub provider: String // oauth provider
}

/// Extractor that validates JWT and injects claims into the handler
pub struct AuthenticatedUser(pub Claims);

#[async_trait]
impl<S> axum::extract::FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the Authorization: Bearer <token> header
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, _state)
                .await
                .map_err(|_| (StatusCode::UNAUTHORIZED, "Missing or invalid Authorization header".into()))?;

        // Get JWT secret from env
        let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        // Decode & validate JWT
        let token_data = decode::<Claims>(
            bearer.token(),
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid or expired token".into()))?;

        Ok(AuthenticatedUser(token_data.claims))
    }
}

