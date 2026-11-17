# ped_tls_crossing

Signalized 4-arm junction C (traffic_light, netconvert default program) with `--sidewalks.guess --crossings.guess --walkingareas`.
Cars on WC->CE and NC->CS; 9 pedestrians cross C at different times so they wait for pedestrian green/red.

- `sim.args`: `--pedestrian.model striping` (default) → `ref.*`
- `sim_nonint.args`: `--pedestrian.model nonInteracting` → `ref_nonint.*`
- `sim_stripe.args`: striping with `--pedestrian.striping.stripe-width 0.5 --pedestrian.striping.jamtime 10` → `ref_stripe.*`
- `extra_outputs.txt`: `--personinfo-output` (ref only, compared as `ref.personinfo.xml`)

Refs: `bash tools/oracle/make_sim_refs.sh ped_tls_crossing` (pristine C++ 1.28.0), `ref_simple.net.xml` from `netconvert $(cat net.args)`.
Persons use `departPos`, `arrivalPos`, walk `speed`, `type="ped"`.
