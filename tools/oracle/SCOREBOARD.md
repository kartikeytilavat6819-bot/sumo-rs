# Scoreboard — current state

Measured by the VM gate (`tools/gcloud/vm_gate.ps1`) on main `fbc6f780`. The gate ran branch
`worktree-agent-sb03` at `f9476459`, which is main plus one temporary commit that adds
`tools/kaggle/agent_run.sh` (prints every row verdict). The final commit removes that file again.

| item | result |
|---|---|
| main | `fbc6f780` |
| `run_all.sh` | PASS=350 FAIL=7 SKIP=63 (420 rows) |
| depart (`cases/depart/run_all_variants.sh`) | 15/15 PASS |
| `run_duarouter.sh` | 16/16 PASS (`results.tsv`) |
| `run_importers.sh` | absent in this checkout |
| `run_outputs.sh` | 38 rows written (lanechange 1, real 14, signal 10, stress 13); verdict tally not captured (script bug), two sampled rows PASS |
| unit tests (`sumo_core`, `utils`) | 82 + 126 passed, 0 failed (2 ignored) |

This page is written from that gate output only. Every row below is PASS, FAIL (listed with its owner)
or SKIP (reason given). Per-case `REPORT.md` files hold older detail and are not re-measured here.

Caveat: the captured gate output lacks the first 16 case rows (`astar_landmark` to `city_bengaluru_blr`).
Their verdicts below are marked `†` and inferred: they are not in the FAIL list, and the SKIP total only
closes (63 = 38 net + 22 sim + 2 PENDING + 1 depart) if `astar_landmark`, `calibrator_multi`, `cfmodels_new`
and `cfmodels_rail` are net SKIPs. Re-run `run_all.sh` on the VM to confirm.

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

## FAIL rows (7)

Owners are from `tools/lead/agent_preamble.md`. All seven fail at the sim stage; their net stages PASS.

| row | gate symptom | owner |
|---|---|---|
| `city_delft_campus_crossings/ref` | sim FAIL: vehicle `car8` `arrival` ref 387.00, ours 388.00 | cd01 |
| `city_florence_pedestrians/ref` | sim FAIL: vehicle `car12` `depart` ref 72.00, ours 73.00 | fp02 |
| `city_german_level_crossing/ref` | sim FAIL: vehicle `car81` `arrivalSpeed` ref 7.65, ours 7.90 | rx01 |
| `city_grenoble_tram_bus/ref` | sim FAIL: vehicle `L1f.0` `devices` differs (ref `tripinfo_L1f.0 fcd_L1f.0 person_L1f.0`) | cd01 |
| `city_phoenix_scale_grid/ref` | sim-PANIC in `microsim/lcmodels/mslcm_lc2013.rs` | px01 |
| `jmparams/ref` | sim FAIL: vehicle `fa.10` `arrivalSpeed` ref 12.72, ours 11.61 | jm01 |
| `routedist/ref` | sim-PANIC: `The route 'rd' for flow 'fDist' is not known.` | jm01 |

The previous scoreboard (933e39f6, 9 FAILs) is fixed: `busride/ref`, `city_paris_etoile/ref`,
`city_portland_actuated/ref`, `containers/ref`, `rerouter_parking/ref`, `taxi/ref`, `taxi/ref_closest`,
`tls_actuated_loops/ref` and `tls_waut/ref` are PASS in this gate. The seven rows above are new cases.

## SKIP rows (63)

Reasons come from `tools/oracle/run_all.sh` (line numbers in brackets).

| reason | rows | count |
|---|---|---|
| `depart` whole-row SKIP [37]: 15 variants run by `cases/depart/run_all_variants.sh` | depart | 1 |
| PENDING marker [41] (intermodal router `MSNet::getIntermodalRouter`, owner ir01) | persons, persons_router_pending | 2 |
| net SKIP `no input.osm and no net.args` [72] | 16 `duarouter_*` (measured by `run_duarouter.sh`, 16/16 PASS), `astar_landmark`, `calibrator_multi`, `cfmodels_new`, `cfmodels_rail`, `containers_tranship`, `device_driverstate`, `device_friction`, `device_glosa`, `device_ssm`, `device_stationfinder`, `device_toc`, `device_toc_takeover`, `ev_mmpevem`, `harmonoise`, `meso_city_plano`, `netconv_edge`, `netwrite_variants`, `person_public_car`, `persons_nonint`, `stops_transport`, `traci_basic`, `walk_rerouter` | 38 |
| sim SKIP `no routes/refs` [164] | 16 `duarouter_*`, `city_london_lh_sidewalks`, `city_tokyo_lefthand_stops` (net-only, have `net.args`), `netconv_edge`, `netwrite_variants`, `stops_transport`, `traci_basic` | 22 |

Notes:
- The net SKIP message is generic: "no net inputs" and "plain XML with no `net_plain.args`" share it.
  `netconv_edge`, `netwrite_variants` and `traci_basic` have no input files, so their own runners measure them.
- `stops_transport` is measured by `run_outputs.sh` (see below), not by the tripinfo loop.

## Case directories (`tools/oracle/cases/`, 172)

Verdicts are the gate rows (`net` / `sim`). "n/n" is the number of sim-stage variant rows, all PASS unless
shown. Descriptions come from each case's first `CASE.md` / `REPORT.md` line; cases with neither file are
named by their directory (marked "name only").

| case | what it is | net | sim |
|---|---|---|---|
| astar_landmark | A* with ALT landmark distances (plain, gzip tables) | SKIP† | PASS† |
| bluelight | blue-light (emergency) vehicles | PASS† | PASS† |
| busline | bus line (name only) | PASS† | PASS† |
| busride | bus ride with stops `bs0` on edge `E1` | PASS† | PASS† |
| calibrator | calibrator | PASS† | PASS† |
| calibrator_multi | several calibrators | SKIP† | PASS† |
| cfmodels | one variant per car-following / lane-change model | PASS† | PASS† |
| cfmodels_new | KraussX car-following model | SKIP† | PASS† |
| cfmodels_rail | rail car-following (Rail model) | SKIP† | PASS† |
| city_amsterdam_bikes | Amsterdam Leidseplein, cycle lanes | PASS† | PASS† |
| city_assen_turbo | Dutch roundabout cluster near Assen, `--roundabouts.guess` | PASS† | PASS† |
| city_atlanta_motorway | motorway interchange with on/off ramps | PASS† | PASS† |
| city_autobahn_interchange | A5/A6 Autobahnkreuz Walldorf, high speeds | PASS† | PASS† |
| city_barcelona_eixample | Eixample grid, many traffic lights | PASS† | PASS† |
| city_bengaluru_blr | Bengaluru MG Road block, left-hand | PASS† | PASS† |
| city_bengaluru_sublane | same as above, sublane model | PASS | PASS |
| city_berlin_priority | Berlin Mitte priority junctions and traffic lights | PASS | 2/2 PASS |
| city_cairo_priority | central Cairo, dense unsignalized priority junctions | PASS | PASS |
| city_copenhagen_canal_bikes | Christianshavn canals and bike lanes, `--osm.bike-access` | PASS | PASS |
| city_delft_campus_crossings | TU Delft campus, pedestrians and walking persons | PASS | **FAIL** (cd01) |
| city_dhaka_scale_jam | Dhaka Gulshan/Banani mixed grid, congested, teleports, 4000 vehicles | PASS | PASS |
| city_florence_pedestrians | Florence old town, pedestrian-heavy, persons walking | PASS | **FAIL** (fp02) |
| city_german_level_crossing | German town, level crossings with barriers, cars and trains | PASS | **FAIL** (rx01) |
| city_grenoble_tram_bus | Grenoble centre, bus stops with riding persons | PASS | **FAIL** (cd01) |
| city_hongkong_lefthand_grid | Mong Kok high-rise grid, left-hand | PASS | PASS |
| city_istanbul_bosphorus_layers | Bosphorus bridge approach at OrtakAy, layered roads | PASS | PASS |
| city_kansas_rural | rural Kansas road extract | PASS | PASS |
| city_karlsruhe_tram | Karlsruhe centre, tram tracks, bus stops (+ptstop, ptline, railway PASS) | PASS | PASS |
| city_kumbakonam_mixed | small Indian town, mixed cars and motorcycles, `--osm.lane-access` | PASS | PASS |
| city_la_interchange | Los Angeles four-level interchange | PASS | PASS |
| city_lisbon_baixa_oneway | Lisbon Baixa, dense one-way grid | PASS | PASS |
| city_london_lh | central London, left-hand | PASS | PASS |
| city_london_lh_sidewalks | London left-hand, net only (sidewalks) | PASS (net.args) | SKIP (no routes/refs) |
| city_manhattan_midtown | Midtown Manhattan grid, many signals | PASS | PASS |
| city_manhattan_sublane | same as above, sublane model | PASS | PASS |
| city_melbourne_guess_signals | Melbourne CBD, signals guessed from OSM nodes | PASS | PASS |
| city_mexico_reforma_tls | Reforma / Insurgentes boulevards, many signals, `--tls.join` | PASS | PASS |
| city_milton_keynes | roundabout-heavy UK new town | PASS | PASS |
| city_moscow_mkad_ramps | MKAD ring road segment, guessed ramps, high flow | PASS | PASS |
| city_mumbai_mixed | central Mumbai, passenger and bus mix | PASS | PASS |
| city_munich | central Munich old-town grid | PASS | PASS |
| city_paris_chapelle_ramps | Porte de la Chapelle, Périphérique ramps | PASS | PASS |
| city_paris_dense | Paris Opéra, dense signals | PASS | PASS |
| city_paris_etoile | Place Charles de Gaulle / Étoile, multi-lane roundabout | PASS | PASS |
| city_phoenix_scale_grid | Phoenix suburban grid, highway and actuated signals, 3273 vehicles | PASS | **PANIC** (px01) |
| city_plano | US suburban arterial, Plano TX | PASS | PASS |
| city_plano_mall_parking | Plano shopping district, parking areas | PASS | PASS |
| city_portland_actuated | downtown Portland grid, actuated signals | PASS | PASS |
| city_rotterdam_port | Waalhaven/Botlek port area, trucks only | PASS | PASS |
| city_saopaulo_grid | central São Paulo, dense grid | PASS | PASS |
| city_sf_hills | steep SF streets, elevation on | PASS | PASS |
| city_singapore_lefthand | central Singapore, left-hand | PASS | PASS |
| city_stelvio_pass_grades | Stelvio pass road, steep grades, trucks, emission output | PASS | PASS |
| city_stevenage_lefthand_roundabouts | Stevenage centre, left-hand roundabouts | PASS | PASS |
| city_tokyo_lefthand | Shibuya, left-hand | PASS | PASS |
| city_tokyo_lefthand_stops | Tokyo, net only (stops) | PASS (net.args) | SKIP (no routes/refs) |
| city_vienna_scale_downtown | dense Vienna inner city, static signals, 3001 vehicles | PASS | PASS |
| city_zurich_tram | central Zurich, tram tracks and trams (+ptstop, ptline, railway PASS) | PASS | PASS |
| closed_edge | closed-edge enforcement; blocks traffic already committed to it | PASS | PASS |
| collision | collision handling | PASS | 4/4 PASS |
| containers | container transport | PASS | PASS |
| containers_tranship | container transhipment | SKIP | PASS |
| depart | 15 departure variants (`run_all_variants.sh`) | — | variants SKIP here; **15/15 PASS** in the depart run |
| departlanepos | departure lane and position | PASS | PASS |
| departopts | departure options | PASS | 4/4 PASS |
| detectors | lane and area detector outputs | PASS | PASS |
| device_driverstate | driver-state device | SKIP | PASS |
| device_friction | friction device | SKIP | PASS |
| device_glosa | GLOSA device | SKIP | PASS |
| device_ssm | SSM device | SKIP | PASS |
| device_stationfinder | station finder device | SKIP | PASS |
| device_toc | takeover (ToC) device | SKIP | PASS |
| device_toc_takeover | takeover (ToC) device with fired takeover and MRM | SKIP | PASS |
| duarouter_alt_all, _alt_landmark, _astar, _ch, _emission_co2, _emission_fuel, _flows, _lane_permission, _priority, _random, _rc_gawron, _rc_logit, _repair, _repair_from_to, _trips, _weights | duarouter route oracle (16 cases) | SKIP | SKIP (`run_duarouter.sh`: **16/16 PASS**) |
| emissions | emission models (HBEFA3, PHEMlight(5), energy, edgedata, meso edgedata) | net-plain PASS | 7/7 PASS |
| ev | electric vehicles, battery and charging | net-plain PASS | 2/2 PASS |
| ev_mmpevem | EV with MMPEVEM emission class, precision 6 | SKIP | PASS |
| fcdattrs | FCD output attributes | PASS | PASS |
| flowprob | flow probability (`ref`, `ref_seed11`) | PASS | 2/2 PASS |
| harmonoise | harmonoise noise model | SKIP | 2/2 PASS |
| jmignore | jmignore (name only) | PASS | PASS |
| jmparams | jmparams (name only) | PASS | **FAIL** (jm01) |
| jmred | jmred (name only) | PASS | PASS |
| jt_allway_stop | 4-arm `allway_stop` junction | PASS | PASS |
| jt_left_before_right | 4-arm `left_before_right` junction | PASS | PASS |
| jt_priority | 4-arm `priority` junction | PASS | PASS |
| jt_priority_stop | 4-arm `priority_stop` junction | PASS | PASS |
| jt_right_before_left | 4-arm `right_before_left` junction | PASS | PASS |
| jt_right_before_left_lefthand | `right_before_left`, left-hand | PASS | PASS |
| jt_t_priority | 3-arm T-junction, `priority` | PASS | PASS |
| jt_traffic_light_right_on_red | 4-arm `traffic_light_right_on_red` | PASS | PASS |
| jt_traffic_light_unregulated | 4-arm `traffic_light_unregulated` | PASS | PASS |
| jt_unregulated | 4-arm `unregulated` | PASS | PASS |
| junction | plain junction net | PASS | PASS |
| keepclear | keep-clear area | PASS | PASS |
| lanechange | lane changing (`ref`, `dur3`, `lat04`, `lcdur3`) | PASS | 4/4 PASS |
| lc_dk2008 | lane-change model DK2008 | PASS | PASS |
| lc_lc2013 | lane-change model LC2013 | PASS | PASS |
| lc_oppo | lane-change model, opposite-direction | PASS | PASS |
| lc_sl2015 | lane-change model SL2015 | PASS | PASS |
| lcparams | lane-change parameters | PASS | PASS |
| lefthand | left-hand traffic net and sim | PASS | PASS |
| meso_city_plano | mesoscopic Plano (name only) | SKIP | PASS |
| meso_junction | mesoscopic junction | PASS | PASS |
| meso_real | mesoscopic Times Square | PASS | PASS |
| netconv_edge | netconvert on bad and unusual OSM input | SKIP | SKIP (no routes/refs) |
| netwrite_variants | netconvert output-writing variants | SKIP | SKIP (no routes/refs) |
| opts_scale_end | scale and end options (`ref`, `ref_end120`, `ref_scale2`) | PASS | 3/3 PASS |
| parking_stop | parking stop | PASS | PASS |
| peds_crossing | pedestrian crossing | PASS | PASS |
| persons | pedestrian persons | PENDING (ir01) | — |
| persons_nonint | pedestrian persons, non-interacting | SKIP | PASS |
| persons_router_pending | pedestrian persons, pending the router | PENDING (ir01) | — |
| person_public_car | persons using public transport and car | SKIP | PASS |
| rail | railway net with passing loop | PASS | PASS |
| real | Times Square, step / reroute / speed-deviation variants | PASS | 7/7 PASS |
| real_dense | congested Times Square, `--end 125` | PASS | PASS |
| reroute | rerouter and routing-device variants | net-plain PASS | 4/4 PASS |
| reroute_alt | rerouting at t=0; astar, ch, chwrapper variants | PASS | 4/4 PASS |
| reroute_bypass | mid-route detour through a bypass | PASS | 3/3 PASS |
| reroute_bypass_p1 | same net, `period=1` | net-plain PASS | PASS |
| reroute_contention | many vehicles compete for a bypass (`ref`, `ref_adapt`) | PASS | 2/2 PASS |
| rerouter_parking | rerouter with parking | PASS | PASS |
| roundabout | roundabout | PASS | PASS |
| route_check | disconnected route handling | net-plain PASS | PASS |
| routedist | route distribution (name only) | PASS | **PANIC** (jm01) |
| routeprobe | route probe | PASS | PASS |
| routes_forms | route-file syntax forms | net-plain PASS | PASS |
| runtimeopts | runtime options (`ref`, `ref_ballistic`, `ref_hw`) | PASS | 3/3 PASS |
| sigma_seed | RNG parity for `sigma` and `speedFactor` draws (`ref`, `ref_seed7`) | PASS | 2/2 PASS |
| signal | traffic light, step and model variants | PASS | 6/6 PASS |
| signal_actuated | actuated TLS | PASS | PASS |
| signal_delay | delay_based TLS | PASS | PASS |
| signal_nema | NEMA TLS | PASS | PASS |
| sl2015 | sl2015 (name only) | PASS | PASS |
| speeddev | speed deviation (`ref`, `ref_seed7`) | PASS | 2/2 PASS |
| speedfactor | speed factor (`ref`, `ref_nodev`) | PASS | 2/2 PASS |
| stops | stops | net-plain PASS | PASS |
| stops_extension | stop extension | PASS | PASS |
| stops_jump | stop jump | PASS | PASS |
| stops_parking | lane, `until`, parking-area and multi-stop | PASS | PASS |
| stops_reroute | stops with reroute | PASS | PASS |
| stops_transport | transport stops (outputs runner, see below) | SKIP | SKIP (no `ref.tripinfo.xml`) |
| stress | 120 vehicles oversaturating Times Square (+ collision, mdd5, ttt variants) | net-plain PASS | 5/5 PASS |
| sublanes | sublane model | PASS | PASS |
| taxi | taxi device | PASS | 2/2 PASS |
| teleport | blocked jam triggers `--time-to-teleport` | PASS | PASS |
| tls_actuated_loops | actuated loops TLS | PASS | PASS |
| tls_waut | waut TLS | PASS | PASS |
| traci_basic | TraCI probes (own runner, see below) | SKIP | SKIP (no routes/refs) |
| trips | trip input | net-plain PASS | PASS |
| ttt_highways | time-to-teleport on highways (name only) | PASS | PASS |
| vclass | vehicle classes | PASS | PASS |
| vss | variable speed signs | PASS | PASS |
| vtypes | vType defaults and per-model overrides | net-plain PASS | PASS |
| walk_rerouter | walking rerouter (name only) | SKIP | PASS |
| zipper | 2-lane edge into a zipper merge | PASS | PASS |

`†`: the gate output did not include these rows; verdict inferred from the totals (see caveat above).

## Other runners (not in the gate)

Last results found in committed docs, except `run_outputs.sh`, `run_duarouter.sh` and the depart run (measured
in the gate above). The committed results are older than the `fbc6f780` gate and were not re-measured.

| runner | last documented result | source |
|---|---|---|
| `run_duarouter.sh` (route oracle) | 16/16 PASS in this gate | gate output |
| `run_outputs.sh` (non-tripinfo outputs) | 38 rows written in this gate (verdicts not tallied, see header); earlier: 32 rows PASS, 0 FAIL, 0 PANIC (commit `552f5a8e`) | `OUTPUTS.md` |
| `run_outputs_city.sh` (city-scale outputs, six largest cities) | 60 rows PASS 60, FAIL 0, PANIC 0 | `OUTPUTS.md` (round om01) |
| `run_netopts.sh` (netconvert options) | 48 PASS, 21 FAIL, 0 PANIC of 69 runs. Failing rows: `city_munich` junction `1413262637` type zipper vs priority | `NETCONVERT_OPTIONS.md` |
| `traci_basic/client.py` (TraCI probes) | 13 of 32 probes match (junction 7/16, signal 6/16); branch `tc01` | `cases/traci_basic/REPORT.md` |
| release build | `PROFILE=release` PASS=172 FAIL=2 SKIP=20 at commit `f29591b4` (same verdicts as debug then); 0.57-0.60x C++ wall time on the 5 largest city cases | `PERF.md` |
| `run_state.sh` (state save/load) | no committed result found; referenced only in `HANDOVER.md` | — |
| `run_netwrite_variants.sh` | no committed result found; `netwrite_variants` is a SKIP row in the gate | — |
| `run_importers.sh` | not in this checkout | — |
