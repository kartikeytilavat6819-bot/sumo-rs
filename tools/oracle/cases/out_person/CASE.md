# out_person

`--person-summary-output` and `--personroute-output` on the `ped_persontrip_public` net/routes.

Sim: `--end 400 --persontrip.walkfactor 0.75 --additional-files tools/oracle/cases/out_person/ped_pt.add.xml`.

Compared by run_all: tripinfo, fcd, stat and `ref.<suffix>.xml` for personsummary, personroute (compare_xml.py, exact).
Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh out_person; rm NEW_CASE` (pristine C++ 1.28.0, WSL).
