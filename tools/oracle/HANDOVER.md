references come ONLY from ~/sumo-pristine; ~/sumo-src is for instrumentation only

# Night run 2026-11-17 (lead loop, 20 Haiku agents, VM gate only) — read this first

- `main` went 151/5 → **429/7** (run_all PASS/FAIL; ~240 new rows added tonight, incl. 3-4k-vehicle cities), depart 15/15, unit tests ok, duarouter 16/16, netopts 188/188. All 7
  FAILs are NEW cases with owners (see tools/lead/agent_preamble.md). Every original city row passes (London/Kansas/
  Karlsruhe/Tokyo: isLeader→MSBitSetLogic dispatch, in01; Paris: computeFurtherLanes break, cb01). Release build gives
  identical verdicts and runs ~0.6x the C++ wall time (tools/oracle/PERF.md). New binary crate: duarouter (18/18 oracle
  cases PASS via tools/oracle/run_duarouter.sh; du03 extends it). State save/load roundtrip runner: tools/oracle/run_state.sh.
- Infra: Kaggle disabled (`gate.ps1.off`, dispatcher stopped). VM gate only: 2 × e2-custom-16-40960 spot (CPU quota
  32 total), 8 slots each, auto-retry on ssh drops / rustc ICE, VM chosen by fewest active runs, account pinned to
  personal Gmail. Official C++ 1.28.0 source built on the VMs at /home/gate/sumo-cpp (`tools/gcloud/vm_cpp_trace.sh`).
- Agent rules + current baseline: `tools/lead/agent_preamble.md`. Merge flow: finished branch → `merge_two.sh` into
  wave2 → `vm_gate.ps1 -Branch wave2` → if no previously passing row is lost, merge wave2 into main and push.
- Not merged on purpose: ls07 (duplicates tc03/ir01 fixes, 4 conflicts). ou04 is carried by pd02, na01 by na02.
- Free OpenCode models (glm/deepseek flash) worked for docs/cases but hit their usage limit; Haiku carries on.

## End of night run — all agents stopped (2026-11-17 morning)

Everything is on GitHub: `main`, `wave2` (integration branch) and every `worktree-agent-*` branch (unfinished
work included). Nothing lives only on the laptop.

**Pending in `wave2`, gated before merging to main** (if the last wave2 gate shows no PASS→FAIL vs main, merge
`wave2` into `main`): lo01 `16add678` (lc_opposite fixed: checkChange shadowed its targetLane parameter), bo01
`60f0e73d` (2 base-offset casts), vs04 `c141cd39` (importers 26/26: OpenDRIVE internal-lane reversal used the wrong
connection's toCP), sl07 `273dd4ea` (state roundtrip: taxi format, rerouter router, drive-way foe order, routeDist
ids), tr05 `43b843ed` (106 new TraCI infra probes MATCH; error texts, LaneArea offset). Also removed: leftover
LCTRACE trace code that lc06's merge had brought to main.

**Open FAIL rows — where to start (measured, not guessed):**
- `city_grenoble_tram_bus`: fixed by cd01 (`a2d29c79`, IntermodalEdge `Stop` kind so routes keep `!stop` edges), but
  that change breaks `ped_persontrip_public/ref` + `ref_carwalk` (fcd/stat differ, tripinfo MATCH). It was reverted on
  wave2 (`2fcd081a`). Find what C++ does differently for those personTrips, then re-apply (`git revert 2fcd081a`).
- `city_delft_campus_crossings`: car42 at junction :44808045, t=347 — C++ stops for person67 on the crossing, ours
  doesn't (cd01/cd02). Crossing blockedAtDist chain reads faithful; suspects: pedestrian state / walkingarea foe.
- `tls_nema_loops`: hang fixed (NEMALogic::init must call init_base). Remaining: at t=5 phase 6's E2 detector counts
  2 vehicles in ours, 0 in C++ → extension keeps GGrr (nm02). Check `lookup_e2_detector_by_id` (base2 offset hack —
  use `mse2_collector_of`) and `build_e2_detector` positions; also NEMAPhase::new leaves myExpectedDuration 0 (C++:
  minDuration).
- `city_rotterdam_rail_yard`: W2 signal (J2 link 3) at t=55 — C++ sees foe driveway J2.1 approached by fr1 and turns
  red; our J2.2 never gets foe J2.1 and build_sub_foe never runs for J2 pairs (rr01). Look at how C++ adds that foe
  in the bidi stage (`myBidi` / departure driveways).
- `city_seattle_downtown_hills`: the official C++ itself reads `foeExitLink->myConflicts[10]` of size 8 (undefined
  behaviour, 17 times) — the reference is not reproducible. Drop/replace this case; the bad index is stored on the
  recheck_set_request_information path (sw01).
- `city_phoenix_scale_grid` (PANIC in mslcm_lc2013.rs, 3k vehicles): px01 was tracing actuated-TL induction-loop
  assignment; check its branch `worktree-agent-px01` for its last notes.

**Bug classes that kept recurring (check these first in any new diff):** missing virtual dispatch (dozens of
fixes tonight); `std::sort` tie order (use `utils::common::std_sort`, see SORT_AUDIT.md); casts from a non-first
base (`base2`/`base3`) without the field offset (use the `*_of` helpers, UB_AUDIT.md "Base-offset casts");
references generated with different flags than run_all uses (always NEW_CASE + make_sim_refs.sh); objects never
freed (`Box::leak`) so C++ destructor side effects never run (lc06: arrived vehicles stayed in partial-vehicle lists).

**Infra notes:** VM disk fills from instrumented C++ copies (`/home/gate/cpp-<ID>`, 2.2 GB each) — vm_cpp_trace.sh
now prunes copies unused for 4 h; manual cleanup = delete `/home/gate/cpp-*` and `/tmp/score*`. `run_all` has a
per-case timeout (`SIM_TIMEOUT`, 600 s). The VMs are spot instances: restart a TERMINATED one with
`gcloud compute instances start sumo-gate2 --zone us-central1-b`.

# Handover — 2026-11-16 (end of session, everything stopped)

Start here in a new session. Then read `tools/lead/agent_preamble.md` (rules every agent gets),
`tools/oracle/SCOREBOARD.md` (case table) and `tools/oracle/LOOP.md` (procedure).

## State at stop

| | |
|---|---|
| `main` | **138 PASS / 2 FAIL** (run_all), depart 15/15, unit tests pass. FAIL: `city_kansas_rural` sim, `city_karlsruhe_tram` sim |
| `wave2` (`a156e504`) | main + rb01 rail fix. **Not pushed to main**: its only check ran on the buggy 4-slot VM gate. Re-gate on Kaggle first |
| Agents | all stopped; every branch committed and pushed to GitHub |
| Kaggle dispatcher | stopped |
| Google VMs `sumo-gate`, `sumo-gate2` | stopped (TERMINATED); `vm_gate.ps1` renamed to `.off` because it auto-starts VMs |
| Kaggle tokens | GitHub repo secrets `KAGGLE_TOKEN_KARTIKEY6819`, `KAGGLE_TOKEN_KARTIKEY5656`, `KAGGLE_TOKEN_KARTIKEYTILAVAT` (locally: `~/.kaggle/gate_token`, `gate_token_acct3`, `gate_token_acct1`) |

## First moves next session

1. Start the dispatcher (exactly one): `powershell -ExecutionPolicy Bypass -File tools\kaggle\gate_dispatcher.ps1 *> $env:TEMP\dispatcher.log` (background).
2. `tools\kaggle\gate.ps1 -Branch wave2` → if no row lost vs main, merge wave2 into main and push.
3. Merge the finished branches below (each via `tools/lead/merge_two.sh <branch>` into wave2, gate, push).
4. Re-launch agents on the in-progress branches, giving each `tools/lead/agent_preamble.md` + its row below.

## Branches

### Finished, ready to merge (gate each on the merged tree)
| Branch | sha | What |
|---|---|---|
| `worktree-agent-c10` | `f439b257` | 5 city cases: Amsterdam (bikes), Paris Chapelle (ramps), LA interchange PASS/PASS; Paris dense + Tokyo lefthand net PASS, sim FAIL (cb01 owns) |
| `worktree-agent-city-batch` | `85ab9c32` | Atlanta, Barcelona, Milton Keynes PASS/PASS; London net PASS, sim FAIL t=304 (in01 owns). Refs regenerated from pristine |
| `worktree-agent-dw01` | `933699cf` | option template audit: help-text fixes, netconvert random/seed options. Report of remaining template diffs in its commit/report |
| `worktree-agent-nc05` | `7b4fa58f` | already in main (permit-unsignalized default) — nothing left |

### In progress (each committed with its next step)
| Branch | sha | Status / next step |
|---|---|---|
| `worktree-agent-ir01` | `710e419b` | Intermodal router ported, **does not compile**: add `'static` bounds at `intermodal_network.rs:110` and `pedestrian_router.rs` 61–63, 84–85, 176–183 (E0310 from `Ptr::get_mut`). Then run case `persons` (PENDING marker) |
| `worktree-agent-pd02` | `9a1a2aec` | PedestrianRouter + MSNet::getPedestrianRouter ported on top of ir01; person vtable (ls07) merged; case `peds_crossing` (crossings/walkingareas). Blocked on ir01 compile. Then first personinfo/FCD diff in MSPModel_Striping/MSStageWalking |
| `worktree-agent-td01` | `428c7676` | initTransportableDevice, MS_CONTAINER_VT, rail-constraint state. Blocked on ir01 compile. Compare ours-load vs C++-load (C++ load != C++ straight); C++ fails to load its own container state (line 54) |
| `worktree-agent-ls07` | `2481a575` | MS_PERSON_VT complete (44 entries). TraCI 212/216 MATCH; remaining: person.moveToXY getPosition error (probe prints full error now), ~22 libsumo stubs |
| `worktree-agent-tc03` | `f06bdcb8` | changeLane effect, version string `SUMO 1.28.0`, TL state length error — fixed. Merge main into it done; full 294-probe harness (`--shard`) not yet run |
| `worktree-agent-mc02` | `1a7bb38b` | meso: ME_VEHICLE_VT (39) + ME_SUMO_VEHICLE_VT (84) complete, initDevices virtual fix; departSpeed=0/empty FCD not yet confirmed fixed; TEMP trace present |
| `worktree-agent-in01` | `03fcf2d4` | London t=304 veh 146: C++ held by link leader (foe 38/80 on `:107777_1_0`); `isLeader` fallback ported (`e5b43ead`); ISLR trace not yet read |
| `worktree-agent-ck01` | `83c883ef` | Kansas veh 56 t=127 released early from `:122135040_15_0`; in01 fix merged; next: getLeaderInfo trace t=126–127 |
| `worktree-agent-ck02` | `47af1f22` | Karlsruhe: first diff measured (veh 56 arrival speed t=113); trace code present |
| `worktree-agent-cb01` | `bd64e070` | sublane cases merged. Paris dense t=398 veh 3 angle 177.14 vs 322.82 (back position / further lanes); Tokyo veh 69 timeLoss, departDelay 1.30 vs 1.33 |
| `worktree-agent-sl01b` | `11a277fd` | junction/signal save/load exact. City scale: Manhattan load matches; Bengaluru veh 115 missing after load |
| `worktree-agent-em02` | `724e42ec` | edgeData emissions panic fixed (MSMeanData_Emissions vtable); edge attribute set differs from C++; run_all.sh failure detail fixed |
| `worktree-agent-ev01` | `d3f167a4` | battery + charging-station output match (merged); precision-6 MMPEVEM/Energy comparison not measured |

## Infrastructure

- **Kaggle gate** (`tools/kaggle/`): `gate.ps1 -Branch <b>` queues; `gate_dispatcher.ps1` (run ONE) batches 3 per notebook,
  4 notebooks × 2 accounts, persists in-flight batches in `~/.kaggle/gateq/_busy.json` (restart-safe).
  `batch_kernel.py` installs the eclipse-sumo 1.28.0 wheel (SUMO_HOME, `$SUMO_PRISTINE_BIN`) and runs a branch's
  `tools/kaggle/agent_run.sh` after the gate (10 min limit) — agents' way to run traces without local builds.
- **Google VM gate** (`tools/gcloud/`, paused): project `project-b17b0335-3081-4c61-89b`, account
  kartikeytilavat6819@gmail.com, welcome credit (~$233 left at start; spend cap we set: $50). Two spot
  e2-standard-16. **Known bugs in the 4-slot `vm_gate_run.sh` — fix before trusting results**: build log is a
  shared `/tmp/build.log` (use a per-slot name, `grep -a`); agents' `agent_run.sh` hard-code `/tmp/target` (use
  `$CARGO_TARGET_DIR`); results from the 4-slot version are not trustworthy. NEVER use the work account.
- **Local**: no Rust builds on the laptop (RAM exhausted with many agents). WSL C++ builds `make -j2` allowed.
- **Lead scripts**: `tools/lead/merge_two.sh` (merge branches into wave2, stop on conflict),
  `tools/lead/merge_wave2_main.sh`, `tools/lead/restart_dispatcher.ps1` (one-off recovery from a log).
- Agents: Haiku (`anthropic/claude-haiku-5-5`) via the local proxy, two Northeastern accounts, session affinity on.

## The bug classes that mattered (read before debugging)

1. **No virtual dispatch** (dominant): C++ virtual call through a base pointer reaches a Rust base no-op or
   `default_*` pure-virtual stub. This session: MSDriveWay move-reminder vtable (rail never held trains),
   MS_PERSON_VT (23 entries), MS_CONTAINER_VT, ME_VEHICLE_VT/ME_SUMO_VEHICLE_VT, MSMeanData_Emissions,
   MSRailSignalControl listener, `isLeader` stub. Check every `..DEFAULT_*_VT` fallthrough.
2. **Contamination**: Rust lines copied from a C++ tree agents had edited (`tools/oracle/CONTAMINATION.md`); all 30
   rows handled. Only `~/sumo-pristine` is an oracle.
3. **Pointer-ordered C++ containers** (`std::set<T*>`): use id/creation order with a `// determinism:` comment.
4. **Wrong option defaults** (`railway.signal.permit-unsignalized`): audit with `--save-template` on both sides.
5. **Off-by-one loop ranges** (`MSRailSignal::initDriveWays` inclusive end).
6. Hardcoded globals / constants standing in for options; ported components nothing calls.

## Harness lessons

- References only from pristine (`make_sim_refs.sh`, completeness-checked; WSL OOM truncates silently).
- Own `CARGO_TARGET_DIR` per tree; `STALE BINARY:` guard; `SCORE_OUT` per run.
- Background gate shells must not be piped into `Select-Object -First` (the agent never wakes up).
- One dispatcher only; duplicate dispatchers corrupt the queue.
- A failing new case is a success; never regenerate a reference to make a case pass.
