#!/usr/bin/env python3
"""Build a self-contained bundle of SUMO complex/TraCI tests for the gate oracle.

The gate VM has the sumo pip wheel (binaries + tools) but not the `tests/` tree,
so the inputs each runner.py needs are copied here, resolved the same way
TextTest's `copy_test_path` does: walk from the test directory up to
tests/complex and take the nearest copy of every file.  Run locally from the
sumo-rs repo root:
    python tools/oracle/build_complex_bundle.py
"""
import pathlib
import re
import shutil

SRC = pathlib.Path("C:/Users/karti/sumo-src/tests/complex")
DST = pathlib.Path("tools/oracle/complex_bundle")

# Chosen for fast runtime and broad domain coverage; GUI/libsumo/external-package
# tests and long-running scenarios are deliberately excluded.
TESTS = """
misc/encoding
misc/rerouting/rerouteTraveltime_default
bugs/ticket5620
vehicle/moveToXY/ticket18457
simulation/convertRoad
bugs/ticket3349
edge/bidi
lane/bidi
vehicle/getNextTLS
bugs/ticket2803
bugs/ticket8945
bugs/ticket3348
bugs/ticket7949
person/force_depart
trafficlight/getSpentDuration
vehicle/add/addTrip
variablespeedsign/variablespeedsign
simulation/state_save_vtype
trafficlight/constraints/getConstraints_types
bugs/ticket15123
lane/lane_intermodal
person/junctionModel.ignoreProb
trafficlight/getSpentDuration2
trafficlight/setLogic_duration0
trafficlight/setLogic_repeated
bugs/ticket11117
bugs/ticket13384
bugs/ticket13710
bugs/ticket15603
bugs/ticket4214
connection/close_named
trafficlight/getSpentDuration3
simulation/findRoute
simulation/loadState/errors
vehicle/add/optionalArgs
edge/setMaxSpeed
lane/setMaxSpeed
person/getWalkingDistance
person/remove_crossing
vehicle/add/add_internal
vehicle/addTrip_junctions
vehicle/setLcContRight
vehicletype/mass
bugs/ticket5309
vehicle/ignoreJunctionFoes
vehicle/insertStop/default
vehicle/replaceStop/default
vehicle/setEmissionClass
vehicle/changeTarget
vehicle/getNextLinks
vehicle/nextTLS
vehicle/nextTLS_parking
vehicle/setStop/newInternalStop
bugs/ticket5561
bugs/ticket5674
bugs/ticket6626
edge/subscribeContextVehicles
lane/setMaxSpeed2
person/junctionModel.ignore
person/start_in_vehicle
vehicle/driverstate
simulation/loadState/IDList
simulation/loadState/persons
contextSubscriptions/parking
person/remove
person/setSpeedFactor
vehicle/getInvisible
vehicle/insertStop/first
vehicle/moveToXY/no_permitted_route_lane
vehicle/speedMode_speedLimit
""".split()

SKIP_NAMES = re.compile(r"\.complex(\..*)?$|^testsuite\.complex$|^update_states\.sh$|\.pyc$")


def main():
    if DST.exists():
        shutil.rmtree(DST)
    for rel in TESTS:
        t = SRC / "traci" / pathlib.Path(rel)
        assert (t / "runner.py").is_file(), t
        out = DST / rel
        out.mkdir(parents=True, exist_ok=True)
        chain = []
        d = t
        while d != SRC.parent:
            chain.append(d)
            if d == SRC:
                break
            d = d.parent
        # farthest -> nearest so the nearest copy wins (TextTest semantics)
        for d in reversed(chain):
            for f in sorted(d.iterdir()):
                if f.is_file() and not SKIP_NAMES.search(f.name):
                    shutil.copy2(f, out / f.name)
        # texttest's copy_test_path_merge:data -- nearest data/ directory wins
        for d in reversed(chain):
            data = d / "data"
            if data.is_dir():
                shutil.copytree(data, out / "data", dirs_exist_ok=True)
    print(f"bundled {len(TESTS)} tests into {DST}")


if __name__ == "__main__":
    main()
