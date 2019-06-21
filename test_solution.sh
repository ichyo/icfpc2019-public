#!/bin/bash

set -eu

echo "Building main"
cargo build --release

OUTPUT="./solutions/test"

echo "Creating $OUTPUT"
mkdir -p $OUTPUT

./target/release/icfpc2019 --output $OUTPUT --input ./input
