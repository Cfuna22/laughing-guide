use axum::{
    extract::{Path, State, Json},
    response::{Redirect, IntoResponse},
    http::StatusCode,
};
use uuid::Uuid;
use crate::models::{CreateLinkRequest, UpdateLinkRequest, CreateLinkResponse};
use crate::db;
use crate::AppState;

pub async fn shorten_link(
    State(state): State<AppState>,
    Json(Payload): Json<CreateLinkRequest>,
) -> impl IntoResponse {
    if !Payload.original_url.starts_with("http") {
        return (StatusCode::BAD_REQUEST, "Invalid URL format".to_string()).into_response();
    }

    match db::create_link(&state.db_pool, Payload).await {
        Ok(link) => {
            let short_url = format!("{}/{}", state.base_url, link.slug);
            let response = CreateLinkResponse {
                short_url,
                slug: link.slug,
                original_url: link.original_url,
            };
            (StatusCode::CREATED, Json(response)).into_response()
        }
        Err(sqlx::Error::RowNotFound) => {
            (StatusCode::CONFLICT, "Link already exists").to_string().into_response()
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").to_string().into_response()
        }
    }
}
