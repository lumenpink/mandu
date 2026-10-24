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

if ! grep -q "if: success()" "$WORKFLOW"; then
  echo "FALHA: O job deploy-banca não exige if: success()!" >&2
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

# 5.1 Verificação de deploy contínuo do site estático via Cloudflare Workers
if ! grep -q "wrangler deploy" "$WORKFLOW"; then
  echo "FALHA: Etapa de deploy do site (wrangler deploy) ausente no ci.yml!" >&2
  exit 1
fi

# 6. GOSTVER 1.5.0
if ! grep -q 'GOST_VERSION="1.5.0"' "$ROOT_DIR/GOSTVER"; then
  echo "FALHA: GOSTVER não está na versão 1.5.0!" >&2
  exit 1
fi

# 7. Garantia de ambiente hermético no job test
if ! grep -A 5 "test:" "$WORKFLOW" | grep -q "lumenpink/gost-ci-cross:latest"; then
  echo "FALHA: O job test não utiliza o container hermético gost-ci-cross:latest!" >&2
  exit 1
fi

# 8. Veto a downloads ad-hoc de toolchain na esteira
if grep -q "sh.rustup.rs" "$WORKFLOW" || grep -q "apt-get" "$WORKFLOW"; then
  echo "FALHA: Instalações ad-hoc (rustup/apt-get) detectadas no ci.yml!" >&2
  exit 1
fi

# 9. Teste de mutação: sensibilidade a regressão no container do job test
python3 -c "
import yaml
with open('$WORKFLOW') as f:
    data = yaml.safe_load(f)
assert 'test' in data.get('jobs', {}), 'Job test ausente'
test_job = data['jobs']['test']
assert test_job.get('container', {}).get('image') == 'docker.io/lumenpink/gost-ci-cross:latest', 'Container image inválida'
deploy_job = data['jobs']['deploy-banca']
assert 'test' in deploy_job.get('needs', []), 'Deploy-banca precisa depender de test'
deploy_steps = [s.get('name', '') for s in deploy_job.get('steps', [])]
assert any('Cloudflare Workers' in name or 'Site' in name for name in deploy_steps), 'Step de deploy do site ausente nos steps'
" || {
  echo "FALHA: Teste estrutural de mutação falhou!" >&2
  exit 1
}

echo "✔ Todas as verificações de conformidade estrita da esteira passaram com sucesso!"
