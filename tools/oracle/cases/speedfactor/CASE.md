# speedfactor

Single 600 m edge `A`, one lane. Vehicle type `vSF` has `speedFactor="norm(1,0.1)"` (flow `fSF`); flow `fDef` uses the default type.

- `ref` (`sim.args`): `--end 120 --seed 11`.
- `ref_nodev` (`sim_nodev.args`): same plus `--default.speeddev 0`, which removes the default type's speed deviation.

Reference: C++ sumo 1.28.0 from ~/sumo-pristine (`net_ref.sh`, `make_sim_refs.sh`).
