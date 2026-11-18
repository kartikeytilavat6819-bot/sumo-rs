#!/bin/bash
c=tools/oracle/cases/elechybrid_overhead; O=/tmp/f03; mkdir -p $O
B=$CARGO_TARGET_DIR/debug/sumo; [ -x $B ] || B=$B.exe
A="-n $c/ref_simple.net.xml -r $c/routes.rou --no-step-log true $(cat $c/sim.args)"
CB=$(bash tools/gcloud/vm_cpp_trace.sh f03 tools/kaggle/f03.patch sumo)
echo "cppbin $CB"
F03T=1 $CB $A --elechybrid-output $O/c_e.xml --overheadwiresegments-output $O/c_o.xml --substations-output $O/c_s.xml 2> $O/c.err >/dev/null
F03T=1 $B $A --elechybrid-output $O/r_e.xml --overheadwiresegments-output $O/r_o.xml --substations-output $O/r_s.xml 2> $O/r.err >/dev/null
grep -a "^TRC" $O/c.err > $O/c.trc; grep -a "^TRC" $O/r.err > $O/r.trc
wc -l $O/c.trc $O/r.trc
python3 - <<'PY'
import re
def load(f):
    out=[]
    for l in open(f):
        if 'reason' in l: l=l.split(' reason')[0]
        out.append(re.sub(r'[ |:]+',' ',l.strip()).split())
    return out
c=load('/tmp/f03/c.trc'); r=load('/tmp/f03/r.trc')
n=0
for i,(a,b) in enumerate(zip(c,r)):
    def eq(x,y):
        try: return abs(float(x)-float(y))<=1e-9*max(1,abs(float(x)))
        except: return x==y
    if len(a)!=len(b) or not all(eq(x,y) for x,y in zip(a,b)):
        print("first diff line",i); 
        for k in range(max(0,i-3),i+4): print("C",' '.join(c[k][:30]) if k<len(c) else ''); print("R",' '.join(r[k][:30]) if k<len(r) else '')
        break
else: print("no diff in", min(len(c),len(r)))
PY
