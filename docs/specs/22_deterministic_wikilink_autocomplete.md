# Spec 22: Autocompletar Determinístico de Wikilinks por Título e Stem

## 1. Contexto & Motivação
Em `crates/mandu-core/src/autocomplete.rs`, o algoritmo de busca aproximada (`FuzzySearcher`) para wikilinks (`complete_wikilinks`) concatenava o título com o caminho absoluto da nota no disco (`format!("{title} {path}")`).

Quando notas são processadas em diretórios temporários ou ambientes de CI cujos caminhos contêm caracteres aleatórios, o casamento aproximado pode casar letras esparsas presentes nos nomes de diretórios pai (como `/tmp/.tmpe...` casando com uma query como `metodo`), gerando falsos-positivos e falhas intermitentes de testes em esteiras determinísticas.

Além disso, do ponto de vista de usabilidade e privacidade, o autocompletar de links internos no Markdown (`[[...]]`) deve pesquisar exclusivamente nos atributos da nota conhecidos pelo usuário:
1. Título (`title`) da nota;
2. Nome base do arquivo (`stem`).

O caminho completo (`path`) deve ser mantido estritamente na descrição/metadados do item de autocompletar, sem poluir a string de entrada do casamento aproximado.

## 2. Requisitos Técnicos
1. Na função `complete_wikilinks`:
   - Construir a string de busca aproximada no formato `format!("{title} {stem}")`, onde `stem` é o `file_stem` de `path` (ou fallback para `title`).
   - Executar `FuzzySearcher::match_candidates` sobre as strings de busca compostas exclusivamente por título e stem.
   - Preservar `path` no campo `description` e na formatação do `label` (`format!("{title} ({path})")`) para não quebrar compatibilidade de exibição.
2. Em `crates/mandu-core/tests/autocomplete_and_peek_test.rs`:
   - Testar o comportamento garantindo que caminhos com segmentos contendo letras da busca não causem falsos-positivos em notas cujo título e stem não correspondem à consulta.

## 3. Critérios de Aceite
- `cargo test -p mandu-core --test autocomplete_and_peek_test` executa com 100% de sucesso determinístico em qualquer diretório de execução.
- `cargo fmt --check` e `cargo clippy --all-targets -- -D warnings` sem violações.
- `cccp check --strict` com conformidade total.
