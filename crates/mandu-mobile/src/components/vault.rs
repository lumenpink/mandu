use crate::state::NoteEntry;
use dioxus::prelude::*;
use std::path::PathBuf;

#[derive(Props, Clone, PartialEq)]
pub struct VaultDrawerProps {
    pub notes: Vec<NoteEntry>,
    pub current_vault_path: PathBuf,
    pub on_select_note: EventHandler<PathBuf>,
    pub on_create_note: EventHandler<String>,
    pub on_close: EventHandler<()>,
}

#[component]
pub fn VaultDrawer(props: VaultDrawerProps) -> Element {
    let mut new_title = use_signal(String::new);
    let mut search = use_signal(String::new);

    let filtered: Vec<_> = props
        .notes
        .iter()
        .filter(|n| {
            let q = search().to_lowercase();
            q.is_empty() || n.title.to_lowercase().contains(&q)
        })
        .cloned()
        .collect();

    rsx! {
        div {
            class: "vault-drawer",
            style: "position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0, 0, 0, 0.7); display: flex; z-index: 50;",
            div {
                style: "width: 80%; max-width: 360px; height: 100%; background: #18191f; border-right: 1px solid #2d3039; display: flex; flex-direction: column; padding: 16px; box-sizing: border-box;",
                div {
                    style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px;",
                    h2 {
                        style: "color: #f8f8f2; font-size: 18px; margin: 0;",
                        "Notas no Cofre"
                    }
                    button {
                        style: "background: transparent; color: #8be9fd; border: none; font-size: 18px; cursor: pointer;",
                        onclick: move |_| props.on_close.call(()),
                        "✕"
                    }
                }
                input {
                    r#type: "text",
                    placeholder: "Buscar notas...",
                    value: "{search}",
                    style: "width: 100%; padding: 8px 12px; background: #282a36; border: 1px solid #44475a; border-radius: 6px; color: #f8f8f2; margin-bottom: 12px; box-sizing: border-box;",
                    oninput: move |evt| search.set(evt.value()),
                }
                div {
                    style: "flex: 1; overflow-y: auto; margin-bottom: 12px;",
                    if filtered.is_empty() {
                        div {
                            style: "color: #6272a4; font-size: 14px; text-align: center; margin-top: 24px;",
                            "Nenhuma nota encontrada"
                        }
                    }
                    for note in filtered {
                        div {
                            key: "{note.relative_path}",
                            style: "padding: 10px 12px; border-bottom: 1px solid #282a36; color: #f8f8f2; cursor: pointer; border-radius: 4px;",
                            onclick: {
                                let p = note.path.clone();
                                move |_| props.on_select_note.call(p.clone())
                            },
                            "📄 {note.title}"
                        }
                    }
                }
                div {
                    style: "display: flex; gap: 8px;",
                    input {
                        r#type: "text",
                        placeholder: "Nova nota...",
                        value: "{new_title}",
                        style: "flex: 1; padding: 8px 12px; background: #282a36; border: 1px solid #44475a; border-radius: 6px; color: #f8f8f2;",
                        oninput: move |evt| new_title.set(evt.value()),
                    }
                    button {
                        style: "background: #50fa7b; color: #1e1f29; border: none; font-weight: bold; padding: 8px 12px; border-radius: 6px; cursor: pointer;",
                        onclick: move |_| {
                            let t = new_title().trim().to_string();
                            if !t.is_empty() {
                                props.on_create_note.call(t);
                                new_title.set(String::new());
                            }
                        },
                        "+"
                    }
                }
            }
            div {
                style: "flex: 1;",
                onclick: move |_| props.on_close.call(()),
            }
        }
    }
}
