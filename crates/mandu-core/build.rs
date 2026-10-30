use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let target = env::var("TARGET").unwrap_or_else(|_| "x86_64-unknown-linux-gnu".to_string());

    // 1. Verificar se existe biblioteca estática local construída no repositório joaju
    let local_paths = [
        PathBuf::from("/home/lumen/src/joaju/target/release"),
        PathBuf::from(format!("/home/lumen/src/joaju/target/{target}/release")),
    ];

    let mut found_dir = None;
    for p in &local_paths {
        if p.join("libjoaju.a").exists() || p.join("joaju.lib").exists() {
            found_dir = Some(p.clone());
            break;
        }
    }

    let link_dir = if let Some(dir) = found_dir {
        dir
    } else {
        // 2. Se não existir localmente, baixar o artefato da Forgejo Generic Packages
        let target_artifact = if target.contains("windows") {
            "libjoaju-x86_64-windows.a"
        } else if target.contains("aarch64") {
            "libjoaju-aarch64-linux.a"
        } else {
            "libjoaju-x86_64-linux.a"
        };

        let dest_lib = out_dir.join("libjoaju.a");
        if !dest_lib.exists() {
            let url = format!(
                "https://gambi.2lp.in/api/packages/lumen/generic/joaju-static/0.2.0/{}",
                target_artifact
            );

            let token = env::var("FORGEJO_TOKEN")
                .or_else(|_| env::var("GIT_2LP_TOKEN"))
                .unwrap_or_else(|_| "2c1912103419968a307522910a2ebe375fa69ecc".to_string());

            let status = Command::new("curl")
                .arg("-s")
                .arg("-f")
                .arg("-H")
                .arg(format!("Authorization: token {}", token))
                .arg("-o")
                .arg(&dest_lib)
                .arg(&url)
                .status();

            if status.is_err() || !status.unwrap().success() || !dest_lib.exists() {
                // Tenta fallback com nome padrão
                let fallback_url = "https://gambi.2lp.in/api/packages/lumen/generic/joaju-static/0.2.0/libjoaju-x86_64-linux.a";
                let _ = Command::new("curl")
                    .arg("-s")
                    .arg("-f")
                    .arg("-H")
                    .arg(format!("Authorization: token {}", token))
                    .arg("-o")
                    .arg(&dest_lib)
                    .arg(fallback_url)
                    .status();
            }
        }
        out_dir
    };

    println!("cargo:rustc-link-search=native={}", link_dir.display());
    println!("cargo:rustc-link-lib=static=joaju");
    println!("cargo:rustc-link-arg=-Wl,--allow-multiple-definition");
    println!("cargo:rerun-if-changed=build.rs");
}
