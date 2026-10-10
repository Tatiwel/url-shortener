use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use axum::{
    Json, Router,
    extract::State,
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

/// Shared application state, available to every handler
#[derive(Clone)]
struct AppState {
    // code -> original URL
    links: Arc<RwLock<HashMap<String, String>>>,
}

// Sets up the Tokio async runtime before running main
#[tokio::main]
async fn main() {
    // Empty map, wrapped for safe sharing between threads
    let state = AppState {
        links: Arc::new(RwLock::new(HashMap::new())),
    };

    // Maps each path + HTTP method to its handler
    let app = Router::new()
        .route("/", get(root))
        .route("/shorten", post(shorten))
        .with_state(state);

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

/// POST /shorten : receives a long URL, stores it and returns a short code
// Json(payload) unwraps the request body into a ShortenRequest
async fn shorten(
    State(state): State<AppState>,
    Json(payload): Json<ShortenRequest>,
) -> Json<ShortenResponse> {
    println!("Received URL: {}", payload.url);

    let code = {
        // Write lock held while checking AND inserting, so no other
        // request can grab the same code in between
        let mut links = state.links.write().unwrap();

        // Draw again until the code is not already in use
        let mut code = generate_code();
        while links.contains_key(&code) {
            code = generate_code();
        }

        links.insert(code.clone(), payload.url);
        println!("Stored links: {}", links.len());
        code
    }; // lock released here

    let short_url = format!("http://localhost:3000/{code}");

    Json(ShortenResponse { code, short_url })
}

/// Generates a random 7-character code (a-z, A-Z, 0-9)
fn generate_code() -> String {
    // &mut: lends the generator, allowing it to change its internal state
    Alphanumeric.sample_string(&mut rand::rng(), 7)
}
