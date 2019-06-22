#!/bin/bash

set -e

LOG_FILE=./solution.log

echo "$(date +"%Y-%m-%d %T"): Building main"
cargo build --release

if [ -z "${OUTPUT}" ]; then
    OUTPUT="./solutions/$(date +"%m%d")/$(date +"%H")/$(date +"%M%S")"
fi

echo "$(date +"%Y-%m-%d %T"): Creating $OUTPUT" | tee -a $LOG_FILE
mkdir -p $OUTPUT

./target/release/icfpc2019 --output $OUTPUT --input ./input

echo "$(date +"%Y-%m-%d %T"): Creating zip $OUTPUT/solutions.zip"
cd $OUTPUT
zip solutions.zip ./*.sol -q

cd -
cargo run --bin score --release -- --input ./input --output $OUTPUT | tee -a $LOG_FILE
