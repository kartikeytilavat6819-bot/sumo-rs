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

## Closure without a routing device (2026-11-16, op03)

Measured with `routes.rou` minus the two `device.rerouting` params, plus `closing.add.xml`, on the instrumented C++
(`SUMO_TRACE_RR=1`) and on branch `worktree-agent-op03-closure` (`5967760`, the commit that makes the rerouter receive its intervals).

| | C++ 1.28 | ours (`5967760`) |
|---|---|---|
| closure reroute, `v0` | t=0, nProh=1 `[11]`, newEdge=13 ? route `10 12 13` | t=0, nProh=1 `[11]`, newEdge=13 ? **panic** `ms_net.rs:3137` (`MSNet::get_router_tt`: router not creatable) |
| `v1`, `v2` | t=5, t=10, same prohibition, route `10 12 13` | not reached |
| `v0` enters `11`? | no: `10_0` t=0..7, `:2_0_0` t=8, `12_0` t=9..21, `13_0` t=22..29, arrival 30.00 | no FCD step written; the process stops during insertion at t=0 |
| held anywhere? | no; nothing waits, the vehicle is rerouted at departure | n/a |

Findings:
- The nProh step and edge list match exactly (t=0, `[11]`).
- C++ never "holds" a vehicle for this closure. `closingReroute` without `allow`/`disallow` has permissions
  `SVC_UNSPECIFIED`: `setPermissions` leaves the lanes alone, and the closure exists only as a router prohibition at
  reroute time. If that reroute failed, C++ would let the vehicle drive through (see the comment at the
  `Cannot keep destination edge` warning). So this configuration has no closed-edge enforcement apart from the router.
- C++ without a device produces exactly the committed reference (`v0` arrival 30.00, same lanes). With the device the
  C++ closure branch is never reached: the t=0 pre-insertion device reroute already picks `12`, which is cheaper.
- Conclusion: one remaining defect for this case, the router (`MSNet::get_router_tt` ? `MSBaseVehicle::reroute` ?
  `SUMOAbstractRouter::compute_2/compute`). Closed-edge enforcement by lane permission is not exercised here; a
  `closingReroute` with `disallow="all"` would be needed to test that.
