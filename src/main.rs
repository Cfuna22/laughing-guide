mod models;
mod db;
mod handlers;

use axum::{
    routing::{get, post, put, delete},
    Router,
};
use sqlx::PgPool;
use dotenvy::dotenv;
use std::env;
use tower_http::trace::TraceLayer;

#[derive(Clone)]
pub struct AppState {
    db_pool: PgPool,
    base_url: String,
}

#[tokio::main]
async fn main( {
    // Load environment variables from .env file
    dotenv().ok();
    
    tracing_subscriber::fmt::init();
    
    // Get database URL from environment
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let base_url = env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
    
    // Create database connection pool
    let db_pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");
    
    // Run migrations (if using sqlx migrate)
    // sqlx::migrate!().run(&db_pool).await.expect("Migration failed");
    
    // Create shared state
    let state = AppState {
        db_pool,
        base_url,
    };
    
    // Build our router
    let app = Router::new()
        // Public endpoints
        .route("/shorten", post(handlers::shorten_link))
        .route("/:slug", get(handlers::redirect))
        
        // Admin endpoints (in real app, add authentication)
        .route("/links", get(handlers::list_links))
        .route("/links/:id", get(handlers::get_link))
        .route("/links/:id", put(handlers::update_link))
        .route("/links/:id", delete(handlers::delete_link))
        
        // Add middleware for logging
        .layer(TraceLayer::new_for_http())
        .with_state(state);
    
    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    
    println!("URL Shortener running on http://localhost:3000");
    println!("Example: POST /shorten with {\"original_url\": \"https://example.com\"}");
    
    axum::serve(listener, app).await.unwrap();
}
