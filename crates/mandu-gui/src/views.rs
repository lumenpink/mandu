//! Painéis Editor (TextEdit) e Preview Vetorial para mandu-gui.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GuiViewMode {
    Source,
    #[default]
    Split,
    Preview,
}

impl GuiViewMode {
    #[must_use]
    pub fn next(self) -> Self {
        match self {
            Self::Source => Self::Split,
            Self::Split => Self::Preview,
            Self::Preview => Self::Source,
        }
    }

    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Self::Source => "Modo: Escrita",
            Self::Split => "Modo: Split",
            Self::Preview => "Modo: Preview",
        }
    }
}

#[derive(Default)]
pub struct ViewsState {
    pub mode: GuiViewMode,
    pub buffer: String,
    pub preview_svg: Option<String>,
    pub word_count: usize,
    pub backlink_count: usize,
    pub status_message: Option<String>,
    pub is_search_open: bool,
}
