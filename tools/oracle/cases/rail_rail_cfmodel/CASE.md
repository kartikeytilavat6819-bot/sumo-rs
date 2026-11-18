# rail_rail_cfmodel

`carFollowModel="Rail"` with three different `trainType` values (`Freight`, `ICE3`,
`RB425`) on a 1600 m single-track line with rail signals. The Rail model derives
traction/resistance curves from the train type, so each vehicle accelerates and
brakes differently. Three trains follow each other eastbound (depart 0/15/30) and
one runs westbound (depart 45). Existing rail cases use the default Krauss model.

Sim: `--end 300 --time-to-teleport -1`.
