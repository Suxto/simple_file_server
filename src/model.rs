use serde::{Deserialize, Serialize};

// 文件信息结构
#[derive(Serialize)]
pub struct FileInfo {
    pub name: String,
    pub is_dir: bool,
}

// 登录请求结构
#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub token: Option<String>,
}

#[derive(Serialize)]
pub struct FilesResponse {
    pub files: Vec<FileInfo>,
}
