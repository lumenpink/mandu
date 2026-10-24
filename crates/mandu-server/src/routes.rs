use crate::state::AppState;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use mandu_core::preview::PreviewEngine;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub version: &'static str,
}

#[derive(Serialize)]
pub struct ConfigResponse {
    pub auth_logout_url: String,
    pub port: u16,
    pub vault_path: String,
}

#[derive(Serialize)]
pub struct VaultItem {
    pub title: String,
    pub relative_path: String,
    pub is_dir: bool,
}

#[derive(Deserialize)]
pub struct NoteQuery {
    pub path: String,
}

#[derive(Serialize, Deserialize)]
pub struct NotePayload {
    pub title: String,
    pub relative_path: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct SaveNotePayload {
    pub content: String,
}

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: String,
}

#[derive(Serialize)]
pub struct SearchResultResponse {
    pub path: String,
    pub title: String,
    pub snippet: String,
}

#[derive(Deserialize)]
pub struct RenderPayload {
    pub engine: Option<String>,
    pub content: String,
}

#[derive(Serialize)]
pub struct RenderResponse {
    pub output: String,
}

#[derive(Deserialize)]
pub struct AiExtractPayload {
    pub content: String,
}

#[derive(Serialize)]
pub struct AiExtractResponse {
    pub summary: String,
    pub keywords: Vec<String>,
}

fn sanitize_relative_path(root: &Path, rel: &str) -> Result<PathBuf, StatusCode> {
    let path = Path::new(rel);
    if path.is_absolute() {
        return Err(StatusCode::BAD_REQUEST);
    }
    for component in path.components() {
        match component {
            std::path::Component::Normal(_) => {}
            _ => return Err(StatusCode::BAD_REQUEST),
        }
    }
    Ok(root.join(path))
}

pub fn create_app(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_handler))
        .route("/api/config", get(config_handler))
        .route("/api/tree", get(tree_handler))
        .route("/api/notes", get(get_note_handler).put(save_note_handler))
        .route("/api/search", get(search_handler))
        .route("/api/render/joaju", post(render_handler))
        .route("/api/ai/extract", post(ai_extract_handler))
        .with_state(state)
}

async fn health_handler() -> impl IntoResponse {
    Json(HealthResponse {
        status: "ok",
        version: mandu_core::VERSION,
    })
}

async fn config_handler(State(state): State<AppState>) -> impl IntoResponse {
    Json(ConfigResponse {
        auth_logout_url: state.config.auth_logout_url.clone(),
        port: state.config.port,
        vault_path: state.config.vault_path.display().to_string(),
    })
}

async fn tree_handler(State(state): State<AppState>) -> Result<Json<Vec<VaultItem>>, StatusCode> {
    let vault_root = &state.config.vault_path;
    if !vault_root.exists() {
        let _ = fs::create_dir_all(vault_root);
    }

    let mut items = Vec::new();
    for entry in WalkDir::new(vault_root)
        .into_iter()
        .filter_entry(|e| {
            e.depth() == 0
                || !e
                    .file_name()
                    .to_str()
                    .is_some_and(|name| name.starts_with('.'))
        })
        .flatten()
    {
        let path = entry.path();
        if path == vault_root {
            continue;
        }

        if let Ok(rel) = path.strip_prefix(vault_root) {
            let rel_str = rel.to_string_lossy().to_string();
            let is_dir = entry.file_type().is_dir();
            let is_md = path.extension().map(|e| e == "md").unwrap_or(false);

            if is_dir || is_md {
                let file_name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or(&rel_str);
                let title = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or(file_name)
                    .to_string();

                items.push(VaultItem {
                    title,
                    relative_path: rel_str,
                    is_dir,
                });
            }
        }
    }

    items.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));
    Ok(Json(items))
}

async fn get_note_handler(
    State(state): State<AppState>,
    Query(query): Query<NoteQuery>,
) -> Result<Json<NotePayload>, StatusCode> {
    let full_path = sanitize_relative_path(&state.config.vault_path, &query.path)?;
    if !full_path.exists() || !full_path.is_file() {
        return Err(StatusCode::NOT_FOUND);
    }

    let content = fs::read_to_string(&full_path).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let title = full_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(&query.path)
        .to_string();

    Ok(Json(NotePayload {
        title,
        relative_path: query.path,
        content,
    }))
}

async fn save_note_handler(
    State(state): State<AppState>,
    Query(query): Query<NoteQuery>,
    Json(payload): Json<SaveNotePayload>,
) -> Result<StatusCode, StatusCode> {
    let full_path = sanitize_relative_path(&state.config.vault_path, &query.path)?;

    if let Some(parent) = full_path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    // Aplica o linter de salvamento do mandu-core para normalizar formatação e timestamps
    let now = chrono::Local::now().to_rfc3339();
    let processed_content = mandu_core::linter::Linter::apply_save_hooks(&payload.content, &now);
    fs::write(&full_path, processed_content).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Atualiza a nota no índice FTS5 in-memory
    if let Ok(db) = state.db.lock() {
        let vault = mandu_core::vault::Vault::new(&state.config.vault_path);
        if let Ok(notes) = vault.scan() {
            let _ = db.index_notes_batch(&notes);
        }
    }

    Ok(StatusCode::OK)
}

async fn search_handler(
    State(state): State<AppState>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<Vec<SearchResultResponse>>, StatusCode> {
    let db = state
        .db
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let results = db
        .search_fts(&query.q)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = results
        .into_iter()
        .map(|r| SearchResultResponse {
            path: r.path,
            title: r.title,
            snippet: r.snippet,
        })
        .collect();

    Ok(Json(response))
}

async fn render_handler(
    State(state): State<AppState>,
    Json(payload): Json<RenderPayload>,
) -> Result<Json<RenderResponse>, StatusCode> {
    let engine = payload.engine.as_deref().unwrap_or("joaju");
    let db = state
        .db
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let svg = PreviewEngine::render_block_svg(engine, &payload.content, Some(&db))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(RenderResponse { output: svg }))
}

async fn ai_extract_handler(
    Json(payload): Json<AiExtractPayload>,
) -> Result<Json<AiExtractResponse>, StatusCode> {
    // Pipeline básico de sumarização e extração de tópicos semânticos
    let lines: Vec<&str> = payload.content.lines().collect();
    let summary = if lines.len() > 3 {
        lines[..3].join(" ")
    } else {
        payload.content.clone()
    };

    let mut keywords = Vec::new();
    for word in payload.content.split_whitespace() {
        let clean = word.trim_matches(|c: char| !c.is_alphanumeric());
        if clean.len() > 5 && !keywords.contains(&clean.to_string()) {
            keywords.push(clean.to_string());
            if keywords.len() >= 5 {
                break;
            }
        }
    }

    Ok(Json(AiExtractResponse { summary, keywords }))
}
