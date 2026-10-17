# Contrato Técnico: Vault Scanner Paralelo e Parser de Notas (Ticket #2)

## 1. Visão Geral
O subsistema de Vault e Parser de `mandu-core` é responsável por escanear o cofre de notas em tempo inferior a 30 ms (para ~2.700 arquivos `.md`), extrair o cabeçalho YAML Frontmatter e o corpo do texto, computar o hash BLAKE3 para controle de modificações e cache incremental, e popular o SQLite in-memory com tabelas relacionais e índice FTS5.

## 2. Tipos de Domínio e Contratos de Dados

### 2.1 `Note`
Estrutura que representa uma nota processada no cofre:
- `path`: `PathBuf` (caminho relativo ou absoluto normalizado)
- `title`: `String` (extraído do Frontmatter `title`/`titulo` ou do nome do arquivo sem extensão `.md`)
- `blake3_hash`: `String` (hash BLAKE3 em formato hexadecimal do conteúdo completo do arquivo)
- `frontmatter_json`: `Option<String>` (metadados do YAML serializados em JSON)
- `body`: `String` (conteúdo Markdown sem o delimitador do YAML Frontmatter)
- `word_count`: `usize` (contagem de palavras do corpo do texto)
- `modified_at`: `Option<String>` (timestamp ISO 8601 extraído do frontmatter ou do mtime)
- `tags`: `Vec<String>` (tags extraídas do frontmatter)

### 2.2 `VaultScanner` e `Vault`
- `Vault::new(path: impl Into<PathBuf>) -> Self`
- `Vault::scan(&self) -> Result<Vec<Note>, VaultError>`: Varredura paralela usando `walkdir` e `rayon::iter::ParallelIterator` para processar múltiplos arquivos simultaneamente.
- `Vault::parse_note_content(path: &Path, content: &str) -> Note`: Função determinística de parsing sem I/O.

### 2.3 `ManduDb` (Indexação em Memória)
- `ManduDb::index_note(&self, note: &Note) -> Result<()>`:
  - Insere ou atualiza `notes`
  - Atualiza o índice FTS5 `notes_fts`
  - Insere tags correspondentes em `tags`
- `ManduDb::search_fts(&self, query: &str) -> Result<Vec<FtsSearchResult>>`:
  - Executa consulta FTS5 com snippet e ordenação de relevância
