# jmparams

Vehicle junction-model parameters on the priority 4-arm junction `c` (geometry from `jmignore`).

- Major road `W2C -> C2E` (flow `fa`, 13.89 m/s); minor road `S2C -> C2N` (flow `fb`, 8.33 m/s).
- vType `vMin`: `jmTimegapMinor=0.5 jmSigmaMinor=0.9 impatience=0.8`.
- vType `vCross`: `jmCrossingGap=5 jmTimegapMinor=2.5 impatience=0`, used by two single vehicles on the minor road:
  - `vIgnIds`: `<param key="junctionModel.ignoreIDs" value="fa.3 fa.4 fa.5"/>` (ignores three major-road vehicles).
  - `vIgnTypes`: `<param key="junctionModel.ignoreTypes" value="vMaj"/>`.

Sim: `--end 200 --seed 3`. Reference: C++ sumo 1.28.0 from ~/sumo-pristine (`net_ref.sh`, `make_sim_refs.sh`).
