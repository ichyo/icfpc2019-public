#!/bin/bash

set -e

LIST_FILE=./submissions.txt
OUTPUT=./best_solution
LOG_FILE=./best.log

echo "$(date +"%Y-%m-%d %T"): Running make best" | tee -a $LOG_FILE
cat $LIST_FILE | tee -a $LOG_FILE

cargo run --bin compare -- --output $OUTPUT --input ./input --file $LIST_FILE

echo "$(date +"%Y-%m-%d %T"): Creating zip $OUTPUT/solutions.zip" | tee -a $LOG_FILE
cd $OUTPUT
zip solutions.zip ./*.sol ./*.buy -q

cd -
cargo run --bin score --release -- --input ./input --output $OUTPUT | tee -a $LOG_FILE

