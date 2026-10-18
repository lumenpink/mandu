# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Inicialização do repositório em conformidade com o padrão GOST (`profiles/lumen.md`).
- Especificação arquitetural canônica em `SPEC.md` detalhando os princípios de escrita honesta, o padrão *HedgeDoc*, a integração com o *Joaju* e a migração de plugins do Obsidian.
- Workspace Cargo multi-crate com `mandu-core`, `mandu-tui` e `mandu-gui`.
- Núcleo `mandu-core` com schema SQLite in-memory (FTS5), hashing BLAKE3, linter POSIX, extrator de referências e stub do Joaju.
- Cascas iniciais `mandu-tui` (Ratatui/Crossterm) e `mandu-gui` (egui/eframe).
- Testes unitários e de integração de contrato com zero warnings e validação rigorosa de linters.
- Deploy de documentação configurado em `wrangler.jsonc` para `mandu.2lp.in` e registro no `catalog.json`.
- Scanner de cofre paralelo com Rayon e WalkDir capaz de processar milhares de notas em milissegundos ignorando diretórios ocultos.
- Parser determinístico de notas com extração de YAML Frontmatter (título, tags, datas), contagem de palavras e cálculo de hash BLAKE3.
- Indexação em lote no SQLite in-memory com suporte a Full-Text Search (FTS5) para busca instantânea de conteúdo acadêmico.
- Suíte de testes abrangente cobrindo casos de borda: quebras de linha CRLF, YAML malformatado, notas vazias e proteção contra travessia em pastas ocultas.
- Extração de referências bibliográficas (.md) em refs/@citekey.md com metadados estruturados (DOI, autores, ano, CSL/YAML).
- Grafo de conexões com suporte a links wiki [[nota]] e [[nota|apelido]], com rastreio de linha e snippet de contexto.
- Consultas relacionais rápidas para backlinks (links inversos) e outgoing links no SQLite in-memory.
- Motor de autocompletar fuzzy ultrarrápido em memória utilizando a biblioteca nucleo.
- Interface TUI HedgeDoc com os 3 modos de visualização (Escrita, Split, Leitura) implementada em mandu-tui.
- Diretriz estrita de zero teclas de função (F1-F12), com alternância de modos via Ctrl+Espaço ou toque na barra de status.
- Barra de status inferior de 1 linha tátil exibindo botões de ação, contagem de palavras e contador de backlinks em tempo real.
- Mapeamento ergonômico de atalhos Ctrl unificados para pesquisa (Ctrl+O), salvamento (Ctrl+S), exportação (Ctrl+D) e saída (Ctrl+Q).
- Interface gráfica GPU nativa (egui/eframe) com visualização HedgeDoc (Escrita, Split, Preview) implementada em mandu-gui.
- Renderizador de preview vetorial com suporte a blocos SVG e renderização Joaju/Typst.
- Barra de status inferior de 1 linha na GUI com botões clicáveis e paridade estrita com o layout de terminal.
- Paridade rigorosa de atalhos ergonômicos Ctrl e conformidade com zero teclas de função para compatibilidade e memória muscular unificada.
- Hooks de salvamento nativos no mandu-core com normalização de CRLF para POSIX (\n) e terminação estrita.
- Atualização e injeção determinística de timestamp modificado no YAML Frontmatter sem alterar dados ou comentários existentes.
- Geradores nativos de notas diárias (diario/YYYY-MM-DD.md) e notas únicas Zettelkasten com suporte a templates e idempotência.
- Compositor nativo de notas com extração atômica de seleções de texto para novas notas e substituição por links wiki.
- Loop interativo completo do terminal com crossterm em raw mode e restauração segura de tela.
- Paleta modal flutuante de busca rápida (Ctrl+O) com casamento fuzzy de notas em tempo real via nucleo.
- Integração de salvamento no buffer (Ctrl+S) persistindo em disco, acionando hooks de linter determinístico e indexando no SQLite.
- Indicador de edições pendentes ([●]) na barra de status inferior de 1 linha e consulta de backlinks em tempo real.
- Motor de visualização Joaju e Typst integrado no `mandu-core` com suporte a blocos de código com cercadura para diagramas, tabelas e árvores genealógicas (pedigree).
- Sistema de cache SQLite in-memory de blocos vetoriais SVG indexados deterministicamente por hash BLAKE3 para renderização instantânea.
- Pipeline de compilação e exportação direta de notas para PDF acadêmico (`Ctrl+D`) em conformidade com os modelos ABNT e UFSC.
- Ativação do atalho `Ctrl+D` no `mandu-tui` com feedback de exportação e status de arquivo gerado em tempo real.

