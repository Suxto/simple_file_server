use crate::{extractors::AuthUser, handler::list::FileRequest, model::AppState};
use axum::{
    extract::Query,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::{io::Write, path::Path};
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

    let full_path = state
        .path
        .get(&root)
        .map(|p| format!("{}{}", p.path, path));

    if full_path.is_none() {
        return (
            StatusCode::NOT_FOUND,
            "路径不存在".to_string(),
        )
            .into_response();
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
        // 如果是文件夹，压缩为zip并返回
        return download_folder_as_zip(file_path, &user.username).await;
    }

    // 如果是文件，按原来逻辑处理
    let file_content = match fs::read(&file_path).await {
        Ok(content) => content,
        Err(e) => {
            error!("读取文件失败: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "读取文件失败").into_response();
        }
    };

    // 获取文件名用于Content-Disposition头
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

// 异步函数：将文件夹压缩并作为zip文件返回
async fn download_folder_as_zip(folder_path: &Path, user_name: &str) -> Response {
    use tempfile::NamedTempFile;
    use std::fs::File as StdFile;
    use tokio::task;

    // 创建临时文件来存储zip
    let temp_file = match NamedTempFile::new() {
        Ok(file) => file,
        Err(e) => {
            error!("创建临时文件失败: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "创建临时文件失败").into_response();
        }
    };

    let temp_path: tempfile::TempPath = temp_file.into_temp_path();
    let file = match StdFile::create(&temp_path){
        Ok(file) => file,
        Err(e) => {
            error!("创建临时文件失败: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "创建临时文件失败").into_response();
        }
    };
    
    let folder_path_string = folder_path.to_path_buf();
    // 在阻塞任务中创建ZIP
    let result = task::spawn_blocking(move || {
        use zip::ZipWriter;
        let mut zip = ZipWriter::new(file);
        // 递归添加文件夹内容到zip
        add_folder_to_zip_sync(&mut zip, &folder_path_string)?;
        Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
    }).await;

    if let Err(e) = result {
        error!("ZIP创建任务失败: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, "ZIP创建任务失败").into_response();
    }

    if let Err(e) = result.unwrap() {
        error!("创建ZIP文件失败: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, "创建ZIP文件失败").into_response();
    }
    
    // 读取生成的zip文件内容
    let zip_content = match tokio::fs::read(&temp_path).await {
        Ok(content) => content,
        Err(e) => {
            error!("读取zip文件失败: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "读取zip文件失败").into_response();
        }
    };

    // 获取文件夹名称作为zip文件名
    let folder_name = match folder_path.file_name() {
        Some(name) => name.to_string_lossy().to_string(),
        None => "archive".to_string(),
    };

    // 构造响应
    (
        StatusCode::OK,
        [
            (
                "Content-Disposition",
                format!("attachment; filename=\"{}.zip\"", folder_name),
            ),
            ("Content-Type", "application/zip".to_string()),
        ],
        zip_content,
    )
        .into_response()
}

// 同步函数：递归添加文件夹内容到zip
fn add_folder_to_zip_sync(
    zip_writer: &mut zip::ZipWriter<std::fs::File>,
    folder_path: &Path,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use zip::write::FileOptions;
    use std::fs;

    // 读取文件夹内容
    let entries = fs::read_dir(folder_path)?;
    
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        // 计算相对于根文件夹的路径
        let relative_path = path.strip_prefix(folder_path.parent().unwrap_or(folder_path))
            .unwrap_or(&path);
        
        if path.is_file() {
            // 添加文件到zip
            let file_content = fs::read(&path)?;
            let file_path_str = relative_path.to_string_lossy().to_string();
            
            zip_writer.start_file(&file_path_str, FileOptions::default().compression_method(zip::CompressionMethod::Stored))?;
            zip_writer.write_all(&file_content)?;
        } else if path.is_dir() {
            // 递归处理子文件夹
            add_folder_to_zip_sync(zip_writer, &path)?;
        }
    }

    Ok(())
}