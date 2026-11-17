# out_bt

Bluetooth devices (btreceiver probability 0.5, btsender 1, range 40, offtime 0.5) and `--bt-output`.

Sim: `--end 150 --device.btreceiver.probability 0.5 --device.btsender.probability 1 --device.btreceiver.range 40 --device.btreceiver.offtime 0.5`.

Compared by run_all: tripinfo, fcd, stat and `ref.<suffix>.xml` for bt (compare_xml.py, exact).
Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh out_bt; rm NEW_CASE` (pristine C++ 1.28.0, WSL).
