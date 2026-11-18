# md_teleport

A vehicle with a long `<stop>` on the single-lane edge `-103226#1` with `--time-to-teleport 10`, so followers on that lane jam and teleport; edgeData with `teleported`/`vaporized` counters (plus a `trackVehicles` variant). Tests teleport accounting in mean data.

Sim: `--end 150 --time-to-teleport 10 --additional-files tools/oracle/cases/md_teleport/meandata.add.xml`.

Compared by run_all: tripinfo, fcd, stat and `ref.ed_tp.xml`, `ref.ed_tp_tr.xml` (compare_xml.py, exact).

Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh md_teleport; rm NEW_CASE` (pristine C++ 1.28.0, WSL).
