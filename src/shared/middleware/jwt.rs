use actix_web::error::Result;
use actix_web::{HttpMessage, HttpRequest};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use crate::shared::exceptions::custom_error::CustomError;

// Struct ของ Claims
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64, // Subject (เช่น user ID)
    pub exp: usize,  // Expiration time
}

// ฟังก์ชันสำหรับสร้าง JWT
pub fn create_token(user_id: i64, secret: &str) -> String {
    let expiration = Utc::now()
        .checked_add_signed(Duration::seconds(3600)) // อายุของโทเค็น (1 ชั่วโมง)
        .expect("Unable to calculate expiration time")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        exp: expiration,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
        .expect("Failed to create token")
}

// ฟังก์ชันสำหรับตรวจสอบ JWT
pub fn validate_token(token: &str, secret: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
}

pub async fn extract_user_id(req: &HttpRequest) -> std::result::Result<i64, CustomError> {
    req.extensions()
        .get::<i64>()
        .copied()
        .ok_or(CustomError::SubNotfound)
}
