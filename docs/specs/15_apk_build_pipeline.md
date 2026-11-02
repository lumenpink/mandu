# Especificação Canônica — Pipeline de Build e Publicação do APK do Mandu (#15)

## 1. Contexto & Problema
O arquivo APK distribuído anteriormente na Pratilera (`mandu.apk`) possuía tamanho anômalo (~16 KB) por ter sido gerado por um modo de contingência no script de empacotamento, sem binários compilados (`.so`), código DEX ou assets Dioxus executáveis. Isso resultava em fechamento forçado ou tela preta em dispositivos Android físicos. Além disso, a esteira de integração contínua (`.forgejo/workflows/ci.yml`) não contemplava uma etapa de compilação automatizada de APK.

## 2. Requisitos & Critérios de Aceitação
1. **Compilação Real na Esteira (CI/CD):**
   - Adicionar o job `deploy-apk` em `.forgejo/workflows/ci.yml`, executado hermeticamente no container oficial `gambi.2lp.in/lumen/gost-ci-android:latest`.
   - Compilar o aplicativo Android (`crates/mandu-mobile`) via toolchain Dioxus 0.7 (`dx build`).
2. **Validação Estrita de Tamanho:**
   - O APK gerado deve possuir tamanho >= 1 MB (1.048.576 bytes). Pacotes de contingência ocos (< 1 MB) devem causar falha imediata na esteira/script.
3. **Assinatura e Publicação Canônica:**
   - Assinar o APK com a chave oficial `in_twolp_release.keystore`.
   - Publicar no Cloudflare R2 (`twolp-bucket/mandu.apk` e versionado por canal em `twolp-bucket/downloads/mandu/${CHANNEL}/mandu.apk`).
   - Disparar atualização do repositório da Pratilera (Banca).
4. **Resolução de Caminhos na Banca:**
   - Garantir que a Banca localize e sincronize o binário canônico gerado em `dist/android/mandu.apk` ou `dist/mandu.apk`.
