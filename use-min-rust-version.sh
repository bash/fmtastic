#!/usr/bin/env bash

set -euo pipefail

rust_version=$(cargo metadata --format-version 1 --no-deps | jq -r '.packages | first | .rust_version')
rustup override set "$rust_version"
