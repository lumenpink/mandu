# Especificação Técnica — Ticket #10: Fechamento do MVP (Autocompletar @ e [[, Peek Modal e Escape Hatch Ctrl+E)

## 1. Contexto e Objetivos
Esta especificação consolida os recursos finais previstos na `SPEC.md` para o fechamento do MVP (`v0.1.0`) do Mandu. Abrange a experiência fluida de escrita acadêmica com autocompletar contextual de citações e wikilinks, inspeção rápida de notas/referências via modal flutuante (*Peek*), integração com editores externos via *Escape Hatch* (`Ctrl+E`) na TUI e atalhos ergonômicos sem teclas de função.

## 2. Requisitos Arquiteturais e Funcionais

### 2.1. Autocompletar Contextual (`@` e `[[`)
- **Gatilho de Citações (`@`)**:
  - Ao digitar o caractere `@` seguido de termo de busca (ou acionar autocompletar), o sistema consulta a tabela `references_md` do SQLite in-memory.
  - Correspondência aproximada em tempo real com `nucleo` sobre `citekey`, título e autores.
  - Ao selecionar um item (Enter ou clique), o editor insere `@citekey`.
- **Gatilho de Links Internos (`[[`)**:
  - Ao abrir colchetes duplos `[[`, o sistema pesquisa os títulos e caminhos das notas no cofre.
  - Ao selecionar, insere `[[nome_da_nota]]` e fecha os colchetes automaticamente.

### 2.2. Modal de Peek (Espiar Conteúdo)
- Permite ler rapidamente o conteúdo de um link wiki `[[nota]]` ou nota de fichamento `@citekey` sob o cursor ou selecionada, sem navegar para longe da nota atual.
- Acionável por atalho de teclado (`Ctrl+P` no TUI e GUI) ou clique contextual.
- Exibe o texto renderizado ou o conteúdo formatado em uma janela modal flutuante sobreposta (*clear overlay* na TUI, `egui::Window` na GUI).
- Tecla `Esc` fecha a janela de Peek e restaura o foco no ponto exato de edição.

### 2.3. Escape Hatch (`Ctrl+E`) no `mandu-tui`
- Atalho global `Ctrl+E` para usuários avançados que necessitam de edição modal ou recursos especializados em editores externos.
- Fluxo de execução:
  1. Se houver edições pendentes no buffer, persiste temporariamente o arquivo ou aplica hooks de salvamento.
  2. Suspende a TUI e restaura o terminal para modo canônico (desabilita *raw mode* e sai da *alternate screen* do crossterm).
  3. Identifica o editor configurado na variável de ambiente `$EDITOR` (com fallback em ordem: `nvim`, `micro`, `nano`, `vi`).
  4. Executa o editor como processo filho em primeiro plano com `std::process::Command`.
  5. Após o encerramento do processo filho, reabilita *alternate screen* e *raw mode*, redesenha a TUI e recarrega o conteúdo do arquivo modificado do disco.

### 2.4. Conformidade Estrita com Zero Teclas de Função
- Nenhuma tecla `F1`–`F12` é mapeada ou consumida.
- Atalhos unificados em ambas as cascas:
  - `Ctrl+Space`: alternância dos modos HedgeDoc (`Source` -> `Split` -> `Preview`).
  - `Ctrl+O`: busca rápida e abertura de notas.
  - `Ctrl+S`: salvamento determinístico com linter.
  - `Ctrl+D`: compilação e exportação para PDF via Joaju/Typst.
  - `Ctrl+P`: modal de Peek (espiar link/citação sob cursor).
  - `Ctrl+E`: Escape Hatch para editor externo (TUI).
  - `Ctrl+Q`: saída segura da aplicação.
