use crate::config::ServerConfig;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub config: ServerConfig,
}

impl AppState {
    pub fn new(config: ServerConfig) -> Self {
        Self { config }
    }
}
