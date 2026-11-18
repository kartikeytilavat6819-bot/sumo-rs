# cfg_step2

Processing option `--step-length 2`: a two-second simulation step (existing cases cover 0.1, 0.25 and 0.5). Odd-numbered departures are snapped to the next step and fcd is sampled on even seconds only. One-lane 600 m edge `A`.

Sim: `--end 60 --step-length 2`.

Note: `make_sim_refs.sh` exits 1 at `check_ref_complete.py` because that checker assumes a 1 s step (fcd's last timestep is 58 s, below `end - 1`). The C++ outputs are written and XML-complete before that check. `compare_sim.py` / `compare_stat.py` handle the 2 s steps fine.

Compared by run_all: tripinfo, fcd and stat (exact). Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh cfg_step2; rm NEW_CASE` (pristine C++ 1.28.0, WSL).
