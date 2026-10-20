//! Módulo de atualização e distribuição na Banca (`https://banca.2lp.in/downloads/mandu/`).

use crate::config::{ManduConfig, UpdateChannel};
use semver::Version;
use std::env;
use std::fs::{self, File};
use std::io::Write;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

/// Endpoints oficiais para checagem e download de versões na Banca.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateEndpoints {
    pub version_url: String,
    pub binary_base_url: String,
}

/// Retorna os endpoints oficiais de distribuição na Banca para o canal especificado.
#[must_use]
pub fn get_distribution_endpoints(channel: UpdateChannel) -> UpdateEndpoints {
    let base = "https://banca.2lp.in/downloads/mandu";
    match channel {
        UpdateChannel::Stable => UpdateEndpoints {
            version_url: format!("{base}/version"),
            binary_base_url: format!("{base}/stable-latest/"),
        },
        UpdateChannel::Nightly => UpdateEndpoints {
            version_url: format!("{base}/nightly-version"),
            binary_base_url: format!("{base}/nightly-latest/"),
        },
    }
}

/// Resolve o canal de distribuição considerando argumentos, ambiente e configuração.
#[must_use]
pub fn resolve_channel(
    arg: Option<&str>,
    env_val: Option<&str>,
    cfg: &ManduConfig,
) -> UpdateChannel {
    if let Some(a) = arg {
        if let Ok(ch) = a.parse::<UpdateChannel>() {
            return ch;
        }
    }

    let env_candidate = env_val.map(ToString::to_string).or_else(|| {
        env::var("MANDU_UPDATE_CHANNEL")
            .ok()
            .or_else(|| env::var("UPDATE_CHANNEL").ok())
    });

    if let Some(e) = env_candidate {
        if let Ok(ch) = e.parse::<UpdateChannel>() {
            return ch;
        }
    }

    cfg.update.channel
}

/// Compara duas versões segundo a precedência SemVer. Retorna true se `remote` for mais recente que `current`.
#[must_use]
pub fn semver_is_newer(remote: &str, current: &str) -> bool {
    let r_clean = remote.trim().trim_start_matches(['v', 'V']);
    let c_clean = current.trim().trim_start_matches(['v', 'V']);

    if let (Ok(rv), Ok(cv)) = (Version::parse(r_clean), Version::parse(c_clean)) {
        return rv > cv;
    }

    // Fallback: comparação lexicográfica direta
    r_clean > c_clean
}

/// Substitui atomicamente um arquivo executável com preservação de permissões Unix e criação de backup `.old`.
///
/// # Errors
/// Retorna erro em caso de falha de I/O na criação do temporário ou renomeação atômica.
pub fn atomic_replace_binary(target: &Path, new_content: &[u8]) -> Result<(), String> {
    let parent = target.parent().unwrap_or_else(|| Path::new("."));
    let file_name = target
        .file_name()
        .map(|f| f.to_string_lossy().to_string())
        .unwrap_or_else(|| "mandu_bin".to_string());

    let temp_path = parent.join(format!(".{file_name}.tmp"));
    let backup_path = parent.join(format!("{file_name}.old"));

    // 1. Gravar no arquivo temporário
    let mut tmp_file = File::create(&temp_path)
        .map_err(|e| format!("Falha ao criar arquivo temporário de atualização: {e}"))?;
    tmp_file
        .write_all(new_content)
        .map_err(|e| format!("Falha ao gravar novo binário: {e}"))?;
    tmp_file
        .flush()
        .map_err(|e| format!("Falha ao descarregar buffer em disco: {e}"))?;

    // 2. Definir permissões de execução no Unix
    #[cfg(unix)]
    {
        let perms = fs::Permissions::from_mode(0o755);
        let _ = fs::set_permissions(&temp_path, perms);
    }

    // 3. Criar cópia de backup do binário anterior se ele existir
    if target.exists() {
        let _ = fs::copy(target, &backup_path);
    }

    // 4. Substituição atômica via rename
    if let Err(err) = fs::rename(&temp_path, target) {
        if backup_path.exists() {
            let _ = fs::copy(&backup_path, target);
        }
        let _ = fs::remove_file(&temp_path);
        return Err(format!(
            "Falha na substituição atômica do executável: {err}"
        ));
    }

    Ok(())
}

/// Retorna os caminhos dos binários do ecossistema Mandu presentes no mesmo diretório informado.
#[must_use]
pub fn detect_installed_binaries(bin_dir: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    for name in ["mandu", "mandu-tui", "mandu-gui"] {
        let p = bin_dir.join(name);
        if p.is_file() {
            found.push(p);
        }
    }
    found
}
