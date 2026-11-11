//! Port of netwrite/NWWriter_XML.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Exporter writing networks using XML (native input) format
/// C++ class `NWWriter_XML` (netwrite/NWWriter_XML.h)
pub struct NWWriter_XML {
}

impl NWWriter_XML {
    /// C++: `void NWWriter_XML::writeNetwork(const OptionsCont &, const std::string &, NBNetBuilder &)` [static]
    /// Writes the network into XML-files (nodes, edges, connections, traffic lights)
    // void
    // NWWriter_XML::writeNetwork(const OptionsCont& oc, const std::string& prefix, NBNetBuilder& nb) {
    //     // check whether plain-output files shall be generated
    //     if (prefix != "") {
    //         const bool haveTypes = nb.getTypeCont().size() > 0;
    //         writeNodes(oc, prefix, nb.getNodeCont());
    //         if (haveTypes) {
    //             writeTypes(prefix, nb.getEdgeCont(), nb.getTypeCont());
    //         }
    //         writeEdgesAndConnections(oc, prefix, nb.getNodeCont(), nb.getEdgeCont());
    //         writeTrafficLights(prefix, nb.getTLLogicCont(), nb.getEdgeCont());
    //         writeConfig(oc, prefix, haveTypes);
    //     }
    //     if (oc.isSet("junctions.join-output") && (oc.getString("junctions.join-output").size() > 0)) {
    //         writeJoinedJunctions(oc.getString("junctions.join-output"), nb.getNodeCont());
    //     }
    //     if (oc.isSet("street-sign-output") && (oc.getString("street-sign-output").size() > 0)) {
    //         writeStreetSigns(oc, nb.getEdgeCont());
    //     }
    //     if (oc.exists("ptstop-output") && oc.isSet("ptstop-output") && (oc.getString("ptstop-output").size() > 0)) {
    //         writePTStops(oc, nb.getPTStopCont());
    //     }
    //     if (oc.exists("ptline-output") && oc.isSet("ptline-output") && (oc.getString("ptline-output").size() > 0)) {
    //         writePTLines(oc, nb.getPTLineCont());
    //     }
    //     if (oc.exists("parking-output") && oc.isSet("parking-output") && (oc.getString("parking-output").size() > 0)) {
    //         writeParkingAreas(oc, nb.getParkingCont(), nb.getEdgeCont());
    //     }
    //     if (oc.exists("taz-output") && oc.isSet("taz-output") && (oc.getString("taz-output").size() > 0)) {
    //         writeDistricts(oc, nb.getDistrictCont());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_network(oc: &OptionsCont, prefix: &str, nb: &mut NBNetBuilder) {
        todo!("port NWWriter_XML::writeNetwork")
    }

    /// C++: `void NWWriter_XML::writeJoinedJunctions(const std::string &, NBNodeCont &)` [static]
    /// Writes the joined-juncionts to file
    // void
    // NWWriter_XML::writeJoinedJunctions(const std::string& filename, NBNodeCont& nc) {
    //     std::map<SumoXMLAttr, std::string> attrs;
    //     attrs[SUMO_ATTR_VERSION] = toString(NETWORK_VERSION);
    //     OutputDevice& device = OutputDevice::getDevice(filename);
    //     device.writeXMLHeader("nodes", "nodes_file.xsd", attrs);
    //     for (const std::set<std::string>& joined : nc.getJoinedClusters()) {
    //         assert(joined.size() > 0);
    //         device.openTag(SUMO_TAG_JOIN).writeAttr(SUMO_ATTR_NODES, joined).closeTag();
    //     }
    //     device.close();
    // }
    #[allow(unused_variables)]
    pub fn write_joined_junctions(filename: &str, nc: &mut NBNodeCont) {
        todo!("port NWWriter_XML::writeJoinedJunctions")
    }

    /// C++: `void NWWriter_XML::writeStreetSigns(const OptionsCont &, NBEdgeCont &)` [static]
    /// Writes street signs as POIs to file
    // void
    // NWWriter_XML::writeStreetSigns(const OptionsCont& oc, NBEdgeCont& ec) {
    //     OutputDevice& device = OutputDevice::getDevice(oc.getString("street-sign-output"));
    //     device.writeXMLHeader("additional", "additional_file.xsd");
    //     for (std::map<std::string, NBEdge*>::const_iterator i = ec.begin(); i != ec.end(); ++i) {
    //         NBEdge* e = (*i).second;
    //         const std::vector<NBSign>& signs =  e->getSigns();
    //         for (std::vector<NBSign>::const_iterator it = signs.begin(); it != signs.end(); ++it) {
    //             it->writeAsPOI(device, e);
    //         }
    //     }
    //     device.close();
    // }
    #[allow(unused_variables)]
    pub fn write_street_signs(oc: &OptionsCont, ec: &mut NBEdgeCont) {
        todo!("port NWWriter_XML::writeStreetSigns")
    }

    /// C++: `void NWWriter_XML::writeConfig(const OptionsCont &, const std::string &, bool)` [static, private]
    /// Writes the configuration file for assempling the network from plain-xml files
    // void
    // NWWriter_XML::writeConfig(const OptionsCont& oc, const std::string& prefix, bool haveTypes) {
    //     if (!oc.exists("node-files")) {
    //         // do not write configuration for netgen
    //         return;
    //     }
    //     const std::string ext = oc.getString("output.format");
    //     OptionsCont* tmp = oc.clone();
    //     tmp->set("node-files", prefix + ".nod." + ext);
    //     tmp->set("edge-files", prefix + ".edg." + ext);
    //     tmp->set("connection-files", prefix + ".con." + ext);
    //     tmp->set("tllogic-files", prefix + ".tll." + ext);
    //     if (haveTypes) {
    //         tmp->set("type-files", prefix + ".typ." + ext);
    //     }
    //     tmp->setDefault("sumo-net-file", "");
    //     tmp->setDefault("plain-output-prefix", "");
    //
    //     const std::string configPath = prefix + ".netccfg";
    //     std::ofstream out(configPath.c_str());
    //     if (!out.good()) {
    //         delete tmp;
    //         throw ProcessError(TLF("Could not save configuration to '%'", configPath));
    //     } else {
    //         tmp->writeConfiguration(out, true, false, false);
    //     }
    //     delete tmp;
    // }
    #[allow(unused_variables)]
    pub fn write_config(oc: &OptionsCont, prefix: &str, haveTypes: bool) {
        todo!("port NWWriter_XML::writeConfig")
    }

    /// C++: `void NWWriter_XML::writeNodes(const OptionsCont &, const std::string &, NBNodeCont &)` [static, private]
    /// Writes the nodes file
    // void
    // NWWriter_XML::writeNodes(const OptionsCont& oc, const std::string& prefix, NBNodeCont& nc) {
    //     const std::string ext = oc.getString("output.format");
    //     const GeoConvHelper& gch = GeoConvHelper::getFinal();
    //     bool useGeo = oc.exists("proj.plain-geo") && oc.getBool("proj.plain-geo");
    //     if (useGeo && !gch.usingGeoProjection()) {
    //         WRITE_WARNING(TL("Ignoring option \"proj.plain-geo\" because no geo-conversion has been defined"));
    //         useGeo = false;
    //     }
    //     const bool geoAccuracy = useGeo || gch.usingInverseGeoProjection();
    //
    //     OutputDevice& device = OutputDevice::getDevice(prefix + ".nod." + ext);
    //     std::map<SumoXMLAttr, std::string> attrs;
    //     attrs[SUMO_ATTR_VERSION] = toString(NETWORK_VERSION);
    //     device.writeXMLHeader("nodes", "nodes_file.xsd", attrs);
    //
    //     // write network offsets and projection to allow reconstruction of original coordinates
    //     if (!useGeo) {
    //         GeoConvHelper::writeLocation(device);
    //     }
    //
    //     // write nodes
    //     TrafficLightType tlsDefaultType = SUMOXMLDefinitions::TrafficLightTypes.get(oc.getString("tls.default-type"));
    //     for (std::map<std::string, NBNode*>::const_iterator i = nc.begin(); i != nc.end(); ++i) {
    //         NBNode* n = (*i).second;
    //         device.openTag(SUMO_TAG_NODE);
    //         device.writeAttr(SUMO_ATTR_ID, n->getID());
    //         // write position
    //         Position pos = n->getPosition();
    //         if (useGeo) {
    //             gch.cartesian2geo(pos);
    //         }
    //         if (geoAccuracy) {
    //             device.setPrecision(gPrecisionGeo);
    //         }
    //         NWFrame::writePositionLong(pos, device);
    //         if (geoAccuracy) {
    //             device.setPrecision();
    //         }
    //
    //         device.writeAttr(SUMO_ATTR_TYPE, toString(n->getType()));
    //         // std::set is already sorted, so the output is stable
    //         std::set<std::string> tlsIDs;
    //         std::set<std::string> controlledInnerEdges;
    //         std::string tlType = "";
    //         for (NBTrafficLightDefinition* tl : n->getControllingTLS()) {
    //             tlsIDs.insert(tl->getID());
    //             std::vector<std::string> cie = tl->getControlledInnerEdges();
    //             controlledInnerEdges.insert(cie.begin(), cie.end());
    //             if (tl->getType() != tlsDefaultType) {
    //                 tlType = toString(tl->getType());
    //             }
    //         }
    //         device.writeOptionalAttr(SUMO_ATTR_TLID, tlsIDs, tlsIDs.empty());
    //         device.writeOptionalAttr(SUMO_ATTR_TLTYPE, tlType, tlType == "");
    //         device.writeOptionalAttr(SUMO_ATTR_CONTROLLED_INNER, controlledInnerEdges, controlledInnerEdges.empty());
    //         NWWriter_SUMO::writeShape(device, gch, n->getShape(), SUMO_ATTR_SHAPE, useGeo, geoAccuracy, n->hasCustomShape());
    //         device.writeOptionalAttr(SUMO_ATTR_RADIUS, n->getRadius(), n->getRadius() == NBNode::UNSPECIFIED_RADIUS);
    //         device.writeOptionalAttr<bool>(SUMO_ATTR_KEEP_CLEAR, n->getKeepClear(), n->getKeepClear());
    //         device.writeOptionalAttr(SUMO_ATTR_RIGHT_OF_WAY, toString(n->getRightOfWay()), n->getRightOfWay() == RightOfWay::DEFAULT);
    //         device.writeOptionalAttr(SUMO_ATTR_FRINGE, toString(n->getFringeType()), n->getFringeType() == FringeType::DEFAULT);
    //         device.writeOptionalAttr(SUMO_ATTR_ROUNDABOUT, toString(n->getRoundaboutType()), n->getRoundaboutType() == RoundaboutType::DEFAULT);
    //         device.writeOptionalAttr(SUMO_ATTR_NAME, StringUtils::escapeXML(n->getName()), n->getName() == "");
    //         n->writeParams(device);
    //         device.closeTag();
    //     }
    //     device.close();
    // }
    #[allow(unused_variables)]
    pub fn write_nodes(oc: &OptionsCont, prefix: &str, nc: &mut NBNodeCont) {
        todo!("port NWWriter_XML::writeNodes")
    }

    /// C++: `void NWWriter_XML::writeTypes(const std::string &, NBEdgeCont &, NBTypeCont &)` [static, private]
    /// Writes the types file
    // void
    // NWWriter_XML::writeTypes(const std::string& prefix, NBEdgeCont& ec, NBTypeCont& tc) {
    //     const std::string ext = OptionsCont::getOptions().getString("output.format");
    //     OutputDevice& device = OutputDevice::getDevice(prefix + ".typ." + ext);
    //     std::map<SumoXMLAttr, std::string> attrs;
    //     attrs[SUMO_ATTR_VERSION] = toString(NETWORK_VERSION);
    //     device.writeXMLHeader(toString(SUMO_TAG_TYPES), "types_file.xsd", attrs);
    //     std::set<std::string> usedTypes = ec.getUsedTypes();
    //     tc.writeEdgeTypes(device, usedTypes);
    //     device.close();
    // }
    #[allow(unused_variables)]
    pub fn write_types(prefix: &str, ec: &mut NBEdgeCont, tc: &mut NBTypeCont) {
        todo!("port NWWriter_XML::writeTypes")
    }

    /// C++: `void NWWriter_XML::writeEdgesAndConnections(const OptionsCont &, const std::string &, NBNodeCont &, NBEdgeCont &)` [static, private]
    /// Writes the edges and connections files
    // void
    // NWWriter_XML::writeEdgesAndConnections(const OptionsCont& oc, const std::string& prefix, NBNodeCont& nc, NBEdgeCont& ec) {
    //     const std::string ext = oc.getString("output.format");
    //     const GeoConvHelper& gch = GeoConvHelper::getFinal();
    //     bool useGeo = oc.exists("proj.plain-geo") && oc.getBool("proj.plain-geo");
    //     const bool geoAccuracy = useGeo || gch.usingInverseGeoProjection();
    //     const bool dlrNavteq = oc.exists("dlr-navteq.plain") && oc.getBool("dlr-navteq.plain");
    //
    //     std::map<SumoXMLAttr, std::string> attrs;
    //     attrs[SUMO_ATTR_VERSION] = toString(NETWORK_VERSION);
    //     OutputDevice& edevice = OutputDevice::getDevice(prefix + ".edg." + ext);
    //     edevice.writeXMLHeader("edges", "edges_file.xsd", attrs);
    //     OutputDevice& cdevice = OutputDevice::getDevice(prefix + ".con." + ext);
    //     cdevice.writeXMLHeader("connections", "connections_file.xsd", attrs);
    //     const bool writeNames = oc.getBool("output.street-names");
    //     const bool writeLanes = oc.getBool("plain-output.lanes");
    //
    //     // write network offsets and projection to allow reconstruction of original coordinates at least for geo-referenced networks
    //     if (!useGeo && gch.usingGeoProjection()) {
    //         GeoConvHelper::writeLocation(edevice);
    //     }
    //     LaneSpreadFunction defaultSpread = SUMOXMLDefinitions::LaneSpreadFunctions.get(oc.getString("default.spreadtype"));
    //     for (std::map<std::string, NBEdge*>::const_iterator i = ec.begin(); i != ec.end(); ++i) {
    //         // write the edge itself to the edges-files
    //         NBEdge* e = (*i).second;
    //         edevice.openTag(SUMO_TAG_EDGE);
    //         edevice.writeAttr(SUMO_ATTR_ID, e->getID());
    //         edevice.writeAttr(SUMO_ATTR_FROM, e->getFromNode()->getID());
    //         edevice.writeAttr(SUMO_ATTR_TO, e->getToNode()->getID());
    //         edevice.writeOptionalAttr(SUMO_ATTR_NAME, StringUtils::escapeXML(e->getStreetName()), !writeNames || e->getStreetName() == "");
    //         edevice.writeAttr(SUMO_ATTR_PRIORITY, e->getPriority());
    //         // write the type if given
    //         edevice.writeOptionalAttr(SUMO_ATTR_TYPE, e->getTypeID(), e->getTypeID() == "");
    //         edevice.writeOptionalAttr(SUMO_ATTR_ROUTINGTYPE, e->getRoutingType(), e->getRoutingType() == "");
    //         edevice.writeAttr(SUMO_ATTR_NUMLANES, e->getNumLanes());
    //         edevice.writeOptionalAttr(SUMO_ATTR_SPEED, e->getSpeed(), e->hasLaneSpecificSpeed());
    //         edevice.writeOptionalAttr(SUMO_ATTR_FRICTION, e->getFriction(), e->getFriction() == NBEdge::UNSPECIFIED_FRICTION || e->hasLaneSpecificFriction());
    //         // write non-default geometry
    //         NWWriter_SUMO::writeShape(edevice, gch, e->getGeometry(), SUMO_ATTR_SHAPE, useGeo, geoAccuracy, !e->hasDefaultGeometry());
    //         // write the spread type if not default ("right")
    //         edevice.writeOptionalAttr(SUMO_ATTR_SPREADTYPE, toString(e->getLaneSpreadFunction()), e->getLaneSpreadFunction() == defaultSpread);
    //         // write the length if it was specified
    //         edevice.writeOptionalAttr(SUMO_ATTR_LENGTH, e->getLoadedLength(), !e->hasLoadedLength());
    //         // some attributes can be set by edge default or per lane. Write as default if possible (efficiency)
    //         edevice.writeOptionalAttr(SUMO_ATTR_WIDTH, e->getLaneWidth(), e->getLaneWidth() == NBEdge::UNSPECIFIED_WIDTH || e->hasLaneSpecificWidth());
    //         edevice.writeOptionalAttr(SUMO_ATTR_ENDOFFSET, e->getEndOffset(), e->getEndOffset() == NBEdge::UNSPECIFIED_OFFSET || e->hasLaneSpecificEndOffset());
    //         // SVC_UNSPECIFIED is here a shortcut for do not write
    //         writePermissions(edevice, e->hasLaneSpecificPermissions() ? SVC_UNSPECIFIED : e->getPermissions(0));
    //         if (!e->hasLaneSpecificStopOffsets() && e->getEdgeStopOffset().isDefined()) {
    //             NWWriter_SUMO::writeStopOffsets(edevice, e->getEdgeStopOffset());
    //         }
    //         edevice.writeOptionalAttr(SUMO_ATTR_DISTANCE, e->getDistance(), e->getDistance() == 0);
    //         edevice.writeOptionalAttr(SUMO_ATTR_BIDI, e->getBidiEdge() == nullptr ? "" : e->getBidiEdge()->getID(), e->getBidiEdge() == nullptr);
    //         if (dlrNavteq) {
    //             NWWriter_DlrNavteq::writeEdgeAttrs(edevice, e);
    //         }
    //         if (e->needsLaneSpecificOutput() || writeLanes) {
    //             int idx = 0;
    //             for (const NBEdge::Lane& lane : e->getLanes()) {
    //                 edevice.openTag(SUMO_TAG_LANE);
    //                 edevice.writeAttr(SUMO_ATTR_INDEX, idx++);
    //                 writePermissions(edevice, e->hasLaneSpecificPermissions() || writeLanes ? lane.permissions : SVC_UNSPECIFIED);
    //                 writePreferences(edevice, lane.preferred);
    //                 edevice.writeOptionalAttr(SUMO_ATTR_WIDTH, lane.width, lane.width == NBEdge::UNSPECIFIED_WIDTH || !(e->hasLaneSpecificWidth() || writeLanes));
    //                 edevice.writeOptionalAttr(SUMO_ATTR_ENDOFFSET, lane.endOffset, lane.endOffset == NBEdge::UNSPECIFIED_OFFSET || !(e->hasLaneSpecificEndOffset() || writeLanes));
    //                 edevice.writeOptionalAttr(SUMO_ATTR_SPEED, lane.speed, !(e->hasLaneSpecificSpeed() || writeLanes));
    //                 edevice.writeOptionalAttr(SUMO_ATTR_ACCELERATION, lane.accelRamp, !lane.accelRamp);
    //                 NWWriter_SUMO::writeShape(edevice, gch, lane.customShape.size() > 0 ? lane.customShape : lane.shape, SUMO_ATTR_SHAPE, useGeo, geoAccuracy, lane.customShape.size() > 0 || writeLanes);
    //                 edevice.writeOptionalAttr(SUMO_ATTR_TYPE, lane.type, lane.type == "");
    //                 const bool changeLeft = (lane.changeLeft != SVCAll && lane.changeLeft != SVC_UNSPECIFIED && lane.changeLeft != SVC_IGNORING);
    //                 edevice.writeOptionalAttr(SUMO_ATTR_CHANGE_LEFT, getVehicleClassNames(lane.changeLeft), !changeLeft);
    //                 const bool changeRight = (lane.changeRight != SVCAll && lane.changeRight != SVC_UNSPECIFIED && lane.changeRight != SVC_IGNORING);
    //                 edevice.writeOptionalAttr(SUMO_ATTR_CHANGE_RIGHT, getVehicleClassNames(lane.changeRight), !changeRight);
    //                 if (lane.oppositeID != "") {
    //                     edevice.openTag(SUMO_TAG_NEIGH);
    //                     edevice.writeAttr(SUMO_ATTR_LANE, lane.oppositeID);
    //                     edevice.closeTag();
    //                 }
    //                 lane.writeParams(edevice);
    //                 NWWriter_SUMO::writeStopOffsets(edevice, lane.laneStopOffset);
    //                 edevice.closeTag();
    //             }
    //         }
    //         e->writeParams(edevice);
    //         edevice.closeTag();
    //         // write this edge's connections to the connections-files
    //         const std::vector<NBEdge::Connection> connections = e->getConnections();
    //         if (connections.empty()) {
    //             // if there are no connections and this appears to be customized, preserve the information
    //             const int numOutgoing = (int)e->getToNode()->getOutgoingEdges().size();
    //             if (numOutgoing > 0) {
    //                 const SVCPermissions inPerm = e->getPermissions();
    //                 SVCPermissions outPerm = 0;
    //                 for (auto out : e->getToNode()->getOutgoingEdges()) {
    //                     outPerm |= out->getPermissions();
    //                 }
    //                 if ((inPerm & outPerm) != 0 && (inPerm & outPerm) != SVC_PEDESTRIAN) {
    //                     cdevice.openTag(SUMO_TAG_CONNECTION);
    //                     cdevice.writeAttr(SUMO_ATTR_FROM, e->getID());
    //                     cdevice.closeTag();
    //                     cdevice.lf();
    //                 }
    //             }
    //         } else {
    //             for (NBEdge::Connection c : connections) {
    //                 NWWriter_SUMO::writeConnection(cdevice, *e, c, false, dlrNavteq ? NWWriter_SUMO::PLAIN_NAVTEQ : NWWriter_SUMO::PLAIN, useGeo, geoAccuracy);
    //             }
    //             cdevice.lf();
    //         }
    //     }
    //     // write roundabout information to the edges-files
    //     if (ec.getRoundabouts().size() > 0) {
    //         edevice.lf();
    //         NWWriter_SUMO::writeRoundabouts(edevice, ec.getRoundabouts(), ec);
    //     }
    //
    //     // write loaded prohibitions to the connections-file
    //     for (std::map<std::string, NBNode*>::const_iterator i = nc.begin(); i != nc.end(); ++i) {
    //         NWWriter_SUMO::writeProhibitions(cdevice, i->second->getProhibitions(), ec);
    //     }
    //     // write pedestrian crossings to the connections-file
    //     for (std::map<std::string, NBNode*>::const_iterator it_node = nc.begin(); it_node != nc.end(); ++it_node) {
    //         const std::vector<NBNode::Crossing*>& crossings = (*it_node).second->getCrossings();
    //         for (auto c : crossings) {
    //             cdevice.openTag(SUMO_TAG_CROSSING);
    //             cdevice.writeAttr(SUMO_ATTR_NODE, (*it_node).second->getID());
    //             cdevice.writeAttr(SUMO_ATTR_EDGES, c->edges);
    //             cdevice.writeAttr(SUMO_ATTR_PRIORITY, c->priority);
    //             cdevice.writeOptionalAttr(SUMO_ATTR_WIDTH, c->customWidth, c->customWidth == NBEdge::UNSPECIFIED_WIDTH);
    //             cdevice.writeOptionalAttr(SUMO_ATTR_TLLINKINDEX, c->customTLIndex, c->customTLIndex == -1);
    //             cdevice.writeOptionalAttr(SUMO_ATTR_TLLINKINDEX2, c->customTLIndex2, c->customTLIndex2 == -1);
    //             NWWriter_SUMO::writeShape(cdevice, gch, c->customShape, SUMO_ATTR_SHAPE, useGeo, geoAccuracy, c->customShape.size() != 0);
    //             NWWriter_SUMO::writeShape(cdevice, gch, c->outlineShape, SUMO_ATTR_OUTLINESHAPE, useGeo, geoAccuracy, c->outlineShape.size() != 0);
    //             c->writeParams(cdevice);
    //             cdevice.closeTag();
    //         }
    //     }
    //     // write custom walkingarea shapes to the connections file
    //     for (std::map<std::string, NBNode*>::const_iterator it_node = nc.begin(); it_node != nc.end(); ++it_node) {
    //         for (const auto& wacs : it_node->second->getWalkingAreaCustomShapes()) {
    //             cdevice.openTag(SUMO_TAG_WALKINGAREA);
    //             cdevice.writeAttr(SUMO_ATTR_NODE, it_node->first);
    //             cdevice.writeAttr(SUMO_ATTR_EDGES, joinNamedToString(wacs.edges, " "));
    //             NWWriter_SUMO::writeShape(cdevice, gch, wacs.shape, SUMO_ATTR_SHAPE, useGeo, geoAccuracy, wacs.shape.size() != 0);
    //             cdevice.writeOptionalAttr(SUMO_ATTR_WIDTH, wacs.width, wacs.width == NBEdge::UNSPECIFIED_WIDTH);
    //             cdevice.closeTag();
    //         }
    //     }
    //
    //     edevice.close();
    //     cdevice.close();
    //
    //     if (dlrNavteq) {
    //         OutputDevice& cdevice2 = OutputDevice::getDevice(prefix + ".cpr." + ext);
    //         cdevice2.writeXMLHeader("connections", "connections_file.xsd");
    //         NWWriter_DlrNavteq::writeProhibitedManoeuvresAttrs(cdevice2, nc);
    //         cdevice2.close();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_edges_and_connections(oc: &OptionsCont, prefix: &str, nc: &mut NBNodeCont, ec: &mut NBEdgeCont) {
        todo!("port NWWriter_XML::writeEdgesAndConnections")
    }

    /// C++: `void NWWriter_XML::writeTrafficLights(const std::string &, NBTrafficLightLogicCont &, NBEdgeCont &)` [static, private]
    /// Writes the traffic lights file
    // void
    // NWWriter_XML::writeTrafficLights(const std::string& prefix, NBTrafficLightLogicCont& tc, NBEdgeCont& ec) {
    //     const std::string ext = OptionsCont::getOptions().getString("output.format");
    //     std::map<SumoXMLAttr, std::string> attrs;
    //     attrs[SUMO_ATTR_VERSION] = toString(NETWORK_VERSION);
    //     OutputDevice& device = OutputDevice::getDevice(prefix + ".tll." + ext);
    //     device.writeXMLHeader("tlLogics", "tllogic_file.xsd", attrs);
    //     const OptionsCont& oc = OptionsCont::getOptions();
    //     const bool dlrNavteq = oc.exists("dlr-navteq.plain") && oc.getBool("dlr-navteq.plain");
    //     NWWriter_SUMO::writeTrafficLights(device, tc, dlrNavteq ? NWWriter_SUMO::PLAIN_NAVTEQ : NWWriter_SUMO::PLAIN);
    //     // we also need to remember the associations between tlLogics and connections
    //     // since the information in con.xml is insufficient
    //     for (std::map<std::string, NBEdge*>::const_iterator i = ec.begin(); i != ec.end(); ++i) {
    //         NBEdge* e = (*i).second;
    //         // write this edge's tl-controlled connections
    //         const std::vector<NBEdge::Connection> connections = e->getConnections();
    //         for (std::vector<NBEdge::Connection>::const_iterator c = connections.begin(); c != connections.end(); ++c) {
    //             if (c->tlID != "") {
    //                 NWWriter_SUMO::writeConnection(device, *e, *c, false, NWWriter_SUMO::TLL);
    //             }
    //         }
    //     }
    //     device.close();
    // }
    #[allow(unused_variables)]
    pub fn write_traffic_lights(prefix: &str, tc: &mut NBTrafficLightLogicCont, ec: &mut NBEdgeCont) {
        todo!("port NWWriter_XML::writeTrafficLights")
    }

    /// C++: `void NWWriter_XML::writePTStops(const OptionsCont &, NBPTStopCont &)` [static, private]
    /// Writes the pt stops file
    // void
    // NWWriter_XML::writePTStops(const OptionsCont& oc, NBPTStopCont& sc) {
    //     OutputDevice& device = OutputDevice::getDevice(oc.getString("ptstop-output"));
    //     device.writeXMLHeader("additional", "additional_file.xsd");
    //     for (const auto& stopIt : sc.getStops()) {
    //         stopIt.second->write(device);
    //     }
    //     device.close();
    // }
    #[allow(unused_variables)]
    pub fn write_pt_stops(oc: &OptionsCont, ec: &mut NBPTStopCont) {
        todo!("port NWWriter_XML::writePTStops")
    }

    /// C++: `void NWWriter_XML::writePTLines(const OptionsCont &, NBPTLineCont &)` [static, private]
    // void NWWriter_XML::writePTLines(const OptionsCont& oc, NBPTLineCont& lc) {
    //     OutputDevice& device = OutputDevice::getDevice(oc.getString("ptline-output"));
    //     device.writeXMLHeader("ptLines", "ptlines_file.xsd");
    //     for (const auto& item : lc.getLines()) {
    //         item.second->write(device);
    //     }
    //     device.close();
    // }
    #[allow(unused_variables)]
    pub fn write_pt_lines(cont: &OptionsCont, lc: &mut NBPTLineCont) {
        todo!("port NWWriter_XML::writePTLines")
    }

    /// C++: `void NWWriter_XML::writeParkingAreas(const OptionsCont &, NBParkingCont &, NBEdgeCont &)` [static, private]
    /// writes imported parking areas to file
    // void NWWriter_XML::writeParkingAreas(const OptionsCont& oc, NBParkingCont& pc, NBEdgeCont& ec) {
    //     OutputDevice& device = OutputDevice::getDevice(oc.getString("parking-output"));
    //     device.writeXMLHeader("additional", "additional_file.xsd");
    //     for (NBParking& p : pc) {
    //         p.write(device, ec);
    //     }
    //     device.close();
    // }
    #[allow(unused_variables)]
    pub fn write_parking_areas(cont: &OptionsCont, pc: &mut NBParkingCont, ec: &mut NBEdgeCont) {
        todo!("port NWWriter_XML::writeParkingAreas")
    }

    /// C++: `void NWWriter_XML::writeDistricts(const OptionsCont &, NBDistrictCont &)` [static, private]
    /// writes imported districts (TAZ) to file
    // void
    // NWWriter_XML::writeDistricts(const OptionsCont& oc, NBDistrictCont& dc) {
    //     OutputDevice& device = OutputDevice::getDevice(oc.getString("taz-output"));
    //     device.writeXMLHeader("additional", "additional_file.xsd");
    //     for (std::map<std::string, NBDistrict*>::const_iterator i = dc.begin(); i != dc.end(); i++) {
    //         NWWriter_SUMO::writeDistrict(device, *(*i).second);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_districts(oc: &OptionsCont, dc: &mut NBDistrictCont) {
        todo!("port NWWriter_XML::writeDistricts")
    }

}

