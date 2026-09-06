use argon2::{Argon2, PasswordHasher, PasswordVerifier, PasswordHash};
use argon2::password_hash::{SaltString, rand_core::OsRng};
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user id
    pub exp: usize,
}

pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string()
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    match PasswordHash::new(hash) {
        Ok(parsed) => Argon2::default().verify_password(password.as_bytes(), &parsed).is_ok(),
        Err(_) => false,
    }
}

pub fn create_token(user_id: &str, secret: &str) -> String {
    let claims = Claims {
        sub: user_id.to_string(),
        exp: (chrono::Utc::now().timestamp() + 60 * 60 * 24 * 30) as usize, // 30 days
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes())).unwrap()
}
