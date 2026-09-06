mod routes;
mod gemini;
mod storage;
mod db;
mod auth;
mod access;

use axum::{routing::{get, post}, Router};
use std::sync::Arc;
use tower_http::cors::CorsLayer;

pub struct AppState {
    pub db: sqlx::PgPool,
    pub gemini_api_key: String,
    pub s3_client: aws_sdk_s3::Client,
    pub s3_bucket: String,
    pub jwt_secret: String,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let db = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("failed to connect to Postgres");

    sqlx::migrate!("./migrations").run(&db).await.expect("migration failed");

    let aws_cfg = aws_config::load_from_env().await;
    let s3_client = aws_sdk_s3::Client::new(&aws_cfg);

    let state = Arc::new(AppState {
        db,
        gemini_api_key: std::env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY not set"),
        s3_client,
        s3_bucket: std::env::var("S3_BUCKET").expect("S3_BUCKET not set"),
        jwt_secret: std::env::var("JWT_SECRET").expect("JWT_SECRET not set"),
    });

    let app = Router::new()
        .route("/health", get(routes::health::check))
        .route("/auth/signup", post(routes::auth::signup))
        .route("/auth/login", post(routes::auth::login))
        .route("/reports/upload", post(routes::reports::upload_report))
        .route("/reports/:id/interpret", get(routes::reports::interpret_report))
        .route("/tww/log", post(routes::tww::log_symptom))
        .route("/tww/history", get(routes::tww::get_history))
        .route("/doctor/:code/patients", get(routes::doctor::patients_for_code))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();
    tracing::info!("astra-nest backend listening on {port}");
    axum::serve(listener, app).await.unwrap();
}
