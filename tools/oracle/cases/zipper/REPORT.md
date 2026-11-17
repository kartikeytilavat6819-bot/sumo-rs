# zipper — 2-lane edge into a zipper merge onto 1 lane

## Net

`nS → nZ` (edge `IN`, 2 lanes, 300 m), then `nZ → nE` (edge `OUT`, 1 lane, 200 m). Node `nZ` is
`type="zipper"`. Built from plain XML (`net.args`).

Without the explicit type, C++ netconvert emits no zipper: `IN_0/IN_1 → OUT_0` are `state="M"` (verified with
`netconvert` on the same nodes/edges, before setting the type). So the `type="zipper"` on `nZ` is required, not
optional.

## Case

- 16 vehicles `z0..z15`, route `IN OUT`, depart in pairs at t=0,3,…,21: one on `departLane=0`, one on `departLane=1`.
  Both lanes always start together, so the two streams arrive at the merge at the same time and must alternate.
- `--end 200`, no additional files.

## Proof the zipper is exercised (C++ 1.28.0, `ref_simple.net.xml`, `ref.*`)

| check | result |
|---|---|
| `state="Z"` links | `IN 0 → OUT 0` via `:nZ_0_0`, `IN 1 → OUT 0` via `:nZ_0_1` (both `state="Z"`) |
| internal lanes used in FCD | `:nZ_0_0`, `:nZ_0_1` both occupied |
| merge order (first entry into `OUT_0`, C++) | `z0(IN_0) z1(IN_1) z2(IN_0) z3(IN_1) z4(IN_0) z6(IN_0) z5(IN_1) z7(IN_1) z8(IN_0) z9(IN_1) z10(IN_0) z11(IN_1) z12(IN_0) z13(IN_1) z14(IN_0) z15(IN_1)` |

The merge alternates lanes for 16 vehicles; the only same-lane pairs are `z4,z6` (IN_0) and `z5,z7` (IN_1).
So the zipper does interleave the two lanes.

## Verdict (ours, `sumo.exe` from `sumo-rs-target-x07/debug`)

**FAIL — first diff at FCD t=19.00, `z2` `x`: ref 220.25, ours 220.81.** Tripinfo first diff: `z0` `arrivalSpeed`
ref 14.40, ours 14.36.

Further divergence (not just a float drift):
- Our run logs **`Warning: Vehicle 'z4' performs emergency braking on lane 'IN_0' with decel=9.00, … time=28.00`**
  and **`Teleporting vehicle 'z10'; collision with vehicle 'z9', lane='OUT_0', … time=39.00`** plus a
  "teleports beyond arrival edge" line. The C++ reference log (`ref.log`) is **empty**: no emergency braking, no
  collision, no teleports.
- Merge order from our FCD: `z2` enters `OUT_0` at t=26 (C++ 27); `z10` never arrives (teleported); `z12/z11/z13/z15/z14`
  reorder. The lane alternation breaks after the first collision.

Interpretation: the zipper link itself is reachable (both lanes feed `OUT`), but our car-following or lane-change
logic on the 2-lane approach to the zipper diverges from C++ by t≈19–28 s, before the merge. Next step: start from
`z2`'s x at t=19 on `IN_0`.

## Reproduce

- Net: `wsl -e bash /mnt/c/.../agent-c01/tools/oracle/cases/zipper/net_ref.sh`
- Reference: `wsl -e bash -lc 'bash …/zipper/sim_ref.sh > …/zipper/ref.log 2>&1'`
- Ours: `sumo.exe -n ref_simple.net.xml -r routes.rou --tripinfo-output … --fcd-output … --statistic-output … --no-step-log true --end 200`
- Compare: `python tools/oracle/compare_sim.py --tripinfo ref.tripinfo.xml ours.tripinfo.xml` (and `--fcd`).
