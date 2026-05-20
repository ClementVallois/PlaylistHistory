use anyhow::Result;
use axum::{Json, Router, routing::get};
use reqwest::StatusCode;
use serde_json::Value;

const DEEZER_URL: &str = "https://api.deezer.com/";
const PLAYLIST_PARAMETER: &str = "playlist/";

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(""))
        .route("/playlist", get(playlist_handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3555").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub struct DeezerClient {
    deezer_url: String,
}

async fn playlist_handler() -> Result<std::string::String, StatusCode> {
    let url = format!("{}{}{}", DEEZER_URL, PLAYLIST_PARAMETER, "908622995");
    let res = reqwest::get(url)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    res.text()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
