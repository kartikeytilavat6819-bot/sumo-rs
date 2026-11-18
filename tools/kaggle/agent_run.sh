#!/bin/bash
c=tools/oracle/cases/elechybrid_overhead; O=/tmp/f03; mkdir -p $O
B=$CARGO_TARGET_DIR/debug/sumo; [ -x $B ] || B=$B.exe
$B -n $c/ref_simple.net.xml -r $c/routes.rou --tripinfo-output $O/ti.xml --fcd-output $O/fcd.xml --no-step-log true $(tr -d '\r' < $c/sim.args) --elechybrid-output $O/elechybrid.xml --overheadwiresegments-output $O/overheadwires.xml --substations-output $O/substations.xml > $O/log 2>&1
grep -ac "out of bounds" $O/log; grep -ac "could not be delivered" $O/log
python3 - <<PY
import re
c="$c"; O="$O"
def ld(f): return [re.findall(r'(\w+)="([^"]*)"',l) for l in open(f) if '<vehicle' in l or '<step' in l or '<tractionSubstation' in l]
for s,tag in (('elechybrid','e'),('overheadwires','o'),('substations','s')):
    r=ld(f"{c}/ref.{s}.xml"); k=ld(f"{O}/{s}.xml"); n=0
    print("==",s,len(r),len(k))
    for i,(a,b) in enumerate(zip(r,k)):
        if a!=b:
            d=[(x[0],x[1],y[1]) for x,y in zip(a,b) if x!=y]
            print(i,d[:4]); n+=1
            if n>=12: break
PY
