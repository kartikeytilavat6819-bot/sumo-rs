#!/bin/bash
# netconvert options users add for OSM, plus SUMO-net re-import: C++ vs ours, compared with compare_net.py.
# Inputs: cases/{real,city_plano,city_munich}/input.osm (--simple-projection true) and the ref_simple.net.xml of
# each case for `-s` re-import. Outputs go to $NETOPTS_OUT (default: a Windows temp dir, not the repo):
#   <out>/<net>/<variant>/{cpp,rust}.log, cpp.net.xml, rust.net.xml, net.log, and <out>/<net>/summary.tsv.
# Verdicts: PASS (same exit code and identical net, or both refuse with the same exit code),
#           FAIL (exit codes or net differ, or a net is missing), PANIC (our binary panicked).
# Usage: run_netopts.sh [net ...]   default: real city_plano city_munich city_berlin_priority city_singapore_lefthand reimport
# NETOPTS_JOBS=n runs n variants at a time (default 1). --proj.utm runs without --simple-projection (they conflict).
B=$(cd "$(dirname "$0")" && pwd)
C=$B/cases
RUST=${RUST_NETCONVERT:-/mnt/c/Users/karti/sumo-rs-target-x07/debug/netconvert.exe}
CPP=${CPP_NETCONVERT:-$HOME/sumo-src/bin/netconvert}
CPP_SH=${CPP_SUMO_HOME:-$HOME/sumo-src}
OUT=${NETOPTS_OUT:-/mnt/c/Users/karti/AppData/Local/Temp/opencode/netopts}
# Windows path for a Windows binary under WSL; identity on native Linux (gate VM, Kaggle)
winpath() { if command -v wslpath >/dev/null 2>&1; then wslpath -w "$1"; else printf '%s' "$1"; fi; }

NAMES=()
ARGS=()
add() { NAMES+=("$1"); ARGS+=("$2"); }
add base ""
add geomRemove "--geometry.remove true"
add ramps "--ramps.guess true"
add roundabouts "--roundabouts.guess true"
add junctionsJoin "--junctions.join true"
add tlsGuess "--tls.guess-signals true"
add tlsJoin "--tls.join true"
add tlsDiscard "--tls.discard-simple true"
add edgesJoin "--edges.join true"
add removeIsolated "--remove-edges.isolated true"
add keepVclass "--keep-edges.by-vclass passenger"
add removeFootway "--remove-edges.by-type highway.footway,highway.cycleway"
add sidewalks "--sidewalks.guess true"
add crossings "--crossings.guess true"
add turnLanes "--osm.turn-lanes true"
add laneAccess "--osm.lane-access true"
add elevation "--osm.elevation true"
add noInternal "--no-internal-links true"
add lanewidth "--default.lanewidth 3.2"
add streetNames "--output.street-names true"
add originalNames "--output.original-names true"
add bundle "--geometry.remove true --ramps.guess true --junctions.join true --tls.guess-signals true --tls.discard-simple true --tls.join true --tls.default-type actuated"
add geomMinDist "--geometry.min-dist 1"
add junctionsJoinDist "--junctions.join true --junctions.join-dist 20"
add tlsGuessFlag "--tls.guess true"
add tlsGuessThreshold "--tls.guess true --tls.guess.threshold 100"
add tlsLayoutIncoming "--tls.layout incoming"
add tlsLayoutAlternate "--tls.layout alternateOneWay"
add tlsDefaultActuated "--tls.default-type actuated"
add tlsDefaultDelay "--tls.default-type delay_based"
add noTurnarounds "--no-turnarounds true"
add cornerDetail0 "--junctions.corner-detail 0"
add linkDetail "--junctions.internal-link-detail 2"
add bikelanes "--bikelanes.guess true"
add allAttributes "--osm.all-attributes true"
add noNormalization "--offset.disable-normalization true"
add projUtm "--proj.utm true"

# run_pair NET VARIANT MODE [netconvert args...]   MODE=osm: NET is a case dir; MODE=reimport: VARIANT is a case dir
run_pair() {
  net=$1; v=$2; mode=$3; shift 3
  d=$OUT/$net/$v
  mkdir -p "$d"
  rm -f "$d/rust.net.xml" "$d/cpp.net.xml"
  if [ "$mode" = osm ]; then
    in=$C/$net/input.osm
    sp=(--simple-projection true)
    case " $* " in *" --proj.utm "*) sp=() ;; esac
    rargs=("${sp[@]}" "$@" --osm-files "$(winpath "$in")")
    cargs=("${sp[@]}" "$@" --osm-files "$in")
  else
    in=$C/$v/ref_simple.net.xml
    rargs=(-s "$(winpath "$in")")
    cargs=(-s "$in")
  fi
  timeout 900 "$RUST" "${rargs[@]}" -o "$(winpath "$d")/rust.net.xml" > "$d/rust.log" 2>&1
  rrc=$?
  SUMO_HOME=$CPP_SH timeout 900 "$CPP" "${cargs[@]}" -o "$d/cpp.net.xml" > "$d/cpp.log" 2>&1
  crc=$?
  if grep -q panicked "$d/rust.log"; then
    V=PANIC; D=$(grep -m1 -A1 panicked "$d/rust.log" | tr '\n' ' ')
  elif [ $rrc -ne $crc ]; then
    V=FAIL; D="exit rust=$rrc cpp=$crc: $(grep -m1 -i error "$d/rust.log")"
  elif [ $crc -ne 0 ]; then
    V=PASS; D="both exit $crc: $(grep -m1 -i error "$d/cpp.log")"
  elif [ -s "$d/rust.net.xml" ] && [ -s "$d/cpp.net.xml" ]; then
    python3 "$B/compare_net.py" "$d/cpp.net.xml" "$d/rust.net.xml" > "$d/net.log" 2>&1
    cr=$?
    if [ $cr -eq 0 ]; then
      V=PASS; D="net identical"
    elif [ $cr -eq 2 ]; then
      V=FAIL; D="net unparseable: $(head -1 "$d/net.log")"
    else
      V=FAIL; D="$(head -1 "$d/net.log") | $(grep SUMMARY "$d/net.log")"
    fi
  else
    V=FAIL; D="exit 0/0 but net missing (rust=$(test -s "$d/rust.net.xml" && echo yes || echo no) cpp=$(test -s "$d/cpp.net.xml" && echo yes || echo no))"
  fi
  printf '%s\t%s\t%s\t%s\n' "$net" "$v" "$V" "$D" >> "$OUT/$net/summary.tsv"
  echo "$net $v $V $D"
}

NETS=("$@")
[ ${#NETS[@]} -eq 0 ] && NETS=(real city_plano city_munich city_berlin_priority city_singapore_lefthand reimport)
for net in "${NETS[@]}"; do
  if [ "$net" = reimport ]; then
    mkdir -p "$OUT/reimport"
    : > "$OUT/reimport/summary.tsv"
    for case in real city_plano city_munich; do
      run_pair reimport "$case" reimport
    done
    continue
  fi
  mkdir -p "$OUT/$net"
  : > "$OUT/$net/summary.tsv"
  for i in "${!NAMES[@]}"; do
    read -ra A <<< "${ARGS[$i]}"
    while [ "$(jobs -rp | wc -l)" -ge "${NETOPTS_JOBS:-1}" ]; do wait -n; done
    run_pair "$net" "${NAMES[$i]}" osm "${A[@]}" &
  done
  wait
  echo "$net TOTAL PASS=$(grep -c $'\tPASS\t' "$OUT/$net/summary.tsv") FAIL=$(grep -c $'\tFAIL\t' "$OUT/$net/summary.tsv") PANIC=$(grep -c $'\tPANIC\t' "$OUT/$net/summary.tsv")"
done
