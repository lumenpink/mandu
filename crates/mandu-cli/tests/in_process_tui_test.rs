use mandu_cli::{resolve_target, TargetMode};

#[test]
fn test_resolve_target_tui_flag() {
    let args = vec!["mandu".to_string(), "--tui".to_string()];
    let decision = resolve_target(&args, true, true);
    assert_eq!(decision.target, TargetMode::Tui);
    assert!(decision.filtered_args.is_empty());
}

#[test]
fn test_resolve_target_headless_defaults_to_tui() {
    let args = vec!["mandu".to_string()];
    let decision = resolve_target(&args, false, false);
    assert_eq!(decision.target, TargetMode::Tui);
}

#[test]
fn test_mandu_cli_exports_in_process_tui_runner() {
    // Verifica que mandu-cli expõe suporte TUI embutido diretamente in-process
    assert!(mandu_cli::has_embedded_tui());
}

#[test]
fn test_parse_vault_and_file_args_variants() {
    // Caso 1: --vault <caminho> e arquivo posicional
    let args1 = vec![
        "--vault".to_string(),
        "/meu/vault".to_string(),
        "minha_nota.md".to_string(),
    ];
    let (v1, f1) = mandu_cli::parse_vault_and_file_args(&args1);
    assert_eq!(v1, Some(std::path::PathBuf::from("/meu/vault")));
    assert_eq!(f1, Some(std::path::PathBuf::from("minha_nota.md")));

    // Caso 2: --vault=<caminho>
    let args2 = vec!["--vault=/outro/vault".to_string()];
    let (v2, f2) = mandu_cli::parse_vault_and_file_args(&args2);
    assert_eq!(v2, Some(std::path::PathBuf::from("/outro/vault")));
    assert_eq!(f2, None);

    // Caso 3: -v <caminho>
    let args3 = vec!["-v".to_string(), "/curto/vault".to_string()];
    let (v3, f3) = mandu_cli::parse_vault_and_file_args(&args3);
    assert_eq!(v3, Some(std::path::PathBuf::from("/curto/vault")));
    assert_eq!(f3, None);

    // Caso 4: sem argumentos
    let args4: Vec<String> = vec![];
    let (v4, f4) = mandu_cli::parse_vault_and_file_args(&args4);
    assert_eq!(v4, None);
    assert_eq!(f4, None);
}
