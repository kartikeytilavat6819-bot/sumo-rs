#!/usr/bin/env python3
# oc98 probe (temporary): per-step state of Stapler_00 around setRoute/setRouteID in vehicle/distance_after_reroute
import os, sys
sys.path.append(os.path.join(os.environ["SUMO_HOME"], "tools"))
import traci
data = sys.argv[2]
traci.start([sys.argv[1], "-c", os.path.join(data, "hello.sumocfg"), "--no-step-log", "--no-warnings"])
for step in range(133):
    traci.simulationStep()
    if step == 120:
        traci.vehicle.setRoute('Stapler_00', ('ed1', 'ed5'))
    if step == 122:
        traci.vehicle.setRouteID('Stapler_00', "short")
    if step >= 120:
        v = 'Stapler_00'
        print(step, "road=%s lane=%s pos=%.2f speed=%.2f acc=%.2f dist=%.2f route=%s idx=%s nextLinks=%s" % (
            traci.vehicle.getRoadID(v), traci.vehicle.getLaneID(v), traci.vehicle.getLanePosition(v),
            traci.vehicle.getSpeed(v), traci.vehicle.getAcceleration(v), traci.vehicle.getDistance(v),
            traci.vehicle.getRoute(v), traci.vehicle.getRouteIndex(v),
            [(l[0], l[1], l[2]) for l in traci.vehicle.getNextLinks(v)]))
traci.close()
