# md_halt

`haltingSpeedThreshold` (absolute 5 m/s) and `haltingSpeedThresholdRel` (0.8 of the speed limit) on edgeData; slow, non-stopped vehicles are counted as waiting, which changes `waitingTime`. These meandata attributes are otherwise untested.

Sim: `--end 200 --additional-files tools/oracle/cases/md_halt/meandata.add.xml`.

Compared by run_all: tripinfo, fcd, stat and `ref.ed_halt.xml`, `ref.ed_haltrel.xml` (compare_xml.py, exact).

Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh md_halt; rm NEW_CASE` (pristine C++ 1.28.0, WSL).
