# md_wrattr_emis

`writeAttributes` used to select the mean-data type implicitly: emission attributes (`CO2_abs CO2_perVeh`) select the emissions type and `noise` selects the harmonoise type (edgeData and laneData). Existing cases only use traffic attributes or explicit `type=`.

Sim: `--end 200 --additional-files tools/oracle/cases/md_wrattr_emis/meandata.add.xml`.

Compared by run_all: tripinfo, fcd, stat and `ref.ed_wae.xml`, `ref.ed_wan.xml`, `ref.ld_wae.xml` (compare_xml.py, exact).

Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh md_wrattr_emis; rm NEW_CASE` (pristine C++ 1.28.0, WSL).
