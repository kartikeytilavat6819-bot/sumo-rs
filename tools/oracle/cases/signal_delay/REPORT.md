# signal_delay: PASS

- Before the fix: `MSTrafficLightLogic::getCurrentPhaseDef is pure virtual`, because `MSDelayBasedTrafficLightLogic` never installed its vtable and its `init` recursed through the base.
- run_all: net PASS, sim PASS (tripinfo, fcd, statistics) against C++ sumo.
- Horizon: same as signal_actuated (`--end 200`). The C++ delay_based run differs from static on this window (11 FCD lines).
- Not covered: `getTLQueueLength`, `getDetectorState` and `activate/deactivateProgram` are not in the vtable (audit rows 3, 4, 6).
