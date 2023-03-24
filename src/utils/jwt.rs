use axum::http::StatusCode;
use chrono::{Duration, Utc};
use dotenvy_macro::dotenv;
use jsonwebtoken::{
    crypto::verify, decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    exp: usize,
    iat: usize,
}

pub fn create_jwt(expiration_duration: &'static str) -> Result<String, StatusCode> {
    let mut now = Utc::now();
    let iat = now.timestamp() as usize;
    let expires_in = Duration::seconds(expiration_duration.parse().unwrap());
    now += expires_in;
    let exp = now.timestamp() as usize;

    let claim = Claims { exp: exp, iat: iat };

    let secret: &'static str = dotenv!("JWT_SECRET");
    let key = EncodingKey::from_secret(secret.as_bytes());

    encode(&Header::default(), &claim, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn is_valid(token: &str) -> Result<bool, StatusCode> {
    let secret: &'static str = dotenv!("JWT_SECRET");
    let key = DecodingKey::from_secret(secret.as_bytes());

    decode::<Claims>(token, &key, &Validation::new(Algorithm::HS256)).map_err(
        |_error| match _error.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        },
    )?;

    Ok(true)
}
