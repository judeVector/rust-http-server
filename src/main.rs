use axum::{routing::post, Router};
use tower_http::cors::CorsLayer;

mod handlers;
mod types;

#[tokio::main]
async fn main() {
    // Configure CORS
    let cors = CorsLayer::permissive();

    // Build our application with a route
    let app = Router::new()
            .route("/keypair", post(handlers::generate_keypair))
            .route("/token/create", post(handlers::create_token))
            .route("/token/mint", post(handlers::mint_token))
            .route("/message/sign", post(handlers::sign_message))
            .route("/message/verify", post(handlers::verify_message))
            .route("/send/sol", post(handlers::send_sol))
            .route("/send/token", post(handlers::send_token))
            .layer(cors);

    // Run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0").await.unwrap();
    println!("🚀 Solana HTTP Server running on http://127.0.0.1:3000");
    println!("📋 Available endpoints:");
    println!("   POST /keypair - Generate a new Solana keypair");
    println!("   POST /token/create - Create a new SPL token");
    println!("   POST /token/mint - Mint SPL tokens");
    println!("   POST /message/sign - Sign a message");
    println!("   POST /message/verify - Verify a signed message");
    println!("   POST /send/sol - Create SOL transfer instruction");
    println!("   POST /send/token - Create SPL token transfer instruction");
    println!("   GET  /health - Health check");
    
    axum::serve(listener, app).await.unwrap();
}

