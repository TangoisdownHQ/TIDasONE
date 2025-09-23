use jsonwebtoken::{encode, EncodingKey, Header};
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

#[derive(Serialize)]
struct Claims {
    sub: String,
    exp: usize,
    provider: String,
}

fn main() {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        + Duration::from_secs(3600);

    let claims = Claims {
        sub: "test-user@example.com".to_string(),
        exp: expiration.as_secs() as usize,
        provider: "manual".to_string(),
    };

    let secret = std::env::var("JWT_SECRET").unwrap();
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes())).unwrap();

    println!("{}", token);
}

