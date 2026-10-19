# Especificação Técnica - Ticket #19: Atalho Ctrl+T para Alternância de Modos e Proteção de Buffer na Exportação

## 1. Contexto e Motivação

1. **Conflito de Atalho com IBus/Fcitx no Linux Desktop**:
   O atalho anterior para alternar os modos de visualização (`Ctrl+Espaço`) é universalmente reservado por subsistemas de método de entrada (IME) como IBus e Fcitx em distribuições Linux com ambientes gráficos (GNOME, KDE, XFCE). Isso interceptava a combinação de teclas antes que ela pudesse atingir o `mandu-tui` ou `mandu-gui`.
   Adicionalmente, o uso de teclas `Alt` foi rejeitado por causar colisões com navegadores e gerenciadores de janela, e `Ctrl+A` / `Ctrl+B` são estritamente reservados para multiplexadores de terminal (`byobu` e `tmux`).
   A solução ergonômica ideal é adotar **`Ctrl+T`** (*Toggle View Mode*), garantindo funcionamento imediato e uniforme tanto no terminal puro quanto na interface gráfica.

2. **Prevenção de Truncamento de Arquivo (Zero-Byte Bug na GUI)**:
   Em `crates/mandu-gui/src/lib.rs`, a função `export_current_note_pdf` realizava uma escrita desprotegida `std::fs::write(&path, &self.state.buffer)` antes de compilar o PDF via Joaju. Em situações em que a nota ainda não havia completado sua carga ou quando o buffer estava em estado limpo/vazio na UI, o arquivo existente em disco era sobrescrito com 0 bytes, destruindo notas como `indice.md` e `demonstracao-joaju.md`.
   A exportação deve ser protegida por validações estritas de buffer:
   - Se o buffer estiver em branco mas o arquivo existir em disco com conteúdo, a escrita do buffer é abortada.
   - Se o buffer estiver sujo (`is_dirty`), o fluxo deve invocar `save_current_note()`, garantindo que os hooks de linter POSIX e atualização de frontmatter sejam aplicados de forma atômica.

3. **Isolamento de Entrada na TUI no Modo Preview**:
   No `mandu-tui`, quando o usuário está no modo `ViewMode::Preview` (Leitura), o editor de texto não está visível. Digitação de teclas alfanuméricas acidentais não deve alterar o buffer da `textarea` oculta nem acionar flags de `is_dirty`.

---

## 2. Especificação dos Atalhos e Interfaces

### 2.1. TUI (`crates/mandu-tui/src/app.rs`)
- Remoção definitiva de `KeyCode::Char(' ')` no bloco de atalhos globais `CONTROL`.
- Adição de `KeyCode::Char('t')` no bloco `CONTROL`:
  ```rust
  KeyCode::Char('t') => {
      self.mode = self.mode.next();
      return true;
  }
  ```
- Atualização da indicação visual no rodapé (`status_text`):
  ```text
  [ 📁 Buscar/Abrir (Ctrl+O) ]  nota.md (X pal. | 🔗 Y backlinks)  [ 👁 Modo: Split (Ctrl+T) ]  [ 📄 Gerar PDF (Ctrl+D) ]
  ```

### 2.2. GUI (`crates/mandu-gui/src/lib.rs` e `crates/mandu-gui/src/views.rs`)
- Remoção de `i.consume_key(egui::Modifiers::COMMAND, egui::Key::Space)`.
- Adição de `i.consume_key(egui::Modifiers::COMMAND, egui::Key::T)` para alternar `self.state.mode`.
- Botão de modo na barra de status inferior estilizado com:
  ```rust
  if ui.button(format!("[ 👁 {} (Ctrl+T) ]", state.mode.label())).clicked() {
      state.mode = state.mode.next();
  }
  ```

---

## 3. Especificação da Proteção de Buffer e Salvamento Seguro

### 3.1. Isolamento de Input no Modo Preview (`mandu-tui`)
No método `handle_key` de `TuiApp`:
```rust
if self.mode == ViewMode::Preview {
    return false;
}
```
Apenas atalhos globais com Control (`Ctrl+Q`, `Ctrl+T`, `Ctrl+O`, `Ctrl+S`, `Ctrl+D`, `Ctrl+P`, `Ctrl+G`, `Ctrl+E`) são processados. Teclas comuns de edição são ignoradas e o cursor/buffer permanece inalterado.

### 3.2. Proteção de Escrita na Exportação (`mandu-gui`)
No método `export_current_note_pdf` de `ManduApp`:
1. Determina o caminho do arquivo ativo (`self.current_file` ou `self.state.current_file`).
2. Se o arquivo existir em disco:
   - Se `self.state.buffer.trim().is_empty()`, não sobrescreve o arquivo no disco; apenas compila o arquivo pré-existente.
3. Se `self.is_dirty` e o buffer não estiver vazio:
   - Invoca `self.save_current_note()` para que o linter formate e persista os dados com segurança.
4. Executa `PreviewEngine::export_note_to_pdf(&path, None)`.

---

## 4. Critérios de Aceite e Verificação
- [ ] No `mandu-tui`, <kbd>Ctrl</kbd>+<kbd>T</kbd> alterna ciclicamente os modos: `Split -> Preview -> Source -> Split`.
- [ ] No `mandu-tui`, <kbd>Ctrl</kbd>+<kbd>Espaço</kbd> não altera o modo de visualização.
- [ ] No `mandu-tui`, digitação no modo `ViewMode::Preview` não muta a `textarea` nem marca `is_dirty = true`.
- [ ] No `mandu-gui`, <kbd>Ctrl</kbd>+<kbd>T</kbd> avança o modo de visualização.
- [ ] No `mandu-gui`, `export_current_note_pdf` com buffer vazio não sobrescreve nem zera arquivos existentes em disco.
- [ ] A barra de status em ambas as interfaces reflete `(Ctrl+T)`.
