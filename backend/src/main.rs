use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::services::ServeDir;

mod filters;
mod handlers;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Serve static files from ../static
    let static_service = ServeDir::new("../static");

    // Router setup
    let app = Router::new()
        .route("/api/chat", post(handlers::chat_handler))
        .route("/ping", get(|| async { "pong" }))
        // Use the correct nesting pattern for Axum 0.7
        .nest_service("/", static_service);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on http://{}", addr);

    // Start server using hyper
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
