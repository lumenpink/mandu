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
