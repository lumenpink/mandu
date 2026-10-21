#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

echo "=== Verificando conformidade do pipeline unificado ci.yml e GOST 1.5.0 ==="

# 1. Arquivo único ci.yml
if [ ! -f "$ROOT_DIR/.forgejo/workflows/ci.yml" ]; then
  echo "FALHA: .forgejo/workflows/ci.yml não existe!" >&2
  exit 1
fi

if [ -f "$ROOT_DIR/.forgejo/workflows/ci.yaml" ]; then
  echo "FALHA: Arquivo legado .forgejo/workflows/ci.yaml ainda existe!" >&2
  exit 1
fi

if [ -f "$ROOT_DIR/.forgejo/workflows/release-banca.yaml" ]; then
  echo "FALHA: Arquivo legado .forgejo/workflows/release-banca.yaml ainda existe!" >&2
  exit 1
fi

# 2. Dependência de jobs no ci.yml
if ! grep -q "needs:.*test" "$ROOT_DIR/.forgejo/workflows/ci.yml"; then
  echo "FALHA: O job de publicação não declara dependência needs: [test]!" >&2
  exit 1
fi

# 3. Imagem de cross-compilação e alvos multiarch
if ! grep -q "docker.io/lumenpink/gost-ci-cross:latest" "$ROOT_DIR/.forgejo/workflows/ci.yml"; then
  echo "FALHA: Imagem gost-ci-cross:latest não declarada no ci.yml!" >&2
  exit 1
fi

if ! grep -q "x86_64-unknown-linux-musl" "$ROOT_DIR/.forgejo/workflows/ci.yml" || \
   ! grep -q "aarch64-unknown-linux-musl" "$ROOT_DIR/.forgejo/workflows/ci.yml" || \
   ! grep -q "x86_64-pc-windows-gnu" "$ROOT_DIR/.forgejo/workflows/ci.yml"; then
  echo "FALHA: Alvos multiarch incompletos no ci.yml!" >&2
  exit 1
fi

# 4. GOSTVER 1.5.0
if ! grep -q 'GOST_VERSION="1.5.0"' "$ROOT_DIR/GOSTVER"; then
  echo "FALHA: GOSTVER não está na versão 1.5.0!" >&2
  exit 1
fi

echo "✔ Todas as verificações de conformidade da esteira passaram com sucesso!"
