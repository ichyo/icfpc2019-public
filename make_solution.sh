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

SECONDS=0
./target/release/icfpc2019 --output $OUTPUT --input ./input --duration 450000
echo "$(date +"%Y-%m-%d %T"): running time is $SECONDS secs" | tee -a $LOG_FILE

echo "$(date +"%Y-%m-%d %T"): Creating zip $OUTPUT/solutions.zip" | tee -a $LOG_FILE
cd $OUTPUT
zip solutions.zip ./*.sol ./*.buy -q

cd -
cargo run --bin score --release -- --input ./input --output $OUTPUT | tee -a $LOG_FILE
