# C++ sort / tie-order audit (ss01)

Scope: every `std::sort(` / `sort(` (ADL) / `stable_sort` / `partial_sort` / `nth_element` / `min_element` /
`max_element` call in `C:\Users\karti\sumo-src\src`, mapped to its Rust counterpart in `crates/`.

Verdict key:
- **a** strict total order (ties impossible, or equal keys are identical values): Rust sort unchanged.
- **b** ties possible and the C++ is `std::sort`: Rust now uses `std_sort` / `std_sort_vec` (libstdc++ port).
- **c** C++ `stable_sort`: Rust stable sort is correct.
- **p** pointer-ordered in C++ (address order): not reproducible, noted in code.
- **m** min/max_element: first-of-minimum semantics checked.
- **n/a** no Rust counterpart (module not ported: gui, netedit, guisim, libsumo GUI glue, etc.).
- **todo** counterpart exists, not yet reviewed in this pass.

Helpers: `utils::common::std_sort::std_sort` (slice of Copy), `std_sort_vec` (any Vec, sorts indices then permutes).

## netbuild

| C++ file:line | Rust file:line | verdict | note |
|---|---|---|---|
| NBAlgorithms.cpp:137 combination_by_angle | nb_algorithms.rs (turning directions) | b | was stable `sort_by_less`; now std_sort, Combination derives Copy |
| NBAlgorithms.cpp:465,474,497,499 | nb_algorithms.rs:333,342,366,369 | b | already std_sort |
| NBAlgorithms.cpp:546,601,611 | nb_algorithms.rs:397,450,456 | b | already std_sort |
| NBAlgorithms.cpp:712 crossing ordering | nb_algorithms.rs:1393 | b | was stable; now std_sort |
| NBAlgorithms_Railway.cpp:656 seqsToReverse (length) | nb_algorithms_railway.rs:2046 | b | was stable; now std_sort_vec |
| NBNode.cpp:2024,2026 | nb_node.rs:2292,2295 | b | already std_sort |
| NBNode.cpp:2930 myCrossings | nb_node.rs:~4780 | b | was stable; now std_sort on indices |
| NBNode.cpp:3032,3037 edges.sort() | nb_node.rs:4971,4976 | a | result used only for equality |
| NBNode.cpp:3188 edges by centroid | nb_node.rs:~5169 | b | was stable; now sort_edges_by (std_sort) |
| NBNode.cpp:3217 rawAngleDiffs | nb_node.rs:5201 | a | doubles |
| NBNode.cpp:4247,4305-4307,4321-4323,4361 | nb_node.rs:7050-7097,7778 | b | already std_sort |
| NBEdge.cpp:1398 relative_outgoing | nb_edge.rs:2961 | b | was stable sort_by; now std_sort (look-ahead comparator, not guaranteed total) |
| NBEdge.cpp:1446,1452 connections | nb_edge.rs:3060,3080 | b | was stable; now std_sort_vec (connections_sorter ties across toEdges) |
| NBEdge.cpp:2744 toLaneMap items | nb_edge.rs:10120 | p | NBEdge* pointer order |
| NBEdge.cpp:2866 knownTargets | nb_edge.rs:~10242 | a | int lane indices, unique via find; sort() on i32 |
| NBEdge.cpp:5042,5057 similar direction | nb_edge.rs:8303,8344 | b | was stable; now std_sort (front of sort used) |
| NBEdgeCont.cpp:464 split_sorter (pos) | nb_edge_cont.rs:885 | b | was stable; now std_sort_vec |
| NBEdgeCont.cpp:468 split.lanes | nb_edge_cont.rs:889 | p | pointer order |
| NBEdgeCont.cpp:990 same_connection_edge | nb_edge_cont.rs:1857 | b | was stable; now std_sort (id tie-break) |
| NBEdgeCont.cpp:2027 roads | nb_edge_cont.rs:4025 | p | pair(double, (NBEdge*, lane)): C++ pointer tie, id tie-break kept (// determinism:) |
| NBOwnTLDef.cpp:234 incoming priority | nb_own_tl_def.rs:2728 | a | tie-break by edge id |
| NBLoadedTLDef.cpp:71 sortPhases (myTime) | nb_loaded_tl_def.rs:1027 | b | equal durations tie; now std_sort |
| NBLoadedTLDef.cpp:467 nodes_by_id | nb_loaded_tl_def.rs:566 | a | unique ids |
| NBTrafficLightDefinition.cpp:75,420 nodes_by_id | nb_traffic_light_definition.rs:248,421 | a | unique ids |
| NBPTStop.cpp:122 myAccesses | nbpt_stop.rs:270 | a | (lane, pos, len) tuples; equal tuples identical |
| NBPTStopCont.cpp:465,509 by distance | nbpt_stop_cont.rs:22 | b | was stable; now std_sort |
| NWWriter_SUMO.cpp:895,898,924 ids | nw_writer_sumo.rs:758,761,2083 | a | unique string ids |
| NWWriter_OpenDrive.cpp:1265,1296 nearby | nw_writer_open_drive.rs:2351,2373 | a | pair(distance, edge id string): total, ids unique |
| NWWriter_XML.cpp (edgeIDs) | nw_writer_xml.rs:738 | a | strings |

## od / dfrouter / router / marouter

| C++ file:line | Rust file:line | verdict | note |
|---|---|---|---|
| ODMatrix.cpp:765 cell_by_begin | od_matrix.rs:1187 | b | was stable; now std_sort (secondary keys exist, dups possible) |
| ODMatrix.cpp:285 descending_departure | od_matrix.rs:558 | a | tie-break by id |
| RODFRouteCont.cpp:97 by_distance | rodf_route_cont.rs:166 | b | no tie-break; now std_sort_vec |
| RODFDetector.cpp:330 departures (SUMOTime) | rodf_detector.rs:492 | a | values |
| RORouteDef.cpp:445 probability | ro_route_def.rs:885 | b | no tie-break; now std_sort_vec |
| RORouteHandler.cpp:765 departures (SUMOTime) | ro_route_handler.rs:1820 | a | values |
| RONet.cpp:563 departures | ro_net.rs:1341 | a | values |

## microsim

| C++ file:line | Rust file:line | verdict | note |
|---|---|---|---|
| MSEdge.cpp:236 by_id | ms_edge.rs:650 | a | unique ids |
| MSEdge.cpp:1283,1291 transportable_by_position | ms_edge.rs:1121,1138 | a | pos tie -> id |
| MSEdgeControl.cpp:245 ComparatorIdLess | ms_edge_control.rs:371 | a | |
| MSLane.cpp:2608 vehicle_position (buffered) | ms_lane.rs:3479 | a | pos tie -> numerical id |
| MSLane.cpp:2621 vehicle_natural_position (myVehicles) | ms_lane.rs:~3499 | b | lateral pos ties; now std_sort_vec |
| MSLane.cpp:2634 natural (myPartialVehicles) | ms_lane.rs:~7183 | b | now std_sort_vec |
| MSLane.cpp:2648 natural (myManeuverReservations) | ms_lane.rs:~7219 | b | now std_sort_vec |
| MSLane.cpp:3275 by_connections_to_sorter | ms_lane.rs:~6636 | b | now std_sort |
| MSLane.cpp:3326 min_element incoming_lane_priority | ms_lane.rs:~6747 | m | loop keeps first minimum: matches |
| MSLane.cpp:3351 outgoing_lane_priority (candidateLinks front) | ms_lane.rs:~6791 | b | was linear scan; now std_sort + front (same as C++) |
| MSLink.cpp:355,364,618 doubles | ms_link.rs:1134,1140,3650 | a | equal doubles identical |
| MSLink.cpp:1427 toSort (time, vehicle*) | ms_link.rs:3167 | p | pointer tie-break in C++ |
| MSVehicleControl.cpp:162,263 ComparatorNumericalIdLess | ms_vehicle_control.rs:493,1348 | a | |
| MSVehicleTransfer.cpp:99 operator< numerical id | ms_vehicle_transfer.rs:240 | a | |
| MSStoppingPlace.cpp:155 spaces (double,(double,vehicle*)) | ms_stopping_place.rs:359 | p | pointer tie-break in C++ |
| MSStoppingPlace.cpp:384 waiting ids | ms_stopping_place.rs:816 | a | strings |
| trigger/MSTriggeredRerouter.cpp:629 (int,int) | ms_triggered_rerouter.rs:754 | a | index unique |
| trigger/MSStoppingPlaceRerouter.cpp:227 blockedTimes | ms_stopping_place_rerouter.rs:690 | a | id + index tie-break |
| trigger/MSStoppingPlaceRerouter.cpp:274 candidates | ms_stopping_place_rerouter.rs:717 | a | id tie-break |
| trigger/MSOverheadWire.cpp:105 vehicle_position (no tie-break) | ms_overhead_wire.rs:277 | b | now std_sort_vec |
| MSE2Collector.cpp:907 compareMoveNotification (distance) | mse2_collector.rs:1044 | b | no tie-break; now std_sort_vec |
| MSE2Collector.cpp:1483 ret.sort() | mse2_collector.rs:1571 | a | vehicle id strings |
| MSE3Collector.cpp:690 ret.sort() | mse3_collector.rs:574 | a | vehicle id strings |
| MSQueueExport.cpp:270,271 counts/lengths | ms_queue_export.rs:19 | a | doubles, equal values identical |
| MSDispatch_Greedy.cpp:53 time_sorter | ms_dispatch_greedy.rs:185 | b | max-time ties; now std_sort |
| MSPModel_Striping.cpp:771,1018,1051,2617 by_xpos | msp_model_striping.rs:955,1121,1844 | a | id tie-break (comparator read) |
| traffic_lights/NEMAController.cpp:953 greatestStartTime (`<=`) | nema_controller.rs:~2624 | b | now std_sort_vec with same `<=` |
| traffic_lights/NEMAController.cpp:1181 distance | nema_controller.rs:~923 | b | now std_sort_vec |
| traffic_lights/NEMAController.cpp:1302 distance | nema_controller.rs:~2924 | b | now std_sort_vec |

## utils / geom / netimport / misc

| C++ file:line | Rust file:line | verdict | note |
|---|---|---|---|
| utils/geom/PositionVector.cpp:668 as_poly_cw | position_vector.rs:542 | b | was stable; now std_sort_vec |
| utils/geom/PositionVector.cpp:740 increasing_x_y | position_vector.rs:548 | b | equal x,y with z differing; now std_sort_vec |
| utils/geom/PositionVector.cpp:2037 min_element y | position_vector.rs:1559 | m | strict `<` keeps first minimum; only y returned |
| netimport/NIImporter_OpenDrive.cpp:561,619 LaneSorter (abs id) | ni_importer_open_drive.rs:1581,1636 | b | abs-id ties (+/-); now std_sort_vec |
| netimport/NIImporter_OpenDrive.cpp:1771 sections_by_s | ni_importer_open_drive.rs:4735 | b | was stable; now std_sort_vec (ties on s) |
| netimport/NIImporter_OpenDrive.cpp:2989 splitPositions | ni_importer_open_drive.rs:5095 | a | doubles |
| netimport/NIImporter_OpenDrive.h:662 junctionControllers | ni_importer_open_drive.rs:3163 | a | map keys (unique strings) |
| netimport/vissim/* (6 sorts) | ni_vissim_connection_cluster.rs:64 (4 C++ sorts commented, unported); ni_vissim_edge.rs | b | same_direction_sorter: now std_sort; callers use front |
| utils/traction_wire/Circuit.cpp:672,782 std::less<int> | circuit.rs:529,580 | a | ints |
| utils/router/KDTreePartition.h:1201,1203 (node x/y) | kd_tree_partition.rs:1127 | b | ties on coordinate + equal numeric id prefix; was sort_unstable_by, now std_sort |
| foreign/RoutingKit sort.h, nested_dissection (nth_element x3), customizable_contraction_hierarchy.cpp:35 | foreign/routingkit/* | a/b | nth_element already std_nth_element (libstdc++ port); CCH sort of u32 + dedup: a |
| libsumo/*.cpp id sorts (ChargingStation, BusStop, MeanData, ParkingArea, OverheadWire, Calibrator, Person) | libsumo/*.rs | a | unique string ids |
| libsumo/Person.cpp:195 reservation_by_id | libsumo/person.rs:476 | a | reservation ids unique |
| utils/common/ToString.h:332 sorted strings | common/to_string.rs:102 | a | |
| activitygen, gui*, netedit*, guisim*, utils/gui*, Helper.cpp max_element | — | n/a | not ported |

## unqualified `sort(` (ADL resolves to std::sort) found by the second grep
Listed above where a counterpart is known; remaining unqualified sites are in: NBNode.cpp:2026/4247 (covered), NBEdge.cpp (covered), NBEdgeCont.cpp:468/990/2027 (partly), MSLane/MSEdge (covered), MSOverheadWire.cpp:105 (todo), MSPModel_Striping (covered), NIImporter_OpenDrive.cpp:1771 (todo), NIVissim (todo), RORouteDef.cpp:445 (covered).

## Priority queues, heaps, min/max, containers (ss02)

| C++ file:line | Rust file:line | verdict | note |
|---|---|---|---|
| utils/router Dijkstra/AStar/AF/AFCentralized/CH/CHBuilder/Node2Edge/SPTree push_heap/pop_heap | ch_builder.rs std_push_heap/std_pop_heap | b | libstdc++ sift ported step by step (already) |
| microsim/MSEventControl.cpp:51,65 push/pop_heap | ms_event_control.rs:23-63 | b | already step port (already) |
| microsim/MSStopOptimizer.cpp:45 priority_queue | ms_stop_optimizer.rs:559 | b | already step port (already) |
| microsim/devices/MSDevice_SSM.h:365 EncounterQueue (begin, foeID) | ms_device_ssm.rs (myPastConflicts) | b | was BinaryHeap (ties differ); now Vec + std_push_heap/std_pop_heap |
| mesosim/MELoop.h:169 LeaderEventQeue | me_loop.rs:7 | a | BinaryHeap kept: LeaderEvent::operator< uses unique eventIndex, so no ties |
| microsim/output/MSInductLoop.h:364 TimeQueue (greater) | ms_induct_loop.rs:40 | a | SUMOTime values, equal values identical |
| netbuild/NBEdge.cpp:2744 toLaneMap items, knownTargets | nb_edge.rs:10120,10242 | a | int lane indices |
| microsim/MSLink.cpp:1427 toSort (time, vehicle*) | ms_link.rs:3167 | p | pointer tie in C++, kept |
| microsim/MSStoppingPlace.cpp:155 spaces | ms_stopping_place.rs:359 | p | pointer tie in C++, kept |
| libsumo/Helper.cpp:1152 max_element(filterLanes) | libsumo helper | a | ints, only the value is used |
| microsim/MSLane.cpp:3326 min_element | ms_lane.rs:~6747 | m | first minimum kept (re-checked) |
| utils/geom/PositionVector.cpp:2037 min_element | position_vector.rs:1559 | m | first minimum kept (re-checked) |

Pointer-ordered containers (`std::set<T*>`, `std::map<T*,…>` with no comparator) keep the id/creation order in Rust, with a `// determinism:` comment (BTreeMap/BTreeSet over `Ptr`, e.g. ms_device_ssm.rs FoeInfoMap). Sites checked in this pass are listed above.

## Not finished in this pass
Entries marked **p** cannot be matched by value and remain as they were. All **todo** rows were resolved in ss02.
