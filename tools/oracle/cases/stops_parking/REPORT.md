# stops_parking — lane stop, `until` stop, parking-area stop, multi-stop

## Net and additional file

Chain `n0 → n1 → n2 → n3`, edges `E0`, `E1`, `E2` (300 m each, 1 lane, 13.89 m/s). Built from plain XML
(`net.args`), connections left to netconvert (linear chain, no con file needed).

`parking.add.xml`: `<parkingArea id="pa0" lane="E1_0" startPos="100" endPos="140" roadsideCapacity="2"/>`,
loaded via `--additional-files` in `sim.args`.

## Case (`routes.rou`, six vehicles, route `E0 E1 E2`)

| veh | depart | stop(s) |
|---|---|---|
| `p0` | 0 | `lane="E0_0" endPos="250" duration="10"` |
| `p1` | 2 | `lane="E1_0" endPos="200" until="45"` |
| `p2` | 4 | `parkingArea="pa0" duration="15"` |
| `p3` | 6 | `lane="E2_0" endPos="150" duration="5"` |
| `p4` | 8 | none (control) |
| `p5` | 10 | `lane="E0_0" endPos="100" until="30"` then `parkingArea="pa0" duration="5"` |

## Proof the stops are exercised (C++ 1.28.0, `ref.tripinfo.xml`, `ref.fcd.xml`)

`stopTime` from tripinfo:

| veh | stopTime | what it shows |
|---|---|---|
| p0 | **10.00** | lane stop, `duration` honoured |
| p1 | 1.00 | `until=45`: vehicle reached E1 only at ~t=44 (queued behind `p0`), so 1 s of stop left |
| p2 | **15.00** | parking-area stop, `duration` honoured; FCD parks it at `E1_0` pos **120.00**, t=54–68 (centre of `pa0`) |
| p3 | 5.00 | lane stop on E2, `duration` honoured |
| p5 | 12.00 | `until=30` lane stop at `E0_0` pos 99.99 (t=24–29), then parking-area stop at `E1_0` pos 139.99 (t=67–70) |

Caveat: FCD speed 0 is not proof of a stop by itself. `p1` and `p2` also queue at `E0_0` pos 242–235 behind `p0`
(t=27–31). The tripinfo `stopTime` column is the proof.

## Verdict (ours, `sumo.exe` from `sumo-rs-target-x07/debug`)

**PANIC.** Our run aborts at about t=62:

```
thread 'main' panicked at crates\sumo_core\src\microsim\ms_vehicle.rs:6022:17:
leaveLane: stop handling on the same edge needs MSRouteIterator, which is not ported yet
Error: leaveLane: stop handling on the same edge needs MSRouteIterator, which is not ported yet
```

Consequences: no tripinfo is written (`DIFF: vehicle p0 missing in candidate`), and the FCD stops at t=62 (63 steps; C++ has 200).

First FCD diff before the panic: **t=50.00, `p2` `x`: ref 415.99, ours 420.03**. That is 4 m apart, i.e. `p2`
is at the parking-area stop position earlier in our run (pos 120.03 vs C++ 115.99 at t=50). Every timestep before
t=50 matches. So the parking-area approach/stop diverges before the panic, and the panic follows the parking stop.

The panic message points at `ms_vehicle.rs:6022` (`leaveLane`, stop handling on the same edge). The parked
vehicle that triggers it is `p2`/`p5` (both use `pa0` on `E1`); the trigger is not isolated further.

## Reproduce

- Net: `bash …/stops_parking/net_ref.sh` (WSL)
- Reference: `bash …/stops_parking/sim_ref.sh` (WSL; writes `ref.log`, empty = no warnings in C++)
- Ours: `sumo.exe -n ref_simple.net.xml -r routes.rou --tripinfo-output … --fcd-output … --statistic-output … --no-step-log true --end 200 --additional-files parking.add.xml`
- Compare: `python tools/oracle/compare_sim.py --tripinfo …` / `--fcd …`
