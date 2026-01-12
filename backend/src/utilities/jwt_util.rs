use std::env;

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,        // climber_id
    pub email: String,
    pub exp: i64,        // expiration time
    pub iat: i64,        // issued at
}

#[derive(Debug)]
pub enum JwtError {
    EncodingError,
    DecodingError,
    ExpiredToken,
    InvalidToken,
    MissingSecret,
}

fn get_secret() -> Result<String, JwtError> {
    env::var("JWT_SECRET").map_err(|_| JwtError::MissingSecret)
}

pub fn create_token(climber_id: i32, email: &str) -> Result<String, JwtError> {
    let secret = get_secret()?;
    let now = Utc::now();
    let expires_at = now + Duration::hours(24);

    let claims = Claims {
        sub: climber_id,
        email: email.to_string(),
        exp: expires_at.timestamp(),
        iat: now.timestamp(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|_| JwtError::EncodingError)
}

pub fn validate_token(token: &str) -> Result<TokenData<Claims>, JwtError> {
    let secret = get_secret()?;

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|err| match err.kind() {
        jsonwebtoken::errors::ErrorKind::ExpiredSignature => JwtError::ExpiredToken,
        _ => JwtError::InvalidToken,
    })
}
