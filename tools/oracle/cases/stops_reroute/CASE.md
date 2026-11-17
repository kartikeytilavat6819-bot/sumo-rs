# stops_reroute

Stops plus rerouting. Network: the `reroute_bypass` bypass net (`LEAD IN AB OUT`, detour `IN AC CB OUT`).
A rerouter on `IN` closes `AB` from t=2 s, so vehicles already on the route must be detoured.

- `v0`: stop on `IN` before the closed edge, then two stops on `OUT` (same edge, after the closed edge).
- `v1`: one later stop on `OUT`.

Checks that stop edge indices are re-mapped onto the new route (MSStop::edge, replaceRoute re-check).
Reference: C++ sumo 1.28.0 from ~/sumo-pristine (`sim_ref.sh`, `net_ref.sh`).
