use dioxus::prelude::*;
use pulldown_cmark::{html, Options, Parser};

#[derive(Props, Clone, PartialEq)]
pub struct NotePreviewProps {
    pub content: String,
}

pub fn markdown_to_html(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(markdown, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

#[component]
pub fn NotePreview(props: NotePreviewProps) -> Element {
    let rendered = markdown_to_html(&props.content);

    rsx! {
        div {
            class: "note-preview",
            style: "flex: 1; padding: 16px; overflow-y: auto; background: #121316; color: #e1e3e8; font-family: sans-serif; line-height: 1.6;",
            div {
                dangerous_inner_html: "{rendered}",
            }
        }
    }
}
