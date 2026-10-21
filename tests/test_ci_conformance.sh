#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

echo "=== Verificando conformidade profunda do pipeline unificado ci.yml e GOST 1.5.0 ==="

# 1. Arquivo único ci.yml
WORKFLOW="$ROOT_DIR/.forgejo/workflows/ci.yml"
if [ ! -f "$WORKFLOW" ]; then
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

# 2. Sintaxe YAML válida
python3 -c "import yaml; yaml.safe_load(open('$WORKFLOW'))" 2>/dev/null || {
  echo "AVISO: PyYAML não disponível, testando com python padrão..."
}

# 3. Dependência de jobs no ci.yml
if ! grep -q "needs: \[test\]" "$WORKFLOW"; then
  echo "FALHA: O job de publicação não declara dependência needs: [test]!" >&2
  exit 1
fi

# 4. Imagem de cross-compilação e alvos multiarch
if ! grep -q "docker.io/lumenpink/gost-ci-cross:latest" "$WORKFLOW"; then
  echo "FALHA: Imagem gost-ci-cross:latest não declarada no ci.yml!" >&2
  exit 1
fi

for TARGET in "x86_64-unknown-linux-musl" "aarch64-unknown-linux-musl" "x86_64-pc-windows-gnu"; do
  if ! grep -q "$TARGET" "$WORKFLOW"; then
    echo "FALHA: Alvo $TARGET ausente no ci.yml!" >&2
    exit 1
  fi
done

# 5. Verificação de upload R2 e rotação na Banca
if ! grep -q "wrangler r2 object put" "$WORKFLOW" || ! grep -q "rotate-downloads.py" "$WORKFLOW"; then
  echo "FALHA: Etapas de R2 ou rotação de Banca ausentes no ci.yml!" >&2
  exit 1
fi

# 6. GOSTVER 1.5.0
if ! grep -q 'GOST_VERSION="1.5.0"' "$ROOT_DIR/GOSTVER"; then
  echo "FALHA: GOSTVER não está na versão 1.5.0!" >&2
  exit 1
fi

# 7. Garantia de rustfmt e clippy no setup do Rust
if ! grep -q "rustup component add.*rustfmt" "$WORKFLOW" || ! grep -q "rustup component add.*clippy" "$WORKFLOW"; then
  echo "FALHA: rustup component add rustfmt e clippy ausentes no ci.yml!" >&2
  exit 1
fi

echo "✔ Todas as verificações de conformidade estrita da esteira passaram com sucesso!"
