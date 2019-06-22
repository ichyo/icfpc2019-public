#!/bin/bash

set -eu

LOG_FILE=test.log
BASEDIR=$(dirname "$0")
cd $BASEDIR

echo "$(date +"%Y-%m-%d %T"): Running test" | tee -a $LOG_FILE
echo "$(date +"%Y-%m-%d %T"): Building main" | tee -a $LOG_FILE
cargo build

OUTPUT="./solutions/test"

echo "Creating $OUTPUT"
mkdir -p $OUTPUT

./target/debug/icfpc2019 --output $OUTPUT --input ./input

cargo run --bin score -- --input ./input --output $OUTPUT | tee -a $LOG_FILE
