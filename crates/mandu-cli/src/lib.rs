//! Despachante inteligente CLI para o ecossistema Mandu.

use std::env;
use std::path::{Path, PathBuf};

/// Modo de destino da interface ou ação do Mandu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetMode {
    Tui,
    Gui,
    GuiHint,
    Update,
    CheckUpdate,
    VersionAll,
}

impl TargetMode {
    #[must_use]
    pub fn binary_name(self) -> &'static str {
        match self {
            Self::Tui => "mandu",
            Self::Gui | Self::GuiHint => "mandu-desktop",
            Self::Update | Self::CheckUpdate | Self::VersionAll => "mandu",
        }
    }
}

/// Decisão de despacho calculada a partir dos parâmetros de linha de comando e ambiente.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DispatchDecision {
    pub target: TargetMode,
    pub filtered_args: Vec<String>,
}

/// Determina o modo de execução (Tui, Gui, Update ou CheckUpdate) com base em argumentos CLI e variáveis de exibição.
#[must_use]
pub fn resolve_target(args: &[String], _has_wayland: bool, _has_x11: bool) -> DispatchDecision {
    let mut target = None;
    let mut filtered_args = Vec::new();

    let skip_first = if let Some(first) = args.first() {
        first.ends_with("mandu") || first.ends_with("mandu.exe")
    } else {
        false
    };

    let slice = if skip_first { &args[1..] } else { args };

    if slice.contains(&"--version".to_string()) && slice.contains(&"--all".to_string()) {
        return DispatchDecision {
            target: TargetMode::VersionAll,
            filtered_args: Vec::new(),
        };
    }

    // Subcomandos prioritários do despachante
    if let Some(first_cmd) = slice.first() {
        if first_cmd == "update" {
            return DispatchDecision {
                target: TargetMode::Update,
                filtered_args: slice[1..].to_vec(),
            };
        } else if first_cmd == "check-update" {
            return DispatchDecision {
                target: TargetMode::CheckUpdate,
                filtered_args: slice[1..].to_vec(),
            };
        }
    }

    for arg in slice {
        match arg.as_str() {
            "--tui" => target = Some(TargetMode::Tui),
            "--gui" => target = Some(TargetMode::GuiHint),
            _ => filtered_args.push(arg.clone()),
        }
    }

    // O binário mandu é estritamente de terminal; seu padrão é sempre Tui
    let resolved_target = target.unwrap_or(TargetMode::Tui);

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

    if target == TargetMode::Update
        || target == TargetMode::CheckUpdate
        || target == TargetMode::Tui
    {
        let self_bin = current_exe_dir
            .map(|d| d.join("mandu"))
            .unwrap_or_else(|| PathBuf::from("mandu"));
        return Ok((self_bin, decision.filtered_args));
    }

    if let Some(bin) = find_binary(target, current_exe_dir, path_env.as_deref()) {
        return Ok((bin, decision.filtered_args));
    }

    // Fallback: se o alvo era GUI e não foi encontrado, recai para TUI no próprio binário
    let self_bin = current_exe_dir
        .map(|d| d.join("mandu"))
        .unwrap_or_else(|| PathBuf::from("mandu"));
    Ok((self_bin, decision.filtered_args))
}

/// Informa se o despachante possui o motor TUI embutido diretamente in-process.
#[must_use]
pub fn has_embedded_tui() -> bool {
    true
}

/// Extrai caminhos opcionais de vault e arquivo dos argumentos filtrados.
#[must_use]
pub fn parse_vault_and_file_args(args: &[String]) -> (Option<PathBuf>, Option<PathBuf>) {
    let mut vault = None;
    let mut file = None;
    let mut skip_next = false;

    for (i, arg) in args.iter().enumerate() {
        if skip_next {
            skip_next = false;
            continue;
        }

        if (arg == "--vault" || arg == "-v") && i + 1 < args.len() {
            vault = Some(PathBuf::from(&args[i + 1]));
            skip_next = true;
        } else if let Some(stripped) = arg.strip_prefix("--vault=") {
            vault = Some(PathBuf::from(stripped));
        } else if !arg.starts_with('-') && file.is_none() {
            file = Some(PathBuf::from(arg));
        }
    }

    (vault, file)
}

/// Executa a interface TUI diretamente no processo corrente sem necessidade de binário externo.
///
/// # Errors
/// Retorna erro caso ocorra falha na inicialização do terminal ou no loop de eventos.
pub fn run_embedded_tui(args: &[String]) -> anyhow::Result<()> {
    let (raw_vault, raw_file) = parse_vault_and_file_args(args);
    let (vault, file) = mandu_core::vault::resolve_vault_and_file(raw_vault, raw_file);
    mandu_tui::run(Some(vault), file)
}
