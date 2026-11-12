# Handoff: state of the sumo-rs port (2026-11-12, end of day)

Read this first, then `AGENTS.md` (rules for every porting agent) and `PORTING.md` (conventions,
product goal, design notes). This file says where things stand and how to continue.

## 1. Where it stands

| Measure | Value |
|---|---|
| Skeleton methods filled, in `main`, compiling | **10,625 / 12,875 (82.5%)** |
| `todo!()` left in `main` | 2,250 (about 300 are C++ pure virtuals with no body) |
| Real methods still to translate | about 1,950 (several hundred more already sit on the branches in section 4) |
| Finished **and verified against real SUMO** | 11.1% of C++ lines (only hand-ported `utils`/`foreign`) |

`main` compiles (`cargo check -p sumo_core`, 0 errors). Nothing in `sumo_core` has been run or
compared against SUMO yet: "filled" means translated and compiling, not correct.

Run `python tools/progress.py` for live numbers.

**Complete and compiling in main:** NBEdge, NBNode, NBEdgeCont/NBNodeCont, NBNetBuilder,
NWWriter_SUMO / OpenDRIVE / XML (OSM -> net.xml pipeline), OSM / OpenDRIVE / VISSIM importers,
NLHandler, NLDetectorBuilder, NLJunctionControlBuilder, NLTriggerBuilder, MSLane, all
car-following models (Krauss, IDM, EIDM, ACC, CACC, Wiedemann, W99, rail, CC, Kerner, NaSch,
PWag2009, SmartSK), all lane-change models (LC2013, SL2015, DK2008, abstract), mesosim, routing
engine, ROPerson, route handlers, pedestrians (striping + bases), SSM, tripinfo, battery /
elecHybrid / charging stations, libsumo (incl. Person), OD demand.

**Not complete:** MSVehicle (32 todo left), MSDriveWay, KD-tree partition, intermodal network,
the long tail of small files (sections 4 and 5).

## 2. How the pipeline works

- `crates/sumo_core` is a skeleton generated from the C++ AST (`tools/skeleton/`, do NOT
  regenerate: it would overwrite filled code). Every method has its C++ body as `//` comments
  above a `todo!("port X::y")`. Agents replace the `todo!()` with a faithful translation.
- **Writer agents** (cheap model, e.g. Haiku) fill `todo!()`s in an assigned file list. They
  never run cargo (20 agents sharing a build lock starve). Each works in its own git worktree
  and commits to its branch `worktree-agent-<id>`.
- **Fixer agents** merge `main` + one or more writer branches into their own worktree, run
  `cargo check -p sumo_core` with their OWN target dir (`CARGO_TARGET_DIR=.../sumo-rs-target-fixN`,
  `CARGO_BUILD_JOBS=4`, 16 GB machine) and fix compile errors until 0.
- **The lead** merges with `bash tools/merge_agent.sh <agent-id | branch>`: it takes a lock
  (`C:\Users\karti\sumo-rs-merge.lock`), commits leftover worktree edits, merges, compile-checks
  with `sumo-rs-target-main`, pushes on success; on build errors it saves the merge as branch
  `fix-<id>` for a fixer and resets main. Branch names it accepts: an agent id,
  `worktree-agent-*`, `fix-*`, `local-fill`. Alias anything else to `fix-<name>` first.
- **Chains** for huge files: each writer starts with `git merge --no-edit <previous writer
  branch>` and fills the next ~30 methods; every 2-3 rounds a fixer makes the chain compile
  ("checkpoint"), and the next writer starts from the fixer's branch. Worked well for
  NBEdge / NBNode / MSLane / MSVehicle.
- **Batching the tail:** `tools/batches/tail_NN.txt` lists small files (~80 todo!() per batch).
- Writer prompt template that worked: "Read AGENTS.md first. You are a WRITER agent: do NOT run
  cargo. Do the work yourself; do not spawn other agents. Your files: ... For each: grep -n
  'todo!(' to find it, Read ~80 lines around it, translate the C++ in the comment above it.
  Commit every ~8 methods. Final report under 80 words: methods done, todo!() left, changed pub
  signatures, branch." Fixer template: same, plus the exact cargo command and target dir.

## 3. Gotchas (learned the hard way)

- **cargo PATH:** in Git Bash, `export PATH="$HOME/.cargo/bin:$PATH"` before cargo, or use
  `C:/Users/karti/.cargo/bin/cargo.exe`. A missing cargo prints nothing if you pipe through grep:
  check `${PIPESTATUS[0]}`.
- **Usage limit:** the Haiku plan stops all agents every ~5 hours ("resets at HH:MM"). Agents keep
  their work; resume each with a message after the reset. Set a timer, or they sit idle.
- **Worktrees vanish:** an agent worktree with no edits is auto-removed when the agent stops.
  Before resuming, recreate it: `git worktree add .claude/worktrees/agent-<id> worktree-agent-<id>`
  (or `-b worktree-agent-<id> main` if the branch is gone).
- **Edits on main** (lead fixes): hold the merge lock (`until mkdir LOCK; do sleep 5; done`)
  while editing + compiling + committing, or a queued merge runs on your half-done tree.
- **Haiku agents sometimes spawn sub-agents** and leave work on other branches; prompts now say
  "do not spawn other agents".
- **Faithfulness beats "fixing" C++:** output is compared to real SUMO, so C++ bugs are ported
  as-is with a `// determinism:` note (example: `PositionVector::operator+` in
  ni_importer_open_drive.rs never advances its iterators; the port reproduces that).
- OpenCode's free models only work through the `opencode` CLI, not its API, and share one small
  daily quota; local vLLM (7B on an 8 GB GPU) produced too little to be worth it.

## 4. Unfinished branches (all pushed to GitHub)

Each was stopped by the usage limit. Uncommitted edits were committed as "WIP" (not compiled).
To continue one: resume/re-run a writer on that branch, then a fixer, then `merge_agent.sh`.

| Branch `worktree-agent-…` | Kind | Content / what's left |
|---|---|---|
| `a0620f0252262cc2e` | writer | MSVehicle chain 9 (starts from compile-clean `ac6ccc1798aad310d`); ~32 todo in ms_vehicle.rs at start |
| `aefe4385a3be9513c` | writer | MSDriveWay chain 3; ~43 todo at start, partly done |
| `a71db91b240aab912` | fixer | merges `a2e3b61f8513670ad` (nb_algorithms_railway + rodf_net, full) + `a38a55cabb9087510` (rail signal constraint + control, full); not yet 0 errors |
| `ae38ef9020b511f9b` | fixer | merges `a974f9bbc0e759c68` (overhead wire, BT sender, BT receiver, all full); mid-fix (adding SumoRNG::seed) |
| `aef93822c0f46f4db` | writer | pc_loader_osm + ms_device_glosa (partly); intermodal_network blocked on router generics |
| `a037eefaa380ba9a6` | writer | VISUM importer rest + rodf_detector (~20 rodf_detector methods left) |
| `a2fff790708cf98ef` + `af0d7653c8746ccb4` | writer | tail batch 03 (marouter done; bluelight, driver_state, fcd_replay in part 2) |
| `a25cc48b3edcf1bd1` | writer | tail batch 01 (partial) |
| `af16c4119b8749a5a` | writer | tail batch 02 (partial) |
| `ab96b955b72a740db` | writer | tail batch 04 (partial) |
| `aa323bbc6f87fbf7c` | writer | tail batch 05 (partial, netgen) |
| `af1558a9c4fa5fa2b` | writer | tail batch 06 (partial) |
| `a2e87633d16a31917` | writer | tail batch 07 (partial, DLR-Navteq loader) |
| `ae4bb7c952b0b393c` | writer | tail batch 08 (partial, RORouteDef) |
| `a825fde489b5fbf06` | writer | tail batch 09 (partial) |
| `a062a6d30d28945a7` | writer | tail batch 10 (partial, af_router) |

Not started: tail batches **11, 12, 13, 14** (their first agents made no edits).
Old `fix-*` branches are superseded by later merges; ignore them.

## 5. What's left, in order

1. **Finish translation (agents, ~1-2 days):** continue the branches above, tail batches 11-14,
   then re-scan: `grep -rc 'todo!("port ' crates/sumo_core/src | grep -v ':0$'`.
2. **Lead design work (~2-3 days), blocks the rest:**
   - **Virtual dispatch.** C++ virtual calls through a base pointer currently run the base
     version or panic (~300 pure virtuals). Plan: `PORTING.md` section "Known architecture gap:
     virtual dispatch". Hit by NBRouterEdge, SUMOVehicle/SUMOTrafficObject, MSPModel, VISSIM
     type parsers, PlanItem (ROPerson), MSMeanData, NBTrafficLightDefinition::compute.
   - **Router generics.** `utils/router` generics (E/V/N/L) have no trait bounds; ~30
     `// shortcut: needs router trait bounds` notes in sumo_abstract_router.rs /
     intermodal_edge.rs / ms_routing_engine.rs; kd_tree_partition.rs (25 todo) and
     intermodal_network.rs are blocked on it.
   - Leftover placeholders: grep `Unmapped` and `shortcut:` for the rest (`/ponytail-debt`
     lists all shortcuts).
3. **First end-to-end run (~3-7 days):** netconvert on a small OSM intersection, then
   `sumo` on the result; fix panics one by one.
4. **Match real SUMO (~2-4 weeks):** compare `.net.xml`, `tripinfo.xml`, `fcd.xml` against the C++
   build (`C:\Users\karti\sumo-src`) and fix differences. Focus on the product path first
   (OSM intersection -> net -> simulation) per `PORTING.md` "Product goal"; skip activitygen,
   marouter, dfrouter, rail, mesosim until that path matches.

Total estimate: ~2-3 weeks to a working, SUMO-matching core path; 3-6 weeks for everything.

## 6. Environment

- Repo: `C:\Users\karti\sumo-rs` (GitHub `kartikeytilavat6819-bot/sumo-rs`, private; keep it
  private). C++ source: `C:\Users\karti\sumo-src`.
- Commit messages end with a `Co-Authored-By:` line for the model used. Never force-push.
- Agents must not push; only the lead (via `merge_agent.sh`) pushes `main`.
