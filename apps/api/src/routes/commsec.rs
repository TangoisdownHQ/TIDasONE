use axum::{
    routing::post,
    Json as AxumJson, Router,
    response::IntoResponse,
    http::StatusCode,
};
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use pqcrypto_mlkem::mlkem1024::{
    keypair as kem_keypair, encapsulate as pq_encapsulate, decapsulate as pq_decapsulate,
    PublicKey, SecretKey, Ciphertext, SharedSecret,
};
use pqcrypto_traits::kem::{
    PublicKey as PKTrait, SecretKey as SKTrait, Ciphertext as CTTrait, SharedSecret as SSTrait,
};

use aes_gcm::{
    Aes256Gcm, KeyInit, Nonce,
    aead::{Aead, Payload},
};

/// Shared state containing persistent PQ keypair
#[derive(Clone)]
pub struct CommsecState {
    pub pk: PublicKey,
    pub sk: SecretKey,
}

pub fn init_commsec_state() -> CommsecState {
    let (pk, sk) = kem_keypair();
    CommsecState { pk, sk }
}

pub fn commsec_routes(state: CommsecState) -> Router {
    Router::new()
        .route("/commsec/keypair", post(get_keypair))
        .route("/commsec/encapsulate", post(encapsulate))
        .route("/commsec/decapsulate", post(decapsulate))
        .route("/commsec/aead/encrypt", post(aead_encrypt))
        .route("/commsec/aead/decrypt", post(aead_decrypt))
        .with_state(Arc::new(state))
}

#[derive(Serialize)]
struct KeypairResponse {
    public_key: String,
    secret_key: String,
}

async fn get_keypair(state: axum::extract::State<Arc<CommsecState>>) -> impl IntoResponse {
    let pk_b64 = general_purpose::STANDARD.encode(state.pk.as_bytes());
    let sk_b64 = general_purpose::STANDARD.encode(state.sk.as_bytes());

    AxumJson(KeypairResponse {
        public_key: pk_b64,
        secret_key: sk_b64,
    }).into_response()
}

#[derive(Deserialize)]
pub struct EncapsulateRequest {
    pub public_key: String,
}

#[derive(Serialize)]
pub struct EncapsulateResponse {
    pub ciphertext: String,
    pub shared_secret: String,
}

pub async fn encapsulate(AxumJson(req): AxumJson<EncapsulateRequest>) -> impl IntoResponse {
    let pk_bytes = match general_purpose::STANDARD.decode(&req.public_key) {
        Ok(b) => b,
        Err(_) => return (StatusCode::BAD_REQUEST, "invalid base64").into_response(),
    };

    let pk = match PublicKey::from_bytes(&pk_bytes) {
        Ok(p) => p,
        Err(_) => return (StatusCode::BAD_REQUEST, "invalid public key").into_response(),
    };

    let (ss, ct): (SharedSecret, Ciphertext) = pq_encapsulate(&pk);

    let ct_b64 = general_purpose::STANDARD.encode(ct.as_bytes());
    let ss_b64 = general_purpose::STANDARD.encode(ss.as_bytes());

    AxumJson(EncapsulateResponse {
        ciphertext: ct_b64,
        shared_secret: ss_b64,
    }).into_response()
}

#[derive(Deserialize)]
pub struct DecapsulateRequest {
    pub secret_key: String,
    pub ciphertext: String,
}

#[derive(Serialize)]
pub struct DecapsulateResponse {
    pub shared_secret: String,
}

pub async fn decapsulate(AxumJson(req): AxumJson<DecapsulateRequest>) -> impl IntoResponse {
    let sk_bytes = match general_purpose::STANDARD.decode(&req.secret_key) {
        Ok(b) => b,
        Err(_) => return (StatusCode::BAD_REQUEST, "invalid base64").into_response(),
    };
    let ct_bytes = match general_purpose::STANDARD.decode(&req.ciphertext) {
        Ok(b) => b,
        Err(_) => return (StatusCode::BAD_REQUEST, "invalid base64").into_response(),
    };

    let sk = match SecretKey::from_bytes(&sk_bytes) {
        Ok(s) => s,
        Err(_) => return (StatusCode::BAD_REQUEST, "invalid secret key").into_response(),
    };
    let ct = match Ciphertext::from_bytes(&ct_bytes) {
        Ok(c) => c,
        Err(_) => return (StatusCode::BAD_REQUEST, "invalid ciphertext").into_response(),
    };

    let ss = pq_decapsulate(&ct, &sk);
    let ss_b64 = general_purpose::STANDARD.encode(ss.as_bytes());

    AxumJson(DecapsulateResponse { shared_secret: ss_b64 }).into_response()
}

#[derive(Deserialize)]
pub struct AeadEncryptRequest {
    pub key: String,
    pub nonce: String,  // ✅ now required
    pub plaintext: String,
    pub associated_data: Option<String>,
}

#[derive(Serialize)]
pub struct AeadEncryptResponse {
    pub ciphertext: String,
}

pub async fn aead_encrypt(AxumJson(req): AxumJson<AeadEncryptRequest>) -> impl IntoResponse {
    let key_bytes = match general_purpose::STANDARD.decode(&req.key) {
        Ok(b) => b,
        Err(_) => return (StatusCode::BAD_REQUEST, "invalid key base64").into_response(),
    };
    let nonce_bytes = match general_purpose::STANDARD.decode(&req.nonce) {
        Ok(b) => b,
        Err(_) => return (StatusCode::BAD_REQUEST, "invalid nonce base64").into_response(),
    };
    if nonce_bytes.len() != 12 {
        return (StatusCode::BAD_REQUEST, "nonce must be 12 bytes").into_response();
    }

    let key = aes_gcm::Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let payload = if let Some(ad) = &req.associated_data {
        Payload { msg: req.plaintext.as_bytes(), aad: ad.as_bytes() }
    } else {
        Payload { msg: req.plaintext.as_bytes(), aad: &[] }
    };

    match cipher.encrypt(nonce, payload) {
        Ok(ct) => {
            let ct_b64 = general_purpose::STANDARD.encode(ct);
            AxumJson(AeadEncryptResponse { ciphertext: ct_b64 }).into_response()
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "encryption failed").into_response(),
    }
}

#[derive(Deserialize)]
pub struct AeadDecryptRequest {
    pub key: String,
    pub nonce: String,  // ✅ now required
    pub ciphertext: String,
    pub associated_data: Option<String>,
}

#[derive(Serialize)]
pub struct AeadDecryptResponse {
    pub plaintext: String,
}

pub async fn aead_decrypt(AxumJson(req): AxumJson<AeadDecryptRequest>) -> impl IntoResponse {
    let key_bytes = match general_purpose::STANDARD.decode(&req.key) {
        Ok(b) => b,
        Err(_) => return (StatusCode::BAD_REQUEST, "invalid key base64").into_response(),
    };
    let nonce_bytes = match general_purpose::STANDARD.decode(&req.nonce) {
        Ok(b) => b,
        Err(_) => return (StatusCode::BAD_REQUEST, "invalid nonce base64").into_response(),
    };
    if nonce_bytes.len() != 12 {
        return (StatusCode::BAD_REQUEST, "nonce must be 12 bytes").into_response();
    }

    let ct_bytes = match general_purpose::STANDARD.decode(&req.ciphertext) {
        Ok(b) => b,
        Err(_) => return (StatusCode::BAD_REQUEST, "invalid ciphertext base64").into_response(),
    };

    let key = aes_gcm::Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let payload = if let Some(ad) = &req.associated_data {
        Payload { msg: &ct_bytes, aad: ad.as_bytes() }
    } else {
        Payload { msg: &ct_bytes, aad: &[] }
    };

    match cipher.decrypt(nonce, payload) {
        Ok(pt) => {
            let pt_str = match String::from_utf8(pt) {
                Ok(s) => s,
                Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "invalid utf-8").into_response(),
            };
            AxumJson(AeadDecryptResponse { plaintext: pt_str }).into_response()
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "decryption failed").into_response(),
    }
}

