use crate::config::ServerConfig;
use mandu_core::db::ManduDb;
use mandu_core::vault::Vault;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub config: ServerConfig,
    pub db: Arc<Mutex<ManduDb>>,
}

impl AppState {
    pub fn new(config: ServerConfig) -> Self {
        let db = ManduDb::open_in_memory().expect("falha ao inicializar ManduDb in-memory");
        let vault = Vault::new(&config.vault_path);
        if let Ok(notes) = vault.scan() {
            let _ = db.index_notes_batch(&notes);
        }
        Self {
            config,
            db: Arc::new(Mutex::new(db)),
        }
    }
}
