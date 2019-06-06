#!/usr/bin/env bash

set -euxo pipefail

cargo check --no-default-features --features stm32f0,stm32f0xx-hal/stm32f042
cargo check --no-default-features --features stm32f103xx
cargo check --no-default-features --features stm32f303xc
cargo check --no-default-features --features stm32l4x2xx
cargo check --no-default-features --features stm32f103xx --target thumbv7m-none-eabi --examples
