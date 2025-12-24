use axum::{
    extract::State,
    extract::Multipart,
    http::StatusCode,
    response::{Response, IntoResponse},
};
use crate::AppState;
use crate::utils;

pub async fn upload(
    State(state): State<AppState>,
    headers: axum::http::header::HeaderMap,
    mut multipart: Multipart,
) -> Response {
    let username = match utils::extract_username(&headers) {
        Some(u) => u,
        None => return (StatusCode::UNAUTHORIZED, "未授权").into_response(),
    };

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field
            .file_name()
            .map(|s| s.to_string())
            .unwrap_or("upload.bin".to_string());

        // 检查权限
        if !utils::has_permission(&state.config, &username, &name) {
            return (StatusCode::FORBIDDEN, "无权上传此文件").into_response();
        }

        let data = field.bytes().await.unwrap();
        let path = state.root_dir.join(&name);
        if let Err(e) = tokio::fs::write(&path, &data).await {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("上传失败: {}", e),
            )
                .into_response();
        }
    }
    (StatusCode::OK, "上传成功").into_response()
}
