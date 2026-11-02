#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WORKFLOW="$ROOT_DIR/.forgejo/workflows/ci.yml"
PACKAGE_SCRIPT="$ROOT_DIR/scripts/package-apk.sh"

echo "=== Testando Conformidade do Pipeline de Build e Publicação do APK Mandu (#15) ==="

# 1. Existência do workflow
if [ ! -f "$WORKFLOW" ]; then
  echo "FALHA: Workflow $WORKFLOW não encontrado!" >&2
  exit 1
fi

# 2. Presença do job deploy-apk
if ! grep -q "deploy-apk:" "$WORKFLOW"; then
  echo "FALHA: Job deploy-apk ausente em $WORKFLOW!" >&2
  exit 1
fi

# 3. Uso do container gost-ci-android
if ! grep -q "gambi.2lp.in/lumen/gost-ci-android:latest" "$WORKFLOW"; then
  echo "FALHA: Container gost-ci-android ausente no job deploy-apk!" >&2
  exit 1
fi

# 4. Validação de tamanho mínimo do APK (>= 1MB / 1048576 bytes)
if ! grep -E -q "1048576|1000000|1M" "$WORKFLOW"; then
  echo "FALHA: Validação de tamanho mínimo do APK (1MB) ausente em $WORKFLOW!" >&2
  exit 1
fi

# 5. Publicação do APK no Cloudflare R2
if ! grep -q "twolp-bucket/mandu.apk" "$WORKFLOW"; then
  echo "FALHA: Publicação canônica twolp-bucket/mandu.apk ausente em $WORKFLOW!" >&2
  exit 1
fi

# 6. Disparo de atualização para a Pratilera (Banca)
if ! grep -q "trigger-pratilera" "$WORKFLOW" && ! grep -q "banca/actions/workflows" "$WORKFLOW"; then
  echo "FALHA: Disparo de notificação para a Pratilera ausente no workflow!" >&2
  exit 1
fi

# 7. Script package-apk.sh não deve gerar APK mock vazio de contingência
if grep -q "hasCode=\"false\"" "$PACKAGE_SCRIPT"; then
  echo "FALHA: package-apk.sh ainda contém fallback de contingência vazio (hasCode=false)!" >&2
  exit 1
fi

echo "✓ Todos os critérios de conformidade do pipeline de APK foram atendidos!"
