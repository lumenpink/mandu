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
