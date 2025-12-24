use crate::dao::Config;

// 简单token生成
pub fn make_token(username: &str) -> String {
    format!("token-{}", username)
}

// 从token中提取用户名
pub fn extract_username_from_token(token: &str) -> Option<String> {
    if token.starts_with("token-") {
        Some(token.strip_prefix("token-").unwrap().to_string())
    } else {
        None
    }
}

// 从Header中提取用户名
pub fn extract_username(headers: &axum::http::header::HeaderMap) -> Option<String> {
    use axum::http::header::AUTHORIZATION;
    if let Some(auth) = headers.get(AUTHORIZATION) {
        let s = auth.to_str().ok()?;
        if let Some(token) = s.strip_prefix("Bearer ") {
            return extract_username_from_token(token);
        }
    }
    None
}

// 检查用户是否有权限访问文件
pub fn has_permission(config: &Config, username: &str, file_name: &str) -> bool {
    let user = match config.users.iter().find(|u| u.username == username) {
        Some(u) => u,
        None => return false,
    };

    // 检查权限
    for perm in &user.permissions {
        if perm == "*" {
            return true; // 全部权限
        }
        if perm == file_name {
            return true; // 精确匹配
        }
        // 支持通配符匹配 folder/*
        if perm.ends_with("/*") {
            let folder = perm.trim_end_matches("/*");
            if file_name.starts_with(&format!("{}/", folder)) || file_name == folder {
                return true;
            }
        }
    }
    false
}
