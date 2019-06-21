#!/bin/bash

set -eu

echo "Building main"
cargo build --release

OUTPUT="./solutions/$(date +"%m%d")/$(date +"%H")/$(date +"%M%S")"

echo "Creating $OUTPUT"
mkdir -p $OUTPUT
export OUTPUT

echo "running..."
for f in ./input/prob-*.desc; do
    ./solve_single.sh $f $OUTPUT
done

echo "Creating zip $OUTPUT/solutions.zip"
cd $OUTPUT
zip solutions.zip ./*.sol -q
