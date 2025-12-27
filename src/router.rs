use crate::model::AppState;
use crate::{handler, model::ConfigFromFile};
use axum::{
    Router,
    extract::DefaultBodyLimit,
    routing::{get, post},
};

pub fn create_router(state: AppState, config: &ConfigFromFile) -> Router {
    let mut router = Router::new()
        .route("/api/login", post(handler::login))
        .route("/api/files", get(handler::list_files))
        .route("/api/upload", post(handler::upload))
        .route("/api/download", get(handler::download))
        .with_state(state);

    if let Some(max_size) = config.misc.as_ref().and_then(|e| e.max_upload_size.clone()) {
        tracing::info!("设置最大上传大小为 {}", max_size);
        router = router.layer(DefaultBodyLimit::max(max_size));
    }

    // 添加静态文件服务作为fallback
    router.fallback_service(tower_http::services::ServeDir::new("static"))
}
