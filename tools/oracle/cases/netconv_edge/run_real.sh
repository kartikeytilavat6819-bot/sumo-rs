#!/bin/bash
# Case 7: real/input.osm under option variants. Logs and nets go to netconvert_edge/real_<variant>/.
# Paths derive from this script's location so the runner works from any worktree (it used to point at
# another agent's worktree, which would have measured the wrong tree).
B=$(cd "$(dirname "$0")" && pwd)
REAL=$B/../real
RUST=${RUST_NETCONVERT:-/mnt/c/Users/karti/sumo-rs-target-main/debug/netconvert.exe}
CPP=$HOME/sumo-src/bin/netconvert
run_variant() {
  v=$1; shift
  d=$B/real_$v
  mkdir -p $d
  wd=$(wslpath -w "$d")
  wr=$(wslpath -w "$REAL")
  rm -f $d/rust.net.xml $d/cpp.net.xml
  timeout 900 "$RUST" --simple-projection true "$@" --osm-files "$wr\\input.osm" -o "$wd\\rust.net.xml" > "$d/rust.log" 2>&1
  echo "real_$v rust_exit=$?"
  SUMO_HOME=$HOME/sumo-src timeout 900 "$CPP" --simple-projection true "$@" --osm-files "$REAL/input.osm" -o "$d/cpp.net.xml" > "$d/cpp.log" 2>&1
  echo "real_$v cpp_exit=$?"
  if [ -s $d/rust.net.xml ] && [ -s $d/cpp.net.xml ]; then
    if python3 $B/../../compare_net.py $d/cpp.net.xml $d/rust.net.xml > $d/net.log 2>&1; then
      echo "real_$v net=MATCH"
    else
      echo "real_$v net=DIFF $(grep -m1 SUMMARY $d/net.log)"
    fi
  else
    echo "real_$v net=skip"
  fi
}
run_variant base
run_variant noturn --no-turnarounds true
run_variant corner3 --junctions.corner-detail 3
run_variant join --junctions.join true
run_variant joinDist --junctions.join true --junctions.join-dist 30
run_variant joinSame --junctions.join-same 10
run_variant geomRemove --geometry.remove true
run_variant geomRemoveLen --geometry.remove true --geometry.remove.min-length 5
run_variant noInternal --no-internal-links true
run_variant tlsJoin --tls.join true --tls.join-dist 50
run_variant tlsCycle60 --tls.cycle.time 60
run_variant tlsGreen --tls.green.time 20 --tls.yellow.time 4
run_variant tlsDefType --tls.default-type actuated
run_variant projScale --proj.scale 2
run_variant projRotate --proj.rotate 30
# registered-but-unported TLS options must refuse, not silently produce the default net
for opt in tls.group-signals tls.ungroup-signals tls.rebuild; do
  d=$B/refuse_$opt; mkdir -p $d; rm -f $d/rust.net.xml
  wd=$(wslpath -w "$d"); wr=$(wslpath -w "$REAL")
  timeout 900 "$RUST" --simple-projection true --$opt true --osm-files "$wr\\input.osm" -o "$wd\\rust.net.xml" > "$d/rust.log" 2>&1
  rc=$?
  if [ $rc -ne 0 ] && [ ! -s $d/rust.net.xml ] && grep -q "shortcut: option --$opt" $d/rust.log; then
    echo "refuse_$opt refused=OK"
  else
    echo "refuse_$opt refused=FAIL rust_exit=$rc"
  fi
done
