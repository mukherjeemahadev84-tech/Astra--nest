use axum::{extract::State, Json};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::AppState;

#[derive(Deserialize)]
pub struct SymptomLogRequest {
    pub user_id: String,
    pub symptom: String,
    pub note: Option<String>,
}

#[derive(Serialize)]
pub struct LogResponse {
    pub id: String,
    pub reassurance: String,
}

// Calm, evidence-based, non-diagnostic responses only — see the guardrail note in gemini.rs.
// A curated symptom -> reassurance map is safer here than a free-form model call for the
// most common TWW symptoms; fall back to the Gemini call only for anything unmapped.
pub async fn log_symptom(
    State(state): State<Arc<AppState>>,
    Json(req): Json<SymptomLogRequest>,
) -> Json<LogResponse> {
    let id = Uuid::new_v4();

    sqlx::query(
        "INSERT INTO tww_logs (id, user_id, symptom, note) VALUES ($1, $2, $3, $4)"
    )
    .bind(id)
    .bind(&req.user_id)
    .bind(&req.symptom)
    .bind(&req.note)
    .execute(&state.db)
    .await
    .expect("insert tww log failed");

    let reassurance = match req.symptom.to_lowercase().as_str() {
        s if s.contains("spot") || s.contains("bleed") => {
            "Light spotting is common during the two-week wait and isn't unusual. Keep noting it, and mention it at your next check-in."
        }
        s if s.contains("cramp") => {
            "Mild cramping is commonly reported during this phase. It's worth tracking, and flag it to your clinic if it becomes severe."
        }
        _ => "Thanks for logging this — your clinic can review your log at your next visit.",
    };

    Json(LogResponse { id: id.to_string(), reassurance: reassurance.to_string() })
}

pub async fn get_history(State(_state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    Json(serde_json::json!({ "note": "wire this to a real query once the schema is finalized" }))
}
