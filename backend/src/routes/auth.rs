use axum::{extract::State, Json};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::AppState;
use crate::auth::{hash_password, verify_password, create_token};

#[derive(Deserialize)]
pub struct SignupRequest {
    pub phone: String,
    pub password: String,
    pub doctor_referral_code: Option<String>,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub phone: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user_id: String,
}

pub async fn signup(
    State(state): State<Arc<AppState>>,
    Json(req): Json<SignupRequest>,
) -> Json<AuthResponse> {
    let user_id = Uuid::new_v4();
    let password_hash = hash_password(&req.password);

    sqlx::query(
        "INSERT INTO users (id, phone, password_hash, doctor_referral_code) VALUES ($1, $2, $3, $4)"
    )
    .bind(user_id)
    .bind(&req.phone)
    .bind(&password_hash)
    .bind(&req.doctor_referral_code)
    .execute(&state.db)
    .await
    .expect("insert user failed");

    let token = create_token(&user_id.to_string(), &state.jwt_secret);
    Json(AuthResponse { token, user_id: user_id.to_string() })
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(req): Json<LoginRequest>,
) -> Json<AuthResponse> {
    let row: (Uuid, String) = sqlx::query_as(
        "SELECT id, password_hash FROM users WHERE phone = $1"
    )
    .bind(&req.phone)
    .fetch_one(&state.db)
    .await
    .expect("user not found");

    assert!(verify_password(&req.password, &row.1), "invalid password");

    let token = create_token(&row.0.to_string(), &state.jwt_secret);
    Json(AuthResponse { token, user_id: row.0.to_string() })
}
