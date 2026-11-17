# Scoreboard — current state

Measured by the VM gate (`tools/gcloud/vm_gate.ps1`) on main `933e39f6`. The gate ran branch
`worktree-agent-sb02` at `5765bfcb`, which is main plus one temporary commit that adds
`tools/kaggle/agent_run.sh` (prints every row verdict). The final commit removes that file again.

**`run_all.sh` PASS=271 FAIL=9 SKIP=26 (306 rows); depart 15/15 PASS; unit tests 81 + 126 passed, 0 failed (2 ignored).**

This page is written from that gate output only. Every row below is either PASS, FAIL (listed
with its owner) or SKIP (reason given). Per-case `REPORT.md` files hold older detail and are not
re-measured here.

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

## FAIL rows (9)

Owners are from `tools/lead/agent_preamble.md`. All nine fail at the sim stage; their net stages PASS.

| row | gate symptom | owner |
|---|---|---|
| `busride/ref` | sim-PANIC: `No connection between edge 'E1' and stop 'bs0' found` | pt02 |
| `city_paris_etoile/ref` | sim FAIL: vehicle 0 `arrivalSpeed` ref 8.72, ours 8.60 | rb02 |
| `city_portland_actuated/ref` | sim FAIL: vehicle 15 `arrivalSpeed` ref 7.76, ours 7.68 | tl01 |
| `containers/ref` | sim-PANIC in `transportables/msp_model.rs` | rc01 |
| `rerouter_parking/ref` | sim-PANIC in `trigger/ms_stopping_place` | rc01 |
| `taxi/ref` | sim-PANIC in `devices/ms_device_taxi.rs` | tx01 |
| `taxi/ref_closest` | sim-PANIC in `devices/ms_device_taxi.rs` | tx01 |
| `tls_actuated_loops/ref` | sim FAIL: vehicle `fn.0` `arrival` ref 32.00, ours 30.00 | tl01 |
| `tls_waut/ref` | sim FAIL: vehicle `fw.3` `arrival` ref 55.00, ours 56.00 | tl01 |

Against the 8093db8c scoreboard (5 FAILs), `city_kansas_rural`, `city_karlsruhe_tram`, `city_london_lh`,
`city_paris_dense` and `city_tokyo_lefthand` now PASS. The nine rows above are not in that old FAIL list.

## SKIP rows (26)

Reasons come from `tools/oracle/run_all.sh`. Checked against the case files on disk.

| reason (run_all.sh) | rows | count |
|---|---|---|
| `depart` is a whole-row SKIP (line 36): 15 variants need 15 sims, run by `cases/depart/run_all_variants.sh` | depart | 1 |
| `PENDING` marker (lines 40-41): intermodal router (`MSNet::getIntermodalRouter`), owner ir01 | persons, persons_router_pending | 2 |
| net SKIP `no input.osm and no net.args` (line 72) | astar_landmark, calibrator_multi, cfmodels_new, cfmodels_rail, device_driverstate, device_friction, device_glosa, device_ssm, device_stationfinder, device_toc, ev_mmpevem, harmonoise, meso_city_plano, stops_transport, netconv_edge, netwrite_variants, traci_basic | 17 |
| sim SKIP `no routes/refs` (line 164): `routes.rou` + `ref.tripinfo.xml` + `ref_simple.net.xml` must all exist | city_london_lh_sidewalks, city_tokyo_lefthand_stops (net-only, have `net.args`); stops_transport (has `routes.rou`, no `ref.tripinfo.xml`); netconv_edge, netwrite_variants, traci_basic | 6 |

Notes:
- The net SKIP message is generic: the branch taken is the same for "no net inputs" and for "plain XML with no
  `net_plain.args`". `netconv_edge`, `netwrite_variants` and `traci_basic` have no input files at all, so they are
  measured by their own runners (see below), not by this loop.
- `stops_transport` is measured by `run_outputs.sh` (see below), not by the tripinfo loop.

## Case directories (`tools/oracle/cases/`, 118)

Verdicts are the gate rows. "sim n/n" is the number of sim-stage variant rows for that case, all PASS
unless shown. Descriptions come from each case's `CASE.md` / `REPORT.md` first line; cases with
neither file are named by their directory.

| case | what it is | net | sim |
|---|---|---|---|
| astar_landmark | A* with ALT landmark distances (plain, gzip tables) | SKIP | ref, ref_gz PASS |
| bluelight | blue-light (emergency) vehicles | PASS | ref PASS |
| busline | bus line (name only) | PASS | ref PASS |
| busride | bus ride with stops `bs0` on edge `E1` | PASS | **FAIL** ref sim-PANIC (pt02) |
| calibrator | calibrator | PASS | ref PASS |
| calibrator_multi | several calibrators | SKIP | ref PASS |
| cfmodels | one variant per car-following / lane-change model | net-plain PASS | 20/20 PASS |
| cfmodels_new | KraussX car-following model | SKIP | ref, ref_KraussX PASS |
| cfmodels_rail | rail car-following (Rail model) | SKIP | ref, ref_Rail PASS |
| city_amsterdam_bikes | Amsterdam Leidseplein, cycle lanes | PASS | PASS |
| city_atlanta_motorway | motorway interchange with on/off ramps | PASS | PASS |
| city_autobahn_interchange | A5/A6 Autobahnkreuz Walldorf, high speeds | PASS | PASS |
| city_barcelona_eixample | Eixample grid, many traffic lights | PASS | PASS |
| city_bengaluru_blr | Bengaluru MG Road, left-hand | PASS | PASS |
| city_bengaluru_sublane | same as above, sublane model | PASS | PASS |
| city_berlin_priority | Berlin Mitte priority junctions and traffic lights | PASS | ref, ref_meso_overtaking PASS |
| city_kansas_rural | rural Kansas road extract | PASS | PASS |
| city_karlsruhe_tram | Karlsruhe centre, tram tracks, bus stops | PASS (+ ptstop, ptline, railway PASS) | PASS |
| city_la_interchange | Los Angeles four-level interchange | PASS | PASS |
| city_london_lh | central London, left-hand | PASS | PASS |
| city_london_lh_sidewalks | London left-hand, net only (sidewalks) | PASS (net.args) | SKIP (no routes/refs) |
| city_manhattan_midtown | Midtown Manhattan grid, many signals | PASS | PASS |
| city_manhattan_sublane | same as above, sublane model | PASS | PASS |
| city_milton_keynes | roundabout-heavy UK new town | PASS | PASS |
| city_mumbai_mixed | central Mumbai, passenger and bus mix | PASS | PASS |
| city_munich | central Munich old-town grid | PASS | PASS |
| city_paris_chapelle_ramps | Porte de la Chapelle, Périphérique ramps | PASS | PASS |
| city_paris_dense | Paris Opéra, dense signals | PASS | PASS |
| city_paris_etoile | Place de l'Étoile, multi-lane roundabout | PASS | **FAIL** ref arrivalSpeed (rb02) |
| city_plano | Plano TX suburban arterial | PASS | PASS |
| city_portland_actuated | downtown Portland, actuated signals | PASS | **FAIL** ref arrivalSpeed (tl01) |
| city_rotterdam_port | Rotterdam port area, trucks only | PASS | PASS |
| city_saopaulo_grid | central São Paulo, dense grid | PASS | PASS |
| city_sf_hills | steep SF streets, elevation on | PASS | PASS |
| city_singapore_lefthand | central Singapore, left-hand | PASS | PASS |
| city_tokyo_lefthand | Shibuya, left-hand | PASS | PASS |
| city_tokyo_lefthand_stops | Tokyo, net only (stops) | PASS (net.args) | SKIP (no routes/refs) |
| city_zurich_tram | central Zurich, tram tracks and trams | PASS (+ ptstop, ptline, railway PASS) | PASS |
| closed_edge | closed lane enforced; blocks traffic already committed to it | PASS | PASS |
| collision | collision handling | PASS | ref, ref_none, ref_remove, ref_teleport PASS |
| containers | container transport | PASS | **FAIL** ref sim-PANIC (rc01) |
| depart | 15 departure variants (`run_all_variants.sh`) | — | variants SKIP here; **15/15 PASS** in the depart run |
| departlanepos | departure lane and position | PASS | PASS |
| detectors | lane and area detector outputs | PASS | PASS |
| device_driverstate | driver-state device | SKIP | PASS |
| device_friction | friction device | SKIP | PASS |
| device_glosa | GLOSA device | SKIP | PASS |
| device_ssm | SSM device | SKIP | PASS |
| device_stationfinder | station finder device | SKIP | PASS |
| device_toc | takeover (ToC) device | SKIP | PASS |
| emissions | emission models (HBEFA3, PHEMlight(5), energy, edgedata, meso edgedata) | net-plain PASS | 7/7 PASS |
| ev | electric vehicles, battery and charging | net-plain PASS | ref, ref_energy PASS |
| ev_mmpevem | EV with MMPEVEM emission class, precision 6 | SKIP | PASS |
| flowprob | flow probability (`ref`, `ref_seed11`) | PASS | 2/2 PASS |
| harmonoise | harmonoise noise model | SKIP | ref, ref_edgedata PASS |
| jmignore | jmignore (name only) | PASS | PASS |
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
| persons_router_pending | pedestrian persons, pending the router | PENDING (ir01) | — |
| rail | railway net with passing loop | PASS | PASS |
| real | Times Square, step / reroute / speed-deviation variants | PASS | 7/7 PASS |
| real_dense | congested Times Square, `--end 125` | PASS | PASS |
| reroute | rerouter and routing-device variants | net-plain PASS | 4/4 PASS |
| reroute_alt | rerouting at t=0; astar, ch, chwrapper variants | PASS | 4/4 PASS |
| reroute_bypass | mid-route detour through a bypass | PASS | 3/3 PASS |
| reroute_bypass_p1 | same net, `period=1` | net-plain PASS | PASS |
| reroute_contention | many vehicles compete for a bypass (`ref`, `ref_adapt`) | PASS | 2/2 PASS |
| rerouter_parking | rerouter with parking | PASS | **FAIL** ref sim-PANIC (rc01) |
| roundabout | roundabout | PASS | PASS |
| route_check | disconnected route handling | net-plain PASS | PASS |
| routeprobe | route probe | PASS | PASS |
| routes_forms | route-file syntax forms | net-plain PASS | PASS |
| sigma_seed | RNG parity for `sigma` and `speedFactor` draws (`ref`, `ref_seed7`) | PASS | 2/2 PASS |
| signal | traffic light, step and model variants | PASS | 6/6 PASS |
| signal_actuated | actuated TLS | PASS | PASS |
| signal_delay | delay_based TLS | PASS | PASS |
| signal_nema | NEMA TLS | PASS | PASS |
| sl2015 | sl2015 (name only) | PASS | PASS |
| speeddev | speed deviation (`ref`, `ref_seed7`) | PASS | 2/2 PASS |
| stops | stops | net-plain PASS | PASS |
| stops_parking | lane, `until`, parking-area and multi-stop | PASS | PASS |
| stops_reroute | stops with reroute | PASS | PASS |
| stops_transport | transport stops (outputs runner, see below) | SKIP | SKIP (no `ref.tripinfo.xml`) |
| stress | 120 vehicles oversaturating Times Square (+ collision, mdd5, ttt variants) | net-plain PASS | 5/5 PASS |
| sublanes | sublane model | PASS | PASS |
| taxi | taxi device | PASS | **FAIL** ref and ref_closest sim-PANIC (tx01) |
| teleport | blocked jam triggers `--time-to-teleport` | PASS | PASS |
| tls_actuated_loops | actuated loops TLS | PASS | **FAIL** ref arrival (tl01) |
| tls_waut | waut TLS | PASS | **FAIL** ref arrival (tl01) |
| traci_basic | TraCI probes (own runner, see below) | SKIP | SKIP (no routes/refs) |
| trips | trip input | net-plain PASS | PASS |
| ttt_highways | time-to-teleport on highways (name only) | PASS | PASS |
| vclass | vehicle classes | PASS | PASS |
| vss | variable speed signs | PASS | PASS |
| vtypes | vType defaults and per-model overrides | net-plain PASS | PASS |
| zipper | 2-lane edge into a zipper merge | PASS | PASS |

## Other runners (not in the gate)

Last results found in committed docs. These are older than the 933e39f6 gate and were not re-measured.

| runner | last documented result | source |
|---|---|---|
| `run_outputs.sh` (non-tripinfo outputs) | 32 rows PASS, 0 FAIL, 0 PANIC on the `real`, `signal`, `stress`, `lanechange` outputs (commit `552f5a8e`); ou04 added `stopout`, `queue_agg` (PASS) | `OUTPUTS.md` |
| `run_netopts.sh` (netconvert options) | 48 PASS, 21 FAIL, 0 PANIC of 69 runs. Failing rows: `city_munich` junction `1413262637` type zipper vs priority | `NETCONVERT_OPTIONS.md` |
| `traci_basic/client.py` (TraCI probes) | 13 of 32 probes match (junction 7/16, signal 6/16); branch `tc01` | `cases/traci_basic/REPORT.md` |
| release build | `PROFILE=release` PASS=172 FAIL=2 SKIP=20 at commit `f29591b4` (same verdicts as debug then); 0.57-0.60x C++ wall time on the 5 largest city cases | `PERF.md` |
| `run_state.sh` (state save/load) | no committed result found; referenced only in `HANDOVER.md` | — |
| `run_netwrite_variants.sh` | no committed result found; `netwrite_variants` is a SKIP row in the gate | — |
| `run_duarouter.sh` | not in this checkout; no `duarouter_*` case directories exist | — |
