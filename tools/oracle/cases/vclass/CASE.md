# vclass

Vehicle classes on class-restricted lanes.

`E0` (400 m) has three lanes with `allow` sets, the lane-child form of the plain-XML edge:

```xml
<edge id="E0" from="n0" to="n1" numLanes="3">
    <lane index="0" allow="bus coach"/>
    <lane index="1" allow="truck delivery motorcycle passenger"/>
    <lane index="2" allow="bicycle"/>
</edge>
```

`E1` is a normal 3-lane edge, and netconvert connects `E0` lane `i` to `E1` lane `i` in the generated
`ref_simple.net.xml`. One vehicle per class — `bus0`/`bus1` (bus), `car0`/`car1` (passenger),
`truck0`, `bike0` (bicycle), `moto0` (motorcycle) — all on `E0 E1`.

The FCD `lane` attribute is the measured hook: a bus that is allowed onto the passenger lane, or a
bicycle that refuses its own lane, diverges on the first step it enters `E0`.

Sim: `--end 150`.
Refs: `make_sim_refs.sh vclass` (pristine C++ 1.28.0), net via `net_ref.sh`.
