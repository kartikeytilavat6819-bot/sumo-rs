# Stale local MSGlobals accessors (sweep op01, 2026-11-16)

Bug class: a file-local `fn g_*()` that stands in for a C++ global with a hardcoded value or an
option re-read. No crash, no warning; the behaviour behind the global is silently off.

Facts checked against `~/sumo-src` (do not re-derive):
- C++ `MSGlobals` statics without an initialiser are zero-initialised. `gUsingInternalLanes` becomes
  true only because `MSFrame::setMSGlobals` sets it to `!no-internal-links` (ported, called from
  `NLBuilder`).
- `gComputeLC` is **not** "default true": it is false except inside `MSEdgeControl::changeLanes`,
  which sets it true before lane changing and false after (`MSEdgeControl.cpp:277/332`). The Rust
  `change_lanes` never set it, so every `compute_lc()` reader saw false all the time. Fixed.
- `gTaxiClasses` is the StdDefs global, seeded by `MSFrame` from `taxi.vclasses`.

Line numbers are from `main` at c4dcf59, before the sweep.

| file:line | accessor | old value | correct value | replacement | verdict |
|---|---|---|---|---|---|
| ms_full_export.rs:7 | g_using_internal_lanes | `false` | `!no-internal-links` (true by default) | MSGlobals::using_internal_lanes | **WRONG**, fixed |
| ms_mean_data.rs:13 | g_using_internal_lanes | `true` | same as above | MSGlobals::using_internal_lanes | wrong with `--no-internal-links`; file excluded (owner), not edited |
| ms_mean_data.rs:10 | g_use_meso_sim | `false` | `mesosim` option | MSGlobals::use_meso_sim | wrong under meso; excluded, not edited |
| mscf_model.rs:37 | g_compute_lc | `false` | true during lane changing | MSGlobals::compute_lc | **WRONG** during LC; excluded, not edited (see note) |
| mscf_model.rs:34 | g_semi_implicit_euler_update | `true` | `!step-method.ballistic` (+ action-step rule) | MSGlobals::semi_implicit_euler_update | wrong under ballistic; excluded |
| mscf_model.rs:40 | g_default_emergency_decel | `-1` | `default.emergencydecel` | MSGlobals::default_emergency_decel | wrong when option set; excluded |
| ms_leader_info.rs:10 | g_lateral_resolution (pub, prelude re-export) | `-1` | `lateral-resolution` | delegates to MSGlobals::lateral_resolution | **WRONG**, fixed. Reached ms_link.rs via the prelude: sublane foe checks never ran |
| ms_lane.rs:401 | g_lateral_resolution | `-1` | `lateral-resolution` | MSGlobals::lateral_resolution | **WRONG** with sublane; excluded (owner), not edited |
| ms_lane.rs:409/414 | g_emergency_insert / g_check_routes | option re-read | same | MSGlobals::emergency_insert / check_routes | equivalent; excluded |
| ms_lane_changer_sublane.rs:1641 | g_lateral_resolution | option re-read | same | MSGlobals::lateral_resolution | equivalent, replaced |
| mslcm_sl2015.rs:53 | g_lateral_resolution | option re-read | same | MSGlobals::lateral_resolution | equivalent, replaced |
| mslcm_sl2015.rs:89 | g_semi_implicit_euler_update | `!step-method.ballistic` | misses the action-step-length override | MSGlobals::semi_implicit_euler_update | **WRONG** when only `default.action-step-length` is set, fixed |
| mslcm_sl2015.rs:94 | g_allow_overtaking_right | option re-read | same | MY_ALLOW_OVERTAKING_RIGHT (MSAbstractLaneChangeModel static) | equivalent, replaced |
| mslcm_sl2015.rs:99 | g_lefthand | `false` | net `lefthand` attribute | MSGlobals::lefthand | **WRONG** on left-hand nets, fixed |
| ms_net.rs:61/64/67/70 | g_use_meso_sim, g_check_4_accidents, g_using_internal_lanes, g_weights_separate_turns | option re-reads | same | MSGlobals::* | equivalent, replaced |
| ms_net.rs (set_g_clear_state) | setter | no-op | sets gClearState | MSGlobals::set_clear_state | **WRONG** (flag never set during state reload), fixed |
| ms_edge_control.rs:469 | gComputeLC never set | — | true around changeLanes | MSGlobals::set_compute_lc | **WRONG**, fixed (biggest behaviour change) |
| ms_vehicle_type.rs:39 | g_action_step_length | delegates | — | MSGlobals::action_step_length | fine, inlined |
| ms_vehicle_type.rs:42 | g_use_meso_sim | `false` | `mesosim` | MSGlobals::use_meso_sim | wrong under meso, fixed |
| ms_vehicle_control.rs:15 | g_use_meso_sim | `false` | `mesosim` | MSGlobals::use_meso_sim | wrong under meso, fixed |
| ms_induct_loop.rs:17 | g_use_meso_sim | `false` | `mesosim` | MSGlobals::use_meso_sim | wrong under meso, fixed |
| ms_induct_loop.rs:20 | g_num_sim_threads | `1` | `threads` | MSGlobals::num_sim_threads | wrong with `--threads`, fixed |
| ms_device_to_c.rs:8 | g_use_meso_sim | `false` | `mesosim` | MSGlobals::use_meso_sim | wrong under meso, fixed |
| ms_device_taxi.rs:8 | g_use_meso_sim | `false` | `mesosim` | MSGlobals::use_meso_sim | wrong under meso, fixed |
| ms_device_taxi.rs:16 | g_taxi_classes (+setter) | private thread_local, `0` | StdDefs gTaxiClasses (`taxi.vclasses`) | std_defs::g_taxi_classes | **WRONG**: device and router used separate copies, fixed |
| ms_device_fcd.rs:30/34 | g_use_meso_sim, g_sublane | option re-reads | same | MSGlobals::* | equivalent, replaced |
| ms_stage_trip.rs:14 | g_use_meso_sim | option re-read | same | MSGlobals::use_meso_sim | equivalent, replaced |
| ms_link.rs:15-85 (6) | — | — | — | already delegate (0868319) | done by another agent |
| ro_edge.rs:311 | g_weights_random_factor | option re-read | StdDefs gWeightsRandomFactor (not ported) | — | equivalent; left, router-side |
| intermodal_network.rs:93 | g_taxi_classes_perm | calls std_defs | — | — | already correct |

## Note for the mscf_model owner

With the three `mscf_model.rs` stubs replaced by `MSGlobals::*` (and the gComputeLC fix above),
`signal sim` flipped FAIL -> PASS in run_all (measured in this sweep, then reverted because the file
is owned elsewhere). The stub `g_compute_lc() -> false` is the likely cause: `get_secure_gap` and the
emergency-decel branch in `MSCFModel` take a different path during lane changing.
