# Contrato Técnico: Interface de Terminal HedgeDoc (Ticket #4)

## 1. Princípios de Interface e Diretriz Zero Teclas de Função
A interface de terminal (`mandu-tui`) é construída com `ratatui`, `crossterm` e `tui-textarea`. Seguindo a diretriz estrita de usabilidade *Lumen-Style*:
- **Zero Teclas de Função:** É expressamente vedado o uso de atalhos baseados em `F1`–`F12`. Isso garante compatibilidade total com teclados compactos (60%/75% como Aula F75) e dispositivos Android/Termux.
- **Memória Muscular Unificada:** Todos os atalhos usam combinações de `Ctrl` + tecla alfanumérica/espaço, idênticos aos usados na futura GUI.

## 2. Modos de Visualização (HedgeDoc Pattern)
1. **`ViewMode::Source`**: O editor Markdown ocupa 100% da área útil vertical.
2. **`ViewMode::Split`**: Divisão horizontal 50%/50% — Editor de código monoespaçado à esquerda e painel de preview/leitura estruturada à direita.
3. **`ViewMode::Preview`**: O painel de leitura diagramada ocupa 100% da área útil vertical.

## 3. Barra de Status Inferior de 1 Linha
Layout do rodapé em linha única:
```text
[ 📁 Buscar/Abrir ]  nota_atual.md (1.240 pal. | 🔗 5 backlinks)  [ 👁 Modo: Split ]  [ 📄 Gerar PDF ]
```
- Seção Esquerda: Botão tátil `[ 📁 Buscar/Abrir ]` (`Ctrl+O`)
- Seção Central: Nome do arquivo ativo, contagem de palavras e contador de backlinks
- Seção Direita: Botão indicador/tátil de modo `[ 👁 Modo: ... ]` (`Ctrl+Espaço`) e ação `[ 📄 Gerar PDF ]` (`Ctrl+D`)

## 4. Contratos de Tipos e Assinaturas
- `ViewMode::next(self) -> Self`
- `ViewMode::label(self) -> &'static str`
- `TuiApp::new(vault_path: Option<PathBuf>) -> Self`
- `TuiApp::handle_key(&mut self, key: crossterm::event::KeyEvent) -> bool`
- `TuiApp::render(&self, frame: &mut ratatui::Frame)`
