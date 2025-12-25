use crate::{extractors::AuthUser, model::AppState};
use axum::{
    Json,
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use tracing::{error, info};

const DEFAULT_PERMISSIONS: u8 = 0b111;

#[derive(Serialize, Deserialize)]
pub struct FileListRequest {
    pub root: String,
    pub path: String,
}

#[derive(Serialize)]
pub struct File {
    pub name: String,
    pub is_dir: bool,
    pub permissions: u8,
}

#[derive(Serialize)]
pub struct FileListResponse {
    pub files: Vec<File>,
}

pub async fn list_files(
    State(state): State<AppState>,
    Query(params): Query<FileListRequest>,
    AuthUser(user): AuthUser,
) -> impl IntoResponse {
    info!(
        "用户 '{}' 请求列出文件: {}/{}",
        &user.username, &params.root, &params.path
    );
    // state.path.get()
    if params.root.is_empty() {
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
                    })
                    .collect(),
            }),
        )
            .into_response();
    }
    if let Some(file_list) = state.path.get(&params.root).and_then(|path| {
        let full_path = format!("{}/{}", path.path, params.path);
        let dir_entries = std::fs::read_dir(&full_path).ok()?;

        let files: Vec<File> = dir_entries
            .filter_map(|entry| entry.ok())
            .map(|entry| {
                let metadata = entry.metadata().ok()?;
                Some(File {
                    name: entry.file_name().to_string_lossy().to_string(),
                    is_dir: metadata.is_dir(),
                    permissions: DEFAULT_PERMISSIONS,
                })
            })
            .filter_map(|f| f)
            .collect();
        Some(files)
    }) {
        info!(
            "用户 '{}' 成功列出文件: {}/{}",
            &user.username, &params.root, &params.path
        );
        (StatusCode::OK, Json(FileListResponse { files: file_list })).into_response()
    } else {
        error!(
            "用户 '{}' 列出文件失败: {}/{}",
            &user.username, &params.root, &params.path
        );
        (
            StatusCode::NOT_FOUND,
            Json(FileListResponse { files: vec![] }),
        )
            .into_response()
    }
}
