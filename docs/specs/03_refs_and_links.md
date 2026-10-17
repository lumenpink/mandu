# Contrato Técnico: Referências Bibliográficas, Grafo de Links e Fuzzy Matching (Ticket #3)

## 1. Visão Geral
Na filosofia *Lumen-Style*, as referências bibliográficas não residem em arquivos `.bib` estáticos, mas sim em notas Markdown vivas (`refs/@autor2026.md` ou notas com `tipo: referencia`). Adicionalmente, as notas se interconectam através de links wiki no formato `[[alvo]]` ou `[[alvo|apelido]]`, alimentando um grafo bidirecional (links de saída e backlinks) e um mecanismo de autocompletar instantâneo com `nucleo`.

## 2. Tipos de Domínio e Contratos

### 2.1 `InternalLink`
Representa uma conexão hipertextual encontrada no corpo Markdown:
- `target`: `String` (nome ou caminho da nota referenciada sem extensão `.md`)
- `alias`: `Option<String>` (texto alternativo de exibição `[[alvo|alias]]`)
- `line_number`: `usize` (linha do arquivo onde o link se encontra, base 1)
- `context_snippet`: `String` (linha de texto ou trecho ao redor do link para exibição no painel de backlinks)

### 2.2 `ReferenceMd`
Representa um artigo, livro ou documento fichado no cofre:
- `citekey`: `String` (chave identificadora única, ex: `silva2026`)
- `note_path`: `String` (caminho do arquivo `.md` que contém o fichamento)
- `doi`: `Option<String>`
- `title`: `String` (título da publicação)
- `authors_json`: `Option<String>` (lista de autores serializada em JSON)
- `year`: `Option<i32>`
- `csl_yaml`: `Option<String>` (metadados estruturados para o motor bibliográfico Joaju)

### 2.3 Operações no Banco de Dados (`ManduDb`)
- `index_note_links(&self, source_path: &str, links: &[InternalLink]) -> Result<()>`:
  - Remove links anteriores de `source_path`
  - Insere novos registros na tabela `links`
- `get_backlinks(&self, target_path: &str) -> Result<Vec<InternalLink>>`:
  - Retorna todas as conexões que apontam para `target_path`
- `get_outgoing_links(&self, source_path: &str) -> Result<Vec<InternalLink>>`:
  - Retorna todas as conexões que partem de `source_path`
- `index_reference(&self, reference: &ReferenceMd) -> Result<()>`:
  - Insere ou atualiza o registro na tabela `references_md`
- `find_reference(&self, citekey: &str) -> Result<Option<ReferenceMd>>`:
  - Busca direta por citekey

### 2.4 Motor Fuzzy (`FuzzySearcher`)
- Indexa termos de busca em memória usando a crate `nucleo`
- Fornece busca de autocomplete para digitação de `[[` (notas do cofre) e `@` (referências da tabela `references_md`) em sub-milissegundos.
