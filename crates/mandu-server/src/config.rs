use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub port: u16,
    pub bind_addr: String,
    pub vault_path: PathBuf,
    pub auth_logout_url: String,
    pub cors_origin: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            port: 3000,
            bind_addr: "0.0.0.0".to_string(),
            vault_path: PathBuf::from("notes"),
            auth_logout_url: "https://auth.2lp.in/logout".to_string(),
            cors_origin: "*".to_string(),
        }
    }
}
