#!/usr/bin/env bash

set -euo pipefail

cargo metadata --format-version 1 --no-deps | jq -r '.packages | first | .rust_version'
