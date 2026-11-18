# cfg_fcd_filter

FCD output option `--fcd-output.filter-edges.input-file`: the plain-text file `edges.txt` lists only `E0`, so fcd output contains cars on `E0` and drops the bicycles travelling on `E1`. The two-edge net `E0`/`E1` comes from `bike_contraflow`.

Sim: `--end 80 --fcd-output.filter-edges.input-file tools/oracle/cases/cfg_fcd_filter/edges.txt`.

Compared by run_all: tripinfo, fcd and stat (exact). Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh cfg_fcd_filter; rm NEW_CASE` (pristine C++ 1.28.0, WSL).
