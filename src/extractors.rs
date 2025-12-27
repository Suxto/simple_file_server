use axum::{
    Json,
    extract::FromRequestParts,
    http::{self, StatusCode, header},
    response::{IntoResponse, Response},
};
use serde_json::json;

use crate::model::{UserConfig, app_state::AppState};

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
    S: Send + Sync + AsRef<AppState>,
{
    type Rejection = AuthError;

    async fn from_request_parts(
        parts: &mut http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let app_state = state.as_ref();
        // 从header中获取token
        let token = read_token_from_req(parts).await.ok_or(AuthError)?;

        // 从session中获取username
        app_state
            .get_user_by_token(token)
            .await
            .map(|e| AuthUser(e.clone()))
            .ok_or(AuthError)
    }
}

async fn read_token_from_req(parts: &http::request::Parts) -> Option<&str> {
    parts
        .headers
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .or_else(|| parts.headers.get("x-token").and_then(|h| h.to_str().ok()))
        .or_else(|| {
            parts.uri.query().and_then(|q| {
                q.split('&')
                    .find(|p| p.starts_with("token="))
                    .and_then(|p| p.split('=').nth(1))
            })
        })
}
