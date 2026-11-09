use mandu_cli::{resolve_target, TargetMode};

#[test]
fn test_terminal_app_always_defaults_to_tui() {
    let args = vec!["mandu".to_string(), "nota.md".to_string()];

    // Mesmo com Wayland ou X11 ativos, mandu é o app de terminal e roda TUI
    let decision = resolve_target(&args, true, true);
    assert_eq!(decision.target, TargetMode::Tui);
    assert_eq!(decision.filtered_args, vec!["nota.md"]);

    let decision_no_display = resolve_target(&args, false, false);
    assert_eq!(decision_no_display.target, TargetMode::Tui);
}

#[test]
fn test_terminal_app_gui_hint() {
    let args_gui = vec!["mandu".to_string(), "--gui".to_string()];
    let decision_gui = resolve_target(&args_gui, true, true);
    assert_eq!(decision_gui.target, TargetMode::GuiHint);
}

#[test]
fn test_mutation_target_mode_binary_names() {
    assert_eq!(TargetMode::Tui.binary_name(), "mandu");
    assert_eq!(TargetMode::Gui.binary_name(), "mandu-desktop");
    assert_eq!(TargetMode::GuiHint.binary_name(), "mandu-desktop");
    assert_eq!(TargetMode::Update.binary_name(), "mandu");
    assert_eq!(TargetMode::CheckUpdate.binary_name(), "mandu");
    assert_eq!(TargetMode::VersionAll.binary_name(), "mandu");
}

#[test]
fn test_mutation_gui_hint_positions_and_filtered_args() {
    let args_end = vec![
        "mandu".to_string(),
        "artigo.md".to_string(),
        "--gui".to_string(),
    ];
    let dec_end = resolve_target(&args_end, false, false);
    assert_eq!(dec_end.target, TargetMode::GuiHint);
    assert_eq!(dec_end.filtered_args, vec!["artigo.md"]);

    let args_mid = vec![
        "mandu".to_string(),
        "--gui".to_string(),
        "--vault".to_string(),
        "/tmp/cofre".to_string(),
    ];
    let dec_mid = resolve_target(&args_mid, true, true);
    assert_eq!(dec_mid.target, TargetMode::GuiHint);
    assert_eq!(dec_mid.filtered_args, vec!["--vault", "/tmp/cofre"]);
}
