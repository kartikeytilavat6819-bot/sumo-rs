# busride

Bus line `L1` on a 3-edge line (E0 E1 E2) with busStops `bs0` (E1) and `bs1` (E2), from `busride.add.xml`.
Synthetic, plain node/edge net (`net.args`).

- `L1f`: a `<flow line="L1">` of buses every 100 s until t=300. Stops: `bs0` until 30 duration 15, `bs1` until 200.
  A stop needs an absolute `until` to enter the intermodal PT schedule; without it the line is dropped.
- Cars `cars`: a car flow on E0 E1 E2.
- Persons: `pw0`, `pw1`, `pw2` walk from E1 to `bs0` and ride to `bs1` (`pw1` walks on to E2). Their ride completes,
  so they appear in personinfo. `pr0` rides from E1 without walking to the stop, so it never boards: a deliberate
  no-board check.

Sim: `sim.args` = `--end 400 --additional-files .../busride.add.xml`. Extra outputs: `sim_outputs.txt`
(`--personinfo-output`, `--stop-output`), compared against `ref_sim_out/`.

Refs: `bash tools/oracle/make_pt_refs.sh busride` (pristine C++ 1.28.0, WSL).
