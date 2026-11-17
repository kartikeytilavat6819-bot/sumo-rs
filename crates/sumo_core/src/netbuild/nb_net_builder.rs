//! Port of netbuild/NBNetBuilder.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

// TEMP-JTRACE
fn jdump(st: &str, ec: &NBEdgeCont) {
    if std::env::var_os("NC_JTRACE").is_none() {
        return;
    }
    for e in ec.myEdges.values().copied() {
        if !e.m().get_from_node().m().get_id().contains("#29more") && !e.m().get_to_node().m().get_id().contains("#29more") {
            continue;
        }
        let mut s = format!("JTRACE st {} e {} {}>{}", st, e.m().get_id(), e.m().get_from_node().m().get_id(), e.m().get_to_node().m().get_id());
        for c in e.m().get_connections().iter() {
            let to = if c.toEdge.is_null() { "null".to_string() } else { c.toEdge.m().get_id().to_string() };
            s += &format!(" {}>{}:{}:{}:{}", c.fromLane, to, c.toLane, c.permissions, c.mayDefinitelyPass as i32);
        }
        eprintln!("{}", s);
    }
}

/// Instance responsible for building networks
/// C++ class `NBNetBuilder` (netbuild/NBNetBuilder.h)
pub struct NBNetBuilder {
    /// The used container for nodes
    pub myNodeCont: NBNodeCont,
    /// The used container for street types
    pub myTypeCont: NBTypeCont,
    /// The used container for edges
    pub myEdgeCont: NBEdgeCont,
    /// The used container for traffic light logics
    pub myTLLCont: NBTrafficLightLogicCont,
    /// The used container for districts
    pub myDistrictCont: NBDistrictCont,
    /// The used container for pt stops
    pub myPTStopCont: NBPTStopCont,
    /// The used container for pt stops
    pub myPTLineCont: NBPTLineCont,
    pub myParkingCont: NBParkingCont,
    /// container for loaded polygon data
    pub myShapeCont: ShapeContainer,
    /// flag to indicate that network has crossings
    pub myNetworkHaveCrossings: bool,
}

impl NBNetBuilder {
    /// C++: `void NBNetBuilder::NBNetBuilder()`
    /// Constructor
    // NBNetBuilder::NBNetBuilder() :
    //     myEdgeCont(myTypeCont),
    //     myNetworkHaveCrossings(false) {
    // }
    #[allow(unused_variables)]
    pub fn new() -> Box<Self> {
        let mut b = Box::new(NBNetBuilder {
            myNodeCont: NBNodeCont::new(),
            myTypeCont: NBTypeCont::new(),
            // placeholder; re-pointed at the boxed builder's own myTypeCont below
            myEdgeCont: NBEdgeCont::new(&mut NBTypeCont::new()),
            myTLLCont: NBTrafficLightLogicCont::new(),
            myDistrictCont: NBDistrictCont::new(),
            myPTStopCont: NBPTStopCont { myPTStops: BTreeMap::new(), myPTStopLookup: BTreeMap::new(), myFloatingStops: Vec::new() },
            myPTLineCont: NBPTLineCont { myPTLines: BTreeMap::new(), myPTLineLookup: BTreeMap::new() },
            myParkingCont: NBParkingCont { base: Vec::new() },
            myShapeCont: ShapeContainer::new(),
            myNetworkHaveCrossings: false,
        });
        b.myEdgeCont = NBEdgeCont::new(&mut b.myTypeCont);
        b
    }

    /// C++: `void NBNetBuilder::~NBNetBuilder()`
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // implicit C++ destructor (no body): Rust drops the members
    }

    /// C++: `void NBNetBuilder::applyOptions(OptionsCont &)`
    /// Initialises the storage by applying given options
    // void
    // NBNetBuilder::applyOptions(OptionsCont& oc) {
    //     // apply options to type control
    //     myTypeCont.setEdgeTypeDefaults(oc.getInt("default.lanenumber"), oc.getFloat("default.lanewidth"), oc.getFloat("default.speed"), oc.getFloat("default.friction"),
    //                                    oc.getInt("default.priority"), parseVehicleClasses(oc.getString("default.allow"), oc.getString("default.disallow")),
    //                                    SUMOXMLDefinitions::LaneSpreadFunctions.get(oc.getString("default.spreadtype")));
    //     // apply options to edge control
    //     myEdgeCont.applyOptions(oc);
    //     // apply options to traffic light logics control
    //     myTLLCont.applyOptions(oc);
    //     NBEdge::setDefaultConnectionLength(oc.getFloat("default.connection-length"));
    // }
    #[allow(unused_variables)]
    pub fn apply_options(&mut self, oc: &mut OptionsCont) {
        // apply options to type control
        self.myTypeCont.set_edge_type_defaults(
            oc.get_int("default.lanenumber").unwrap(),
            oc.get_float("default.lanewidth").unwrap(),
            oc.get_float("default.speed").unwrap(),
            oc.get_float("default.friction").unwrap(),
            oc.get_int("default.priority").unwrap(),
            parse_vehicle_classes_allow_disallow(&oc.get_string("default.allow").unwrap(), &oc.get_string("default.disallow").unwrap(), utils::common::std_defs::NETWORK_VERSION),
            LaneSpreadFunctions.get(&oc.get_string("default.spreadtype").unwrap()).unwrap(),
        );
        // apply options to edge control
        self.myEdgeCont.apply_options(oc);
        // apply options to traffic light logics control
        self.myTLLCont.apply_options(oc);
        NBEdge::set_default_connection_length(oc.get_float("default.connection-length").unwrap());
    }

    /// C++: `void NBNetBuilder::compute(OptionsCont &, const std::set<std::string> &, bool)`
    /// Performs the network building steps
    // void
    // NBNetBuilder::compute(OptionsCont& oc, const std::set<std::string>& explicitTurnarounds, bool mayAddOrRemove) {
    //     GeoConvHelper& geoConvHelper = GeoConvHelper::getProcessing();
    //
    //     const bool lefthand = oc.getBool("lefthand");
    //     if (lefthand) {
    //         mirrorX();
    //     }
    //
    //     // MODIFYING THE SETS OF NODES AND EDGES
    //     // Removes edges that are connecting the same node
    //     long before = PROGRESS_BEGIN_TIME_MESSAGE(TL("Removing self-loops"));
    //     int numRemovedEdges = 0;
    //     numRemovedEdges += myNodeCont.removeSelfLoops(myDistrictCont, myEdgeCont, myTLLCont);
    //     PROGRESS_TIME_MESSAGE(before);
    //     if (mayAddOrRemove && oc.exists("remove-edges.isolated") && oc.getBool("remove-edges.isolated")) {
    //         before = PROGRESS_BEGIN_TIME_MESSAGE(TL("Finding isolated roads"));
    //         numRemovedEdges += myNodeCont.removeIsolatedRoads(myDistrictCont, myEdgeCont);
    //         PROGRESS_TIME_MESSAGE(before);
    //     }
    //     if (mayAddOrRemove && oc.exists("keep-edges.components") && oc.getInt("keep-edges.components") > 0) {
    //         before = PROGRESS_BEGIN_TIME_MESSAGE(TL("Finding largest components"));
    //         const bool hasStops = oc.exists("ptstop-output") && oc.isSet("ptstop-output") && !myPTStopCont.getStops().empty();
    //         numRemovedEdges += myNodeCont.removeComponents(myDistrictCont, myEdgeCont, oc.getInt("keep-edges.components"), hasStops);
    //         PROGRESS_TIME_MESSAGE(before);
    //     }
    //     if (mayAddOrRemove && oc.exists("keep-edges.postload") && oc.getBool("keep-edges.postload")) {
    //         // pre-process lines to set permissions
    //         if (!myPTLineCont.getLines().empty()) {
    //             before = PROGRESS_BEGIN_TIME_MESSAGE(TL("Revising public transport stops based on pt lines"));
    //             myPTLineCont.process(myEdgeCont, myPTStopCont);
    //             PROGRESS_TIME_MESSAGE(before);
    //         }
    //         if (oc.isSet("keep-edges.explicit") || oc.isSet("keep-edges.input-file")) {
    //             before = PROGRESS_BEGIN_TIME_MESSAGE(TL("Removing unwished edges"));
    //             numRemovedEdges += myEdgeCont.removeUnwishedEdges(myDistrictCont);
    //             PROGRESS_TIME_MESSAGE(before);
    //         }
    //         const int removed = myEdgeCont.removeEdgesBySpeed(myDistrictCont);
    //         if (removed > 0) {
    //             numRemovedEdges += removed;
    //             WRITE_MESSAGEF(TL(" Removed % edges because by minimum speed."), removed);
    //         }
    //         const int removed2 = myEdgeCont.removeEdgesByPermissions(myDistrictCont);
    //         if (removed2 > 0) {
    //             numRemovedEdges += removed2;
    //             WRITE_MESSAGEF(TL(" Removed % edges based on vClass."), removed2);
    //         }
    //     }
    //     if (mayAddOrRemove && oc.getFloat("keep-lanes.min-width") > 0.) {
    //         const int removed = myEdgeCont.removeLanesByWidth(myDistrictCont, oc.getFloat("keep-lanes.min-width"));
    //         if (removed > 0) {
    //             numRemovedEdges += removed;
    //             WRITE_MESSAGEF(TL(" Removed % edges because of lane width."), removed);
    //         }
    //     }
    //     if (mayAddOrRemove && oc.exists("junctions.attach-removed") && oc.getFloat("junctions.attach-removed") >= 0) {
    //         const int numSplit = myEdgeCont.attachRemoved(myNodeCont, myDistrictCont, oc.getFloat("junctions.attach-removed"));
    //         if (numSplit > 0) {
    //             WRITE_MESSAGEF(TL(" Split % edges to attach removed nodes"), numSplit);
    //         }
    //     }
    //     // Processing pt stops and lines
    //     if (!myPTStopCont.getStops().empty()) {
    //         before = PROGRESS_BEGIN_TIME_MESSAGE(TL("Processing public transport stops"));
    //         if (!(oc.exists("ptline-output") && oc.isSet("ptline-output"))
    //                 && !oc.getBool("ptstop-output.no-bidi")) {
    //             myPTStopCont.localizePTStops(myEdgeCont);
    //         }
    //         myPTStopCont.assignEdgeForFloatingStops(myEdgeCont, 20);
    //         myPTStopCont.assignLanes(myEdgeCont);
    //         PROGRESS_TIME_MESSAGE(before);
    //     }
    //     if (mayAddOrRemove && oc.exists("keep-edges.components") && oc.getInt("keep-edges.components") > 0) {
    //         // post process rail components unless they have stops
    //         numRemovedEdges += myNodeCont.removeRailComponents(myDistrictCont, myEdgeCont, myPTStopCont);
    //     }
    //     // removal is done, clean up roundabouts
    //     if (numRemovedEdges > 0) {
    //         myEdgeCont.cleanupRoundabouts();
    //     }
    //
    //     if (!myPTLineCont.getLines().empty()) {
    //         before = PROGRESS_BEGIN_TIME_MESSAGE(TL("Revising public transport stops based on pt lines"));
    //         myPTLineCont.process(myEdgeCont, myPTStopCont);
    //         PROGRESS_TIME_MESSAGE(before);
    //     }
    //
    //     if (!myPTStopCont.getStops().empty() && !oc.getBool("ptstop-output.no-bidi")) {
    //         before = PROGRESS_BEGIN_TIME_MESSAGE(TL("Align pt stop id signs with corresponding edge id signs"));
    //         myPTStopCont.alignIdSigns();
    //         PROGRESS_TIME_MESSAGE(before);
    //     }
    //     // analyze and fix railway topology
    //     int numAddedBidi = 0;
    //     if (oc.exists("railway.topology.all-bidi") && oc.getBool("railway.topology.all-bidi")) {
    //         NBTurningDirectionsComputer::computeTurnDirections(myNodeCont, false);
    //         numAddedBidi = NBRailwayTopologyAnalyzer::makeAllBidi(myEdgeCont);
    //     } else if (oc.exists("railway.topology.repair") && oc.getBool("railway.topology.repair")) {
    //         // correct railway angles for angle-based connectivity heuristic
    //         myEdgeCont.checkGeometries(0, false,
    //                                    oc.getFloat("geometry.min-radius"), false,
    //                                    oc.getBool("geometry.min-radius.fix.railways"), true);
    //         NBTurningDirectionsComputer::computeTurnDirections(myNodeCont, false);
    //         numAddedBidi = NBRailwayTopologyAnalyzer::repairTopology(myEdgeCont, myPTStopCont, myPTLineCont);
    //     }
    //     NBRailwaySignalGuesser::guessRailSignals(myNodeCont, myEdgeCont, myPTStopCont, myDistrictCont);
    //     if (numAddedBidi > 0) {
    //         // update routes
    //         myPTLineCont.process(myEdgeCont, myPTStopCont, true);
    //     }
    //     if (oc.exists("railway.topology.direction-priority") && oc.getBool("railway.topology.direction-priority")) {
    //         NBTurningDirectionsComputer::computeTurnDirections(myNodeCont, false); // recompute after new edges were added
    //         NBRailwayTopologyAnalyzer::extendDirectionPriority(myEdgeCont, true);
    //     } else if (oc.exists("railway.topology.extend-priority")
    //             && (oc.getBool("railway.topology.extend-priority") || oc.getStringVector("railway.topology.ptline-priority").size() > 0)) {
    //         NBTurningDirectionsComputer::computeTurnDirections(myNodeCont, false); // recompute after new edges were added
    //         if (oc.getStringVector("railway.topology.ptline-priority").size() > 0) {
    //             SVCPermissions vClasses = parseVehicleClasses(oc.getStringVector("railway.topology.ptline-priority"));
    //             NBRailwayTopologyAnalyzer::setPTLinePriority(myEdgeCont, myPTLineCont, vClasses);
    //         }
    //         NBRailwayTopologyAnalyzer::extendDirectionPriority(myEdgeCont, false);
    //     }
    //     if (oc.exists("railway.topology.output") && oc.isSet("railway.topology.output")) {
    //         NBTurningDirectionsComputer::computeTurnDirections(myNodeCont, false); // recompute after new edges were added
    //         NBRailwayTopologyAnalyzer::analyzeTopology(myEdgeCont);
    //     }
    //     if (oc.exists("railway.geometry.straighten") && oc.getBool("railway.geometry.straighten")) {
    //         NBTurningDirectionsComputer::computeTurnDirections(myNodeCont, false); // recompute after new edges were added
    //         NBRailwayGeometryHelper::straigthenCorrdidor(myEdgeCont, oc.getFloat("geometry.max-angle"));
    //     }
    //
    //
    //     if (mayAddOrRemove && oc.exists("edges.join-tram-dist") && oc.getFloat("edges.join-tram-dist") >= 0) {
    //         // should come before joining junctions
    //         before = PROGRESS_BEGIN_TIME_MESSAGE(TL("Joining tram edges"));
    //         int numJoinedTramEdges = myEdgeCont.joinTramEdges(myDistrictCont, myPTStopCont, myPTLineCont, oc.getFloat("edges.join-tram-dist"));
    //         PROGRESS_TIME_MESSAGE(before);
    //         if (numJoinedTramEdges > 0) {
    //             WRITE_MESSAGEF(TL(" Joined % tram edges into roads."), toString(numJoinedTramEdges));
    //         }
    //     }
    //     if (oc.getBool("junctions.join")
    //             || (oc.exists("ramps.guess") && oc.getBool("ramps.guess"))
    //             || oc.getBool("tls.guess.joining")
    //             || (oc.exists("tls.guess-signals") && oc.getBool("tls.guess-signals"))) {
    //         // preliminary geometry computations to determine the length of edges
    //         // This depends on turning directions and sorting of edge list
    //         // in case junctions are joined geometry computations have to be repeated
    //         // preliminary roundabout computations to avoid damaging roundabouts via junctions.join or ramps.guess
    //         NBTurningDirectionsComputer::computeTurnDirections(myNodeCont, false);
    //         NBNodesEdgesSorter::sortNodesEdges(myNodeCont);
    //         myEdgeCont.computeLaneShapes();
    //         myNodeCont.computeNodeShapes();
    //         myEdgeCont.computeEdgeShapes();
    //         if (oc.getBool("roundabouts.guess")) {
    //             myEdgeCont.guessRoundabouts();
    //         }
    //         const std::set<EdgeSet>& roundabouts = myEdgeCont.getRoundabouts();
    //         for (std::set<EdgeSet>::const_iterator it_round = roundabouts.begin();
    //                 it_round != roundabouts.end(); ++it_round) {
    //             std::vector<std::string> nodeIDs;
    //             for (EdgeSet::const_iterator it_edge = it_round->begin(); it_edge != it_round->end(); ++it_edge) {
    //                 nodeIDs.push_back((*it_edge)->getToNode()->getID());
    //             }
    //             myNodeCont.addJoinExclusion(nodeIDs);
    //         }
    //         NBNodeTypeComputer::validateRailCrossings(myNodeCont, myTLLCont);
    //     } else if ((myEdgeCont.hasGuessedRoundabouts() || oc.getBool("crossings.guess")) && oc.getBool("roundabouts.guess")) {
    //         myEdgeCont.guessRoundabouts();
    //         myEdgeCont.markRoundabouts();
    //     }
    //     // join junctions (may create new "geometry"-nodes so it needs to come before removing these
    //     if (mayAddOrRemove && oc.exists("junctions.join-exclude") && oc.isSet("junctions.join-exclude")) {
    //         myNodeCont.addJoinExclusion(oc.getStringVector("junctions.join-exclude"));
    //     }
    //     int numJoined = myNodeCont.joinLoadedClusters(myDistrictCont, myEdgeCont, myTLLCont);
    //     if (mayAddOrRemove && oc.getBool("junctions.join")) {
    //         before = PROGRESS_BEGIN_TIME_MESSAGE(TL("Joining junction clusters"));
    //         numJoined += myNodeCont.joinJunctions(oc.getFloat("junctions.join-dist"), myDistrictCont, myEdgeCont, myTLLCont, myPTStopCont);
    //         PROGRESS_TIME_MESSAGE(before);
    //     }
    //     if (numJoined > 0) {
    //         WRITE_MESSAGEF(TL(" Joined % junction cluster(s)."), toString(numJoined));
    //     }
    //     if (mayAddOrRemove && oc.getFloat("junctions.join-same") >= 0) {
    //         before = PROGRESS_BEGIN_TIME_MESSAGE(TL("Joining junctions with similar coordinates"));
    //         int numJoined2 = myNodeCont.joinSameJunctions(myDistrictCont, myEdgeCont, myTLLCont, oc.getFloat("junctions.join-same"));
    //         PROGRESS_TIME_MESSAGE(before);
    //         if (numJoined2 > 0) {
    //             WRITE_MESSAGEF(TL(" Joined % junctions."), toString(numJoined2));
    //         }
    //     }
    //     //
    //     if (mayAddOrRemove && oc.exists("join-lanes") && oc.getBool("join-lanes")) {
    //         before = PROGRESS_BEGIN_TIME_MESSAGE(TL("Joining lanes"));
    //         const int num = myEdgeCont.joinLanes(SVC_IGNORING) + myEdgeCont.joinLanes(SVC_PEDESTRIAN);
    //         PROGRESS_TIME_MESSAGE(before);
    //         WRITE_MESSAGEF(TL("   Joined lanes on % edges."), toString(num));
    //     }
    //     //
    //     if (mayAddOrRemove) {
    //         const bool removeGeometryNodes = oc.exists("geometry.remove") && oc.getBool("geometry.remove");
    //         before = PROGRESS_BEGIN_TIME_MESSAGE("Removing empty nodes" + std::string(removeGeometryNodes ? " and geometry nodes" : ""));
    //         // removeUnwishedNodes needs turnDirections. @todo: try to call this less often
    //         NBTurningDirectionsComputer::computeTurnDirections(myNodeCont, false);
    //         const int numRemoved = myNodeCont.removeUnwishedNodes(myDistrictCont, myEdgeCont, myTLLCont, myPTStopCont, myPTLineCont, myParkingCont, removeGeometryNodes);
    //         PROGRESS_TIME_MESSAGE(before);
    //         WRITE_MESSAGEF(TL("   % nodes removed."), toString(numRemoved));
    //     }
    //
    //     // MOVE TO ORIGIN
    //     // compute new boundary after network modifications have taken place
    //     Boundary boundary;
    //     for (std::map<std::string, NBNode*>::const_iterator it = myNodeCont.begin(); it != myNodeCont.end(); ++it) {
    //         boundary.add(it->second->getPosition());
    //     }
    //     for (std::map<std::string, NBEdge*>::const_iterator it = myEdgeCont.begin(); it != myEdgeCont.end(); ++it) {
    //         boundary.add(it->second->getGeometry().getBoxBoundary());
    //     }
    //     geoConvHelper.setConvBoundary(boundary);
    //
    //     if (!oc.getBool("offset.disable-normalization") && oc.isDefault("offset.x") && oc.isDefault("offset.y")) {
    //         moveToOrigin(geoConvHelper, lefthand);
    //     }
    //     roundInputs();
    //     geoConvHelper.computeFinal(lefthand); // information needed for location element fixed at this point
    //
    //     // reset shapes and angles for stable re-computation
    //     if (myNodeCont.resetNodeShapes()) {
    //         myEdgeCont.computeAngles();
    //     }
    //
    //     if (oc.exists("geometry.min-dist") && !oc.isDefault("geometry.min-dist")) {
    //         before = PROGRESS_BEGIN_TIME_MESSAGE(TL("Reducing geometries"));
    //         myEdgeCont.reduceGeometries(oc.getFloat("geometry.min-dist"));
    //         PROGRESS_TIME_MESSAGE(before);
    //     }
    //     // @note: removing geometry can create similar edges so joinSimilarEdges  must come afterwards
    //     // @note: likewise splitting can destroy similarities so joinSimilarEdges must come before
    //     if (mayAddOrRemove && oc.getBool("edges.join")) {
    //         before = PROGRESS_BEGIN_TIME_MESSAGE(TL("Joining similar edges"));
    //         const bool removeDuplicates = oc.getFloat("junctions.join-same") >= 0;
    //         myNodeCont.joinSimilarEdges(myDistrictCont, myEdgeCont, myTLLCont, removeDuplicates);
    //         // now we may have new chances to remove geometry if wished
    //         if (oc.exists("geometry.remove") && oc.getBool("geometry.remove")) {
    //             myNodeCont.removeUnwishedNodes(myDistrictCont, myEdgeCont, myTLLCont, myPTStopCont, myPTLineCont, myParkingCont, true);
    //         }
    //         PROGRESS_TIME_MESSAGE(before);
    //     }
    //     if (oc.getBool("opposites.guess")) {
    //         PROGRESS_BEGIN_MESSAGE(TL("guessing opposite direction edges"));
    //         myEdgeCont.guessOpposites();
    //         PROGRESS_DONE_MESSAGE();
    //     }
    //     //
    //     if (mayAddOrRemove && oc.exists("geometry.split") && oc.getBool("geometry.split")) {
    //         before = PROGRESS_BEGIN_TIME_MESSAGE(TL("Splitting geometry edges"));
    //         myEdgeCont.splitGeometry(myDistrictCont, myNodeCont);
    //         // newly split junctions might also be joinable
    //         PROGRESS_TIME_MESSAGE(before);
    //         if (oc.getFloat("junctions.join-same") >= 0) {
    //             int numJoined3 = myNodeCont.joinSameJunctions(myDistrictCont, myEdgeCont, myTLLCont, oc.getFloat("junctions.join-same"));
    //             if (numJoined3 > 0) {
    //                 WRITE_MESSAGEF(TL(" Joined % junctions after splitting geometry."), toString(numJoined3));
    //             }
    //         }
    //     }
    //     // turning direction
    //     before = PROGRESS_BEGIN_TIME_MESSAGE(TL("Computing turning directions"));
    //     NBTurningDirectionsComputer::computeTurnDirections(myNodeCont);
    //     PROGRESS_TIME_MESSAGE(before);
    //     // correct edge geometries to avoid overlap
    //     if (oc.exists("geometry.avoid-overlap") && oc.getBool("geometry.avoid-overlap")) {
    //         myNodeCont.avoidOverlap();
    //     }
    //
    //     // GUESS TLS POSITIONS
    //     before = PROGRESS_BEGIN_TIME_MESSAGE(TL("Assigning nodes to traffic lights"));
    //     if (oc.isSet("tls.set")) {
    //         std::vector<std::string> tlControlledNodes = oc.getStringVector("tls.set");
    //         TrafficLightType type = SUMOXMLDefinitions::TrafficLightTypes.get(oc.getString("tls.default-type"));
    //         for (std::vector<std::string>::const_iterator i = tlControlledNodes.begin(); i != tlControlledNodes.end(); ++i) {
    //             NBNode* node = myNodeCont.retrieve(*i);
    //             if (node == nullptr) {
    //                 WRITE_WARNING("Building a tl-logic for junction '" + *i + "' is not possible." + "\n The junction '" + *i + "' is not known.");
    //             } else {
    //                 myNodeCont.setAsTLControlled(node, myTLLCont, type);
    //             }
    //         }
    //     }
    //     myNodeCont.guessTLs(oc, myTLLCont);
    //     PROGRESS_TIME_MESSAGE(before);
    //
    //     // guess ramps (after guessing tls because ramps should not be build at traffic lights)
    //     const bool modifyRamps = mayAddOrRemove && (
    //                                  (oc.exists("ramps.guess") && oc.getBool("ramps.guess"))
    //                                  || (oc.exists("ramps.set") && oc.isSet("ramps.set")));
    //     if (modifyRamps || (oc.exists("ramps.guess-acceleration-lanes") && oc.getBool("ramps.guess-acceleration-lanes"))) {
    //         before = SysUtils::getCurrentMillis();
    //         if (modifyRamps) {
    //             PROGRESS_BEGIN_MESSAGE(TL("Guessing and setting on-/off-ramps"));
    //         }
    //         NBNodesEdgesSorter::sortNodesEdges(myNodeCont);
    //         NBRampsComputer rc;
    //         rc.computeRamps(*this, oc, mayAddOrRemove);
    //
    //         if (modifyRamps) {
    //             PROGRESS_TIME_MESSAGE(before);
    //         }
    //     }
    //     // guess bike lanes
    //     if (mayAddOrRemove && ((oc.getBool("bikelanes.guess") || oc.getBool("bikelanes.guess.from-permissions")))) {
    //         const int bikelanes = myEdgeCont.guessSpecialLanes(SVC_BICYCLE, oc.getFloat("default.bikelane-width"),
    //                               oc.getFloat("bikelanes.guess.min-speed"),
    //                               oc.getFloat("bikelanes.guess.max-speed"),
    //                               oc.getBool("bikelanes.guess.from-permissions"),
    //                               "bikelanes.guess.exclude",
    //                               myTLLCont);
    //         WRITE_MESSAGEF(TL("Guessed % bike lanes."), toString(bikelanes));
    //     }
    //
    //     // guess sidewalks
    //     if (mayAddOrRemove && ((oc.getBool("sidewalks.guess") || oc.getBool("sidewalks.guess.from-permissions")))) {
    //         const int sidewalks = myEdgeCont.guessSpecialLanes(SVC_PEDESTRIAN, oc.getFloat("default.sidewalk-width"),
    //                               oc.getFloat("sidewalks.guess.min-speed"),
    //                               oc.getFloat("sidewalks.guess.max-speed"),
    //                               oc.getBool("sidewalks.guess.from-permissions"),
    //                               "sidewalks.guess.exclude",
    //                               myTLLCont);
    //         WRITE_MESSAGEF(TL("Guessed % sidewalks."), toString(sidewalks));
    //     }
    //     // check whether any not previously setable connections may be set now
    //     myEdgeCont.recheckPostProcessConnections();
    //
    //     // remap ids if wished
    //     if (mayAddOrRemove) {
    //         const bool numericalIDs = oc.getBool("numerical-ids");
    //         const bool reservedIDs = oc.isSet("reserved-ids");
    //         const bool keptIDs = oc.isSet("kept-ids");
    //         int numChangedEdges = myEdgeCont.remapIDs(numericalIDs, reservedIDs, keptIDs, oc.getString("prefix.edge"), myPTStopCont);
    //         int numChangedNodes = myNodeCont.remapIDs(numericalIDs, reservedIDs, keptIDs, oc.getString("prefix.junction"), myTLLCont);
    //         if (numChangedEdges + numChangedNodes > 0) {
    //             WRITE_MESSAGEF(TL("Remapped % edge IDs and % node IDs."), toString(numChangedEdges), toString(numChangedNodes));
    //         }
    //     }
    //
    //     //
    //     if (oc.exists("geometry.max-angle")) {
    //         myEdgeCont.checkGeometries(
    //             DEG2RAD(oc.getFloat("geometry.max-angle")),
    //             oc.getBool("geometry.max-angle.fix"),
    //             oc.getFloat("geometry.min-radius"),
    //             oc.getBool("geometry.min-radius.fix"),
    //             oc.getBool("geometry.min-radius.fix.railways"));
    //     }
    //
    //     // GEOMETRY COMPUTATION
    //     //
    //     before = PROGRESS_BEGIN_TIME_MESSAGE(TL("Sorting nodes' edges"));
    //     NBNodesEdgesSorter::sortNodesEdges(myNodeCont);
    //     PROGRESS_TIME_MESSAGE(before);
    //     myEdgeCont.computeLaneShapes();
    //     //
    //     before = PROGRESS_BEGIN_TIME_MESSAGE(TL("Computing node shapes"));
    //     if (oc.exists("geometry.junction-mismatch-threshold")) {
    //         myNodeCont.computeNodeShapes(oc.getFloat("geometry.junction-mismatch-threshold"));
    //     } else {
    //         myNodeCont.computeNodeShapes();
    //     }
    //     PROGRESS_TIME_MESSAGE(before);
    //     //
    //     before = PROGRESS_BEGIN_TIME_MESSAGE(TL("Computing edge shapes"));
    //     myEdgeCont.computeEdgeShapes(oc.getBool("geometry.max-grade.fix") ? oc.getFloat("geometry.max-grade") / 100 : -1);
    //     PROGRESS_TIME_MESSAGE(before);
    //     // resort edges based on the node and edge shapes
    //     NBNodesEdgesSorter::sortNodesEdges(myNodeCont, true);
    //     NBTurningDirectionsComputer::computeTurnDirections(myNodeCont, false);
    //
    //     // APPLY SPEED MODIFICATIONS
    //     if (oc.exists("speed.offset")) {
    //         const double speedOffset = oc.getFloat("speed.offset");
    //         const double speedFactor = oc.getFloat("speed.factor");
    //         const double speedMin = oc.getFloat("speed.minimum");
    //         if (speedOffset != 0 || speedFactor != 1 || speedMin > 0) {
    //             before = PROGRESS_BEGIN_TIME_MESSAGE(TL("Applying speed modifications"));
    //             for (const auto& it : myEdgeCont) {
    //                 NBEdge* const e = it.second;
    //                 for (int i = 0; i < e->getNumLanes(); i++) {
    //                     e->setSpeed(i, MAX2(e->getLaneSpeed(i) * speedFactor + speedOffset, speedMin));
    //                 }
    //             }
    //             PROGRESS_TIME_MESSAGE(before);
    //         }
    //     }
    //
    //     // CONNECTIONS COMPUTATION
    //     //
    //     before = PROGRESS_BEGIN_TIME_MESSAGE(TL("Computing node types"));
    //     NBNode::initRailSignalClasses(myNodeCont);
    //     NBNodeTypeComputer::computeNodeTypes(myNodeCont, myTLLCont);
    //     PROGRESS_TIME_MESSAGE(before);
    //     //
    //     myNetworkHaveCrossings = oc.getBool("walkingareas");
    //     if (mayAddOrRemove && oc.getBool("crossings.guess")) {
    //         myNetworkHaveCrossings = true;
    //         int crossings = 0;
    //         for (std::map<std::string, NBNode*>::const_iterator i = myNodeCont.begin(); i != myNodeCont.end(); ++i) {
    //             crossings += (*i).second->guessCrossings();
    //         }
    //         WRITE_MESSAGEF(TL("Guessed % pedestrian crossings."), toString(crossings));
    //     }
    //     if (!myNetworkHaveCrossings) {
    //         bool haveValidCrossings = false;
    //         // recheck whether we had crossings in the input
    //         for (std::map<std::string, NBNode*>::const_iterator i = myNodeCont.begin(); i != myNodeCont.end(); ++i) {
    //             if (i->second->getCrossings().size() > 0) {
    //                 myNetworkHaveCrossings = true;
    //                 haveValidCrossings = true;
    //                 break;
    //             } else if (i->second->getCrossingsIncludingInvalid().size() > 0) {
    //                 myNetworkHaveCrossings = true;
    //             }
    //         }
    //         if (myNetworkHaveCrossings && !haveValidCrossings) {
    //             // initial crossings removed or invalidated, keep walkingareas
    //             oc.resetWritable();
    //             oc.set("walkingareas", "true");
    //         }
    //     }
    //
    //     if (!mayAddOrRemove && myNetworkHaveCrossings) {
    //         // crossings added via netedit
    //         oc.resetWritable();
    //         oc.set("no-internal-links", "false");
    //     }
    //
    //     //
    //     before = PROGRESS_BEGIN_TIME_MESSAGE(TL("Computing priorities"));
    //     NBEdgePriorityComputer::computeEdgePriorities(myNodeCont);
    //     PROGRESS_TIME_MESSAGE(before);
    //     //
    //     before = PROGRESS_BEGIN_TIME_MESSAGE(TL("Computing approached edges"));
    //     myEdgeCont.computeEdge2Edges(oc.getBool("no-left-connections"));
    //     PROGRESS_TIME_MESSAGE(before);
    //     //
    //     if (oc.getBool("roundabouts.guess")) {
    //         before = PROGRESS_BEGIN_TIME_MESSAGE(TL("Guessing and setting roundabouts"));
    //         const int numGuessed = myEdgeCont.guessRoundabouts();
    //         if (numGuessed > 0) {
    //             WRITE_MESSAGEF(TL(" Guessed % roundabout(s)."), toString(numGuessed));
    //         }
    //         PROGRESS_TIME_MESSAGE(before);
    //     }
    //     myEdgeCont.markRoundabouts();
    //     //
    //     before = PROGRESS_BEGIN_TIME_MESSAGE(TL("Computing approaching lanes"));
    //     myEdgeCont.computeLanes2Edges();
    //     PROGRESS_TIME_MESSAGE(before);
    //     //
    //     before = PROGRESS_BEGIN_TIME_MESSAGE(TL("Dividing of lanes on approached lanes"));
    //     myNodeCont.computeLanes2Lanes();
    //     myEdgeCont.sortOutgoingLanesConnections();
    //     PROGRESS_TIME_MESSAGE(before);
    //     //
    //     if (oc.getBool("fringe.guess")) {
    //         before = PROGRESS_BEGIN_TIME_MESSAGE(TL("Guessing Network fringe"));
    //         const int numGuessed = myNodeCont.guessFringe();
    //         if (numGuessed > 0) {
    //             WRITE_MESSAGEF(TL(" Guessed % fringe nodes."), toString(numGuessed));
    //         }
    //         PROGRESS_TIME_MESSAGE(before);
    //     }
    //     //
    //     before = PROGRESS_BEGIN_TIME_MESSAGE(TL("Processing turnarounds"));
    //     if (!oc.getBool("no-turnarounds")) {
    //         myEdgeCont.appendTurnarounds(
    //             oc.getBool("no-turnarounds.tls"),
    //             oc.getBool("no-turnarounds.fringe"),
    //             oc.getBool("no-turnarounds.except-deadend"),
    //             oc.getBool("no-turnarounds.except-turnlane"),
    //             oc.getBool("no-turnarounds.geometry"));
    //     } else {
    //         myEdgeCont.appendTurnarounds(explicitTurnarounds, oc.getBool("no-turnarounds.tls"));
    //     }
    //     if (oc.exists("railway.topology.repair.stop-turn") && oc.getBool("railway.topology.repair.stop-turn")
    //             && myPTStopCont.getStops().size() > 0) {
    //         // allow direction reversal at all bidi-edges with stops
    //         myEdgeCont.appendRailwayTurnarounds(myPTStopCont);
    //     }
    //     PROGRESS_TIME_MESSAGE(before);
    //     //
    //     before = PROGRESS_BEGIN_TIME_MESSAGE(TL("Rechecking of lane endings"));
    //     myEdgeCont.recheckLanes();
    //     PROGRESS_TIME_MESSAGE(before);
    //
    //     if (myNetworkHaveCrossings && !oc.getBool("no-internal-links")) {
    //         for (std::map<std::string, NBNode*>::const_iterator i = myNodeCont.begin(); i != myNodeCont.end(); ++i) {
    //             i->second->buildCrossingsAndWalkingAreas();
    //         }
    //     } else {
    //         for (std::map<std::string, NBNode*>::const_iterator i = myNodeCont.begin(); i != myNodeCont.end(); ++i) {
    //             // needed by netedit if the last crossings was deleted from the network
    //             // and walkingareas have been invalidated since the last call to compute()
    //             i->second->discardWalkingareas();
    //         }
    //         if (oc.getBool("no-internal-links")) {
    //             for (std::map<std::string, NBNode*>::const_iterator i = myNodeCont.begin(); i != myNodeCont.end(); ++i) {
    //                 i->second->discardAllCrossings(false);
    //             }
    //         }
    //     }
    //     // join traffic lights (after building connections)
    //     if (oc.getBool("tls.join")) {
    //         before = PROGRESS_BEGIN_TIME_MESSAGE(TL("Joining traffic light nodes"));
    //         myNodeCont.joinTLS(myTLLCont, oc.getFloat("tls.join-dist"));
    //         PROGRESS_TIME_MESSAGE(before);
    //     }
    //
    //     // COMPUTING RIGHT-OF-WAY AND TRAFFIC LIGHT PROGRAMS
    //     //
    //     before = PROGRESS_BEGIN_TIME_MESSAGE(TL("Computing traffic light control information"));
    //     myTLLCont.setTLControllingInformation(myEdgeCont, myNodeCont);
    //     if (oc.exists("opendrive-files") && oc.isSet("opendrive-files")) {
    //         myTLLCont.setOpenDriveSignalParameters();
    //     }
    //     PROGRESS_TIME_MESSAGE(before);
    //     //
    //     before = PROGRESS_BEGIN_TIME_MESSAGE(TL("Computing node logics"));
    //     myNodeCont.computeLogics(myEdgeCont);
    //     PROGRESS_TIME_MESSAGE(before);
    //
    //     //
    //     before = PROGRESS_BEGIN_TIME_MESSAGE(TL("Computing traffic light logics"));
    //     std::pair<int, int> numbers = myTLLCont.computeLogics(oc);
    //     PROGRESS_TIME_MESSAGE(before);
    //     std::string progCount = "";
    //     if (numbers.first != numbers.second) {
    //         progCount = "(" + toString(numbers.second) + " programs) ";
    //     }
    //     WRITE_MESSAGEF(TL(" % traffic light(s) %computed."), toString(numbers.first), progCount);
    //     if (oc.exists("opendrive-files") && oc.isSet("opendrive-files") && oc.getBool("opendrive.signal-groups")) {
    //         myTLLCont.applyOpenDriveControllers(oc);
    //     }
    //
    //     for (std::map<std::string, NBEdge*>::const_iterator i = myEdgeCont.begin(); i != myEdgeCont.end(); ++i) {
    //         (*i).second->sortOutgoingConnectionsByIndex();
    //     }
    //     // FINISHING INNER EDGES
    //     std::set<NBTrafficLightDefinition*> largeNodeTLS;
    //     if (!oc.getBool("no-internal-links")) {
    //         before = PROGRESS_BEGIN_TIME_MESSAGE(TL("Building inner edges"));
    //         // walking areas shall only be built if crossings are wished as well
    //         for (const auto& item : myNodeCont) {
    //             if (item.second->buildInnerEdges() > NBTrafficLightDefinition::MIN_YELLOW_SECONDS) {
    //                 const std::set<NBTrafficLightDefinition*>& tlDefs = item.second->getControllingTLS();
    //                 largeNodeTLS.insert(tlDefs.begin(), tlDefs.end());
    //             }
    //         }
    //         PROGRESS_TIME_MESSAGE(before);
    //     }
    //     // PATCH NODE SHAPES
    //     if (oc.getFloat("junctions.scurve-stretch") > 0) {
    //         // @note: nodes have collected correction hints in buildInnerEdges()
    //         before = PROGRESS_BEGIN_TIME_MESSAGE(TL("stretching junctions to smooth geometries"));
    //         myEdgeCont.computeLaneShapes();
    //         myNodeCont.computeNodeShapes();
    //         myEdgeCont.computeEdgeShapes(oc.getBool("geometry.max-grade.fix") ? oc.getFloat("geometry.max-grade") / 100 : -1);
    //         for (const auto& item : myNodeCont) {
    //             item.second->buildInnerEdges();
    //         }
    //         PROGRESS_TIME_MESSAGE(before);
    //     }
    //     if (myEdgeCont.getNumEdgeSplits() > 0 && !oc.getBool("no-internal-links")) {
    //         // edges with custom lengths were split, this has to take into account
    //         // internal edge lengths (after geometry computation)
    //         myEdgeCont.fixSplitCustomLength();
    //     }
    //     // recheck phases for large junctions
    //     for (NBTrafficLightDefinition* def : largeNodeTLS) {
    //         myTLLCont.computeSingleLogic(oc, def);
    //     }
    //     // compute lane-to-lane node logics (require traffic lights and inner edges to be done)
    //     myNodeCont.computeLogics2(myEdgeCont, oc);
    //
    //     // remove guessed traffic lights at junctions without conflicts (requires computeLogics2)
    //     myNodeCont.recheckGuessedTLS(myTLLCont);
    //
    //     // compute keepClear status (requires computeLogics2)
    //     myNodeCont.computeKeepClear();
    //
    //     //
    //     if (oc.isSet("street-sign-output")) {
    //         before = PROGRESS_BEGIN_TIME_MESSAGE(TL("Generating street signs"));
    //         myEdgeCont.generateStreetSigns();
    //         PROGRESS_TIME_MESSAGE(before);
    //     }
    //
    //
    //     if (lefthand != oc.getBool("flip-y-axis")) {
    //         mirrorX();
    //     }
    //
    //     if (oc.exists("geometry.check-overlap")  && oc.getFloat("geometry.check-overlap") > 0) {
    //         before = PROGRESS_BEGIN_TIME_MESSAGE(TL("Checking overlapping edges"));
    //         myEdgeCont.checkOverlap(oc.getFloat("geometry.check-overlap"), oc.getFloat("geometry.check-overlap.vertical-threshold"));
    //         PROGRESS_TIME_MESSAGE(before);
    //     }
    //     if (geoConvHelper.getConvBoundary().getZRange() > 0 && oc.getFloat("geometry.max-grade") > 0) {
    //         before = PROGRESS_BEGIN_TIME_MESSAGE(TL("Checking edge grade"));
    //         // user input is in %
    //         myEdgeCont.checkGrade(oc.getFloat("geometry.max-grade") / 100);
    //         PROGRESS_TIME_MESSAGE(before);
    //     }
    //
    //     // find accesses for pt rail stops and add bidi-stops
    //     if (!myPTStopCont.getStops().empty()) {
    //         // re-adapt stop lanes after adding special lanes and cutting edge shapes at junction
    //         myPTStopCont.assignLanes(myEdgeCont);
    //         before = SysUtils::getCurrentMillis();
    //         int numBidiStops = 0;
    //         if (!oc.getBool("ptstop-output.no-bidi")) {
    //             numBidiStops = myPTStopCont.generateBidiStops(myEdgeCont);
    //         } else {
    //             numBidiStops = myPTStopCont.countBidiStops(myEdgeCont);
    //         }
    //         PROGRESS_BEGIN_MESSAGE(TL("Find accesses for pt rail stops"));
    //         double maxRadius = oc.getFloat("railway.access-distance");
    //         double accessFactor = oc.getFloat("railway.access-factor");
    //         int maxCount = oc.getInt("railway.max-accesses");
    //         myPTStopCont.findAccessEdgesForRailStops(myEdgeCont, maxRadius, maxCount, accessFactor);
    //         PROGRESS_TIME_MESSAGE(before);
    //         if (numBidiStops > 0) {
    //             myPTLineCont.fixBidiStops(myEdgeCont);
    //         }
    //     }
    //     myPTLineCont.removeInvalidEdges(myEdgeCont);
    //     // ensure that all turning lanes have sufficient permissions
    //     myPTLineCont.fixPermissions();
    //     if (oc.exists("ptline-clean-up") && oc.getBool("ptline-clean-up")) {
    //         before = PROGRESS_BEGIN_TIME_MESSAGE(TL("Cleaning up public transport stops that are not served by any line"));
    //         std::set<std::string> usedStops = myPTLineCont.getServedPTStops();
    //         myPTStopCont.postprocess(usedStops);
    //         PROGRESS_TIME_MESSAGE(before);
    //     } else {
    //         int numDeletedStops = myPTStopCont.cleanupDeleted(myEdgeCont);
    //         if (numDeletedStops > 0) {
    //             WRITE_WARNINGF(TL("Removed % pt stops because they could not be assigned to the network"), toString(numDeletedStops));
    //         }
    //     }
    //
    //     if (oc.exists("ignore-change-restrictions") && !oc.isDefault("ignore-change-restrictions")) {
    //         SVCPermissions ignoring = parseVehicleClasses(oc.getStringVector("ignore-change-restrictions"));
    //         myEdgeCont.updateAllChangeRestrictions(ignoring);
    //     }
    //
    //     NBRequest::reportWarnings();
    //     // report on very large networks
    //     if (MAX2(geoConvHelper.getConvBoundary().xmax(), geoConvHelper.getConvBoundary().ymax()) > 1000000 ||
    //             MIN2(geoConvHelper.getConvBoundary().xmin(), geoConvHelper.getConvBoundary().ymin()) < -1000000) {
    //         WRITE_WARNING(TL("Network contains very large coordinates and will probably flicker in the GUI. Check for outlying nodes and make sure the network is shifted to the coordinate origin"));
    //     }
    //
    //     // clean up OSM processing params
    //     if (oc.exists("osm-files") && oc.isSet("osm-files")) {
    //         for (auto item : myEdgeCont) {
    //             item.second->unsetParameter(NBTrafficLightDefinition::OSM_DIRECTION);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn compute(&mut self, oc: &mut OptionsCont, explicitTurnarounds: &BTreeSet<String>, mayAddOrRemove: bool) {
        // PROGRESS_BEGIN/PROGRESS_TIME_MESSAGE timing output is not ported; the WRITE_MESSAGEF lines are.
        let lefthand = oc.get_bool("lefthand").unwrap();
        if lefthand {
            self.mirror_x();
        }

        // MODIFYING THE SETS OF NODES AND EDGES
        // Removes edges that are connecting the same node
        let mut numRemovedEdges: i32 = 0;
        numRemovedEdges += self.myNodeCont.remove_self_loops(&mut self.myDistrictCont, &mut self.myEdgeCont, &mut self.myTLLCont);
        if mayAddOrRemove && oc.exists("remove-edges.isolated") && oc.get_bool("remove-edges.isolated").unwrap() {
            numRemovedEdges += self.myNodeCont.remove_isolated_roads(&mut self.myDistrictCont, &mut self.myEdgeCont);
        }
        if mayAddOrRemove && oc.exists("keep-edges.components") && oc.get_int("keep-edges.components").unwrap() > 0 {
            let hasStops = oc.exists("ptstop-output") && oc.is_set("ptstop-output", true).unwrap() && !self.myPTStopCont.get_stops().is_empty();
            numRemovedEdges += self.myNodeCont.remove_components(&mut self.myDistrictCont, &mut self.myEdgeCont, oc.get_int("keep-edges.components").unwrap(), hasStops);
        }
        if mayAddOrRemove && oc.exists("keep-edges.postload") && oc.get_bool("keep-edges.postload").unwrap() {
            // pre-process lines to set permissions
            if !self.myPTLineCont.get_lines().is_empty() {
                self.myPTLineCont.process(&mut self.myEdgeCont, &mut self.myPTStopCont, false);
            }
            if oc.is_set("keep-edges.explicit", true).unwrap() || oc.is_set("keep-edges.input-file", true).unwrap() {
                numRemovedEdges += self.myEdgeCont.remove_unwished_edges(&mut self.myDistrictCont);
            }
            let removed = self.myEdgeCont.remove_edges_by_speed(&mut self.myDistrictCont);
            if removed > 0 {
                numRemovedEdges += removed;
                write_messagef!(" Removed % edges because by minimum speed.", removed);
            }
            let removed2 = self.myEdgeCont.remove_edges_by_permissions(&mut self.myDistrictCont);
            if removed2 > 0 {
                numRemovedEdges += removed2;
                write_messagef!(" Removed % edges based on vClass.", removed2);
            }
        }
        if mayAddOrRemove && oc.get_float("keep-lanes.min-width").unwrap() > 0.0 {
            let removed = self.myEdgeCont.remove_lanes_by_width(&mut self.myDistrictCont, oc.get_float("keep-lanes.min-width").unwrap());
            if removed > 0 {
                numRemovedEdges += removed;
                write_messagef!(" Removed % edges because of lane width.", removed);
            }
        }
        if mayAddOrRemove && oc.exists("junctions.attach-removed") && oc.get_float("junctions.attach-removed").unwrap() >= 0.0 {
            let numSplit = self.myEdgeCont.attach_removed(&mut self.myNodeCont, &mut self.myDistrictCont, oc.get_float("junctions.attach-removed").unwrap());
            if numSplit > 0 {
                write_messagef!(" Split % edges to attach removed nodes", numSplit);
            }
        }
        // Processing pt stops and lines
        if !self.myPTStopCont.get_stops().is_empty() {
            if !(oc.exists("ptline-output") && oc.is_set("ptline-output", true).unwrap()) && !oc.get_bool("ptstop-output.no-bidi").unwrap() {
                self.myPTStopCont.localize_pt_stops(&mut self.myEdgeCont);
            }
            self.myPTStopCont.assign_edge_for_floating_stops(&mut self.myEdgeCont, 20.0);
            self.myPTStopCont.assign_lanes(&mut self.myEdgeCont);
        }
        if mayAddOrRemove && oc.exists("keep-edges.components") && oc.get_int("keep-edges.components").unwrap() > 0 {
            // post process rail components unless they have stops
            numRemovedEdges += self.myNodeCont.remove_rail_components(&mut self.myDistrictCont, &mut self.myEdgeCont, &mut self.myPTStopCont);
        }
        // removal is done, clean up roundabouts
        if numRemovedEdges > 0 {
            self.myEdgeCont.cleanup_roundabouts();
        }

        if !self.myPTLineCont.get_lines().is_empty() {
            self.myPTLineCont.process(&mut self.myEdgeCont, &mut self.myPTStopCont, false);
        }

        if !self.myPTStopCont.get_stops().is_empty() && !oc.get_bool("ptstop-output.no-bidi").unwrap() {
            self.myPTStopCont.align_id_signs();
        }
        // analyze and fix railway topology
        let mut numAddedBidi: i32 = 0;
        if oc.exists("railway.topology.all-bidi") && oc.get_bool("railway.topology.all-bidi").unwrap() {
            NBTurningDirectionsComputer::compute_turn_directions(&mut self.myNodeCont, false);
            numAddedBidi = NBRailwayTopologyAnalyzer::make_all_bidi(&mut self.myEdgeCont);
        } else if oc.exists("railway.topology.repair") && oc.get_bool("railway.topology.repair").unwrap() {
            // correct railway angles for angle-based connectivity heuristic
            self.myEdgeCont.check_geometries(0.0, false, oc.get_float("geometry.min-radius").unwrap(), false, oc.get_bool("geometry.min-radius.fix.railways").unwrap(), true);
            NBTurningDirectionsComputer::compute_turn_directions(&mut self.myNodeCont, false);
            numAddedBidi = NBRailwayTopologyAnalyzer::repair_topology(&mut self.myEdgeCont, &mut self.myPTStopCont, &mut self.myPTLineCont);
        }
        NBRailwaySignalGuesser::guess_rail_signals(&mut self.myNodeCont, &mut self.myEdgeCont, &mut self.myPTStopCont, &mut self.myDistrictCont);
        if numAddedBidi > 0 {
            // update routes
            self.myPTLineCont.process(&mut self.myEdgeCont, &mut self.myPTStopCont, true);
        }
        if oc.exists("railway.topology.direction-priority") && oc.get_bool("railway.topology.direction-priority").unwrap() {
            NBTurningDirectionsComputer::compute_turn_directions(&mut self.myNodeCont, false); // recompute after new edges were added
            NBRailwayTopologyAnalyzer::extend_direction_priority(&mut self.myEdgeCont, true);
        } else if oc.exists("railway.topology.extend-priority")
            && (oc.get_bool("railway.topology.extend-priority").unwrap() || oc.get_string_vector("railway.topology.ptline-priority").unwrap().len() > 0)
        {
            NBTurningDirectionsComputer::compute_turn_directions(&mut self.myNodeCont, false); // recompute after new edges were added
            if oc.get_string_vector("railway.topology.ptline-priority").unwrap().len() > 0 {
                let vClasses = parse_vehicle_classes_vec(oc.get_string_vector("railway.topology.ptline-priority").unwrap()).unwrap();
                NBRailwayTopologyAnalyzer::set_pt_line_priority(&mut self.myEdgeCont, &mut self.myPTLineCont, vClasses);
            }
            NBRailwayTopologyAnalyzer::extend_direction_priority(&mut self.myEdgeCont, false);
        }
        if oc.exists("railway.topology.output") && oc.is_set("railway.topology.output", true).unwrap() {
            NBTurningDirectionsComputer::compute_turn_directions(&mut self.myNodeCont, false); // recompute after new edges were added
            NBRailwayTopologyAnalyzer::analyze_topology(&mut self.myEdgeCont);
        }
        if oc.exists("railway.geometry.straighten") && oc.get_bool("railway.geometry.straighten").unwrap() {
            NBTurningDirectionsComputer::compute_turn_directions(&mut self.myNodeCont, false); // recompute after new edges were added
            NBRailwayGeometryHelper::straigthen_corrdidor(&mut self.myEdgeCont, oc.get_float("geometry.max-angle").unwrap());
        }

        if mayAddOrRemove && oc.exists("edges.join-tram-dist") && oc.get_float("edges.join-tram-dist").unwrap() >= 0.0 {
            // should come before joining junctions
            let numJoinedTramEdges = self.myEdgeCont.join_tram_edges(&mut self.myDistrictCont, &mut self.myPTStopCont, &mut self.myPTLineCont, oc.get_float("edges.join-tram-dist").unwrap());
            if numJoinedTramEdges > 0 {
                write_messagef!(" Joined % tram edges into roads.", numJoinedTramEdges);
            }
        }
        if oc.get_bool("junctions.join").unwrap()
            || (oc.exists("ramps.guess") && oc.get_bool("ramps.guess").unwrap())
            || oc.get_bool("tls.guess.joining").unwrap()
            || (oc.exists("tls.guess-signals") && oc.get_bool("tls.guess-signals").unwrap())
        {
            // preliminary geometry computations to determine the length of edges
            // This depends on turning directions and sorting of edge list
            // in case junctions are joined geometry computations have to be repeated
            // preliminary roundabout computations to avoid damaging roundabouts via junctions.join or ramps.guess
            NBTurningDirectionsComputer::compute_turn_directions(&mut self.myNodeCont, false);
            NBNodesEdgesSorter::sort_nodes_edges(&mut self.myNodeCont, false);
            self.myEdgeCont.compute_lane_shapes();
            self.myNodeCont.compute_node_shapes(-1.0);
            self.myEdgeCont.compute_edge_shapes(-1.0);
            if oc.get_bool("roundabouts.guess").unwrap() {
                self.myEdgeCont.guess_roundabouts();
            }
            let roundabouts = self.myEdgeCont.get_roundabouts();
            for round in roundabouts.iter() {
                let mut nodeIDs: Vec<String> = Vec::new();
                for edge in round.iter() {
                    nodeIDs.push(edge.get_to_node().get_id().to_string());
                }
                self.myNodeCont.add_join_exclusion(&nodeIDs);
            }
            NBNodeTypeComputer::validate_rail_crossings(&mut self.myNodeCont, &mut self.myTLLCont);
        } else if (self.myEdgeCont.has_guessed_roundabouts() || oc.get_bool("crossings.guess").unwrap()) && oc.get_bool("roundabouts.guess").unwrap() {
            self.myEdgeCont.guess_roundabouts();
            self.myEdgeCont.mark_roundabouts();
        }
        // join junctions (may create new "geometry"-nodes so it needs to come before removing these
        if mayAddOrRemove && oc.exists("junctions.join-exclude") && oc.is_set("junctions.join-exclude", true).unwrap() {
            self.myNodeCont.add_join_exclusion(oc.get_string_vector("junctions.join-exclude").unwrap());
        }
        let mut numJoined = self.myNodeCont.join_loaded_clusters(&mut self.myDistrictCont, &mut self.myEdgeCont, &mut self.myTLLCont);
        if mayAddOrRemove && oc.get_bool("junctions.join").unwrap() {
            numJoined += self.myNodeCont.join_junctions(oc.get_float("junctions.join-dist").unwrap(), &mut self.myDistrictCont, &mut self.myEdgeCont, &mut self.myTLLCont, &mut self.myPTStopCont);
            jdump("join", &self.myEdgeCont);
        }
        if numJoined > 0 {
            write_messagef!(" Joined % junction cluster(s).", numJoined);
        }
        if mayAddOrRemove && oc.get_float("junctions.join-same").unwrap() >= 0.0 {
            let numJoined2 = self.myNodeCont.join_same_junctions(&mut self.myDistrictCont, &mut self.myEdgeCont, &mut self.myTLLCont, oc.get_float("junctions.join-same").unwrap());
            if numJoined2 > 0 {
                write_messagef!(" Joined % junctions.", numJoined2);
            }
        }
        //
        if mayAddOrRemove && oc.exists("join-lanes") && oc.get_bool("join-lanes").unwrap() {
            let num = self.myEdgeCont.join_lanes(SVC_IGNORING) + self.myEdgeCont.join_lanes(SVC_PEDESTRIAN);
            write_messagef!("   Joined lanes on % edges.", num);
        }
        //
        if mayAddOrRemove {
            let removeGeometryNodes = oc.exists("geometry.remove") && oc.get_bool("geometry.remove").unwrap();
            // removeUnwishedNodes needs turnDirections. @todo: try to call this less often
            NBTurningDirectionsComputer::compute_turn_directions(&mut self.myNodeCont, false);
            let numRemoved = self.myNodeCont.remove_unwished_nodes(&mut self.myDistrictCont, &mut self.myEdgeCont, &mut self.myTLLCont, &mut self.myPTStopCont, &mut self.myPTLineCont, &mut self.myParkingCont, removeGeometryNodes);
            write_messagef!("   % nodes removed.", numRemoved);
        }

        // MOVE TO ORIGIN
        // compute new boundary after network modifications have taken place
        let mut boundary = Boundary::new();
        for node in self.myNodeCont.myNodes.values() {
            boundary.add_pos(node.get_position());
        }
        for edge in self.myEdgeCont.myEdges.values() {
            boundary.add_boundary(&edge.get_geometry().get_box_boundary());
        }
        GeoConvHelper::with_processing(|g| g.set_conv_boundary(boundary));

        if !oc.get_bool("offset.disable-normalization").unwrap() && oc.is_default("offset.x") && oc.is_default("offset.y") {
            GeoConvHelper::with_processing(|g| self.move_to_origin(g, lefthand));
        }
        self.round_inputs();
        GeoConvHelper::compute_final(lefthand); // information needed for location element fixed at this point

        // reset shapes and angles for stable re-computation
        if self.myNodeCont.reset_node_shapes() {
            self.myEdgeCont.compute_angles();
        }

        if oc.exists("geometry.min-dist") && !oc.is_default("geometry.min-dist") {
            self.myEdgeCont.reduce_geometries(oc.get_float("geometry.min-dist").unwrap());
        }
        // @note: removing geometry can create similar edges so joinSimilarEdges  must come afterwards
        // @note: likewise splitting can destroy similarities so joinSimilarEdges must come before
        if mayAddOrRemove && oc.get_bool("edges.join").unwrap() {
            let removeDuplicates = oc.get_float("junctions.join-same").unwrap() >= 0.0;
            self.myNodeCont.join_similar_edges(&mut self.myDistrictCont, &mut self.myEdgeCont, &mut self.myTLLCont, removeDuplicates);
            // now we may have new chances to remove geometry if wished
            if oc.exists("geometry.remove") && oc.get_bool("geometry.remove").unwrap() {
                self.myNodeCont.remove_unwished_nodes(&mut self.myDistrictCont, &mut self.myEdgeCont, &mut self.myTLLCont, &mut self.myPTStopCont, &mut self.myPTLineCont, &mut self.myParkingCont, true);
            }
        }
        if oc.get_bool("opposites.guess").unwrap() {
            self.myEdgeCont.guess_opposites();
        }
        //
        if mayAddOrRemove && oc.exists("geometry.split") && oc.get_bool("geometry.split").unwrap() {
            self.myEdgeCont.split_geometry(&mut self.myDistrictCont, &mut self.myNodeCont);
            // newly split junctions might also be joinable
            if oc.get_float("junctions.join-same").unwrap() >= 0.0 {
                let numJoined3 = self.myNodeCont.join_same_junctions(&mut self.myDistrictCont, &mut self.myEdgeCont, &mut self.myTLLCont, oc.get_float("junctions.join-same").unwrap());
                if numJoined3 > 0 {
                    write_messagef!(" Joined % junctions after splitting geometry.", numJoined3);
                }
            }
        }
        // turning direction
        NBTurningDirectionsComputer::compute_turn_directions(&mut self.myNodeCont, true);
        // correct edge geometries to avoid overlap
        if oc.exists("geometry.avoid-overlap") && oc.get_bool("geometry.avoid-overlap").unwrap() {
            self.myNodeCont.avoid_overlap();
        }

        // GUESS TLS POSITIONS
        if oc.is_set("tls.set", true).unwrap() {
            let tlControlledNodes = oc.get_string_vector("tls.set").unwrap().clone();
            let type_ = TrafficLightTypes.get(&oc.get_string("tls.default-type").unwrap()).unwrap();
            for i in tlControlledNodes.iter() {
                let node = self.myNodeCont.retrieve(i);
                if node.is_null() {
                    utils::common::msg_handler::write_warning(&format!("Building a tl-logic for junction '{}' is not possible.\n The junction '{}' is not known.", i, i));
                } else {
                    self.myNodeCont.set_as_tl_controlled(node, &mut self.myTLLCont, type_, String::new());
                }
            }
        }
        self.myNodeCont.guess_t_ls(oc, &mut self.myTLLCont);

        // guess ramps (after guessing tls because ramps should not be build at traffic lights)
        let modifyRamps = mayAddOrRemove
            && ((oc.exists("ramps.guess") && oc.get_bool("ramps.guess").unwrap()) || (oc.exists("ramps.set") && oc.is_set("ramps.set", true).unwrap()));
        if modifyRamps || (oc.exists("ramps.guess-acceleration-lanes") && oc.get_bool("ramps.guess-acceleration-lanes").unwrap()) {
            NBNodesEdgesSorter::sort_nodes_edges(&mut self.myNodeCont, false);
            let mut rc = NBRampsComputer::new();
            rc.compute_ramps(self, oc, mayAddOrRemove);
        }
        // guess bike lanes
        if mayAddOrRemove && (oc.get_bool("bikelanes.guess").unwrap() || oc.get_bool("bikelanes.guess.from-permissions").unwrap()) {
            let bikelanes = self.myEdgeCont.guess_special_lanes(
                SVC_BICYCLE,
                oc.get_float("default.bikelane-width").unwrap(),
                oc.get_float("bikelanes.guess.min-speed").unwrap(),
                oc.get_float("bikelanes.guess.max-speed").unwrap(),
                oc.get_bool("bikelanes.guess.from-permissions").unwrap(),
                "bikelanes.guess.exclude",
                &mut self.myTLLCont,
            );
            write_messagef!("Guessed % bike lanes.", bikelanes);
        }

        // guess sidewalks
        if mayAddOrRemove && (oc.get_bool("sidewalks.guess").unwrap() || oc.get_bool("sidewalks.guess.from-permissions").unwrap()) {
            let sidewalks = self.myEdgeCont.guess_special_lanes(
                SVC_PEDESTRIAN,
                oc.get_float("default.sidewalk-width").unwrap(),
                oc.get_float("sidewalks.guess.min-speed").unwrap(),
                oc.get_float("sidewalks.guess.max-speed").unwrap(),
                oc.get_bool("sidewalks.guess.from-permissions").unwrap(),
                "sidewalks.guess.exclude",
                &mut self.myTLLCont,
            );
            write_messagef!("Guessed % sidewalks.", sidewalks);
        }
        // check whether any not previously setable connections may be set now
        self.myEdgeCont.recheck_post_process_connections();

        // remap ids if wished
        if mayAddOrRemove {
            let numericalIDs = oc.get_bool("numerical-ids").unwrap();
            let reservedIDs = oc.is_set("reserved-ids", true).unwrap();
            let keptIDs = oc.is_set("kept-ids", true).unwrap();
            let numChangedEdges = self.myEdgeCont.remap_i_ds(numericalIDs, reservedIDs, keptIDs, &oc.get_string("prefix.edge").unwrap(), &mut self.myPTStopCont);
            let numChangedNodes = self.myNodeCont.remap_i_ds(numericalIDs, reservedIDs, keptIDs, &oc.get_string("prefix.junction").unwrap(), &mut self.myTLLCont);
            if numChangedEdges + numChangedNodes > 0 {
                write_messagef!("Remapped % edge IDs and % node IDs.", numChangedEdges, numChangedNodes);
            }
        }

        //
        if oc.exists("geometry.max-angle") {
            self.myEdgeCont.check_geometries(
                utils::geom::geom_helper::deg2rad(oc.get_float("geometry.max-angle").unwrap()),
                oc.get_bool("geometry.max-angle.fix").unwrap(),
                oc.get_float("geometry.min-radius").unwrap(),
                oc.get_bool("geometry.min-radius.fix").unwrap(),
                oc.get_bool("geometry.min-radius.fix.railways").unwrap(),
                false,
            );
        }

        // GEOMETRY COMPUTATION
        //
        NBNodesEdgesSorter::sort_nodes_edges(&mut self.myNodeCont, false);
        self.myEdgeCont.compute_lane_shapes();
        //
        if oc.exists("geometry.junction-mismatch-threshold") {
            self.myNodeCont.compute_node_shapes(oc.get_float("geometry.junction-mismatch-threshold").unwrap());
        } else {
            self.myNodeCont.compute_node_shapes(-1.0);
        }
        //
        let maxGrade = if oc.get_bool("geometry.max-grade.fix").unwrap() { oc.get_float("geometry.max-grade").unwrap() / 100.0 } else { -1.0 };
        self.myEdgeCont.compute_edge_shapes(maxGrade);
        // resort edges based on the node and edge shapes
        NBNodesEdgesSorter::sort_nodes_edges(&mut self.myNodeCont, true);
        NBTurningDirectionsComputer::compute_turn_directions(&mut self.myNodeCont, false);

        // APPLY SPEED MODIFICATIONS
        if oc.exists("speed.offset") {
            let speedOffset = oc.get_float("speed.offset").unwrap();
            let speedFactor = oc.get_float("speed.factor").unwrap();
            let speedMin = oc.get_float("speed.minimum").unwrap();
            if speedOffset != 0.0 || speedFactor != 1.0 || speedMin > 0.0 {
                for edge in self.myEdgeCont.myEdges.values() {
                    let e = edge.m();
                    for i in 0..e.get_num_lanes() {
                        let v = e.get_lane_speed(i) * speedFactor + speedOffset;
                        // MAX2(a, b) = a > b ? a : b
                        e.set_speed(i, if v > speedMin { v } else { speedMin });
                    }
                }
            }
        }

        // CONNECTIONS COMPUTATION
        //
        NBNode::init_rail_signal_classes(&self.myNodeCont);
        NBNodeTypeComputer::compute_node_types(&mut self.myNodeCont, &mut self.myTLLCont);
        //
        self.myNetworkHaveCrossings = oc.get_bool("walkingareas").unwrap();
        if mayAddOrRemove && oc.get_bool("crossings.guess").unwrap() {
            self.myNetworkHaveCrossings = true;
            let mut crossings: i32 = 0;
            for node in self.myNodeCont.myNodes.values() {
                crossings += node.m().guess_crossings();
            }
            write_messagef!("Guessed % pedestrian crossings.", crossings);
        }
        if !self.myNetworkHaveCrossings {
            let mut haveValidCrossings = false;
            // recheck whether we had crossings in the input
            for node in self.myNodeCont.myNodes.values() {
                if node.get_crossings().len() > 0 {
                    self.myNetworkHaveCrossings = true;
                    haveValidCrossings = true;
                    break;
                } else if node.get_crossings_including_invalid().len() > 0 {
                    self.myNetworkHaveCrossings = true;
                }
            }
            if self.myNetworkHaveCrossings && !haveValidCrossings {
                // initial crossings removed or invalidated, keep walkingareas
                oc.reset_writable();
                oc.set("walkingareas", "true", false).unwrap();
            }
        }

        if !mayAddOrRemove && self.myNetworkHaveCrossings {
            // crossings added via netedit
            oc.reset_writable();
            oc.set("no-internal-links", "false", false).unwrap();
        }

        //
        NBEdgePriorityComputer::compute_edge_priorities(&mut self.myNodeCont);
        //
        self.myEdgeCont.compute_edge2_edges(oc.get_bool("no-left-connections").unwrap());
        jdump("e2e", &self.myEdgeCont);
        //
        if oc.get_bool("roundabouts.guess").unwrap() {
            let numGuessed = self.myEdgeCont.guess_roundabouts();
            if numGuessed > 0 {
                write_messagef!(" Guessed % roundabout(s).", numGuessed);
            }
        }
        self.myEdgeCont.mark_roundabouts();
        //
        self.myEdgeCont.compute_lanes2_edges();
        //
        self.myNodeCont.compute_lanes2_lanes();
        jdump("l2l", &self.myEdgeCont);
        self.myEdgeCont.sort_outgoing_lanes_connections();
        //
        if oc.get_bool("fringe.guess").unwrap() {
            let numGuessed = self.myNodeCont.guess_fringe();
            if numGuessed > 0 {
                write_messagef!(" Guessed % fringe nodes.", numGuessed);
            }
        }
        //
        if !oc.get_bool("no-turnarounds").unwrap() {
            self.myEdgeCont.append_turnarounds(
                oc.get_bool("no-turnarounds.tls").unwrap(),
                oc.get_bool("no-turnarounds.fringe").unwrap(),
                oc.get_bool("no-turnarounds.except-deadend").unwrap(),
                oc.get_bool("no-turnarounds.except-turnlane").unwrap(),
                oc.get_bool("no-turnarounds.geometry").unwrap(),
            );
        } else {
            self.myEdgeCont.append_turnarounds_2(explicitTurnarounds, oc.get_bool("no-turnarounds.tls").unwrap());
        }
        if oc.exists("railway.topology.repair.stop-turn") && oc.get_bool("railway.topology.repair.stop-turn").unwrap() && self.myPTStopCont.get_stops().len() > 0 {
            // allow direction reversal at all bidi-edges with stops
            self.myEdgeCont.append_railway_turnarounds(&self.myPTStopCont);
        }
        //
        self.myEdgeCont.recheck_lanes();
        jdump("recheck", &self.myEdgeCont);

        if self.myNetworkHaveCrossings && !oc.get_bool("no-internal-links").unwrap() {
            for node in self.myNodeCont.myNodes.values() {
                node.m().build_crossings_and_walking_areas();
            }
        } else {
            for node in self.myNodeCont.myNodes.values() {
                // needed by netedit if the last crossings was deleted from the network
                // and walkingareas have been invalidated since the last call to compute()
                node.m().discard_walkingareas();
            }
            if oc.get_bool("no-internal-links").unwrap() {
                for node in self.myNodeCont.myNodes.values() {
                    node.m().discard_all_crossings(false);
                }
            }
        }
        // join traffic lights (after building connections)
        if oc.get_bool("tls.join").unwrap() {
            self.myNodeCont.join_tls(&mut self.myTLLCont, oc.get_float("tls.join-dist").unwrap());
        }

        // COMPUTING RIGHT-OF-WAY AND TRAFFIC LIGHT PROGRAMS
        //
        self.myTLLCont.set_tl_controlling_information(&self.myEdgeCont, &self.myNodeCont);
        if oc.exists("opendrive-files") && oc.is_set("opendrive-files", true).unwrap() {
            self.myTLLCont.set_open_drive_signal_parameters();
        }
        //
        self.myNodeCont.compute_logics(&self.myEdgeCont);

        //
        let numbers = self.myTLLCont.compute_logics(oc);
        let mut progCount = String::new();
        if numbers.0 != numbers.1 {
            progCount = format!("({} programs) ", numbers.1);
        }
        write_messagef!(" % traffic light(s) %computed.", numbers.0, progCount);
        if oc.exists("opendrive-files") && oc.is_set("opendrive-files", true).unwrap() && oc.get_bool("opendrive.signal-groups").unwrap() {
            self.myTLLCont.apply_open_drive_controllers(oc);
        }

        for edge in self.myEdgeCont.myEdges.values() {
            edge.m().sort_outgoing_connections_by_index();
        }
        // FINISHING INNER EDGES
        // determinism: C++ std::set of pointers; only the order of recomputation depends on it
        let mut largeNodeTLS: BTreeSet<Ptr<NBTrafficLightDefinition>> = BTreeSet::new();
        if !oc.get_bool("no-internal-links").unwrap() {
            // walking areas shall only be built if crossings are wished as well
            for item in self.myNodeCont.myNodes.values() {
                if item.m().build_inner_edges() > NBTrafficLightDefinition_MIN_YELLOW_SECONDS as f64 {
                    let tlDefs = item.get_controlling_tls();
                    largeNodeTLS.extend(tlDefs.iter().cloned());
                }
            }
        }
        // PATCH NODE SHAPES
        if oc.get_float("junctions.scurve-stretch").unwrap() > 0.0 {
            // @note: nodes have collected correction hints in buildInnerEdges()
            self.myEdgeCont.compute_lane_shapes();
            self.myNodeCont.compute_node_shapes(-1.0);
            let maxGrade = if oc.get_bool("geometry.max-grade.fix").unwrap() { oc.get_float("geometry.max-grade").unwrap() / 100.0 } else { -1.0 };
            self.myEdgeCont.compute_edge_shapes(maxGrade);
            for item in self.myNodeCont.myNodes.values() {
                item.m().build_inner_edges();
            }
        }
        if self.myEdgeCont.get_num_edge_splits() > 0 && !oc.get_bool("no-internal-links").unwrap() {
            // edges with custom lengths were split, this has to take into account
            // internal edge lengths (after geometry computation)
            self.myEdgeCont.fix_split_custom_length();
        }
        // recheck phases for large junctions
        for def in largeNodeTLS.iter() {
            self.myTLLCont.compute_single_logic(oc, *def);
        }
        // compute lane-to-lane node logics (require traffic lights and inner edges to be done)
        self.myNodeCont.compute_logics2(&self.myEdgeCont, oc);

        // remove guessed traffic lights at junctions without conflicts (requires computeLogics2)
        self.myNodeCont.recheck_guessed_tls(&mut self.myTLLCont);

        // compute keepClear status (requires computeLogics2)
        self.myNodeCont.compute_keep_clear();

        //
        if oc.is_set("street-sign-output", true).unwrap() {
            self.myEdgeCont.generate_street_signs();
        }

        if lefthand != oc.get_bool("flip-y-axis").unwrap() {
            self.mirror_x();
        }

        if oc.exists("geometry.check-overlap") && oc.get_float("geometry.check-overlap").unwrap() > 0.0 {
            self.myEdgeCont.check_overlap(oc.get_float("geometry.check-overlap").unwrap(), oc.get_float("geometry.check-overlap.vertical-threshold").unwrap());
        }
        if GeoConvHelper::with_processing(|g| g.get_conv_boundary().get_z_range()) > 0.0 && oc.get_float("geometry.max-grade").unwrap() > 0.0 {
            // user input is in %
            self.myEdgeCont.check_grade(oc.get_float("geometry.max-grade").unwrap() / 100.0);
        }

        // find accesses for pt rail stops and add bidi-stops
        if !self.myPTStopCont.get_stops().is_empty() {
            // re-adapt stop lanes after adding special lanes and cutting edge shapes at junction
            self.myPTStopCont.assign_lanes(&mut self.myEdgeCont);
            let numBidiStops = if !oc.get_bool("ptstop-output.no-bidi").unwrap() {
                self.myPTStopCont.generate_bidi_stops(&mut self.myEdgeCont)
            } else {
                self.myPTStopCont.count_bidi_stops(&mut self.myEdgeCont)
            };
            let maxRadius = oc.get_float("railway.access-distance").unwrap();
            let accessFactor = oc.get_float("railway.access-factor").unwrap();
            let maxCount = oc.get_int("railway.max-accesses").unwrap();
            self.myPTStopCont.find_access_edges_for_rail_stops(&mut self.myEdgeCont, maxRadius, maxCount, accessFactor);
            if numBidiStops > 0 {
                self.myPTLineCont.fix_bidi_stops(&self.myEdgeCont);
            }
        }
        self.myPTLineCont.remove_invalid_edges(&self.myEdgeCont);
        // ensure that all turning lanes have sufficient permissions
        self.myPTLineCont.fix_permissions();
        if oc.exists("ptline-clean-up") && oc.get_bool("ptline-clean-up").unwrap() {
            let mut usedStops = self.myPTLineCont.get_served_pt_stops();
            self.myPTStopCont.postprocess(&mut usedStops);
        } else {
            let numDeletedStops = self.myPTStopCont.cleanup_deleted(&mut self.myEdgeCont);
            if numDeletedStops > 0 {
                write_warningf!("Removed % pt stops because they could not be assigned to the network", numDeletedStops);
            }
        }

        if oc.exists("ignore-change-restrictions") && !oc.is_default("ignore-change-restrictions") {
            let ignoring = parse_vehicle_classes_vec(oc.get_string_vector("ignore-change-restrictions").unwrap()).unwrap();
            self.myEdgeCont.update_all_change_restrictions(ignoring);
        }

        NBRequest::report_warnings();
        // report on very large networks
        let conv = GeoConvHelper::with_processing(|g| *g.get_conv_boundary());
        let maxCoord = if conv.xmax() > conv.ymax() { conv.xmax() } else { conv.ymax() }; // MAX2
        let minCoord = if conv.xmin() < conv.ymin() { conv.xmin() } else { conv.ymin() }; // MIN2
        if maxCoord > 1000000.0 || minCoord < -1000000.0 {
            utils::common::msg_handler::write_warning("Network contains very large coordinates and will probably flicker in the GUI. Check for outlying nodes and make sure the network is shifted to the coordinate origin");
        }

        // clean up OSM processing params
        if oc.exists("osm-files") && oc.is_set("osm-files", true).unwrap() {
            for item in self.myEdgeCont.myEdges.values() {
                item.m().base2.unset_parameter(NBTrafficLightDefinition_OSM_DIRECTION);
            }
        }
    }

    /// C++: `NBEdgeCont & NBNetBuilder::getEdgeCont()`
    /// Returns a reference to edge container
    // inline NBEdgeCont& getEdgeCont() {
    //         return myEdgeCont;
    //     }
    #[allow(unused_variables)]
    pub fn get_edge_cont(&mut self) -> &mut NBEdgeCont {
        &mut self.myEdgeCont
    }

    /// C++: `NBNodeCont & NBNetBuilder::getNodeCont()`
    /// Returns a reference to the node container
    // inline NBNodeCont& getNodeCont() {
    //         return myNodeCont;
    //     }
    #[allow(unused_variables)]
    pub fn get_node_cont(&mut self) -> &mut NBNodeCont {
        &mut self.myNodeCont
    }

    /// C++: `NBTypeCont & NBNetBuilder::getTypeCont()`
    /// Returns a reference to the type container
    // inline NBTypeCont& getTypeCont() {
    //         return myTypeCont;
    //     }
    #[allow(unused_variables)]
    pub fn get_type_cont(&mut self) -> &mut NBTypeCont {
        &mut self.myTypeCont
    }

    /// C++: `NBTrafficLightLogicCont & NBNetBuilder::getTLLogicCont()`
    /// Returns a reference to the traffic light logics container
    // inline NBTrafficLightLogicCont& getTLLogicCont() {
    //         return myTLLCont;
    //     }
    #[allow(unused_variables)]
    pub fn get_tl_logic_cont(&mut self) -> &mut NBTrafficLightLogicCont {
        &mut self.myTLLCont
    }

    /// C++: `NBDistrictCont & NBNetBuilder::getDistrictCont()`
    /// Returns a reference the districts container
    // inline NBDistrictCont& getDistrictCont() {
    //         return myDistrictCont;
    //     }
    #[allow(unused_variables)]
    pub fn get_district_cont(&mut self) -> &mut NBDistrictCont {
        &mut self.myDistrictCont
    }

    /// C++: `NBPTStopCont & NBNetBuilder::getPTStopCont()`
    /// Returns a reference to the pt stop container
    // inline NBPTStopCont& getPTStopCont() {
    //         return myPTStopCont;
    //     }
    #[allow(unused_variables)]
    pub fn get_pt_stop_cont(&mut self) -> &mut NBPTStopCont {
        &mut self.myPTStopCont
    }

    /// C++: `NBPTLineCont & NBNetBuilder::getPTLineCont()`
    /// Returns a reference to the pt line container
    // inline NBPTLineCont& getPTLineCont() {
    //         return myPTLineCont;
    //     }
    #[allow(unused_variables)]
    pub fn get_pt_line_cont(&mut self) -> &mut NBPTLineCont {
        &mut self.myPTLineCont
    }

    /// C++: `NBParkingCont & NBNetBuilder::getParkingCont()`
    // inline NBParkingCont& getParkingCont() {
    //         return myParkingCont;
    //     }
    #[allow(unused_variables)]
    pub fn get_parking_cont(&mut self) -> &mut NBParkingCont {
        &mut self.myParkingCont
    }

    /// C++: `ShapeContainer & NBNetBuilder::getShapeCont()`
    // inline ShapeContainer& getShapeCont() {
    //         return myShapeCont;
    //     }
    #[allow(unused_variables)]
    pub fn get_shape_cont(&mut self) -> &mut ShapeContainer {
        &mut self.myShapeCont
    }

    /// C++: `bool NBNetBuilder::haveNetworkCrossings()`
    /// notify about style of loaded network (Without Crossings)
    // inline bool haveNetworkCrossings() {
    //         return myNetworkHaveCrossings;
    //     }
    #[allow(unused_variables)]
    pub fn have_network_crossings(&mut self) -> bool {
        self.myNetworkHaveCrossings
    }

    /// C++: `void NBNetBuilder::setHaveNetworkCrossings(bool)`
    /// enable crossing in networks
    // inline void setHaveNetworkCrossings(bool value) {
    //         myNetworkHaveCrossings = value;
    //     }
    #[allow(unused_variables)]
    pub fn set_have_network_crossings(&mut self, value: bool) {
        self.myNetworkHaveCrossings = value;
    }

    /// C++: `bool NBNetBuilder::transformCoordinate(Position &, bool, GeoConvHelper *)` [static]
    /// transforms loaded coordinates handles projections, offsets (using GeoConvHelper) and import of height data (using NBHeightMapper)
    // bool
    // NBNetBuilder::transformCoordinate(Position& from, bool includeInBoundary, GeoConvHelper* from_srs) {
    //     Position orig(from);
    //     bool ok = true;
    //     if (GeoConvHelper::getNumLoaded() > 1
    //             && GeoConvHelper::getLoaded().usingGeoProjection()
    //             && from_srs != nullptr
    //             && from_srs->usingGeoProjection()
    //             && *from_srs != GeoConvHelper::getLoaded()) {
    //         from_srs->cartesian2geo(from);
    //         ok &= GeoConvHelper::getLoaded().x2cartesian(from, false);
    //     }
    //     if (from_srs == nullptr || !GeoConvHelper::getProcessing().usingGeoProjection()) {
    //         // if getProcessing is not a geo-projection, assume it a cartesian transformation (i.e. shift)
    //         ok &= GeoConvHelper::getProcessing().x2cartesian(from, includeInBoundary);
    //
    //         if (from_srs == nullptr && GeoConvHelper::getProcessing().usingGeoProjection()
    //                 && GeoConvHelper::getNumLoaded() > 0
    //                 && GeoConvHelper::getLoaded().usingGeoProjection()) {
    //             // apply geo patch to loaded geo-network (offset must match)
    //             from = from + GeoConvHelper::getLoaded().getOffset();
    //         }
    //     }
    //     if (ok) {
    //         const NBHeightMapper& hm = NBHeightMapper::get();
    //         if (hm.ready()) {
    //             if (from_srs != nullptr && from_srs->usingGeoProjection()) {
    //                 from_srs->cartesian2geo(orig);
    //             }
    //             from.setz(hm.getZ(orig) + GeoConvHelper::getProcessing().getOffset().z());
    //         }
    //     }
    //     const double eps = 1e-6;
    //     from.set(std::round(from.x() / eps) * eps, std::round(from.y() / eps) * eps, std::round(from.z() / eps) * eps);
    //     return ok;
    // }
    #[allow(unused_variables)]
    pub fn transform_coordinate(from: &mut Position, includeInBoundary: bool, from_srs: Ptr<GeoConvHelper>) -> bool {
        // Position orig(from);
        let orig = *from;
        let mut ok = true;
        if GeoConvHelper::get_num_loaded() > 1
            && GeoConvHelper::with_loaded(|l| l.using_geo_projection())
            && !from_srs.is_null()
            && from_srs.using_geo_projection()
            && GeoConvHelper::with_loaded(|l| *l != *from_srs)
        {
            from_srs.cartesian2geo(from);
            ok &= GeoConvHelper::with_loaded(|l| l.x2cartesian(from, false));
        }
        if from_srs.is_null() || !GeoConvHelper::with_processing(|p| p.using_geo_projection()) {
            // if getProcessing is not a geo-projection, assume it a cartesian transformation (i.e. shift)
            ok &= GeoConvHelper::with_processing(|p| p.x2cartesian(from, includeInBoundary));

            if from_srs.is_null() && GeoConvHelper::with_processing(|p| p.using_geo_projection()) && GeoConvHelper::get_num_loaded() > 0 && GeoConvHelper::with_loaded(|l| l.using_geo_projection()) {
                // apply geo patch to loaded geo-network (offset must match)
                let off = GeoConvHelper::with_loaded(|l| l.get_offset());
                *from = *from + off;
            }
        }
        if ok {
            let hm = NBHeightMapper::get();
            if hm.ready() {
                let mut o = orig;
                if !from_srs.is_null() && from_srs.using_geo_projection() {
                    from_srs.cartesian2geo(&mut o);
                }
                let z = hm.get_z(&o) + GeoConvHelper::with_processing(|p| p.get_offset().z());
                from.setz(z);
            }
        }
        let eps = 1e-6;
        let (nx, ny, nz) = ((from.x() / eps).round() * eps, (from.y() / eps).round() * eps, (from.z() / eps).round() * eps);
        from.set(nx, ny);
        from.setz(nz);
        ok
    }

    /// C++: `bool NBNetBuilder::transformCoordinates(PositionVector &, bool, GeoConvHelper *)` [static]
    // bool
    // NBNetBuilder::transformCoordinates(PositionVector& from, bool includeInBoundary, GeoConvHelper* from_srs) {
    //     const double maxLength = OptionsCont::getOptions().getFloat("geometry.max-segment-length");
    //     if (maxLength > 0 && from.size() > 1) {
    //         // transformation to cartesian coordinates must happen before we can check segment length
    //         PositionVector copy = from;
    //         for (int i = 0; i < (int) from.size(); i++) {
    //             transformCoordinate(copy[i], false);
    //         }
    //         addGeometrySegments(from, copy, maxLength);
    //     }
    //     bool ok = true;
    //     for (int i = 0; i < (int) from.size(); i++) {
    //         ok = ok && transformCoordinate(from[i], includeInBoundary, from_srs);
    //     }
    //     return ok;
    // }
    #[allow(unused_variables)]
    pub fn transform_coordinates(from: &mut PositionVector, includeInBoundary: bool, from_srs: Ptr<GeoConvHelper>) -> bool {
        let maxLength = utils::options::options_cont::get_float("geometry.max-segment-length");
        if maxLength > 0.0 && from.len() > 1 {
            // transformation to cartesian coordinates must happen before we can check segment length
            let mut copy = from.clone();
            for i in 0..copy.len() {
                Self::transform_coordinate(&mut copy[i], false, Ptr::from_raw(std::ptr::null_mut()));
            }
            Self::add_geometry_segments(from, &copy, maxLength);
        }
        let mut ok = true;
        for i in 0..from.len() {
            ok = ok && Self::transform_coordinate(&mut from[i], includeInBoundary, from_srs);
        }
        ok
    }

    /// C++: `int NBNetBuilder::addGeometrySegments(PositionVector &, const PositionVector &, const double)` [static]
    /// insertion geometry points to ensure maximum segment length between points
    // int
    // NBNetBuilder::addGeometrySegments(PositionVector& from, const PositionVector& cartesian, const double maxLength) {
    //     // check lengths and insert new points where needed (in the original
    //     // coordinate system)
    //     int inserted = 0;
    //     for (int i = 0; i < (int)cartesian.size() - 1; i++) {
    //         Position start = from[i + inserted];
    //         Position end = from[i + inserted + 1];
    //         double length = cartesian[i].distanceTo(cartesian[i + 1]);
    //         const Position step = (end - start) * (maxLength / length);
    //         int steps = 0;
    //         while (length > maxLength) {
    //             length -= maxLength;
    //             steps++;
    //             from.insert(from.begin() + i + inserted + 1, start + (step * steps));
    //             inserted++;
    //         }
    //     }
    //     return inserted;
    // }
    #[allow(unused_variables)]
    pub fn add_geometry_segments(from: &mut PositionVector, cartesian: &PositionVector, maxLength: f64) -> i32 {
        // check lengths and insert new points where needed (in the original
        // coordinate system)
        let mut inserted: i32 = 0;
        for i in 0..(cartesian.len() as i32 - 1) {
            let start = from[(i + inserted) as usize];
            let end = from[(i + inserted + 1) as usize];
            let mut length = cartesian[i as usize].distance_to(&cartesian[(i + 1) as usize]);
            let step = (end - start) * (maxLength / length);
            let mut steps: i32 = 0;
            while length > maxLength {
                length -= maxLength;
                steps += 1;
                from.insert((i + inserted + 1) as usize, start + step * (steps as f64));
                inserted += 1;
            }
        }
        inserted
    }

    /// C++: `bool NBNetBuilder::runningNetedit()` [static]
    /// whether netbuilding takes place in the context of netedit
    // bool
    // NBNetBuilder::runningNetedit() {
    //     // see GNELoadThread::fillOptions
    //     return OptionsCont::getOptions().exists("new");
    // }
    #[allow(unused_variables)]
    pub fn running_netedit() -> bool {
        // see GNELoadThread::fillOptions
        utils::options::options_cont::exists("new")
    }

    /// C++: `void NBNetBuilder::moveToOrigin(GeoConvHelper &, bool)` [private]
    /// shift network so its lower left corner is at 0,0
    // void
    // NBNetBuilder::moveToOrigin(GeoConvHelper& geoConvHelper, bool lefthand) {
    //     long before = PROGRESS_BEGIN_TIME_MESSAGE(TL("Moving network to origin"));
    //     Boundary boundary = geoConvHelper.getConvBoundary();
    //     const double x = -boundary.xmin();
    //     const double y = -(lefthand ? boundary.ymax() : boundary.ymin());
    //     //if (lefthand) {
    //     //    y = boundary.ymax();
    //     //}
    //     for (std::map<std::string, NBNode*>::const_iterator i = myNodeCont.begin(); i != myNodeCont.end(); ++i) {
    //         (*i).second->reshiftPosition(x, y);
    //     }
    //     for (std::map<std::string, NBEdge*>::const_iterator i = myEdgeCont.begin(); i != myEdgeCont.end(); ++i) {
    //         (*i).second->reshiftPosition(x, y);
    //     }
    //     for (std::map<std::string, NBDistrict*>::const_iterator i = myDistrictCont.begin(); i != myDistrictCont.end(); ++i) {
    //         (*i).second->reshiftPosition(x, y);
    //     }
    //     for (const auto& stopIt : myPTStopCont.getStops()) {
    //         stopIt.second->reshiftPosition(x, y);
    //     }
    //     geoConvHelper.moveConvertedBy(x, y);
    //     PROGRESS_TIME_MESSAGE(before);
    // }
    #[allow(unused_variables)]
    pub fn move_to_origin(&mut self, geoConvHelper: &mut GeoConvHelper, lefthand: bool) {
        let boundary = *geoConvHelper.get_conv_boundary();
        let x = -boundary.xmin();
        let y = -(if lefthand { boundary.ymax() } else { boundary.ymin() });
        for node in self.myNodeCont.myNodes.values() {
            node.m().reshift_position(x, y);
        }
        for edge in self.myEdgeCont.myEdges.values() {
            edge.m().reshift_position(x, y);
        }
        for district in self.myDistrictCont.myDistricts.values() {
            district.m().reshift_position(x, y);
        }
        for stop in self.myPTStopCont.get_stops().values() {
            stop.m().reshift_position(x, y);
        }
        geoConvHelper.move_converted_by(x, y);
    }

    /// C++: `void NBNetBuilder::roundInputs()` [private]
    /// ensure consistency between input and output geometries and speeds
    // void
    // NBNetBuilder::roundInputs() {
    //     // ensure that computed speeds (using during right-of-way computation) are the same as the written speeds
    //     for (auto item : myEdgeCont) {
    //         item.second->roundSpeed();
    //     }
    //     // ensure that derived junction and lane geometry computation is done with the same input
    //     // edge coordinates as those that will be written to the output.
    //     for (std::map<std::string, NBNode*>::const_iterator i = myNodeCont.begin(); i != myNodeCont.end(); ++i) {
    //         (*i).second->roundGeometry();
    //     }
    //     for (std::map<std::string, NBEdge*>::const_iterator i = myEdgeCont.begin(); i != myEdgeCont.end(); ++i) {
    //         (*i).second->roundGeometry();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn round_inputs(&mut self) {
        // ensure that computed speeds (using during right-of-way computation) are the same as the written speeds
        for edge in self.myEdgeCont.myEdges.values() {
            edge.m().round_speed();
        }
        // ensure that derived junction and lane geometry computation is done with the same input
        // edge coordinates as those that will be written to the output.
        for node in self.myNodeCont.myNodes.values() {
            node.m().round_geometry();
        }
        for edge in self.myEdgeCont.myEdges.values() {
            edge.m().round_geometry();
        }
    }

    /// C++: `void NBNetBuilder::mirrorX()` [private]
    /// mirror the network along the X-axis
    // void
    // NBNetBuilder::mirrorX() {
    //     // mirror the network along the X-axis
    //     for (std::map<std::string, NBNode*>::const_iterator i = myNodeCont.begin(); i != myNodeCont.end(); ++i) {
    //         (*i).second->mirrorX();
    //     }
    //     for (std::map<std::string, NBEdge*>::const_iterator i = myEdgeCont.begin(); i != myEdgeCont.end(); ++i) {
    //         (*i).second->mirrorX();
    //     }
    //     for (std::map<std::string, NBDistrict*>::const_iterator i = myDistrictCont.begin(); i != myDistrictCont.end(); ++i) {
    //         (*i).second->mirrorX();
    //     }
    //     for (const auto& stopIt : myPTStopCont.getStops()) {
    //         stopIt.second->mirrorX();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn mirror_x(&mut self) {
        // mirror the network along the X-axis
        for node in self.myNodeCont.myNodes.values() {
            node.m().mirror_x();
        }
        for edge in self.myEdgeCont.myEdges.values() {
            edge.m().mirror_x();
        }
        for district in self.myDistrictCont.myDistricts.values() {
            district.m().mirror_x();
        }
        for stop in self.myPTStopCont.get_stops().values() {
            stop.m().mirror_x();
        }
    }

    /// C++: `void NBNetBuilder::NBNetBuilder(const NBNetBuilder &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &NBNetBuilder) -> Self {
        panic!("NBNetBuilder: copy constructor is invalidated in C++ (private, never defined)")
    }

    /// C++: `NBNetBuilder & NBNetBuilder::operator=(const NBNetBuilder &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &NBNetBuilder) -> &mut NBNetBuilder {
        panic!("NBNetBuilder: assignment is invalidated in C++ (private operator=, never defined)")
    }

}

/// Sorts nodes by their ids
/// C++ class `NBNetBuilder::by_id_sorter` (netbuild/NBNetBuilder.h)
pub struct NBNetBuilder_by_id_sorter {
}

impl NBNetBuilder_by_id_sorter {
    /// C++: `void by_id_sorter::by_id_sorter()`
    /// constructor
    // explicit by_id_sorter() {}
    #[allow(unused_variables)]
    pub fn new() -> Self {
        NBNetBuilder_by_id_sorter {}
    }

    /// C++: `int by_id_sorter::operator()(const NBNode *, const NBNode *) const` [const]
    /// selection operator
    // int operator()(const NBNode* n1, const NBNode* n2) const {
    //             return n1->getID() < n2->getID();
    //         }
    #[allow(unused_variables)]
    pub fn call_op(&self, n1: Ptr<NBNode>, n2: Ptr<NBNode>) -> i32 {
        (n1.get_id() < n2.get_id()) as i32
    }

}


