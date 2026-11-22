#!/bin/bash
# oc93 probe 5: count calls to the functions on the stuck stack. Whichever
# counter explodes identifies the loop; no source changes needed.
set -u
cd "$(dirname "$0")/../.." || exit 1
OUR="$CARGO_TARGET_DIR/debug/sumo"
OUT=/tmp/oc93
mkdir -p "$OUT"

cat > "$OUT/cmds.gdb" <<'EOF'
set pagination off
set confirm off
set $n_pro = 0
set $n_grt = 0
set $n_not = 0
set $n_try = 0
set $n_tri = 0
break sumo_abstract_router.rs:851
commands
silent
set $n_pro = $n_pro + 1
continue
end
break ms_routing_engine.rs:845
commands
silent
set $n_grt = $n_grt + 1
continue
end
break ms_device_routing.rs:390
commands
silent
set $n_not = $n_not + 1
continue
end
break ms_insertion_control.rs:848
commands
silent
set $n_try = $n_try + 1
continue
end
break ms_base_vehicle.rs:3048
commands
silent
set $n_tri = $n_tri + 1
continue
end
run
printf "\n==== COUNTS ====\nprohibit=%d get_router_tt=%d notify_enter=%d try_insert=%d activate_reminders=%d\n", $n_pro, $n_grt, $n_not, $n_try, $n_tri
bt 20
kill
EOF

dir="tools/oracle/cases/tt_rail_siding_loop_3veh"
args=$(cat "$dir/sim.args" | tr -d '\r')
timeout -s INT 22 gdb -q -batch -x "$OUT/cmds.gdb" \
    --args "$OUR" -n "$dir/ref_simple.net.xml" -r "$dir/routes.rou" \
      --fcd-output "$OUT/cnt.fcd.xml" --no-step-log true $args \
    > "$OUT/counts.txt" 2>&1
echo "==== gdb breakpoints / errors ===="
grep -aE 'Breakpoint [0-9]|No line|not defined|Cannot' "$OUT/counts.txt" | head -20
echo "==== counts + backtrace ===="
sed -n '/==== COUNTS ====/,$p' "$OUT/counts.txt" | head -30
echo "last fcd: $(grep -a '<timestep' "$OUT/cnt.fcd.xml" 2>/dev/null | tail -1 | grep -ao 'time="[0-9.]*"' | tail -1)"
