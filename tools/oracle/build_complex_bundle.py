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
bugs/ticket1010
bugs/ticket10212
bugs/ticket10212b
bugs/ticket11870
bugs/ticket12810
bugs/ticket12944
bugs/ticket13053
bugs/ticket1433
bugs/ticket15486
bugs/ticket16721
bugs/ticket2321
bugs/ticket2777
bugs/ticket2804
bugs/ticket3140
bugs/ticket3505
bugs/ticket3505_test_with_begin_ge_savetime
bugs/ticket3505_with_load_offset
bugs/ticket3522
bugs/ticket3634
bugs/ticket3648
bugs/ticket3810
bugs/ticket4141
bugs/ticket4324
bugs/ticket4418
bugs/ticket4432
bugs/ticket4514
bugs/ticket4563
bugs/ticket4642
bugs/ticket4920
bugs/ticket5114
bugs/ticket5390
bugs/ticket5712
bugs/ticket5831
bugs/ticket5932
bugs/ticket5990
bugs/ticket6239
bugs/ticket7057
bugs/ticket8714
busstop/busstop
busstop/noStops
calibrator/calibrator
chargingstation/basic
chargingstation/chargingPower
chargingstation/efficiency
connection/busySocket
connection/clientCloses
connection/close_instance
connection/closingOutput
connection/multiconnect
connection/multipleConnections/asyncDisconnect
connection/multipleConnections/basic
connection/multipleConnections/concurringSubscriptions
connection/multipleConnections/differentSteplengths
connection/multipleConnections/orderDependentResults
connection/multipleConnections/subsecondSUMOstep
connection/multipleConnections/unspecifiedOrderWarning
connection/parallelConnection
connection/repeatedConnection
connection/tracing
contextSubscriptions
contextSubscriptions/elevated
contextSubscriptions/matrix_test
contextSubscriptions/simulation_5
edge/edge
edge/permissions_cch_reroute
edge/permissions_internal_lane
inductionloop/inductionloop
inductionloop/intervalData
inductionloop/override
inductionloop/persons_internalDet
inductionloop/persons_walk
inductionloop/persons_walk_wrongType
junction/junction
lanearea/intervalData
lanearea/lanearea
lanearea/override
meandata/meandata
meandata/meandata_vectorized
meandata/meandata_vectorized_lanes
multientryexit/multientryexit
overheadwire/overheadwire
parkingarea/parkingarea
poi/poi
polygon/add_large
polygon/polygon
polygon/polygon_dynamics
route/route
vehicletype/save_and_load_vtype_params
vehicletype/vehicletype
person/abort_riding
person/appendDrivingStage_stoppingPlaces
person/arrivalPos
person/moveTo/after_creation
person/moveTo/after_creation_crossing
person/moveTo/after_creation_walkingarea
person/moveTo/after_creation_walkingarea2
person/moveToXY/after_creation
person/moveToXY/after_creation_add_walk
person/moveToXY/keepRoute1_junction
person/moveToXY/keepRoute2_junction
person/moveToXY/keepRoute2_junction_offroad
person/moveToXY/leaveRoute
person/moveToXY/leaveRoute_jaywalk
person/moveToXY/leaveRoute_walkingarea
person/moveToXY/matchThreshold
person/moveToXY/outside_network
person/moveToXY/within_route
person/moveToXY/within_route_jaywalk
person/moveToXY/within_route_onRoad
person/moveToXY/within_route_walkingarea
person/person
person/priorStage
person/priorStage2
person/remove_add
person/setSpeedFactor2
person/trip_chain
simulation/collisions
simulation/load
simulation/loadState/busStop
simulation/loadState/calibrator
simulation/loadState/detectors
simulation/loadState/persons2
simulation/loadState/persons_include
simulation/loadState/railSignal
simulation/loadState/repeat
simulation/loadState/rerouting
simulation/loadState/reset_routeFiles
simulation/loadState/subscription
simulation/loadState/tlProgramOnline
simulation/simulation
trafficlight/NEMA/maxGreens
trafficlight/NEMA/offset
trafficlight/NEMA/phaseCalls
trafficlight/NEMA/splits
trafficlight/NEMA/splits_invalid
trafficlight/actuated
trafficlight/actuated_conditions
trafficlight/constraints/addConstraint
trafficlight/constraints/getConstraints
trafficlight/constraints/removeConstraints_all
trafficlight/constraints/removeConstraints_foeId
trafficlight/constraints/removeConstraints_foeSignal
trafficlight/constraints/removeConstraints_signalId
trafficlight/constraints/removeConstraints_tripId
trafficlight/constraints/swap/1
trafficlight/constraints/swap/2
trafficlight/constraints/swap/2_swapLater
trafficlight/constraints/swap/3
trafficlight/constraints/swap/4
trafficlight/constraints/swap/4b
trafficlight/constraints/swap/5
trafficlight/constraints/swap/6
trafficlight/constraints/swap/bidi
trafficlight/constraints/swap/egoLeader
trafficlight/constraints/swap/foeInsertion
trafficlight/constraints/swap/insertion
trafficlight/constraints/swap/insertion_tripId
trafficlight/constraints/swap/looped
trafficlight/constraints/swap/redundant_1
trafficlight/constraints/swap/redundant_2
trafficlight/constraints/swap/redundant_2_routes
trafficlight/constraints/swapConstraints
trafficlight/constraints/swapConstraints_loadState
trafficlight/constraints/updateConstraints
trafficlight/crossings
trafficlight/crossings_twoSided
trafficlight/getBlocking_reroute
trafficlight/offset
trafficlight/railsignal_state
trafficlight/setProgramLogic_types
trafficlight/trafficlight
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
