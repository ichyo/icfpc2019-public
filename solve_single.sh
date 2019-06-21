#!/bin/sh

set -eu

FILE=$1
OUTPUT=$2
ID=$(echo $FILE | sed -E 's/\.\/input\/prob\-([0-9]+)\.desc/\1/')
INPUT_FILE=./input/prob-$ID.desc
OUTPUT_FILE=$OUTPUT/prob-$ID.sol
./target/release/icfpc2019 < $INPUT_FILE > $OUTPUT_FILE
