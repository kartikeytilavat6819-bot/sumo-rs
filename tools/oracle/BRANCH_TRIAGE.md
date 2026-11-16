# Branch triage (2026-11-16, agent br01)

Procedure: `tools/oracle/UNMERGED_BRANCHES.md`. Candidates: `worktree-agent-a*` (hex), `fix-*`, `local-fill*`
with `git rev-list --count main..BR` > 0. Result: 24 candidates (8 `fix-*`, 16 `worktree-agent-*`).
`local-fill` and the other `fix-*` names (`fix-free01`, `fix-wave2`, `fix-nbnode-final`) have 0 commits
not in main, so they are skipped. `fixer-01` and `fixer-nb-node` do not match the pattern.

Method, per branch:
- files = `git diff --name-only main...BR`.
- For each file, compare function bodies on `main` and on `BR`, ignoring `//` comment lines.
  A "stub" is a line with `todo!(`, `unimplemented!(`, `panic!("shortcut`, or `panic!("...not yet/not ported`.
- FILL = a fn that is a stub on main, exists on BR, and has no stub on BR.
- REGRESS = a fn that is a stub on BR but has a body on main (branch is behind main).
- NEW = a fn that exists on BR but nowhere in that file on main.

Main state: no `todo!(` outside comments in `crates/sumo_core`. Main still has one `panic!("shortcut`
inside `libsumo/vehicle.rs::add`, and stubs in the files the big `fix-*` branches touch (8 files for
`fix-a2404`/`fix-a498`, 40 for `fix-a60ca`). No branch fills any of them.

Result: 0 USEFUL, 12 UNKNOWN, 12 SUPERSEDED.

| branch | commits | files | class | functions it would add | notes |
|---|---|---|---|---|---|
| worktree-agent-a721edd79098f0ecd | 10 | 2 | UNKNOWN | 17 new fns in `ms_drive_way.rs` (`dw_lookup`, `dw_switch_drive_ways`, `dw_route_lookup`, `get_id`, `junction_foes_link`, ...) and `ms_rail_signal.rs::set_requested_drive_way` | Main has full drive-way bodies; branch has 43 stubs there. Helper names look branch-local; check against C++ before porting. |
| worktree-agent-aefe4385a3be9513c | 15 | 2 | UNKNOWN | 21 new fns in `ms_drive_way.rs` (`set_contains_numerical_id`, `drive_way_numerical_id_set`, `dw_*`, `lane_ids_joined`, `edge_ids_joined`, `junction_foes_link`) | Same file family as a721; branch has 11 stubs where main has bodies. |
| worktree-agent-a974f9bbc0e759c68 | 5 | 3 | UNKNOWN | 9 new fns in `ms_device_b_treceiver.rs`, `ms_device_b_tsender.rs`, `ms_overhead_wire.rs` (`init_update_once`, `copy_vehicle_state`, `int_of`, `typed_device_or_null`, `r_has`, `r_at`, `s_vehicles_has`, `s_vehicles_at`, `execute`) | Branch file is behind main. Helper names look branch-local. |
| worktree-agent-ae38ef9020b511f9b | 7 | 4 | UNKNOWN | same 9 fns as a974 | Also touches `utils/common/rand_helper.rs`, which a974 does not. Check that diff. |
| worktree-agent-a3d7b9c0bb2b46960 | 2 | 2 | UNKNOWN | `copy_vehicle_state`, `int_of`, `execute` (`ms_overhead_wire.rs`) | Branch has 12 stubs where main has bodies. |
| worktree-agent-a71db91b240aab912 | 19 | 4 | UNKNOWN | `rodf_net.rs::route_heap_pop/route_span/route_heap_push`; `ms_rail_signal_control.rs::edge_vector_to_string/sim_step` | Last commit is "WIP: paused at usage limit". Uncommitted work in that worktree is not in this branch. |
| worktree-agent-a2e3b61f8513670ad | 15 | 2 | UNKNOWN | `rodf_net.rs::route_heap_pop/route_span/route_heap_push` | Commit subject "Fill rodf_net (part 6)". Main has no route-heap helpers. |
| worktree-agent-a38a55cabb9087510 | 1 | 2 | UNKNOWN | `ms_rail_signal_control.rs::edge_vector_to_string`, `sim_step` | Small. Check `sim_step` against C++ `MSRailSignalControl`. |
| worktree-agent-a037eefaa380ba9a6 | 6 | 2 | UNKNOWN | `rodf_detector.rs::delete_detector/get_device_ptr/delete_distributor/copy_route_cont/delete_route_cont`; `ni_importer_visum.rs::must/prefix_bytes/throw_error/lp_get` | Commit subject "WIP: paused at usage limit". |
| worktree-agent-aed32ea227acfc908 | 2 | 1 | UNKNOWN | `ni_importer_visum.rs::must/prefix_bytes/throw_error/lp_get` | Same four helpers as a037. |
| worktree-agent-a18f3c6bbd8763597 | 1 | 1 | UNKNOWN | 11 fns in `ms_device_tripinfo.rs` (`lateral_resolution`, `avg_f`, `avg_t`, `take_token`, `fmt_fixed`, `device_list_string`, `statics`, `use_meso_sim`, ...) | Several look like test and format helpers (`*_match_cpp_format`, `*_defaults_like_cpp_stream_extraction`). Check for `#[cfg(test)]`. |
| worktree-agent-aef93822c0f46f4db | 2 | 2 | UNKNOWN | `ms_device_glosa.rs::phase_state_char`; `pc_loader_osm.rs::delete_ptr`, `msg_instance` | Branch has 2 stubs where main has bodies in `glosa`. |
| fix-a60ca9556cae1357c | 1 (merge) | 344 | SUPERSEDED | 13 new fns (not listed). Its `libsumo/vehicle.rs::add` is a `panic!("... not yet ported")` stub replacing main's real body. | Would regress 3,674 spots. Also carries `AGENTS.md` and `tools/local_fill.py`. Do not merge. An earlier "fill" of `add` was a false positive, caught by the stub-aware check. |
| fix-a2404ac0bfd4c7fc0 | 1 (merge) | 32 | SUPERSEDED | none | Main has stubs in 8 of these files; branch fills none. Branch is behind main (211 regress). |
| fix-a498d714ba555444f | 1 (merge) | 32 | SUPERSEDED | none | Same file list and results as fix-a2404. |
| fix-a378aaf1aee25a2df | 1 | 1 | SUPERSEDED | none | `nb_node.rs`; branch is behind main (87 regress). |
| fix-a4fa9a39e2dfe8cd7 | 1 | 1 | SUPERSEDED | none | `nb_edge.rs`; branch is behind main (173 regress). |
| fix-a7f04e2955ce6dfa5 | 1 | 2 | SUPERSEDED | none | `me_segment.rs`, `me_vehicle.rs`; branch is behind main (16 regress). |
| fix-aaca63208867bfe46 | 1 | 1 | SUPERSEDED | none | `ms_vehicle.rs`; branch is behind main (149 regress). |
| fix-ab5de0c78731e6d1c | 1 | 1 | SUPERSEDED | none | `nb_node.rs`; branch is behind main (61 regress). |
| worktree-agent-a093b56da517de8ba | 3 | 2 | SUPERSEDED | none | `ms_drive_way.rs`, `ms_rail_signal.rs`; branch is behind main (84 regress). |
| worktree-agent-a0dbb51997553e84b | 2 | 2 | SUPERSEDED | none | `rodf_net.rs`, `nb_algorithms_railway.rs`; branch is behind main (67 regress). |
| worktree-agent-a2fff790708cf98ef | 2 | 4 | SUPERSEDED | none | `marouter/*`; branch is behind main (6 regress). |
| worktree-agent-af0d7653c8746ccb4 | 3 | 4 | SUPERSEDED | none | `marouter/*`; same files as a2fff. |

Notes:
- SUPERSEDED means "the branch fills no stub that main still has". It does not mean the branch is safe
  to merge. Most of these branches are behind main, so merging would regress it.
- UNKNOWN needs a C++ check per new function. Whether a branch-added fn corresponds to a real C++
  method missing from main was not verified in this pass.
- The WIP branches (`a71db`, `a037`) have uncommitted work in their worktrees that this triage does
  not see.
- Nothing merged, deleted, or pushed.
