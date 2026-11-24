#!/usr/bin/env python3
# oc97 probe (temporary): train routes found by traci.simulation.findRoute on the rail rerouter closing case.
# usage: oc97_route_probe.py <sumo-binary> <case-dir>
import os, sys
sys.path.append(os.path.join(os.environ["SUMO_HOME"], "tools"))
import traci

binary, case = sys.argv[1], sys.argv[2]
traci.start([binary, "-n", os.path.join(case, "net.net.xml"), "-r", os.path.join(case, "input_routes.rou.xml"),
             "--no-step-log", "--no-warnings", "--xml-validation", "never"])
traci.simulationStep()
for what, fn in (("vtype train", lambda: traci.vehicletype.getParameter("train", "closingReroute.maxDelay")),
                 ("vehicle v_0", lambda: traci.vehicle.getParameter("v_0", "closingReroute.maxDelay")),
                 ("vehicle v_0 typeID", lambda: traci.vehicle.getTypeID("v_0"))):
    try:
        print("param %s: %r" % (what, fn()))
    except Exception as e:
        print("param %s failed: %s" % (what, e))
for (a, b) in (("E6", "E4"), ("E0", "E4"), ("E6", "E7"), ("E6", "E8"), ("E5", "E4")):
    try:
        st = traci.simulation.findRoute(a, b, "train")
        print("findRoute %s->%s edges=%s cost=%.2f length=%.2f" % (a, b, list(st.edges), st.cost, st.length))
    except Exception as e:
        print("findRoute %s->%s failed: %s" % (a, b, e))
traci.close()
