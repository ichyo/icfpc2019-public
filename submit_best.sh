#!/bin/bash

set -e

OUTPUT=./best_solution

./make_best.sh
./submit.sh $OUTPUT/solutions.zip
