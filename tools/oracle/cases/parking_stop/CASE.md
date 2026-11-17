# parking_stop

Parked stops (`parking="true"`) on a plain lane and on a `parkingArea`, plus a normal parking-area stop.

Net `E0` (1 lane) → `E1` (2 lanes), 400 m each. `parking_stop.add.xml` holds
`<parkingArea id="pa0" lane="E1_0" startPos="100" endPos="140" roadsideCapacity="2"/>`.

| veh | depart | stop |
|---|---|---|
| pk0 | 0 | `lane=E1_0 endPos=250 parking=true duration=40` |
| pk1 | 5 | `parkingArea=pa0 parking=true duration=60` |
| c0 | 10 | none |
| pk2 | 20 | `parkingArea=pa0 duration=30` |
| c1 | 30 | none |
| pk3 | 40 | `lane=E1_1 endPos=320 parking=true until=120` |
| c2 | 60 | none |

Sim: `--end 200 --additional-files .../parking_stop.add.xml`.
Refs: `make_sim_refs.sh parking_stop` (pristine C++ 1.28.0), net via `net_ref.sh`.
