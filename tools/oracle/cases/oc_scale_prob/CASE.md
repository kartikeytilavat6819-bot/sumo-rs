# oc_scale_prob

`--scale` with a probability flow (thinning via `getQuota`), a `number` flow, a vType with
`scale="2"`, and a `vTypeDistribution` whose member has `scale="3"`. Two scale values (0.6 and 2.5)
exercise both thinning and up-scaling. Sim: `--end 90 --seed 7`.
