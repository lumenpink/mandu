---
titulo: Especificação Arquitetural e Técnica do Ecossistema Mandu
subtitulo: Ambiente de Escrita Acadêmica e Gestão de Conhecimento em 100% Rust Integrado ao Motor Joaju
autor: Lumen Pink
ano: 2026
perfil: abnt
status: v0.1.0-draft
---

# 1. Visão Geral e Princípios Arquiteturais

**Mandu** (do Guarani *mandu'a*: memória, lembrança, recordação) é um ambiente de escrita acadêmica, gestão de conhecimento e publicação científica construído em **100% Rust**. O projeto nasce para substituir editores baseados em Electron (como o Obsidian), eliminando o consumo excessivo de recursos, a complexidade de configuração e a divergência entre a visualização de tela e o documento exportado.

## 1.1. Pilares Arquiteturais

1. **Padrão HedgeDoc (Separação Estrita entre Escrita e Preview):** Fim do *Live Preview* instável onde o cursor pula e a sintaxe Markdown desaparece durante a digitação. A edição ocorre em código-fonte monoespaçado previsível (painel esquerdo) e a visualização ocorre em um painel renderizado sincronizado (painel direito), alternáveis entre si.
2. **Pipeline Único de Renderização via Joaju:** Não existem dois motores de renderização (um HTML/CSS para tela e outro para PDF). Toda visualização do documento passa diretamente pelo motor em Rust do **Joaju** (Typst + módulos científicos nativos), garantindo que o preview na tela seja exatamente idêntico ao PDF final nos perfis `trabalho-ufsc`, `abnt` (NBR 14724) e `vancouver`.
3. **Core Desacoplado (Multi-View):** Toda a regra de negócio, indexação em banco de dados, leitura de arquivos e integração com o Joaju vivem isolados na biblioteca `mandu-core`. As interfaces **`mandu-tui`** (`ratatui`) e **`mandu-gui`** (`egui`) atuam exclusivamente como camadas de apresentação.
4. **Zero Interfaces de Configuração:** O aplicativo não possui menus ou janelas de preferências. Configurações de infraestrutura do sistema ficam em `~/.config/mandu/config.toml`. Configurações tipográficas e de compilação vivem exclusivamente no YAML Frontmatter de cada nota.
5. **Sincronização Desacoplada:** O Mandu opera diretamente sobre arquivos `.md` no sistema de arquivos local. A sincronização entre dispositivos (Debian Linux, servidor `baquara` e tablet Android) é delegada integralmente ao **Syncthing** e ao **Git**, sendo monitorada em tempo real via *file watcher*.

---

# 2. Estrutura do Cargo Workspace

O repositório utiliza um Cargo Workspace unificado para compartilhar tipos e compilar tanto para Desktop (Linux X11/Wayland) quanto para Mobile (Android ARM64 via Termux para TUI e APK nativo para GUI):

~~~text
mandu/
├── Cargo.toml                  # Manifesto do Workspace
├── SPEC.md                     # Especificação Arquitetural
└── crates/
    ├── mandu-core/             # Cérebro do cofre (Zero dependências de UI)
    │   ├── Cargo.toml
    │   └── src/
    │       ├── lib.rs          # API pública para as interfaces (TUI/GUI)
    │       ├── config.rs       # Parser de ~/.config/mandu/config.toml
    │       ├── vault.rs        # Operações de I/O, File Watcher (notify) e Compositor
    │       ├── db.rs           # Indexador SQLite In-Memory (FTS5) + Cache BLAKE3
    │       ├── linter.rs       # Hooks de salvamento (YAML, datas, formatação)
    │       ├── refs.rs         # Extrator e resolvedor de referências em notas .md
    │       └── preview.rs      # Ponte direta com a crate do Joaju (SVG / PDF)
    │
    ├── mandu-tui/              # Interface de Terminal (Debian st/Kitty + Android Termux)
    │   ├── Cargo.toml
    │   └── src/
    │       ├── main.rs         # Event loop (crossterm + ratatui)
    │       ├── editor.rs       # Wrapper tui-textarea + Escape Hatch para $EDITOR (nvim/micro)
    │       └── ui.rs           # Layout 3 modos + renderização Sixel/Kitty/Unicode
    │
    └── mandu-gui/              # Interface Gráfica GPU (Debian Desktop + Android APK)
        ├── Cargo.toml
        └── src/
            ├── lib.rs          # Lógica compartilhada egui/eframe + android_main()
            ├── main.rs         # Entrypoint binário para Debian Linux
            └── views.rs        # Painéis Editor (TextEdit) + Preview Vetorial (Joaju SVG)
~~~

---

# 3. Especificação do `mandu-core`

## 3.1. Indexação em Memória (SQLite + FTS5)

Ao abrir o cofre (~2.700+ notas e ~1.200+ pastas), o `mandu-core` executa uma varredura paralela (`walkdir` + `rayon`), extraindo o YAML Frontmatter, links internos e blocos de código em menos de 30 ms e populando uma instância **SQLite in-memory** (persistida em disco apenas para cache de artefatos pesados):

~~~sql
-- Tabela principal de notas do cofre
CREATE TABLE notes (
    path TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    blake3_hash TEXT NOT NULL,
    frontmatter_json TEXT,
    word_count INTEGER DEFAULT 0,
    modified_at TEXT
);

-- Índice Full-Text Search (FTS5) para busca instantânea no conteúdo
CREATE VIRTUAL TABLE notes_fts USING fts5(path, title, body);

-- Grafo de conexões (alimenta Links de Saída, Backlinks e Peek)
CREATE TABLE links (
    source_path TEXT NOT NULL,
    target_path TEXT NOT NULL,
    line_number INTEGER,
    context_snippet TEXT,
    PRIMARY KEY (source_path, target_path, line_number)
);
CREATE INDEX idx_backlinks ON links(target_path);

-- Etiquetas (Tags)
CREATE TABLE tags (
    tag TEXT NOT NULL,
    note_path TEXT NOT NULL,
    PRIMARY KEY (tag, note_path)
);

-- Referências Bibliográficas (Notas .md de referência)
CREATE TABLE references_md (
    citekey TEXT PRIMARY KEY,   -- ex: "silva2026" (nome do arquivo ou chave no YAML)
    note_path TEXT NOT NULL,
    doi TEXT,
    title TEXT,
    authors_json TEXT,
    year INTEGER,
    csl_yaml TEXT               -- Metadados prontos para injeção no Hayagriva/Typst do Joaju
);

-- Cache de renderização de blocos do Joaju por Hash BLAKE3
CREATE TABLE joaju_block_cache (
    block_blake3 TEXT PRIMARY KEY,
    engine TEXT NOT NULL,       -- ex: "heredograma", "table", "mermaid", "d2", "quimica"
    rendered_svg BLOB NOT NULL,
    created_at INTEGER
);
~~~

## 3.2. Gestão Bibliográfica Integrada (Notas `.md` como Referências)

Em vez de depender de um arquivo `.bib` estático ou de plugins acoplados ao Zotero Desktop:
1. **Um Arquivo `.md` por Referência:** Um script extrator (ou criação manual) gera uma nota `.md` para cada artigo/livro (ex: `refs/@autor2026.md` ou identificada por `tipo: referencia` no YAML).
2. **Anotações e Fichamento Integrados:** O YAML Frontmatter guarda os metadados estritos (título, autores, DOI, periódico, ano), enquanto o corpo da nota `.md` fica livre para comentários pessoais, citações diretas e conexões com outras notas.
3. **Autocompletar Nativo (`@`):** Ao digitar `@` no editor (TUI ou GUI), o `mandu-core` consulta a tabela `references_md` via *fuzzy matching* (`nucleo`) e exibe o seletor de artigos.
4. **Injeção Dinâmica no Joaju:** Na hora de renderizar o preview ou exportar o PDF, o `mandu-core` coleta apenas as *citekeys* presentes no texto atual, extrai a coluna `csl_yaml` do SQLite e entrega em memória para o motor bibliográfico do **Joaju** formatar segundo os perfis `trabalho-ufsc`, `abnt` ou `vancouver`.

## 3.3. Cache Incremental por Bloco (BLAKE3)

Para garantir 60/120 FPS no preview sem recompilar diagramas inalterados a cada tecla digitada:
* O parser separa os blocos de código especiais do Joaju (`heredograma`, `pedtools`, `kinship2`, `table`, `mermaid`, `d2`, `plantuml`, `graphviz`, `quimica`, `gist`).
* Calcula o hash **BLAKE3** do conteúdo do bloco + diretivas (`%% layout.width=...`).
* Se o hash existir na tabela `joaju_block_cache`, o SVG já renderizado é retornado em **0 ms**. Apenas os blocos modificados acionam os motores do Joaju.

## 3.4. Hooks de Salvamento, Gramática Canônica e Operações Nativas de Cofre

Executados automaticamente na chamada `vault::save_note()`, eliminando plugins externos:
* **Gramática Canônica em Inglês:** O YAML Frontmatter utiliza universalmente chaves em inglês: `title`, `type` (`note`, `daily`, `reference`), `modified`, `tags`, `number_headings`, `numbering`, `origin` e `date`. O parser preserva tolerância retroativa a arquivos legados, mas todos os geradores e hooks produzem exclusivamente chaves em inglês.
* **Auto-Update Modified Date:** Atualiza ou insere deterministicamente a chave canônica `modified: <timestamp>` no YAML Frontmatter antes de persistir em disco.
* **Linter Nativo POSIX:** Normaliza quebras de linha CRLF para `\n` e assegura terminação estrita com quebra final única.
* **Numeração Automática Não-Destrutiva de Títulos e Gramática de Padrões (`number_type` / `numbering`):**
  Quando configurado com `number_headings: true` ou informando um padrão em `number_type: "<padrão>"` (a presença de `number_type` ou `numbering` presume e ativa automaticamente a numeração), o Mandu preserva o texto-fonte limpo em Markdown padrão (`# Título`) e calcula a hierarquia dinâmica na camada de apresentação (Preview TUI, GUI e exportação Typst PDF):
  * **Tokens Suportados por Nível (H1 a H6):**
    * `1`: Numerais arábicos (`1, 2, 3...`).
    * `a`: Alfabeto latino minúsculo (`a, b, c...`).
    * `A`: Alfabeto latino maiúsculo (`A, B, C...`).
    * `i`: Numerais romanos minúsculos (`i, ii, iii, iv, v...`).
    * `I`: Numerais romanos maiúsculos (`I, II, III, IV, V...`).
    * `_`: **Supressão de nível** (ex: `"_.1.a"` omite a numeração em H1, aplica números em H2 e letras em H3, mantendo contadores isolados).
  * **Regra de Herança de Padrão:** Níveis hierárquicos não explicitados no padrão herdam o token do último nível declarado (ex: `"1.1.a"` projeta `'a'` para H4, H5 e H6).
  * **Cabeçalhos Não-Numerados Avulsos (`#_`, `# _` e `{-}`):** Seções documentais livres (ex: `#_ Introdução`, `##_ Contexto`, `# Conclusão {-}`) têm o marcador removido na apresentação visual (`▌ Introdução`), não recebem número e **não avançam nem corrompem os contadores** das seções numeradas subsequentes.
  * **Repasse Typst PDF:** O padrão é compilado deterministicamente como `#set heading(numbering: "<pattern>")`.
* **Listas Ordenadas Auto-Incrementais:** No painel de visualização (`mandu-tui` e `mandu-gui`), linhas iniciadas por `<digito>+. ` (mesmo que todas digitem `1. ` no código-fonte) são renderizadas com numeração sequencial real (` 1. `, ` 2. `, ` 3. `...), recuo estruturado e destaque tipográfico em amarelo/ciano negrito, reiniciando o contador para 1 após qualquer interrupção de bloco (linhas vazias, cabeçalhos, citações, blocos de código).
* **Compositor de Notas (Extract / Split):** Comando nativo que move o bloco selecionado para um novo arquivo `.md`, gerando cabeçalho com `title`, `origin` e `type: note`, e substituindo o trecho original pelo `[[link]]` correspondente.
* **Gerador de Notas (Zettelkasten / Diárias / Modelos):** Criação instantânea com `id`, `date: YYYY-MM-DD`, `type: daily`/`type: note` e esqueleto estruturado.

---

# 4. Especificação das Interfaces (`mandu-tui` & `mandu-gui`)

Ambas as interfaces obedecem à diretriz de **mínima poluição visual**, sem barras de ferramentas com dezenas de ícones ou menus cascata.

## 4.1. Os 3 Modos de Visualização (HedgeDoc Pattern)

Alternados rapidamente via teclado (`Ctrl+Espaço`) ou toque na barra de status (com estrita proibição de teclas de função F1–F12 para paridade total com dispositivos compactos e tablets):
1. **`[Modo 1: Escrita (Source)]`** — 100% da tela dedicada ao editor de texto monoespaçado.
2. **`[Modo 2: Split (HedgeDoc)]`** — Tela dividida ao meio: Editor Markdown à esquerda e Preview renderizado pelo Joaju à direita, com *scroll sync* proporcional baseado nos offsets de linha da AST.
3. **`[Modo 3: Leitura (Preview)]`** — 100% da tela dedicada ao documento diagramado pelo Joaju, com links navegáveis e inspeção de diagramas/heredogramas.

## 4.2. Barra de Status de 1 Linha (Rodapé Único)

Para viabilizar o uso no tablet Android mesmo quando desconectado do teclado físico (Aula F75), o rodapé de 1 linha funciona simultaneamente como indicador de estado e barra de ações clicáveis por toque:

~~~text
[ 📁 Buscar/Abrir ]  nota_atual.md (1.240 pal. | 🔗 5 backlinks)  [ 👁 Modo: Split ]  [ 📄 Gerar PDF ]
~~~

## 4.3. Particularidades de Cada View

| Recurso | `mandu-tui` (`ratatui` + `crossterm`) | `mandu-gui` (`egui` / `eframe`) |
| :--- | :--- | :--- |
| **Plataformas Alvo** | Debian Linux (`st`, `kitty`) e Android (`Termux`). | Debian Linux (Binário nativo) e Android (APK nativo). |
| **Motor de Edição** | `tui-textarea` com numeração de linhas e destaque de sintaxe Markdown/YAML. | `egui::TextEdit` monoespaçado com `Layouter` leve para coloração de sintaxe. |
| **Escape Hatch (`Ctrl+E`)** | Suspende a TUI, abre o arquivo na linha exata dentro do `nvim` ou `micro` e retoma ao sair. | Opcional (pode invocar terminal externo no Desktop). |
| **Renderização do Preview** | Texto estruturado em terminal + imagens/SVGs do Joaju via `ratatui-image` (Kitty/Sixel no Desktop, blocos Unicode ou `termux-open` no Android). | Renderização vetorial direta na GPU (`wgpu`/`glow`) das páginas e SVGs gerados pelo motor Joaju/Typst. |
| **Peek (`Espiar Página`)** | Popup flutuante em texto (`Clear` widget) ao acionar atalho sobre um `[[link]]` ou `@citekey`. | Card flutuante tipográfico ao passar o mouse ou tocar sobre um link/citação. |

---

# 5. Matriz de Migração de Plugins (De Obsidian para Mandu)

Consolidação da auditoria realizada sobre os plugins anteriormente ativos no Obsidian:

| Funcionalidade / Plugin Original | Status no Mandu | Implementação na Nova Arquitetura |
| :--- | :---: | :--- |
| **Joaju Live Preview** | **Core (MVP)** | Integrado nativamente via crate do Joaju em Rust (pipeline único de preview e PDF). |
| **Citations / Zotero Integration** | **Core (MVP)** | Substituídos por notas `.md` de referência + indexador SQLite (`references_md`) + CSL nativo no Joaju. |
| **Link Embed / Rich Links / Auto Link Title** | **Core (MVP)** | Absorvidos pelos cards tipográficos de links do próprio Joaju. |
| **Navegação Rápida / Procurar** | **Core (MVP)** | Consultas em < 1 ms via SQLite FTS5 + *fuzzy matcher* `nucleo` (`Ctrl+O` / `Ctrl+F`). |
| **Links Inversos / Links de Saída** | **Core (MVP)** | Consultas indexadas na tabela `links` do SQLite em memória. |
| **Espiar Página (Page Preview)** | **Core (MVP)** | Modal flutuante (*Peek*) acionado por atalho ou hover sobre `[[links]]` e `@citekeys`. |
| **Compositor de Notas** | **Core (MVP)** | Função nativa de extração e fusão de notas em `vault.rs` (`Ctrl+Shift+N`). |
| **Linter / Update Modified Date / Title Index** | **Core (MVP)** | Executados em microssegundos no *hook* de salvamento (`linter.rs`). |
| **Notas Diárias / Notas Únicas / Modelos** | **Core (MVP)** | Geradores nativos de arquivo com carimbo de data/hora e cópia de template. |
| **Sumário / Painel de Etiquetas / Contador** | **Core (MVP)** | Extraídos em tempo real da AST (`pulldown-cmark`) e da tabela `tags`. |
| **Tela (Canvas) / Slides / Gráfico Global** | **Eliminado** | Removidos. Diagramas são escritos declarativamente nos motores do Joaju (`d2`, `mermaid`, `graphviz`). |
| **Ver Propriedades / Metadata Menu / Multi Prop.**| **Eliminado** | Removidos. O YAML Frontmatter é editado diretamente como texto puro no editor. |
| **Recuperação de Arquivos / Sync / Publish** | **Eliminado** | Removidos. Versionamento e sincronização delegados ao Git + Syncthing. |
| **Gravador de Áudio** | **Eliminado** | Removido do escopo. |

---

# 6. Roadmap Pós-MVP (Fases de Expansão)

Os quatro módulos abaixo ficam fora do escopo inicial (`v0.1.0`) para garantir simplicidade e entrega rápida do núcleo de escrita, entrando sequencialmente nas versões seguintes:

## Fase 2 (`v0.2.0`) — Templates Programáveis (Substituto do *Templater*)
* **Escopo:** Embutir a crate **`minijinja`** no `mandu-core`.
* **Funcionamento:** Permitir o uso de variáveis dinâmicas (`{{ data }}`, `{{ titulo }}`), estruturas condicionais (`{% if perfil == 'trabalho-ufsc' %}`) e filtros dentro dos arquivos de modelo em Rust puro, sem depender de runtime JavaScript.

## Fase 3 (`v0.3.0`) — Queries Dinâmicas no Cofre (Substituto do *Dataview*)
* **Escopo:** Criar o motor de blocos ` ```mandu-sql ` (ou ` ```query `) integrado ao SQLite em memória.
* **Funcionamento:** Permitir que uma nota execute consultas SQL reais sobre os metadados, tags e links das outras 2.700+ notas do cofre e repasse a tabela resultante diretamente para o motor **`table` do Joaju**, renderizando listas dinâmicas de artigos, tarefas ou casos clínicos no padrão ABNT/APA.

## Fase 4 (`v0.4.0`) — Verificação Gramatical e Estilo (Substituto do *LanguageTool*)
* **Escopo:** Cliente assíncrono em background no `mandu-core` para a API do LanguageTool (local no servidor `baquara` ou pública).
* **Funcionamento:** Envio assíncrono apenas dos parágrafos modificados (controlados por hash BLAKE3) para verificação ortográfica e gramatical em português e inglês acadêmico, exibindo diagnósticos discretos no editor TUI/GUI.

## Fase 5 (`v0.5.0`) — Assistente Contextual (Substituto do *Gemini Scribe*)
* **Escopo:** Módulo opcional e enxuto de comunicação via API REST com LLMs.
* **Funcionamento:** Acionado exclusivamente sob demanda via Paleta de Comandos (`Ctrl+P`), enviando a nota atual e as notas diretamente conectadas a ela no SQLite como contexto para síntese, revisão ou estruturação, inserindo a resposta no buffer sem criar janelas de chat permanentes na interface.
