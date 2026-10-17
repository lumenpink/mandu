//! Configurações de infraestrutura do Mandu (`~/.config/mandu/config.toml`).

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ManduConfig {
    pub default_vault: Option<PathBuf>,
    pub editor_command: Option<String>,
}
