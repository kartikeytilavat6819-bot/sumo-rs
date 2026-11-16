# netconvert options users add: C++ vs ours

Runner: `tools/oracle/run_netopts.sh [net ...]`. Each OSM run uses `--simple-projection true` plus the option.
Nets: `real`, `city_plano`, `city_munich` (`cases/*/input.osm`); re-import is `-s ref_simple.net.xml` for each case.
Binaries: C++ netconvert from `~/sumo-pristine/bin/netconvert` (untouched copy, SUMO_HOME `~/sumo-pristine/lib/python3.12/site-packages/sumo`) vs ours `sumo-rs-target-x07\debug\netconvert.exe`.
Compare: `compare_net.py` (C++ = reference). Logs and nets go to `%TEMP%\opencode\netopts-rv01\` (re-run, Nov 16); the earlier run is in `%TEMP%\opencode\netopts\`.

Totals: **48 PASS, 21 FAIL, 0 PANIC** of 69 runs. Old run (`~/sumo-src` C++): 48 PASS, 21 FAIL, 0 PANIC. No verdict changed.

Verdicts: PASS = same exit code and identical net; FAIL = exit or net differs; PANIC = our binary panicked.

Caveats: a 7-byte gap in every net is header-only (generation timestamp, input path, output path); `compare_net.py` ignores it.
`compare_net.py` checks attributes of junction, node, edge, lane, connection, tlLogic, phase, crossing and walkingArea only.
It does not check roundabout elements or junction children (`request`, `neigh`).

| option | net | verdict | first diff / panic | changed vs old run |
|---|---|---|---|---|
| (none, control) | real | PASS | net identical | no |
| (none, control) | city_plano | PASS | net identical | no |
| (none, control) | city_munich | FAIL | junction '1413262637' type: ref='zipper' cand='priority' (12 diffs) | no |
| `--geometry.remove true` | real | PASS | net identical | no |
| `--geometry.remove true` | city_plano | PASS | net identical | no |
| `--geometry.remove true` | city_munich | FAIL | junction '1413262637' type: ref='zipper' cand='priority' (12 diffs) | no |
| `--ramps.guess true` | real | PASS | net identical | no |
| `--ramps.guess true` | city_plano | PASS | net identical | no |
| `--ramps.guess true` | city_munich | FAIL | junction '1413262637' type: ref='zipper' cand='priority' (12 diffs) | no |
| `--roundabouts.guess true` | real | PASS | net identical | no |
| `--roundabouts.guess true` | city_plano | PASS | net identical | no |
| `--roundabouts.guess true` | city_munich | FAIL | junction '1413262637' type: ref='zipper' cand='priority' (12 diffs) | no |
| `--junctions.join true` | real | PASS | net identical | no |
| `--junctions.join true` | city_plano | PASS | net identical | no |
| `--junctions.join true` | city_munich | FAIL | junction '1413262637' type: ref='zipper' cand='priority' (12 diffs) | no |
| `--tls.guess-signals true` | real | PASS | net identical | no |
| `--tls.guess-signals true` | city_plano | PASS | net identical | no |
| `--tls.guess-signals true` | city_munich | FAIL | junction '1413262637' type: ref='zipper' cand='priority' (12 diffs) | no |
| `--tls.join true` | real | PASS | net identical | no |
| `--tls.join true` | city_plano | PASS | net identical | no |
| `--tls.join true` | city_munich | FAIL | junction '1413262637' type: ref='zipper' cand='priority' (12 diffs) | no |
| `--tls.discard-simple true` | real | PASS | net identical | no |
| `--tls.discard-simple true` | city_plano | PASS | net identical | no |
| `--tls.discard-simple true` | city_munich | FAIL | junction '1413262637' type: ref='zipper' cand='priority' (12 diffs) | no |
| `--edges.join true` | real | PASS | net identical | no |
| `--edges.join true` | city_plano | PASS | net identical | no |
| `--edges.join true` | city_munich | FAIL | junction '1413262637' type: ref='zipper' cand='priority' (12 diffs) | no |
| `--remove-edges.isolated true` | real | PASS | net identical | no |
| `--remove-edges.isolated true` | city_plano | PASS | net identical | no |
| `--remove-edges.isolated true` | city_munich | FAIL | junction '1413262637' type: ref='zipper' cand='priority' (12 diffs) | no |
| `--keep-edges.by-vclass passenger` | real | PASS | net identical | no |
| `--keep-edges.by-vclass passenger` | city_plano | PASS | net identical | no |
| `--keep-edges.by-vclass passenger` | city_munich | PASS | net identical | no |
| `--remove-edges.by-type highway.footway,highway.cycleway` | real | PASS | net identical | no |
| `--remove-edges.by-type highway.footway,highway.cycleway` | city_plano | PASS | net identical | no |
| `--remove-edges.by-type highway.footway,highway.cycleway` | city_munich | FAIL | junction '1413262637' type: ref='zipper' cand='priority' (12 diffs) | no |
| `--sidewalks.guess true` | real | PASS | net identical | no |
| `--sidewalks.guess true` | city_plano | PASS | net identical | no |
| `--sidewalks.guess true` | city_munich | PASS | net identical | no |
| `--crossings.guess true` | real | PASS | net identical | no |
| `--crossings.guess true` | city_plano | PASS | net identical | no |
| `--crossings.guess true` | city_munich | FAIL | junction '1413262637' type: ref='zipper' cand='priority' (12 diffs) | no |
| `--osm.turn-lanes true` | real | PASS | net identical | no |
| `--osm.turn-lanes true` | city_plano | PASS | net identical | no |
| `--osm.turn-lanes true` | city_munich | FAIL | junction '1413262637' type: ref='zipper' cand='priority' (12 diffs) | no |
| `--osm.lane-access true` | real | PASS | net identical | no |
| `--osm.lane-access true` | city_plano | PASS | net identical | no |
| `--osm.lane-access true` | city_munich | FAIL | junction '1413262637' type: ref='zipper' cand='priority' (12 diffs) | no |
| `--osm.elevation true` | real | PASS | net identical | no |
| `--osm.elevation true` | city_plano | PASS | net identical | no |
| `--osm.elevation true` | city_munich | FAIL | junction '1413262637' type: ref='zipper' cand='priority' (12 diffs) | no |
| `--no-internal-links true` | real | PASS | net identical | no |
| `--no-internal-links true` | city_plano | PASS | net identical | no |
| `--no-internal-links true` | city_munich | FAIL | junction '1413262637' type: ref='zipper' cand='priority' (12 diffs) | no |
| `--default.lanewidth 3.2` | real | PASS | net identical | no |
| `--default.lanewidth 3.2` | city_plano | PASS | net identical | no |
| `--default.lanewidth 3.2` | city_munich | FAIL | junction '1413262637' type: ref='zipper' cand='priority' (12 diffs) | no |
| `--output.street-names true` | real | PASS | net identical | no |
| `--output.street-names true` | city_plano | PASS | net identical | no |
| `--output.street-names true` | city_munich | FAIL | junction '1413262637' type: ref='zipper' cand='priority' (12 diffs) | no |
| `--output.original-names true` | real | PASS | net identical | no |
| `--output.original-names true` | city_plano | PASS | net identical | no |
| `--output.original-names true` | city_munich | FAIL | junction '1413262637' type: ref='zipper' cand='priority' (12 diffs) | no |
| bundle: `--geometry.remove --ramps.guess --junctions.join --tls.guess-signals --tls.discard-simple --tls.join --tls.default-type actuated` | real | PASS | net identical | no |
| bundle (as above) | city_plano | PASS | net identical | no |
| bundle (as above) | city_munich | FAIL | junction '1413262637' type: ref='zipper' cand='priority' (12 diffs) | no |
| re-import `-s ref_simple.net.xml` | real | PASS | net identical | no |
| re-import `-s ref_simple.net.xml` | city_plano | PASS | net identical | no |
| re-import `-s ref_simple.net.xml` | city_munich | FAIL | connection '127751701#1->127751701#2 0->0' state: ref='m' cand='M' (3 diffs) | no |

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
- `run_netopts.sh` was changed in this re-run: the C++ binary and SUMO_HOME are now overridable (`CPP_NETCONVERT`, `CPP_SUMO_HOME`), and the file's CRLF line endings were converted to LF (bash could not parse it with CRLF).
