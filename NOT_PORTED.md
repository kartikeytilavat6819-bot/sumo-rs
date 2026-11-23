# What is not ported (and how to add it later)

This port reproduces Eclipse SUMO **1.28.0** (the official release, PyPI `eclipse-sumo==1.28.0`) and is checked
against the official binary's output, case by case. A few parts are deliberately **not ported**. Their official
test cases stay in the repo but are *parked*: the case directory under `tools/oracle/cases/<case>/` contains a
`PENDING` file whose first line says why, and the gate skips it. Delete the `PENDING` file to run a case again.

List every parked case and its reason:

```sh
for f in tools/oracle/cases/*/PENDING; do echo "$(basename $(dirname $f)): $(head -1 $f)"; done
```

## 1. JuPedSim pedestrian model (24 cases) — dropped, decision of 2026-11-22

**What it is.** [JuPedSim](https://www.jupedsim.org/) is an external pedestrian-simulation library
(Forschungszentrum Jülich). SUMO selects it with `--pedestrian.model jupedsim`; the C++ class
`MSPModel_JuPedSim` (`src/microsim/transportables/MSPModel_JuPedSim.cpp`, 1,366 lines, plus a 239-line header)
does not simulate pedestrians itself but drives the library through its C API.

**Why it is not ported.** The official wheel does not contain JuPedSim source; it ships two external shared
libraries next to the `sumo` binary (`eclipse_sumo.libs/`):

| Library | Version in the 1.28.0 wheel | Used for |
|---|---|---|
| `libjupedsim` | 1.4.2 (240 exported `JPS_*` symbols) | pedestrian dynamics (CollisionFreeSpeed, CollisionFreeSpeedV2, GeneralizedCentrifugalForce, SocialForce), journeys, geometry |
| `libgeos_c` | GEOS 3.x C API | building the walkable-area geometry (buffer, union, difference, convex hull, WKT) |

Reproducing the official output exactly requires the same floating-point behaviour as those libraries, so the
only practical faithful route is calling them, not rewriting them.

**Affected cases (24).** All under `tools/oracle/cases/tt_pedestrian_model_jupedsim_*`:
- 18 need the model: `bad_state_bug`, `crossing_*` (6), `geometry_check`, `geometry_generation`,
  `geometry_switching`, `identical_start_pos`, `rerouter_*` (3), `speed_change`, `speed_change_poly`,
  `speed_factor`, `tls`, `tls_backward`, `two_edges`, `vanishing_area`, `walking_area`, `waypoint_radius`,
  `waypoints` (see each case's `PENDING`).
- 6 additionally make the official binary itself segfault; those can never match and stay parked regardless.

**Where the Rust code is.** `crates/sumo_core/src/microsim/transportables/msp_model_ju_ped_sim.rs` (generated
skeleton, still stubs) and `msp_model_ju_ped_sim_legacy.rs`. All other pedestrian models (`striping`,
`nonInteracting`) are ported and tested.

**How to add it later (recommended plan, ~1–2 days).**
1. Add an FFI module (e.g. `crates/sumo_core/src/foreign/jupedsim/`) that loads `libjupedsim` and `libgeos_c` at
   runtime (`libloading`), resolving the path from an env var or from the official wheel's
   `eclipse_sumo.libs/` directory. Declare the ~70 `JPS_*` functions and ~40 GEOS functions that
   `MSPModel_JuPedSim.cpp` uses (simulation create/iterate/switch geometry, agents add/mark/remove/query,
   journeys and stages, geometry builder, the four model builders and per-agent parameter/state get/set).
2. Port `MSPModel_JuPedSim.cpp` function by function against the release source, keeping the exact call order.
3. Gate the model on the libraries being found; otherwise keep today's behaviour (the model is unavailable).
4. Delete the `PENDING` files of the 18 non-crashing cases and run them through the gate.

Consequence: a build of this port that wants JuPedSim needs those two shared libraries at runtime (Linux), exactly
like the official SUMO build does. A pure-Rust rewrite of JuPedSim + GEOS is possible but is weeks of work and would
likely drift in the last floating-point digits.

## 2. XSD schema validation (3 cases)

`--xml-validation` / `-X always` with schema files is not ported: `crates/utils/src/xml` has no schema
validator (Xerces grammar pool / schema loading). Cases: `tt_meta_xml_validation`, `tt_meta_xml_validation_net`,
`tt_meta_xml_validation_invalid_schema`. Adding it means porting Xerces' schema validation or binding a validator
that reports the same messages.

## 3. Cases the official binary itself cannot run (kept parked permanently)

The reference output is not reproducible because the official 1.28.0 binary crashes, hangs or reads memory out of
bounds on these inputs. They are not port gaps:
- official segfault: one case besides the 6 JuPedSim ones (see `PENDING`, "Official sumo segfaults");
- official deadlock with `--time-to-teleport` (see `PENDING`, "does not terminate");
- `city_seattle_downtown_hills`: official C++ reads `myConflicts` out of bounds (undefined behaviour);
- complex TraCI test `vehicle/moveToXY/no_permitted_route_lane`: official segfaults (exit 139), excluded in
  `tools/oracle/complex_suite.py`.

## 4. Known shortcuts inside ported code

Search the code for `shortcut:` comments; each names its limit and when to upgrade. Notable ones:
- `routingMode == 3` (FareModul fare-based intermodal routing) is not ported.

## 5. Adding new features on top of this port

The port is meant to stay behaviour-identical to SUMO 1.28.0, so the official-output gate keeps working:
- new features that change default behaviour should sit behind a new option (default off), so every existing
  official case still matches;
- keep porting conventions from `PORTING.md` / `AGENTS.md` (one Rust item per C++ item, C++ names, C++
  file:line in commit messages);
- run the gate (`tools/oracle/run_all.sh` and the tool oracles) before merging; a change that makes an official
  case differ is a regression unless the feature intends it, in which case document it here.
