use axum::{extract::{State, Path, Multipart}, Json};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::AppState;
use crate::gemini::interpret_marker;
use crate::storage::upload_report_file;

#[derive(Serialize)]
pub struct UploadResponse {
    pub report_id: String,
}

// Accepts a multipart file upload (the lab report image/PDF), stores it in S3.
// OCR/marker-extraction step (Gemini vision or a dedicated OCR call) plugs in here next —
// left as a follow-up so this scaffold stays reviewable in one pass.
pub async fn upload_report(
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> Json<UploadResponse> {
    let report_id = Uuid::new_v4();

    while let Some(field) = multipart.next_field().await.unwrap() {
        let content_type = field.content_type().unwrap_or("application/octet-stream").to_string();
        let bytes = field.bytes().await.unwrap().to_vec();
        let key = format!("reports/{report_id}");
        upload_report_file(&state.s3_client, &state.s3_bucket, &key, bytes, &content_type)
            .await
            .expect("s3 upload failed");
    }

    sqlx::query("INSERT INTO reports (id) VALUES ($1)")
        .bind(report_id)
        .execute(&state.db)
        .await
        .expect("insert report failed");

    Json(UploadResponse { report_id: report_id.to_string() })
}

#[derive(Deserialize)]
pub struct InterpretQuery {
    pub marker_name: String,
    pub value: String,
    pub unit: String,
    pub cycle_day: Option<i32>,
}

#[derive(Serialize)]
pub struct InterpretResponse {
    pub explanation: String,
}

pub async fn interpret_report(
    State(state): State<Arc<AppState>>,
    Path(_report_id): Path<String>,
    axum::extract::Query(q): axum::extract::Query<InterpretQuery>,
) -> Json<InterpretResponse> {
    let explanation = interpret_marker(
        &state.gemini_api_key,
        &q.marker_name,
        &q.value,
        &q.unit,
        q.cycle_day,
    )
    .await
    .unwrap_or_else(|_| "Could not generate an explanation right now.".to_string());

    Json(InterpretResponse { explanation })
}
