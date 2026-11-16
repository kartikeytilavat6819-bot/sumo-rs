# signal_nema: PASS

- Before the fix: the same pure-virtual panic as delay_based, since `NEMALogic` installed no vtable.
- run_all: net PASS, sim PASS (tripinfo, fcd, statistics) against C++ sumo.
- Horizon: same as signal_actuated. The C++ NEMA run differs from static on this window (11 FCD lines).
- Not covered: `activate/deactivateProgram` are not in the vtable (audit row 4).
