//! Port of netwrite/NWWriter_SUMO.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::sumo_vehicle_class::write_permissions;
use utils::geom::POSITION_EPS;

/// C++ enum `NWWriter_SUMO::ConnectionStyle`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum NWWriter_SUMO_ConnectionStyle {
    SUMONET = 0,
    PLAIN = 1,
    PLAIN_NAVTEQ = 2,
    TLL = 3,
}

/// Exporter writing networks using the SUMO format
/// C++ class `NWWriter_SUMO` (netwrite/NWWriter_SUMO.h)
pub struct NWWriter_SUMO {
}

impl NWWriter_SUMO {
    /// C++: `void NWWriter_SUMO::writeNetwork(const OptionsCont &, NBNetBuilder &)` [static]
    /// Writes the network into a SUMO-file
    // void
    // NWWriter_SUMO::writeNetwork(const OptionsCont& oc, NBNetBuilder& nb) {
    //     // check whether a sumo net-file shall be generated
    //     if (!oc.isSet("output-file")) {
    //         return;
    //     }
    //     OutputDevice& device = OutputDevice::getDevice(oc.getString("output-file"));
    //     std::map<SumoXMLAttr, std::string> attrs;
    //     attrs[SUMO_ATTR_VERSION] = toString(NETWORK_VERSION);
    //     if (oc.getBool("lefthand") != oc.getBool("flip-y-axis")) {
    //         attrs[SUMO_ATTR_LEFTHAND] = "true";
    //     } else if (oc.getBool("lefthand")) {
    //         // network was flipped, correct written link directions
    //         OptionsCont::getOptions().resetWritable();
    //         OptionsCont::getOptions().set("lefthand", "false");
    //     }
    //     LaneSpreadFunction defaultSpread = SUMOXMLDefinitions::LaneSpreadFunctions.get(oc.getString("default.spreadtype"));
    //     const int cornerDetail = oc.getInt("junctions.corner-detail");
    //     if (cornerDetail > 0) {
    //         attrs[SUMO_ATTR_CORNERDETAIL] = toString(cornerDetail);
    //     }
    //     if (!oc.isDefault("junctions.internal-link-detail")) {
    //         attrs[SUMO_ATTR_LINKDETAIL] = toString(oc.getInt("junctions.internal-link-detail"));
    //     }
    //     if (oc.getBool("rectangular-lane-cut")) {
    //         attrs[SUMO_ATTR_RECTANGULAR_LANE_CUT] = "true";
    //     }
    //     if (oc.getBool("crossings.guess") || oc.getBool("walkingareas")) {
    //         attrs[SUMO_ATTR_WALKINGAREAS] = "true";
    //     }
    //     if (oc.getFloat("junctions.limit-turn-speed") > 0) {
    //         attrs[SUMO_ATTR_LIMIT_TURN_SPEED] = toString(oc.getFloat("junctions.limit-turn-speed"));
    //     }
    //     if (!oc.isDefault("check-lane-foes.all")) {
    //         attrs[SUMO_ATTR_CHECKLANEFOES_ALL] = toString(oc.getBool("check-lane-foes.all"));
    //     }
    //     if (!oc.isDefault("check-lane-foes.roundabout")) {
    //         attrs[SUMO_ATTR_CHECKLANEFOES_ROUNDABOUT] = toString(oc.getBool("check-lane-foes.roundabout"));
    //     }
    //     if (!oc.isDefault("tls.ignore-internal-junction-jam")) {
    //         attrs[SUMO_ATTR_TLS_IGNORE_INTERNAL_JUNCTION_JAM] = toString(oc.getBool("tls.ignore-internal-junction-jam"));
    //     }
    //     if (defaultSpread != LaneSpreadFunction::RIGHT) {
    //         attrs[SUMO_ATTR_SPREADTYPE] = oc.getString("default.spreadtype");
    //     }
    //     if (oc.exists("geometry.avoid-overlap") && !oc.getBool("geometry.avoid-overlap")) {
    //         attrs[SUMO_ATTR_AVOID_OVERLAP] = toString(oc.getBool("geometry.avoid-overlap"));
    //     }
    //     if (oc.exists("junctions.higher-speed") && oc.getBool("junctions.higher-speed")) {
    //         attrs[SUMO_ATTR_HIGHER_SPEED] = toString(oc.getBool("junctions.higher-speed"));
    //     }
    //     if (oc.exists("internal-junctions.vehicle-width") && !oc.isDefault("internal-junctions.vehicle-width")) {
    //         attrs[SUMO_ATTR_INTERNAL_JUNCTIONS_VEHICLE_WIDTH] = toString(oc.getFloat("internal-junctions.vehicle-width"));
    //     }
    //     if (!oc.isDefault("junctions.minimal-shape")) {
    //         attrs[SUMO_ATTR_JUNCTIONS_MINIMAL_SHAPE] = toString(oc.getBool("junctions.minimal-shape"));
    //     }
    //     if (!oc.isDefault("junctions.endpoint-shape")) {
    //         attrs[SUMO_ATTR_JUNCTIONS_ENDPOINT_SHAPE] = toString(oc.getBool("junctions.endpoint-shape"));
    //     }
    //     device.writeXMLHeader("net", "net_file.xsd", attrs); // street names may contain non-ascii chars
    //     device.lf();
    //     // get involved container
    //     const NBNodeCont& nc = nb.getNodeCont();
    //     const NBEdgeCont& ec = nb.getEdgeCont();
    //     const NBDistrictCont& dc = nb.getDistrictCont();
    //
    //     // write network offsets and projection
    //     GeoConvHelper::writeLocation(device);
    //
    //     // write edge types and restrictions
    //     std::set<std::string> usedTypes = ec.getUsedTypes();
    //     nb.getTypeCont().writeEdgeTypes(device, usedTypes);
    //
    //     // write inner lanes
    //     if (!oc.getBool("no-internal-links")) {
    //         bool hadAny = false;
    //         for (std::map<std::string, NBNode*>::const_iterator i = nc.begin(); i != nc.end(); ++i) {
    //             hadAny |= writeInternalEdges(device, ec, *(*i).second);
    //         }
    //         if (hadAny) {
    //             device.lf();
    //         }
    //     }
    //
    //     // write edges with lanes and connected edges
    //     bool noNames = !oc.getBool("output.street-names");
    //     for (std::map<std::string, NBEdge*>::const_iterator i = ec.begin(); i != ec.end(); ++i) {
    //         writeEdge(device, *(*i).second, noNames, defaultSpread);
    //     }
    //     device.lf();
    //
    //     // write tls logics
    //     writeTrafficLights(device, nb.getTLLogicCont());
    //
    //     // write the nodes (junctions)
    //     for (std::map<std::string, NBNode*>::const_iterator i = nc.begin(); i != nc.end(); ++i) {
    //         writeJunction(device, *(*i).second);
    //     }
    //     device.lf();
    //     const bool includeInternal = !oc.getBool("no-internal-links");
    //     if (includeInternal) {
    //         // ... internal nodes if not unwanted
    //         bool hadAny = false;
    //         for (std::map<std::string, NBNode*>::const_iterator i = nc.begin(); i != nc.end(); ++i) {
    //             hadAny |= writeInternalNodes(device, *(*i).second);
    //         }
    //         if (hadAny) {
    //             device.lf();
    //         }
    //     }
    //
    //     // write the successors of lanes
    //     int numConnections = 0;
    //     for (std::map<std::string, NBEdge*>::const_iterator it_edge = ec.begin(); it_edge != ec.end(); it_edge++) {
    //         NBEdge* from = it_edge->second;
    //         const std::vector<NBEdge::Connection>& connections = from->getConnections();
    //         numConnections += (int)connections.size();
    //         for (const NBEdge::Connection& con : connections) {
    //             writeConnection(device, *from, con, includeInternal);
    //         }
    //     }
    //     if (numConnections > 0) {
    //         device.lf();
    //     }
    //     if (includeInternal) {
    //         // ... internal successors if not unwanted
    //         bool hadAny = false;
    //         for (std::map<std::string, NBNode*>::const_iterator i = nc.begin(); i != nc.end(); ++i) {
    //             hadAny |= writeInternalConnections(device, *(*i).second);
    //         }
    //         if (hadAny) {
    //             device.lf();
    //         }
    //     }
    //     for (std::map<std::string, NBNode*>::const_iterator i = nc.begin(); i != nc.end(); ++i) {
    //         NBNode* node = (*i).second;
    //         // write connections from pedestrian crossings
    //         std::vector<NBNode::Crossing*> crossings = node->getCrossings();
    //         for (auto c : crossings) {
    //             NWWriter_SUMO::writeInternalConnection(device, c->id, c->nextWalkingArea, 0, 0, "", LinkDirection::STRAIGHT, c->tlID, c->tlLinkIndex2);
    //         }
    //         // write connections from pedestrian walking areas
    //         for (const NBNode::WalkingArea& wa : node->getWalkingAreas()) {
    //             for (const std::string& cID : wa.nextCrossings) {
    //                 const NBNode::Crossing& nextCrossing = *node->getCrossing(cID);
    //                 // connection to next crossing (may be tls-controlled)
    //                 device.openTag(SUMO_TAG_CONNECTION);
    //                 device.writeAttr(SUMO_ATTR_FROM, wa.id);
    //                 device.writeAttr(SUMO_ATTR_TO, cID);
    //                 device.writeAttr(SUMO_ATTR_FROM_LANE, 0);
    //                 device.writeAttr(SUMO_ATTR_TO_LANE, 0);
    //                 if (nextCrossing.tlID != "") {
    //                     device.writeAttr(SUMO_ATTR_TLID, nextCrossing.tlID);
    //                     assert(nextCrossing.tlLinkIndex >= 0);
    //                     device.writeAttr(SUMO_ATTR_TLLINKINDEX, nextCrossing.tlLinkIndex);
    //                 }
    //                 device.writeAttr(SUMO_ATTR_DIR, LinkDirection::STRAIGHT);
    //                 device.writeAttr(SUMO_ATTR_STATE, nextCrossing.priority ? LINKSTATE_MAJOR : LINKSTATE_MINOR);
    //                 device.closeTag();
    //             }
    //             // optional connections from/to sidewalk
    //             std::string edgeID;
    //             int laneIndex;
    //             for (const std::string& sw : wa.nextSidewalks) {
    //                 NBHelpers::interpretLaneID(sw, edgeID, laneIndex);
    //                 NWWriter_SUMO::writeInternalConnection(device, wa.id, edgeID, 0, laneIndex, "");
    //             }
    //             for (const std::string& sw : wa.prevSidewalks) {
    //                 NBHelpers::interpretLaneID(sw, edgeID, laneIndex);
    //                 NWWriter_SUMO::writeInternalConnection(device, edgeID, wa.id, laneIndex, 0, "");
    //             }
    //         }
    //     }
    //
    //     // write loaded prohibitions
    //     for (std::map<std::string, NBNode*>::const_iterator i = nc.begin(); i != nc.end(); ++i) {
    //         writeProhibitions(device, i->second->getProhibitions(), ec);
    //     }
    //
    //     // write roundabout information
    //     writeRoundabouts(device, ec.getRoundabouts(), ec);
    //
    //     // write the districts
    //     if (dc.size() != 0 && oc.isDefault("taz-output")) {
    //         WRITE_WARNING(TL("Embedding TAZ-data inside the network is deprecated. Use option --taz-output instead"));
    //         for (std::map<std::string, NBDistrict*>::const_iterator i = dc.begin(); i != dc.end(); i++) {
    //             writeDistrict(device, *(*i).second);
    //         }
    //         device.lf();
    //     }
    //     device.close();
    // }
    #[allow(unused_variables)]
    pub fn write_network(oc: &OptionsCont, nb: &mut NBNetBuilder) {
        // check whether a sumo net-file shall be generated
        if !oc.is_set("output-file", true).unwrap() {
            return;
        }
        let device = OutputDevice::get_device(&oc.get_string("output-file").unwrap(), true).unwrap();
        let mut guard = device.borrow_mut();
        let into: &mut OutputDevice = &mut guard;
        let mut attrs: BTreeMap<SumoXMLAttr, String> = BTreeMap::new();
        let ver = utils::common::std_defs::NETWORK_VERSION;
        attrs.insert(SUMO_ATTR_VERSION, format!("{}.{}", ver.0, ver.1 as i32));
        if oc.get_bool("lefthand").unwrap() != oc.get_bool("flip-y-axis").unwrap() {
            attrs.insert(SUMO_ATTR_LEFTHAND, "true".to_string());
        } else if oc.get_bool("lefthand").unwrap() {
            // network was flipped, correct written link directions
            utils::options::options_cont::with_options(|o| {
                o.reset_writable();
                o.set("lefthand", "false", false).unwrap();
            });
        }
        let defaultSpread = LaneSpreadFunctions.get(&oc.get_string("default.spreadtype").unwrap()).unwrap();
        let cornerDetail = oc.get_int("junctions.corner-detail").unwrap();
        if cornerDetail > 0 {
            attrs.insert(SUMO_ATTR_CORNERDETAIL, cornerDetail.to_string());
        }
        if !oc.is_default("junctions.internal-link-detail") {
            attrs.insert(SUMO_ATTR_LINKDETAIL, oc.get_int("junctions.internal-link-detail").unwrap().to_string());
        }
        if oc.get_bool("rectangular-lane-cut").unwrap() {
            attrs.insert(SUMO_ATTR_RECTANGULAR_LANE_CUT, "true".to_string());
        }
        if oc.get_bool("crossings.guess").unwrap() || oc.get_bool("walkingareas").unwrap() {
            attrs.insert(SUMO_ATTR_WALKINGAREAS, "true".to_string());
        }
        if oc.get_float("junctions.limit-turn-speed").unwrap() > 0.0 {
            attrs.insert(SUMO_ATTR_LIMIT_TURN_SPEED, to_str_f64(oc.get_float("junctions.limit-turn-speed").unwrap()));
        }
        if !oc.is_default("check-lane-foes.all") {
            attrs.insert(SUMO_ATTR_CHECKLANEFOES_ALL, oc.get_bool("check-lane-foes.all").unwrap().to_string());
        }
        if !oc.is_default("check-lane-foes.roundabout") {
            attrs.insert(SUMO_ATTR_CHECKLANEFOES_ROUNDABOUT, oc.get_bool("check-lane-foes.roundabout").unwrap().to_string());
        }
        if !oc.is_default("tls.ignore-internal-junction-jam") {
            attrs.insert(SUMO_ATTR_TLS_IGNORE_INTERNAL_JUNCTION_JAM, oc.get_bool("tls.ignore-internal-junction-jam").unwrap().to_string());
        }
        if defaultSpread != LaneSpreadFunction::RIGHT {
            attrs.insert(SUMO_ATTR_SPREADTYPE, oc.get_string("default.spreadtype").unwrap());
        }
        if oc.exists("geometry.avoid-overlap") && !oc.get_bool("geometry.avoid-overlap").unwrap() {
            // C++ toString(bool) uses ostringstream without boolalpha -> "0"/"1"
            let v = oc.get_bool("geometry.avoid-overlap").unwrap();
            attrs.insert(SUMO_ATTR_AVOID_OVERLAP, if v { "1" } else { "0" }.to_string());
        }
        if oc.exists("junctions.higher-speed") && oc.get_bool("junctions.higher-speed").unwrap() {
            attrs.insert(SUMO_ATTR_HIGHER_SPEED, oc.get_bool("junctions.higher-speed").unwrap().to_string());
        }
        if oc.exists("internal-junctions.vehicle-width") && !oc.is_default("internal-junctions.vehicle-width") {
            attrs.insert(SUMO_ATTR_INTERNAL_JUNCTIONS_VEHICLE_WIDTH, to_str_f64(oc.get_float("internal-junctions.vehicle-width").unwrap()));
        }
        if !oc.is_default("junctions.minimal-shape") {
            attrs.insert(SUMO_ATTR_JUNCTIONS_MINIMAL_SHAPE, oc.get_bool("junctions.minimal-shape").unwrap().to_string());
        }
        if !oc.is_default("junctions.endpoint-shape") {
            attrs.insert(SUMO_ATTR_JUNCTIONS_ENDPOINT_SHAPE, oc.get_bool("junctions.endpoint-shape").unwrap().to_string());
        }
        into.write_xml_header("net", "net_file.xsd", attrs, true); // street names may contain non-ascii chars
        into.lf();

        // write network offsets and projection
        write_location(into);

        // write edge types and restrictions
        let usedTypes = nb.myEdgeCont.get_used_types();
        nb.myTypeCont.write_edge_types(into, &usedTypes);

        // write inner lanes
        if !oc.get_bool("no-internal-links").unwrap() {
            let mut hadAny = false;
            for node in nb.myNodeCont.myNodes.values() {
                hadAny |= Self::write_internal_edges(into, &nb.myEdgeCont, node.m());
            }
            if hadAny {
                into.lf();
            }
        }

        // write edges with lanes and connected edges
        let noNames = !oc.get_bool("output.street-names").unwrap();
        for edge in nb.myEdgeCont.myEdges.values() {
            Self::write_edge(into, edge.m(), noNames, defaultSpread);
        }
        into.lf();

        // write tls logics
        Self::write_traffic_lights(into, &nb.myTLLCont, NWWriter_SUMO_ConnectionStyle::SUMONET);

        // write the nodes (junctions)
        for node in nb.myNodeCont.myNodes.values() {
            Self::write_junction(into, node.m());
        }
        into.lf();
        let includeInternal = !oc.get_bool("no-internal-links").unwrap();
        if includeInternal {
            // ... internal nodes if not unwanted
            let mut hadAny = false;
            for node in nb.myNodeCont.myNodes.values() {
                hadAny |= Self::write_internal_nodes(into, node.m());
            }
            if hadAny {
                into.lf();
            }
        }

        // write the successors of lanes
        let mut numConnections: i32 = 0;
        for from in nb.myEdgeCont.myEdges.values() {
            let from_e = from.m();
            let connections = from_e.get_connections();
            numConnections += connections.len() as i32;
            for con in connections.iter() {
                Self::write_connection(into, from_e, con, includeInternal, NWWriter_SUMO_ConnectionStyle::SUMONET, false, false);
            }
        }
        if numConnections > 0 {
            into.lf();
        }
        if includeInternal {
            // ... internal successors if not unwanted
            let mut hadAny = false;
            for node in nb.myNodeCont.myNodes.values() {
                hadAny |= Self::write_internal_connections(into, node.m());
            }
            if hadAny {
                into.lf();
            }
        }
        for node in nb.myNodeCont.myNodes.values() {
            let n = node.m();
            // write connections from pedestrian crossings
            let crossings = n.get_crossings();
            for c in crossings.iter() {
                Self::write_internal_connection(into, &c.id, &c.nextWalkingArea, 0, 0, "", LinkDirection::STRAIGHT, &c.tlID, c.tlLinkIndex2, false, NBEDGE_UNSPECIFIED_VISIBILITY_DISTANCE);
            }
            // write connections from pedestrian walking areas
            for wa in n.get_walking_areas().iter() {
                for cID in wa.nextCrossings.iter() {
                    let nextCrossing = n.get_crossing(cID);
                    // connection to next crossing (may be tls-controlled)
                    into.open_tag_id(SUMO_TAG_CONNECTION);
                    into.write_attr(SUMO_ATTR_FROM, wa.id.as_str());
                    into.write_attr(SUMO_ATTR_TO, cID.as_str());
                    into.write_attr(SUMO_ATTR_FROM_LANE, &0i32);
                    into.write_attr(SUMO_ATTR_TO_LANE, &0i32);
                    if nextCrossing.tlID != "" {
                        into.write_attr(SUMO_ATTR_TLID, nextCrossing.tlID.as_str());
                        into.write_attr(SUMO_ATTR_TLLINKINDEX, &nextCrossing.tlLinkIndex);
                    }
                    into.write_attr(SUMO_ATTR_DIR, &LinkDirection::STRAIGHT);
                    into.write_attr(SUMO_ATTR_STATE, LinkStates.get_string(if nextCrossing.priority { LINKSTATE_MAJOR } else { LINKSTATE_MINOR }));
                    into.close_tag("");
                }
                // optional connections from/to sidewalk
                let mut edgeID = String::new();
                let mut laneIndex: i32 = 0;
                for sw in wa.nextSidewalks.iter() {
                    NBHelpers::interpret_lane_id(sw, &mut edgeID, &mut laneIndex);
                    Self::write_internal_connection(into, &wa.id, &edgeID, 0, laneIndex, "", LinkDirection::STRAIGHT, "", -1, false, NBEDGE_UNSPECIFIED_VISIBILITY_DISTANCE);
                }
                for sw in wa.prevSidewalks.iter() {
                    NBHelpers::interpret_lane_id(sw, &mut edgeID, &mut laneIndex);
                    Self::write_internal_connection(into, &edgeID, &wa.id, laneIndex, 0, "", LinkDirection::STRAIGHT, "", -1, false, NBEDGE_UNSPECIFIED_VISIBILITY_DISTANCE);
                }
            }
        }

        // write loaded prohibitions
        for node in nb.myNodeCont.myNodes.values() {
            Self::write_prohibitions(into, node.m().get_prohibitions(), &nb.myEdgeCont);
        }

        // write roundabout information
        Self::write_roundabouts(into, &nb.myEdgeCont.get_roundabouts(), &nb.myEdgeCont);

        // write the districts
        if nb.myDistrictCont.myDistricts.len() != 0 && oc.is_default("taz-output") {
            utils::common::msg_handler::write_warning("Embedding TAZ-data inside the network is deprecated. Use option --taz-output instead");
            for d in nb.myDistrictCont.myDistricts.values() {
                Self::write_district(into, d.m());
            }
            into.lf();
        }
        let _ = into.close();
    }

    /// C++: `void NWWriter_SUMO::writeConnection(OutputDevice &, const NBEdge &, const NBEdge::Connection &, bool, ConnectionStyle, bool, bool)` [static]
    /// Writes connections outgoing from the given edge (also used in NWWriter_XML)
    // void
    // NWWriter_SUMO::writeConnection(OutputDevice& into, const NBEdge& from, const NBEdge::Connection& c,
    //                                bool includeInternal, ConnectionStyle style, bool useGeo, bool geoAccuracy) {
    //     assert(c.toEdge != nullptr);
    //     into.openTag(SUMO_TAG_CONNECTION);
    //     into.writeAttr(SUMO_ATTR_FROM, from.getID());
    //     into.writeAttr(SUMO_ATTR_TO, c.toEdge->getID());
    //     into.writeAttr(SUMO_ATTR_FROM_LANE, c.fromLane);
    //     into.writeAttr(SUMO_ATTR_TO_LANE, c.toLane);
    //     if (style != TLL) {
    //         into.writeOptionalAttr(SUMO_ATTR_PASS, c.mayDefinitelyPass, !c.mayDefinitelyPass);
    //         into.writeOptionalAttr<bool>(SUMO_ATTR_KEEP_CLEAR, false, c.keepClear != KEEPCLEAR_FALSE);
    //         into.writeOptionalAttr(SUMO_ATTR_CONTPOS, c.contPos, c.contPos == NBEdge::UNSPECIFIED_CONTPOS);
    //         writePermissions(into, c.permissions);
    //         const bool changeLeft = (c.changeLeft != SVC_UNSPECIFIED && c.changeLeft != SVCAll && c.changeLeft != SVC_IGNORING);
    //         into.writeOptionalAttr(SUMO_ATTR_CHANGE_LEFT, getVehicleClassNames(c.changeLeft), !changeLeft);
    //         const bool changeRight = (c.changeRight != SVC_UNSPECIFIED && c.changeRight != SVCAll && c.changeRight != SVC_IGNORING);
    //         into.writeOptionalAttr(SUMO_ATTR_CHANGE_RIGHT, getVehicleClassNames(c.changeRight), !changeRight);
    //         into.writeOptionalAttr(SUMO_ATTR_SPEED, c.speed, c.speed == NBEdge::UNSPECIFIED_SPEED);
    //         into.writeOptionalAttr(SUMO_ATTR_LENGTH, c.customLength, c.customLength == NBEdge::UNSPECIFIED_LOADED_LENGTH);
    //         writeShape(into, GeoConvHelper::getFinal(), c.customShape, SUMO_ATTR_SHAPE, useGeo, geoAccuracy, c.customShape.size() != 0);
    //         into.writeOptionalAttr(SUMO_ATTR_UNCONTROLLED, c.uncontrolled, !c.uncontrolled);
    //         into.writeOptionalAttr(SUMO_ATTR_INDIRECT, c.indirectLeft, !c.indirectLeft);
    //         into.writeOptionalAttr(SUMO_ATTR_TYPE, c.edgeType, c.edgeType == "");
    //     }
    //     if (style != PLAIN && style != PLAIN_NAVTEQ) {
    //         if (includeInternal) {
    //             into.writeAttr(SUMO_ATTR_VIA, c.getInternalLaneID());
    //         }
    //         // set information about the controlling tl if any
    //         if (c.tlID != "") {
    //             into.writeAttr(SUMO_ATTR_TLID, c.tlID);
    //             into.writeAttr(SUMO_ATTR_TLLINKINDEX, c.tlLinkIndex);
    //             if (c.tlLinkIndex2 >= 0) {
    //                 into.writeAttr(SUMO_ATTR_TLLINKINDEX2, c.tlLinkIndex2);
    //             }
    //         }
    //     }
    //     if (style != TLL) {
    //         if (style == SUMONET) {
    //             // write the direction information
    //             LinkDirection dir = from.getToNode()->getDirection(&from, c.toEdge, OptionsCont::getOptions().getBool("lefthand"));
    //             assert(dir != LinkDirection::NODIR);
    //             into.writeAttr(SUMO_ATTR_DIR, toString(dir));
    //             // write the state information
    //             const LinkState linkState = from.getToNode()->getLinkState(
    //                                             &from, c.toEdge, c.fromLane, c.toLane, c.mayDefinitelyPass, c.tlID);
    //             into.writeAttr(SUMO_ATTR_STATE, linkState);
    //             if (linkState == LINKSTATE_MINOR
    //                     && c.visibility == NBEdge::UNSPECIFIED_VISIBILITY_DISTANCE
    //                     && c.toEdge->getJunctionPriority(c.toEdge->getToNode()) == NBEdge::JunctionPriority::ROUNDABOUT) {
    //                 const double visibilityDistance = OptionsCont::getOptions().getFloat("roundabouts.visibility-distance");
    //                 if (visibilityDistance != NBEdge::UNSPECIFIED_VISIBILITY_DISTANCE) {
    //                     into.writeAttr(SUMO_ATTR_VISIBILITY_DISTANCE, visibilityDistance);
    //                 }
    //             }
    //         }
    //         if (c.visibility != NBEdge::UNSPECIFIED_VISIBILITY_DISTANCE) {
    //             into.writeAttr(SUMO_ATTR_VISIBILITY_DISTANCE, c.visibility);
    //         }
    //     }
    //     if (style == PLAIN_NAVTEQ) {
    //         NWWriter_DlrNavteq::writeConnectionAttrs(into, c);
    //     }
    //     c.writeParams(into);
    //     into.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn write_connection(into: &mut OutputDevice, from: &NBEdge, c: &NBEdge_Connection, includeInternal: bool, style: NWWriter_SUMO_ConnectionStyle, useGeo: bool, geoAccuracy: bool) {
        into.open_tag_id(SUMO_TAG_CONNECTION);
        into.write_attr(SUMO_ATTR_FROM, from.get_id());
        into.write_attr(SUMO_ATTR_TO, c.toEdge.get_id());
        into.write_attr(SUMO_ATTR_FROM_LANE, &c.fromLane);
        into.write_attr(SUMO_ATTR_TO_LANE, &c.toLane);
        if style != NWWriter_SUMO_ConnectionStyle::TLL {
            into.write_optional_attr(SUMO_ATTR_PASS, &c.mayDefinitelyPass, !c.mayDefinitelyPass);
            into.write_optional_attr(SUMO_ATTR_KEEP_CLEAR, &false, c.keepClear != KeepClear::KEEPCLEAR_FALSE);
            into.write_optional_attr(SUMO_ATTR_CONTPOS, &c.contPos, c.contPos == NBEDGE_UNSPECIFIED_CONTPOS);
            write_permissions(into, c.permissions);
            let changeLeft = c.changeLeft != SVC_UNSPECIFIED && c.changeLeft != SVC_ALL && c.changeLeft != SVC_IGNORING;
            into.write_optional_attr(SUMO_ATTR_CHANGE_LEFT, &get_vehicle_class_names(c.changeLeft, false), !changeLeft);
            let changeRight = c.changeRight != SVC_UNSPECIFIED && c.changeRight != SVC_ALL && c.changeRight != SVC_IGNORING;
            into.write_optional_attr(SUMO_ATTR_CHANGE_RIGHT, &get_vehicle_class_names(c.changeRight, false), !changeRight);
            into.write_optional_attr(SUMO_ATTR_SPEED, &c.speed, c.speed == NBEDGE_UNSPECIFIED_SPEED);
            into.write_optional_attr(SUMO_ATTR_LENGTH, &c.customLength, c.customLength == NBEDGE_UNSPECIFIED_LOADED_LENGTH);
            let hasShape = c.customShape.len() != 0;
            let customShape = c.customShape.clone();
            GeoConvHelper::with_final(|g| Self::write_shape(into, g, customShape, SUMO_ATTR_SHAPE, useGeo, geoAccuracy, hasShape));
            into.write_optional_attr(SUMO_ATTR_UNCONTROLLED, &c.uncontrolled, !c.uncontrolled);
            into.write_optional_attr(SUMO_ATTR_INDIRECT, &c.indirectLeft, !c.indirectLeft);
            into.write_optional_attr(SUMO_ATTR_TYPE, c.edgeType.as_str(), c.edgeType == "");
        }
        if style != NWWriter_SUMO_ConnectionStyle::PLAIN && style != NWWriter_SUMO_ConnectionStyle::PLAIN_NAVTEQ {
            if includeInternal {
                into.write_attr(SUMO_ATTR_VIA, c.get_internal_lane_id().as_str());
            }
            // set information about the controlling tl if any
            if c.tlID != "" {
                into.write_attr(SUMO_ATTR_TLID, c.tlID.as_str());
                into.write_attr(SUMO_ATTR_TLLINKINDEX, &c.tlLinkIndex);
                if c.tlLinkIndex2 >= 0 {
                    into.write_attr(SUMO_ATTR_TLLINKINDEX2, &c.tlLinkIndex2);
                }
            }
        }
        if style != NWWriter_SUMO_ConnectionStyle::TLL {
            if style == NWWriter_SUMO_ConnectionStyle::SUMONET {
                // write the direction information
                let dir = from.get_to_node().get_direction(Ptr::new(from), c.toEdge, utils::options::options_cont::get_bool("lefthand"));
                into.write_attr(SUMO_ATTR_DIR, &dir);
                // write the state information
                let linkState = from.get_to_node().get_link_state(Ptr::new(from), c.toEdge, c.fromLane, c.toLane, c.mayDefinitelyPass, &c.tlID);
                into.write_attr(SUMO_ATTR_STATE, LinkStates.get_string(linkState));
                if linkState == LINKSTATE_MINOR
                    && c.visibility == NBEDGE_UNSPECIFIED_VISIBILITY_DISTANCE
                    && c.toEdge.get_junction_priority(c.toEdge.get_to_node()) == NBEdge_JunctionPriority::ROUNDABOUT as i32
                {
                    let visibilityDistance = utils::options::options_cont::get_float("roundabouts.visibility-distance");
                    if visibilityDistance != NBEDGE_UNSPECIFIED_VISIBILITY_DISTANCE {
                        into.write_attr(SUMO_ATTR_VISIBILITY_DISTANCE, &visibilityDistance);
                    }
                }
            }
            if c.visibility != NBEDGE_UNSPECIFIED_VISIBILITY_DISTANCE {
                into.write_attr(SUMO_ATTR_VISIBILITY_DISTANCE, &c.visibility);
            }
        }
        if style == NWWriter_SUMO_ConnectionStyle::PLAIN_NAVTEQ {
            NWWriter_DlrNavteq::write_connection_attrs(into, c);
        }
        c.base.write_params(into);
        into.close_tag("");
    }

    /// C++: `void NWWriter_SUMO::writeProhibitions(OutputDevice &, const NBConnectionProhibits &, const NBEdgeCont &)` [static]
    /// writes the given prohibitions
    // void
    // NWWriter_SUMO::writeProhibitions(OutputDevice& into, const NBConnectionProhibits& prohibitions, const NBEdgeCont& ec) {
    //     // the edges may have been erased from NBEdgeCont but their pointers are still valid so we need to check
    //     for (NBConnectionProhibits::const_iterator j = prohibitions.begin(); j != prohibitions.end(); j++) {
    //         NBConnection prohibited = (*j).first;
    //         if (ec.retrieve(prohibited.getFrom()->getID()) == nullptr
    //                 || ec.retrieve(prohibited.getTo()->getID()) == nullptr) {
    //             continue;
    //         }
    //         const NBConnectionVector& prohibiting = (*j).second;
    //         for (NBConnectionVector::const_iterator k = prohibiting.begin(); k != prohibiting.end(); k++) {
    //             NBConnection prohibitor = *k;
    //             if (ec.retrieve(prohibitor.getFrom()->getID()) == nullptr
    //                     || ec.retrieve(prohibitor.getTo()->getID()) == nullptr) {
    //                 continue;
    //             }
    //             into.openTag(SUMO_TAG_PROHIBITION);
    //             into.writeAttr(SUMO_ATTR_PROHIBITOR, prohibitionConnection(prohibitor));
    //             into.writeAttr(SUMO_ATTR_PROHIBITED, prohibitionConnection(prohibited));
    //             into.closeTag();
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_prohibitions(into: &mut OutputDevice, prohibitions: &NBConnectionProhibits, ec: &NBEdgeCont) {
        // the edges may have been erased from NBEdgeCont but their pointers are still valid so we need to check
        for (prohibited, prohibiting) in prohibitions.iter() {
            if ec.retrieve(&prohibited.get_from().get_id(), false).is_null() || ec.retrieve(&prohibited.get_to().get_id(), false).is_null() {
                continue;
            }
            for prohibitor in prohibiting.iter() {
                if ec.retrieve(&prohibitor.get_from().get_id(), false).is_null() || ec.retrieve(&prohibitor.get_to().get_id(), false).is_null() {
                    continue;
                }
                into.open_tag_id(SUMO_TAG_PROHIBITION);
                into.write_attr(SUMO_ATTR_PROHIBITOR, Self::prohibition_connection(prohibitor).as_str());
                into.write_attr(SUMO_ATTR_PROHIBITED, Self::prohibition_connection(prohibited).as_str());
                into.close_tag("");
            }
        }
    }

    /// C++: `void NWWriter_SUMO::writeTrafficLights(OutputDevice &, const NBTrafficLightLogicCont &, ConnectionStyle)` [static]
    /// writes the traffic light logics to the given device
    // void
    // NWWriter_SUMO::writeTrafficLights(OutputDevice& into, const NBTrafficLightLogicCont& tllCont, ConnectionStyle style) {
    //     std::vector<NBTrafficLightLogic*> logics = tllCont.getComputed();
    //     for (NBTrafficLightLogic* logic : logics) {
    //         NBTrafficLightDefinition* def = tllCont.getDefinition(logic->getID(), logic->getProgramID());
    //         writeTrafficLight(into, logic, def, style);
    //         // only raise warnings on write instead of on compute (to avoid cluttering netedit)
    //         assert(def != nullptr);
    //         def->finalChecks();
    //     }
    //     if (logics.size() > 0) {
    //         into.lf();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_traffic_lights(into: &mut OutputDevice, tllCont: &NBTrafficLightLogicCont, style: NWWriter_SUMO_ConnectionStyle) {
        let logics = tllCont.get_computed();
        for logic in logics.iter() {
            let def = tllCont.get_definition(logic.base.get_id(), logic.get_program_id());
            Self::write_traffic_light(into, *logic, def, style);
            // only raise warnings on write instead of on compute (to avoid cluttering netedit)
            def.m().final_checks();
        }
        if logics.len() > 0 {
            into.lf();
        }
    }

    /// C++: `void NWWriter_SUMO::writeTrafficLight(OutputDevice &, const NBTrafficLightLogic *, const NBTrafficLightDefinition *, ConnectionStyle)` [static]
    /// writes a single traffic light logic to the given device
    // void
    // NWWriter_SUMO::writeTrafficLight(OutputDevice& into, const NBTrafficLightLogic* logic, const NBTrafficLightDefinition* def, ConnectionStyle style) {
    //     into.openTag(SUMO_TAG_TLLOGIC);
    //     into.writeAttr(SUMO_ATTR_ID, logic->getID());
    //     into.writeAttr(SUMO_ATTR_TYPE, logic->getType());
    //     into.writeAttr(SUMO_ATTR_PROGRAMID, logic->getProgramID());
    //     into.writeAttr(SUMO_ATTR_OFFSET, logic->getOffset() == SUMOTime_MAX ? "begin" : writeSUMOTime(logic->getOffset()));
    //     if (style == PLAIN_NAVTEQ) {
    //         into.writeAttr(SUMO_ATTR_NODES, toString(def->getNodes()));
    //         PositionVector nodePos;
    //         for (const NBNode* n : def->getNodes()) {
    //             Position pos = n->getPosition();
    //             GeoConvHelper::getFinal().cartesian2geo(pos);
    //             nodePos.push_back(pos);
    //         }
    //         into.setPrecision(gPrecisionGeo);
    //         into.writeAttr(SUMO_ATTR_POSITION, nodePos);
    //         into.setPrecision();
    //     }
    //     // write the phases
    //     const bool varPhaseLength = logic->getType() != TrafficLightType::STATIC;
    //     for (const NBTrafficLightLogic::PhaseDefinition& phase : logic->getPhases()) {
    //         into.openTag(SUMO_TAG_PHASE);
    //         into.writeAttr(SUMO_ATTR_DURATION, writeSUMOTime(phase.duration));
    //         if (phase.duration < TIME2STEPS(10)) {
    //             into.writePadding(" ");
    //         }
    //         into.writeAttr(SUMO_ATTR_STATE, phase.state);
    //         into.writeOptionalAttr(SUMO_ATTR_MINDURATION, writeSUMOTime(phase.minDur), !varPhaseLength || phase.minDur == NBTrafficLightDefinition::UNSPECIFIED_DURATION);
    //         into.writeOptionalAttr(SUMO_ATTR_MAXDURATION, writeSUMOTime(phase.maxDur), !varPhaseLength || phase.maxDur == NBTrafficLightDefinition::UNSPECIFIED_DURATION);
    //         into.writeOptionalAttr(SUMO_ATTR_EARLIEST_END, writeSUMOTime(phase.earliestEnd), !varPhaseLength || phase.earliestEnd == NBTrafficLightDefinition::UNSPECIFIED_DURATION);
    //         into.writeOptionalAttr(SUMO_ATTR_LATEST_END, writeSUMOTime(phase.latestEnd), !varPhaseLength || phase.latestEnd == NBTrafficLightDefinition::UNSPECIFIED_DURATION);
    //         // NEMA attributes
    //         into.writeOptionalAttr(SUMO_ATTR_VEHICLEEXTENSION, writeSUMOTime(phase.vehExt), !varPhaseLength || phase.vehExt == NBTrafficLightDefinition::UNSPECIFIED_DURATION);
    //         into.writeOptionalAttr(SUMO_ATTR_YELLOW, writeSUMOTime(phase.yellow), !varPhaseLength || phase.yellow == NBTrafficLightDefinition::UNSPECIFIED_DURATION);
    //         into.writeOptionalAttr(SUMO_ATTR_RED, writeSUMOTime(phase.red), !varPhaseLength || phase.red == NBTrafficLightDefinition::UNSPECIFIED_DURATION);
    //
    //         into.writeOptionalAttr(SUMO_ATTR_NAME, phase.name, phase.name == "", true);
    //         into.writeOptionalAttr(SUMO_ATTR_NEXT, phase.next, phase.next.empty());
    //         into.closeTag();
    //     }
    //     // write params
    //     logic->writeParams(into);
    //     into.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn write_traffic_light(into: &mut OutputDevice, logic: Ptr<NBTrafficLightLogic>, def: Ptr<NBTrafficLightDefinition>, style: NWWriter_SUMO_ConnectionStyle) {
        into.open_tag_id(SUMO_TAG_TLLOGIC);
        into.write_attr(SUMO_ATTR_ID, logic.base.get_id());
        into.write_attr(SUMO_ATTR_TYPE, &logic.get_type());
        into.write_attr(SUMO_ATTR_PROGRAMID, logic.get_program_id());
        let offset = logic.get_offset();
        let offsetStr = if offset == SUMOTIME_MAX { "begin".to_string() } else { Self::write_sumo_time(offset) };
        into.write_attr(SUMO_ATTR_OFFSET, offsetStr.as_str());
        if style == NWWriter_SUMO_ConnectionStyle::PLAIN_NAVTEQ {
            let nodeIDs: Vec<String> = def.get_nodes().iter().map(|n| n.get_id().to_string()).collect();
            into.write_attr(SUMO_ATTR_NODES, nodeIDs.join(" ").as_str());
            let mut nodePos = PositionVector::default();
            for n in def.get_nodes().iter() {
                let mut pos = *n.get_position();
                GeoConvHelper::with_final(|g| g.cartesian2geo(&mut pos));
                nodePos.push(pos);
            }
            into.set_precision(utils::common::std_defs::g_precision_geo());
            into.write_attr(SUMO_ATTR_POSITION, &nodePos);
            into.set_precision(utils::common::std_defs::g_precision());
        }
        // write the phases
        let varPhaseLength = logic.get_type() != TrafficLightType::STATIC;
        for phase in logic.get_phases().iter() {
            into.open_tag_id(SUMO_TAG_PHASE);
            into.write_attr(SUMO_ATTR_DURATION, Self::write_sumo_time(phase.duration).as_str());
            if phase.duration < time2steps(10.0) {
                into.write_padding(" ");
            }
            into.write_attr(SUMO_ATTR_STATE, phase.state.as_str());
            let unspec = NBTrafficLightDefinition_UNSPECIFIED_DURATION;
            into.write_optional_attr(SUMO_ATTR_MINDURATION, &Self::write_sumo_time(phase.minDur), !varPhaseLength || phase.minDur == unspec);
            into.write_optional_attr(SUMO_ATTR_MAXDURATION, &Self::write_sumo_time(phase.maxDur), !varPhaseLength || phase.maxDur == unspec);
            into.write_optional_attr(SUMO_ATTR_EARLIEST_END, &Self::write_sumo_time(phase.earliestEnd), !varPhaseLength || phase.earliestEnd == unspec);
            into.write_optional_attr(SUMO_ATTR_LATEST_END, &Self::write_sumo_time(phase.latestEnd), !varPhaseLength || phase.latestEnd == unspec);
            // NEMA attributes
            into.write_optional_attr(SUMO_ATTR_VEHICLEEXTENSION, &Self::write_sumo_time(phase.vehExt), !varPhaseLength || phase.vehExt == unspec);
            into.write_optional_attr(SUMO_ATTR_YELLOW, &Self::write_sumo_time(phase.yellow), !varPhaseLength || phase.yellow == unspec);
            into.write_optional_attr(SUMO_ATTR_RED, &Self::write_sumo_time(phase.red), !varPhaseLength || phase.red == unspec);

            if phase.name != "" {
                into.write_attr_escaped(SUMO_ATTR_NAME, &phase.name, true);
            }
            into.write_optional_attr(SUMO_ATTR_NEXT, &phase.next, phase.next.is_empty());
            into.close_tag("");
        }
        // write params
        logic.base2.write_params(into);
        into.close_tag("");
    }

    /// C++: `void NWWriter_SUMO::writeRoundabouts(OutputDevice &, const std::set<EdgeSet> &, const NBEdgeCont &)` [static]
    /// Writes roundabouts
    // void
    // NWWriter_SUMO::writeRoundabouts(OutputDevice& into, const std::set<EdgeSet>& roundabouts,
    //                                 const NBEdgeCont& ec) {
    //     //  make output deterministic
    //     std::vector<std::vector<std::string> > edgeIDs;
    //     for (std::set<EdgeSet>::const_iterator i = roundabouts.begin(); i != roundabouts.end(); ++i) {
    //         std::vector<std::string> tEdgeIDs;
    //         for (EdgeSet::const_iterator j = (*i).begin(); j != (*i).end(); ++j) {
    //             // the edges may have been erased from NBEdgeCont but their pointers are still valid
    //             // we verify their existance in writeRoundabout()
    //             tEdgeIDs.push_back((*j)->getID());
    //         }
    //         std::sort(tEdgeIDs.begin(), tEdgeIDs.end());
    //         edgeIDs.push_back(tEdgeIDs);
    //     }
    //     std::sort(edgeIDs.begin(), edgeIDs.end());
    //     //  write
    //     for (std::vector<std::vector<std::string> >::const_iterator i = edgeIDs.begin(); i != edgeIDs.end(); ++i) {
    //         writeRoundabout(into, *i, ec);
    //     }
    //     if (roundabouts.size() != 0) {
    //         into.lf();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_roundabouts(into: &mut OutputDevice, roundabouts: &BTreeSet<EdgeSet>, ec: &NBEdgeCont) {
        //  make output deterministic
        let mut edgeIDs: Vec<Vec<String>> = Vec::new();
        for r in roundabouts.iter() {
            let mut tEdgeIDs: Vec<String> = Vec::new();
            for j in r.iter() {
                // the edges may have been erased from NBEdgeCont but their pointers are still valid
                // we verify their existance in writeRoundabout()
                tEdgeIDs.push(j.get_id().to_string());
            }
            tEdgeIDs.sort();
            edgeIDs.push(tEdgeIDs);
        }
        edgeIDs.sort();
        //  write
        for i in edgeIDs.iter() {
            Self::write_roundabout(into, i, ec);
        }
        if roundabouts.len() != 0 {
            into.lf();
        }
    }

    /// C++: `void NWWriter_SUMO::writeStopOffsets(OutputDevice &, const StopOffset &)` [static]
    /// Write a stopOffset element into output device
    // void
    // NWWriter_SUMO::writeStopOffsets(OutputDevice& into, const StopOffset& stopOffset) {
    //     if (stopOffset.isDefined()) {
    //         const std::string ss_vclasses = getVehicleClassNames(stopOffset.getPermissions());
    //         if (ss_vclasses.length() == 0) {
    //             // This stopOffset would have no effect...
    //             return;
    //         }
    //         into.openTag(SUMO_TAG_STOPOFFSET);
    //         const std::string ss_exceptions = getVehicleClassNames(~stopOffset.getPermissions());
    //         if (ss_vclasses.length() <= ss_exceptions.length()) {
    //             into.writeAttr(SUMO_ATTR_VCLASSES, ss_vclasses);
    //         } else {
    //             if (ss_exceptions.length() == 0) {
    //                 into.writeAttr(SUMO_ATTR_VCLASSES, "all");
    //             } else {
    //                 into.writeAttr(SUMO_ATTR_EXCEPTIONS, ss_exceptions);
    //             }
    //         }
    //         into.writeAttr(SUMO_ATTR_VALUE, stopOffset.getOffset());
    //         into.closeTag();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_stop_offsets(into: &mut OutputDevice, stopOffset: &StopOffset) {
        if stopOffset.is_defined() {
            let ss_vclasses = get_vehicle_class_names(stopOffset.get_permissions(), false);
            if ss_vclasses.len() == 0 {
                // This stopOffset would have no effect...
                return;
            }
            into.open_tag_id(SUMO_TAG_STOPOFFSET);
            let ss_exceptions = get_vehicle_class_names(!stopOffset.get_permissions(), false);
            if ss_vclasses.len() <= ss_exceptions.len() {
                into.write_attr(SUMO_ATTR_VCLASSES, ss_vclasses.as_str());
            } else if ss_exceptions.len() == 0 {
                into.write_attr(SUMO_ATTR_VCLASSES, "all");
            } else {
                into.write_attr(SUMO_ATTR_EXCEPTIONS, ss_exceptions.as_str());
            }
            into.write_attr(SUMO_ATTR_VALUE, &stopOffset.get_offset());
            into.close_tag("");
        }
    }

    /// C++: `void NWWriter_SUMO::writeDistrict(OutputDevice &, const NBDistrict &)` [static]
    /// Writes a district
    // void
    // NWWriter_SUMO::writeDistrict(OutputDevice& into, const NBDistrict& d) {
    //     std::vector<double> sourceW = d.getSourceWeights();
    //     VectorHelper<double>::normaliseSum(sourceW, 1.0);
    //     std::vector<double> sinkW = d.getSinkWeights();
    //     VectorHelper<double>::normaliseSum(sinkW, 1.0);
    //     // write the head and the id of the district
    //     into.openTag(SUMO_TAG_TAZ).writeAttr(SUMO_ATTR_ID, d.getID());
    //     if (d.getShape().size() > 0) {
    //         into.writeAttr(SUMO_ATTR_SHAPE, d.getShape());
    //     }
    //     // write all sources
    //     const std::vector<NBEdge*>& sources = d.getSourceEdges();
    //     for (int i = 0; i < (int)sources.size(); i++) {
    //         // write the head and the id of the source
    //         into.openTag(SUMO_TAG_TAZSOURCE).writeAttr(SUMO_ATTR_ID, sources[i]->getID()).writeAttr(SUMO_ATTR_WEIGHT, sourceW[i]);
    //         into.closeTag();
    //     }
    //     // write all sinks
    //     const std::vector<NBEdge*>& sinks = d.getSinkEdges();
    //     for (int i = 0; i < (int)sinks.size(); i++) {
    //         // write the head and the id of the sink
    //         into.openTag(SUMO_TAG_TAZSINK).writeAttr(SUMO_ATTR_ID, sinks[i]->getID()).writeAttr(SUMO_ATTR_WEIGHT, sinkW[i]);
    //         into.closeTag();
    //     }
    //     // write the tail
    //     into.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn write_district(into: &mut OutputDevice, d: &NBDistrict) {
        let mut sourceW: Vec<f64> = d.get_source_weights().clone();
        normalise_sum(&mut sourceW, 1.0);
        let mut sinkW: Vec<f64> = d.get_sink_weights().clone();
        normalise_sum(&mut sinkW, 1.0);
        // write the head and the id of the district
        into.open_tag_id(SUMO_TAG_TAZ).write_attr(SUMO_ATTR_ID, d.base.get_id());
        if d.get_shape().len() > 0 {
            into.write_attr(SUMO_ATTR_SHAPE, d.get_shape());
        }
        // write all sources
        let sources = d.get_source_edges();
        for i in 0..sources.len() {
            // write the head and the id of the source
            into.open_tag_id(SUMO_TAG_TAZSOURCE).write_attr(SUMO_ATTR_ID, sources[i].get_id()).write_attr(SUMO_ATTR_WEIGHT, &sourceW[i]);
            into.close_tag("");
        }
        // write all sinks
        let sinks = d.get_sink_edges();
        for i in 0..sinks.len() {
            // write the head and the id of the sink
            into.open_tag_id(SUMO_TAG_TAZSINK).write_attr(SUMO_ATTR_ID, sinks[i].get_id()).write_attr(SUMO_ATTR_WEIGHT, &sinkW[i]);
            into.close_tag("");
        }
        // write the tail
        into.close_tag("");
    }

    /// C++: `void NWWriter_SUMO::writeShape(OutputDevice &, const GeoConvHelper &, PositionVector, SumoXMLAttr, bool, bool, bool)` [static]
    // void
    // NWWriter_SUMO::writeShape(OutputDevice& out, const GeoConvHelper& gch, PositionVector shape, SumoXMLAttr attr, bool useGeo, bool geoAccuracy, bool needsWrite) {
    //     if (!needsWrite) {
    //         out.writeOptionalAttr(attr, shape, true);
    //         return;
    //     }
    //     if (useGeo) {
    //         for (int i = 0; i < (int) shape.size(); i++) {
    //             gch.cartesian2geo(shape[i]);
    //         }
    //     }
    //     if (geoAccuracy) {
    //         out.setPrecision(gPrecisionGeo);
    //     }
    //     out.writeAttr(attr, shape);
    //     if (geoAccuracy) {
    //         out.setPrecision();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_shape(out: &mut OutputDevice, gch: &GeoConvHelper, shape: PositionVector, attr: SumoXMLAttr, useGeo: bool, geoAccuracy: bool, needsWrite: bool) {
        if !needsWrite {
            out.write_optional_attr(attr, &shape, true);
            return;
        }
        let mut shape = shape;
        if useGeo {
            for i in 0..shape.len() {
                gch.cartesian2geo(&mut shape[i]);
            }
        }
        if geoAccuracy {
            out.set_precision(utils::common::std_defs::g_precision_geo());
        }
        out.write_attr(attr, &shape);
        if geoAccuracy {
            out.set_precision(utils::common::std_defs::g_precision());
        }
    }

    /// C++: `bool NWWriter_SUMO::writeInternalEdges(OutputDevice &, const NBEdgeCont &, const NBNode &)` [static, private]
    /// Writes internal edges (<edge ... with id[0]==':') of the given node
    // bool
    // NWWriter_SUMO::writeInternalEdges(OutputDevice& into, const NBEdgeCont& ec, const NBNode& n) {
    //     bool ret = false;
    //     const EdgeVector& incoming = n.getIncomingEdges();
    //     // first pass: determine opposite internal edges and average their length
    //     std::map<std::string, std::string> oppositeLaneID;
    //     std::map<std::string, double> oppositeLengths;
    //     for (NBEdge* e : incoming) {
    //         for (const NBEdge::Connection& c : e->getConnections()) {
    //             double oppositeLength = 0;
    //             const std::string op = getOppositeInternalID(ec, e, c, oppositeLength);
    //             oppositeLaneID[c.getInternalLaneID()] = op;
    //             if (op != "") {
    //                 oppositeLengths[c.id] = oppositeLength;
    //             }
    //         }
    //     }
    //     if (oppositeLengths.size() > 0) {
    //         for (NBEdge* e : incoming) {
    //             for (NBEdge::Connection& c : e->getConnections()) {
    //                 if (oppositeLengths.count(c.id) > 0) {
    //                     c.length = (c.length + oppositeLengths[c.id]) / 2;
    //                 }
    //             }
    //         }
    //     }
    //
    //     for (NBEdge* e : incoming) {
    //         const std::vector<NBEdge::Connection>& elv = e->getConnections();
    //         if (elv.size() > 0) {
    //             bool haveVia = false;
    //             std::string edgeID = "";
    //             double bidiLength = -1;
    //             // second pass: write non-via edges
    //             for (const NBEdge::Connection& k : elv) {
    //                 if (k.toEdge == nullptr) {
    //                     assert(false); // should never happen. tell me when it does
    //                     continue;
    //                 }
    //                 if (edgeID != k.id) {
    //                     if (edgeID != "") {
    //                         // close the previous edge
    //                         into.closeTag();
    //                     }
    //                     edgeID = k.id;
    //                     into.openTag(SUMO_TAG_EDGE);
    //                     into.writeAttr(SUMO_ATTR_ID, edgeID);
    //                     into.writeAttr(SUMO_ATTR_FUNCTION, SumoXMLEdgeFunc::INTERNAL);
    //                     if (k.edgeType != "") {
    //                         into.writeAttr(SUMO_ATTR_TYPE, k.edgeType);
    //                     }
    //                     bidiLength = -1;
    //                     if (e->getBidiEdge() && k.toEdge->getBidiEdge() &&
    //                             e != k.toEdge->getTurnDestination(true)) {
    //                         const std::string bidiEdge = getInternalBidi(e, k, bidiLength);
    //                         if (bidiEdge != "") {
    //                             into.writeAttr(SUMO_ATTR_BIDI, bidiEdge);
    //                         }
    //                     }
    //                     // open a new edge
    //                 }
    //                 // to avoid changing to an internal lane which has a successor
    //                 // with the wrong permissions we need to inherit them from the successor
    //                 const NBEdge::Lane& successor = k.toEdge->getLanes()[k.toLane];
    //                 SVCPermissions permissions = (k.permissions != SVC_UNSPECIFIED) ? k.permissions : (
    //                                                  successor.permissions & e->getPermissions(k.fromLane));
    //                 SVCPermissions changeLeft = k.changeLeft != SVC_UNSPECIFIED ? k.changeLeft : SVCAll;
    //                 SVCPermissions changeRight = k.changeRight != SVC_UNSPECIFIED ? k.changeRight : SVCAll;
    //                 const double width = e->getInternalLaneWidth(n, k, successor, false);
    //                 const double length = bidiLength > 0 ? bidiLength : k.length;
    //                 writeLane(into, k.getInternalLaneID(), k.vmax, k.friction,
    //                           permissions, successor.preferred,
    //                           changeLeft, changeRight,
    //                           NBEdge::UNSPECIFIED_OFFSET, NBEdge::UNSPECIFIED_OFFSET,
    //                           StopOffset(), width, k.shape, &k,
    //                           length, k.internalLaneIndex, oppositeLaneID[k.getInternalLaneID()], "");
    //                 haveVia = haveVia || k.haveVia;
    //             }
    //             ret = true;
    //             into.closeTag(); // close the last edge
    //             // third pass: write via edges
    //             if (haveVia) {
    //                 std::string viaEdgeID = "";
    //                 for (const NBEdge::Connection& k : elv) {
    //                     if (!k.haveVia) {
    //                         continue;
    //                     }
    //                     if (k.toEdge == nullptr) {
    //                         assert(false); // should never happen. tell me when it does
    //                         continue;
    //                     }
    //                     if (viaEdgeID != k.viaID) {
    //                         if (viaEdgeID != "") {
    //                             // close the previous edge
    //                             into.closeTag();
    //                         }
    //                         viaEdgeID = k.viaID;
    //                         // open a new edge
    //                         into.openTag(SUMO_TAG_EDGE);
    //                         into.writeAttr(SUMO_ATTR_ID, viaEdgeID);
    //                         into.writeAttr(SUMO_ATTR_FUNCTION, SumoXMLEdgeFunc::INTERNAL);
    //                         if (k.edgeType != "") {
    //                             into.writeAttr(SUMO_ATTR_TYPE, k.edgeType);
    //                         }
    //                     }
    //                     const NBEdge::Lane& successor = k.toEdge->getLanes()[k.toLane];
    //                     SVCPermissions permissions = (k.permissions != SVC_UNSPECIFIED) ? k.permissions : (
    //                                                      successor.permissions & e->getPermissions(k.fromLane));
    //                     const double width = e->getInternalLaneWidth(n, k, successor, true);
    //                     writeLane(into, k.getInternalViaLaneID(), k.vmax, k.friction, permissions, successor.preferred,
    //                               SVCAll, SVCAll, // #XXX todo
    //                               NBEdge::UNSPECIFIED_OFFSET, NBEdge::UNSPECIFIED_OFFSET,
    //                               StopOffset(), width, k.viaShape, &k,
    //                               MAX2(k.viaLength, POSITION_EPS), // microsim needs positive length
    //                               k.internalViaLaneIndex, "", "");
    //                 }
    //                 into.closeTag();
    //             }
    //         }
    //     }
    //     // write pedestrian crossings
    //     const double crossingSpeed = OptionsCont::getOptions().getFloat("default.crossing-speed");
    //     for (const NBNode::Crossing* c : n.getCrossings()) {
    //         into.openTag(SUMO_TAG_EDGE).writeAttr(SUMO_ATTR_ID, c->id).writeAttr(SUMO_ATTR_FUNCTION, SumoXMLEdgeFunc::CROSSING);
    //         into.writeAttr(SUMO_ATTR_CROSSING_EDGES, c->edges);
    //         writeLane(into, c->id + "_0", crossingSpeed, NBEdge::UNSPECIFIED_FRICTION, SVC_PEDESTRIAN, 0, SVCAll, SVCAll,
    //                   NBEdge::UNSPECIFIED_OFFSET, NBEdge::UNSPECIFIED_OFFSET,
    //                   StopOffset(), c->width, c->shape, c,
    //                   MAX2(c->shape.length(), POSITION_EPS), 0, "", "", false, c->customShape.size() != 0, c->outlineShape);
    //         into.closeTag();
    //     }
    //     // write pedestrian walking areas
    //     const double walkingareaSpeed = OptionsCont::getOptions().getFloat("default.walkingarea-speed");
    //     for (const NBNode::WalkingArea& wa : n.getWalkingAreas()) {
    //         into.openTag(SUMO_TAG_EDGE).writeAttr(SUMO_ATTR_ID, wa.id).writeAttr(SUMO_ATTR_FUNCTION, SumoXMLEdgeFunc::WALKINGAREA);
    //         writeLane(into, wa.id + "_0", walkingareaSpeed, NBEdge::UNSPECIFIED_FRICTION, SVC_PEDESTRIAN, 0, SVCAll, SVCAll,
    //                   NBEdge::UNSPECIFIED_OFFSET, NBEdge::UNSPECIFIED_OFFSET,
    //                   StopOffset(), wa.width, wa.shape, nullptr, wa.length, 0, "", "", false, wa.hasCustomShape);
    //         into.closeTag();
    //     }
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn write_internal_edges(into: &mut OutputDevice, ec: &NBEdgeCont, n: &NBNode) -> bool {
        let mut ret = false;
        let incoming: Vec<Ptr<NBEdge>> = n.get_incoming_edges().clone();
        // first pass: determine opposite internal edges and average their length
        let mut oppositeLaneID: BTreeMap<String, String> = BTreeMap::new();
        let mut oppositeLengths: BTreeMap<String, f64> = BTreeMap::new();
        for e in incoming.iter() {
            for c in e.get_connections().iter() {
                let mut oppositeLength = 0.0;
                let op = Self::get_opposite_internal_id(ec, *e, c, &mut oppositeLength);
                oppositeLaneID.insert(c.get_internal_lane_id(), op.clone());
                if op != "" {
                    oppositeLengths.insert(c.id.clone(), oppositeLength);
                }
            }
        }
        if oppositeLengths.len() > 0 {
            for e in incoming.iter() {
                for c in e.m().myConnections.iter_mut() {
                    if let Some(ol) = oppositeLengths.get(&c.id) {
                        c.length = (c.length + *ol) / 2.0;
                    }
                }
            }
        }

        for e in incoming.iter() {
            let elv = e.get_connections();
            if elv.len() > 0 {
                let mut haveVia = false;
                let mut edgeID = String::new();
                let mut bidiLength = -1.0;
                // second pass: write non-via edges
                for k in elv.iter() {
                    if k.toEdge.is_null() {
                        continue; // should never happen
                    }
                    if edgeID != k.id {
                        if edgeID != "" {
                            // close the previous edge
                            into.close_tag("");
                        }
                        edgeID = k.id.clone();
                        into.open_tag_id(SUMO_TAG_EDGE);
                        into.write_attr(SUMO_ATTR_ID, edgeID.as_str());
                        into.write_attr(SUMO_ATTR_FUNCTION, &SumoXMLEdgeFunc::INTERNAL);
                        if k.edgeType != "" {
                            into.write_attr(SUMO_ATTR_TYPE, k.edgeType.as_str());
                        }
                        bidiLength = -1.0;
                        if !e.get_bidi_edge().is_null() && !k.toEdge.get_bidi_edge().is_null() && *e != k.toEdge.get_turn_destination(true) {
                            let bidiEdge = Self::get_internal_bidi(*e, k, &mut bidiLength);
                            if bidiEdge != "" {
                                into.write_attr(SUMO_ATTR_BIDI, bidiEdge.as_str());
                            }
                        }
                        // open a new edge
                    }
                    // to avoid changing to an internal lane which has a successor
                    // with the wrong permissions we need to inherit them from the successor
                    let successor = &k.toEdge.get_lanes()[k.toLane as usize];
                    let permissions = if k.permissions != SVC_UNSPECIFIED { k.permissions } else { successor.permissions & e.get_permissions(k.fromLane) };
                    let changeLeft = if k.changeLeft != SVC_UNSPECIFIED { k.changeLeft } else { SVC_ALL };
                    let changeRight = if k.changeRight != SVC_UNSPECIFIED { k.changeRight } else { SVC_ALL };
                    let width = e.get_internal_lane_width(n, k, successor, false);
                    let length = if bidiLength > 0.0 { bidiLength } else { k.length };
                    let opp = oppositeLaneID.get(&k.get_internal_lane_id()).cloned().unwrap_or_default();
                    Self::write_lane(
                        into,
                        &k.get_internal_lane_id(),
                        k.vmax,
                        k.friction,
                        permissions,
                        successor.preferred,
                        changeLeft,
                        changeRight,
                        NBEDGE_UNSPECIFIED_OFFSET,
                        NBEDGE_UNSPECIFIED_OFFSET,
                        &StopOffset::default(),
                        width,
                        k.shape.clone(),
                        Ptr::new(&k.base),
                        length,
                        k.internalLaneIndex,
                        &opp,
                        "",
                        false,
                        false,
                        &PositionVector::default(),
                    );
                    haveVia = haveVia || k.haveVia;
                }
                ret = true;
                into.close_tag(""); // close the last edge
                // third pass: write via edges
                if haveVia {
                    let mut viaEdgeID = String::new();
                    for k in elv.iter() {
                        if !k.haveVia {
                            continue;
                        }
                        if k.toEdge.is_null() {
                            continue; // should never happen
                        }
                        if viaEdgeID != k.viaID {
                            if viaEdgeID != "" {
                                // close the previous edge
                                into.close_tag("");
                            }
                            viaEdgeID = k.viaID.clone();
                            // open a new edge
                            into.open_tag_id(SUMO_TAG_EDGE);
                            into.write_attr(SUMO_ATTR_ID, viaEdgeID.as_str());
                            into.write_attr(SUMO_ATTR_FUNCTION, &SumoXMLEdgeFunc::INTERNAL);
                            if k.edgeType != "" {
                                into.write_attr(SUMO_ATTR_TYPE, k.edgeType.as_str());
                            }
                        }
                        let successor = &k.toEdge.get_lanes()[k.toLane as usize];
                        let permissions = if k.permissions != SVC_UNSPECIFIED { k.permissions } else { successor.permissions & e.get_permissions(k.fromLane) };
                        let width = e.get_internal_lane_width(n, k, successor, true);
                        Self::write_lane(
                            into,
                            &k.get_internal_via_lane_id(),
                            k.vmax,
                            k.friction,
                            permissions,
                            successor.preferred,
                            SVC_ALL,
                            SVC_ALL, // #XXX todo
                            NBEDGE_UNSPECIFIED_OFFSET,
                            NBEDGE_UNSPECIFIED_OFFSET,
                            &StopOffset::default(),
                            width,
                            k.viaShape.clone(),
                            Ptr::new(&k.base),
                            nw_max2(k.viaLength, POSITION_EPS), // microsim needs positive length
                            k.internalViaLaneIndex,
                            "",
                            "",
                            false,
                            false,
                            &PositionVector::default(),
                        );
                    }
                    into.close_tag("");
                }
            }
        }
        // write pedestrian crossings
        let crossingSpeed = utils::options::options_cont::get_float("default.crossing-speed");
        for c in n.get_crossings().iter() {
            into.open_tag_id(SUMO_TAG_EDGE).write_attr(SUMO_ATTR_ID, c.id.as_str()).write_attr(SUMO_ATTR_FUNCTION, &SumoXMLEdgeFunc::CROSSING);
            let edgeIDs: Vec<String> = c.edges.iter().map(|e| e.get_id().to_string()).collect();
            into.write_attr(SUMO_ATTR_CROSSING_EDGES, &edgeIDs);
            Self::write_lane(
                into,
                &format!("{}_0", c.id),
                crossingSpeed,
                NBEDGE_UNSPECIFIED_FRICTION,
                SVC_PEDESTRIAN,
                0,
                SVC_ALL,
                SVC_ALL,
                NBEDGE_UNSPECIFIED_OFFSET,
                NBEDGE_UNSPECIFIED_OFFSET,
                &StopOffset::default(),
                c.width,
                c.shape.clone(),
                Ptr::new(&c.base),
                nw_max2(c.shape.length(), POSITION_EPS),
                0,
                "",
                "",
                false,
                c.customShape.len() != 0,
                &c.outlineShape,
            );
            into.close_tag("");
        }
        // write pedestrian walking areas
        let walkingareaSpeed = utils::options::options_cont::get_float("default.walkingarea-speed");
        for wa in n.get_walking_areas().iter() {
            into.open_tag_id(SUMO_TAG_EDGE).write_attr(SUMO_ATTR_ID, wa.id.as_str()).write_attr(SUMO_ATTR_FUNCTION, &SumoXMLEdgeFunc::WALKINGAREA);
            Self::write_lane(
                into,
                &format!("{}_0", wa.id),
                walkingareaSpeed,
                NBEDGE_UNSPECIFIED_FRICTION,
                SVC_PEDESTRIAN,
                0,
                SVC_ALL,
                SVC_ALL,
                NBEDGE_UNSPECIFIED_OFFSET,
                NBEDGE_UNSPECIFIED_OFFSET,
                &StopOffset::default(),
                wa.width,
                wa.shape.clone(),
                Ptr::from_raw(std::ptr::null_mut()),
                wa.length,
                0,
                "",
                "",
                false,
                wa.hasCustomShape,
                &PositionVector::default(),
            );
            into.close_tag("");
        }
        ret
    }

    /// C++: `std::string NWWriter_SUMO::getInternalBidi(const NBEdge *, const NBEdge::Connection &, double &)` [static, private]
    /// retrieve bidi edge id for internal corresponding to the given connection
    // std::string
    // NWWriter_SUMO::getInternalBidi(const NBEdge* e, const NBEdge::Connection& k, double& length) {
    //     const NBEdge* fromBidi = e->getTurnDestination(true);
    //     const NBEdge* toBidi = k.toEdge->getTurnDestination(true);
    //     const std::vector<NBEdge::Connection> cons = toBidi->getConnectionsFromLane(-1, fromBidi, -1);
    //     if (cons.size() > 0) {
    //         if (e->getNumLanes() == 1 && k.toEdge->getNumLanes() == 1 && fromBidi->getNumLanes() == 1 && toBidi->getNumLanes() == 1) {
    //             length = (k.length + cons.back().length) / 2;
    //             return cons.back().id;
    //         }
    //         // do a more careful check in case there are parallel internal edges
    //         // note: k is the first connection with the new id
    //         for (const NBEdge::Connection& c : e->getConnections()) {
    //             if (c.id == k.id) {
    //                 PositionVector rShape = c.shape.reverse();
    //                 for (const NBEdge::Connection& k2 : cons) {
    //                     if (k2.shape.almostSame(rShape, POSITION_EPS)) {
    //                         length = (c.length + k2.length) / 2;
    //                         return k2.id;
    //                     }
    //                 }
    //             }
    //         }
    //     } else {
    //         WRITE_WARNINGF(TL("Could not find bidi-connection for edge '%'"), k.id)
    //     }
    //     return "";
    // }
    #[allow(unused_variables)]
    pub fn get_internal_bidi(e: Ptr<NBEdge>, k: &NBEdge_Connection, length: &mut f64) -> String {
        let fromBidi = e.get_turn_destination(true);
        let toBidi = k.toEdge.get_turn_destination(true);
        let cons = toBidi.get_connections_from_lane(-1, fromBidi, -1);
        if cons.len() > 0 {
            let last = cons.last().unwrap();
            if e.get_num_lanes() == 1 && k.toEdge.get_num_lanes() == 1 && fromBidi.get_num_lanes() == 1 && toBidi.get_num_lanes() == 1 {
                *length = (k.length + last.length) / 2.0;
                return last.id.clone();
            }
            // do a more careful check in case there are parallel internal edges
            // note: k is the first connection with the new id
            for c in e.get_connections().iter() {
                if c.id == k.id {
                    let rShape = c.shape.reverse();
                    for k2 in cons.iter() {
                        if k2.shape.almost_same(&rShape, POSITION_EPS) {
                            *length = (c.length + k2.length) / 2.0;
                            return k2.id.clone();
                        }
                    }
                }
            }
        } else {
            write_warningf!("Could not find bidi-connection for edge '%'", k.id.clone());
        }
        String::new()
    }

    /// C++: `void NWWriter_SUMO::writeEdge(OutputDevice &, const NBEdge &, bool, LaneSpreadFunction)` [static, private]
    /// Writes an edge (<edge ...)
    // void
    // NWWriter_SUMO::writeEdge(OutputDevice& into, const NBEdge& e, bool noNames, LaneSpreadFunction defaultSpread) {
    //     // write the edge's begin
    //     into.openTag(SUMO_TAG_EDGE).writeAttr(SUMO_ATTR_ID, e.getID());
    //     into.writeAttr(SUMO_ATTR_FROM, e.getFromNode()->getID());
    //     into.writeAttr(SUMO_ATTR_TO, e.getToNode()->getID());
    //     if (!noNames && e.getStreetName() != "") {
    //         into.writeAttr(SUMO_ATTR_NAME, StringUtils::escapeXML(e.getStreetName()));
    //     }
    //     into.writeAttr(SUMO_ATTR_PRIORITY, e.getPriority());
    //     if (e.getTypeID() != "") {
    //         into.writeAttr(SUMO_ATTR_TYPE, e.getTypeID());
    //     }
    //     if (e.getRoutingType() != "") {
    //         into.writeAttr(SUMO_ATTR_ROUTINGTYPE, e.getRoutingType());
    //     }
    //     if (e.isMacroscopicConnector()) {
    //         into.writeAttr(SUMO_ATTR_FUNCTION, SumoXMLEdgeFunc::CONNECTOR);
    //     }
    //     // write the spread type if not default ("right")
    //     if (e.getLaneSpreadFunction() != defaultSpread) {
    //         into.writeAttr(SUMO_ATTR_SPREADTYPE, e.getLaneSpreadFunction());
    //     }
    //     if (e.hasLoadedLength()) {
    //         into.writeAttr(SUMO_ATTR_LENGTH, e.getLoadedLength());
    //     }
    //     if (!e.hasDefaultGeometry()) {
    //         into.writeAttr(SUMO_ATTR_SHAPE, e.getGeometry());
    //     }
    //     if (e.getEdgeStopOffset().isDefined()) {
    //         writeStopOffsets(into, e.getEdgeStopOffset());
    //     }
    //     if (e.getBidiEdge()) {
    //         into.writeAttr(SUMO_ATTR_BIDI, e.getBidiEdge()->getID());
    //     }
    //     if (e.getDistance() != 0) {
    //         into.writeAttr(SUMO_ATTR_DISTANCE, e.getDistance());
    //     }
    //
    //     // write the lanes
    //     const std::vector<NBEdge::Lane>& lanes = e.getLanes();
    //
    //     double length = e.getFinalLength();
    //     if (e.getBidiEdge() != nullptr) {
    //         length = (length + e.getBidiEdge()->getFinalLength()) / 2;
    //     }
    //     double startOffset = e.isBidiRail() ? e.getTurnDestination(true)->getEndOffset() : 0;
    //     for (int i = 0; i < (int) lanes.size(); i++) {
    //         const NBEdge::Lane& l = lanes[i];
    //         StopOffset stopOffset;
    //         if (l.laneStopOffset != e.getEdgeStopOffset()) {
    //             stopOffset = l.laneStopOffset;
    //         }
    //         writeLane(into, e.getLaneID(i), l.speed, l.friction,
    //                   l.permissions, l.preferred,
    //                   l.changeLeft, l.changeRight,
    //                   startOffset, l.endOffset,
    //                   stopOffset, l.width, l.shape, &l,
    //                   length, i, l.oppositeID, l.type, l.accelRamp, l.customShape.size() > 0);
    //     }
    //     // close the edge
    //     e.writeParams(into);
    //     into.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn write_edge(into: &mut OutputDevice, e: &NBEdge, noNames: bool, defaultSpread: LaneSpreadFunction) {
        // write the edge's begin
        into.open_tag_id(SUMO_TAG_EDGE).write_attr(SUMO_ATTR_ID, e.get_id());
        into.write_attr(SUMO_ATTR_FROM, e.get_from_node().get_id());
        into.write_attr(SUMO_ATTR_TO, e.get_to_node().get_id());
        if !noNames && e.get_street_name() != "" {
            into.write_attr(SUMO_ATTR_NAME, utils::common::string_utils::escape_xml(e.get_street_name(), false).as_str());
        }
        into.write_attr(SUMO_ATTR_PRIORITY, &e.get_priority());
        if e.get_type_id() != "" {
            into.write_attr(SUMO_ATTR_TYPE, e.get_type_id());
        }
        if e.get_routing_type() != "" {
            into.write_attr(SUMO_ATTR_ROUTINGTYPE, e.get_routing_type());
        }
        if e.is_macroscopic_connector() {
            into.write_attr(SUMO_ATTR_FUNCTION, &SumoXMLEdgeFunc::CONNECTOR);
        }
        // write the spread type if not default ("right")
        if e.get_lane_spread_function() != defaultSpread {
            into.write_attr(SUMO_ATTR_SPREADTYPE, &e.get_lane_spread_function());
        }
        if e.has_loaded_length() {
            into.write_attr(SUMO_ATTR_LENGTH, &e.get_loaded_length());
        }
        if !e.has_default_geometry() {
            into.write_attr(SUMO_ATTR_SHAPE, e.get_geometry());
        }
        if e.get_edge_stop_offset().is_defined() {
            Self::write_stop_offsets(into, e.get_edge_stop_offset());
        }
        if !e.get_bidi_edge().is_null() {
            into.write_attr(SUMO_ATTR_BIDI, e.get_bidi_edge().get_id());
        }
        if e.get_distance() != 0.0 {
            into.write_attr(SUMO_ATTR_DISTANCE, &e.get_distance());
        }

        // write the lanes
        let lanes = e.get_lanes();
        let mut length = e.get_final_length();
        if !e.get_bidi_edge().is_null() {
            length = (length + e.get_bidi_edge().get_final_length()) / 2.0;
        }
        let startOffset = if e.is_bidi_rail(false) { e.get_turn_destination(true).get_end_offset() } else { 0.0 };
        for i in 0..lanes.len() {
            let l = &lanes[i];
            let mut stopOffset = StopOffset::default();
            if l.laneStopOffset != *e.get_edge_stop_offset() {
                stopOffset = l.laneStopOffset;
            }
            Self::write_lane(
                into,
                &e.get_lane_id(i as i32),
                l.speed,
                l.friction,
                l.permissions,
                l.preferred,
                l.changeLeft,
                l.changeRight,
                startOffset,
                l.endOffset,
                &stopOffset,
                l.width,
                l.shape.clone(),
                Ptr::new(&l.base),
                length,
                i as i32,
                &l.oppositeID,
                &l.type_,
                l.accelRamp,
                l.customShape.len() > 0,
                &PositionVector::default(),
            );
        }
        // close the edge
        e.base2.write_params(into);
        into.close_tag("");
    }

    /// C++: `void NWWriter_SUMO::writeLane(OutputDevice &, const std::string &, double, double, SVCPermissions, SVCPermissions, SVCPermissions, SVCPermissions, double, double, const StopOffset &, double, PositionVector, const Parameterised *, double, int, const std::string &, const std::string &, bool, bool, const PositionVector &)` [static, private]
    /// Writes a lane (<lane ...) of an edge
    // void
    // NWWriter_SUMO::writeLane(OutputDevice& into, const std::string& lID,
    //                          double speed, double friction,
    //                          SVCPermissions permissions, SVCPermissions preferred,
    //                          SVCPermissions changeLeft, SVCPermissions changeRight,
    //                          double startOffset, double endOffset,
    //                          const StopOffset& stopOffset, double width, PositionVector shape,
    //                          const Parameterised* params, double length, int index,
    //                          const std::string& oppositeID,
    //                          const std::string& type,
    //                          bool accelRamp, bool customShape,
    //                          const PositionVector& outlineShape) {
    //     // output the lane's attributes
    //     into.openTag(SUMO_TAG_LANE).writeAttr(SUMO_ATTR_ID, lID);
    //     // the first lane of an edge will be the depart lane
    //     into.writeAttr(SUMO_ATTR_INDEX, index);
    //     // write the list of allowed/disallowed vehicle classes
    //     if (permissions != SVC_UNSPECIFIED) {
    //         writePermissions(into, permissions);
    //     }
    //     writePreferences(into, preferred);
    //     // some further information
    //     into.writeAttr(SUMO_ATTR_SPEED, MAX2(0.0, speed));
    //     if (friction != NBEdge::UNSPECIFIED_FRICTION) {
    //         into.writeAttr(SUMO_ATTR_FRICTION, friction);
    //     }
    //     into.writeAttr(SUMO_ATTR_LENGTH, length);
    //     if (endOffset != NBEdge::UNSPECIFIED_OFFSET) {
    //         into.writeAttr(SUMO_ATTR_ENDOFFSET, endOffset);
    //     }
    //     if (width != NBEdge::UNSPECIFIED_WIDTH) {
    //         into.writeAttr(SUMO_ATTR_WIDTH, width);
    //     }
    //     if (accelRamp) {
    //         into.writeAttr<bool>(SUMO_ATTR_ACCELERATION, accelRamp);
    //     }
    //     if (customShape) {
    //         into.writeAttr(SUMO_ATTR_CUSTOMSHAPE, true);
    //     }
    //     if (endOffset > 0 || startOffset > 0) {
    //         startOffset = MIN2(startOffset, shape.length() - POSITION_EPS);
    //         endOffset = MIN2(endOffset, shape.length() - startOffset - POSITION_EPS);
    //         assert(startOffset + endOffset < shape.length());
    //         shape = shape.getSubpart(startOffset, shape.length() - endOffset);
    //     }
    //     into.writeAttr(SUMO_ATTR_SHAPE, shape);
    //     if (type != "") {
    //         into.writeAttr(SUMO_ATTR_TYPE, type);
    //     }
    //     if (changeLeft != SVC_UNSPECIFIED && changeLeft != SVCAll && changeLeft != SVC_IGNORING) {
    //         into.writeAttr(SUMO_ATTR_CHANGE_LEFT, getVehicleClassNames(changeLeft));
    //     }
    //     if (changeRight != SVC_UNSPECIFIED && changeRight != SVCAll && changeRight != SVC_IGNORING) {
    //         into.writeAttr(SUMO_ATTR_CHANGE_RIGHT, getVehicleClassNames(changeRight));
    //     }
    //     if (stopOffset.isDefined()) {
    //         writeStopOffsets(into, stopOffset);
    //     }
    //     if (outlineShape.size() != 0) {
    //         into.writeAttr(SUMO_ATTR_OUTLINESHAPE, outlineShape);
    //     }
    //
    //     if (oppositeID != "" && oppositeID != "-") {
    //         into.openTag(SUMO_TAG_NEIGH);
    //         into.writeAttr(SUMO_ATTR_LANE, oppositeID);
    //         into.closeTag();
    //     }
    //
    //     if (params != nullptr) {
    //         params->writeParams(into);
    //     }
    //
    //     into.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn write_lane(into: &mut OutputDevice, lID: &str, speed: f64, friction: f64, permissions: SVCPermissions, preferred: SVCPermissions, changeLeft: SVCPermissions, changeRight: SVCPermissions, startOffset: f64, endOffset: f64, stopOffset: &StopOffset, width: f64, shape: PositionVector, params: Ptr<Parameterised>, length: f64, index: i32, oppositeID: &str, type_: &str, accelRamp: bool, customShape: bool, outlineShape: &PositionVector) {
        // output the lane's attributes
        into.open_tag_id(SUMO_TAG_LANE).write_attr(SUMO_ATTR_ID, lID);
        // the first lane of an edge will be the depart lane
        into.write_attr(SUMO_ATTR_INDEX, &index);
        // write the list of allowed/disallowed vehicle classes
        if permissions != SVC_UNSPECIFIED {
            write_permissions(into, permissions);
        }
        write_preferences(into, preferred);
        // some further information
        into.write_attr(SUMO_ATTR_SPEED, &nw_max2(0.0, speed));
        if friction != NBEDGE_UNSPECIFIED_FRICTION {
            into.write_attr(SUMO_ATTR_FRICTION, &friction);
        }
        into.write_attr(SUMO_ATTR_LENGTH, &length);
        if endOffset != NBEDGE_UNSPECIFIED_OFFSET {
            into.write_attr(SUMO_ATTR_ENDOFFSET, &endOffset);
        }
        if width != NBEDGE_UNSPECIFIED_WIDTH {
            into.write_attr(SUMO_ATTR_WIDTH, &width);
        }
        if accelRamp {
            into.write_attr(SUMO_ATTR_ACCELERATION, &true);
        }
        if customShape {
            into.write_attr(SUMO_ATTR_CUSTOMSHAPE, &true);
        }
        let mut startOffset = startOffset;
        let mut endOffset = endOffset;
        let mut shape = shape;
        if endOffset > 0.0 || startOffset > 0.0 {
            startOffset = nw_min2(startOffset, shape.length() - POSITION_EPS);
            endOffset = nw_min2(endOffset, shape.length() - startOffset - POSITION_EPS);
            shape = shape.get_subpart(startOffset, shape.length() - endOffset);
        }
        into.write_attr(SUMO_ATTR_SHAPE, &shape);
        if type_ != "" {
            into.write_attr(SUMO_ATTR_TYPE, type_);
        }
        if changeLeft != SVC_UNSPECIFIED && changeLeft != SVC_ALL && changeLeft != SVC_IGNORING {
            into.write_attr(SUMO_ATTR_CHANGE_LEFT, get_vehicle_class_names(changeLeft, false).as_str());
        }
        if changeRight != SVC_UNSPECIFIED && changeRight != SVC_ALL && changeRight != SVC_IGNORING {
            into.write_attr(SUMO_ATTR_CHANGE_RIGHT, get_vehicle_class_names(changeRight, false).as_str());
        }
        if stopOffset.is_defined() {
            Self::write_stop_offsets(into, stopOffset);
        }
        if outlineShape.len() != 0 {
            into.write_attr(SUMO_ATTR_OUTLINESHAPE, outlineShape);
        }

        if oppositeID != "" && oppositeID != "-" {
            into.open_tag_id(SUMO_TAG_NEIGH);
            into.write_attr(SUMO_ATTR_LANE, oppositeID);
            into.close_tag("");
        }

        if let Some(p) = params.get() {
            p.write_params(into);
        }

        into.close_tag("");
    }

    /// C++: `void NWWriter_SUMO::writeJunction(OutputDevice &, const NBNode &)` [static, private]
    /// Writes a junction (<junction ...)
    // void
    // NWWriter_SUMO::writeJunction(OutputDevice& into, const NBNode& n) {
    //     // write the attributes
    //     into.openTag(SUMO_TAG_JUNCTION).writeAttr(SUMO_ATTR_ID, n.getID());
    //     into.writeAttr(SUMO_ATTR_TYPE, n.getType());
    //     NWFrame::writePositionLong(n.getPosition(), into);
    //     // write the incoming lanes
    //     std::vector<std::string> incLanes;
    //     const std::vector<NBEdge*>& incoming = n.getIncomingEdges();
    //     for (std::vector<NBEdge*>::const_iterator i = incoming.begin(); i != incoming.end(); ++i) {
    //         int noLanes = (*i)->getNumLanes();
    //         for (int j = 0; j < noLanes; j++) {
    //             incLanes.push_back((*i)->getLaneID(j));
    //         }
    //     }
    //     std::vector<NBNode::Crossing*> crossings = n.getCrossings();
    //     std::set<std::string> prevWAs;
    //     // avoid duplicates
    //     for (auto c : crossings) {
    //         if (prevWAs.count(c->prevWalkingArea) == 0) {
    //             incLanes.push_back(c->prevWalkingArea + "_0");
    //             prevWAs.insert(c->prevWalkingArea);
    //         }
    //     }
    //     into.writeAttr(SUMO_ATTR_INCLANES, incLanes);
    //     // write the internal lanes
    //     std::vector<std::string> intLanes;
    //     if (!OptionsCont::getOptions().getBool("no-internal-links")) {
    //         for (EdgeVector::const_iterator i = incoming.begin(); i != incoming.end(); i++) {
    //             const std::vector<NBEdge::Connection>& elv = (*i)->getConnections();
    //             for (std::vector<NBEdge::Connection>::const_iterator k = elv.begin(); k != elv.end(); ++k) {
    //                 if ((*k).toEdge == nullptr) {
    //                     continue;
    //                 }
    //                 if (!(*k).haveVia) {
    //                     intLanes.push_back((*k).getInternalLaneID());
    //                 } else {
    //                     intLanes.push_back((*k).getInternalViaLaneID());
    //                 }
    //             }
    //         }
    //     }
    //     if (n.getType() != SumoXMLNodeType::DEAD_END && n.getType() != SumoXMLNodeType::NOJUNCTION) {
    //         for (auto c : crossings) {
    //             intLanes.push_back(c->id + "_0");
    //         }
    //     }
    //     into.writeAttr(SUMO_ATTR_INTLANES, intLanes);
    //     // close writing
    //     into.writeAttr(SUMO_ATTR_SHAPE, n.getShape().simplified());
    //     // write optional radius
    //     if (n.getRadius() != NBNode::UNSPECIFIED_RADIUS) {
    //         into.writeAttr(SUMO_ATTR_RADIUS, n.getRadius());
    //     }
    //     // specify whether a custom shape was used
    //     if (n.hasCustomShape()) {
    //         into.writeAttr(SUMO_ATTR_CUSTOMSHAPE, true);
    //     }
    //     if (n.getRightOfWay() != RightOfWay::DEFAULT) {
    //         into.writeAttr<std::string>(SUMO_ATTR_RIGHT_OF_WAY, toString(n.getRightOfWay()));
    //     }
    //     if (n.getFringeType() != FringeType::DEFAULT) {
    //         into.writeAttr<std::string>(SUMO_ATTR_FRINGE, toString(n.getFringeType()));
    //     }
    //     if (n.getRoundaboutType() != RoundaboutType::DEFAULT) {
    //         into.writeAttr<std::string>(SUMO_ATTR_ROUNDABOUT, toString(n.getRoundaboutType()));
    //     }
    //     if (n.getName() != "") {
    //         into.writeAttr<std::string>(SUMO_ATTR_NAME, StringUtils::escapeXML(n.getName()));
    //     }
    //     if (n.getType() != SumoXMLNodeType::DEAD_END) {
    //         // write right-of-way logics
    //         n.writeLogic(into);
    //     }
    //     n.writeParams(into);
    //     into.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn write_junction(into: &mut OutputDevice, n: &NBNode) {
        // write the attributes
        into.open_tag_id(SUMO_TAG_JUNCTION).write_attr(SUMO_ATTR_ID, n.base.get_id());
        into.write_attr(SUMO_ATTR_TYPE, &n.get_type());
        NWFrame::write_position_long(n.get_position(), into);
        // write the incoming lanes
        let mut incLanes: Vec<String> = Vec::new();
        let incoming = n.get_incoming_edges();
        for i in incoming.iter() {
            let noLanes = i.get_num_lanes();
            for j in 0..noLanes {
                incLanes.push(i.get_lane_id(j));
            }
        }
        let crossings = n.get_crossings();
        let mut prevWAs: BTreeSet<String> = BTreeSet::new();
        // avoid duplicates
        for c in crossings.iter() {
            if !prevWAs.contains(&c.prevWalkingArea) {
                incLanes.push(format!("{}_0", c.prevWalkingArea));
                prevWAs.insert(c.prevWalkingArea.clone());
            }
        }
        into.write_attr(SUMO_ATTR_INCLANES, &incLanes);
        // write the internal lanes
        let mut intLanes: Vec<String> = Vec::new();
        if !utils::options::options_cont::get_bool("no-internal-links") {
            for i in incoming.iter() {
                for k in i.get_connections().iter() {
                    if k.toEdge.is_null() {
                        continue;
                    }
                    if !k.haveVia {
                        intLanes.push(k.get_internal_lane_id());
                    } else {
                        intLanes.push(k.get_internal_via_lane_id());
                    }
                }
            }
        }
        if n.get_type() != SumoXMLNodeType::DEAD_END && n.get_type() != SumoXMLNodeType::NOJUNCTION {
            for c in crossings.iter() {
                intLanes.push(format!("{}_0", c.id));
            }
        }
        into.write_attr(SUMO_ATTR_INTLANES, &intLanes);
        // close writing
        into.write_attr(SUMO_ATTR_SHAPE, &n.get_shape().simplified());
        // write optional radius
        if n.get_radius() != NBNODE_UNSPECIFIED_RADIUS {
            into.write_attr(SUMO_ATTR_RADIUS, &n.get_radius());
        }
        // specify whether a custom shape was used
        if n.has_custom_shape() {
            into.write_attr(SUMO_ATTR_CUSTOMSHAPE, &true);
        }
        if n.get_right_of_way() != RightOfWay::DEFAULT {
            into.write_attr(SUMO_ATTR_RIGHT_OF_WAY, &n.get_right_of_way());
        }
        if n.get_fringe_type() != FringeType::DEFAULT {
            into.write_attr(SUMO_ATTR_FRINGE, &n.get_fringe_type());
        }
        if n.get_roundabout_type() != RoundaboutType::DEFAULT {
            into.write_attr(SUMO_ATTR_ROUNDABOUT, &n.get_roundabout_type());
        }
        if n.get_name() != "" {
            into.write_attr(SUMO_ATTR_NAME, utils::common::string_utils::escape_xml(n.get_name(), false).as_str());
        }
        if n.get_type() != SumoXMLNodeType::DEAD_END {
            // write right-of-way logics
            n.write_logic(into);
        }
        n.base2.write_params(into);
        into.close_tag("");
    }

    /// C++: `bool NWWriter_SUMO::writeInternalNodes(OutputDevice &, const NBNode &)` [static, private]
    /// Writes internal junctions (<junction with id[0]==':' ...) of the given node
    // bool
    // NWWriter_SUMO::writeInternalNodes(OutputDevice& into, const NBNode& n) {
    //     bool ret = false;
    //     const std::vector<NBEdge*>& incoming = n.getIncomingEdges();
    //     // build the list of internal lane ids
    //     std::vector<std::string> internalLaneIDs;
    //     std::map<std::string, std::string> viaIDs;
    //     for (const NBEdge* in : incoming) {
    //         for (const auto& con : in->getConnections()) {
    //             if (con.toEdge != nullptr) {
    //                 internalLaneIDs.push_back(con.getInternalLaneID());
    //                 if (con.viaID != "") {
    //                     viaIDs[con.getInternalLaneID()] = (con.getInternalViaLaneID());
    //                 }
    //             }
    //         }
    //     }
    //     for (auto c : n.getCrossings()) {
    //         internalLaneIDs.push_back(c->id + "_0");
    //     }
    //     // write the internal nodes
    //     for (const NBEdge* in : incoming) {
    //         for (const auto& con : in->getConnections()) {
    //             if (con.toEdge == nullptr || !con.haveVia) {
    //                 continue;
    //             }
    //             Position pos = con.shape[-1];
    //             into.openTag(SUMO_TAG_JUNCTION).writeAttr(SUMO_ATTR_ID, con.getInternalViaLaneID());
    //             into.writeAttr(SUMO_ATTR_TYPE, SumoXMLNodeType::INTERNAL);
    //             NWFrame::writePositionLong(pos, into);
    //             std::string incLanes = con.getInternalLaneID();
    //             std::vector<std::string> foeIDs;
    //             for (std::string incLane : con.foeIncomingLanes) {
    //                 if (incLane[0] == ':') {
    //                     // intersecting left turns
    //                     const int index = StringUtils::toInt(incLane.substr(1));
    //                     incLane = internalLaneIDs[index];
    //                     if (viaIDs[incLane] != "") {
    //                         foeIDs.push_back(viaIDs[incLane]);
    //                     }
    //                 }
    //                 incLanes += " " + incLane;
    //             }
    //             into.writeAttr(SUMO_ATTR_INCLANES, incLanes);
    //             const std::vector<int>& foes = con.foeInternalLinks;
    //             for (int foe : foes) {
    //                 foeIDs.push_back(internalLaneIDs[foe]);
    //             }
    //             into.writeAttr(SUMO_ATTR_INTLANES, joinToString(foeIDs, " "));
    //             into.closeTag();
    //             ret = true;
    //         }
    //     }
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn write_internal_nodes(into: &mut OutputDevice, n: &NBNode) -> bool {
        let mut ret = false;
        let incoming = n.get_incoming_edges();
        // build the list of internal lane ids
        let mut internalLaneIDs: Vec<String> = Vec::new();
        let mut viaIDs: BTreeMap<String, String> = BTreeMap::new();
        for inE in incoming.iter() {
            for con in inE.get_connections().iter() {
                if !con.toEdge.is_null() {
                    internalLaneIDs.push(con.get_internal_lane_id());
                    if con.viaID != "" {
                        viaIDs.insert(con.get_internal_lane_id(), con.get_internal_via_lane_id());
                    }
                }
            }
        }
        for c in n.get_crossings().iter() {
            internalLaneIDs.push(format!("{}_0", c.id));
        }
        // write the internal nodes
        for inE in incoming.iter() {
            for con in inE.get_connections().iter() {
                if con.toEdge.is_null() || !con.haveVia {
                    continue;
                }
                let pos = *con.shape.at(-1);
                into.open_tag_id(SUMO_TAG_JUNCTION).write_attr(SUMO_ATTR_ID, con.get_internal_via_lane_id().as_str());
                into.write_attr(SUMO_ATTR_TYPE, &SumoXMLNodeType::INTERNAL);
                NWFrame::write_position_long(&pos, into);
                let mut incLanes: String = con.get_internal_lane_id();
                let mut foeIDs: Vec<String> = Vec::new();
                for incLane0 in con.foeIncomingLanes.iter() {
                    let mut incLane = incLane0.clone();
                    if incLane.starts_with(':') {
                        // intersecting left turns
                        let index = utils::common::string_utils::to_int(&incLane[1..]).unwrap() as usize;
                        incLane = internalLaneIDs[index].clone();
                        let via = viaIDs.get(&incLane).cloned().unwrap_or_default();
                        if via != "" {
                            foeIDs.push(via);
                        }
                    }
                    incLanes.push(' ');
                    incLanes.push_str(&incLane);
                }
                into.write_attr(SUMO_ATTR_INCLANES, incLanes.as_str());
                for foe in con.foeInternalLinks.iter() {
                    foeIDs.push(internalLaneIDs[*foe as usize].clone());
                }
                into.write_attr(SUMO_ATTR_INTLANES, foeIDs.join(" ").as_str());
                into.close_tag("");
                ret = true;
            }
        }
        ret
    }

    /// C++: `bool NWWriter_SUMO::writeInternalConnections(OutputDevice &, const NBNode &)` [static, private]
    /// Writes inner connections within the node
    // bool
    // NWWriter_SUMO::writeInternalConnections(OutputDevice& into, const NBNode& n) {
    //     bool ret = false;
    //     const bool lefthand = OptionsCont::getOptions().getBool("lefthand");
    //     for (const NBEdge* const from : n.getIncomingEdges()) {
    //         for (const NBEdge::Connection& c : from->getConnections()) {
    //             LinkDirection dir = n.getDirection(from, c.toEdge, lefthand);
    //             assert(c.toEdge != 0);
    //             if (c.haveVia) {
    //                 // internal split with optional signal
    //                 std::string tlID = "";
    //                 int linkIndex2 = NBConnection::InvalidTlIndex;
    //                 if (c.tlLinkIndex2 != NBConnection::InvalidTlIndex) {
    //                     linkIndex2 = c.tlLinkIndex2;
    //                     tlID = c.tlID;
    //                 }
    //                 writeInternalConnection(into, c.id, c.toEdge->getID(), c.internalLaneIndex, c.toLane, c.getInternalViaLaneID(), dir, tlID, linkIndex2, false, c.visibility);
    //                 writeInternalConnection(into, c.viaID, c.toEdge->getID(), c.internalViaLaneIndex, c.toLane, "", dir, "", NBConnection::InvalidTlIndex,
    //                                         n.brakeForCrossingOnExit(c.toEdge, dir, c.indirectLeft));
    //             } else {
    //                 // no internal split
    //                 writeInternalConnection(into, c.id, c.toEdge->getID(), c.internalLaneIndex, c.toLane, "", dir);
    //             }
    //             ret = true;
    //         }
    //     }
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn write_internal_connections(into: &mut OutputDevice, n: &NBNode) -> bool {
        let mut ret = false;
        let lefthand = utils::options::options_cont::get_bool("lefthand");
        for from in n.get_incoming_edges().iter() {
            for c in from.get_connections().iter() {
                let dir = n.get_direction(*from, c.toEdge, lefthand);
                if c.haveVia {
                    // internal split with optional signal
                    let mut tlID = String::new();
                    let mut linkIndex2 = NBCONNECTION_INVALID_TL_INDEX;
                    if c.tlLinkIndex2 != NBCONNECTION_INVALID_TL_INDEX {
                        linkIndex2 = c.tlLinkIndex2;
                        tlID = c.tlID.clone();
                    }
                    let via = c.get_internal_via_lane_id();
                    Self::write_internal_connection(into, &c.id, c.toEdge.get_id(), c.internalLaneIndex, c.toLane, &via, dir, &tlID, linkIndex2, false, c.visibility);
                    let minor = n.brake_for_crossing_on_exit(c.toEdge, dir, c.indirectLeft);
                    Self::write_internal_connection(into, &c.viaID, c.toEdge.get_id(), c.internalViaLaneIndex, c.toLane, "", dir, "", NBCONNECTION_INVALID_TL_INDEX, minor, NBEDGE_UNSPECIFIED_VISIBILITY_DISTANCE);
                } else {
                    // no internal split
                    Self::write_internal_connection(into, &c.id, c.toEdge.get_id(), c.internalLaneIndex, c.toLane, "", dir, "", NBCONNECTION_INVALID_TL_INDEX, false, NBEDGE_UNSPECIFIED_VISIBILITY_DISTANCE);
                }
                ret = true;
            }
        }
        ret
    }

    /// C++: `void NWWriter_SUMO::writeInternalConnection(OutputDevice &, const std::string &, const std::string &, int, int, const std::string &, LinkDirection, const std::string &, int, bool, double)` [static, private]
    /// Writes a single internal connection
    // void
    // NWWriter_SUMO::writeInternalConnection(OutputDevice& into,
    //                                        const std::string& from, const std::string& to,
    //                                        int fromLane, int toLane, const std::string& via,
    //                                        LinkDirection dir,
    //                                        const std::string& tlID, int linkIndex,
    //                                        bool minor,
    //                                        double visibility) {
    //     into.openTag(SUMO_TAG_CONNECTION);
    //     into.writeAttr(SUMO_ATTR_FROM, from);
    //     into.writeAttr(SUMO_ATTR_TO, to);
    //     into.writeAttr(SUMO_ATTR_FROM_LANE, fromLane);
    //     into.writeAttr(SUMO_ATTR_TO_LANE, toLane);
    //     if (via != "") {
    //         into.writeAttr(SUMO_ATTR_VIA, via);
    //     }
    //     if (tlID != "" && linkIndex != NBConnection::InvalidTlIndex) {
    //         // used for the reverse direction of pedestrian crossings
    //         into.writeAttr(SUMO_ATTR_TLID, tlID);
    //         into.writeAttr(SUMO_ATTR_TLLINKINDEX, linkIndex);
    //     }
    //     into.writeAttr(SUMO_ATTR_DIR, dir);
    //     into.writeAttr(SUMO_ATTR_STATE, ((via != "" || minor) ? "m" : "M"));
    //     if (visibility != NBEdge::UNSPECIFIED_VISIBILITY_DISTANCE) {
    //         into.writeAttr(SUMO_ATTR_VISIBILITY_DISTANCE, visibility);
    //     }
    //     into.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn write_internal_connection(into: &mut OutputDevice, from: &str, to: &str, fromLane: i32, toLane: i32, via: &str, dir: LinkDirection, tlID: &str, linkIndex: i32, minor: bool, visibility: f64) {
        into.open_tag_id(SUMO_TAG_CONNECTION);
        into.write_attr(SUMO_ATTR_FROM, from);
        into.write_attr(SUMO_ATTR_TO, to);
        into.write_attr(SUMO_ATTR_FROM_LANE, &fromLane);
        into.write_attr(SUMO_ATTR_TO_LANE, &toLane);
        if via != "" {
            into.write_attr(SUMO_ATTR_VIA, via);
        }
        if tlID != "" && linkIndex != NBCONNECTION_INVALID_TL_INDEX {
            // used for the reverse direction of pedestrian crossings
            into.write_attr(SUMO_ATTR_TLID, tlID);
            into.write_attr(SUMO_ATTR_TLLINKINDEX, &linkIndex);
        }
        into.write_attr(SUMO_ATTR_DIR, &dir);
        into.write_attr(SUMO_ATTR_STATE, if via != "" || minor { "m" } else { "M" });
        if visibility != NBEDGE_UNSPECIFIED_VISIBILITY_DISTANCE {
            into.write_attr(SUMO_ATTR_VISIBILITY_DISTANCE, &visibility);
        }
        into.close_tag("");
    }

    /// C++: `std::string NWWriter_SUMO::writeSUMOTime(SUMOTime)` [static, private]
    /// writes a SUMOTime as int if possible, otherwise as a float
    // std::string
    // NWWriter_SUMO::writeSUMOTime(SUMOTime steps) {
    //     double time = STEPS2TIME(steps);
    //     if (time == std::floor(time)) {
    //         return toString(int(time));
    //     } else {
    //         return toString(time);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_sumo_time(time: SUMOTime) -> String {
        let t = steps2time(time);
        if t == t.floor() {
            (t as i32).to_string()
        } else {
            to_str_f64(t)
        }
    }

    /// C++: `std::string NWWriter_SUMO::prohibitionConnection(const NBConnection &)` [static, private]
    /// the attribute value for a prohibition
    // std::string
    // NWWriter_SUMO::prohibitionConnection(const NBConnection& c) {
    //     return c.getFrom()->getID() + "->" + c.getTo()->getID();
    // }
    #[allow(unused_variables)]
    pub fn prohibition_connection(c: &NBConnection) -> String {
        format!("{}->{}", c.get_from().get_id(), c.get_to().get_id())
    }

    /// C++: `void NWWriter_SUMO::writeRoundabout(OutputDevice &, const std::vector<std::string> &, const NBEdgeCont &)` [static, private]
    /// Writes a roundabout
    // void
    // NWWriter_SUMO::writeRoundabout(OutputDevice& into, const std::vector<std::string>& edgeIDs,
    //                                const NBEdgeCont& ec) {
    //     std::vector<std::string> validEdgeIDs;
    //     std::vector<std::string> invalidEdgeIDs;
    //     std::vector<std::string> nodeIDs;
    //     for (std::vector<std::string>::const_iterator i = edgeIDs.begin(); i != edgeIDs.end(); ++i) {
    //         const NBEdge* edge = ec.retrieve(*i);
    //         if (edge != nullptr) {
    //             nodeIDs.push_back(edge->getToNode()->getID());
    //             validEdgeIDs.push_back(edge->getID());
    //         } else {
    //             invalidEdgeIDs.push_back(*i);
    //         }
    //     }
    //     std::sort(nodeIDs.begin(), nodeIDs.end());
    //     if (validEdgeIDs.size() > 0) {
    //         into.openTag(SUMO_TAG_ROUNDABOUT);
    //         into.writeAttr(SUMO_ATTR_NODES, joinToString(nodeIDs, " "));
    //         into.writeAttr(SUMO_ATTR_EDGES, joinToString(validEdgeIDs, " "));
    //         into.closeTag();
    //         if (invalidEdgeIDs.size() > 0) {
    //             WRITE_WARNING("Writing incomplete roundabout. Edges: '"
    //                           + joinToString(invalidEdgeIDs, " ") + "' no longer exist'");
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_roundabout(into: &mut OutputDevice, r: &Vec<String>, ec: &NBEdgeCont) {
        let mut validEdgeIDs: Vec<String> = Vec::new();
        let mut invalidEdgeIDs: Vec<String> = Vec::new();
        let mut nodeIDs: Vec<String> = Vec::new();
        for id in r.iter() {
            let edge = ec.retrieve(id, false);
            if !edge.is_null() {
                nodeIDs.push(edge.get_to_node().get_id().to_string());
                validEdgeIDs.push(edge.get_id().to_string());
            } else {
                invalidEdgeIDs.push(id.clone());
            }
        }
        nodeIDs.sort();
        if validEdgeIDs.len() > 0 {
            into.open_tag_id(SUMO_TAG_ROUNDABOUT);
            into.write_attr(SUMO_ATTR_NODES, nodeIDs.join(" ").as_str());
            into.write_attr(SUMO_ATTR_EDGES, validEdgeIDs.join(" ").as_str());
            into.close_tag("");
            if invalidEdgeIDs.len() > 0 {
                utils::common::msg_handler::write_warning(&format!("Writing incomplete roundabout. Edges: '{}' no longer exist'", invalidEdgeIDs.join(" ")));
            }
        }
    }

    /// C++: `std::string NWWriter_SUMO::getOppositeInternalID(const NBEdgeCont &, const NBEdge *, const NBEdge::Connection &, double &)` [static, private]
    /// retrieve the id of the opposite direction internal lane if it exists
    // std::string
    // NWWriter_SUMO::getOppositeInternalID(const NBEdgeCont& ec, const NBEdge* from, const NBEdge::Connection& con, double& oppositeLength) {
    //     const NBEdge::Lane& succ = con.toEdge->getLanes()[con.toLane];
    //     const NBEdge::Lane& pred = from->getLanes()[con.fromLane];
    //     const bool lefthand = OptionsCont::getOptions().getBool("lefthand");
    //     if (succ.oppositeID != "" && succ.oppositeID != "-" && pred.oppositeID != "" && pred.oppositeID != "-") {
    // #ifdef DEBUG_OPPOSITE_INTERNAL
    //         std::cout << "getOppositeInternalID con=" << con.getDescription(from) << " (" << con.getInternalLaneID() << ")\n";
    // #endif
    //         // find the connection that connects succ.oppositeID to pred.oppositeID
    //         const NBEdge* succOpp = ec.retrieve(succ.oppositeID.substr(0, succ.oppositeID.rfind("_")));
    //         const NBEdge* predOpp = ec.retrieve(pred.oppositeID.substr(0, pred.oppositeID.rfind("_")));
    //         assert(succOpp != 0);
    //         assert(predOpp != 0);
    //         const std::vector<NBEdge::Connection>& connections = succOpp->getConnections();
    //         for (std::vector<NBEdge::Connection>::const_iterator it_c = connections.begin(); it_c != connections.end(); it_c++) {
    //             const NBEdge::Connection& conOpp = *it_c;
    //             if (succOpp != from // turnaround
    //                     && predOpp == conOpp.toEdge
    //                     && succOpp->getLaneID(conOpp.fromLane) == succ.oppositeID
    //                     && predOpp->getLaneID(conOpp.toLane) == pred.oppositeID
    //                     && from->getToNode()->getDirection(from, con.toEdge, lefthand) == LinkDirection::STRAIGHT
    //                     && from->getToNode()->getDirection(succOpp, predOpp, lefthand) == LinkDirection::STRAIGHT
    //                ) {
    // #ifdef DEBUG_OPPOSITE_INTERNAL
    //                 std::cout << "  found " << conOpp.getInternalLaneID() << "\n";
    // #endif
    //                 oppositeLength = conOpp.length;
    //                 return conOpp.getInternalLaneID();
    //             } else {
    //                 /*
    //                 #ifdef DEBUG_OPPOSITE_INTERNAL
    //                 std::cout << "  rejected " << conOpp.getInternalLaneID()
    //                     << "\n     succ.oppositeID=" << succ.oppositeID
    //                     << "\n         succOppLane=" << succOpp->getLaneID(conOpp.fromLane)
    //                     << "\n     pred.oppositeID=" << pred.oppositeID
    //                     << "\n         predOppLane=" << predOpp->getLaneID(conOpp.toLane)
    //                     << "\n      predOpp=" << predOpp->getID()
    //                     << "\n     conOppTo=" << conOpp.toEdge->getID()
    //                     << "\n     len1=" << con.shape.length()
    //                     << "\n     len2=" << conOpp.shape.length()
    //                     << "\n";
    //                 #endif
    //                 */
    //             }
    //         }
    //         return "";
    //     } else {
    //         return "";
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_opposite_internal_id(ec: &NBEdgeCont, from: Ptr<NBEdge>, con: &NBEdge_Connection, oppositeLength: &mut f64) -> String {
        let succ = &con.toEdge.get_lanes()[con.toLane as usize];
        let pred = &from.get_lanes()[con.fromLane as usize];
        let lefthand = utils::options::options_cont::get_bool("lefthand");
        // C++ rfind() returns npos when absent, so substr(0, npos) keeps the whole string
        let strip_last = |s: &str| -> String {
            match s.rfind('_') {
                Some(p) => s[..p].to_string(),
                None => s.to_string(),
            }
        };
        if succ.oppositeID != "" && succ.oppositeID != "-" && pred.oppositeID != "" && pred.oppositeID != "-" {
            // find the connection that connects succ.oppositeID to pred.oppositeID
            let succOpp = ec.retrieve(&strip_last(&succ.oppositeID), false);
            let predOpp = ec.retrieve(&strip_last(&pred.oppositeID), false);
            for conOpp in succOpp.get_connections().iter() {
                if succOpp != from // turnaround
                    && predOpp == conOpp.toEdge
                    && succOpp.get_lane_id(conOpp.fromLane) == succ.oppositeID
                    && predOpp.get_lane_id(conOpp.toLane) == pred.oppositeID
                    && from.get_to_node().get_direction(from, con.toEdge, lefthand) == LinkDirection::STRAIGHT
                    && succOpp.get_to_node().get_direction(succOpp, predOpp, lefthand) == LinkDirection::STRAIGHT
                {
                    *oppositeLength = conOpp.length;
                    return conOpp.get_internal_lane_id();
                }
            }
            String::new()
        } else {
            String::new()
        }
    }

}


// ---- helpers for the port above (C++ free functions and constants used by NWWriter_SUMO.cpp) ----

const NBEDGE_UNSPECIFIED_WIDTH: f64 = -1.0;
const NBEDGE_UNSPECIFIED_OFFSET: f64 = 0.0;
const NBEDGE_UNSPECIFIED_SPEED: f64 = -1.0;
const NBEDGE_UNSPECIFIED_FRICTION: f64 = 1.0;
const NBEDGE_UNSPECIFIED_CONTPOS: f64 = -1.0;
const NBEDGE_UNSPECIFIED_VISIBILITY_DISTANCE: f64 = -1.0;
const NBEDGE_UNSPECIFIED_LOADED_LENGTH: f64 = -1.0;
const NBNODE_UNSPECIFIED_RADIUS: f64 = -1.0;
const NBCONNECTION_INVALID_TL_INDEX: i32 = -1;

/// toString(double) with the global precision (gPrecision)
fn to_str_f64(v: f64) -> String {
    utils::common::to_string::f64_to_string_prec(v, utils::common::std_defs::g_precision())
}

/// MIN2 / MAX2 of the C++ headers (a < b ? a : b, a > b ? a : b)
fn nw_min2(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}
fn nw_max2(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}

/// VectorHelper<double>::normaliseSum
fn normalise_sum(v: &mut Vec<f64>, msum: f64) {
    if msum == 0.0 || v.is_empty() {
        return;
    }
    let mut rsum = 0.0;
    for x in v.iter() {
        rsum += *x;
    }
    if rsum == 0.0 {
        let val = 1.0 * msum / (v.len() as f64);
        for x in v.iter_mut() {
            *x = val;
        }
        return;
    }
    let d = rsum / msum;
    for x in v.iter_mut() {
        *x /= d;
    }
}

/// writePreferences (SUMOVehicleClass.cpp)
fn write_preferences(into: &mut OutputDevice, preferred: SVCPermissions) {
    let is_null = preferred == SVC_ALL || preferred == 0;
    into.write_optional_attr(SUMO_ATTR_PREFER, get_vehicle_class_names(preferred, false).as_str(), is_null);
}

/// GeoConvHelper::writeLocation (utils/geom/GeoConvHelper.cpp), applied to the final helper
fn write_location(into: &mut OutputDevice) {
    GeoConvHelper::with_final(|f| {
        into.open_tag_id(SUMO_TAG_LOCATION);
        into.write_attr(SUMO_ATTR_NET_OFFSET, &PositionVector(vec![f.get_offset_base()]));
        let prec = into.get_precision();
        into.write_attr(SUMO_ATTR_CONV_BOUNDARY, f.get_conv_boundary().to_string_prec(prec).as_str());
        if f.using_geo_projection() {
            into.set_precision(utils::common::std_defs::g_precision_geo());
        }
        let prec = into.get_precision();
        into.write_attr(SUMO_ATTR_ORIG_BOUNDARY, f.get_orig_boundary().to_string_prec(prec).as_str());
        if f.using_geo_projection() {
            into.set_precision(utils::common::std_defs::g_precision());
        }
        into.write_attr(SUMO_ATTR_ORIG_PROJ, utils::common::string_utils::escape_xml(f.get_proj_string(), false).as_str());
        into.close_tag("");
        into.lf();
    });
}
