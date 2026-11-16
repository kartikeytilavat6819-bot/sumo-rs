# Reroute with alternative — verdict (2026-11-16)

| Stage | Verdict | Detail |
|---|---|---|
| net | PASS | exact |
| sim (`ref`) | **FAIL** | tripinfo: `v0` arrival ref 30.00 vs ours 31.00 |

**First difference:** `t=8.00 vehicle v0 field y: ref=64.37 ours=68.99`.

Cause (from FCD lane sequences):
- C++: `v0` leaves `10_0`, takes junction internal `:2_0_0` (onto `12`) at t=8, then `12_0` → `13_0`. The closed
  edge `11` is avoided.
- Ours: `v0` takes `:2_1_0` (onto `11`, the closed edge) at t=8, then `11_0` → `:4_1_0` → `13_0`.

So the device-driven detour does not happen in our run: the closing rerouter's closure of `11` does not
change the route the vehicle follows, and vehicles still drive the closed edge. `v1` and `v2` show the same
pattern, with `v2` also diverging later. The 1 s arrival gap and the y-offset follow from the different edge.

Also in the same run: `reroute/ref_device` fails for a different reason (devices attribute, see its REPORT),
not related to this case.
