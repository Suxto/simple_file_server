use axum::{
    extract::State,
    http::{StatusCode, HeaderMap},
    Json,
    response::IntoResponse,
};
use crate::model::{FilesResponse, FileInfo};
use crate::AppState;
use crate::utils;

pub async fn list_files(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let username = match utils::extract_username(&headers) {
        Some(u) => u,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(FilesResponse { files: vec![] }),
            )
                .into_response();
        }
    };

    let mut files = vec![];
    let mut dir = match tokio::fs::read_dir(&state.root_dir).await {
        Ok(d) => d,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(FilesResponse { files: vec![] }),
            )
                .into_response();
        }
    };

    while let Ok(Some(entry)) = dir.next_entry().await {
        let file_name = entry.file_name().to_string_lossy().to_string();
        if utils::has_permission(&state.config, &username, &file_name) {
            let meta = entry.metadata().await.unwrap();
            files.push(FileInfo {
                name: file_name,
                is_dir: meta.is_dir(),
            });
        }
    }
    (StatusCode::OK, Json(FilesResponse { files })).into_response()
}
