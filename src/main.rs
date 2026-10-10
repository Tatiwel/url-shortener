use axum::{
    Json, Router,
    routing::{get, post},
};

use serde::{Deserialize, Serialize};

use rand::distr::{Alphanumeric, SampleString};

#[derive(Deserialize)]
struct ShortenRequest {
    url: String,
}

#[derive(Serialize)]
struct ShortenResponse {
    code: String,
    short_url: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/shorten", post(shorten));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello from the URL shortener!"
}

async fn shorten(Json(payload): Json<ShortenRequest>) -> Json<ShortenResponse> {
    println!("Received URL: {}", payload.url);

    let code = generate_code();
    let short_url = format!("http://localhost:3000/{code}");

    Json(ShortenResponse { code, short_url })
}

fn generate_code() -> String {
    Alphanumeric.sample_string(&mut rand::rng(), 7)
}
