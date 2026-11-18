import os, sys
sys.path.insert(0, "/home/karti/sumo-pristine/lib/python3.12/site-packages/sumo/tools")
os.environ["SUMO_HOME"] = "/home/karti/sumo-pristine/lib/python3.12/site-packages/sumo"
import traci
import subprocess

CASE = "/mnt/c/Users/karti/sumo-rs/.claude/worktrees/agent-oc30/tools/oracle/cases/junction"
net = CASE + "/ref_simple.net.xml"
rou = CASE + "/routes.rou"
port = 8899
p = subprocess.Popen(["/home/karti/sumo-pristine/bin/sumo", "-n", net, "-r", rou,
                      "--remote-port", str(port), "--no-step-log", "true", "--no-warnings", "true"])
traci.init(port=port, numRetries=20)
traci.simulationStep(0)
traci.simulationStep(10)
E1, E2, E3 = "-103224#1", "-103224#2", "-103224#3"
print("edges:", traci.edge.getIDList())
for label, fn in [
    ("intermodal(E1,E3)", lambda: traci.simulation.findIntermodalRoute(E1, E3)),
    ("intermodal(E1,nope)", lambda: traci.simulation.findIntermodalRoute(E1, "nope")),
    ("intermodal(nope,E3)", lambda: traci.simulation.findIntermodalRoute("nope", E3)),
    ("intermodal(E1,E3,modes=car)", lambda: traci.simulation.findIntermodalRoute(E1, E3, modes="car")),
    ("intermodal(E1,E3,modes=walk)", lambda: traci.simulation.findIntermodalRoute(E1, E3, modes="walk")),
    ("intermodal(E1,E3,modes=bicycle)", lambda: traci.simulation.findIntermodalRoute(E1, E3, modes="bicycle")),
]:
    try:
        r = fn()
        print(label, "=>", [vars(s) for s in r] if isinstance(r, list) else r)
    except Exception as e:
        print(label, "=> EXC", type(e).__name__, str(e))
traci.close()
p.wait(timeout=10)
