# Especificação Técnica — Ticket #7: Runtime Interativo do Terminal, Busca Fuzzy e Salvamento

## 1. Contexto e Objetivos
Esta especificação define o contrato do runtime interativo para a interface de terminal `mandu-tui`, integrando o loop de eventos Crossterm/Ratatui, a paleta de busca rápida de notas (`Ctrl+O`) alimentada pelo motor fuzzy `nucleo`, o salvamento persistente (`Ctrl+S`) com hooks de linter determinístico e a sincronização contínua com a base de dados in-memory `mandu_core::db::Database`.

## 2. Requisitos Arquiteturais e de Usabilidade

### 2.1. Zero Teclas de Função
- A integridade ergonômica e a compatibilidade multiplataforma (incluindo teclados 60%, 65% e terminais móveis como Termux) proíbem expressamente qualquer tecla de função (`F1` a `F12`).
- Toda a operação é governada por combinações `Ctrl`:
  - `Ctrl+Space`: Alternar modos de visualização (`Source` ⇄ `Split` ⇄ `Preview`).
  - `Ctrl+O`: Abrir / fechar a paleta de busca rápida de notas.
  - `Ctrl+S`: Salvar nota ativa, acionar `Linter::apply_save_hooks` e sincronizar SQLite.
  - `Ctrl+Q`: Encerrar a aplicação.
  - `Esc`: Fechar modal de busca ativa.
  - `Enter`: Abrir nota selecionada no modal de busca.

### 2.2. Modal de Busca Fuzzy (`SearchModalState`)
- Ativado via `Ctrl+O`.
- Renderizado como uma camada flutuante centralizada sobre a tela principal via Ratatui `Clear` e `Block`.
- Contém:
  - Campo de entrada de texto com cursor visível.
  - Lista ranqueada em tempo real dos arquivos e títulos do cofre utilizando `nucleo::Matcher`.
  - Item selecionado com destaque invertido (`Modifier::REVERSED`).
  - Navegação entre itens via setas `Up`/`Down`.

### 2.3. Fluxo de Edição, Salvamento e Linter
- O buffer de texto é mantido via `tui_textarea::TextArea`.
- Alterações marcam a nota como modificada (`is_dirty = true`), exibindo o indicador `[●]` na barra de status.
- Ao salvar (`Ctrl+S`):
  1. `Linter::apply_save_hooks` processa o conteúdo, injetando/atualizando `modificado` e garantindo terminação POSIX.
  2. O arquivo é persistido em disco.
  3. A nota é atualizada no SQLite via `Database::upsert_note`.
  4. O buffer é atualizado com o texto higienizado e a flag `is_dirty` é limpa.
