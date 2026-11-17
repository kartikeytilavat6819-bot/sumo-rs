# netconvert options users add: C++ vs ours

Runner: `tools/oracle/run_netopts.sh [net ...]`. Each OSM run uses `--simple-projection true` plus the option.
Nets: `real`, `city_plano`, `city_munich` (`cases/*/input.osm`); re-import is `-s ref_simple.net.xml` for each case.
Binaries: C++ netconvert 1.28.0 (WSL) vs ours `sumo-rs-target-x07\debug\netconvert.exe`.
Compare: `compare_net.py` (C++ = reference). Logs and nets go to `%TEMP%\opencode\netopts\`, not the repo.

Totals (run 1, main 1820f92c, 69 runs): **48 PASS, 21 FAIL, 0 PANIC**.

## Run 2 (nx02): 5 OSM nets x 37 variants + re-import

Nets: `real`, `city_plano`, `city_munich`, `city_berlin_priority`, `city_singapore_lefthand` (the last two are the newest OSM cases).
Totals: **187 PASS, 1 FAIL, 0 PANIC** of 188 runs (real 37/37, city_plano 37/37, city_munich 36/37, city_berlin_priority 37/37, city_singapore_lefthand 37/37, re-import 3/3).
Runner: `run_netopts.sh` (portable: `CPP_NETCONVERT`, `CPP_SUMO_HOME`, `NETOPTS_OUT`, `NETOPTS_JOBS`). `--proj.utm` runs without `--simple-projection` (the two conflict).

New options added: `geometry.min-dist`, `junctions.join-dist`, `tls.guess` (+ `tls.guess.threshold`), `tls.layout` (`incoming`, `alternateOneWay`), `tls.default-type` (`actuated`, `delay_based`), `no-turnarounds`, `junctions.corner-detail 0`, `junctions.internal-link-detail`, `bikelanes.guess`, `osm.all-attributes`, `offset.disable-normalization`, `proj.utm`.
Already covered in run 1: `geometry.remove`, `ramps.guess`, `roundabouts.guess`, `junctions.join`, `tls.join`, `edges.join`, `remove-edges.isolated`, `keep-edges.by-vclass`, `sidewalks.guess`, `crossings.guess`, `osm.turn-lanes`, `no-internal-links`, `default.lanewidth`.

| variant | net | verdict | first diff |
|---|---|---|---|
| `junctionsJoinDist` (`--junctions.join true --junctions.join-dist 20`) | city_munich | FAIL | lane `:1689059001_0_0` length ref=15.11 ours=10.21 (341 groups) |
| every other variant | all 5 nets | PASS | net identical |

Finding on the FAIL (measured): with join-dist 20 both sides give the same warnings except for the intersecting-left-turn pair and one cycle-time message. Both nets have 740 vs 741 junctions: ours adds `:cluster_12861231068_12861231069_14264810438_14264810439_#29more_60_0`. Joined node `…#29more_48_0` in ours is missing incoming lane `125226164#0_0`, and its internal node positions differ, so the member set of this cluster differs. Not yet located.
Fixed on the way: C++ `NodeSet` is `std::set<NBNode*, ComparatorIdLess>` (iterates by id), but the Rust `NodeSet` iterates by address (`Ptr::cmp`). The cluster loops in `nb_node_cont.rs` now iterate by id (`nodes_by_id`). This is faithful and does not change any PASS, but it did not fix the FAIL above.
Note: `city_munich` base now PASSes on main; the zipper junction FAIL in run 1 is gone.

Verdicts: PASS = same exit code and identical net; FAIL = exit or net differs; PANIC = our binary panicked.

Caveats: a 7-byte gap in every net is header-only (generation timestamp, input path, output path); `compare_net.py` ignores it.
`compare_net.py` checks attributes of junction, node, edge, lane, connection, tlLogic, phase, crossing and walkingArea only.
It does not check roundabout elements or junction children (`request`, `neigh`).

| option | net | verdict | first diff / panic |
|---|---|---|---|
| (none, control) | real | PASS | net identical |
| (none, control) | city_plano | PASS | net identical |
| (none, control) | city_munich | FAIL | junction '1413262637' type: ref='zipper' cand='priority' (12 diffs) |
| `--geometry.remove true` | real | PASS | net identical |
| `--geometry.remove true` | city_plano | PASS | net identical |
| `--geometry.remove true` | city_munich | FAIL | junction '1413262637' type: ref='zipper' cand='priority' (12 diffs) |
| `--ramps.guess true` | real | PASS | net identical |
| `--ramps.guess true` | city_plano | PASS | net identical |
| `--ramps.guess true` | city_munich | FAIL | junction '1413262637' type: ref='zipper' cand='priority' (12 diffs) |
| `--roundabouts.guess true` | real | PASS | net identical |
| `--roundabouts.guess true` | city_plano | PASS | net identical |
| `--roundabouts.guess true` | city_munich | FAIL | junction '1413262637' type: ref='zipper' cand='priority' (12 diffs) |
| `--junctions.join true` | real | PASS | net identical |
| `--junctions.join true` | city_plano | PASS | net identical |
| `--junctions.join true` | city_munich | FAIL | junction '1413262637' type: ref='zipper' cand='priority' (12 diffs) |
| `--tls.guess-signals true` | real | PASS | net identical |
| `--tls.guess-signals true` | city_plano | PASS | net identical |
| `--tls.guess-signals true` | city_munich | FAIL | junction '1413262637' type: ref='zipper' cand='priority' (12 diffs) |
| `--tls.join true` | real | PASS | net identical |
| `--tls.join true` | city_plano | PASS | net identical |
| `--tls.join true` | city_munich | FAIL | junction '1413262637' type: ref='zipper' cand='priority' (12 diffs) |
| `--tls.discard-simple true` | real | PASS | net identical |
| `--tls.discard-simple true` | city_plano | PASS | net identical |
| `--tls.discard-simple true` | city_munich | FAIL | junction '1413262637' type: ref='zipper' cand='priority' (12 diffs) |
| `--edges.join true` | real | PASS | net identical |
| `--edges.join true` | city_plano | PASS | net identical |
| `--edges.join true` | city_munich | FAIL | junction '1413262637' type: ref='zipper' cand='priority' (12 diffs) |
| `--remove-edges.isolated true` | real | PASS | net identical |
| `--remove-edges.isolated true` | city_plano | PASS | net identical |
| `--remove-edges.isolated true` | city_munich | FAIL | junction '1413262637' type: ref='zipper' cand='priority' (12 diffs) |
| `--keep-edges.by-vclass passenger` | real | PASS | net identical |
| `--keep-edges.by-vclass passenger` | city_plano | PASS | net identical |
| `--keep-edges.by-vclass passenger` | city_munich | PASS | net identical (3304 elements, both sides) |
| `--remove-edges.by-type highway.footway,highway.cycleway` | real | PASS | net identical |
| `--remove-edges.by-type highway.footway,highway.cycleway` | city_plano | PASS | net identical |
| `--remove-edges.by-type highway.footway,highway.cycleway` | city_munich | FAIL | junction '1413262637' type: ref='zipper' cand='priority' (12 diffs) |
| `--sidewalks.guess true` | real | PASS | net identical |
| `--sidewalks.guess true` | city_plano | PASS | net identical |
| `--sidewalks.guess true` | city_munich | PASS | net identical |
| `--crossings.guess true` | real | PASS | net identical |
| `--crossings.guess true` | city_plano | PASS | net identical |
| `--crossings.guess true` | city_munich | FAIL | junction '1413262637' type: ref='zipper' cand='priority' (12 diffs) |
| `--osm.turn-lanes true` | real | PASS | net identical |
| `--osm.turn-lanes true` | city_plano | PASS | net identical |
| `--osm.turn-lanes true` | city_munich | FAIL | junction '1413262637' type: ref='zipper' cand='priority' (12 diffs) |
| `--osm.lane-access true` | real | PASS | net identical |
| `--osm.lane-access true` | city_plano | PASS | net identical |
| `--osm.lane-access true` | city_munich | FAIL | junction '1413262637' type: ref='zipper' cand='priority' (12 diffs) |
| `--osm.elevation true` | real | PASS | net identical |
| `--osm.elevation true` | city_plano | PASS | net identical |
| `--osm.elevation true` | city_munich | FAIL | junction '1413262637' type: ref='zipper' cand='priority' (12 diffs) |
| `--no-internal-links true` | real | PASS | net identical |
| `--no-internal-links true` | city_plano | PASS | net identical |
| `--no-internal-links true` | city_munich | FAIL | junction '1413262637' type: ref='zipper' cand='priority' (12 diffs) |
| `--default.lanewidth 3.2` | real | PASS | net identical |
| `--default.lanewidth 3.2` | city_plano | PASS | net identical |
| `--default.lanewidth 3.2` | city_munich | FAIL | junction '1413262637' type: ref='zipper' cand='priority' (12 diffs) |
| `--output.street-names true` | real | PASS | net identical |
| `--output.street-names true` | city_plano | PASS | net identical |
| `--output.street-names true` | city_munich | FAIL | junction '1413262637' type: ref='zipper' cand='priority' (12 diffs) |
| `--output.original-names true` | real | PASS | net identical |
| `--output.original-names true` | city_plano | PASS | net identical |
| `--output.original-names true` | city_munich | FAIL | junction '1413262637' type: ref='zipper' cand='priority' (12 diffs) |
| bundle: `--geometry.remove --ramps.guess --junctions.join --tls.guess-signals --tls.discard-simple --tls.join --tls.default-type actuated` | real | PASS | net identical |
| bundle (as above) | city_plano | PASS | net identical |
| bundle (as above) | city_munich | FAIL | junction '1413262637' type: ref='zipper' cand='priority' (12 diffs) |
| re-import `-s ref_simple.net.xml` | real | PASS | net identical |
| re-import `-s ref_simple.net.xml` | city_plano | PASS | net identical |
| re-import `-s ref_simple.net.xml` | city_munich | FAIL | connection '127751701#1->127751701#2 0->0' state: ref='m' cand='M' (3 diffs) |

## Findings

1. **city_munich fails on every option except `--keep-edges.by-vclass` and `--sidewalks.guess`, including the control.**
   The first diff is the same in all 20 failing rows: junction `1413262637` has type `zipper` in C++ and `priority` in ours.
   Two more junctions (`1689007011`, `1689059009`) differ the same way.
   Nine connection link states also differ: C++ `Z` or `m`, ours `M`.
   This is present with no option at all, so it is a base-import defect, not an option defect.
   Likely area: zipper junction typing and link-state assignment. Not confirmed; no code was inspected.
2. **Re-import of the C++ `city_munich` net reproduces the `m`/`M` link-state mismatch** (3 connections: `127751701#1`, `127751703#3`, `156672300#1`). Re-import itself is clean on `real` and `city_plano`.
3. **`real` and `city_plano` match C++ on all 22 option variants and the re-import.** No panics or refusals on any option, including the bundle.
4. No PANIC rows. Every option name is accepted by both binaries (checked with `--help`).

## Open items

- Ours warns `Discarding unknown compound 'psv'` and `Discarding unusable type 'railway.platform'` on `city_munich`. The C++ log was not compared line by line, so the warning text is unverified.
- The `--keep-edges.by-vclass` and `--sidewalks.guess` PASS on `city_munich` is recorded as identical. Its element count is not compared with a non-option baseline.
