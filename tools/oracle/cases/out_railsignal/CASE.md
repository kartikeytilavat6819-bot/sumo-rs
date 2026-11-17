# out_railsignal

`--railsignal-block-output` and `--railsignal-vehicle-output` on the `rail` net (single track with passing loop) with 9 trains in both directions.

Sim: `--end 600 --time-to-teleport -1`.

Compared by run_all: tripinfo, fcd, stat and `ref.<suffix>.xml` for railblock, railveh (compare_xml.py, exact).
Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh out_railsignal; rm NEW_CASE` (pristine C++ 1.28.0, WSL).
