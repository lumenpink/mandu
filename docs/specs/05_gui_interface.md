# Contrato Técnico: Interface Gráfica GPU Nativa (Ticket #5)

## 1. Princípios de Interface e Diretriz Zero Teclas de Função
A interface gráfica (`mandu-gui`) é construída em 100% Rust utilizando `egui` e `eframe` (acelerada por GPU via OpenGL/Vulkan/wgpu), sem qualquer dependência de WebKit ou Electron:
- **Zero Teclas de Função:** Nenhuma funcionalidade depende ou reage a teclas `F1`–`F12`.
- **Memória Muscular Unificada:** Todos os atalhos são rigorosamente idênticos aos da interface de terminal (`mandu-tui`).

## 2. Modos de Visualização (HedgeDoc Pattern)
1. **`GuiViewMode::Source`**: Editor monoespaçado ocupa 100% da área útil vertical.
2. **`GuiViewMode::Split`**: Divisão 50%/50% com `egui::TextEdit` multiline à esquerda e visualizador de leitura com blocos do Joaju à direita.
3. **`GuiViewMode::Preview`**: Visualizador de leitura estruturada ocupa 100% da área útil vertical.

## 3. Barra de Status Inferior de 1 Linha
Localizada na base da janela, com componentes clicáveis:
- Botão `[ 📁 Buscar/Abrir ]` (`Ctrl+O`)
- Rótulo de informações `nota_atual.md (1.240 pal. | 🔗 5 backlinks)`
- Botão seletor de modo `[ 👁 Modo: Split ]` (`Ctrl+Espaço`)
- Botão de exportação `[ 📄 Gerar PDF ]` (`Ctrl+D`)

## 4. Contratos de Tipos e Assinaturas
- `GuiViewMode::next(self) -> Self`
- `GuiViewMode::label(self) -> &'static str`
- `ManduApp::new(cc: &eframe::CreationContext) -> Self`
- `ManduApp::handle_input(&mut self, ctx: &egui::Context)`
- `ManduApp::status_text(&self) -> String`
