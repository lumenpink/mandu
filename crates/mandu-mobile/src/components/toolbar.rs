use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct QuickToolbarProps {
    pub on_action: EventHandler<String>,
}

#[component]
pub fn QuickToolbar(props: QuickToolbarProps) -> Element {
    let actions = [
        ("[[ ]]", "wikilink", "Wiki Link"),
        ("#", "heading", "Cabeçalho"),
        ("- [ ]", "task", "Tarefa"),
        ("**B**", "bold", "Negrito"),
        ("*I*", "italic", "Itálico"),
        ("$fx$", "math", "Fórmula"),
        ("```", "code", "Bloco de Código"),
    ];

    rsx! {
        div {
            class: "quick-toolbar",
            style: "display: flex; gap: 8px; overflow-x: auto; padding: 8px 12px; background: #1c1d22; border-top: 1px solid #2d3039; position: sticky; bottom: 0; z-index: 10;",
            for (label, action, title) in actions {
                button {
                    key: "{action}",
                    r#type: "button",
                    title: "{title}",
                    style: "background: #282a36; color: #f8f8f2; border: 1px solid #44475a; border-radius: 6px; padding: 6px 12px; font-size: 14px; font-weight: 600; white-space: nowrap; cursor: pointer;",
                    onclick: move |_| props.on_action.call(action.to_string()),
                    "{label}"
                }
            }
        }
    }
}
