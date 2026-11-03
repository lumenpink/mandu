#!/usr/bin/env bash
set -euo pipefail

CARGO_TOML="Cargo.toml"
CI_YML=".forgejo/workflows/ci.yml"

echo "Verificando perfis de debuginfo em $CARGO_TOML..."

# 1. Verificar se [profile.dev] tem debug = 1
if ! grep -A 2 '\[profile\.dev\]' "$CARGO_TOML" | grep -q 'debug = 1'; then
    echo "FALHA: [profile.dev] nao contem 'debug = 1' em $CARGO_TOML" >&2
    exit 1
fi

# 2. Verificar se [profile.test] tem debug = 1
if ! grep -A 2 '\[profile\.test\]' "$CARGO_TOML" | grep -q 'debug = 1'; then
    echo "FALHA: [profile.test] nao contem 'debug = 1' em $CARGO_TOML" >&2
    exit 1
fi

echo "Verificando variavel CARGO_PROFILE_TEST_DEBUG em $CI_YML..."

# 3. Verificar se CARGO_PROFILE_TEST_DEBUG: "1" esta configurado na CI
if ! grep -q 'CARGO_PROFILE_TEST_DEBUG: "1"' "$CI_YML"; then
    echo "FALHA: $CI_YML nao define CARGO_PROFILE_TEST_DEBUG: \"1\"" >&2
    exit 1
fi

echo "SUCESSO: Configuracao de debuginfo validada para otimizacao de memoria."
exit 0
