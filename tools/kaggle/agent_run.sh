#!/usr/bin/env bash
set -u
C=tools/oracle/cases/out_ssm_measures
mkdir -p /tmp/z02; cd /tmp/z02
R=$OLDPWD
"$CARGO_TARGET_DIR/debug/sumo" -n $R/$C/ref_simple.net.xml -r $R/$C/routes.rou $(cat $R/$C/sim.args) --device.ssm.file /tmp/z02/cand.ssm.xml --no-step-log > run.log 2>&1
grep -c "<conflict" $R/$C/ref.ssm.xml /tmp/z02/cand.ssm.xml
grep "<conflict" $R/$C/ref.ssm.xml | head -12
echo ---; grep "<conflict" /tmp/z02/cand.ssm.xml | head -12
echo ---REF; sed -n '/<SSMLog>/,+14p' $R/$C/ref.ssm.xml | cut -c1-300
echo ---CAND; sed -n '/<SSMLog>/,+14p' /tmp/z02/cand.ssm.xml | cut -c1-300
