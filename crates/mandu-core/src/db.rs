//! Banco de dados SQLite in-memory (FTS5) e cache BLAKE3.

use rusqlite::{Connection, Result};

pub struct ManduDb {
    conn: Connection,
}

impl ManduDb {
    pub fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        let db = Self { conn };
        db.init_schema()?;
        Ok(db)
    }

    pub fn init_schema(&self) -> Result<()> {
        self.conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS notes (
                path TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                blake3_hash TEXT NOT NULL,
                frontmatter_json TEXT,
                word_count INTEGER DEFAULT 0,
                modified_at TEXT
            );

            CREATE VIRTUAL TABLE IF NOT EXISTS notes_fts USING fts5(
                path,
                title,
                body
            );

            CREATE TABLE IF NOT EXISTS links (
                source_path TEXT NOT NULL,
                target_path TEXT NOT NULL,
                line_number INTEGER,
                context_snippet TEXT,
                PRIMARY KEY (source_path, target_path, line_number)
            );
            CREATE INDEX IF NOT EXISTS idx_backlinks ON links(target_path);

            CREATE TABLE IF NOT EXISTS tags (
                tag TEXT NOT NULL,
                note_path TEXT NOT NULL,
                PRIMARY KEY (tag, note_path)
            );

            CREATE TABLE IF NOT EXISTS references_md (
                citekey TEXT PRIMARY KEY,
                note_path TEXT NOT NULL,
                doi TEXT,
                title TEXT,
                authors_json TEXT,
                year INTEGER,
                csl_yaml TEXT
            );

            CREATE TABLE IF NOT EXISTS joaju_block_cache (
                block_blake3 TEXT PRIMARY KEY,
                engine TEXT NOT NULL,
                rendered_svg BLOB NOT NULL,
                created_at INTEGER
            );
            "#,
        )?;
        Ok(())
    }

    #[must_use]
    pub fn connection(&self) -> &Connection {
        &self.conn
    }

    pub fn index_note(&self, note: &crate::vault::Note) -> Result<()> {
        let path_str = note.path.to_string_lossy();
        self.conn.execute(
            r#"
            INSERT OR REPLACE INTO notes (path, title, blake3_hash, frontmatter_json, word_count, modified_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            "#,
            (
                path_str.as_ref(),
                &note.title,
                &note.blake3_hash,
                &note.frontmatter_json,
                note.word_count as i64,
                &note.modified_at,
            ),
        )?;

        self.conn.execute(
            r#"
            INSERT INTO notes_fts (path, title, body)
            VALUES (?1, ?2, ?3)
            "#,
            (path_str.as_ref(), &note.title, &note.body),
        )?;

        self.conn
            .execute("DELETE FROM tags WHERE note_path = ?1", [path_str.as_ref()])?;

        let mut tag_stmt = self
            .conn
            .prepare("INSERT OR IGNORE INTO tags (tag, note_path) VALUES (?1, ?2)")?;
        for tag in &note.tags {
            tag_stmt.execute((tag, path_str.as_ref()))?;
        }

        Ok(())
    }

    pub fn search_fts(&self, query: &str) -> Result<Vec<FtsSearchResult>> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT path, title, snippet(notes_fts, 2, '<b>', '</b>', '...', 10)
            FROM notes_fts
            WHERE notes_fts MATCH ?1
            ORDER BY rank
            LIMIT 50
            "#,
        )?;

        let rows = stmt.query_map([query], |row| {
            Ok(FtsSearchResult {
                path: row.get(0)?,
                title: row.get(1)?,
                snippet: row.get(2)?,
            })
        })?;

        let mut results = Vec::new();
        for r in rows {
            results.push(r?);
        }
        Ok(results)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FtsSearchResult {
    pub path: String,
    pub title: String,
    pub snippet: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_in_memory_and_schema_initialization() {
        let db = ManduDb::open_in_memory().expect("Falha ao abrir SQLite in-memory");
        let conn = db.connection();

        conn.execute(
            "INSERT INTO notes (path, title, blake3_hash, word_count) VALUES (?1, ?2, ?3, ?4)",
            ("artigo.md", "Meu Artigo", "hash123", 42),
        )
        .expect("Falha ao inserir nota");

        conn.execute(
            "INSERT INTO notes_fts (path, title, body) VALUES (?1, ?2, ?3)",
            (
                "artigo.md",
                "Meu Artigo",
                "Conteúdo acadêmico sobre genética e hereditariedade",
            ),
        )
        .expect("Falha ao inserir no FTS5");

        let mut stmt = conn
            .prepare("SELECT path FROM notes_fts WHERE notes_fts MATCH 'genética'")
            .expect("Falha ao preparar busca FTS5");
        let mut rows = stmt.query([]).expect("Falha ao executar query FTS5");

        let first_row = rows
            .next()
            .expect("Erro ao ler linha")
            .expect("Linha não encontrada");
        let path: String = first_row.get(0).expect("Erro ao obter coluna");
        assert_eq!(path, "artigo.md");
    }
}
