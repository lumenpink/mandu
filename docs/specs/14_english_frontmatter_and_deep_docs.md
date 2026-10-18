# Especificação Técnica: Padronização Universal de Frontmatter em Inglês e Arquitetura de Documentos

## 1. Contexto e Motivação
O Mandu foi concebido como um ambiente de notas soberano, interoperável e aderente aos padrões globais do ecossistema Markdown acadêmico (Obsidian, Zola, Hugo, Typst e Zettelkasten).
O uso prévio de metadados em português (`titulo`, `modificado`, `numerar_titulos`, `tipo: nota`) gerava inconsistência com ferramentas internacionais e reduzia a portabilidade das notas.

Este ticket padroniza a gramática canônica de YAML Frontmatter **estritamente em língua inglesa**, preserva compatibilidade retroativa resiliente para notas preexistentes e documenta com profundidade a arquitetura de processamento e numeração de seções.

---

## 2. Gramática Canônica do YAML Frontmatter

Toda nota no Mandu pode opcionalmente iniciar com um bloco delimitado por `---` contendo pares chave-valor YAML. Todas as chaves canônicas são definidas em minúsculas e em inglês:

```yaml
---
title: "Título da Nota"
type: note
modified: 2026-10-18T19:00:00Z
tags:
  - pesquisa
  - epistemologia
number_headings: true
numbering: "1.1"
origin: "[[Nota Origem]]"
date: 2026-10-18
---
```

### 2.1. Dicionário de Chaves Canônicas

| Chave Canônica | Tipo | Descrição | Equivalentes Legados (Compatibilidade) |
| :--- | :--- | :--- | :--- |
| `title` | `String` | Título formal da nota acadêmica. | `titulo` |
| `type` | `String` | Categoria ontológica da nota (`note`, `daily`, `reference`). | `tipo` |
| `modified` | `String` | Timestamp ISO 8601 UTC do último salvamento. | `modificado` |
| `tags` | `Array<String>` / `String` | Marcadores semânticos de categorização. | — |
| `number_headings` | `Boolean` | Ativa a numeração automática hierárquica de seções. | `numerar_titulos` |
| `numbering` | `String` / `Boolean` | Padrão de numeração Typst (ex: `"1.1"`). | `numeracao` |
| `origin` | `String` | Wikilink para a nota pai da qual o trecho foi extraído. | `origem` |
| `date` | `String` | Data cronológica para notas diárias (`YYYY-MM-DD`). | `data` |

---

## 3. Comportamento dos Geradores Nativos e Linter

Todos os geradores e ganchos (*save hooks*) do `mandu-core` passam a produzir **exclusivamente chaves em inglês**:

1. **Linter Hook (`Linter::apply_save_hooks`):**
   - Ao salvar um arquivo, localiza a linha `modified:` ou a antiga `modificado:` e atualiza para `modified: <timestamp>`.
   - Se nenhuma chave de modificação existir, injeta `modified: <timestamp>` imediatamente antes de `---`.
   - Se não houver frontmatter, cria o cabeçalho canônico com `modified: <timestamp>`.

2. **Gerador de Nota Diária (`create_daily_note`):**
   ```yaml
   ---
   date: YYYY-MM-DD
   type: daily
   ---
   ```

3. **Gerador Zettelkasten Único (`create_unique_note`):**
   ```yaml
   ---
   id: 20261018190000
   title: "Título da Nota"
   type: note
   ---
   ```

4. **Compositor de Extração Atômica (`extract_to_new_note`):**
   ```yaml
   ---
   title: "Novo Conceito"
   origin: "[[Nota Origem]]"
   type: note
   ---
   ```

---

## 4. Arquitetura de Numeração Automática Não-Destrutiva

### 4.1. Filosofia Não-Destrutiva
A numeração de seções no Mandu opera sobre o princípio da **apresentação desacoplada**:
- **O arquivo-fonte permanece limpo:** As linhas iniciam com `# Seção`, `## Subseção`. O usuário nunca é forçado a renumerar manualmente o documento ao mover seções.
- **Renderização Dinâmica:** A função `apply_heading_numbering` do `mandu-core` avalia a árvore de títulos e sintetiza as numerações (`1.`, `1.1.`, `1.1.1.`) dinamicamente no momento da exibição (TUI Split/Leitura, GUI Preview) e na geração do PDF via Typst (`#set heading(numbering: "1.1")`).
- **Imunidade de Blocos de Código:** Seções dentro de cercaduras (```` ``` ````) são ignoradas para garantir que códigos (ex.: comentários em Python/Bash `# comment`) não recebam números.
