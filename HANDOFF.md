# Handoff: state of the sumo-rs port (2026-11-12, end of day)

> **2026-11-22:** the JuPedSim pedestrian model was deliberately dropped; see [NOT_PORTED.md](NOT_PORTED.md) for everything that is not ported and how to add it.

Read this first, then `AGENTS.md` (rules for every porting agent) and `PORTING.md` (conventions,
product goal, design notes). This file says where things stand and how to continue.

## 1. Where it stands

| Measure | Value |
|---|---|
| Skeleton methods filled, in `main` | **11,541 / 12,875 (89.6%)** |
| `todo!()` left in `main` | 1,334 across 164 files (about 300 of the original were C++ pure virtuals with no body) |
| Real methods still to translate | 1,334 (855 are assigned to the s01–s20 swarm; the rest are blocked, see section 5) |
| C++ lines in PORTING.md's "Done" table | 11.1% (48,367 / 434,394; only hand-ported `utils`/`foreign`/parts of others) |

`main` compiled (`cargo check -p sumo_core`, 0 errors) at the last merge. Nothing in `sumo_core` has been
compared against SUMO output yet: "filled" means translated, not correct. The C++ oracle is now built
(see section 2), so that comparison can start.

Run `python tools/progress.py` for live numbers. Set `SUMO_SRC` to the C++ checkout
(`SUMO_SRC=C:\Users\karti\sumo-src`), or it fails with a division by zero when run from a worktree.

**Complete and compiling in main:** NBEdge, NBNode, NBEdgeCont/NBNodeCont, NBNetBuilder,
NWWriter_SUMO / OpenDRIVE / XML (OSM -> net.xml pipeline), OSM / OpenDRIVE / VISSIM importers,
NLHandler, NLDetectorBuilder, NLJunctionControlBuilder, NLTriggerBuilder, MSLane, all
car-following models (Krauss, IDM, EIDM, ACC, CACC, Wiedemann, W99, rail, CC, Kerner, NaSch,
PWag2009, SmartSK), all lane-change models (LC2013, SL2015, DK2008, abstract), mesosim, routing
engine, ROPerson, route handlers, pedestrians (striping + bases), SSM, tripinfo, battery /
elecHybrid / charging stations, libsumo (incl. Person), OD demand.

**Not complete:** MSVehicle (20 todo left), MSDriveWay, KD-tree partition, intermodal network,
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
  **Trap:** a branch named `fixer-01` is silently ignored (the script does nothing and reports no
  error). Rename or alias it to `fix-01` before merging.
- **C++ oracle is built in WSL:** `~/sumo-src/bin/sumo` and `~/sumo-src/bin/netconvert` (SUMO v1.28.0).
  Run them with `SUMO_HOME=~/sumo-src` set, or they cannot find their data files.
- **No Rust binaries yet:** there is no `netconvert` or `sumo` binary in `crates/` yet, so there is
  nothing on our side to run against the oracle. The first end-to-end run (section 5) creates them.
- **Scope every search.** `find /` can pin 6 CPU cores for hours on this machine. Always limit
  `find`, `grep`, and `rg` to a specific directory (for example `crates/` or `C:\Users\karti\sumo-src\src`).
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

## 4. Branches (live: `git branch -r`, all on GitHub)

Checked on 2026-11-12 against `origin/main` (2f434e7). "Merged" means the branch tip is an ancestor
of `main`, so it is in. Branches that were squash-merged will show as unmerged even if their content is
in `main`, so check the log before re-running them.

**Merged into `main` (19):** `worktree-agent-a0620f0252262cc2e`, `a062a6d30d28945a7`,
`a25cc48b3edcf1bd1`, `a2e87633d16a31917`, `a825fde489b5fbf06`, `aa323bbc6f87fbf7c`,
`ab96b955b72a740db`, `ae4bb7c952b0b393c`, `af1558a9c4fa5fa2b`, `af16c4119b8749a5a`,
`worktree-agent-free01`, `free03`, `free08`, `worktree-agent-h01`, `h03`, `h04`, `h05`, `h06`, `h07`.

**Not merged (12 worktree branches, plus `fixer-01`):**

| Branch | Tip | Notes |
|---|---|---|
| `origin/fixer-01` | 101d315 | Not a `worktree-agent-*`, so `merge_agent.sh` ignores it (see section 2). Lead's merge branch. |
| `worktree-agent-a037eefaa380ba9a6` | 91091a2 | VISUM importer rest + rodf_detector |
| `worktree-agent-a2e3b61f8513670ad` | 4f4cf36 | nb_algorithms_railway + rodf_net (input to a71db91b) |
| `worktree-agent-a2fff790708cf98ef` | aaf6e13 | tail batch 03 (marouter done; bluelight, driver_state, fcd_replay in part 2) |
| `worktree-agent-a38a55cabb9087510` | 4e4a161 | rail signal constraint + control (input to a71db91b) |
| `worktree-agent-a71db91b240aab912` | 5e30538 | fixer: merges a2e3b61 + a38a55c; not yet 0 errors |
| `worktree-agent-a974f9bbc0e759c68` | 8aca9d5 | overhead wire, BT sender, BT receiver (input to ae38ef90) |
| `worktree-agent-ae38ef9020b511f9b` | 143dd64 | fixer: merges a974f9b; mid-fix (adding SumoRNG::seed) |
| `worktree-agent-aef93822c0f46f4db` | b764d14 | pc_loader_osm + ms_device_glosa (partly); intermodal_network blocked on router generics |
| `worktree-agent-aefe4385a3be9513c` | 50b7486 | MSDriveWay chain 3; partly done |
| `worktree-agent-af0d7653c8746ccb4` | bd91f2f | tail batch 03 part 2 |
| `worktree-agent-free06` | 5cea443 | |
| `worktree-agent-h02` | 19bf53b | |

Not started: tail batches **11, 12, 13, 14** (their first agents made no edits).
Old `fix-*` branches are superseded by later merges; ignore them.

## 5. What's left, in order

1. **Finish translation (agents, ~1-2 days):** 1,334 todos are left. Breakdown (these buckets overlap,
   so they do not sum to 1,334; they add to 1,382):
   - 855 are assigned to the s01–s20 swarm.
   - 393 are blocked on router generics (see step 2).
   - 86 are blocked on the SUMO vtable (see step 2).
   - 48 are blocked on GEOS/gRPC.
   Continue the unmerged branches in section 4, then tail batches 11-14. Re-scan with
   `Select-String` in PowerShell (`grep` is not on PATH here), or from Git Bash:
   `grep -rc 'todo!("port ' crates/sumo_core/src | grep -v ':0$'`.

   The four buckets add to 1,382, which is more than the 1,334 left, so either they overlap or they
   were counted at different times. Re-count before planning around them.
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
   oracle (WSL: `~/sumo-src/bin/{sumo,netconvert}`, `SUMO_HOME=~/sumo-src`) and fix differences. Focus on the product path first
   (OSM intersection -> net -> simulation) per `PORTING.md` "Product goal"; skip activitygen,
   marouter, dfrouter, rail, mesosim until that path matches.

Total estimate: ~2-3 weeks to a working, SUMO-matching core path; 3-6 weeks for everything.

## 6. Environment

- Repo: `C:\Users\karti\sumo-rs` (GitHub `kartikeytilavat6819-bot/sumo-rs`, private; keep it
  private). C++ source: `C:\Users\karti\sumo-src`.
- Commit messages end with a `Co-Authored-By:` line for the model used. Never force-push.
- Agents must not push; only the lead (via `merge_agent.sh`) pushes `main`.
