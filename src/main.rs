#![deny(clippy::all)]

use banana::bootstrap;
use std::env;
use tokio::net::TcpListener;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let port = env::var("PORT").expect("PORT must be set");
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await.unwrap();
    tracing::debug!("listening on {}", &addr);

    let app = bootstrap::create_app().await;
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
