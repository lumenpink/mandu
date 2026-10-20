use super::toolbar::QuickToolbar;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct NoteEditorProps {
    pub content: String,
    pub on_change: EventHandler<String>,
    pub on_toolbar_action: EventHandler<String>,
}

#[component]
pub fn NoteEditor(props: NoteEditorProps) -> Element {
    rsx! {
        div {
            class: "note-editor-container",
            style: "flex: 1; display: flex; flex-direction: column; height: 100%; background: #121316;",
            textarea {
                class: "note-textarea",
                style: "flex: 1; width: 100%; border: none; outline: none; resize: none; padding: 16px; background: transparent; color: #f0f2f5; font-family: monospace; font-size: 16px; line-height: 1.5; box-sizing: border-box;",
                value: "{props.content}",
                placeholder: "Escreva suas anotações aqui...",
                oninput: move |evt| props.on_change.call(evt.value()),
            }
            QuickToolbar {
                on_action: move |action| props.on_toolbar_action.call(action),
            }
        }
    }
}
