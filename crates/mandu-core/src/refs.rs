//! Extrator e resolvedor de referências bibliográficas e links internos [[nota]].

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceMd {
    pub citekey: String,
    pub note_path: String,
    pub doi: Option<String>,
    pub title: String,
    pub authors_json: Option<String>,
    pub year: Option<i32>,
    pub csl_yaml: Option<String>,
}
