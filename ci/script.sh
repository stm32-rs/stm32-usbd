#!/usr/bin/env bash

set -euxo pipefail

cargo check --features ram_access_1x16
cargo check --features ram_access_2x16
cargo check --example hal --features ram_access_1x16
