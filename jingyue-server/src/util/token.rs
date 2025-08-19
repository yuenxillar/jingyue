use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::Local;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    aud: Option<String>, // Optional. Audience
    exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: usize, // Optional. Issued at (as UTC timestamp)
    iss: String, // Optional. Issuer
    nbf: Option<usize>, // Optional. Not Before (as UTC timestamp)
    sub: String, // Optional. Subject (whom token refers to)
}

impl Claims {
    pub fn from_params(user_id: impl ToString) -> Self {
        let now = Local::now().timestamp() as usize;
        Self {
            aud: None,
            exp: now + (JWT_EXPIRATION_TIME as usize),
            iat: now,
            iss: String::from("jingyue"),
            nbf: None,
            sub: user_id.to_string(),
        }
    }
}

pub(crate) const JWT_EXPIRATION_TIME: u32 = 60 * 60 * 8; // 8 hours

const SECRET_KEY: &'static str = "r3BNM4VcvVCEbRKBJXjWfSvr5G/bu6dBCVeMjzjKTCyPxC8RmJTTd";

pub struct GenerateToken;

impl GenerateToken {
    pub fn generate(claims: &Claims) -> Option<String> {
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(SECRET_KEY.as_ref()),
        )
        .ok()
    }

    pub fn decode_with_claims(token: &str) -> Option<Claims> {
        decode::<Claims>(
            &token,
            &DecodingKey::from_secret(SECRET_KEY.as_ref()),
            &Validation::default(),
        )
        .map(|s| s.claims)
        .ok()
    }

    pub fn verify(token: &str) -> bool {
        match Self::decode_with_claims(token) {
            Some(_) => true,
            None => false,
        }
    }
}
