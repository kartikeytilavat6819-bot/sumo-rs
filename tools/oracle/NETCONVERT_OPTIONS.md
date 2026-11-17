# netconvert options users add for OSM: C++ vs ours

Runner: `tools/oracle/run_netopts.sh [net ...]` (default: `real city_plano city_munich reimport`). Each OSM run uses
`--simple-projection true` plus the option. Nets: `real`, `city_plano`, `city_munich` (`cases/*/input.osm`); re-import is
`-s ref_simple.net.xml` for each case.

Binaries, set through env vars (defaults are WSL paths on the author's machine):
- `RUST_NETCONVERT`: ours. Gate VM: `$CARGO_TARGET_DIR/debug/netconvert`.
- `CPP_NETCONVERT`: C++ 1.28.0 reference. Gate VM: `$SUMO_PRISTINE_BIN/netconvert`.
- `CPP_SUMO_HOME`: its SUMO_HOME. `NETOPTS_OUT`: output dir (logs, nets, `<net>/summary.tsv`).

The script runs on WSL and on native Linux: `winpath` uses `wslpath -w` when present and the plain path otherwise.

Compare: `compare_net.py` (C++ = reference). Verdicts: PASS = same exit code and identical net; FAIL = exit or net differs;
PANIC = our binary panicked.

## Measured 2026-11-16 (gate VM, commit `2160d17d`, our binary built from main `c0707ba0` + the SAX change in `d5c6c6ba`)

**69 runs: 69 PASS, 0 FAIL, 0 PANIC.**

| net | runs | verdict |
|---|---|---|
| real | 22 (control + 21 options/bundle) | 22 PASS, net identical |
| city_plano | 22 | 22 PASS, net identical |
| city_munich | 22 | 22 PASS, net identical |
| re-import `-s ref_simple.net.xml` (real, city_plano, city_munich) | 3 | 3 PASS, net identical |

The options: `--geometry.remove`, `--ramps.guess`, `--roundabouts.guess`, `--junctions.join`, `--tls.guess-signals`,
`--tls.join`, `--tls.discard-simple`, `--edges.join`, `--remove-edges.isolated`, `--keep-edges.by-vclass passenger`,
`--remove-edges.by-type highway.footway,highway.cycleway`, `--sidewalks.guess`, `--crossings.guess`, `--osm.turn-lanes`,
`--osm.lane-access`, `--osm.elevation`, `--no-internal-links`, `--default.lanewidth 3.2`, `--output.street-names`,
`--output.original-names`, and the bundle (`--geometry.remove --ramps.guess --junctions.join --tls.guess-signals
--tls.discard-simple --tls.join --tls.default-type actuated`).

## History

- Earlier runs (x07 binary, Nov 16 morning) had 21 FAIL on city_munich: junction `1413262637` type `zipper` (C++) vs
  `priority` (ours), plus `m`/`M` link states on re-import. Those rows now PASS on main (`c0707ba0`). This agent changed no
  code for them; the cause was fixed on main before this run.
- The old "unknown compound `psv`" and "unusable type `railway.platform`" warnings were not compared line by line, so
  their text is still unverified. The verdicts above only compare nets, not logs.

## What the verdict does not cover

- `compare_net.py` checks attributes of junction, node, edge, lane, connection, tlLogic, phase, crossing and walkingArea only.
  It does not check roundabout elements or junction children (`request`, `neigh`). A PASS does not prove those match.
- Logs are not compared (warning text, traffic-light summary wording).
- The 7-byte header gap (generation stamp, input and output paths) is ignored by `compare_net.py`.
