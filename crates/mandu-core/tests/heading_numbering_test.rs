use mandu_core::vault::{apply_heading_numbering, Vault};
use std::path::Path;

#[test]
fn test_apply_heading_numbering_hierarchy() {
    let input = r#"# Introdução
Texto sobre introdução.

## Fundamentos
Mais detalhes.

### Subtópico A
Detalhe fino.

### Subtópico B
Outro detalhe.

# Metodologia
Abordagem empírica.

## Amostragem
Dados coletados.
"#;

    let numbered = apply_heading_numbering(input);

    assert!(numbered.contains("# 1. Introdução"));
    assert!(numbered.contains("## 1.1. Fundamentos"));
    assert!(numbered.contains("### 1.1.1. Subtópico A"));
    assert!(numbered.contains("### 1.1.2. Subtópico B"));
    assert!(numbered.contains("# 2. Metodologia"));
    assert!(numbered.contains("## 2.1. Amostragem"));
}

#[test]
fn test_heading_numbering_ignores_code_blocks() {
    let input = r##"# Seção Um

```python
# Isto é um comentário em python, não deve ser numerado
def hello():
    return "# não é título"
```

## Seção Dois
Texto aqui.
"##;

    let numbered = apply_heading_numbering(input);

    assert!(numbered.contains("# 1. Seção Um"));
    assert!(numbered.contains("# Isto é um comentário em python, não deve ser numerado"));
    assert!(numbered.contains("## 1.1. Seção Dois"));
}

#[test]
fn test_extract_number_headings_from_frontmatter() {
    let content_true = r#"---
titulo: Nota com Numeração
numerar_titulos: true
---

# Título
"#;
    let note = Vault::parse_note_content(Path::new("nota.md"), content_true);
    assert!(note.number_headings);

    let content_synonym = r#"---
titulo: Nota com Numeração
numbering: "1.1"
---

# Título
"#;
    let note_syn = Vault::parse_note_content(Path::new("nota.md"), content_synonym);
    assert!(note_syn.number_headings);

    let content_false = r#"---
titulo: Nota Sem Numeração
---

# Título
"#;
    let note_false = Vault::parse_note_content(Path::new("nota.md"), content_false);
    assert!(!note_false.number_headings);
}

#[test]
fn test_heading_numbering_skip_levels_and_empty() {
    // Empty document
    assert_eq!(apply_heading_numbering(""), "");

    // Skip levels: # directly to ###
    let input_skip = r#"# Título Principal
### Subseção Direta
## Seção Secundária
"#;
    let numbered = apply_heading_numbering(input_skip);
    assert!(numbered.contains("# 1. Título Principal"));
    assert!(numbered.contains("### 1.0.1. Subseção Direta"));
    assert!(numbered.contains("## 1.1. Seção Secundária"));
}

#[test]
fn test_heading_numbering_preserves_existing_numbers_and_whitespace() {
    let input_existing = r#"# 1. Já Numerado
## 1.1. Subtópico Já Numerado
# Novo Título Sem Número
"#;
    let numbered = apply_heading_numbering(input_existing);
    assert!(numbered.contains("# 1. Já Numerado"));
    assert!(numbered.contains("## 1.1. Subtópico Já Numerado"));
    assert!(numbered.contains("# 2. Novo Título Sem Número"));
}

#[test]
fn test_apply_heading_numbering_with_pattern_skip_and_alpha() {
    let input = r#"# Introdução
Texto aqui.

## Fundamentos
Mais texto.

### Detalhe Alfa
Detalhe 1.

### Detalhe Beta
Detalhe 2.

## Conclusão
Fim da seção.
"#;

    // Padrão _.1.a -> H1 sem número, H2 numérico, H3 alfabético minúsculo
    let numbered = mandu_core::vault::apply_heading_numbering_with_pattern(input, "_.1.a");

    assert!(
        numbered.contains("# Introdução"),
        "H1 deve estar desprovido de numeração"
    );
    assert!(
        !numbered.contains("# 1. Introdução"),
        "H1 não deve receber '1.'"
    );
    assert!(
        numbered.contains("## 1. Fundamentos"),
        "H2 deve receber '1.'"
    );
    assert!(
        numbered.contains("### 1.a. Detalhe Alfa"),
        "H3 deve receber '1.a.'"
    );
    assert!(
        numbered.contains("### 1.b. Detalhe Beta"),
        "H3 deve receber '1.b.'"
    );
    assert!(
        numbered.contains("## 2. Conclusão"),
        "H2 deve incrementar para '2.'"
    );
}

#[test]
fn test_apply_heading_numbering_pattern_inheritance_to_h6() {
    let input = r#"# Cap Um
## Sec Um
### Sub Um
#### Nivel Quatro
##### Nivel Cinco
###### Nivel Seis
"#;

    // Padrão 1.1.a declara apenas 3 níveis. H4, H5, H6 devem herdar 'a'
    let numbered = mandu_core::vault::apply_heading_numbering_with_pattern(input, "1.1.a");

    assert!(numbered.contains("# 1. Cap Um"));
    assert!(numbered.contains("## 1.1. Sec Um"));
    assert!(numbered.contains("### 1.1.a. Sub Um"));
    assert!(numbered.contains("#### 1.1.a.a. Nivel Quatro"));
    assert!(numbered.contains("##### 1.1.a.a.a. Nivel Cinco"));
    assert!(numbered.contains("###### 1.1.a.a.a.a. Nivel Seis"));
}

#[test]
fn test_apply_heading_numbering_roman_and_uppercase() {
    let input = r#"# Parte Um
## Seção A
### Item 1
# Parte Dois
## Seção A
"#;

    let numbered = mandu_core::vault::apply_heading_numbering_with_pattern(input, "I.A.1");

    assert!(numbered.contains("# I. Parte Um"));
    assert!(numbered.contains("## I.A. Seção A"));
    assert!(numbered.contains("### I.A.1. Item 1"));
    assert!(numbered.contains("# II. Parte Dois"));
    assert!(numbered.contains("## II.A. Seção A"));
}

#[test]
fn test_unassigned_adhoc_headings_suppression_and_counter_immunity() {
    let input = r#"#_ Prefácio
Texto do prefácio.

# Primeiro Capítulo
Texto do primeiro capítulo.

##_ Nota Metodológica
Aviso sem número.

## Primeira Seção
Conteúdo da primeira seção.

# Conclusão {-}
Texto de encerramento.
"#;

    let numbered = mandu_core::vault::apply_heading_numbering_with_pattern(input, "1.1.1");

    // O marcador #_ deve ser limpo para # e não ter número
    assert!(
        numbered.contains("# Prefácio"),
        "Prefácio deve ficar limpo sem #_"
    );
    assert!(!numbered.contains("# 1. Prefácio"));

    // O primeiro capítulo deve começar em 1, sem ser consumido pelo prefácio
    assert!(numbered.contains("# 1. Primeiro Capítulo"));

    // Nota metodológica ##_ deve ficar limpa e sem número
    assert!(numbered.contains("## Nota Metodológica"));
    assert!(!numbered.contains("## 1.1. Nota Metodológica"));

    // A primeira seção deve ser 1.1.
    assert!(numbered.contains("## 1.1. Primeira Seção"));

    // Conclusão {-} deve ter o {-} removido e não ter número
    assert!(numbered.contains("# Conclusão"));
    assert!(!numbered.contains("{-}"));
}

#[test]
fn test_frontmatter_number_type_and_numbering_inference() {
    let content = r#"---
title: Tese de Doutorado
number_type: "_.1.a"
---

# Introdução
## Fundamentos
"#;

    let note = Vault::parse_note_content(Path::new("tese.md"), content);
    assert!(
        note.number_headings,
        "number_type deve inferir number_headings = true"
    );
    assert_eq!(note.number_type.as_deref(), Some("_.1.a"));
}

#[test]
fn test_mutation_skip_in_middle_pattern() {
    let input = r#"# Título Principal
## Seção Oculta
### Subtópico Alfa
#### Nível Quatro Herdado
"#;

    // 1._.a -> H1 numérico, H2 suprimido, H3 alfabético, H4 herda alfabético
    let numbered = mandu_core::vault::apply_heading_numbering_with_pattern(input, "1._.a");

    assert!(numbered.contains("# 1. Título Principal"));
    assert!(numbered.contains("## Seção Oculta"));
    assert!(!numbered.lines().any(|l| l.starts_with("## 1.")));
    assert!(numbered.contains("### 1.a. Subtópico Alfa"));
    assert!(numbered.contains("#### 1.a.a. Nível Quatro Herdado"));
}

#[test]
fn test_mutation_frontmatter_aliases_and_subsequent_reset() {
    let content_alias = r#"---
title: Relatório
tipo_numeracao: "I.A"
---

# Capítulo I
## Seção A
## Seção B
# Capítulo II
## Seção A
"#;

    let note = Vault::parse_note_content(Path::new("relatorio.md"), content_alias);
    assert!(note.number_headings);
    assert_eq!(note.number_type.as_deref(), Some("I.A"));

    let numbered = mandu_core::vault::apply_heading_numbering_with_pattern(
        &note.body,
        note.number_type.as_deref().unwrap(),
    );
    assert!(numbered.contains("# I. Capítulo I"));
    assert!(numbered.contains("## I.A. Seção A"));
    assert!(numbered.contains("## I.B. Seção B"));
    assert!(numbered.contains("# II. Capítulo II"));
    assert!(numbered.contains("## II.A. Seção A"));
}

#[test]
fn test_mutation_roman_and_alpha_bounds() {
    // Gerar 10 seções romanas para verificar IV, IX, X
    let mut input = String::new();
    for i in 1..=10 {
        input.push_str(&format!("# Seção {i}\n"));
    }

    let numbered = mandu_core::vault::apply_heading_numbering_with_pattern(&input, "i");
    assert!(numbered.contains("# iv. Seção 4"));
    assert!(numbered.contains("# ix. Seção 9"));
    assert!(numbered.contains("# x. Seção 10"));
}
