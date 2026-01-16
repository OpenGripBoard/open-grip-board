use base64::prelude::*;
use jwt_simple::prelude::*;
use serde::{Deserialize, Serialize};
use std::env;

use crate::errors::errors::JwtError;

#[derive(Serialize, Deserialize)]
pub struct ClimberDetailsClaim {
    climber_id: i32,
    email: String,
}

fn get_key() -> Result<HS256Key, JwtError> {
    let secret_b64: String = env::var("JWT_HS256_SECRET")?;
    let secret: Vec<u8> = BASE64_STANDARD.decode(secret_b64)?;
    Ok(HS256Key::from_bytes(&secret))
}

pub fn create_token(climber_id: i32, email: String) -> Result<String, JwtError> {
    let key = get_key()?;
    let climber_details_claim = ClimberDetailsClaim {
        climber_id: climber_id,
        email: email,
    };
    let claims = Claims::with_custom_claims(climber_details_claim, Duration::from_hours(24));
    let token: String = key.authenticate(claims)?;
    return Ok(token);
}

pub fn validate_token(token: &str) -> Result<JWTClaims<ClimberDetailsClaim>, JwtError> {
    let key = get_key()?;
    let claims = key.verify_token::<ClimberDetailsClaim>(&token, None)?;
    return Ok(claims);
}
