# Stub audit: permissive placeholders in the movement path

Scope: `crates/sumo_core/src/microsim/{ms_edge,ms_lane,ms_link,ms_vehicle,ms_base_vehicle,ms_junction,ms_intersection,ms_lane_change_model*,lcmodels/*}.rs`.
Method: grep for bodies that are a bare `0`, `0.0`, `true`, `false`, `f64::MAX`, `SUMOTIME_MAX`, `Ptr::null()`, `-1`, or a `panic!("shortcut`. Each candidate was compared with the C++ in `sumo-src/src/microsim` and `lcmodels`.

Verdict key: **real** = the Rust body is a permissive placeholder and was ported; **faithful** = matches the C++ (a base default, or a fallback inside a real branch); **real, open** = permissive or panicking, not fixed.

| stub | verdict | action |
|---|---|---|
| `MSEdge::get_brutto_occupancy` (ms_edge.rs) | real (returned `0`) | ported from `MSEdge.cpp:1692` meso segment walk. Now reachable only in meso runs; no live Rust caller. |
| `MSLane::free_insertion` stop check (ms_lane.rs) | real (`panic!` on any vehicle with a stop) | replaced with `veh.get_next_stop().edge == *veh.get_current_route_edge()` (`MSLane.cpp:517`). |
| `MSBaseVehicle::get_insertion_checks` | real (`NONE`) | fixed earlier (commit 8f727d8) |
| `MSVehicle::ignore_red` | real (`true`) | fixed earlier |
| `MSBaseVehicle::has_influencer` | real (panic) | patched to `false` earlier |
| `MSLink::get_leave_time` | faithful | none |
| `MSLink::get_foe_length_behind_crossing`, `get_foe_conflict_size` | faithful | none |
| `MSLink::is_on_coming_ped` | faithful (`0` is the >90° branch) | none |
| `MSLane::get_crossing_index` | faithful (`-1` is the not-found return) | none |
| `MSVehicle::implicit_speed_remote` / `implicit_delta_pos_remote` | faithful (`0.0` = `myConsiderMaxDeceleration` off / `dist == max` branch) | none |
| `MSVehicle::get_curve_radius`, `get_distance_to_position`, `get_lane_index`, `next_link_priority`, `get_waiting_time_for`, `get_best_lane_offset`, `remaining_stop_duration` | faithful | none |
| `MSBaseVehicle::get_acceleration`, `get_right_side_on_edge`, `get_lateral_position_on_lane`, `get_time_loss_seconds`, `get_stop_duration` | faithful (C++ base returns `0`; `MSVehicle` overrides where it matters) | none |
| `MSEdge::get_minimum_travel_time`, `inc/dec_vaporization` | faithful | none |
| `MSAbstractLaneChangeModel::check_change_before_committing`, `get_shadow_direction`, `get_cooperative_help_speed` | faithful (base default / branch) | none |
| `MSBaseVehicle::get_emissions` (`0.0`) | real, open | C++ calls `PollutantsInterface::compute` when on road; out of movement path, not fixed |
| `MSBaseVehicle::get_elec_hybrid_current` (`0.0`) | real, open | C++ looks up `MSDevice_ElecHybrid`; out of movement path, not fixed |
| `MSBaseVehicle::get_harmonoise_noise_emissions` (`0.0`) | real, open | out of movement path, not fixed |
| `MSBaseVehicle::activate_reminders_on_reroute` (`0`) | unverified | C++ `MSBaseVehicle.cpp:1772`; check its tail before next audit |

Not yet reviewed: the bare-literal `ms_edge.rs` / `ms_lane.rs` hits flagged as `Ptr::null()` (mostly `Option` fallbacks) and the `lcmodels` `true`/`false` defaults.
