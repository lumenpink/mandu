use crate::state::AppState;
use axum::{routing::get, Router};

pub fn create_app(_state: AppState) -> Router {
    // Stub inicial para fase RED
    Router::new().route("/health", get(|| async { "not implemented" }))
}
