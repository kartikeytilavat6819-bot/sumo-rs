# Junction types — right-of-way cases

Ten cases, one per junction type (`cases/jt_*/`). Each is a 4-arm junction (3 arms for the T) with
20 vehicles in perpendicular pairs departing 0–54 s, so the rule decides who goes. Net: `compare_net.py`
against C++ netconvert. Sim: tripinfo, FCD and statistics against C++ sumo 1.28.0, run with run_all's flags.
References: `tools/oracle/make_jt_refs.sh` (WSL). Ours: `C:\Users\karti\sumo-rs-target-x07\debug`, not rebuilt.

| case | net | tripinfo | FCD | stat | first diff |
|---|---|---|---|---|---|
| jt_priority | PASS | MATCH | MATCH | MATCH | — |
| jt_priority_stop | PASS | MATCH | MATCH | MATCH | — |
| jt_right_before_left | PASS | MATCH | MATCH | MATCH | — |
| jt_left_before_right | PASS | **FAIL** | **FAIL** | **FAIL** | `v19` arrival ref 89.00 vs ours 110.00 |
| jt_allway_stop | PASS | MATCH | MATCH | MATCH | — |
| jt_unregulated | PASS | MATCH | MATCH | MATCH | — |
| jt_traffic_light_right_on_red | PASS | MATCH | MATCH | MATCH | — |
| jt_traffic_light_unregulated | PASS | MATCH | MATCH | MATCH | — |
| jt_t_priority | PASS | MATCH | MATCH | MATCH | — |
| jt_right_before_left_lefthand | PASS | MATCH | MATCH | MATCH | — |

**Totals:** net 10/10 PASS; sim 9/10 MATCH. The one divergence is `left_before_right`. Waiting differs
from the first conflicting departures: C++ `v18` waits 3 s, ours 0 s; C++ `v19` waits 2 s, ours 18 s.

Caveat: our binary may predate this worktree's sources. It was built 10:39 on Nov 16. The checkout is
newer. Not rebuilt, since writers do not run cargo; re-run after a build before trusting the one FAIL.
