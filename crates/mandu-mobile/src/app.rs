use crate::components::{NoteEditor, NotePreview, VaultDrawer};
use crate::state::{MobileState, ViewMode};
use dioxus::prelude::*;
use std::path::PathBuf;

#[derive(Props, Clone, PartialEq)]
pub struct AppProps {
    pub initial_vault: Option<PathBuf>,
}

#[component]
pub fn App(props: AppProps) -> Element {
    let vault_dir = props
        .initial_vault
        .clone()
        .unwrap_or_else(|| PathBuf::from("notes"));
    let mut state = use_signal(move || MobileState::new(vault_dir));

    let current_title = state()
        .active_note
        .as_ref()
        .map(|n| n.title.clone())
        .unwrap_or_else(|| "Sem nota selecionada".to_string());
    let current_content = state().buffer.clone();
    let is_dirty = state().is_dirty;
    let mode = state().view_mode;
    let is_drawer_open = state().show_vault_drawer;
    let status_text = state().status.clone();
    let word_count = current_content.split_whitespace().count();

    rsx! {
        div {
            class: "mandu-app",
            style: "display: flex; flex-direction: column; width: 100vw; height: 100vh; overflow: hidden; background: #121316; color: #f0f2f5;",
            // Top App Bar
            header {
                style: "display: flex; justify-content: space-between; align-items: center; padding: 10px 16px; background: #18191f; border-bottom: 1px solid #2d3039;",
                div {
                    style: "display: flex; align-items: center; gap: 10px;",
                    button {
                        style: "background: #282a36; color: #f8f8f2; border: 1px solid #44475a; border-radius: 6px; padding: 6px 10px; cursor: pointer;",
                        onclick: move |_| {
                            state.write().show_vault_drawer = true;
                        },
                        "☰ Cofre"
                    }
                    span {
                        style: "font-weight: bold; font-size: 16px; max-width: 180px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;",
                        "{current_title}"
                    }
                    if is_dirty {
                        span {
                            style: "color: #ffb86c; font-size: 12px;",
                            "[●]"
                        }
                    }
                }
                div {
                    style: "display: flex; gap: 8px;",
                    button {
                        style: if mode == ViewMode::Editor {
                            "background: #bd93f9; color: #1e1f29; border: none; font-weight: bold; border-radius: 6px; padding: 6px 12px; cursor: pointer;"
                        } else {
                            "background: #282a36; color: #f8f8f2; border: 1px solid #44475a; border-radius: 6px; padding: 6px 12px; cursor: pointer;"
                        },
                        onclick: move |_| {
                            state.write().view_mode = ViewMode::Editor;
                        },
                        "Editar"
                    }
                    button {
                        style: if mode == ViewMode::Preview {
                            "background: #bd93f9; color: #1e1f29; border: none; font-weight: bold; border-radius: 6px; padding: 6px 12px; cursor: pointer;"
                        } else {
                            "background: #282a36; color: #f8f8f2; border: 1px solid #44475a; border-radius: 6px; padding: 6px 12px; cursor: pointer;"
                        },
                        onclick: move |_| {
                            state.write().view_mode = ViewMode::Preview;
                        },
                        "Prévia"
                    }
                    button {
                        style: "background: #50fa7b; color: #1e1f29; border: none; font-weight: bold; border-radius: 6px; padding: 6px 12px; cursor: pointer;",
                        onclick: move |_| {
                            let _ = state.write().save_current_note();
                        },
                        "Salvar"
                    }
                }
            }

            // Main View Area
            main {
                style: "flex: 1; display: flex; flex-direction: column; overflow: hidden;",
                if mode == ViewMode::Editor {
                    NoteEditor {
                        content: current_content.clone(),
                        on_change: move |new_val: String| {
                            let mut s = state.write();
                            s.buffer = new_val;
                            s.is_dirty = true;
                        },
                        on_toolbar_action: move |action: String| {
                            state.write().apply_toolbar_action(&action);
                        },
                    }
                } else {
                    NotePreview {
                        content: current_content.clone(),
                    }
                }
            }

            // Bottom Status Bar
            footer {
                style: "display: flex; justify-content: space-between; align-items: center; padding: 4px 16px; background: #18191f; border-top: 1px solid #2d3039; font-size: 12px; color: #8be9fd;",
                span { "{status_text}" }
                span { "{word_count} palavras" }
            }

            // Vault Drawer Modal
            if is_drawer_open {
                VaultDrawer {
                    notes: state().notes.clone(),
                    current_vault_path: state().vault_path.clone(),
                    on_select_note: move |p: PathBuf| {
                        state.write().select_note(&p);
                    },
                    on_create_note: move |t: String| {
                        let _ = state.write().create_note(&t);
                    },
                    on_close: move |_| {
                        state.write().show_vault_drawer = false;
                    },
                }
            }
        }
    }
}
