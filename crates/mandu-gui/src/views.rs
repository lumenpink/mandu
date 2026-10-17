//! Painéis Editor (TextEdit) e Preview Vetorial para mandu-gui.

#[derive(Default)]
pub struct ViewsState {
    pub buffer: String,
    pub preview_svg: Option<String>,
}
