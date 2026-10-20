# Especificação Técnica — Migração da Imagem Cross do Mandu para Docker Hub (#6)

## 1. Contexto e Motivação
A esteira de publicação multi-arquitetura na Banca (`.forgejo/workflows/release-banca.yaml`) utilizava temporariamente a sub-imagem `codeberg.org/lumenpink/gost-ci-cross:latest`. Devido à centralização soberana de contêineres de CI no **Docker Hub** (`docker.io/lumenpink/...`), a referência da imagem deve ser atualizada para `docker.io/lumenpink/gost-ci-cross:latest`.

## 2. Requisitos de Implementação
1. **Contêiner da Esteira:** `.forgejo/workflows/release-banca.yaml` deve declarar `image: docker.io/lumenpink/gost-ci-cross:latest`.
2. **Documentação Técnica:** `docs/specs/05_multiarch_banca_pipeline.md` atualizado refletindo o registry correto.
3. **Validação Automatizada:** `tests/test-pipeline-multiarch.sh` exigindo exclusivamente o registro do Docker Hub.

## 3. Critérios de Aceite
1. Teste `tests/test-pipeline-multiarch.sh` aprovado sem falhas.
2. Conformidade integral nas auditorias de governança `cccp check --strict`.
