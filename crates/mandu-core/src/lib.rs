//! Mandu Core — Cérebro desacoplado do cofre, SQLite in-memory (FTS5) e resolvedor de notas.

pub mod autocomplete;
pub mod config;
pub mod db;
pub mod linter;
pub mod peek;
pub mod preview;
pub mod refs;
pub mod vault;


/// Versão do núcleo Mandu Core
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
