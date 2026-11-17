# oc_repeat_cycle

Repeated routes on the a-b-c-d ring: `loop` has `repeat="2" cycleTime="100"` with both a `duration`
stop and an `until`+`extension` stop (the until time shifts by the cycle time each lap); `loop2`
uses `repeat="3"` with a plain duration stop. Used by two vehicles and a flow. Sim: `--end 300 --seed 5`.
