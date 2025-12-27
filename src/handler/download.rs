use crate::{extractors::AuthUser, handler::list::FileRequest, model::AppState};
use axum::{
    extract::Query,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::path::Path;
use tokio::fs;
use tracing::{error, info};

pub async fn download(
    State(state): State<AppState>,
    Query(params): Query<FileRequest>,
    AuthUser(user): AuthUser,
) -> Response {
    let root = params.root.unwrap_or("".to_string());
    let path = params.path.unwrap_or("".to_string());
    info!("用户 '{}' 请求下载: {}/{}", &user.username, &root, &path);

    let full_path = state.path.get(&root).map(|p| format!("{}{}", p.path, path));

    if full_path.is_none() {
        return (StatusCode::NOT_FOUND, "路径不存在".to_string()).into_response();
    }

    // 从查询参数中获取路径
    let path_str = full_path.unwrap();
    info!("用户 '{}' 下载文件: {}", &user.username, &path_str);
    let file_path = Path::new(&path_str);

    // 检查文件是否存在
    if !file_path.exists() {
        return (StatusCode::NOT_FOUND, "文件不存在").into_response();
    }

    // 检查是否为文件夹
    if file_path.is_dir() {
        return (StatusCode::NOT_IMPLEMENTED, "不支持文件夹下载").into_response();
    }

    let file_content = match fs::read(&file_path).await {
        Ok(content) => content,
        Err(e) => {
            error!("读取文件失败: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "读取文件失败").into_response();
        }
    };

    let file_name = match file_path.file_name() {
        Some(name) => name.to_string_lossy().to_string(),
        None => return (StatusCode::INTERNAL_SERVER_ERROR, "无效的文件名").into_response(),
    };

    // 构造响应
    (
        StatusCode::OK,
        [
            (
                "Content-Disposition",
                format!("attachment; filename=\"{}\"", file_name),
            ),
            (
                "Content-Type",
                mime_guess::from_path(&file_path)
                    .first_or_octet_stream()
                    .to_string(),
            ),
        ],
        file_content,
    )
        .into_response()
}
