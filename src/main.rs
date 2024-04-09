use std::sync::Arc;

use axum::{http::Method, response::IntoResponse, routing::get};
use official::{htmx::Root, AppState};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let state = AppState {};

    let app = axum::Router::new()
        .with_state(Arc::new(state))
        .route("/", get(root))
        .nest_service("/assets", tower_http::services::ServeDir::new("assets"))
        .nest("/api", official::htmx::route())
        .layer(CorsLayer::new().allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
        ]));

    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind localhost");

    axum::serve(listener, app)
        .await
        .expect("Could not serve the server");
}

async fn root() -> impl IntoResponse {
    Root::new()
}
