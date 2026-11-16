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
