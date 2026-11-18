# jt_left_before_right_lh — left-before-right junction, lefthand traffic

Same 4-arm node/edge/connection layout as jt_left_before_right, but the net is built with
netconvert `--lefthand`. Combines two behaviours that are only tested separately today:
the left-before-right priority rule (jt_left_before_right, righthand) and lefthand geometry
(jt_right_before_left_lefthand). 20 cars depart in perpendicular pairs at 0,6,...,54 s.
