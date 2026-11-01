use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let target = env::var("TARGET").unwrap_or_else(|_| "x86_64-unknown-linux-gnu".to_string());

    // 1. Verificar se existe biblioteca estática local construída no repositório joaju
    let mut local_paths = vec![PathBuf::from(format!(
        "/home/lumen/src/joaju/target/{target}/release"
    ))];
    if target.contains("x86_64-unknown-linux") {
        local_paths.push(PathBuf::from(
            "/home/lumen/src/joaju/target/x86_64-unknown-linux-musl/release",
        ));
        local_paths.push(PathBuf::from("/home/lumen/src/joaju/target/release"));
    }

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
            let version =
                env::var("JOAJU_STATIC_VERSION").unwrap_or_else(|_| "latest".to_string());
            let url = format!(
                "https://gambi.2lp.in/api/packages/lumen/generic/joaju-static/{}/{}",
                version, target_artifact
            );

            let token = env::var("GAMBI_2LP_IN_TOKEN")
                .or_else(|_| env::var("GIT_2LP_TOKEN"))
                .or_else(|_| env::var("REGISTRY_TOKEN"))
                .or_else(|_| env::var("LUMEN_REGISTRY_TOKEN"))
                .or_else(|_| env::var("FORGEJO_TOKEN"))
                .unwrap_or_else(|_| "2c1912103419968a307522910a2ebe375fa69ecc".to_string());
            let token = token.strip_prefix("Bearer ").unwrap_or(&token).to_string();

            let status = Command::new("curl")
                .arg("-s")
                .arg("-f")
                .arg("--http1.1")
                .arg("--retry")
                .arg("3")
                .arg("-H")
                .arg(format!("Authorization: token {}", token))
                .arg("-o")
                .arg(&dest_lib)
                .arg(&url)
                .status();

            if status.is_err() || !status.unwrap().success() || !dest_lib.exists() {
                // Fallback resiliente para versao 0.2.0 se latest falhar
                let fallback_url = format!(
                    "https://gambi.2lp.in/api/packages/lumen/generic/joaju-static/0.2.0/{}",
                    target_artifact
                );
                let fallback_status = Command::new("curl")
                    .arg("-s")
                    .arg("-f")
                    .arg("--http1.1")
                    .arg("--retry")
                    .arg("3")
                    .arg("-H")
                    .arg(format!("Authorization: token {}", token))
                    .arg("-o")
                    .arg(&dest_lib)
                    .arg(&fallback_url)
                    .status();

                if fallback_status.is_err()
                    || !fallback_status.unwrap().success()
                    || !dest_lib.exists()
                {
                    panic!(
                        "Falha ao baixar artefato estático joaju ({}) de {} (e fallback 0.2.0). Verifique conectividade e credenciais.",
                        target_artifact, url
                    );
                }
            }
        }
        out_dir
    };

    println!("cargo:rustc-link-search=native={}", link_dir.display());
    println!("cargo:rustc-link-lib=static=joaju");
    // Tolerar duplicidade de símbolos entre o runtime Rust contido no libjoaju.a e a libstd
    println!("cargo:rustc-link-arg=-Wl,--allow-multiple-definition");
    println!("cargo:rerun-if-changed=build.rs");
}
