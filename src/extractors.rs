use axum::{
    extract::{FromRequestParts},
    http::{self, header, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use crate::{
    model::{app_state::AppState, UserConfig},
};

/// 自动从请求中提取并验证用户信息的Extractor
///
/// 使用方法：在handler参数中直接使用 `AuthUser`
/// ```ignore
/// pub async fn my_handler(
///     AuthUser(user): AuthUser,
/// ) -> impl IntoResponse {
///     // user 是 UserConfig，包含用户信息
/// }
/// ```
pub struct AuthUser(pub UserConfig);

/// 认证失败的错误响应
pub struct AuthError;

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        // 返回401状态码而不是重定向，前端可以处理此错误
        let body = Json(json!({
            "error": "Unauthorized",
            "message": "Authentication failed. Please log in again."
        }));
        (StatusCode::UNAUTHORIZED, body).into_response()
    }
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut http::request::Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let app_state = parts.extensions.get::<AppState>().ok_or(AuthError)?;

        // 从header中获取token
        let token = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "))
            .or_else(|| parts.headers.get("x-token").and_then(|h| h.to_str().ok()))
            .ok_or(AuthError)?;

        // 从session中获取username
        let username = app_state
            .user_sessions
            .lock()
            .await
            .get(token)
            .cloned()
            .ok_or(AuthError)?;

        // 获取用户配置
        let user_config = app_state
            .user_config
            .get(&username)
            .cloned()
            .ok_or(AuthError)?;

        Ok(AuthUser(user_config))
    }
}

/// 使用简单的方式提取用户名（不验证权限）
pub struct Username(pub String);

impl<S> FromRequestParts<S> for Username
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut http::request::Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let app_state = parts.extensions.get::<AppState>().ok_or(AuthError)?;

        // 从header中获取token
        let token = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "))
            .or_else(|| parts.headers.get("x-token").and_then(|h| h.to_str().ok()))
            .ok_or(AuthError)?;

        // 从session中获取username
        let username = app_state
            .user_sessions
            .lock()
            .await
            .get(token)
            .cloned()
            .ok_or(AuthError)?;

        Ok(Username(username))
    }
}