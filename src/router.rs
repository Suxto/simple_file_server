use crate::model::AppState;
use crate::handler;
use axum::{
    Router,
    routing::{get, post},
};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/api/login", post(handler::login))
        .route("/api/files", get(handler::list_files))
        .route("/api/upload", post(handler::upload))
        .route("/api/download", get(handler::download))
        .nest_service("/", tower_http::services::ServeDir::new("static"))
        .with_state(state)
}
