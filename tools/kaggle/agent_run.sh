#!/bin/bash
OUT=/tmp/z07dua bash tools/oracle/run_duarouter.sh duarouter_flows_randomize
d=/tmp/z07dua/duarouter_flows_randomize
ls $d/ref $d/cand
cat $d/*.diff | head -80
cat tools/oracle/cases/duarouter_flows/flows.xml
head -50 $d/ref/routes.rou.alt.xml
