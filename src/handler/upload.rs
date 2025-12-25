use crate::utils;
use crate::{extractors::AuthUser, model::AppState};
use axum::{
    extract::Multipart,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub async fn upload(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    mut multipart: Multipart,
) -> Response {

    (StatusCode::OK, "上传成功").into_response()
}
