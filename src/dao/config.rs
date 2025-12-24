use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct UserConfig {
    pub username: String,
    pub password: String,
    pub permissions: Vec<String>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Path {
    pub path: String,
    pub name: String,
}

// 配置文件结构
#[derive(Clone, Deserialize, Serialize)]
pub struct Config {
    pub users: Vec<UserConfig>,
    pub paths: Vec<Path>,
}
