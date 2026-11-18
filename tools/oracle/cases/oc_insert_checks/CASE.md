# oc_insert_checks

Tests the vehicle/vType `insertionChecks` attribute (`none`, `collision`, `leaderGap`, `followerGap`,
space-separated combinations, `all`) with flows and vehicles that all try to insert at the same lane
position. The `global` variant overrides the default with `--insertion-checks none` (affects only
vehicles without an explicit attribute). Sim: `--end 90 --seed 5`.
