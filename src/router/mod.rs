use crate::controllers::{health_check, root, user};
use axum::{
    routing::{get, post},
    Router,
};

pub fn router() -> Router {
    Router::new()
        .route("/", get(root))
        .nest("/auth", auth_routes())
        .nest("/health-check", health_check_routes())
}

fn auth_routes() -> Router {
    Router::new()
        .route("/sing-in", post(user::sign_in))
        .route("/sing-up", post(user::sign_up))
}

fn health_check_routes() -> Router {
    Router::new().route("/", get(health_check::index))
}
