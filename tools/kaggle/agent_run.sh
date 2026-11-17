#!/bin/bash
# ss02 temporary runner (removed in the final commit): netopts (188) and the duarouter oracle
bash tools/oracle/run_netopts.sh 2>&1 | grep TOTAL
bash tools/oracle/run_duarouter.sh 2>&1 | tail -n 3
