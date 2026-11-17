# busline

Bus line `L1` on a 3-edge line (E0 E1 E2), two `busStop`s (`bs0` on E1, `bs1` on E2) from `busline.add.xml`.
`bus0` stops at `bs0` for 15 s and at `bs1` until t=120; `bus1` departs at 60 s with 10 s and until t=200.
Four cars share the line. Sim: `--end 250 --additional-files .../busline.add.xml`.
Refs: `make_sim_refs.sh busline` (pristine C++ 1.28.0).
