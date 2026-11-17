# Contamination fixes (branch worktree-agent-cm01)

Reference: official SUMO 1.28 at `~/sumo-src/src` (restored from tarball). Rust logic and the pasted `//` C++ blocks were reverted to official.

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
| 10 | ms_move_reminder.rs enum TELEPORT_CONTINUATION_ARRIVED | fixed | c3add684 |
| 11 | ms_mean_data_net.rs nVehLeft teleport check | fixed | ec313ff8 |
| 12 | ms_device_tripinfo.rs STATE_EMPTY / meso state / teleport reasons | fixed | 88c4f54c |
| 13 | ms_base_vehicle.rs sourcePos (reroute, not a getter) | fixed; ac01 merge-conflict risk | a2d15b08, 01d9ae4c |
| 14 | ms_base_vehicle.rs isTripInit (reroute, not a getter) | fixed; ac01 merge-conflict risk | a2d15b08 |
| 15 | ms_edge.rs predecessor updatePermissions loop | fixed | c5d22fb8 |
| 16 | ms_frame.rs + nl_builder.rs ignore.* options | fixed | fc5f1f0c |
| 17 | meso-lookahead option (ms_frame, ms_net, ms_simple_tll) | fixed (ms_net lookahead = 0 shortcut) | fc5f1f0c, c42357fd |
| 18 | ms_frame.rs tls.actuated.* options | audit-wrong: already matches official L509-516 | - |
| 19 | ms_net / MESegment lookahead queue choice | partial: option removed; MESegment lookahead logic remains. COORDINATED with mc01 (meso) | c42357fd |
| 20 | ms_state_handler.rs meso event counter, detector state | fixed for event counter and detector state. Meso save/load shortcuts remain (COORDINATED with mc01) | f93a4ea9, 41703f67 |
| 21 | detector saveState/loadState/getTimes (control, file output, induct loop) | fixed; official has none | 41703f67 |
| 22 | ms_actuated_traffic_light_logic.rs | fixed: placement clause, saveState override, loadExtraState body + loadTimeSinceLastDetection. Dispatch of loadExtraState is a shortcut (no vtable hook) | ec5e085f, 308e1ddd, 63f11528 |
| 23 | ms_drive_way.rs (whole file) | COORDINATED (rail agent rb01) | - |
| 24 | ms_rail_signal.rs lookBack | fixed | d7a1d808 |
| 25 | ms_rail_signal_control.rs deadlock subsystem | COORDINATED (rail agent rb01) | - |
| 26 | me_segment.rs lookahead queue choice / sidingDeadlock | COORDINATED (meso mc01 and rail rb01) | - |
| 27 | me_loop.rs ttt-based recheck | fixed | 8d68c851 |
| 28 | ms_device.rs DefaultAssignment cache | fixed | 42602360 |
| 29 | mslcm_sl2015.rs keep-space clause | fixed | 4ea3eba8 |
| 30 | nbpt_stop_cont.rs cleanupDeleted(reAssign) | fixed | 564896be |

## Gate

- `63f11528`: build OK, unit tests OK, depart 15/15. Oracle PASS=128 FAIL=4 SKIP=18.
- main 1e54f986 baseline: PASS=126 FAIL=6. Failing rows on branch (`city_kansas_rural`, `city_karlsruhe_tram`, `city_munich`, `emissions/ref_phemlight5`) are a subset of main's failing set. No regressions.
- Newly passing vs main: `city_bengaluru_blr`, `emissions/ref_phemlight`.
