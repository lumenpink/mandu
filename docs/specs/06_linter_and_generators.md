# Especificação Técnica — Ticket #6: Hooks de Salvamento, Linter e Geradores de Notas

## 1. Contexto e Objetivos
Esta especificação estabelece os requisitos e o contrato da API para o linter determinístico executado durante o salvamento de notas no `mandu-core`, além dos geradores de notas diárias, notas Zettelkasten e do compositor de notas.

## 2. Linter Determinístico e Hooks de Salvamento (`linter.rs`)
O módulo `linter` é responsável por executar transformações determinísticas e indolores na nota no momento do salvamento (`Ctrl+S`), garantindo integridade documental sem alterar campos desconhecidos ou comentários do YAML Frontmatter:

### 2.1. Normalização de Quebras de Linha
- Converter todas as quebras CRLF (`\r\n`) para quebras padrão POSIX (`\n`).
- Garantir terminação estrita do arquivo com um único caractere `\n` final.

### 2.2. Atualização / Inserção do Campo `modificado`
- Recebe o conteúdo Markdown e uma string de timestamp (ex.: `2026-10-17T15:00:00-03:00` ou ISO 8601).
- Se houver bloco YAML Frontmatter delimitado por `---` no início do documento:
  - Se a chave `modificado:` já existir, seu valor é atualizado mantendo o formato `modificado: <timestamp>`.
  - Se a chave `modificado:` não existir, ela é adicionada imediatamente antes da linha de fechamento `---`.
  - Todos os outros metadados (tags, aliases, campos arbitrários, comentários) permanecem estritamente preservados.
- Se o documento não possuir YAML Frontmatter:
  - Um bloco de frontmatter inicial é gerado contendo o campo `modificado: <timestamp>`.

### 2.3. Sincronização do Título H1
- Se o documento possuir um cabeçalho Markdown de nível 1 (`# Título da Nota`), o campo `titulo: "Título da Nota"` pode ser inferido ou validado se ausente.

## 3. Geradores de Notas e Compositor (`vault.rs`)

### 3.1. Gerador de Nota Diária (`create_daily_note`)
- Cria ou abre a nota no caminho `<vault_root>/diario/<YYYY-MM-DD>.md`.
- Garante a criação do subdiretório `diario/` se inexistente.
- Insere frontmatter padrão com `tipo: diario`, `data: YYYY-MM-DD`, tags e esqueleto de seções (ex.: tarefas, notas do dia).
- Se a nota já existir, não sobrescreve o conteúdo existente, apenas retorna o caminho existente.

### 3.2. Gerador de Nota Zettelkasten / Única (`create_unique_note`)
- Cria nota no formato `<vault_root>/<ID>.md` (onde ID segue o carimbo temporal `YYYYMMDDHHMMSS`).
- Preenche frontmatter com `id: <ID>`, `titulo: <title>` e `tipo: nota`.

### 3.3. Compositor de Notas (`extract_to_new_note`)
- Extrai um intervalo de caracteres (`std::ops::Range<usize>`) do buffer da nota de origem.
- Na nota de origem, substitui o trecho extraído por um wikilink `[[<new_note_title>]]`.
- Cria a nova nota no caminho `<vault_root>/<new_note_name>.md` contendo:
  - Frontmatter inicial com referência à nota de origem (`origem: "[[<origem_title>]]"`).
  - O conteúdo extraído.
- Retorna o conteúdo atualizado da nota original e o conteúdo da nova nota.
