# hw_acc_platoon

Two-lane highway (E0, 1000 m, 25 m/s). A slow leader in lane 0 is followed by a tight string of
ACC (`carFollowModel="ACC"`) vehicles with non-default CF gains set via `<param>`; a few fast
Krauss cars in lane 1 overtake and cut back in, perturbing the ACC string.

Sim: `--end 200`. Net from node/edge XML via `net_ref.sh`.
