use axum::{routing::post, Router};
use tower_http::cors::CorsLayer;
use std::env;

mod handlers;
mod types;

#[tokio::main]
async fn main() {
    // Configure CORS
    let cors = CorsLayer::permissive();

    // Build our application with routes
    let app = Router::new()
        .route("/keypair", post(handlers::generate_keypair))
        .route("/token/create", post(handlers::create_token))
        .route("/token/mint", post(handlers::mint_token))
        .route("/message/sign", post(handlers::sign_message))
        .route("/message/verify", post(handlers::verify_message))
        .route("/send/sol", post(handlers::send_sol))
        .route("/send/token", post(handlers::send_token))
        // Add the health check endpoint if you want it
        .route("/health", axum::routing::get(|| async { "OK" }))
        .layer(cors);

    // Get port from environment or use 3000 as default
    let port = env::var("PORT").unwrap_or("3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    println!("🚀 Solana HTTP Server running on http://{}", addr);

    // Run the server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
