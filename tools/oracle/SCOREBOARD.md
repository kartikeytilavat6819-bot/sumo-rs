# Scoreboard — current state

Measured by the VM gate (`tools/gcloud/vm_gate.ps1`) on main at sha `8093db8c`
(main has since moved to `b150f85c`, a docs/CI commit):

**`run_all.sh` PASS=151 FAIL=5 SKIP=18; depart 15/15; unit tests 67 + 118 passed, 0 failed.**

This page is written from that gate result only. Per-case `REPORT.md` verdicts are not repeated
here; read them for details. A row is `PASS (gate 8093db8c)` unless it is in the FAIL list or
explained as SKIP.

## Reproduce

The gate builds and runs on the VM; do not trust a local shared target dir (stale-binary risk):

```powershell
powershell -File tools/gcloud/vm_gate.ps1        # VM gate: build + run_all + depart + unit tests
```

Locally, from a checkout with a freshly built binary:

```bash
CARGO_TARGET_DIR=<own dir> bash tools/oracle/run_all.sh     # prints STALE BINARY if the binary is older than the sources
bash tools/oracle/cases/depart/run_all_variants.sh          # 15 depart variants
```

`run_all.sh` runs the checkout it lives in. Net stage: `--osm-files input.osm --simple-projection`
(plus `net.args` if present), or `net.args` alone for plain-XML cases. Sim stage: one row per
committed reference variant (`ref` uses `sim.args`, `ref_<v>` uses `sim_<v>.args`).

## FAIL rows (sim) on main at 8093db8c

| row | owner |
|---|---|
| `city_kansas_rural/ref` sim | ck01 |
| `city_karlsruhe_tram/ref` sim | ck02 |
| `city_london_lh/ref` sim | in01 (fix being merged) |
| `city_paris_dense/ref` sim | cb01 |
| `city_tokyo_lefthand/ref` sim | cb01 |

All five fail at the sim stage; their net stages PASS.

## SKIPs (18)

- **Net stage SKIP, no `input.osm` and no `net.args` (14):** cfmodels, emissions, ev, netconv_edge,
  reroute, reroute_bypass_p1, route_check, state_roundtrip, stops, stress, traci_basic, trips,
  vtypes (13 rows) + depart (whole-row SKIP, own runner).
- **Sim stage SKIP, no routes/refs (3):** netconv_edge, traci_basic, state_roundtrip — each has
  its own runner (see below).

`netconv_edge`, `traci_basic`, `state_roundtrip` and `depart` are measured by their own runners
(`run_all.sh` in the case dir, `client.py`, `roundtrip.sh`, `run_all_variants.sh`); their status
is not part of the 151 PASS count.

## Case directories (`tools/oracle/cases/`)

Status from the 8093db8c gate. The four own-runner cases are listed with what they cover.

| case | what it tests | status |
|---|---|---|
| cfmodels | car-following models on the stress net/routes (`--end 200`): IDM/Krauss/W99/EIDM/Wiedemann/SmartSK/ACC/CACC/CC … | PASS (gate 8093db8c) |
| city_amsterdam_bikes | Amsterdam centre OSM extract, net + sim | PASS (gate 8093db8c) |
| city_atlanta_motorway | Atlanta motorway OSM extract, net + sim | PASS (gate 8093db8c) |
| city_barcelona_eixample | Barcelona Eixample OSM extract, net + sim | PASS (gate 8093db8c) |
| city_bengaluru_blr | Bengaluru OSM extract, `--lefthand` | PASS (gate 8093db8c) |
| city_bengaluru_sublane | same net/routes as `city_bengaluru_blr` + sublane model | PASS (gate 8093db8c) |
| city_kansas_rural | rural Kansas OSM extract, right-hand traffic | **FAIL** (`ref` sim) — ck01 |
| city_karlsruhe_tram | Karlsruhe OSM extract with tram ways | **FAIL** (`ref` sim) — ck02 |
| city_la_interchange | Los Angeles interchange OSM extract | PASS (gate 8093db8c) |
| city_london_lh | London (Trafalgar Square) OSM extract, left-hand traffic | **FAIL** (`ref` sim) — in01, fix being merged |
| city_manhattan_midtown | Manhattan midtown OSM extract | PASS (gate 8093db8c) |
| city_manhattan_sublane | same net/routes as `city_manhattan_midtown` + sublane model | PASS (gate 8093db8c) |
| city_milton_keynes | Milton Keynes OSM extract | PASS (gate 8093db8c) |
| city_munich | Munich old-town grid OSM extract | PASS (gate 8093db8c) |
| city_paris_chapelle_ramps | Paris Chapelle ramps OSM extract | PASS (gate 8093db8c) |
| city_paris_dense | dense Paris OSM extract | **FAIL** (`ref` sim) — cb01 |
| city_plano | Plano TX suburban arterial OSM extract (UTM report in `REPORT_UTM.md`) | PASS (gate 8093db8c) |
| city_tokyo_lefthand | Tokyo OSM extract, left-hand traffic | **FAIL** (`ref` sim) — cb01 |
| closed_edge | a closed lane blocks traffic already committed to it (`v0` reroutes at t=8) | PASS (gate 8093db8c) |
| depart | 15 departure variants (`lane_*`, `pos_*`, `speed_*`) | SKIP here (own runner); **15/15 PASS** |
| detectors | lane/area detector additional-file outputs + sim | PASS (gate 8093db8c) |
| emissions | emission models: energy, HBEFA3, PHEMlight(+5) | PASS (gate 8093db8c) |
| ev | EV battery + charging-station outputs | PASS (gate 8093db8c) |
| junction | plain OSM junction net + sim | PASS (gate 8093db8c) |
| jt_allway_stop | allway_stop junction from plain XML | PASS (gate 8093db8c) |
| jt_left_before_right | left_before_right junction | PASS (gate 8093db8c) |
| jt_priority | priority junction (unregulated arms) | PASS (gate 8093db8c) |
| jt_priority_stop | priority_stop junction | PASS (gate 8093db8c) |
| jt_right_before_left | right_before_left junction | PASS (gate 8093db8c) |
| jt_right_before_left_lefthand | right_before_left + `--lefthand` | PASS (gate 8093db8c) |
| jt_traffic_light_right_on_red | traffic_light_right_on_red junction | PASS (gate 8093db8c) |
| jt_traffic_light_unregulated | traffic_light_unregulated junction | PASS (gate 8093db8c) |
| jt_t_priority | T-junction, 3 arms, priority | PASS (gate 8093db8c) |
| jt_unregulated | unregulated junction | PASS (gate 8093db8c) |
| lanechange | lane changing, `dur3` / `lat04` / `lcdur3` variants | PASS (gate 8093db8c) |
| lefthand | left-hand traffic net + sim (`net.args` = `--lefthand`) | PASS (gate 8093db8c) |
| netconv_edge | netconvert on bad/unusual OSM corpora | SKIP here (own runner, `run_all.sh` in the dir) |
| rail | hand-written railway net with a passing loop | PASS (gate 8093db8c) |
| real | Times Square, step-length/step-method/reroute variants | PASS (gate 8093db8c) |
| real_dense | congested Times Square (`--end 125`) | PASS (gate 8093db8c) |
| reroute | rerouter + routing-device variants (`ref`, `closing`, `device`) | PASS (gate 8093db8c) |
| reroute_alt | rerouting at t=0; astar / ch / chwrapper variants | PASS (gate 8093db8c) |
| reroute_bypass | mid-route detour through a bypass, `period=1000` | PASS (gate 8093db8c) |
| reroute_bypass_p1 | same net, `period=1`; C++ reverts the detour | PASS (gate 8093db8c) |
| reroute_contention | 12 vehicles reroute at once and compete for a bypass | PASS (gate 8093db8c) |
| roundabout | roundabout net + sim | PASS (gate 8093db8c) |
| routes_forms | route-file syntax forms (inline `<route>`, `<trip>`) | PASS (gate 8093db8c) |
| route_check | route validation incl. disconnected route (`run_error.sh`) | PASS (gate 8093db8c) |
| sigma_seed | RNG parity for `sigma`/`speedFactor` draws under `--seed` | PASS (gate 8093db8c) |
| signal | traffic light sim, step variants | PASS (gate 8093db8c) |
| signal_actuated | actuated TLS | PASS (gate 8093db8c) |
| signal_delay | delay_based TLS | PASS (gate 8093db8c) |
| signal_nema | NEMA TLS | PASS (gate 8093db8c) |
| state_roundtrip | save/load state at t=10 | SKIP here (own runner, `roundtrip.sh`) |
| stops | stop handling | PASS (gate 8093db8c) |
| stops_parking | lane stop / `until` / parking-area / multi-stop | PASS (gate 8093db8c) |
| stress | 120 vehicles oversaturating Times Square (+ collision/mdd5/ttt variants) | PASS (gate 8093db8c) |
| sublanes | `--lateral-resolution 0.8` sublane model | PASS (gate 8093db8c) |
| teleport | blocked jam triggers `--time-to-teleport 20` | PASS (gate 8093db8c) |
| traci_basic | TraCI live command probes | SKIP here (own runner, `client.py`) |
| trips | trip-based input | PASS (gate 8093db8c) |
| vtypes | vType defaults and per-model overrides | PASS (gate 8093db8c) |
| zipper | 2-lane edge into a zipper merge | PASS (gate 8093db8c) |

## Still-open non-case work

Non-tripinfo outputs, TraCI coverage, meso, and the other known gaps are not re-measured by the
8093db8c gate and keep their previous status; see `tools/oracle/OUTPUTS.md`,
`tools/oracle/TRAci_STATUS.md` and the older SCOREBOARD history in git.
