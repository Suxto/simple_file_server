use crate::model::AppState;
use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use tracing::{error, info};

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

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Json<LoginResponse> {
    let valid = state
        .user_config
        .get(&payload.username)
        .filter(|u| u.password == payload.password)
        .is_some();

    let session_id = Uuid::new_v4().to_string();
    state
        .add_session(session_id.clone(), payload.username.clone())
        .await;

    if valid {
        info!("用户 '{}' 登录成功", payload.username);
        Json(LoginResponse {
            success: true,
            token: Some(session_id),
        })
    } else {
        error!("用户 '{}' 登录失败", payload.username);
        Json(LoginResponse {
            success: false,
            token: None,
        })
    }
}
