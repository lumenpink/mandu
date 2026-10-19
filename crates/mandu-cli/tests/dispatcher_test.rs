use mandu_cli::{find_binary, resolve_target, TargetMode};
use std::fs::File;
use tempfile::tempdir;

#[test]
fn test_dispatcher_override_flags() {
    // Flag --tui força Tui mesmo se houver display gráfico
    let args = vec![
        "mandu".to_string(),
        "--tui".to_string(),
        "nota.md".to_string(),
    ];
    let decision = resolve_target(&args, true, true);
    assert_eq!(decision.target, TargetMode::Tui);
    assert_eq!(decision.filtered_args, vec!["nota.md"]);

    // Flag --gui força Gui mesmo em ambiente sem display
    let args_gui = vec![
        "mandu".to_string(),
        "--gui".to_string(),
        "tese.md".to_string(),
    ];
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

#[test]
fn test_mutation_dispatcher_empty_args_and_flag_positions() {
    // 1. Chamada sem argumentos
    let empty_args: Vec<String> = vec![];
    let dec_empty = resolve_target(&empty_args, false, false);
    assert_eq!(dec_empty.target, TargetMode::Tui);
    assert!(dec_empty.filtered_args.is_empty());

    // 2. Chamada com argumentos múltiplos preservando ordem
    let args_multi = vec![
        "mandu".to_string(),
        "artigo.md".to_string(),
        "--vault".to_string(),
        "/meu/cofre".to_string(),
    ];
    let dec_multi = resolve_target(&args_multi, false, false);
    assert_eq!(dec_multi.target, TargetMode::Tui);
    assert_eq!(
        dec_multi.filtered_args,
        vec!["artigo.md", "--vault", "/meu/cofre"]
    );

    // 3. Flag no meio ou final dos argumentos
    let args_middle = vec![
        "mandu".to_string(),
        "artigo.md".to_string(),
        "--tui".to_string(),
        "--vault".to_string(),
        "/meu/cofre".to_string(),
    ];
    let dec_middle = resolve_target(&args_middle, true, true);
    assert_eq!(dec_middle.target, TargetMode::Tui);
    assert_eq!(
        dec_middle.filtered_args,
        vec!["artigo.md", "--vault", "/meu/cofre"]
    );
}

#[test]
fn test_mutation_dispatcher_path_and_fallback_resolution() {
    let dir = tempdir().expect("tempdir");
    let tui_path = dir.path().join("mandu-tui");
    File::create(&tui_path).expect("create dummy mandu-tui");

    // Simular que PATH aponta para o diretório temporário
    let path_str = dir.path().to_string_lossy().to_string();

    // Procurar com current_exe_dir = None e PATH informado
    let found_via_path = find_binary(TargetMode::Tui, None, Some(&path_str));
    assert!(found_via_path.is_some());
    assert_eq!(found_via_path.unwrap(), tui_path);

    // Fallback completo: se chamamos em ambiente gráfico onde mandu-gui não existe,
    // o fallback automático para mandu-tui deve encontrar mandu-tui
    let args = vec!["mandu".to_string(), "documento.md".to_string()];
    let res = mandu_cli::resolve_and_find_binary(&args, Some(dir.path()));
    assert!(res.is_ok());
    let (found_bin, filtered) = res.unwrap();
    assert_eq!(found_bin, tui_path);
    assert_eq!(filtered, vec!["documento.md"]);
}
