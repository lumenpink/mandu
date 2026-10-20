use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    Editor,
    Preview,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NoteEntry {
    pub title: String,
    pub path: PathBuf,
    pub relative_path: String,
}

#[derive(Debug, Clone)]
pub struct MobileState {
    pub vault_path: PathBuf,
    pub notes: Vec<NoteEntry>,
    pub active_note: Option<NoteEntry>,
    pub buffer: String,
    pub is_dirty: bool,
    pub view_mode: ViewMode,
    pub search_query: String,
    pub show_vault_drawer: bool,
    pub status: String,
}

impl MobileState {
    pub fn new<P: Into<PathBuf>>(vault_path: P) -> Self {
        let path = vault_path.into();
        let mut state = Self {
            vault_path: path,
            notes: Vec::new(),
            active_note: None,
            buffer: String::new(),
            is_dirty: false,
            view_mode: ViewMode::Editor,
            search_query: String::new(),
            show_vault_drawer: false,
            status: "Pronto".to_string(),
        };
        state.scan_vault();
        state
    }

    pub fn scan_vault(&mut self) {
        self.notes.clear();
        if !self.vault_path.exists() {
            let _ = fs::create_dir_all(&self.vault_path);
        }

        if let Ok(entries) = fs::read_dir(&self.vault_path) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_file() && p.extension().map(|e| e == "md").unwrap_or(false) {
                    let file_name = p
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("sem_titulo.md");
                    let title = p
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or(file_name)
                        .to_string();

                    self.notes.push(NoteEntry {
                        title,
                        path: p.clone(),
                        relative_path: file_name.to_string(),
                    });
                }
            }
        }

        self.notes.sort_by(|a, b| a.title.cmp(&b.title));

        if self.active_note.is_none() && !self.notes.is_empty() {
            let first = self.notes[0].clone();
            self.select_note(&first.path);
        }
    }

    pub fn select_note(&mut self, path: &Path) {
        if let Ok(content) = fs::read_to_string(path) {
            let file_name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("nota.md");
            let title = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or(file_name)
                .to_string();

            self.active_note = Some(NoteEntry {
                title,
                path: path.to_path_buf(),
                relative_path: file_name.to_string(),
            });
            self.buffer = content;
            self.is_dirty = false;
            self.show_vault_drawer = false;
            self.status = "Nota carregada".to_string();
        }
    }

    pub fn save_current_note(&mut self) -> Result<(), String> {
        let note = self.active_note.as_ref().ok_or("Nenhuma nota aberta")?;
        let normalized = if self.buffer.ends_with('\n') {
            self.buffer.clone()
        } else {
            format!("{}\n", self.buffer)
        };

        fs::write(&note.path, &normalized).map_err(|e| e.to_string())?;
        self.buffer = normalized;
        self.is_dirty = false;
        self.status = "✓ Salvo".to_string();
        Ok(())
    }

    pub fn create_note(&mut self, title: &str) -> Result<(), String> {
        let clean_title = title.trim();
        if clean_title.is_empty() {
            return Err("Título não pode ser vazio".to_string());
        }

        let file_name = format!("{clean_title}.md");
        let path = self.vault_path.join(&file_name);
        let initial_content = format!("# {clean_title}\n\n");

        fs::write(&path, &initial_content).map_err(|e| e.to_string())?;
        self.scan_vault();
        self.select_note(&path);
        Ok(())
    }

    pub fn apply_toolbar_action(&mut self, action: &str) {
        match action {
            "wikilink" => {
                self.buffer.push_str("[[nota]]");
            }
            "heading" => {
                if !self.buffer.ends_with('\n') && !self.buffer.is_empty() {
                    self.buffer.push('\n');
                }
                self.buffer.push_str("# ");
            }
            "task" => {
                if !self.buffer.ends_with('\n') && !self.buffer.is_empty() {
                    self.buffer.push('\n');
                }
                self.buffer.push_str("- [ ] ");
            }
            "bold" => {
                self.buffer.push_str("**negrito**");
            }
            "italic" => {
                self.buffer.push_str("*itálico*");
            }
            "math" => {
                self.buffer.push_str("$formula$");
            }
            "code" => {
                self.buffer.push_str("\n```\n\n```\n");
            }
            _ => {}
        }
        self.is_dirty = true;
    }

    pub fn filtered_notes(&self) -> Vec<&NoteEntry> {
        let q = self.search_query.trim().to_lowercase();
        if q.is_empty() {
            self.notes.iter().collect()
        } else {
            self.notes
                .iter()
                .filter(|n| n.title.to_lowercase().contains(&q))
                .collect()
        }
    }
}
