# teleport — a blocked jam that triggers `--time-to-teleport 20`

## Net and case

Chain `n0 → n1 → n2`, edges `E0`, `E1` (300 m each, 1 lane, 13.89 m/s), built from plain XML (`net.args`).

- `b0` (depart 0): route `E0 E1`, `<stop lane="E1_0" endPos="290" duration="1000"/>`. It blocks `E1` for the whole run.
- `f0..f4` (depart 2,4,6,8,10): route `E0 E1`, no stops. They queue behind `b0`.
- `sim.args`: `--end 120 --time-to-teleport 20`.

## Proof it is exercised (C++ 1.28.0)

`ref.log`:

```
Warning: Teleporting vehicle 'f0'; waited too long (blocked), lane='E1_0', time=71.00.
Warning: Vehicle 'f0' teleports beyond arrival edge 'E1', time=71.00.
Warning: Teleporting vehicle 'f1'; waited too long (blocked), lane='E1_0', time=97.00.
Warning: Vehicle 'f1' teleports beyond arrival edge 'E1', time=97.00.
```

From `ref.fcd.xml`:

| vehicle | first stopped (speed < 0.1) | teleported | blocked time |
|---|---|---|---|
| `b0` | t=45.00, `E1_0` pos 289.98 | never (stopped, not teleported) | — |
| `f0` | t=51.00, pos 282.48 | t=71 | 20 s = `--time-to-teleport` |
| `f1` | t=55.00, pos 274.98 | t=97 | |

`f0` is blocked for exactly 20 s, then teleported. So the blocked-vehicle timeout path is taken, not an
arbitrary removal.

## Verdict (ours, `sumo.exe` from `sumo-rs-target-x07/debug`)

**PANIC before any teleport, at t≈42.** Our run stops with:

```
thread 'main' panicked at crates\sumo_core\src\microsim\ms_vehicle.rs:6022:17:
leaveLane: stop handling on the same edge needs MSRouteIterator, which is not ported yet
```

So our log has **no** teleport warnings and no `f0` in tripinfo (`DIFF: vehicle f0 missing in candidate`).

First FCD diff, **t=41.00, `b0` `x`: ref 580.40, ours 583.94** (pos 280.40 vs 283.94). Ours also never brakes for
the stop: `b0` runs at 13.99 m/s to pos 283.94 at t=41 and 297.37 at t=42, past its `endPos=290`. C++ stops it at
289.98. So the same stop-approach failure as in `stops_parking` (a vehicle with a stop does not decelerate to
its stop) is the root cause here, and the panic follows the overshoot. This means the teleport path itself is not
measured yet; it needs the stop approach fixed first.

## Reproduce

- Net: `bash …/teleport/net_ref.sh` (WSL)
- Reference: `bash …/teleport/sim_ref.sh` (WSL; warnings go to `ref.log`)
- Ours: `sumo.exe -n ref_simple.net.xml -r routes.rou --tripinfo-output … --fcd-output … --statistic-output … --no-step-log true --end 120 --time-to-teleport 20`
- Compare: `python tools/oracle/compare_sim.py --fcd …`
