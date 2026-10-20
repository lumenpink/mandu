//! Entrypoint binário do despachante unificado mandu.

use mandu_core::config::ManduConfig;
use mandu_core::updater::{
    atomic_replace_binary, detect_installed_binaries, get_distribution_endpoints, resolve_channel,
    semver_is_newer,
};
use std::env;
#[cfg(unix)]
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::{self, Command};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 && (args[1] == "--help" || args[1] == "-h") {
        print_help();
        return;
    }

    let has_wayland = env::var("WAYLAND_DISPLAY").is_ok_and(|v| !v.trim().is_empty());
    let has_x11 = env::var("DISPLAY").is_ok_and(|v| !v.trim().is_empty());
    let decision = mandu_cli::resolve_target(&args, has_wayland, has_x11);

    match decision.target {
        mandu_cli::TargetMode::CheckUpdate => {
            run_check_update(&decision.filtered_args);
            return;
        }
        mandu_cli::TargetMode::Update => {
            run_update(&decision.filtered_args);
            return;
        }
        _ => {}
    }

    let current_exe_dir = env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()));

    match mandu_cli::resolve_and_find_binary(&args, current_exe_dir.as_deref()) {
        Ok((bin_path, target_args)) => {
            #[cfg(unix)]
            {
                let err = Command::new(&bin_path).args(&target_args).exec();
                eprintln!("Erro ao substituir imagem de processo ({bin_path:?}): {err}");
                process::exit(1);
            }
            #[cfg(not(unix))]
            {
                let status = Command::new(&bin_path)
                    .args(&target_args)
                    .status()
                    .unwrap_or_else(|e| {
                        eprintln!("Erro ao executar processo ({bin_path:?}): {e}");
                        process::exit(1);
                    });
                process::exit(status.code().unwrap_or(1));
            }
        }
        Err(err_msg) => {
            eprintln!("{err_msg}");
            process::exit(127);
        }
    }
}

fn print_help() {
    println!("Mandu - Despachante Unificado");
    println!("Uso: mandu [--tui | --gui] [ARQUIVO] [--vault CAMINHO]");
    println!("     mandu update [--channel stable|nightly] [--pin VERSÃO] [--unpin]");
    println!("     mandu check-update [--channel stable|nightly]");
    println!();
    println!("Opções do despachante:");
    println!("  --tui         Força a interface de terminal (mandu-tui)");
    println!("  --gui         Força a interface gráfica (mandu-gui)");
    println!("  -h, --help    Exibe esta ajuda");
    println!();
    println!("Comandos de distribuição na Banca:");
    println!("  check-update  Verifica a disponibilidade de novas versões remotas");
    println!("  update        Atualiza atomicamente todos os binários instalados do Mandu");
}

fn parse_channel_arg(args: &[String]) -> Option<&str> {
    for (i, a) in args.iter().enumerate() {
        if (a == "--channel" || a == "-c") && i + 1 < args.len() {
            return Some(&args[i + 1]);
        }
    }
    None
}

fn parse_pin_arg(args: &[String]) -> Option<&str> {
    for (i, a) in args.iter().enumerate() {
        if (a == "--pin" || a == "-p") && i + 1 < args.len() {
            return Some(&args[i + 1]);
        }
    }
    None
}

fn run_check_update(args: &[String]) {
    let cfg = ManduConfig::load_hierarchical(None);
    let channel_arg = parse_channel_arg(args);
    let channel = resolve_channel(channel_arg, None, &cfg);

    if let Some(ref pin) = cfg.update.pin {
        println!("★ Mandu está fixado na versão {pin}. Atualizações automáticas suspensas.");
        println!("  Para remover a fixação, execute: mandu update --unpin");
        return;
    }

    let endpoints = get_distribution_endpoints(channel);
    println!("★ Consultando atualizações na Banca ({channel})...");
    println!("  Endpoint: {}", endpoints.version_url);

    match query_remote_version(&endpoints.version_url) {
        Ok(remote) => {
            let current = env!("CARGO_PKG_VERSION");
            if semver_is_newer(&remote, current) {
                println!("✔ Nova versão disponível: v{remote} (instalada: v{current})");
                println!("  Para atualizar, execute: mandu update --channel {channel}");
            } else {
                println!("✔ O Mandu já está na versão mais recente (v{current}, canal {channel}).");
            }
        }
        Err(err) => {
            eprintln!("⚠ Não foi possível verificar atualizações no momento: {err}");
        }
    }
}

fn run_update(args: &[String]) {
    let mut cfg = ManduConfig::load_hierarchical(None);

    if args.iter().any(|a| a == "--unpin" || a == "-u") {
        cfg.update.pin = None;
        if let Err(e) = cfg.save_global() {
            eprintln!("Erro ao gravar configuração global: {e}");
        } else {
            println!("✔ Fixação de versão removida. Atualizações automáticas retomadas.");
        }
        return;
    }

    if let Some(pin_ver) = parse_pin_arg(args) {
        cfg.update.pin = Some(pin_ver.to_string());
        if let Err(e) = cfg.save_global() {
            eprintln!("Erro ao gravar configuração global: {e}");
        } else {
            println!("✔ Mandu fixado com sucesso na versão {pin_ver}.");
        }
        return;
    }

    let channel_arg = parse_channel_arg(args);
    let channel = resolve_channel(channel_arg, None, &cfg);
    let endpoints = get_distribution_endpoints(channel);

    println!("★ Verificando atualizações na Banca ({channel})...");
    let current_exe = env::current_exe().unwrap_or_else(|_| std::path::PathBuf::from("mandu"));
    let bin_dir = current_exe.parent().unwrap_or_else(|| Path::new("."));
    let installed = detect_installed_binaries(bin_dir);

    println!("  Diretório de instalação: {}", bin_dir.display());
    println!(
        "  Binários detectados: {}",
        installed
            .iter()
            .filter_map(|p| p.file_name().map(|f| f.to_string_lossy().to_string()))
            .collect::<Vec<_>>()
            .join(", ")
    );

    let mut updated_count = 0;
    for bin_path in &installed {
        let name = bin_path.file_name().unwrap().to_string_lossy();
        let download_url = format!("{}{name}", endpoints.binary_base_url);
        println!("  Baixando {name} de {download_url}...");
        match download_binary_bytes(&download_url) {
            Ok(bytes) => {
                if let Err(e) = atomic_replace_binary(bin_path, &bytes) {
                    eprintln!("  ✗ Erro ao atualizar {name}: {e}");
                } else {
                    println!("  ✔ {name} atualizado com sucesso!");
                    updated_count += 1;
                }
            }
            Err(e) => {
                eprintln!("  ⚠ Falha ao transferir {name}: {e}");
            }
        }
    }

    if updated_count > 0 {
        println!("✔ Atualização concluída com sucesso ({updated_count} binários atualizados).");
    } else {
        println!("⚠ Nenhum binário pôde ser atualizado. Verifique a conexão com a Banca.");
    }
}

fn query_remote_version(url: &str) -> Result<String, String> {
    let output = Command::new("curl")
        .args(["-fsSL", "--max-time", "5", url])
        .output()
        .map_err(|e| format!("Falha ao invocar curl: {e}"))?;

    if output.status.success() {
        let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !text.is_empty() {
            return Ok(text);
        }
    }
    Err("Resposta vazia ou erro HTTP na consulta".to_string())
}

fn download_binary_bytes(url: &str) -> Result<Vec<u8>, String> {
    let output = Command::new("curl")
        .args(["-fsSL", "--max-time", "30", url])
        .output()
        .map_err(|e| format!("Falha ao invocar curl: {e}"))?;

    if output.status.success() && !output.stdout.is_empty() {
        Ok(output.stdout)
    } else {
        Err(format!(
            "Erro HTTP ou resposta vazia ({})",
            output.status.code().unwrap_or(1)
        ))
    }
}
