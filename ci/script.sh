#!/usr/bin/env bash

set -euxo pipefail

cargo check --no-default-features --features stm32f042xx
#cargo check --no-default-features --features stm32f048xx
#cargo check --no-default-features --features stm32f072xx
#cargo check --no-default-features --features stm32f078xx
cargo check --no-default-features --features stm32f103xx
cargo check --no-default-features --features stm32f303xc
cargo check --no-default-features --features stm32l4x2xx
