# Especificação Técnica — Ticket #8: Motor de Visualização Joaju / Typst Preview, Cache BLAKE3 e Exportação PDF

## 1. Contexto e Objetivos
Esta especificação estabelece a integração entre o `mandu-core` e o orquestrador tipográfico e compilador `joaju`, viabilizando o pipeline unificado de pré-visualização vetorial estruturada (painel direito no modo Split e 100% no modo Leitura) e a compilação do documento completo para PDF via atalho `Ctrl+D`.

## 2. Requisitos Arquiteturais e de Cache

### 2.1. Cache Vetorial SQLite in-memory (`joaju_block_cache`)
- Todo bloco declarativo (heredogramas `pedtools`/`kinship2`, fórmulas químicas `openchemlib`/`openbabel`, tabelas, etc.) tem seu conteúdo e identificador de motor submetidos a hash BLAKE3:
  `block_hash = blake3::hash(format!("{engine}:{code}").as_bytes()).to_hex()`
- Antes de invocar a renderização, o banco SQLite in-memory (`ManduDb`) é consultado na tabela `joaju_block_cache`.
- Em caso de acerto de cache (*cache hit*), o SVG armazenado é retornado em sub-milissegundo sem consumo de CPU.
- Em caso de falha (*cache miss*), o motor gera o SVG vetorial, armazena no SQLite com timestamp atual e retorna o resultado.

### 2.2. Decomposição de Documento para Preview (`PreviewSegment`)
- O Markdown é decomposto em segmentos sequenciais:
  - `PreviewSegment::Text(String)`: trechos normais em Markdown/HTML.
  - `PreviewSegment::SvgBlock { engine: String, svg: String }`: blocos visuais renderizados.
- Permite que as cascas `mandu-tui` (via blocos/sixel/texto) e `mandu-gui` (via renderizador GPU de SVG) consumam exatamente a mesma representação estruturada.

### 2.3. Compilação para PDF (`Ctrl+D`)
- A função `compile_markdown_to_pdf(markdown: &str, template: Option<&str>)` realiza:
  1. Extração de metadados do YAML Frontmatter (`titulo`, `autor`, `instituicao`, `ano`, `local`).
  2. Pré-processamento de blocos de código acadêmicos.
  3. Conversão para documento Typst e compilação para bytes PDF (`Vec<u8>`).
- O método `export_note_to_pdf` grava o PDF compilado no disco no mesmo diretório da nota ativa com extensão `.pdf` (ou no subdiretório `export/`).
