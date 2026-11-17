#!/bin/bash
# td01 (temporary, removed before merge): container-only straight run vs save@60 + load, C++ oracle and ours.
# SUMO_HOME unset: ours never validates XML; C++ with SUMO_HOME set rejects its own state file (speedFactor on container).
unset SUMO_HOME
W=/tmp/td01_cont; rm -rf "$W"; mkdir -p "$W"; cd "$W" || exit 1
C=${SUMO_PRISTINE_BIN:-$HOME/sumo-pristine/bin}
OURS=/tmp/target/debug/sumo
"$C/netgenerate" --grid --grid.number 3 --grid.length 200 -o net.xml >ng.log 2>&1
cat > routes.xml <<'EOF'
<routes>
    <vType id="truck" vClass="truck" personCapacity="4" containerCapacity="4"/>
    <route id="r0" edges="A0B0 B0C0"/>
    <vehicle id="v0" type="truck" route="r0" depart="0">
        <stop edge="A0B0" pos="100" duration="30"/>
    </vehicle>
    <container id="c0" depart="0">
        <transport from="A0B0" to="B0C0" lines="v0"/>
        <tranship edges="B0C0" speed="1"/>
    </container>
</routes>
EOF
run_side() {  # $1=label $2=sumo binary
    RUST_BACKTRACE=1 "$2" -n net.xml -r routes.xml --no-step-log --end 200 --save-state.times 60 --save-state.prefix "$1_st" --save-state.suffix .xml --save-state.transportables true --tripinfo-output "$1_straight_ti.xml" --personinfo-output "$1_straight_pi.xml" >"$1_straight.log" 2>&1
    echo "$1 straight exit $?"
    grep -E "Error|panicked|sumo_core::" "$1_straight.log" | head -n 6
    STATE=$(ls "$1"_st_*.xml 2>/dev/null | head -1)
    echo "$1 state file: ${STATE:-none}"
    [ -n "$STATE" ] || return
    "$2" -n net.xml --load-state "$STATE" --no-step-log --end 200 --tripinfo-output "$1_load_ti.xml" >"$1_load.log" 2>&1
    echo "$1 load exit $?"
    grep -E "Error|panicked|sumo_core::" "$1_load.log" | head -n 6
}
run_side cpp "$C/sumo"
run_side ours "$OURS"
verdict() {
    if [ ! -f "$2" ]; then echo "MISSING $2"; return; fi
    if diff <(grep -v -E '<!--|<snapshot' "$1") <(grep -v -E '<!--|<snapshot' "$2") >/dev/null; then echo "SAME $1 $2"; else echo "DIFF $1 $2"; diff <(grep -v -E '<!--|<snapshot' "$1") <(grep -v -E '<!--|<snapshot' "$2") | head -n 12; fi
}
verdict cpp_straight_ti.xml ours_straight_ti.xml
verdict cpp_load_ti.xml ours_load_ti.xml
verdict cpp_st_60.00.xml ours_st_60.00.xml
