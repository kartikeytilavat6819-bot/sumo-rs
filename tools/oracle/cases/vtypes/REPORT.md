# vType / car-following robustness — junction net

Net: `tools/oracle/cases/junction/ref_simple.net.xml`. Routes: `routes.rou` (2 vehicles per model, route `r0`, departs 0–115 s). Additional file: `vtypes.add.xml`. Run with `--end 120`.

Reference: C++ SUMO v1.28.0 (WSL, `run_ref.sh`). Ours: `C:\Users\karti\sumo-rs-target-main\debug\sumo.exe`.

## Result

| Model | Ours (`--additional-files`) | Ours (inline vType, `routes_inline.rou`) | Ref arrival / duration (s) | Verdict |
|---|---|---|---|---|
| Krauss | crash: `ShapeHandler::loadFiles: XMLSubSys::runParser not wired` | crash: `SUMOSAXHandler::getFileName not ported` | 24.00 / 25.00 | crash |
| IDM | same crash | same crash | 27.00 / 27.00 | crash |
| EIDM | same crash | same crash | 27.00 / 27.00 | crash |
| Wiedemann | same crash | same crash | 24.00 / 24.00 | crash |
| W99 | same crash | same crash | 24.00 / 24.00 | crash |
| ACC | same crash | same crash | not arrived by t=120 | crash (no ref data) |
| CACC | same crash | same crash | not arrived by t=120 | crash (no ref data) |

## Findings

- Every model fails before any vehicle is simulated, so **no per-model behaviour was measured**. The crash is in the loader, not in the car-following code.
- Without `--additional-files`, ours fails with `The vehicle type 'Krauss' ... is not known` (expected, as the vType is missing).
- Control: the repo's plain `routes.rou` (default vType, no vType file) runs to completion on ours (5 vehicles arrive). So the base simulation loop works; only vType loading is blocked.
- Ref ACC/CACC vehicles depart at 100–115 s, so they could not arrive before `--end 120`. The route design is too tight for those two models. Use a longer `--end` for them.

## Which models are usable today

None, from this test. Krauss, IDM, EIDM, Wiedemann, W99, ACC and CACC are all blocked by the unported vType and route-file loading path. The car-following models have not been tested individually.

## Per-model run (branch `worktree-agent-cf04`)

Vehicle-type dispatch is wired: all 13 non-Krauss models (IDM, EIDM, ACC, CACC, CC, Wiedemann, W99, Kerner, NaSch, PWag2009, SmartSK, Rail, Daniel1) install their own `MSCFModelVt` in the constructor. Each vtable carries that model's overrides of `stop_speed_2` (C++ `stopSpeed`, pure virtual in the base, overridden by all 15 concrete models) and the other C++ overrides. Krauss, KraussOrig1, KraussPS and KraussX were already wired. Krauss-family behaviour is unchanged.

Input: `cf/<model>.add.xml` (one vType per model, same attributes as `vtypes.add.xml`; CC gets `lanesCount="2"`) and `cf/<model>.rou.xml` (junction `routes.rou` with every vehicle on that vType). The shared `routes.rou` does not assign any vType, so it cannot be used for per-model runs. Model tags are the C++ `SUMOXMLDefinitions` names: Kerner is `BKerner`, PWag2009 is `PWagner2009`.

| Model | Ours: `sumo.exe --additional-files` | Dispatch test (base pointer) | C++ durations, 5 vehicles (s) | Matches C++ |
|---|---|---|---|---|
| IDM | crash: `ShapeHandler::loadFiles: XMLSubSys::runParser not wired` | pass | 13, 23, 28, 27, 31 | not measured |
| EIDM | same crash | pass | 13, 23, 29, 27, 32 | not measured |
| ACC | same crash | pass | 10, 23, 28, 28, 30 | not measured |
| CACC | same crash | pass | 10, 23, 28, 24, 28 | not measured |
| CC | same crash | pass | 11, 24, 25, 29, 37 | not measured |
| Wiedemann | same crash | pass | 10, 23, 28, 28, 30 | not measured |
| W99 | same crash | pass | 10, 23, 28, 35, 31 | not measured |
| Kerner (`BKerner`) | same crash | pass | 20, 23, 42, 48, 45 | not measured |
| NaSch | same crash | pass | 13, 27, 26, 27, 35 | not measured |
| PWag2009 (`PWagner2009`) | same crash | pass | 12, 23, 29, 26, 30 | not measured |
| SmartSK | same crash | pass | 11, 328, 328, 338, 337 | not measured |
| Rail | same crash | pass | 11, 23, 30, 28, 32 | not measured |
| Daniel1 | same crash | pass | 12, 24, 26, 31, 30 | not measured |

- **Runs:** 0 of 13. Every `--additional-files` run stops in `ShapeHandler::load_files` before any vehicle is simulated. The inline-vType route file hits a different unported step (`SUMOSAXHandler::getFileName not ported`). The control run without a vType file fails too (`The vehicle type 'Krauss' ... is not known`). This is the loader, not car-following code.
- **Dispatch test:** `mscf_model::tests::wired_models_report_their_own_id_through_base_pointer` builds each of the 13 models, casts it to `Ptr<MSCFModel>`, and checks `get_model_id()` against the model's C++ tag. It passes. `stop_speed_2` is not directly exercised by a test; its wiring is verified by compilation and by the table entries only.
- **Durations:** the C++ values above come from `run_cf_models.sh` (WSL, junction net, 5 vehicles per model on `cf/<model>.rou.xml`). Our durations cannot be compared until the loader is wired. The first table's C++ numbers (e.g. IDM 27.00) come from `vtypes/routes.rou`, which has 2 vehicles per model, so they are a different input and not comparable to the values above.
