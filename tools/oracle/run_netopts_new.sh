#!/bin/bash
# netconvert option rows that run_netopts.sh and the case net.args never use: C++ 1.28.0 netconvert vs ours.
# Each row: <name> <input> <options>. Input is osm:<case> (--simple-projection true + cases/<case>/input.osm),
# xml:<case> (the case's net.args), plain (cases/vtypes/plain), or args:<netconvert args> (repo-relative paths).
# @D in the options is the per-side output dir.
# The net is compared with compare_net.py; extra files a row writes (@D/...) are compared byte for byte after
# dropping the 'generated on' stamp line. Run from the repo root on the gate VM (SUMO_PRISTINE_BIN, CARGO_TARGET_DIR set).
# Outputs: $NETOPTS2_OUT/<row>/{cpp,rust}.log, cpp.net.xml, rust.net.xml, cpp_out/, rust_out/, net.log.
# Verdicts: PASS (same exit code, identical net and extra files), FAIL (exit code or content differs; the
# detail is the first divergence), PANIC (ours panicked). Usage: run_netopts_new.sh [row ...]  (default: all rows)
# NETOPTS2_JOBS=n runs n rows at a time (default 4).
B=$(cd "$(dirname "$0")" && pwd)
ROOT=$(cd "$B/../.." && pwd)
cd "$ROOT" || exit 1
C=$B/cases
RUST=${RUST_NETCONVERT:-$CARGO_TARGET_DIR/debug/netconvert}
CPP=${CPP_NETCONVERT:-$SUMO_PRISTINE_BIN/netconvert}
OUT=${NETOPTS2_OUT:-/tmp/netopts2}
PLAIN="--node-files tools/oracle/cases/vtypes/plain/net.nod.xml --edge-files tools/oracle/cases/vtypes/plain/net.edg.xml --connection-files tools/oracle/cases/vtypes/plain/net.con.xml --type-files tools/oracle/cases/vtypes/plain/net.typ.xml --tllogic-files tools/oracle/cases/vtypes/plain/net.tll.xml"

NAMES=(); INS=(); OPTS=()
row() { NAMES+=("$1"); INS+=("$2"); OPTS+=("$3"); }
row geomMaxSegment osm:real "--geometry.max-segment-length 15"
row geomSplit osm:real "--geometry.split true"
row geomMinRadiusFix osm:city_plano "--geometry.min-radius.fix true"
row geomMaxGradeFix osm:city_stelvio_pass_grades "--osm.elevation true --geometry.max-grade.fix true"
row geomAvoidOverlap osm:city_plano "--geometry.avoid-overlap true"
row junctionsLimitTurnSpeed osm:real "--junctions.limit-turn-speed 5.5"
row junctionsLeftBeforeRight osm:city_kumbakonam_mixed "--junctions.left-before-right true"
row tlsNoMixed osm:city_plano "--tls.no-mixed true"
row tlsMinMaxDur osm:city_plano "--tls.min-dur 5 --tls.max-dur 60"
row tlsRedAllred osm:real "--tls.red.time 3 --tls.allred.time 2"
row tlsGuessJoining osm:city_plano "--tls.guess true --tls.guess.joining true"
row tlsUncontrolledWithin osm:city_plano "--tls.uncontrolled-within true"
row rampsAccelLanes osm:city_moscow_mkad_ramps "--ramps.guess true --ramps.guess-acceleration-lanes true"
row rampsLength osm:city_moscow_mkad_ramps "--ramps.guess true --ramps.ramp-length 150"
row rampsNoSplit osm:city_paris_chapelle_ramps "--ramps.guess true --ramps.no-split true"
row keepComponents osm:city_cairo_priority "--keep-edges.components 1"
row keepMinSpeed osm:city_plano "--keep-edges.min-speed 10"
row edgesJoinTramDist osm:city_karlsruhe_tram "--edges.join-tram-dist 30"
row speedFactorOffsetMin osm:real "--speed.factor 0.8 --speed.offset -2 --speed.minimum 5"
row speedInKmhXml xml:zipper "--speed-in-kmh true"
row defaultSpreadtype osm:real "--default.spreadtype roadCenter"
row defaultSidewalkWidth osm:city_kumbakonam_mixed "--sidewalks.guess true --default.sidewalk-width 2.5"
row defaultRightOfWay osm:real "--default.right-of-way edgePriority"
row numericalIds osm:city_plano "--numerical-ids true"
row opposites_fixLengths xml:lc_opposite "--opposites.guess.fix-lengths true"
row ptstopNoBidi osm:city_karlsruhe_tram "--ptstop-output @D/ptstop.xml --ptstop-output.no-bidi true"
row outputRemovedNodes osm:real "--geometry.remove true --output.removed-nodes true"
row railwayTopologyOutput osm:city_german_level_crossing "--railway.topology.repair true --railway.topology.output @D/topo.xml"
row plainOutputLanes plain "--plain-output-prefix @D/plain/net --plain-output.lanes true"
row tlsCycleTime osm:city_melbourne_guess_signals "--tls.guess-signals true --tls.cycle.time 60"
row roundaboutsGuessFalse osm:city_carmel_roundabouts_guess "--roundabouts.guess false"
row opposites osm:city_cairo_priority "--opposites.guess true"
row sidewalksFromPermissions osm:city_london_lh_sidewalks "--sidewalks.guess true --sidewalks.guess.from-permissions true"
row crossingsSpeedThreshold osm:city_dublin_tram_crossings "--crossings.guess true --crossings.guess.speed-threshold 30"
row lefthandOsm osm:real "--lefthand true"
row rectangularLaneCut osm:city_paris_dense "--rectangular-lane-cut true"
row defaultSpeed osm:city_plano "--default.speed 8.33"
row defaultLanenumber osm:city_plano "--default.lanenumber 2"
row tlsMinorLeftMaxSpeed osm:city_mexico_reforma_tls "--tls.minor-left.max-speed 8"
row tlsLeftGreenTime osm:city_mexico_reforma_tls "--tls.left-green.time 4"
row defaultJunctionsKeepClear osm:real "--default.junctions.keep-clear false"
row simpleProjectionFalse osm:city_plano "--simple-projection false"
# I/O format rows: args:<netconvert args> (paths relative to the repo root) are new small fixtures under cases/vtypes/fmt
row opendriveOutput plain "--opendrive-output @D/net.xodr"
row opendriveInputStraight "args:--opendrive-files tools/oracle/cases/vtypes/fmt/straight.xodr" ""
row opendriveInputJunction "args:--opendrive-files tools/oracle/cases/vtypes/fmt/junction.xodr" ""
row matsimInput "args:--matsim-files tools/oracle/cases/vtypes/fmt/network.xml" ""
row visumInput "args:--visum-file tools/oracle/cases/vtypes/fmt/tiny.net" ""
row plainFromSumoNet "args:--sumo-net-file tools/oracle/cases/departopts/ref_simple.net.xml" "--plain-output-prefix @D/plain/net"
row matsimOutput plain "--matsim-output @D/matsim.xml"
row dlrNavteqOutput plain "--dlr-navteq-output @D/navteq"
row amitranOutput plain "--amitran-output @D/amitran.xml"
row streetSignOutput osm:signal "--street-sign-output @D/signs.xml"
row parkingOutput osm:city_plano_mall_parking "--parking-output @D/parking.xml"
row polygonOutput "args:--opendrive-files tools/oracle/cases/vtypes/fmt/junction.xodr" "--polygon-output @D/poly.xml"
row outputStreetNamesOff osm:city_plano "--output.street-names false"

# run_row NAME INPUT OPTIONS
run_row() {
  local v=$1 in=$2 opts=$3
  local d=$OUT/$v
  rm -rf "$d"; mkdir -p "$d/cpp_out/plain" "$d/rust_out/plain"
  local src
  case $in in
    osm:*) src="--simple-projection true --osm-files $C/${in#osm:}/input.osm" ;;
    xml:*) src=$(tr -d '\r' < "$C/${in#xml:}/net.args") ;;
    plain) src=$PLAIN ;;
    args:*) src=${in#args:} ;;
  esac
  local co=${opts//@D/$d\/cpp_out} ro=${opts//@D/$d\/rust_out}
  local -a sa ca ra
  read -ra sa <<< "$src"; read -ra ca <<< "$co"; read -ra ra <<< "$ro"
  timeout 900 "$RUST" "${sa[@]}" "${ra[@]}" -o "$d/rust.net.xml" > "$d/rust.log" 2>&1
  local rrc=$?
  timeout 900 "$CPP" "${sa[@]}" "${ca[@]}" -o "$d/cpp.net.xml" > "$d/cpp.log" 2>&1
  local crc=$?
  local V D=""
  if grep -q panicked "$d/rust.log"; then
    V=PANIC; D=$(grep -m1 -A1 panicked "$d/rust.log" | tr '\n' ' ')
  elif [ $rrc -ne $crc ]; then
    V=FAIL; D="exit rust=$rrc cpp=$crc: $(grep -m1 -i error "$d/rust.log")"
  elif [ $crc -ne 0 ]; then
    V=PASS; D="both exit $crc: $(grep -m1 -i error "$d/cpp.log")"
  elif [ -s "$d/rust.net.xml" ] && [ -s "$d/cpp.net.xml" ]; then
    python3 "$B/compare_net.py" "$d/cpp.net.xml" "$d/rust.net.xml" > "$d/net.log" 2>&1
    local cr=$?
    if [ $cr -eq 0 ]; then V=PASS; D="net identical"
    elif [ $cr -eq 2 ]; then V=FAIL; D="net unparseable: $(head -1 "$d/net.log")"
    else V=FAIL; D="$(head -1 "$d/net.log") | $(grep SUMMARY "$d/net.log")"; fi
    if [ "$V" = PASS ]; then
      # the -o net path is echoed into some extra files (netconvert configuration): cpp/rust names differ, normalize them
      local norm='s#'"$d"'/cpp_out#OUT#g; s#'"$d"'/rust_out#OUT#g; s#'"$d"'/cpp.net.xml#NET#g; s#'"$d"'/rust.net.xml#NET#g; /generated on/d; s/\r$//'
      local lc lr
      lc=$(cd "$d/cpp_out" && find . -type f | sort); lr=$(cd "$d/rust_out" && find . -type f | sort)
      if [ "$lc" != "$lr" ]; then V=FAIL; D="output files differ: cpp=[$(echo $lc)] rust=[$(echo $lr)]"
      else
        for f in $lc; do
          # OpenDRIVE writes the wall-clock time into <header date="..."> with second precision;
          # cpp and rust are run seconds apart, so normalize it like the 'generated on' stamp
          local fnorm=$norm
          case "$f" in
            *.xodr) fnorm="$fnorm; s/date=\"[^\"]*\"/date=\"TIME\"/g" ;;
          esac
          if ! cmp -s <(sed -e "$fnorm" "$d/cpp_out/$f") <(sed -e "$fnorm" "$d/rust_out/$f"); then
            V=FAIL; D="$f differs: $(diff <(sed -e "$fnorm" "$d/cpp_out/$f") <(sed -e "$fnorm" "$d/rust_out/$f") | head -1)"; break
          fi
        done
      fi
    fi
  else
    V=FAIL; D="exit 0/0 but net missing (rust=$(test -s "$d/rust.net.xml" && echo yes || echo no) cpp=$(test -s "$d/cpp.net.xml" && echo yes || echo no))"
  fi
  printf '%s\t%s\t%s\n' "$v" "$V" "$D" >> "$OUT/summary.tsv"
  echo "$v $V $D"
}

mkdir -p "$OUT"; : > "$OUT/summary.tsv"
WANT=("$@")
for i in "${!NAMES[@]}"; do
  n=${NAMES[$i]}
  if [ ${#WANT[@]} -gt 0 ]; then
    case " ${WANT[*]} " in *" $n "*) ;; *) continue ;; esac
  fi
  while [ "$(jobs -rp | wc -l)" -ge "${NETOPTS2_JOBS:-4}" ]; do wait -n; done
  run_row "$n" "${INS[$i]}" "${OPTS[$i]}" &
done
wait
echo "NETOPTS_NEW TOTAL PASS=$(grep -c $'\tPASS\t' "$OUT/summary.tsv") FAIL=$(grep -c $'\tFAIL\t' "$OUT/summary.tsv") PANIC=$(grep -c $'\tPANIC\t' "$OUT/summary.tsv")"
