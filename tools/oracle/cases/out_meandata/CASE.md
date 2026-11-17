# out_meandata

Mean data beyond edgeData traffic: `--edgedata-output`, `--lanedata-output`, laneData emissions + harmonoise, edgeData withInternal/excludeEmpty, edge subset with aggregate/maxTraveltime/minSamples, and trackVehicles + writeAttributes + vTypes filter.

Sim: `--end 300 --additional-files tools/oracle/cases/out_meandata/meandata.add.xml`.

Compared by run_all: tripinfo, stat and `ref.<suffix>.xml` for edgedata, lanedata, ld_emis, ld_noise, ed_int, ed_sub, ed_attr (compare_xml.py, exact).

`extra_outputs.txt` = command-line outputs; `add_outputs.txt` = files written by the additional file, resolved next to it (run_all/make_sim_refs move/compare them).
Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh out_meandata; rm NEW_CASE` (pristine C++ 1.28.0, WSL).

Gate 2026-11-17 (cv01, fc4c07bf): FAIL — ed_attr (trackVehicles + writeAttributes + vTypes=truck): first edge sampledSeconds ref 10.48 vs ours 11.48; all other meandata outputs match.
