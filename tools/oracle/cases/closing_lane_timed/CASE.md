# closing_lane_timed

`closingLaneReroute` closing E1 lane 0 for passengers only during a finite interval [30,90). Cars departing
before, during and after that window must choose lanes differently; FCD `lane` on E1 is the hook.
Sim: `--end 150 --additional-files .../closing_lane_timed.add.xml`.