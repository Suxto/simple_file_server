use crate::model::AppState;
use crate::handler;
use axum::{
    Router,
    routing::{get, post},
    http::Method,
    http,
};
use tower::ServiceBuilder;

pub fn create_router(state: AppState) -> Router {
    let mut router = Router::new()
        .route("/api/login", post(handler::login))
        .route("/api/files", get(handler::list_files))
        .route("/api/upload", post(handler::upload))
        .route("/api/download", get(handler::download_file))
        .route("/api/download/{*path}", get(handler::download_folder_as_zip))
        .with_state(state);

    // 添加静态文件服务作为fallback
    router.fallback_service(tower_http::services::ServeDir::new("static"))
}