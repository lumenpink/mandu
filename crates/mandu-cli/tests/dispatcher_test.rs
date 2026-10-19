use mandu_cli::{find_binary, resolve_target, TargetMode};
use std::fs::File;
use tempfile::tempdir;

#[test]
fn test_dispatcher_override_flags() {
    // Flag --tui força Tui mesmo se houver display gráfico
    let args = vec!["mandu".to_string(), "--tui".to_string(), "nota.md".to_string()];
    let decision = resolve_target(&args, true, true);
    assert_eq!(decision.target, TargetMode::Tui);
    assert_eq!(decision.filtered_args, vec!["nota.md"]);

    // Flag --gui força Gui mesmo em ambiente sem display
    let args_gui = vec!["mandu".to_string(), "--gui".to_string(), "tese.md".to_string()];
    let decision_gui = resolve_target(&args_gui, false, false);
    assert_eq!(decision_gui.target, TargetMode::Gui);
    assert_eq!(decision_gui.filtered_args, vec!["tese.md"]);
}

#[test]
fn test_dispatcher_environment_detection() {
    let args = vec!["mandu".to_string(), "nota.md".to_string()];

    // Sem servidor gráfico -> Tui
    let decision_no_display = resolve_target(&args, false, false);
    assert_eq!(decision_no_display.target, TargetMode::Tui);
    assert_eq!(decision_no_display.filtered_args, vec!["nota.md"]);

    // Com Wayland -> Gui
    let decision_wayland = resolve_target(&args, true, false);
    assert_eq!(decision_wayland.target, TargetMode::Gui);

    // Com X11 -> Gui
    let decision_x11 = resolve_target(&args, false, true);
    assert_eq!(decision_x11.target, TargetMode::Gui);
}

#[test]
fn test_dispatcher_binary_discovery_and_fallback() {
    let dir = tempdir().expect("tempdir");
    let tui_path = dir.path().join("mandu-tui");
    File::create(&tui_path).expect("create dummy mandu-tui");

    // Localizar mandu-tui no diretório local
    let found_tui = find_binary(TargetMode::Tui, Some(dir.path()), None);
    assert!(found_tui.is_some());
    assert_eq!(found_tui.unwrap(), tui_path);

    // mandu-gui não existe no dir, deve retornar None
    let found_gui = find_binary(TargetMode::Gui, Some(dir.path()), None);
    assert!(found_gui.is_none());
}
