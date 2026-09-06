use axum::{extract::{State, Path}, Json};
use std::sync::Arc;
use crate::AppState;

// Lets a doctor look up patients who signed up with their referral code —
// this is the feature that gives the doctor a clinical reason (not just goodwill)
// to keep referring patients. Read-only, no write access to patient data from here.
pub async fn patients_for_code(
    State(state): State<Arc<AppState>>,
    Path(code): Path<String>,
) -> Json<serde_json::Value> {
    let rows: Vec<(uuid::Uuid, String)> = sqlx::query_as(
        "SELECT id, phone FROM users WHERE doctor_referral_code = $1"
    )
    .bind(&code)
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    Json(serde_json::json!({
        "referral_code": code,
        "patients": rows.iter().map(|r| serde_json::json!({ "id": r.0.to_string(), "phone": r.1 })).collect::<Vec<_>>()
    }))
}
