use crate::controllers::middleware::auth;
use crate::controllers::{recipes, root, user};
use axum::routing::{get, post};
use axum::{middleware, Router};

pub fn router() -> Router {
    Router::new()
        .route("/", get(root))
        .nest("/", public_routes())
        .nest("/", private_routes())
}

pub fn public_routes() -> Router {
    Router::new()
        .route("/user/login", post(user::login))
        .route("/user/register", post(user::register))
        .route("/user/refresh", post(user::refresh))
}

pub fn private_routes() -> Router {
    Router::new()
        .route("/recipes", get(recipes::index))
        .route_layer(middleware::from_fn(auth::guard))
}
