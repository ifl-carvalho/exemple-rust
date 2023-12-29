use crate::repositories::create_repositories;
use crate::router::router;
use axum::{http::header::CONTENT_TYPE, Extension, Router};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

pub fn cors() -> CorsLayer {
    CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(vec![CONTENT_TYPE])
}

pub async fn create_app() -> Router {
    let repositories = Arc::new(create_repositories().await);
    router().layer(cors()).layer(Extension(repositories))
}
