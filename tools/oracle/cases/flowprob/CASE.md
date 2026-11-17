# flowprob

Two flows on a 2-edge line: `fp` with `probability="0.30"` over 0–200 s (RNG-driven) and `fn` with `number="4"`.
Seeds: `sim.args` `--seed 7`; variant `sim_seed11.args` `--seed 11` checks the same flow with another seed.
Refs: `make_sim_refs.sh flowprob` (pristine C++ 1.28.0).
