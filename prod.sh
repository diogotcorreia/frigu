#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

pushd frontend
# note: when using SpaRouter this needs to be
#   "trunk build --public-url /assets/"
trunk build
popd

pushd server
cargo run --release -- --port 8080
popd
