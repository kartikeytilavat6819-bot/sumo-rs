#!/bin/bash
# oc29 arc-flag diagnostics. Runs on the gate after tests.
cd "$(dirname "$0")/../.." || exit 1
export PATH="$HOME/.cargo/bin:$PATH"
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/tmp/target}
export SUMO_PRISTINE_BIN=${SUMO_PRISTINE_BIN:-$HOME/sumo-pristine/bin}
BIN="$CARGO_TARGET_DIR/debug/duarouter"
REF="$SUMO_PRISTINE_BIN/duarouter"
echo "=== BIN=$BIN REF=$REF ==="

echo "=== [1] duarouter oracle: arcflag case ==="
OUT=/tmp/arcflag_oracle
rm -rf "$OUT"
OUT="$OUT" bash tools/oracle/run_duarouter.sh duarouter_arcflag 2>&1 | tail -5
echo "--- ref log ---"; sed -n '1,15p' "$OUT/duarouter_arcflag/ref.log" 2>/dev/null
echo "--- cand log ---"; sed -n '1,15p' "$OUT/duarouter_arcflag/cand.log" 2>/dev/null
echo "--- routes diff (head) ---"; head -40 "$OUT/duarouter_arcflag/routes.rou.xml.diff" 2>/dev/null

echo "=== [2] probe official binary on several nets/options ==="
mkdir -p /tmp/arcflag_probe
python3 - <<'PY'
import re, os
cases = [
  ("reroute", "tools/oracle/cases/reroute/ref_simple.net.xml", None),
  ("plano", "tools/oracle/cases/city_plano/ref_simple.net.xml", "tools/oracle/cases/city_plano/routes.rou"),
  ("assen", "tools/oracle/cases/city_assen_turbo/ref_simple.net.xml", "tools/oracle/cases/city_assen_turbo/routes.rou"),
  ("munich", "tools/oracle/cases/city_munich/ref_simple.net.xml", "tools/oracle/cases/city_munich/routes.rou"),
]
for name, net, rou in cases:
    if not os.path.exists(net): continue
    if rou and os.path.exists(rou):
        edges=[]
        for line in open(rou, errors='ignore'):
            m = re.search(r'<route edges="([^"]+)"', line)
            if m:
                es = m.group(1).split()
                if len(es) > 2: edges.append((es[0], es[-1]))
            if len(edges) >= 8: break
        with open(f"/tmp/arcflag_probe/{name}.xml","w") as f:
            f.write('<?xml version="1.0" encoding="UTF-8"?>\n<trips>\n')
            for i,(a,b) in enumerate(edges):
                f.write(f'  <trip id="t{i}" depart="{i*2}" from="{a}" to="{b}"/>\n')
            f.write('</trips>\n')
    else:
        # reuse the committed arcflag trips
        open(f"/tmp/arcflag_probe/{name}.xml","w").write(open("tools/oracle/cases/duarouter_arcflag/trips.xml").read())
    print(name, net)
PY
for name in reroute plano assen munich; do
  net=$(case $name in reroute) echo tools/oracle/cases/reroute/ref_simple.net.xml;; plano) echo tools/oracle/cases/city_plano/ref_simple.net.xml;; assen) echo tools/oracle/cases/city_assen_turbo/ref_simple.net.xml;; munich) echo tools/oracle/cases/city_munich/ref_simple.net.xml;; esac)
  [ -f "$net" ] || continue
  for ie in true false; do
    "$REF" -n "$net" --trip-files /tmp/arcflag_probe/$name.xml \
      --routing-algorithm arcflag --ignore-errors $ie \
      --output-file /tmp/arcflag_probe/$name.$ie.rou.xml --no-step-log true \
      >/tmp/arcflag_probe/$name.$ie.log 2>&1
    rc=$?
    nerr=$(grep -c '^Error' /tmp/arcflag_probe/$name.$ie.log 2>/dev/null || echo 0)
    nwarn=$(grep -c '^Warning' /tmp/arcflag_probe/$name.$ie.log 2>/dev/null || echo 0)
    nroutes=$(grep -c '<route ' /tmp/arcflag_probe/$name.$ie.rou.xml 2>/dev/null || echo 0)
    echo "$name ignore-errors=$ie rc=$rc errors=$nerr warnings=$nwarn routes=$nroutes"
  done
done
