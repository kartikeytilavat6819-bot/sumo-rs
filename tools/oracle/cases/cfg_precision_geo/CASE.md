# cfg_precision_geo

Output-format options `--fcd-output.geo true` (write lon/lat instead of local x/y in fcd) and `--precision.geo 3` (three decimals for geographic values instead of the default six). The net carries a real lat/lon projection (`projParameter="-"`, taken from `bike_guess_lane`); vehicles run on edge `10`.

Sim: `--end 70 --fcd-output.geo true --precision.geo 3`.

Compared by run_all: tripinfo, fcd and stat (exact). Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh cfg_precision_geo; rm NEW_CASE` (pristine C++ 1.28.0, WSL).
