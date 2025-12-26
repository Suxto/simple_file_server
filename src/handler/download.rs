use axum::{
    extract::{Path, Query, State},
    http::header,
    response::Response,
};
use serde::Deserialize;
use std::path::Path as StdPath;

use crate::{extractors::AuthUser, model::AppState};

#[derive(Deserialize)]
pub struct DownloadQuery {
    name: String,
    token: String,
}

pub async fn download_file(
    State(state): State<AppState>,
    Query(query): Query<DownloadQuery>,
) -> Response {
    // 验证token
    let username = {
        let sessions = state.user_sessions.lock().await;
        if let Some(username) = sessions.get(&query.token).cloned() {
            username
        } else {
            return Response::builder()
                .status(401)
                .body(axum::body::Body::from("Invalid token"))
                .unwrap();
        }
    };

    // 检查用户权限
    let user_config = {
        if let Some(config) = state.user_config.get(&username) {
            config.clone()
        } else {
            return Response::builder()
                .status(401)
                .body(axum::body::Body::from("User not found"))
                .unwrap();
        }
    };

    let file_path = StdPath::new("files").join(&query.name);

    // 检查权限
    if !crate::utils::check_permission(&user_config.permissions_tree, &query.name, 1) {
        return Response::builder()
            .status(403)
            .body(axum::body::Body::from("No permission"))
            .unwrap();
    }

    // 检查文件是否存在
    if !file_path.exists() {
        return Response::builder()
            .status(404)
            .body(axum::body::Body::from("File not found"))
            .unwrap();
    }

    // 返回文件
    if let Ok(file) = tokio::fs::read(&file_path).await {
        use mime_guess::from_path;
        let mime = from_path(&file_path).first_or_octet_stream();

        Response::builder()
            .status(200)
            .header(header::CONTENT_TYPE, mime.as_ref())
            .header(
                header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"{}\"", query.name),
            )
            .body(axum::body::Body::from(file))
            .unwrap()
    } else {
        Response::builder()
            .status(500)
            .body(axum::body::Body::from("Read file error"))
            .unwrap()
    }
}

pub async fn download_folder_as_zip(
    Path(folder_path_str): Path<String>,
    State(state): State<AppState>,
    AuthUser(_user): AuthUser,
) -> Response {
    // 验证用户对整个文件夹的访问权限
    let user_sessions = state.user_sessions.lock().await;
    let token = "temp_token"; // 实际实现中，需要从请求头获取token
    let username = match user_sessions.get(token).cloned() {
        Some(name) => name,
        None => {
            return Response::builder()
                .status(401)
                .body(axum::body::Body::from("Invalid token"))
                .unwrap();
        }
    };

    let user_config = match state.user_config.get(&username) {
        Some(config) => config,
        None => {
            return Response::builder()
                .status(401)
                .body(axum::body::Body::from("User not found"))
                .unwrap();
        }
    };

    let folder_path = StdPath::new("files").join(&folder_path_str);

    // 检查文件夹是否存在
    if !folder_path.exists() || !folder_path.is_dir() {
        return Response::builder()
            .status(404)
            .body(axum::body::Body::from("Folder not found"))
            .unwrap();
    }

    // 检查权限 - 简化实现，实际应该检查文件夹中每个文件的权限
    if !crate::utils::check_permission(&user_config.permissions_tree, &folder_path_str, 1) {
        return Response::builder()
            .status(403)
            .body(axum::body::Body::from("No permission"))
            .unwrap();
    }

    // 临时使用错误响应，直到 zip 功能修复
    Response::builder()
        .status(501) // Not Implemented
        .body(axum::body::Body::from("Download folder as zip is not yet implemented"))
        .unwrap()
}
