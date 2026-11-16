# Case `ev`: electric vehicles, battery and charging

Reference: C++ sumo 1.28.0 (WSL), flags as in `run_all.sh` plus `--battery-output` and `--chargingstations-output`.
Ours: `C:\Users\karti\sumo-rs-target-x07\debug\sumo.exe`, same flags, run from the repo root.

## Scene

- Net: `junction/ref_simple.net.xml`. Route `r0` = `-103224#0 -103224#1 -103224#2 -103224#3`, `r1` = `-103227#0..#2`.
- `chargingstation.add.xml`: `cs0` on `-103224#2_0`, 20–60 m, 50 kW, efficiency 0.95.
- vType `EV`: `Energy/unknown`, params `has.battery.device=true`, `device.battery.capacity=35000`,
  `device.battery.chargeLevel=10000`, `maximumChargeRate=150000`, `stoppingThreshold=0.1`.
- `ev_charge`: `r0`, depart 0, `<stop chargingStation="cs0" duration="40"/>`.
- `ev_drive`: `r1`, depart 5, no stop.

## Proof that C++ charges

`python charge_proof.py ref.battery.xml ref.chargingstations.xml`:

- `ev_charge` battery 10000.00 Wh at t=0 → 10501.10 Wh at the end.
- At station from t=18.00 to t=57.00 s. Station total 527.78 Wh into `ev_charge`.
- `ev_drive` is never charged.

## Verdicts (ours vs C++)

| output | verdict | first diff |
|---|---|---|
| tripinfo | **FAIL** | `ev_drive` `devices`: ref `tripinfo_ev_drive battery_ev_drive fcd_ev_drive`, ours has no `battery_*`. Ours has no `<battery>` element for either EV. Same gap for `ev_charge`. |
| fcd | MATCH | |
| stat | MATCH | |
| battery-output | **FAIL** | ours has 0 timestep records; C++ has 83 |
| chargingstations-output | **FAIL** | ours is empty; C++ has `cs0` (527.78 Wh, 40 steps) and `ev_charge` (18.00–57.00) |

No panic. Exit 0 on both sides.

Traffic is identical (FCD and stat match, `ev_charge` stops 40 s and both arrive at 67.00). The battery device is
the only divergence: ours never builds it, so the EV never charges. The vType branch that sets the
`Energy/unknown` class from `has.battery.device` is present (`ms_vehicle_type.rs:1598`). The build call exists
(`ms_device.rs:181`). `ms_device_battery.rs` still has 1 `todo!()` and 7 `shortcut:` notes. Not chased further.

## Reproduce

```
bash -c 'cd /mnt/c/.../cases/ev && SUMO_HOME=$HOME/sumo-src $HOME/sumo-src/bin/sumo -n ref_simple.net.xml -r routes.rou ... --additional-files $D/chargingstation.add.xml --battery-output ref.battery.xml --chargingstations-output ref.chargingstations.xml'
python compare_ev.py ref.battery.xml CAND.battery.xml ref.chargingstations.xml CAND.chargingstations.xml
```

`sim.args` (`--end 120 --additional-files tools/oracle/cases/ev/chargingstation.add.xml`) is picked up by `run_all.sh`.
The sim stage will show FAIL on tripinfo and PASS on FCD and stat. The battery and charging outputs are only in `compare_ev.py`, not in `run_all.sh`.
