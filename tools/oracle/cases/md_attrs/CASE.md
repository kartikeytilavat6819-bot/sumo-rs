# md_attrs

`writeAttributes` subsets: traffic aggregates (`timeLoss flow distance`), vehicle counters (`departed arrived entered left laneChangedFrom laneChangedTo vaporized teleported`), and a laneData subset (`sampledSeconds speed density occupancy`). Each subset selects a different set of output columns.

Sim: `--end 200 --additional-files tools/oracle/cases/md_attrs/meandata.add.xml`.

Compared by run_all: tripinfo, fcd, stat and `ref.ed_wa_tl.xml`, `ref.ed_wa_cnt.xml`, `ref.ld_wa.xml` (compare_xml.py, exact).

Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh md_attrs; rm NEW_CASE` (pristine C++ 1.28.0, WSL).
