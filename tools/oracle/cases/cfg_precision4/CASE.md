# cfg_precision4

Output-format option `--precision 4`: every numeric value in tripinfo, fcd and statistic output is written with four decimals instead of the default two. Fractional depart times and depart positions make the difference visible. One-lane 600 m edge `A`.

Sim: `--end 80 --precision 4`.

Compared by run_all: tripinfo, fcd and stat (exact). Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh cfg_precision4; rm NEW_CASE` (pristine C++ 1.28.0, WSL).
