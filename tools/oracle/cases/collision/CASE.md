# collision

Forced collision on one lane, one row per `--collision.action`.

Net: `E0 n0→n1 → E1 n1→n2`, 400 m, 1 lane. `ram` is inserted at `departPos=97` at t=5 while `lead`
(from t=0 at `departPos=100`, `departSpeed=0`, `<stop endPos=100 duration=20>`) still occupies
95…100. `--insertion-checks none` is what makes the overlapping insertion possible; the C++ then
reports `Vehicle 'ram'; collision with vehicle 'lead', lane='E0_0', gap=-4.50, time=5.00, stage=insertion`.

Variants (`sim.args` + `sim_<v>.args`, refs `ref_<v>.*`):

| row | `--collision.action` | C++ result |
|---|---|---|
| `ref` | `warn` | warning only, identical numbers to `none`; all 3 arrive (74/83/84 s) |
| `ref_none` | `none` | detection disabled, identical numbers to `warn` |
| `ref_remove` | `remove` | **both** participants removed: `arrival=5.00 vaporized="collision"`, `chase` arrives 80 s |
| `ref_teleport` | `teleport` | `ram` teleported to `E1` at t=5 and arrives at 38 s; `lead` 73 s, `chase` 80 s |

All four rows carry `--end 150 --insertion-checks none`.
Refs: `make_sim_refs.sh collision` (pristine C++ 1.28.0), net via `net_ref.sh`.
