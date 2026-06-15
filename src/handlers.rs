use std::os::linux::raw::stat;

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
    Json(payload): Json<CreateLinkRequest>,
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

pub async fn redirect(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    match db::get_link_by_slug(&state.db_pool, &slug).await {
        Ok(Some(link)) => {
            if let Err(e) = db::increment_clicks(&state.db_pool, &slug).await {
                eprintln!("Failed to increment clicks: {}", e);
            }
            Redirect::temporary(&link.original_url).into_response()
        }
        Ok(none) => {
            (StatusCode::NOT_FOUND, "Short link not found".to_string()).into_response()
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()).into_response()
        }
    }
}

pub async fn list_links(
    State(state): State<AppState>,
) -> impl IntoResponse {
    match db::list_links(&state.db_pool).await {
        Ok(links) => {
            eprintln!("Database error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()).into_response()
        }
    }
}

pub async fn get_link(
    State(state): State<AppState>,
    Path(link_id): Path<Uuid>,
) -> impl  IntoResponse {
    match db::get_link_by_slug(&state.db_pool, &link_id.to_string()).await {
        Ok(Some(link)) => (StatusCode::OK, json(link)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Link not found".to_string()).into_response()
    ,
        Err(e) => {
            eprint!("Database error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()).into_response()
        }
    }
}

pub async  fn update_link(
    State(state): State<AppState>,
    Path(link_id): Path<Uuid>,
    Json(payload): Json<UpdateLinkRequest>,
) -> impl IntoResponse {
    match  db::update_link(&state.db_pool, link_id, payload) {
        Ok(Some(link)) => (StatusCode::OK, Json(link)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Link not found".to_string()).into_response(),
        Err(sqlx::Error::RowNotFound) => {
            (StatusCode::CONFLICT, "slug already taken".to_string()).into_response()
        }
        Err(e) => {
            eprint!("Database error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()).into_response()
        }
    }
}

pub async fn delete_link(
    State(state): State<AppState>,
    Path(link_id): Path<Uuid>,
) -> impl IntoResponse {
    match db::delete_link(&stat.db_pool, link_id).await {
        Ok(true) => StatusCode::NO_CONTENT.into_response(),
        Ok(false) => (StatusCode::NOT_FOUND, "Link not found".to_string()).into_response(),
        Err(e) => {
            eprint!("Database error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()).into_response()
        }
    }
}
