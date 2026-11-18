# cfg_human_time

Output-format option `--human-readable-time true`: written times use `H:MM:SS.ss` instead of plain seconds in tripinfo, fcd and statistic output. One-lane 600 m edge `A`.

Sim: `--end 80 --human-readable-time true`.

Compared by run_all: tripinfo, fcd and stat (exact). Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh cfg_human_time; rm NEW_CASE` (pristine C++ 1.28.0, WSL).

Note: `make_sim_refs.sh` exits 1 at `check_ref_complete.py` because that checker does `float(arrival)` and HRT writes `00:00:43`; the C++ outputs themselves are written and XML-complete before that point. `compare_sim.py` / `compare_stat.py` handle non-numeric time strings via string equality, so the gate can still compare this case.
