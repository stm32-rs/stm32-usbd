#!/usr/bin/env bash

set -e

families=$(cat Cargo.toml | grep -E "^stm32[^ ]+ = \[" | cut -d " " -f1)

set -euxo pipefail

for family in $families; do
    cargo check --no-default-features --features $family
done
