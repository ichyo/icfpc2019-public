#!/bin/bash

set -eu

echo "Building main"
cargo build --release

OUTPUT="./solutions/$(date +"%m%d")/$(date +"%H")/$(date +"%M%S")"

echo "Creating $OUTPUT"
mkdir -p $OUTPUT

./target/release/icfpc2019 --output $OUTPUT --input ./input

echo "Creating zip $OUTPUT/solutions.zip"
cd $OUTPUT
zip solutions.zip ./*.sol -q
