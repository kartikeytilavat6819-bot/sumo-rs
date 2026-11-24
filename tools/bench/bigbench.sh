#!/bin/bash
# Rust vs official C++ sumo 1.28.0 on generated 1-hour grid scenarios (official netgenerate + randomTrips/duarouter)
export SUMO_HOME=/home/gate/sumo_pkg/sumo PATH=/home/gate/sumo_pkg/sumo/bin:$PATH
CPP=/home/gate/sumo_pkg/sumo/bin/sumo; RS=/home/gate/demo/target/release/sumo
W=/home/gate/bench; mkdir -p $W; cd $W
gen() {
  [ -f $1.rou.xml ] || { netgenerate --grid --grid.number $2 --grid.length 200 --default.lanenumber 2 --tls.guess true -o $1.net.xml >/dev/null 2>&1
    python3 $SUMO_HOME/tools/randomTrips.py -n $1.net.xml -b 0 -e 3600 -p $3 --seed 42 --min-distance 600 -o $1.trips.xml -r $1.rou.xml >/dev/null 2>&1; }
  echo "$1: $(grep -c '<vehicle ' $1.rou.xml) vehicles on a $2x$2 grid"
}
norm() { sed -e 's/generated on [^ ]* by/generated on X by/' -e 's/\.\(cpp\|rs\)\./.X./g' -e '/<performance /d' "$1"; }
gen small 15 1.0; gen medium 15 0.3; gen large 25 0.2
printf '\n%-8s %-6s %9s %9s %11s %11s %8s  %s\n' scenario model cpp_s rust_s cpp_peakMB rust_peakMB speedup outputs
run() { # scen mode extra
  s=$1; mode=$2; extra=$3
  for b in cpp rs; do bin=$CPP; [ $b = rs ] && bin=$RS; tt=0; mm=0
    for i in 1 2 3; do
      /usr/bin/time -o tm.txt -f "%e %M" $bin -n $s.net.xml -r $s.rou.xml $extra --no-step-log true --duration-log.disable true \
        --tripinfo-output $s.$mode.$b.tripinfo.xml --statistic-output $s.$mode.$b.stats.xml >/dev/null 2>$s.$mode.$b.err
      read e m < <(tail -1 tm.txt); tt=$(echo "$tt + $e" | bc); [ "$m" -gt "$mm" ] && mm=$m
    done
    eval "T_$b=$(echo "scale=3; $tt/3" | bc); M_$b=$((mm/1024))"
  done
  o=""; for k in tripinfo stats; do cmp -s <(norm $s.$mode.cpp.$k.xml) <(norm $s.$mode.rs.$k.xml) && o="$o $k=identical" || o="$o $k=DIFF"; done
  printf '%-8s %-6s %9.2f %9.2f %11d %11d %7.2fx %s\n' "$s" "$mode" "$T_cpp" "$T_rs" "$M_cpp" "$M_rs" "$(echo "scale=3; $T_cpp/$T_rs" | bc)" "$o"
}
run small micro ""; run medium micro ""; run large micro ""; run large meso "--mesosim true"
echo; echo "machine: $(nproc) vCPU, $(free -g | awk '/Mem/{print $2}') GB RAM; time = mean of 3 runs, peak memory = max of 3 runs"
