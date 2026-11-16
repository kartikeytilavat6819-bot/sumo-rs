# Contamination fixes (branch worktree-agent-cm01)

Reference: official SUMO 1.28 at `C:\Users\karti\sumo-src\src` (restored from tarball). Rust code and the pasted `//` C++ blocks were both reverted to official.

| # | Row (file: item) | Status | Commit |
|---|---|---|---|
| 1 | ms_lane.rs saveState (approaching for rail junctions) | already fixed (st01 cherry-pick) | 5dfae063 |
| 2 | ms_lane.rs insertion: sidingDeadlock check | fixed | 2f255a0d |
| 3 | ms_lane.rs updateInternalLanePermissions | fixed | 31440c09 |
| 4 | ms_lane.rs getLogicalPredecessorLane bidi clauses | fixed | 55282210 |
| 5 | ms_lane_changer.rs getLastBlocked empty check | fixed | fefcc999 |
| 6 | ms_vehicle.rs vMustReverse NUMERICAL_EPS -> POSITION_EPS | fixed | 41ed4c53 |
| 7 | ms_vehicle.rs checkReversal planningToStop guard | fixed | 41ed4c53 |
| 8 | ms_vehicle.rs neededFutureRoute formula | fixed | f5b2b1b7 |
| 9 | ms_vehicle_transfer.rs leaveLane notification | fixed | 9ac0a9bc |
| 10 | ms_move_reminder.rs enum TELEPORT_CONTINUATION_ARRIVED | fixed (enum now 19 values; drive_way name arm removed) | c3add684 |
| 11 | ms_mean_data_net.rs nVehLeft teleport check | fixed | ec313ff8 |
| 12 | ms_device_tripinfo.rs STATE_EMPTY / meso state / teleport reasons | fixed | 88c4f54c |
| 13 | ms_base_vehicle.rs sourcePos (reroute, not a getter) | fixed; ac01 merge-conflict risk, flagged | a2d15b08 |
| 14 | ms_base_vehicle.rs isTripInit (reroute, not a getter) | fixed; ac01 merge-conflict risk, flagged | a2d15b08 |
| 15 | ms_edge.rs predecessor updatePermissions loop | fixed | c5d22fb8 |
| 16 | ms_frame.rs + nl_builder.rs ignore.routeelements / ignore.additionalelements | fixed | fc5f1f0c |
| 17 | ms_frame.rs / ms_net.rs / ms_simple_traffic_light_logic.rs meso-lookahead option | fixed (ms_net lookahead = 0 shortcut; MESegment logic remains, see 26) | fc5f1f0c, c42357fd |
| 18 | ms_frame.rs tls.actuated.* options | audit-wrong: Rust already matches official L509-516 | - |
| 19 | ms_net.rs MSNet lookahead assignment | partial (see 17) | c42357fd |
| 20 | ms_state_handler.rs meso counter / detector / queue state | NOT DONE | - |
| 21 | ms_detector_control / ms_detector_file_output / ms_induct_loop saveState/loadState/getTimes | NOT DONE | - |
| 22 | ms_actuated_traffic_light_logic.rs placement clause | fixed (ec5e085f). saveState/loadExtraState override missing in Rust: NOT DONE, needs port | ec5e085f |
| 23 | ms_drive_way.rs (whole file, incl. getExistingDriveWay caller in ms_rail_signal) | NOT DONE | - |
| 24 | ms_rail_signal.rs lookBack | fixed | d7a1d808 |
| 25 | ms_rail_signal_control.rs deadlock subsystem / myMaxTrainLength | NOT DONE (needs findDeadlockFoes/addDWDeadlockChecks port, depends on 23) | - |
| 26 | me_segment.rs lookahead queue choice, addDownstreamFollowers, sidingDeadlock | NOT DONE (depends on 23) | - |
| 27 | me_loop.rs ttt-based gridlock/teleport recheck | fixed | 8d68c851 |
| 28 | ms_device.rs DefaultAssignment cache | fixed | 42602360 |
| 29 | mslcm_sl2015.rs keep-space reverse-gap clause | fixed | 4ea3eba8 |
| 30 | nbpt_stop_cont.rs cleanupDeleted(reAssign) | fixed (callers updated) | 564896be |

## Gate

- `88c4f54c` (after rows 1-12): build ok; oracle PASS=118 FAIL=2 SKIP=18 (`city_munich` net, `real/ref_reroute` sim); depart 15/15; unit tests pass.
- Baseline gate on `main` run to decide whether the 2 oracle failures are new.
