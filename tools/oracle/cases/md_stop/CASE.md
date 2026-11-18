# md_stop

Vehicles that stop on the edge (lane-level `<stop>` with a duration) and how the stop affects mean-data `waitingTime`, `timeLoss` and `sampledSeconds`; one output also uses `trackVehicles`.

Sim: `--end 200 --additional-files tools/oracle/cases/md_stop/meandata.add.xml`.

Compared by run_all: tripinfo, fcd, stat and `ref.ed_stop.xml`, `ref.ed_stop_tr.xml` (compare_xml.py, exact).

Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh md_stop; rm NEW_CASE` (pristine C++ 1.28.0, WSL).
