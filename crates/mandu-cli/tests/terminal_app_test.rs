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
