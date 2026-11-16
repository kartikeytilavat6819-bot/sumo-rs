# Router provider: handover (branch `worktree-agent-rr02`, WIP commit `b44e6ae`)

Status: **does not compile, nothing has been run.** Nothing below is verified.

## (a) What the code now does

- `DijkstraRouter::with_compute`, `AStarRouter::with_compute`, `RailwayRouter::with_compute`
  install the concrete `compute` into the base router's new slot
  `SUMOAbstractRouter::myCompute` (`utils/router/sumo_abstract_router.rs`). The base `compute`
  dispatches to that slot; it panics only when no slot is installed. This stands in for C++ virtual
  dispatch. `DijkstraRouter::clone` copies the slot.
- `MSNet::get_router_tt` (`microsim/ms_net.rs`) follows C++ `getRouterTT`: uses
  `MSGlobals::num_sim_threads()` to pick rng 0, builds one router per rng index and caches it in
  `myRouterTT`. `dijkstra` gets `DijkstraRouter(getAllEdges, true, MSNet::get_travel_time, None,
  false, null, true, false)`. `astar` and anything else (with a warning) gets `AStarRouter(..., None,
  true, false)`. `CCH` panics with a shortcut. It then calls `prohibit(prohibited)` on the router.
- `MSRoutingEngine::init_router` (`microsim/devices/ms_routing_engine.rs`) builds the Dijkstra or
  A* router with `myEffortFunc` (`get_effort` or `get_effort_extra`), an optional railway router
  when the net has bidi edges, and a `RouterProvider` with null pedestrian and intermodal routers.
  It stores the provider in a `thread_local` `ROUTER_PROVIDER`. `cleanup()` clears it.
- `MSRoutingEngine::get_router_tt` builds the provider on first use (init weight update, init edge
  weights, init router) and prohibits on `get_vehicle_router(svc)`.
- The base prohibition helpers (`e_numerical_id`, `e_prohibits`, `e_restricts`, `e_length`,
  `e_id`, `e_is_internal`, `e_via_successors`, `v_vclass`) were panic stubs. They are now real
  bodies that call the router traits. Before this, any non-empty prohibition, and any router with
  `havePermissions`, panicked. Note that `prohibit` keys are `Ptr<()>` and are cast with
  `cast::<E>()`.
- A unit test `detours_around_a_prohibited_edge_through_the_base_router` in `dijkstra_router.rs`
  uses a diamond `in -> {top(10), bot(20)} -> out`. Expected: `[in, top, out]` unprohibited, and
  `[in, bot, out]` after prohibiting `top`. **It has never run.**

## (b) The 41-error cause

The base impl `impl<E, V> SUMOAbstractRouter<E, V>` was bounded to
`impl<E: RouterEdge<V>, V: RouterVehicle>` so the helpers could call trait methods. Every caller
that uses the base type with an `E` or `V` that does not satisfy both bounds breaks. The errors
are in:

- `a_star_lookup_table.rs` (`recompute_costs`, `compute`, `drop_` on landmark routers)
- `af_build.rs` (`drop_` on `Node2EdgeRouter<FlippedEdge, FlippedNode, V, M>`)
- `cch_router.rs` (fallback `reset`, `prohibit`, `set_bulk_mode`, `set_msg_handler`)
- `ch_builder.rs` (`get_effort` on the effort provider)
- `dijkstra_router.rs` (`SUMOAbstractRouter::new` in `clone`, `new_2`; `set_auto_bulk_mode`)
- `intermodal_router.rs`, `pedestrian_router.rs` (base instantiated over `IntermodalEdge` and
  `IntermodalTrip<E, N, V>`, which does not implement `RouterVehicle` for every `E`)
- `railway_router.rs` (base over `RailEdge<E, V>`)
- `router_provider.rs` (`clone`, `set_bulk_mode` over generic `E, V`)

The cause is scope, not missing logic. The base is instantiated for five edge/vehicle pairs
(`MSEdge`/`SUMOVehicle`, `ROEdge`, `IntermodalEdge`/`IntermodalTrip`, `RailEdge`, `FlippedEdge`),
and the bounds either cover all of them or none. Bounding the impl blanket-covers all of them and
breaks the ones that do not satisfy the bounds.

## (c) What was tried

1. **Trait-import fix.** The first build had 10 errors because `RouterEdge` and `RouterVehicle` were
   not imported in `sumo_abstract_router.rs`. Fixed by adding the import.
2. **Bound the whole base impl (what is committed).** Result: 41 errors, listed above. This was
   the only bound strategy tried. The attempt that was intended next, narrowing bounds per method,
   was not made.

Strategies not tried, and why they might work:

- **Per-method `where` clauses** on only the methods that call the helpers (`prohibit`,
  `is_prohibited`, `build_path_from`, the via-successor walk). Concern: `compute_looped`, `compute_2`
  and others call these, so the bound may still propagate upward.
- **Helpers that take no trait bound**, for example storing the numerical id and length in
  `SUMOAbstractRouter_EdgeInfo` at construction, where the bounded constructors can fill them in.
  This removes the need for the bound on the base. Concern: prohibition keys are `Ptr<()>`, so the
  id lookup still needs a stored id per edge.
- **Concrete-type base for the MSNet provider** only, leaving the generic base unbounded. Concern:
  this is a fork of the generic type.

## Remaining blockers for the two rerouting cases

- `reroute_alt`: needs the provider to build and the Dijkstra route to be `10 -> 12 -> 13` at t=0,
  arriving at 30.00.
- `closed_edge`: needs the provider and the parked closure-side fix on
  `worktree-agent-op03-closure` (`5967760`), which is not merged.
- Intermodal routing (`get_intermodal_router_tt`) and `reroute(vehicle)` in `MSRoutingEngine` are
  still stubs. The provider has no pedestrian or intermodal router.
