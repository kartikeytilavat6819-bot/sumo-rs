#!/bin/bash
# oc65 temporary measurement: first batch of the complex/TraCI oracle.
echo "=== COMPLEX SUITE BATCH 1"
COMPLEX_LIMIT=12 COMPLEX_TIMEOUT=20 COMPLEX_OUT=/tmp/complex1 timeout 540 bash tools/oracle/run_complex_suite.sh 2>&1 | tail -40
echo "=== DONE"
