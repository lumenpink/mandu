//! Entrypoint binário do despachante unificado mandu.

use std::env;
#[cfg(unix)]
use std::os::unix::process::CommandExt;
use std::process::{self, Command};

fn main() {
    let args: Vec<String> = env::args().collect();

    // Ajuda resumida do despachante
    if args.len() == 2 && (args[1] == "--help" || args[1] == "-h") {
        println!("Mandu - Despachante Unificado");
        println!("Uso: mandu [--tui | --gui] [ARQUIVO] [--vault CAMINHO]");
        println!();
        println!("Opções do despachante:");
        println!("  --tui         Força a interface de terminal (mandu-tui)");
        println!("  --gui         Força a interface gráfica (mandu-gui)");
        println!("  -h, --help    Exibe esta ajuda");
        println!();
        println!("Sem opções, mandu seleciona mandu-gui se houver display ativo ($WAYLAND_DISPLAY ou $DISPLAY),");
        println!("ou mandu-tui caso contrário (com fallback automático se mandu-gui não estiver instalado).");
        return;
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
