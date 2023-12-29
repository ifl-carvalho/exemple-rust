use crate::controllers::{health_check, root, user};
use axum::{
    routing::{get, post},
    Router,
};

pub fn router() -> Router {
    Router::new()
        .route("/", get(root))
        .nest("/user", auth_routes())
        .nest("/health-check", health_check_routes())
}

fn auth_routes() -> Router {
    Router::new()
        .route("/login", post(user::login))
        .route("/register", post(user::register))
}

fn health_check_routes() -> Router {
    Router::new().route("/", get(health_check::index))
}
