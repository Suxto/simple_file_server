use crate::model::Config;

// 简单token生成
pub fn make_token(username: &str) -> String {
    format!("token-{}", username)
}
