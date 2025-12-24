use axum::{
    extract::State,
    extract::Query,
    http::StatusCode,
    response::{Response, IntoResponse},
};
use std::collections::HashMap;
use crate::AppState;
use crate::utils;

pub async fn download(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Response {
    let token = params.get("token").cloned().unwrap_or_default();
    let username = match utils::extract_username_from_token(&token) {
        Some(u) => u,
        None => return (StatusCode::UNAUTHORIZED, "未授权").into_response(),
    };

    let name = params.get("name").cloned().unwrap_or_default();

    // 检查权限
    if !utils::has_permission(&state.config, &username, &name) {
        return (StatusCode::FORBIDDEN, "无权下载此文件").into_response();
    }

    let path = state.root_dir.join(&name);
    if !path.exists() {
        return (StatusCode::NOT_FOUND, "文件不存在").into_response();
    }
    let file = match tokio::fs::read(&path).await {
        Ok(f) => f,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "读取失败").into_response(),
    };
    (
        StatusCode::OK,
        [(
            axum::http::header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", name),
        )],
        file,
    )
        .into_response()
}
