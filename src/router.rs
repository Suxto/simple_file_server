use crate::handler;
use crate::model::AppState;
use axum::{
    Router,
    extract::DefaultBodyLimit,
    routing::{get, post},
};

pub fn create_router(state: AppState) -> Router {
    let router = Router::new()
        .route("/api/login", post(handler::login))
        .route("/api/files", get(handler::list_files))
        .route("/api/upload", post(handler::upload))
        .route("/api/download", get(handler::download))
        .with_state(state)
        .layer(DefaultBodyLimit::max(2 * 1024 * 1024 * 1024));

    // 添加静态文件服务作为fallback
    router.fallback_service(tower_http::services::ServeDir::new("static"))
}
