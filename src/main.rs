use axum::{
    Json, Router,
    routing::{get, post},
};

use serde::{Deserialize, Serialize};

// SampleString is a trait: it must be imported for .sample_string() to work
use rand::distr::{Alphanumeric, SampleString};

/// Request body for POST /shorten (JSON -> struct)
#[derive(Deserialize)]
struct ShortenRequest {
    url: String,
}

/// Response body for POST /shorten (struct -> JSON)
#[derive(Serialize)]
struct ShortenResponse {
    code: String,
    short_url: String,
}

// Sets up the Tokio async runtime before running main
#[tokio::main]
async fn main() {
    // Maps each path + HTTP method to its handler
    let app = Router::new()
        .route("/", get(root))
        .route("/shorten", post(shorten));

    // 0.0.0.0 = accept connections from any network interface
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server running on http://localhost:3000");

    // Runs the server forever, waiting for requests
    axum::serve(listener, app).await.unwrap();
}

/// GET / : health check message
async fn root() -> &'static str {
    "Hello from the URL shortener!"
}

/// POST /shorten : receives a long URL and returns a short code
// Json(payload) unwraps the request body into a ShortenRequest
async fn shorten(Json(payload): Json<ShortenRequest>) -> Json<ShortenResponse> {
    println!("Received URL: {}", payload.url);

    let code = generate_code();
    let short_url = format!("http://localhost:3000/{code}");

    Json(ShortenResponse { code, short_url })
}

/// Generates a random 7-character code (a-z, A-Z, 0-9)
fn generate_code() -> String {
    // &mut: lends the generator, allowing it to change its internal state
    Alphanumeric.sample_string(&mut rand::rng(), 7)
}
