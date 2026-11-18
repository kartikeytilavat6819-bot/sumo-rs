# hw_cfmix

Two-lane highway (E0, 1200 m, 25 m/s) mixing three car-following models: Krauss, IDM
(`delta=3, stepping=0.1`) and EIDM (`treaction`, `tpreview`, `ccoolness`). A slow leader
platoons with alternating model followers; faster cars in the other lane overtake.

Sim: `--end 200`. Net from node/edge XML via `net_ref.sh`.
