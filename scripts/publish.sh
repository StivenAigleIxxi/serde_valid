#!/bin/sh

set -e

cd "$(dirname "$0")/.."

cd derive
cargo publish

cd ../literal
cargo publish

# wait tarball package publishment
sleep 20

cd ../
cargo publish
