use crate::{
    extractors::AuthUser,
    model::AppState,
};
use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::AsyncWriteExt; // 引入 write_all_buf
use tracing::{error, info, warn};

pub async fn upload(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    mut multipart: Multipart,
) -> Response {
    let mut file_name: Option<String> = None;
    let mut root: Option<String> = None;
    let mut path: Option<String> = None;
    let mut full_path: Option<PathBuf> = None;

    // 遍历字段
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or("unknown");
        match name {
            "root" => {
                if let Some(root_config) = state.path.get(&field.text().await.unwrap_or_default()) {
                    root = Some(root_config.path.clone());
                } else {
                    return (StatusCode::NOT_FOUND, "Root不存在").into_response();
                }
            }
            "path" => {
                path = Some(field.text().await.unwrap_or_default());
            }
            "file" => {
                if root.is_none()|| path.is_none() {
                    return (StatusCode::BAD_REQUEST, "缺少根目录和路径").into_response();
                }
                file_name = Some(field.file_name().unwrap_or("unknown").to_string());
                
                // 在这里我们开始处理文件流
                let clean_file_name = sanitize_filename::sanitize(&file_name.as_ref().unwrap());
                let tentative_path = PathBuf::from(&root.as_ref().unwrap())
                    .join(path.as_ref().unwrap())
                    .join(clean_file_name);

                // 创建目录
                if let Some(parent) = tentative_path.parent() {
                    if let Err(e) = tokio::fs::create_dir_all(parent).await {
                        error!("Failed to create directory {:?}: {}", parent, e);
                        return (StatusCode::INTERNAL_SERVER_ERROR, "创建目录失败").into_response();
                    }
                }

                // 创建文件句柄
                match File::create(&tentative_path).await {
                    Ok(mut file) => {
                        // field 本身是一个 Stream，我们不需要把整个文件读入内存
                        use futures_util::StreamExt; // 需要 futures 库
                        
                        while let Some(chunk_result) = field.next().await {
                            match chunk_result {
                                Ok(chunk) => {
                                    if let Err(e) = file.write_all(&chunk).await {
                                        error!("写入文件块失败: {}", e);
                                        return (StatusCode::INTERNAL_SERVER_ERROR, "文件写入失败").into_response();
                                    }
                                }
                                Err(e) => {
                                    error!("读取数据流失败: {}", e);
                                    return (StatusCode::INTERNAL_SERVER_ERROR, "读取数据流失败").into_response();
                                }
                            }
                        }
                        
                        // 确保所有数据刷入磁盘
                        if let Err(e) = file.flush().await {
                            error!("Flush 失败: {}", e);
                            return (StatusCode::INTERNAL_SERVER_ERROR, "Flush 失败").into_response();
                        }
                        
                        full_path = Some(tentative_path);
                    }
                    Err(e) => {
                        error!("无法创建文件 {:?}: {}", tentative_path, e);
                        return (StatusCode::INTERNAL_SERVER_ERROR, "无法创建文件").into_response();
                    }
                }
            }
            _ => {
                warn!("unknown field: {}", name);
            }
        }
    }

    info!(
        "用户 '{}' 上传文件: {}{}，文件名: {}",
        &user.username,
        root.unwrap_or("!UNKNOWN!".to_string()),
        path.unwrap_or("!UNKNOWN!".to_string()),
        &file_name.as_ref().unwrap_or(&"unknown".to_string())
    );

    if full_path.is_none() {
        return (StatusCode::BAD_REQUEST, "文件接收失败").into_response();
    }

    (StatusCode::OK, "上传成功").into_response()
}