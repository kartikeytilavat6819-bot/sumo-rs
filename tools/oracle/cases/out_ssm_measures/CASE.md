# out_ssm_measures

SSM device with every measure (TTC, DRAC, PET, PPET, MDRAC, BR, SGAP, TGAP), write-positions and write-lane-positions, crossing/merging conflicts on the junction net.

Sim: `--end 150 --device.ssm.probability 1 --device.ssm.measures TTC,DRAC,PET,PPET,MDRAC,BR,SGAP,TGAP --device.ssm.thresholds 3,3,2,2,3.4,0,50,3 --device.ssm.range 50 --device.ssm.extratime 3 --device.ssm.write-positions true --device.ssm.write-lane-positions true`.

Compared by run_all: tripinfo, fcd, stat and `ref.<suffix>.xml` for ssm (compare_xml.py, exact).
Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh out_ssm_measures; rm NEW_CASE` (pristine C++ 1.28.0, WSL).

Gate 2026-11-17 (cv01, fc4c07bf): FAIL — ssm: SSMLog child count ref 283 vs ours 212 (the 212 conflicts match in count; the 71 <globalMeasures> elements for BR/SGAP/TGAP are missing).
