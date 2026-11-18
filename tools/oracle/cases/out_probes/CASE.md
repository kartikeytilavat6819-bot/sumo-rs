# out_probes

vTypeProbe, routeProbe and calibrator `output` (the calibrator writes relative to the working directory, i.e. the repo root, hence `../../../../out_probes_cal.xml` in add_outputs.txt).

Sim: `--end 250 --additional-files tools/oracle/cases/out_probes/probes.add.xml`.

Compared by run_all: tripinfo, fcd, stat and `ref.<suffix>.xml` for vtp, rp, cal (compare_xml.py, exact).

`extra_outputs.txt` = command-line outputs; `add_outputs.txt` = files written by the additional file, resolved next to it (run_all/make_sim_refs move/compare them).
Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh out_probes; rm NEW_CASE` (pristine C++ 1.28.0, WSL).
