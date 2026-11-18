#!/bin/bash
# oc69 measurement: run all imported netconvert TextTest rows against pristine C++.
B=$(cd "$(dirname "$0")" && pwd)
ROOT=$(cd "$B/../.." && pwd)
cd "$ROOT" || exit 1
export NETOPTS2_OUT=/tmp/oc69_netopts
names=$(grep '^row ' tools/oracle/import_texttest_net.rows | awk '{print $2}')
bash tools/oracle/run_netopts_new.sh $names 2>&1 | grep -E 'FAIL|PANIC|TOTAL' | head -60
