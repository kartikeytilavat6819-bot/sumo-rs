#!/bin/bash
bash tools/oracle/run_netopts_new.sh ptstopNoBidi 2>&1 | tail -20
D=${NETOPTS2_OUT:-/tmp/netopts2}/ptstopNoBidi
ls $D
diff $D/cpp_out/ptstop.xml $D/rust_out/ptstop.xml | head -40
