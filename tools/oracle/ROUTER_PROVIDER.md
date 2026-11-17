# Router provider: status (branch `worktree-agent-op04`)

Status: **compiles, provider builds and caches, prohibitions honoured, measured.** Supersedes the rr02 handover.

## Bounds

main already bounds `SUMOAbstractRouter` per method (`309edd8`); this branch merges main and keeps only the
compute dispatch slot (`myCompute`) on top. `MSNet::get_router_tt` builds one `DijkstraRouter` per rng index
(cached in `myRouterTT`), `MSRoutingEngine::init_router` builds the provider. `get_router_tt(.., {})` resets
earlier prohibitions, as in C++.

## Verification

- Unit: `dijkstra_router::tests::detours_around_a_prohibited_edge_through_the_base_router` asserts
  `[in, top, out]`, then `[in, bot, out]` with `top` prohibited, then `[in, top, out]` after reset. (The rr02
  version could never pass: its test edges had no via-successors.)
- run_all on this branch: PASS=22 FAIL=3 SKIP=8 (unchanged), depart 15/15. reroute_alt/closed_edge still FAIL
  here because the rerouter never fires on main (no move-reminder vtable, no child elements).

## What the two FAIL rows need (not in files owned here)

Measured with two scratch patches applied on top of this branch, then reverted:

1. `5967760` (`worktree-agent-op03-closure`): rerouter vtable, child elements, forwards `getClosed()` to the
   router. On its own: router returns `10 12 13` at t=0 (correct), but v0 stops at the end of edge 10
   (first FCD diff t=6).
2. `ms_base_vehicle.rs::replace_route_edges` calls `MSBaseVehicle::replace_route` statically, so
   `MSVehicle::replace_route` (`updateBestLanes(true)`) never runs and the vehicle keeps its old best lanes.
   Scratch fix: call `MSVehicle::replace_route` through `cast::<MSVehicle>()` (needs a real virtual dispatch,
   meso vehicles are not MSVehicle).

With both: reroute_alt FCD+tripinfo MATCH (v0 `10 12 13`, arrival 30.00), closed_edge FCD+tripinfo MATCH
(v0 rerouted onto 12). run_all PASS=23 FAIL=2, but **`reroute/ref_device` newly FAILs**: C++ fails to reroute
v1 (`No route ... found`, rerouteNo=0, arrives 4.00); we reroute it to its current edge (rerouteNo=1, arrives
3.00). The rerouter's `hasReroutingDevice` is a `get_device(&())` stub that reports false, so we take the
MSNet router instead of `MSRoutingEngine::getRouterTT`; that is the first suspect.
