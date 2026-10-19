# Especificação Técnica - Ticket #18: Listas Numeradas Auto-Incrementais, Gramática de Padrões de Numeração e Títulos Desvinculados

## 1. Contexto e Motivação
Em editores Markdown modernos e processadores tipográficos (como Typst e Pandoc), autores frequentemente enfrentam dois desafios essenciais na estruturação de notas e documentos técnicos:

1. **Listas Ordenadas Automáticas**:
   Na digitação rápida de Markdown, é convenção comum iniciar todos os itens com `1. ` (ex: `1. Item A\n1. Item B\n1. Item C`), delegando ao parser a ordenação visual e a renumeração quando itens são movidos, adicionados ou excluídos. Anteriormente, o `mandu-tui` e o `mandu-gui` exibiam apenas o texto literal sem auto-incremento sequencial nem destaque visual diferenciado de listas com marcadores.

2. **Gramática Flexível de Numeração de Cabeçalhos e Títulos Avulsos Desvinculados**:
   O Mandu anteriormente suportava apenas a flag booleana `number_headings: true` que aplicava rigidamente a numeração arábica `1.1.1.`. No entanto, teses acadêmicas (ABNT), documentos jurídicos e relatórios corporativos requerem esquemas hierárquicos mistos (ex: romano/alfabético `"I.A.1"`, arábico/alfabético `"1.1.a"`, ou supressão de H1 `"_.1.a"` para dar liberdade ao título do documento). 
   Além disso, seções como *Introdução*, *Prefácio*, *Conclusão*, *Referências Bibliográficas* e *Anexos* por convenção documental **não devem ser numeradas**, mas precisam manter sua hierarquia estrutural sem avançar ou corromper os contadores dos capítulos subsequentes.

---

## 2. Especificação de Listas Ordenadas Auto-Incrementais

### 2.1. Detecção e Parsing
- Qualquer linha do corpo da nota cujo conteúdo (após remoção de espaços à esquerda) inicie com um ou mais dígitos seguidos de um ponto e um espaço (`^\s*(\d+)\.\s+(.*)$`) é classificada como item de lista ordenada (`OrderedListItem`).
- Um contador sequencial (`ordered_list_counter: usize`) é mantido durante a varredura do bloco.
- Ao encontrar o primeiro item, o contador é inicializado em 1. Para cada item consecutivo subsequente, o contador é incrementado em 1 (`counter += 1`), independentemente do número digitado no código-fonte original.

### 2.2. Renderização e Tipografia
- **TUI (`mandu-tui`)**:
  - Prefixo formatado: `format!("  {counter}. ")` estilizado com `Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)` (ou Cyan conforme tema).
  - Resto da linha: submetido ao `parse_inline_spans` para preservação integral de wikilinks `[[...]]`, tags `@...`, código inline e formatações tipográficas.
- **GUI (`mandu-gui`)**:
  - Alinhamento em bloco com recuo de lista e marcador destacado com o numeral calculado.

### 2.3. Reset de Bloco
O contador de lista ordenada é reinicializado para `0` imediatamente após qualquer quebra de fluxo:
- Linha vazia ou em branco (`line.trim().is_empty()`).
- Cabeçalhos de qualquer nível (`# `, `## `, `### ` etc.).
- Citações em bloco (`> `).
- Delimitadores de código ou blocos Joaju/Typst (` ``` `).
- Linhas horizontais (`---`, `***`, `___`).
- Listas não-ordenadas (`- `, `* `).

---

## 3. Especificação da Gramática de Numeração de Cabeçalhos (`number_type` / `numbering`)

### 3.1. Declaração no Frontmatter YAML
A numeração de cabeçalhos pode ser declarada no frontmatter através das chaves `number_type` ou `numbering` (com aliases legados `numeracao` / `tipo_numeracao`):

```yaml
---
title: Relatório de Arquitetura Mandu
number_type: "_.1.a"
---
```

> **Regra de Inferência Automática:** A presença de uma chave `number_type` ou `numbering` com valor não vazio ativa automaticamente `number_headings = true`, dispensando qualquer chave booleana redundante.

### 3.2. Tokens de Formatação Suportados (H1 a H6)
Os tokens válidos no padrão são delimitados por ponto (`.`) e mapeados para os níveis hierárquicos:

| Token | Tipo | Sequência Gerada | Exemplo de Saída |
| :---: | :--- | :--- | :--- |
| `1` | Arábico | `1, 2, 3, 4, 5...` | `1.`, `2.`, `10.` |
| `a` | Latino Minúsculo | `a, b, c, ... z, aa, ab...` | `a.`, `b.`, `c.` |
| `A` | Latino Maiúsculo | `A, B, C, ... Z, AA, AB...` | `A.`, `B.`, `C.` |
| `i` | Romano Minúsculo | `i, ii, iii, iv, v, vi, vii, viii, ix, x...` | `i.`, `ii.`, `iv.` |
| `I` | Romano Maiúsculo | `I, II, III, IV, V, VI, VII, VIII, IX, X...` | `I.`, `II.`, `IV.` |
| `_` | **Supressão** | *(vazio / omitido)* | Nenhuma numeração emitida |

### 3.3. Regra de Herança para Níveis Subsequentes Não Declarados
Se a especificação do padrão possuir menos de 6 níveis (ex: `"1.1.a"`, que declara apenas H1, H2 e H3), **todos os níveis hierárquicos subsequentes (H4 a H6) herdam o token do último nível declarado**:
- Padrão `"1.1.a"`: H1=`1`, H2=`1`, H3=`a`, H4=`a`, H5=`a`, H6=`a`.
- Padrão `"I.A"`: H1=`I`, H2=`A`, H3=`A`, H4=`A`, H5=`A`, H6=`A`.
- Padrão `"1"`: H1=`1`, H2=`1`, H3=`1`, H4=`1`, H5=`1`, H6=`1`.
- Padrão `"1._"`: H1=`1`, H2=`_`, H3=`_`, H4=`_` (subníveis suprimidos).

### 3.4. Mecânica do Símbolo de Supressão `_`
- Quando um nível é definido como `_`, o cabeçalho correspondente **não recebe prefixo numérico**:
  - Exemplo com `"_.1.a"`:
    - `# Título Principal` -> `# Título Principal` *(sem número)*
    - `## Seção A` -> `## 1. Seção A`
    - `### Detalhe` -> `### 1.a. Detalhe`
    - `## Seção B` -> `## 2. Seção B`
- A contagem hierárquica é preservada: ao encontrar um novo H1, os contadores de H2 a H6 são reiniciados para 0, permitindo que a próxima seção `##` recomece de `1.` sem conflitos.

---

## 4. Especificação de Cabeçalhos Não-Numerados Avulsos (`#_`, `# _`, `{-}`)

Em documentos onde a numeração global está ativada, seções pontuais precisam ser desvinculadas da contagem numérica.

### 4.1. Sintaxe Suportada
O Mandu reconhece três formas equivalentes e tolerantes:
1. `#_ Título` / `##_ Título` (notação rápida sem espaço).
2. `# _ Título` / `## _ Título` (notação estrita com espaço, compatível com CommonMark/Obsidian).
3. `# Título {-}` / `## Título {-}` (notação Pandoc / Quarto / RMarkdown).

### 4.2. Comportamento no Processamento e Visualização
1. **Limpeza Visual na Apresentação**:
   O marcador de supressão (`_`, `_ ` ou `{-}`) é **completamente removido** do texto exibido. O usuário vê na TUI e GUI o título formatado limpo:
   - `#_ Introdução` é renderizado como `▌ Introdução`.
   - `##_ Nota Metodológica` é renderizado como `▍ Nota Metodológica`.
2. **Imunidade de Contadores**:
   Títulos desvinculados **não incrementam nem avançam os contadores** de seus respectivos níveis.
   - Exemplo:
     ```markdown
     #_ Introdução
     # Primeiro Capítulo
     ```
     `#_ Introdução` não consome o índice 1; portanto, `# Primeiro Capítulo` recebe `# 1. Primeiro Capítulo`.
3. **Ponte Joaju / Typst**:
   Na exportação para Typst / PDF, cabeçalhos desvinculados são traduzidos como:
   ```typst
   #heading(level: 1, numbering: none)[Introdução]
   ```

---

## 5. Algoritmo de Conversão de Números Romanos e Alfabéticos
- **Alfabético (`a` / `A`)**: Mapeamento biunívoco bijetivo em base 26: $1 \to a$, $26 \to z$, $27 \to aa$.
- **Romano (`i` / `I`)**: Tabela padrão de numerais romanos até 3999 (`M`, `CM`, `D`, `CD`, `C`, `XC`, `L`, `XL`, `X`, `IX`, `V`, `IV`, `I`).

---

## 6. Critérios de Aceite e Verificação
- [ ] Teste unitário para auto-incremento de listas ordenadas (`1. \n 1. \n 1. ` -> ` 1. `, ` 2. `, ` 3. `).
- [ ] Teste unitário para reset de contagem de listas com quebra de parágrafo e blocos.
- [ ] Teste unitário comprovando a inferência de `number_headings = true` a partir de `number_type` / `numbering`.
- [ ] Teste unitário para os padrões `"_.1.a"`, `"1.1.a"`, `"I.A.1"`, `"1.1.1.a.a.a"`.
- [ ] Teste unitário para herança do último token em níveis subsequentes não declarados.
- [ ] Teste unitário comprovando a remoção do marcador e preservação de contadores em `#_`, `# _` e `{-}`.
