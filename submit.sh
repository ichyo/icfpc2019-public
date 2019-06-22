#!/bin/sh

set -e
KEY=this_is_dummy_private_id
LOG_FILE=./submission.log

if [ -z "$1" ]; then
    OUTPUT="./solutions/$(date +"%m%d")/$(date +"%H")/$(date +"%M%S")"
    OUTPUT=$OUTPUT ./make_solution.sh
    FILE=$OUTPUT/solutions.zip
else
    FILE=$1
fi

echo "$(date +"%Y-%m-%d %T"): Submitting $FILE" | tee -a $LOG_FILE
echo "$(date +"%Y-%m-%d %T"): MD5: $(md5sum $FILE)" | tee -a $LOG_FILE
curl -F "private_id=$KEY" -F "file=@$FILE" https://monadic-lab.org/submit | tee -a $LOG_FILE
