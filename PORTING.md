# Porting status

Upstream baseline: eclipse-sumo/sumo @ 04695a8f0 (2026-11-11). 733k lines of C++ in `src/`.

Progress metric: share of `tests/sumo` texttest cases passing (`python tests/run_tests.py`).

## Conventions

- One Rust module per C++ file, same name in snake_case (`MSVehicle.cpp` -> `microsim/ms_vehicle.rs`), with a header line naming the C++ file it ports.
- Crates mirror `src/` directories: `utils`, `microsim`, `netload`, ... and binaries `sumo`, `netconvert`, ...
- C++ globals (gPrecision, DELTA_T, OptionsCont::getOptions) become module-level statics/thread-locals with getter/setter functions.
- Floating-point code keeps the C++ evaluation order so results stay bit-identical.
- Anything deliberately left out is marked `// shortcut: <limit>, <when to upgrade>`.

## Order of work

1. utils (common, options, geom, xml, iodevices, vehicle) - what the simulator needs
2. microsim + netload + sumo binary - first tests passing
3. lane changing, junctions, TLS, detectors, outputs - widen test coverage
4. mesosim, libsumo/TraCI
5. netbuild/netimport/netwrite (netconvert), routers, polyconvert, od, activitygen
6. GUI (sumo-gui, netedit) - needs a Rust GUI toolkit in place of FOX

## Done

| C++ | Rust | Notes |
|-----|------|-------|
| utils/common/SUMOTime | crates/utils/src/common/time.rs | |
| utils/common/RandHelper | crates/utils/src/common/rand_helper.rs | mt19937 ported by hand, verified against std value |
| utils/common/StdDefs | crates/utils/src/common/std_defs.rs | |
| utils/common/StringUtils (parsing) | crates/utils/src/common/string_utils.rs | partial |
| utils/common/ToString (doubles) | crates/utils/src/common/to_string.rs | partial |

## microsim ownership design (decided before porting microsim)

C++ microsim is a pointer graph (vehicle -> lane -> edge -> links -> lanes ...) mutated in phases.
Rust equivalent:

- `MSNet` owns flat arenas: `edges: Vec<MSEdge>`, `lanes: Vec<MSLane>`, `links: Vec<MSLink>`,
  `junctions: Vec<..>`, `vehicles: slab of RefCell<MSVehicle>`.
- Every C++ pointer becomes a typed index: `EdgeId(u32)`, `LaneId(u32)`, `LinkId(u32)`, `VehId(u32)`;
  nullable pointers become `Option<_>`. Network topology is immutable after loading.
- Per-step mutable state (vehicle state, lane vehicle lists, link approach info) lives in `RefCell`s,
  so the simulation step can hold `&MSNet` while each phase mutates one object and reads others,
  matching SUMO's phase structure (planMovements -> setJunctionApproaches -> executeMovements -> changeLanes).
- `MSNet::getInstance()` becomes an explicit `&MSNet` context parameter, not a global.
- Static config (`MSGlobals`, `OptionsCont`) stays global (read-mostly) via getter functions.

## Prior art (surveyed 2026-11-11)

No faithful SUMO port in Rust exists. Useful references:

- traci-rs (github.com/Fundacio-i2CAT/traci-rs, EPL-2.0): Rust TraCI client, to use as a test client for our TraCI server.
- A/B Street (github.com/a-b-street/abstreet): mature Rust traffic sim, typed-ID arenas like our design.
- sumo-types (crates.io): .net/.rou/.add.xml structs; maybe a cross-check for our loaders.
- Crates: quick-xml (SAX), flate2 (gzip), rstar (R-tree), proj (needs PROJ C lib), egui / iced / slint (GUI, to replace FOX).

## Method changes from LLM-rewrite research (2026-11-11)

Sources: luaur (github.com/pjankiewicz/luaur/blob/main/docs/TRANSLATION.md, 205k LOC C++ -> Rust),
Ladybird LibJS (ladybird.org/posts/adopting-rust), fish (fishshell.com/blog/rustport),
Intel IDEAS (github.com/IntelLabs/IDEAS), ACToR (github.com/kisp-nus/ACToR), Req2Rust (arxiv.org/html/2611.07442).

1. Skeleton first: generate all types + signatures from the clang AST (`todo!()` bodies), then fill
   bodies per symbol in dependency order; mutually recursive groups (e.g. MSVehicle/MSLane/MSLink) designed by hand.
2. Lockstep oracle: run real SUMO alongside with high `--precision` and fcd/netstate dumps; report the
   first diverging timestep + vehicle instead of only end-to-end texttest pass/fail.
3. Drift gate: reject `todo!()`, `unimplemented!()`, "elided" comments, dropped `mod` lines and
   weakened tests in modules marked done.
4. Run the oracle on Linux (WSL): expected files come from Linux; Windows libm differs for exp/ln/pow.
5. Determinism hazards to port explicitly: std::sort tie order (unstable introsort), std::set/map
   ordered by pointer, unordered_map iteration order.
6. RefCell-per-object design must be checked against lane-changing/junction code before porting it.
