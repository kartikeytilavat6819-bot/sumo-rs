# hw_cacc_platoon

Two-lane highway (E0, 1000 m, 25 m/s). A slow Krauss leader is followed by a CACC
(`carFollowModel="CACC"`) platoon with non-default CACC gains (`speedControlGainCACC`,
`gapControlGainGap`, `tauCACCToACC`, ...) via `<param>`. Fast Krauss cars overtake in lane 1.

Sim: `--end 200`. Net from node/edge XML via `net_ref.sh`.
