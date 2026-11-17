# route_check: what C++ does with a disconnected route

Net: `ref_simple.net.xml` (copy of `cases/junction`). Routes: `vbad` on `-103224#0 -103227#1` (no connection), `vgood` on a valid route.

## C++ 1.28.0 (measured in WSL)

| Variant | Behaviour | Exit status |
|---|---|---|
| default (`gCheckRoutes` true) | `Error: Vehicle 'vbad' has no valid route. No connection between edge '-103224#0' and edge '-103227#1'.` then `Quitting (on error).`; nothing simulated | 0 (see below) |
| `--ignore-route-errors true` | `vbad` waits on `-103224#0_0` and is teleported at t=307 s onto `-103227#1`; run completes. Reference committed as `ref.*` with `sim.args` | 0 |

Exit status: C++ `sumo` also returns 0 for a missing route file, while our binary returns 1 for any ProcessError. This is a global `sumo_main` divergence, not part of this case, so `run_error.sh` compares the `Error:` line only.

## Ours

- **Before the fix:** default run exits 0, no route error, and `vbad` teleports at t=307 s exactly as the ignore variant does. `run_error.sh`: **FAIL**. This is the `gCheckRoutes` shortcut.
- **After the fix:** the `Error:` line matches C++ byte for byte. `run_error.sh`: **PASS**.
- **Ignore variant** (`run_all`): PASS before and after.

## Gaps
- `run_all.sh` only scores the ignore variant. It runs a sim stage only when `ref.tripinfo.xml` exists, and it counts any nonzero exit as FAIL, so the default-error path cannot be scored there. `run_error.sh` covers it.
- Exit-status parity on ProcessError is open (global, not this case).
