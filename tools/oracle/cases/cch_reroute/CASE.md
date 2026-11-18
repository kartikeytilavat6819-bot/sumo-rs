# cch_reroute

CCH routing in the simulation: every vehicle reroutes once at insertion
(`--device.rerouting.probability 1`) using `--routing-algorithm CCH`. A timed
edge closure (`closing.add.xml`) makes the reroute choose an alternative path,
so the CCH metric/paths are exercised (MSRoutingEngine's live CCH family via
`getPublishedCCHMetric`, and MSNet's free-flow CCH router via
`getFreeflowCCHMetric`). Inputs are copied from `reroute_alt`; references come
from the official SUMO 1.28.0 via `make_sim_refs.sh`.
