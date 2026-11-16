#!/bin/bash
# td01 (temporary, removed before merge): person ride straight run vs save@60 + load, C++ oracle and ours.
W=/tmp/td01_rt
rm -rf "$W"; mkdir -p "$W"; cd "$W" || exit 1
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
    <person id="p0" depart="0">
        <walk edges="A0B0"/>
        <ride from="A0B0" to="B0C0" lines="v0"/>
        <walk edges="B0C0"/>
    </person>
    <person id="p1" depart="10">
        <walk edges="A0B0"/>
        <ride from="A0B0" to="B0C0" lines="v0"/>
    </person>
</routes>
EOF
run_side() {  # $1=label $2=sumo binary
    RUST_BACKTRACE=1 "$2" -n net.xml -r routes.xml --no-step-log --end 200 --save-state.times 60 --save-state.prefix "$1_st" --save-state.suffix .xml --save-state.transportables true --tripinfo-output "$1_straight_ti.xml" --personinfo-output "$1_straight_pi.xml" >"$1_straight.log" 2>&1
    echo "$1 straight exit $?"
    tail -n 4 "$1_straight.log"
    grep -E "sumo_core::|at crates" "$1_straight.log" | head -n 14
    STATE=$(ls "$1"_st_*.xml 2>/dev/null | head -1)
    echo "$1 state file: ${STATE:-none}"
    [ -n "$STATE" ] || return
    "$2" -n net.xml --load-state "$STATE" --no-step-log --end 200 --tripinfo-output "$1_load_ti.xml" --personinfo-output "$1_load_pi.xml" >"$1_load.log" 2>&1
    echo "$1 load exit $?"
    tail -n 3 "$1_load.log"
    grep -q -E 'Error|Quitting' "$1_load.log" && { echo "$1 state lines 50-56:"; sed -n 50,56p "$STATE" | cut -c1-300; }
}
run_side cpp "$C/sumo"
run_side ours "$OURS"
verdict() {
    if [ ! -f "$2" ]; then echo "MISSING $2"; return; fi
    if diff <(grep -v -E '<!--|value="' "$1") <(grep -v -E '<!--|value="' "$2") >/dev/null; then echo "SAME $1 $2"; else echo "DIFF $1 $2"; diff <(grep -v -E '<!--|value="' "$1") <(grep -v -E '<!--|value="' "$2") | head -n 12; fi
}
for k in ti pi; do
    verdict cpp_straight_$k.xml cpp_load_$k.xml
    verdict ours_straight_$k.xml ours_load_$k.xml
    verdict cpp_straight_$k.xml ours_straight_$k.xml
done
