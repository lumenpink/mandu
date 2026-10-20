use mandu_core::config::{ManduConfig, UpdateChannel, UpdateConfig};
use mandu_core::updater::{
    atomic_replace_binary, get_distribution_endpoints, resolve_channel, semver_is_newer,
    UpdateEndpoints,
};
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

#[test]
fn test_update_config_serialization_and_defaults() {
    let default_cfg = ManduConfig::default();
    assert_eq!(default_cfg.update.channel, UpdateChannel::Stable);
    assert_eq!(default_cfg.update.pin, None);
    assert!(default_cfg.update.auto_check);

    // Desserialização a partir de TOML
    let toml_str = r#"
        [update]
        channel = "nightly"
        pin = "0.2.5"
        auto_check = false
    "#;
    let cfg: ManduConfig = toml::from_str(toml_str).expect("deserialize toml");
    assert_eq!(cfg.update.channel, UpdateChannel::Nightly);
    assert_eq!(cfg.update.pin, Some("0.2.5".to_string()));
    assert!(!cfg.update.auto_check);
}

#[test]
fn test_updater_channel_resolution() {
    let mut cfg = ManduConfig::default();
    cfg.update.channel = UpdateChannel::Nightly;

    // Override explícito por argumento tem prioridade
    let ch_arg = resolve_channel(Some("stable"), None, &cfg);
    assert_eq!(ch_arg, UpdateChannel::Stable);

    // Variável de ambiente tem prioridade sobre o config se argumento for None
    let ch_env = resolve_channel(None, Some("nightly"), &cfg);
    assert_eq!(ch_env, UpdateChannel::Nightly);

    // Sem argumento e sem env, usa a preferência do config
    let ch_cfg = resolve_channel(None, None, &cfg);
    assert_eq!(ch_cfg, UpdateChannel::Nightly);
}

#[test]
fn test_updater_semver_comparison() {
    assert!(semver_is_newer("0.2.0", "0.1.0"));
    assert!(!semver_is_newer("0.1.0", "0.2.0"));
    assert!(!semver_is_newer("0.1.0", "0.1.0"));
    assert!(semver_is_newer("v0.2.0", "0.1.0"));
    assert!(semver_is_newer("0.1.0-dev.200", "0.1.0-dev.100"));
}

#[test]
fn test_updater_distribution_endpoints() {
    let endpoints_stable = get_distribution_endpoints(UpdateChannel::Stable);
    assert_eq!(
        endpoints_stable.version_url,
        "https://banca.2lp.in/downloads/mandu/version"
    );
    assert!(endpoints_stable.binary_base_url.contains("/downloads/mandu/stable-latest/"));

    let endpoints_nightly = get_distribution_endpoints(UpdateChannel::Nightly);
    assert_eq!(
        endpoints_nightly.version_url,
        "https://banca.2lp.in/downloads/mandu/nightly-version"
    );
    assert!(endpoints_nightly.binary_base_url.contains("/downloads/mandu/nightly-latest/"));
}

#[test]
fn test_updater_atomic_replace_and_rollback() {
    let dir = tempdir().expect("tempdir");
    let target = dir.path().join("dummy_bin");
    let mut f = File::create(&target).expect("create target");
    writeln!(f, "versao_antiga").expect("write target");

    // Novo conteúdo para atualizar
    let new_content = b"versao_nova";
    let replace_res = atomic_replace_binary(&target, new_content);
    assert!(replace_res.is_ok());

    let updated_content = std::fs::read_to_string(&target).expect("read updated");
    assert_eq!(updated_content.trim(), "versao_nova");

    // Backup .old deve existir
    let old_backup = dir.path().join("dummy_bin.old");
    assert!(old_backup.exists());
    let old_content = std::fs::read_to_string(&old_backup).expect("read old");
    assert_eq!(old_content.trim(), "versao_antiga");
}
