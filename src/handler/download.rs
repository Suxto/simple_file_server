use axum::{
    extract::State,
    extract::Query,
    http::StatusCode,
    response::{Response, IntoResponse},
};
use std::collections::HashMap;
use crate::model::AppState;
use crate::utils;

pub async fn download(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Response {
    
    (
        StatusCode::OK,
        
    )
        .into_response()
}
