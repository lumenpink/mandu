# Especificação Técnica — Ticket #9: Runtime Interativo GPU, Modal de Busca Fuzzy Ctrl+O, Preview Joaju e Salvamento Ctrl+S

## 1. Contexto e Objetivos
Esta especificação detalha a implementação do runtime interativo completo para a casca gráfica `mandu-gui` (construída sobre `egui` e `eframe`), conectando a interface ao `mandu-core` para sincronização com o banco de dados SQLite in-memory, busca fuzzy com `nucleo`, hooks determinísticos de salvamento e renderização de pré-visualização Joaju/Markdown.

## 2. Requisitos Arquiteturais e Funcionais

### 2.1. Conexão ao Estado do Cofre (`mandu-core`)
- `ManduApp` mantém ou referencia instâncias de:
  - `Database`: SQLite in-memory com tabelas de notas, FTS5, links e cache vetorial Joaju.
  - `PreviewEngine`: renderizador de blocos visuais e compilador PDF.
  - Lista de notas indexadas no cofre para busca rápida.
  - Indicador de estado de edição (`is_dirty: bool` ou contagem de modificações pendentes `[●]`).

### 2.2. Paleta Modal Flutuante de Busca Rápida (`Ctrl+O`)
- Abertura através do atalho global `Ctrl+O` ou clique no botão `[ 📁 Buscar/Abrir ]` da barra de status.
- Renderizada como janela modal sobreposta no centro do viewport (`egui::Window`).
- Campo de busca com foco automático ao abrir.
- Execução síncrona/reativa do algoritmo de busca fuzzy com a biblioteca `nucleo` sobre títulos e caminhos das notas no cofre.
- Lista rolável de notas filtradas com índice de seleção navegável via teclas `Up` e `Down`.
- Abertura da nota selecionada com a tecla `Enter` ou clique duplo, carregando o conteúdo no buffer do editor e recalculando contagem de palavras e backlinks.
- Fechamento imediato da modal com a tecla `Esc` ou clique externo.

### 2.3. Salvamento Reativo com Linter Determinístico (`Ctrl+S`)
- Acionamento via atalho global `Ctrl+S`.
- Execução dos hooks de salvamento (`mandu_core::linter::apply_save_hooks`):
  1. Normalização de CRLF para quebras POSIX (`\n`) estritas.
  2. Injeção ou atualização do campo `modificado` no YAML Frontmatter com timestamp ISO 8601 UTC.
  3. Preservação de dados e comentários existentes.
- Escrita atômica do arquivo no sistema de arquivos.
- Reindexação da nota atualizada no banco de dados SQLite in-memory.
- Atualização da mensagem de status temporária ("Nota salva com sucesso") e limpeza do indicador de modificação não salva.

### 2.4. Renderização do Painel de Visualização Joaju e Exportação PDF (`Ctrl+D`)
- Nos modos `Split` e `Preview`, o painel de visualização processa o Markdown em segmentos sequenciais via `mandu_core::preview::parse_preview_segments`:
  - Segmentos de texto Markdown exibidos com formatação estruturada de leitura.
  - Segmentos de diagrama/código Joaju (heredogramas, tabelas, fórmulas) exibidos como blocos renderizados ou representação vetorial SVG em caixa dedicada.
- Atalho `Ctrl+D` ou clique no botão `[ 📄 Gerar PDF ]` invoca `PreviewEngine::compile_note_to_pdf`, exportando a nota ativa para PDF acadêmico com notificação na barra de status.

### 2.5. Conformidade Estrita com Zero Teclas de Função
- Nenhuma tecla de função (`F1`–`F12`) pode ser interceptada ou atribuída a ações na GUI.
- Paridade estrita de atalhos ergonômicos `Ctrl`:
  - `Ctrl+Space`: alternância cíclica de modos HedgeDoc (`Source` -> `Split` -> `Preview` -> `Source`).
  - `Ctrl+O`: busca rápida e abertura de notas.
  - `Ctrl+S`: salvamento e aplicação de linter determinístico.
  - `Ctrl+D`: exportação para PDF acadêmico via Joaju.
  - `Ctrl+Q`: encerramento limpo da aplicação.
