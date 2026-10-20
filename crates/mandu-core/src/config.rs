//! Configurações de infraestrutura do Mandu (`~/.config/mandu/config.toml` e `.mandu/config.toml`).

use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

/// Canal de distribuição para atualizações na Banca.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum UpdateChannel {
    #[default]
    Stable,
    Nightly,
}

impl std::fmt::Display for UpdateChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Stable => write!(f, "stable"),
            Self::Nightly => write!(f, "nightly"),
        }
    }
}

impl FromStr for UpdateChannel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "stable" => Ok(Self::Stable),
            "nightly" => Ok(Self::Nightly),
            other => Err(format!(
                "Canal inválido '{other}'. Opções suportadas: 'stable' ou 'nightly'."
            )),
        }
    }
}

/// Configurações do atualizador automático.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateConfig {
    #[serde(default)]
    pub channel: UpdateChannel,
    #[serde(default)]
    pub pin: Option<String>,
    #[serde(default = "default_true")]
    pub auto_check: bool,
}

const fn default_true() -> bool {
    true
}

impl Default for UpdateConfig {
    fn default() -> Self {
        Self {
            channel: UpdateChannel::Stable,
            pin: None,
            auto_check: true,
        }
    }
}

/// Configuração central do Mandu.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ManduConfig {
    pub default_vault: Option<PathBuf>,
    pub editor_command: Option<String>,
    #[serde(default)]
    pub update: UpdateConfig,
}

impl ManduConfig {
    /// Carrega configuração hierárquica (local do cofre `.mandu/config.toml` ou global `~/.config/mandu/config.toml`).
    #[must_use]
    pub fn load_hierarchical(vault_path: Option<&Path>) -> Self {
        // 1. Tentar carregar do cofre local: .mandu/config.toml
        if let Some(v_path) = vault_path {
            let local_cfg_path = v_path.join(".mandu").join("config.toml");
            if let Ok(cfg) = Self::load_from_file(&local_cfg_path) {
                return cfg;
            }
        }

        // 2. Tentar carregar da raiz atual: .mandu/config.toml
        let cwd_cfg_path = Path::new(".mandu").join("config.toml");
        if let Ok(cfg) = Self::load_from_file(&cwd_cfg_path) {
            return cfg;
        }

        // 3. Tentar carregar do global ~/.config/mandu/config.toml
        if let Some(global_cfg_path) = Self::get_global_config_path() {
            if let Ok(cfg) = Self::load_from_file(&global_cfg_path) {
                return cfg;
            }
        }

        Self::default()
    }

    /// Carrega a configuração a partir de um arquivo TOML específico.
    ///
    /// # Errors
    /// Retorna erro se a leitura ou parsing TOML falhar.
    pub fn load_from_file(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let cfg: Self = toml::from_str(&content)?;
        Ok(cfg)
    }

    /// Salva a configuração no caminho global padrão (~/.config/mandu/config.toml).
    ///
    /// # Errors
    /// Retorna erro se a gravação em disco falhar.
    pub fn save_global(&self) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let path = Self::get_global_config_path()
            .ok_or_else(|| "Não foi possível resolver o diretório de configuração do usuário")?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let serialized = toml::to_string_pretty(self)?;
        fs::write(&path, serialized)?;
        Ok(path)
    }

    /// Retorna o caminho do arquivo global de configuração do usuário.
    #[must_use]
    pub fn get_global_config_path() -> Option<PathBuf> {
        if let Ok(xdg) = env::var("XDG_CONFIG_HOME") {
            if !xdg.trim().is_empty() {
                return Some(PathBuf::from(xdg).join("mandu").join("config.toml"));
            }
        }
        if let Ok(home) = env::var("HOME") {
            if !home.trim().is_empty() {
                return Some(
                    PathBuf::from(home)
                        .join(".config")
                        .join("mandu")
                        .join("config.toml"),
                );
            }
        }
        None
    }
}
