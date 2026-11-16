# Reference audit: committed vs pristine SUMO 1.28.0

Pristine binary: `eclipse-sumo==1.28.0` from PyPI, installed in WSL at `~/sumo-pristine`
(`sumo`/`netconvert` report `Eclipse SUMO ... 1.28.0`, sha256 of `sumo` = `450cb887af6a08dac1f9bd0454effa6dea2bb4e9f19e67adcb64ca36ce490c3a`).
`SUMO_HOME=~/sumo-pristine/lib/python3.12/site-packages/sumo`.

Method. Every committed reference was regenerated into a temp copy with the exact flags of
`run_all.sh`: net = `--osm-files input.osm --simple-projection` (or `net.args` alone) with `net.args`;
sim = `-n ref_simple.net.xml -r routes.rou <sim*.args> --no-step-log true` plus `--fcd-output`/`--statistic-output`
(as run_all does); depart via `cases/depart/run_ref.sh`; cf via `cases/vtypes/run_cf_models.sh`.
A file is IDENTICAL when its content equals the pristine output after removing: the `generated on` header line,
path-valued `<configuration>` echo lines (they record where the command ran), `<performance>` wall-clock lines,
and CR characters (the working copy is CRLF, the index is LF).

Summary: 102 IDENTICAL, 1 DIFFERENT, 8 NOT-REGENERATED
(file keys below; a key is one case/variant, grouping its tripinfo/fcd/stat).

| case/variant | files | committed vs pristine | first diff / note |
|---|---|---|---|
| `city_munich/ref` | fcd, stat, tripinfo | IDENTICAL | |
| `city_munich/net` | net | IDENTICAL | |
| `city_plano/ref` | fcd, stat, tripinfo | IDENTICAL | |
| `city_plano/net` | net | IDENTICAL | |
| `closed_edge/ref` | fcd, stat, tripinfo | IDENTICAL | |
| `closed_edge/net` | net | IDENTICAL | |
| `depart/lane_1` | statistic, tripinfo | IDENTICAL (both abort) | Pristine exit 1: `Invalid departLane definition for vehicle 'v3'`. Committed output equals it (0 trips). Reference encodes a run official SUMO rejects. |
| `depart/lane_allowed` | statistic, tripinfo | IDENTICAL | |
| `depart/lane_best` | statistic, tripinfo | IDENTICAL | |
| `depart/lane_first` | statistic, tripinfo | IDENTICAL | |
| `depart/lane_random` | statistic, tripinfo | IDENTICAL | |
| `depart/pos_5` | statistic, tripinfo | IDENTICAL | |
| `depart/pos_base` | statistic, tripinfo | IDENTICAL | |
| `depart/pos_free` | statistic, tripinfo | IDENTICAL | |
| `depart/pos_max` | statistic, tripinfo | IDENTICAL (both abort) | Pristine exit 1: `Invalid departPos definition for vehicle 'v0'` (`max` is not valid in 1.28.0). Committed output equals it (0 trips). Reference encodes a run official SUMO rejects. |
| `depart/pos_random` | statistic, tripinfo | IDENTICAL | |
| `depart/speed_0` | statistic, tripinfo | IDENTICAL | |
| `depart/speed_desired` | statistic, tripinfo | IDENTICAL | |
| `depart/speed_last` | statistic, tripinfo | IDENTICAL | |
| `depart/speed_max` | statistic, tripinfo | IDENTICAL | |
| `depart/speed_speedLimit` | statistic, tripinfo | IDENTICAL | |
| `detectors/ref` | fcd, stat, tripinfo | IDENTICAL | |
| `detectors/net` | net | IDENTICAL | |
| `junction/ref` | fcd, stat, tripinfo | IDENTICAL | |
| `junction/net` | net | IDENTICAL | |
| `lanechange/ref` | fcd, stat, tripinfo | IDENTICAL | |
| `lanechange/ref_lat04` | fcd, stat, tripinfo | IDENTICAL | |
| `lanechange/ref_lcdur3` | fcd, stat, tripinfo | IDENTICAL | |
| `lanechange/net` | net | IDENTICAL | |
| `lefthand/ref` | fcd, stat, tripinfo | IDENTICAL | |
| `lefthand/net` | net | IDENTICAL | |
| `real/ref` | fcd, stat, tripinfo | IDENTICAL | |
| `real/ref_actstep` | fcd, stat, tripinfo | IDENTICAL | |
| `real/ref_ballistic` | fcd, stat, tripinfo | IDENTICAL | |
| `real/ref_reroute` | fcd, stat, tripinfo | DIFFERENT (replaced) | fcd: line 52: committed='        <vehicle id="v2" x="184.51" y="224.68" angle="235.00" type="DEFAULT_VEHTYPE" speed="10.96" pos="36.37" lane="682360554#1_3" slope="0; stat: line 24: committed='    <vehicleTripStatistics count="6" routeLength="107.40" speed="15.31" duration="7.67" waitingTime="0.00" timeLoss="0.59" departDelay="0.00; tripinfo: line 20: committed='    <tripinfo id="v0" depart="0.00" departLane="397795464#0_0" departPos="2.26" departSpeed="11.85" departDelay="0.00" arrival="8.00" arriva |
| `real/net` | net | IDENTICAL | |
| `real/ref_speeddev` | fcd, stat, tripinfo | IDENTICAL | |
| `real/ref_step01` | fcd, stat, tripinfo | IDENTICAL | |
| `real/ref_step05` | fcd, stat, tripinfo | IDENTICAL | |
| `real_dense/ref` | fcd, stat, tripinfo | IDENTICAL | |
| `real_dense/net` | net | IDENTICAL (semantic) | compare_net.py: 0 differences over 1165 elements. Committed net is one unwrapped line; byte form differs, so not replaced. |
| `reroute/ref` | fcd, stat, tripinfo | IDENTICAL | |
| `reroute/ref_closing` | fcd, stat, tripinfo | IDENTICAL | |
| `reroute/ref_device` | fcd, stat, tripinfo | IDENTICAL | |
| `reroute/net` | net | NOT-REGENERATED | no pristine output |
| `reroute_alt/ref` | fcd, stat, tripinfo | IDENTICAL | |
| `reroute_alt/ref_astar` | fcd, stat, tripinfo | IDENTICAL | |
| `reroute_alt/ref_ch` | fcd, stat, tripinfo | IDENTICAL | |
| `reroute_alt/ref_chwrapper` | fcd, stat, tripinfo | IDENTICAL | |
| `reroute_alt/net` | net | IDENTICAL | |
| `reroute_bypass/ref` | fcd, stat, tripinfo | IDENTICAL | |
| `reroute_bypass/net` | net | IDENTICAL | |
| `reroute_bypass_p1/ref` | fcd, stat, tripinfo | IDENTICAL | |
| `reroute_bypass_p1/net` | net | NOT-REGENERATED | no pristine output |
| `reroute_contention/ref` | fcd, stat, tripinfo | IDENTICAL | |
| `reroute_contention/ref_adapt` | fcd, stat, tripinfo | IDENTICAL | |
| `reroute_contention/net` | net | IDENTICAL | |
| `roundabout/ref` | fcd, stat, tripinfo | IDENTICAL | |
| `roundabout/net` | net | IDENTICAL | |
| `route_check/ref` | fcd, stat, tripinfo | IDENTICAL | |
| `route_check/net` | net | NOT-REGENERATED | no pristine output |
| `routes_forms/ref` | fcd, stat, tripinfo | IDENTICAL | |
| `routes_forms/net` | net | NOT-REGENERATED | no pristine output |
| `sigma_seed/ref` | fcd, stat, tripinfo | IDENTICAL | |
| `sigma_seed/ref_seed7` | fcd, stat, tripinfo | IDENTICAL | |
| `sigma_seed/net` | net | IDENTICAL | |
| `signal/ref` | fcd, stat, tripinfo | IDENTICAL | |
| `signal/ref_actstep` | fcd, stat, tripinfo | IDENTICAL | |
| `signal/ref_ballistic` | fcd, stat, tripinfo | IDENTICAL | |
| `signal/net` | net | IDENTICAL | |
| `signal/ref_step01` | fcd, stat, tripinfo | IDENTICAL | |
| `signal/ref_step05` | fcd, stat, tripinfo | IDENTICAL | |
| `signal_actuated/ref` | fcd, stat, tripinfo | IDENTICAL | |
| `signal_actuated/net` | net | IDENTICAL | |
| `signal_delay/ref` | fcd, stat, tripinfo | IDENTICAL | |
| `signal_delay/net` | net | IDENTICAL | |
| `signal_nema/ref` | fcd, stat, tripinfo | IDENTICAL | |
| `signal_nema/net` | net | IDENTICAL | |
| `stops/ref` | fcd, stat, tripinfo | IDENTICAL | |
| `stops/net` | net | NOT-REGENERATED | no pristine output |
| `stops_parking/ref` | fcd, stat, tripinfo | IDENTICAL | |
| `stops_parking/net` | net | IDENTICAL | |
| `stress/ref` | fcd, stat, tripinfo | IDENTICAL | |
| `stress/ref_collision` | fcd, stat, tripinfo | IDENTICAL | |
| `stress/ref_mdd5` | fcd, stat, tripinfo | IDENTICAL | |
| `stress/net` | net | NOT-REGENERATED | no pristine output |
| `stress/ref_ttt30` | fcd, stat, tripinfo | IDENTICAL | |
| `stress/ref_ttt_m1` | fcd, stat, tripinfo | IDENTICAL | |
| `sublanes/ref` | fcd, stat, tripinfo | IDENTICAL | |
| `sublanes/net` | net | IDENTICAL | |
| `teleport/ref` | fcd, stat, tripinfo | IDENTICAL | |
| `teleport/net` | net | IDENTICAL | |
| `trips/ref` | fcd, stat, tripinfo | IDENTICAL | |
| `trips/net` | net | NOT-REGENERATED | no pristine output |
| `vtypes/cf/acc` | tripinfo | IDENTICAL | |
| `vtypes/cf/cacc` | tripinfo | IDENTICAL | |
| `vtypes/cf/cc` | tripinfo | IDENTICAL | |
| `vtypes/cf/daniel1` | tripinfo | IDENTICAL | |
| `vtypes/cf/eidm` | tripinfo | IDENTICAL | |
| `vtypes/cf/idm` | tripinfo | IDENTICAL | |
| `vtypes/cf/kerner` | tripinfo | IDENTICAL | |
| `vtypes/cf/nasch` | tripinfo | IDENTICAL | |
| `vtypes/cf/pwag2009` | tripinfo | IDENTICAL | |
| `vtypes/cf/rail` | tripinfo | IDENTICAL | |
| `vtypes/cf/smartsk` | tripinfo | IDENTICAL | |
| `vtypes/cf/w99` | tripinfo | IDENTICAL | |
| `vtypes/cf/wiedemann` | tripinfo | IDENTICAL | |
| `vtypes/ref` | fcd, stat, tripinfo | IDENTICAL | |
| `vtypes/net` | net | NOT-REGENERATED | no pristine output |
| `zipper/ref` | fcd, stat, tripinfo | IDENTICAL | |
| `zipper/net` | net | IDENTICAL | |

## Not regenerated

- `<case>/ref_simple.net.xml` for `reroute`, `reroute_bypass_p1`, `route_check`, `routes_forms`, `stops`, `stress`, `trips`, `vtypes`:
  no `input.osm` and no `net.args`, so run_all has no rule to rebuild the net. Their sim refs were regenerated on the committed net and are compared above.
- `junction`, `roundabout`, `signal`: `ref.net.xml` (not `ref_simple`) is not produced by run_all; not covered here.
- Logs and stdout/stderr captures (`ref.log`, `ref*.stdout.txt`, `ref*.stderr.txt`) are not compared.

## Replaced in this commit series

- `real/ref_reroute`
