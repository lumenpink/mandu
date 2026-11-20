# Spec 50: Desabilitar staticlib joaju no build Android e expor stubs FFI

## 1. Contexto e Problema
Na compilação do `mandu-mobile` para alvos Android (`aarch64-linux-android` e `x86_64-linux-android`), o job de CI de empacotamento do APK (`dx build --android --renderer webview --release`) falhava na etapa de linkedição com erros de:
1. Símbolos duplicados entre o runtime Rust da libstd Android e a staticlib `libjoaju-aarch64-linux.a` (`__rustc::__rust_foreign_exception`, `rust_eh_personality`).
2. Símbolo duplicado `joaju_compile_markdown_to_html` (presente tanto na lib Rust `joaju` compilada pelo Cargo quanto no `libjoaju-aarch64-linux.a`).
3. Símbolos indefinidos de chamadas ausentes na Bionic libc do Android (`posix_spawn_file_actions_addchdir_np`, `__xpg_strerror_r`).

Isso ocorria porque o arquivo de construção `crates/mandu-core/build.rs` identificava qualquer alvo contendo `aarch64` como Linux tradicional (`libjoaju-aarch64-linux.a`), baixava o artefato compilado contra glibc e instruía o Cargo a vincular estaticamente `joaju` (`cargo:rustc-link-lib=static=joaju`).

## 2. Decisões Arquiteturais e Árvore de Decisão
Para que Natasha compreenda as razões por trás deste ajuste estrutural:

- **Alternativa A (Descartada):** Compilar uma versão da staticlib `libjoaju.a` nativamente para cada alvo Android (`aarch64-linux-android`, etc.) contra a Bionic libc e empacotar no generic packages da Forgejo.
  - *Por que descartada:* No ecossistema móvel do Mandu, os motores de PDF via Typst e compilação científica pesada nativa são intencionalmente desativados ou substituídos pela pré-visualização webview/HTML com a esteira `joaju::compile_markdown_to_html_with_source`. O custo e peso do motor Typst estático dentro de um APK móvel não se justificam e violam a separação de responsabilidades do cliente móvel leve.
- **Alternativa B (Descartada):** Forçar flags de linkedição permissivas (`-Wl,--allow-multiple-definition` e stubs de glibc injetados via linker wrapper).
  - *Por que descartada:* Cria instabilidade de runtime severa e quebra a portabilidade Android em diferentes níveis de API da Bionic libc.
- **Alternativa C (Escolhida):**
  1. No `crates/mandu-core/build.rs`, detectar se a variável de ambiente `TARGET` contém `android`. Em caso afirmativo, abortar imediatamente a linkedição da staticlib `joaju` (`return;`).
  2. No módulo FFI `crates/mandu-core/src/joaju_ffi.rs`, condicionar as declarações `extern "C"` e as funções reais a `#[cfg(not(target_os = "android"))]`.
  3. No mesmo módulo, para `#[cfg(target_os = "android")]`, fornecer implementações seguras que retornam `Err(...)` indicando que a funcionalidade FFI estática é desabilitada em ambiente Android.

## 3. Plano de Implementação
1. Modificar `crates/mandu-core/build.rs`:
   - Checar `if target.contains("android") { return; }` logo no início de `main()`.
2. Modificar `crates/mandu-core/src/joaju_ffi.rs`:
   - Envolver o bloco `extern "C"` e as funções FFI sob `#[cfg(not(target_os = "android"))]`.
   - Adicionar bloco `#[cfg(target_os = "android")]` com os stubs compatíveis retornando erros explícitos.
3. Testar a compilação cruzada para `aarch64-linux-android` usando `cargo check --target aarch64-linux-android -p mandu-core`.
4. Executar os testes nativos para certificar que plataformas desktop continuam funcionando sem regressão.
