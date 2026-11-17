#!/bin/bash
# nx02 diagnostic for city_munich --junctions.join-dist 20 (TEMPORARY: removed in the final commit)
cd "$(dirname "$0")/../.." || exit 1
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/tmp/target}
D=/tmp/diag_nx02; rm -rf $D; mkdir -p $D
C=tools/oracle/cases/city_munich
ARGS="--simple-projection true --junctions.join true --junctions.join-dist 20 --osm-files"
"$CARGO_TARGET_DIR/debug/netconvert" $ARGS "$C/input.osm" -o $D/rust.net.xml > $D/rust.log 2>&1; echo "rust rc=$?"
SUMO_HOME=$SUMO_HOME "$SUMO_PRISTINE_BIN/netconvert" $ARGS "$C/input.osm" -o $D/cpp.net.xml > $D/cpp.log 2>&1; echo "cpp rc=$?"
echo "== warnings cpp / rust (junction-related)"
grep -aiE "junction|cluster" $D/cpp.log | sort | uniq -c | head -20
echo "--"
grep -aiE "junction|cluster" $D/rust.log | sort | uniq -c | head -20
echo "== junction id sets"
python3 - "$D/cpp.net.xml" "$D/rust.net.xml" <<'EOF'
import sys, xml.etree.ElementTree as ET
def load(p):
    r = ET.parse(p).getroot()
    J = {j.get('id'): dict(j.attrib) for j in r.iter('junction')}
    L = {}
    for e in r.iter('edge'):
        for l in e.iter('lane'):
            L[l.get('id')] = dict(l.attrib)
    return J, L
cj, cl = load(sys.argv[1]); rj, rl = load(sys.argv[2])
print("junctions cpp=%d rust=%d only_cpp=%s only_rust=%s" % (len(cj), len(rj), sorted(set(cj)-set(rj))[:10], sorted(set(rj)-set(cj))[:10]))
print("lanes cpp=%d rust=%d only_cpp=%d only_rust=%d" % (len(cl), len(rl), len(set(cl)-set(rl)), len(set(rl)-set(cl))))
n = 0
for k in sorted(cj):
    if k in rj and cj[k] != rj[k]:
        print("JDIFF", k, {a: (cj[k].get(a), rj[k].get(a)) for a in set(cj[k]) | set(rj[k]) if cj[k].get(a) != rj[k].get(a)})
        n += 1
        if n >= 4: break
n = 0
for k in sorted(cl):
    if k in rl and cl[k] != rl[k]:
        print("LDIFF", k, cl[k].get('length'), rl[k].get('length'), "| cpp shape:", (cl[k].get('shape') or '')[:160], "| rust shape:", (rl[k].get('shape') or '')[:160])
        n += 1
        if n >= 4: break
EOF
