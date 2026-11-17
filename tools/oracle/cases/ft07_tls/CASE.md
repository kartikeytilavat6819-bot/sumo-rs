# ft07_tls

Times Square net (`ref_simple.net.xml`, copied), six vehicles on routes A and B, `--end 200`.

- Base (`ref.*`): default signal control.
- `alloff`: `--tls.all-off true`.
- `actuated`: `--tls.actuated.jam-threshold 0.5`.

Refs: `bash tools/oracle/make_sim_refs.sh ft07_tls` (C++ 1.28.0 from ~/sumo-pristine, WSL). `NEW_CASE` marks a new case.
