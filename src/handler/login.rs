use crate::AppState;
use crate::model::{LoginRequest, LoginResponse};
use crate::utils;
use axum::{Json, extract::State};

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Json<LoginResponse> {
    let valid = state
        .config
        .users
        .iter()
        .find(|u| u.username == payload.username && u.password == payload.password)
        .is_some();

    if valid {
        Json(LoginResponse {
            success: true,
            token: Some(utils::make_token(&payload.username)),
        })
    } else {
        Json(LoginResponse {
            success: false,
            token: None,
        })
    }
}
