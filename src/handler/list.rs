use std::os::windows::fs::MetadataExt;

use crate::{extractors::AuthUser, model::AppState};
use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use tracing::{error, info};

const DEFAULT_PERMISSIONS: u8 = 0b111;

#[derive(Serialize, Deserialize)]
pub struct FileRequest {
    pub root: Option<String>,
    pub path: Option<String>,
}

#[derive(Serialize)]
pub struct File {
    pub name: String,
    pub is_dir: bool,
    pub permissions: u8,
    pub size: u64,
}

#[derive(Serialize)]
pub struct FileListResponse {
    pub files: Vec<File>,
}

pub async fn list_files(
    State(state): State<AppState>,
    Query(params): Query<FileRequest>,
    AuthUser(user): AuthUser,
) -> impl IntoResponse {
    let root = params.root.unwrap_or("".to_string());
    let path = params.path.unwrap_or("".to_string());
    info!("用户 '{}' 请求列出: {}/{}", &user.username, &root, &path);

    if root.is_empty() {
        return (
            StatusCode::OK,
            Json(FileListResponse {
                files: state
                    .path
                    .values()
                    .map(|f| File {
                        name: f.name.clone(),
                        is_dir: true,
                        permissions: DEFAULT_PERMISSIONS,
                        size: 0,
                    })
                    .collect(),
            }),
        )
            .into_response();
    }
    if let Some(file_list) = state.path.get(&root).and_then(|p| {
        let full_path = match path.len() {
            0 => p.path.clone(),
            _ => format!("{}{}", p.path, path),
        };
        info!("用户 '{}' 读取目录: {}", &user.username, &full_path);
        let dir_entries = std::fs::read_dir(&full_path).ok()?;

        let files: Vec<File> = dir_entries
            .filter_map(|entry| entry.ok())
            .map(|entry| {
                let metadata = entry.metadata().ok()?;
                let is_dir = metadata.is_dir();
                Some(File {
                    name: entry.file_name().to_string_lossy().to_string(),
                    is_dir: is_dir,
                    permissions: DEFAULT_PERMISSIONS,
                    size: if is_dir { 0 } else { metadata.file_size() },
                })
            })
            .filter_map(|f| f)
            .collect();
        Some(files)
    }) {
        info!(
            "用户 '{}' 查看目录成功: {}/{}",
            &user.username, &root, &path
        );
        (StatusCode::OK, Json(FileListResponse { files: file_list })).into_response()
    } else {
        error!(
            "用户 '{}' 查看目录失败: {}/{}",
            &user.username, &root, &path
        );
        (
            StatusCode::NOT_FOUND,
            Json(FileListResponse { files: vec![] }),
        )
            .into_response()
    }
}
