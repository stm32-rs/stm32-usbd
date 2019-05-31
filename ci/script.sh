#!/usr/bin/env bash

set -e

families=$(cat Cargo.toml | grep -E "^stm32[^ ]+ = \[" | cut -d " " -f1)

set -euxo pipefail

for family in $families; do
    cargo check --no-default-features --features $family
done

cargo check --no-default-features --features stm32f103xx --target thumbv7m-none-eabi --examples
