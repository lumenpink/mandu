//! Despachante inteligente CLI para o ecossistema Mandu.

use std::env;
use std::path::{Path, PathBuf};

/// Modo de destino da interface do Mandu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetMode {
    Tui,
    Gui,
}

impl TargetMode {
    #[must_use]
    pub fn binary_name(self) -> &'static str {
        match self {
            Self::Tui => "mandu-tui",
            Self::Gui => "mandu-gui",
        }
    }
}

/// Decisão de despacho calculada a partir dos parâmetros de linha de comando e ambiente.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DispatchDecision {
    pub target: TargetMode,
    pub filtered_args: Vec<String>,
}

/// Determina o modo de execução (Tui ou Gui) com base em argumentos CLI e variáveis de exibição.
#[must_use]
pub fn resolve_target(args: &[String], has_wayland: bool, has_x11: bool) -> DispatchDecision {
    let mut target = None;
    let mut filtered_args = Vec::new();

    let skip_first = if let Some(first) = args.first() {
        first.ends_with("mandu") || first.ends_with("mandu.exe")
    } else {
        false
    };

    let slice = if skip_first { &args[1..] } else { args };

    for arg in slice {
        match arg.as_str() {
            "--tui" => target = Some(TargetMode::Tui),
            "--gui" => target = Some(TargetMode::Gui),
            _ => filtered_args.push(arg.clone()),
        }
    }

    let resolved_target = target.unwrap_or(if has_wayland || has_x11 {
        TargetMode::Gui
    } else {
        TargetMode::Tui
    });

    DispatchDecision {
        target: resolved_target,
        filtered_args,
    }
}

/// Localiza o binário no diretório do executável corrente ou no PATH.
#[must_use]
pub fn find_binary(
    target: TargetMode,
    current_exe_dir: Option<&Path>,
    path_env: Option<&str>,
) -> Option<PathBuf> {
    let name = target.binary_name();

    // 1. Verificar no mesmo diretório do executável mandu
    if let Some(dir) = current_exe_dir {
        let candidate = dir.join(name);
        if candidate.is_file() {
            return Some(candidate);
        }
    }

    // 2. Verificar nos diretórios do PATH
    if let Some(paths) = path_env {
        for dir in env::split_paths(paths) {
            let candidate = dir.join(name);
            if candidate.is_file() {
                return Some(candidate);
            }
        }
    }

    None
}

/// Executa a resolução completa com fallback automático de GUI para TUI se mandu-gui estiver ausente.
///
/// # Errors
/// Retorna erro caso nenhum binário seja localizado no PATH ou localmente.
pub fn resolve_and_find_binary(
    args: &[String],
    current_exe_dir: Option<&Path>,
) -> Result<(PathBuf, Vec<String>), String> {
    let has_wayland = env::var("WAYLAND_DISPLAY").is_ok_and(|v| !v.trim().is_empty());
    let has_x11 = env::var("DISPLAY").is_ok_and(|v| !v.trim().is_empty());
    let path_env = env::var("PATH").ok();

    let decision = resolve_target(args, has_wayland, has_x11);
    let target = decision.target;

    if let Some(bin) = find_binary(target, current_exe_dir, path_env.as_deref()) {
        return Ok((bin, decision.filtered_args));
    }

    // Fallback: se o alvo era GUI e não foi encontrado, tenta TUI como fallback
    if target == TargetMode::Gui {
        if let Some(tui_bin) = find_binary(TargetMode::Tui, current_exe_dir, path_env.as_deref()) {
            return Ok((tui_bin, decision.filtered_args));
        }
    }

    Err(format!(
        "Erro: Executável '{}' não foi encontrado no PATH nem no diretório local.",
        target.binary_name()
    ))
}
