# cfg_begin_end

Processing option `--begin 100` together with `--end 160`: simulation starts at 100 s, so a vehicle scheduled at 20 s and one at 100 s plus a flow between 110 and 150 s are handled relative to the shifted horizon. One-lane 600 m edge `A`.

Sim: `--begin 100 --end 160`.

Compared by run_all: tripinfo, fcd and stat (exact). Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh cfg_begin_end; rm NEW_CASE` (pristine C++ 1.28.0, WSL).
