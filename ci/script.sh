#!/usr/bin/env bash

set -euxo pipefail

cargo check
cargo check --example hal
cargo fmt -- --check
