#!/bin/sh

set -eu
FILE=$1
KEY=this_is_dummy_private_id
LOG_FILE=./submission.log

echo "$(date +"%Y-%m-%d %T"): Submitting $FILE" | tee -a $LOG_FILE
echo "$(date +"%Y-%m-%d %T"): MD5: $(md5sum $FILE)" | tee -a $LOG_FILE
curl -F "private_id=$KEY" -F "file=@$FILE" https://monadic-lab.org/submit | tee -a $LOG_FILE
