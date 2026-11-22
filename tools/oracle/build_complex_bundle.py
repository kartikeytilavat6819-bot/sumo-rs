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
misc/concurrent
misc/execute_move
misc/option_list_append
misc/params
misc/paramsWithKey
misc/paramsWithKey_subscription
misc/parkingSearch/retrieveParkingMemory
misc/rerouting/add_with_taz
misc/rerouting/findIntermodalRoute/appendTrip_car_public_speedFactor
misc/rerouting/findIntermodalRoute/car_public_speedFactor
misc/rerouting/findIntermodalRoute/car_public_speedFactor_noTrains
misc/rerouting/findIntermodalRoute/car_public_speedFactor_routingMode
misc/rerouting/findIntermodalRoute/taxi
misc/rerouting/findIntermodalRoute/taxi_startEndWalk
misc/rerouting/person/stop_walk_stop_walk
misc/rerouting/person/walk
misc/rerouting/person/walk_stop_walk
misc/rerouting/person/walk_walk_stop_walk
misc/rerouting/rerouteParkingArea_end
misc/rerouting/rerouteParkingArea_intermediate
misc/rerouting/rerouteParkingArea_looped
misc/rerouting/rerouteParkingArea_undeparted
misc/rerouting/rerouteTraveltime_adaptEdgeWeight
misc/rerouting/rerouteTraveltime_adaptEdgeWeight_varyWithTime
misc/rerouting/rerouteTraveltime_adaptVehicleEdgeWeight
misc/rerouting/rerouter_routingMode
misc/rerouting/routingMode
misc/steplistener
misc/subscription_time
misc/tracing
misc/tracing_close_start
misc/tracing_noGetters
misc/tracing_printGetters
misc/tracing_repeat
misc/verbose
vehicle/add/addTrip2
vehicle/add/attributes
vehicle/add/departSpeed_invalid
vehicle/add/departSpeed_invalid_ignore
vehicle/add/departTriggered
vehicle/add/remove_reinsert
vehicle/addTrip_taz
vehicle/alightingPersons
vehicle/cfmodel/CACC
vehicle/cfmodel/setParameter
vehicle/changeLane/basic
vehicle/changeLane/changeOpposite
vehicle/changeLane/changeOpposite_default
vehicle/changeLane/changeRelative
vehicle/changeLane/changeRelative_invalid
vehicle/changeLane/changed_while_blocked
vehicle/changeLane/lane_deadend
vehicle/changeLane/stay
vehicle/changeLane_subLane/basic
vehicle/changeLane_subLane/changeCurrent
vehicle/changeLane_subLane/changeOpposite
vehicle/changeLane_subLane/changeOpposite_default
vehicle/changeLane_subLane/changeRelative
vehicle/changeLane_subLane/changeRelative_blocked
vehicle/changeLane_subLane/changeRelative_invalid
vehicle/changeLane_subLane/lane_deadend
vehicle/changeLane_subLane/stay
vehicle/changeSublane
vehicle/delayed_taz_departure
vehicle/distance_after_reroute
vehicle/getFollowSpeed
vehicle/getFollower
vehicle/getJunctionFoes
vehicle/getLeader
vehicle/getLeader2
vehicle/getNeighbors
vehicle/getNeighborsOpposite
vehicle/getNeighborsSublane
vehicle/getParameter
vehicle/getShadowLaneID
vehicle/getStops
vehicle/insertStop/all
vehicle/insertStop/empty_id
vehicle/insertStop/first_highspeed
vehicle/insertStop/first_init
vehicle/insertStop/first_init_setSpeed
vehicle/insertStop/first_later
vehicle/insertStop/first_loop
vehicle/insertStop/first_teleport
vehicle/insertStop/invalid
vehicle/insertStop/keepRoute
vehicle/insertStop/keepRoute2
vehicle/insertStop/last
vehicle/insertStop/looped
vehicle/insertStop/looped2
vehicle/insertStop/non_looped_routes_2same_stops
vehicle/insertStop/non_looped_routes_3same_stops
vehicle/insertStop/non_looped_routes_3same_stops_init
vehicle/insertStop/non_looped_routes_6_stops_init
vehicle/insertStop/on_junction
vehicle/insertStop/reached_invalid
vehicle/insertStop/routeStop_init
vehicle/insertStop/second
vehicle/insertStop/second_change_attribute
vehicle/insertStop/second_teleport
vehicle/insertStop/second_teleport_oncoming
vehicle/insertStop/second_unreachable_ignore_rerouter
vehicle/insertStop/second_unreachable_teleport
vehicle/insertStop/stop_forbidden_ignore_rerouter_copy
vehicle/insertStop/third
vehicle/insertStop/third_later
vehicle/insertStop/unreachable
vehicle/junctionModel.ignore
vehicle/keepAfterArrival
vehicle/keepAfterArrival_state
vehicle/meso
vehicle/moveTo/controller_loop
vehicle/moveTo/moveTo_detector
vehicle/moveTo/odometer
vehicle/moveTo/opposite_driving
vehicle/moveToXY/collision
vehicle/moveToXY/customLength_keepRoute2
vehicle/moveToXY/elevation_sublane
vehicle/moveToXY/extreme_elevation
vehicle/moveToXY/extreme_elevation_keeproute2
vehicle/moveToXY/guess_next_edge_keepRoute0
vehicle/moveToXY/guess_next_edge_keepRoute2
vehicle/moveToXY/guess_next_edge_keepRoute2_reverse
vehicle/moveToXY/intersection
vehicle/moveToXY/keepRoute2
vehicle/moveToXY/keepRoute2_parking
vehicle/moveToXY/low_stepLength
vehicle/moveToXY/low_stepLength_keepRoute2
vehicle/moveToXY/matchThreshold
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
