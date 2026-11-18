# out_detectors

E1 (inductionLoop, incl. vTypes filter), instantInductionLoop, E2 (single lane with thresholds, multi-lane `lanes`) and E3 (entryExit) detector outputs on the shared junction net (`junction` case) with ~140 vehicles.

Sim: `--end 300 --additional-files tools/oracle/cases/out_detectors/detectors.add.xml`.

Compared by run_all: tripinfo, fcd, stat and `ref.<suffix>.xml` for e1, ie1, e2, e3 (compare_xml.py, exact).

`extra_outputs.txt` = command-line outputs; `add_outputs.txt` = files written by the additional file, resolved next to it (run_all/make_sim_refs move/compare them).
Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh out_detectors; rm NEW_CASE` (pristine C++ 1.28.0, WSL).
