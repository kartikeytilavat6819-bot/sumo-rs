#!/bin/bash
# TEMPORARY diagnostic (em03): harmonoise edgeData noise. Removed before the final commit.
D=tools/oracle/cases/harmonoise
cd "$D" || exit 1
"$CARGO_TARGET_DIR/debug/sumo" -n ref_simple.net.xml -r routes.rou --additional-files edgedata.add.xml --end 200 --no-step-log true >/dev/null 2>agent_err.txt
echo "exit=$?"
echo "--- trace counts: notify=$(grep -c TRACE_HN.notify agent_err.txt) update=$(grep -c 'TRACE_HN update' agent_err.txt)"
grep -m3 TRACE_HN agent_err.txt
grep 'TRACE_HN update' agent_err.txt | awk '{print $NF}' | sort | uniq -c | sort -rn | head -3
echo "--- ours (edge -103224#*)"
grep 'edge id="-103224#' out_edgedata.xml
echo "--- ref"
grep 'edge id="-103224#' ref_edgedata.edge.xml
echo "--- ours interval header"
grep -m2 'interval ' out_edgedata.xml
echo "--- ref interval header"
grep -m2 'interval ' ref_edgedata.edge.xml
echo "--- count ours/ref edge lines with noise>0"
grep -c 'noise="[1-9]' out_edgedata.xml
grep -c 'noise="[1-9]' ref_edgedata.edge.xml
