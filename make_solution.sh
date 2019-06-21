#!/bin/sh

set -eu

echo "Building main"
cargo build --release

OUTPUT="./solutions/$(date +"%m%d")/$(date +"%H")/$(date +"%M%S")"

echo "Creating $OUTPUT"
mkdir -p $OUTPUT
export OUTPUT

echo "running..."
for f in ./input/prob-*.desc; do
    ID=$(echo $f | sed -E 's/\.\/input\/prob\-([0-9]+)\.desc/\1/')
    INPUT_FILE=./input/prob-$ID.desc
    OUTPUT_FILE=$OUTPUT/prob-$ID.sol
    ./target/release/icfpc2019 < $INPUT_FILE > $OUTPUT_FILE
done

echo "Creating zip $OUTPUT/solutions.zip"
zip $OUTPUT/solutions.zip $OUTPUT
