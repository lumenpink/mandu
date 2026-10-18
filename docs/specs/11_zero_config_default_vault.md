# Especificação Técnica — Ticket #11: Ergonomia Zero-Config: Resolução Automática do Cofre Padrão e Ponto de Entrada

## 1. Visão Geral
Esta especificação estabelece o comportamento determinístico para inicialização do ecossistema Mandu (`mandu-tui` e `mandu-gui`) quando invocado sem argumentos ou com argumentos parciais. O objetivo é eliminar atritos de uso no terminal, permitindo que o comando executado no diretório de um cofre funcione imediatamente de forma contextualizada.

## 2. Contrato de Resolução (`resolve_vault_and_file`)

A função canônica deve residir no módulo `mandu_core::vault` com a assinatura:

```rust
pub fn resolve_vault_and_file(
    vault_arg: Option<PathBuf>,
    file_arg: Option<PathBuf>,
) -> (PathBuf, Option<PathBuf>)
```

### 2.1 Resolução da Raiz do Cofre (`vault`)
1. Se `vault_arg` for fornecido (`Some(v)`), este caminho é utilizado como raiz do cofre.
2. Se `vault_arg` for omitido (`None`):
   - O cofre padrão resolve para o diretório de trabalho atual do processo (`std::env::current_dir()`).
   - Caso `current_dir()` falhe por qualquer razão de sistema, faz fallback determinístico para `PathBuf::from(".")`.

### 2.2 Resolução da Nota Inicial (`file`)
1. Se `file_arg` for fornecido (`Some(f)`), este arquivo é retornado diretamente.
2. Se `file_arg` for omitido (`None`):
   - O resolvedor inspeciona a raiz do cofre resolvido na seguinte ordem de precedência:
     1. `<vault>/indice.md`
     2. `<vault>/README.md`
   - Se algum desses arquivos existir no sistema de arquivos e for um arquivo regular, ele é selecionado e retornado como `Some(caminho)`.
   - Se nenhum existir, retorna `None`, permitindo que as cascas abram um buffer vazio conectado ao cofre.

## 3. Integração com as Cascas
Tanto `crates/mandu-tui/src/main.rs` quanto `crates/mandu-gui/src/main.rs` devem chamar `resolve_vault_and_file` antes de instanciar suas respectivas estruturas de aplicação, garantindo paridade estrita e previsibilidade.
