#!/bin/bash

set -eu

LOG_FILE=test.log
BASEDIR=$(dirname "$0")
cd $BASEDIR

echo "$(date +"%Y-%m-%d %T"): HEAD $(git rev-parse HEAD)" | tee -a $LOG_FILE
echo "$(date +"%Y-%m-%d %T"): Running test" | tee -a $LOG_FILE
echo "$(date +"%Y-%m-%d %T"): Building main" | tee -a $LOG_FILE
cargo build --release

OUTPUT="./solutions/test"

echo "Creating $OUTPUT"
mkdir -p $OUTPUT

SECONDS=0
./target/release/icfpc2019 --output $OUTPUT --input ./input
echo "$(date +"%Y-%m-%d %T"): running time is $SECONDS secs" | tee -a $LOG_FILE

cargo run --bin score --release -- --input ./input --output $OUTPUT | tee -a $LOG_FILE
