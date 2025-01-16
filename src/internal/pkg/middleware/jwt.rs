use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation, TokenData};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};
use actix_web::error::Result;

// Struct ของ Claims
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: i64, // Subject (เช่น user ID)
    exp: usize,  // Expiration time
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
