# md_vtypes

`vTypes` filters selecting the `car` type, the `truck` type, and both, with a `trackVehicles` variant (period 50), using the standard car+truck flows.

Sim: `--end 200 --additional-files tools/oracle/cases/md_vtypes/meandata.add.xml`.

Compared by run_all: tripinfo, fcd, stat and `ref.ed_vt_car.xml`, `ref.ed_vt_truck.xml`, `ref.ed_vt_both.xml` (compare_xml.py, exact).

Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh md_vtypes; rm NEW_CASE` (pristine C++ 1.28.0, WSL).
