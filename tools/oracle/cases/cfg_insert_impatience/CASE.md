# cfg_insert_impatience

Insertion/impatience options combined: `--eager-insert`, `--max-depart-delay 8` and `--time-to-impatience 15` on a 25-vehicle burst that cannot all be inserted at once. Existing cases cover these options separately on other nets; this one isolates their interaction. One-lane 600 m edge `A`.

Sim: `--end 100 --eager-insert --max-depart-delay 8 --time-to-impatience 15`.

Compared by run_all: tripinfo, fcd and stat (exact). Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh cfg_insert_impatience; rm NEW_CASE` (pristine C++ 1.28.0, WSL).
