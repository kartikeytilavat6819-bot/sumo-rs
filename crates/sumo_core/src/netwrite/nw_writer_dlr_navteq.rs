//! Port of netwrite/NWWriter_DlrNavteq.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Exporter writing networks using XML (native input) format
/// C++ class `NWWriter_DlrNavteq` (netwrite/NWWriter_DlrNavteq.h)
pub struct NWWriter_DlrNavteq {
}

// C++ static members of NWWriter_DlrNavteq (port as module-level statics, see AGENTS.md):
//   static const std::string NWWriter_DlrNavteq::UNDEFINED;

impl NWWriter_DlrNavteq {
    /// C++: `void NWWriter_DlrNavteq::writeNetwork(const OptionsCont &, NBNetBuilder &)` [static]
    /// Writes the network into XML-files (nodes, edges, connections, traffic lights)
    // void
    // NWWriter_DlrNavteq::writeNetwork(const OptionsCont& oc, NBNetBuilder& nb) {
    //     // check whether a matsim-file shall be generated
    //     if (!oc.isSet("dlr-navteq-output")) {
    //         return;
    //     }
    //     std::map<const NBEdge*, std::string> internalNodes;
    //     writeNodesUnsplitted(oc, nb.getNodeCont(), nb.getEdgeCont(), internalNodes);
    //     writeLinksUnsplitted(oc, nb.getEdgeCont(), internalNodes);
    //     writeTrafficSignals(oc, nb.getNodeCont());
    //     writeProhibitedManoeuvres(oc, nb.getNodeCont(), nb.getEdgeCont());
    //     writeConnectedLanes(oc, nb.getNodeCont());
    // }
    #[allow(unused_variables)]
    pub fn write_network(oc: &OptionsCont, nb: &mut NBNetBuilder) {
        todo!("port NWWriter_DlrNavteq::writeNetwork")
    }

    /// C++: `int NWWriter_DlrNavteq::getRoadClass(const NBEdge *const)` [static]
    /// get the navteq road class
    // int
    // NWWriter_DlrNavteq::getRoadClass(const NBEdge* const edge) {
    //     // quoting the navteq manual:
    //     // As a general rule, Functional Road Class assignments have no direct
    //     // correlation with other road attributes like speed, controlled access, route type, etc.
    //     // if the network is based on OSM, we can use the highway types for determining FRC
    //     std::string type = edge->getTypeID();
    //     if (StringUtils::startsWith(type, "highway.")) {
    //         type = type.substr(8);
    //     }
    //     if (StringUtils::startsWith(type, "motorway")) {
    //         return 0;
    //     } else if (StringUtils::startsWith(type, "trunk")) {
    //         return 1;
    //     } else if (StringUtils::startsWith(type, "primary")) {
    //         return 1;
    //     } else if (StringUtils::startsWith(type, "secondary")) {
    //         return 2;
    //     } else if (StringUtils::startsWith(type, "tertiary")) {
    //         return 3;
    //     } else if (type == "unclassified") {
    //         return 3;
    //     } else if (type == "living_street" || type == "residential" || type == "road" || type == "service" || type == "track" || type == "cycleway" || type == "path" || type == "footway") {
    //         return 4;
    //     }
    //     // as a fallback we do a simple speed / lane-count mapping anyway
    //     // the resulting functional road class layers probably won't be connected as required
    //     const int kph = speedInKph(edge->getSpeed());
    //     if ((kph) > 100) {
    //         return 0;
    //     }
    //     if ((kph) > 70) {
    //         return 1;
    //     }
    //     if ((kph) > 50) {
    //         return (edge->getNumLanes() > 1 ? 2 : 3);
    //     }
    //     if ((kph) > 30) {
    //         return 3;
    //     }
    //     return 4;
    // }
    #[allow(unused_variables)]
    pub fn get_road_class(edge: Ptr<NBEdge>) -> i32 {
        todo!("port NWWriter_DlrNavteq::getRoadClass")
    }

    /// C++: `int NWWriter_DlrNavteq::getBrunnelType(const NBEdge *const)` [static]
    /// get the navteq brunnel type
    // int
    // NWWriter_DlrNavteq::getBrunnelType(const NBEdge* const edge) {
    //     if (edge->hasParameter("bridge")) {
    //         return 1;
    //     } else if (edge->hasParameter("tunnel")) {
    //         return 4;
    //     } else if (edge->getTypeID() == "route.ferry") {
    //         return 10;
    //     }
    //     return -1; // UNDEFINED
    // }
    #[allow(unused_variables)]
    pub fn get_brunnel_type(edge: Ptr<NBEdge>) -> i32 {
        todo!("port NWWriter_DlrNavteq::getBrunnelType")
    }

    /// C++: `int NWWriter_DlrNavteq::getFormOfWay(const NBEdge *const)` [static]
    /// get the form of way
    // int
    // NWWriter_DlrNavteq::getFormOfWay(const NBEdge* const edge) {
    //     if (edge->getPermissions() == SVC_PEDESTRIAN) {
    //         return 15;
    //     } else if (edge->getJunctionPriority(edge->getToNode()) == NBEdge::JunctionPriority::ROUNDABOUT) {
    //         return 4;
    //     } else if (edge->getTypeID() == "highway.service") {
    //         return 14;
    //     } else if (edge->getTypeID().find("_link") != std::string::npos) {
    //         return 10;
    //     }
    //     return 3; // speed category 1-8;
    // }
    #[allow(unused_variables)]
    pub fn get_form_of_way(edge: Ptr<NBEdge>) -> i32 {
        todo!("port NWWriter_DlrNavteq::getFormOfWay")
    }

    /// C++: `void NWWriter_DlrNavteq::writeEdgeAttrs(OutputDevice &, const NBEdge *)` [static]
    /// extra attributes into plain output
    // void
    // NWWriter_DlrNavteq::writeEdgeAttrs(OutputDevice& device, const NBEdge* edge) {
    //     const int kph = speedInKph(edge->getSpeed());
    //     device.writeAttr("speedCategory", getSpeedCategory(kph));
    //     device.writeAttr("speedRestriction", getSpeedCategoryUpperBound(kph));
    //     device.writeAttr("maxWidth", getMaxLaneWidth(edge));
    //     device.writeAttr("FRC", getRoadClass(edge));
    //     device.writeAttr("vehicleTypeCode", getAllowedTypes(edge->getPermissions()));
    //     device.writeAttr("formOfWay", getFormOfWay(edge));
    //     device.writeAttr("isRamp", edge->isOffRamp());
    //     device.writeAttr("brunnelType", getBrunnelType(edge));
    //     device.writeAttr("postalCode", getSinglePostalCode(edge->getParameter("postal_code", UNDEFINED), edge->getID()));
    //     device.writeAttr("surface", edge->getParameter("surface", UNDEFINED));
    // }
    #[allow(unused_variables)]
    pub fn write_edge_attrs(device: &mut OutputDevice, edge: Ptr<NBEdge>) {
        todo!("port NWWriter_DlrNavteq::writeEdgeAttrs")
    }

    /// C++: `void NWWriter_DlrNavteq::writeConnectionAttrs(OutputDevice &, const NBEdge::Connection &)` [static]
    // void
    // NWWriter_DlrNavteq::writeConnectionAttrs(OutputDevice& device, const NBEdge::Connection& c) {
    //     device.writeAttr(SUMO_ATTR_NODE, c.toEdge->getFromNode()->getID());
    //     device.writeAttr("vehicleTypeCode", getAllowedTypes(c.permissions));
    // }
    #[allow(unused_variables)]
    pub fn write_connection_attrs(device: &mut OutputDevice, c: &NBEdge_Connection) {
        todo!("port NWWriter_DlrNavteq::writeConnectionAttrs")
    }

    /// C++: `void NWWriter_DlrNavteq::writeProhibitedManoeuvresAttrs(OutputDevice &, const NBNodeCont &)` [static]
    // void
    // NWWriter_DlrNavteq::writeProhibitedManoeuvresAttrs(OutputDevice& device, const NBNodeCont& nc) {
    //     // similar to writeProhibitedManoeuvres but as xml/csv/parquet
    //     for (std::map<std::string, NBNode*>::const_iterator i = nc.begin(); i != nc.end(); ++i) {
    //         NBNode* n = (*i).second;
    //         const EdgeVector& incoming = n->getIncomingEdges();
    //         const EdgeVector& outgoing = n->getOutgoingEdges();
    //         for (EdgeVector::const_iterator j = incoming.begin(); j != incoming.end(); ++j) {
    //             NBEdge* inEdge = *j;
    //             const SVCPermissions inPerm = inEdge->getPermissions();
    //             for (EdgeVector::const_iterator k = outgoing.begin(); k != outgoing.end(); ++k) {
    //                 NBEdge* outEdge = *k;
    //                 const SVCPermissions outPerm = outEdge->getPermissions();
    //                 const SVCPermissions commonPerm = inPerm & outPerm;
    //                 if (commonPerm != 0 && commonPerm != SVC_PEDESTRIAN && !inEdge->isConnectedTo(outEdge)) {
    //                     device.openTag(SUMO_TAG_PROHIBITED_CONNECTION);
    //                     device.writeAttr(SUMO_ATTR_FROM, inEdge->getID());
    //                     device.writeAttr(SUMO_ATTR_TO, outEdge->getID());
    //                     device.closeTag();
    //                 }
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_prohibited_manoeuvres_attrs(device: &mut OutputDevice, nc: &NBNodeCont) {
        todo!("port NWWriter_DlrNavteq::writeProhibitedManoeuvresAttrs")
    }

    /// C++: `void NWWriter_DlrNavteq::writeNodesUnsplitted(const OptionsCont &, const NBNodeCont &, const NBEdgeCont &, std::map<const NBEdge *, std::string> &)` [static, private]
    /// Writes the nodes_unsplitted file
    // void
    // NWWriter_DlrNavteq::writeNodesUnsplitted(const OptionsCont& oc, const NBNodeCont& nc, const NBEdgeCont& ec, std::map<const NBEdge*, std::string>& internalNodes) {
    //     // For "real" nodes we simply use the node id.
    //     // For internal nodes (geometry vectors describing edge geometry in the parlance of this format)
    //     // we use the id of the edge and do not bother with
    //     // compression (each direction gets its own internal node).
    //     OutputDevice& device = OutputDevice::getDevice(oc.getString("dlr-navteq-output") + "_nodes_unsplitted.txt");
    //     writeHeader(device, oc);
    //     const GeoConvHelper& gch = GeoConvHelper::getFinal();
    //     const bool haveGeo = gch.usingGeoProjection();
    //     const double geoScale = pow(10.0f, haveGeo ? 5 : 2); // see NIImporter_DlrNavteq::GEO_SCALE
    //     device.setPrecision(oc.getInt("dlr-navteq.precision"));
    //     if (!haveGeo) {
    //         WRITE_WARNING(TL("DlrNavteq node data will be written in (floating point) cartesian coordinates"));
    //     }
    //     // write format specifier
    //     device << "# NODE_ID\tIS_BETWEEN_NODE\tamount_of_geocoordinates\tx1\ty1\t[x2 y2  ... xn  yn]\n";
    //     // write header
    //     Boundary boundary = gch.getConvBoundary();
    //     Position min(boundary.xmin(), boundary.ymin());
    //     Position max(boundary.xmax(), boundary.ymax());
    //     gch.cartesian2geo(min);
    //     min.mul(geoScale);
    //     gch.cartesian2geo(max);
    //     max.mul(geoScale);
    //     int multinodes = 0;
    //     for (std::map<std::string, NBEdge*>::const_iterator i = ec.begin(); i != ec.end(); ++i) {
    //         if ((*i).second->getGeometry().size() > 2) {
    //             multinodes++;
    //         }
    //     }
    //     device << "# [xmin_region] " << min.x() << "\n";
    //     device << "# [xmax_region] " << max.x() << "\n";
    //     device << "# [ymin_region] " << min.y() << "\n";
    //     device << "# [ymax_region] " << max.y() << "\n";
    //     device << "# [elements_multinode] " << multinodes << "\n";
    //     device << "# [elements_normalnode] " << nc.size() << "\n";
    //     device << "# [xmin] " << min.x() << "\n";
    //     device << "# [xmax] " << max.x() << "\n";
    //     device << "# [ymin] " << min.y() << "\n";
    //     device << "# [ymax] " << max.y() << "\n";
    //     // write normal nodes
    //     for (std::map<std::string, NBNode*>::const_iterator i = nc.begin(); i != nc.end(); ++i) {
    //         NBNode* n = (*i).second;
    //         Position pos = n->getPosition();
    //         gch.cartesian2geo(pos);
    //         pos.mul(geoScale);
    //         device << n->getID() << "\t0\t1\t" << pos.x() << "\t" << pos.y() << "\n";
    //     }
    //     // write "internal" nodes
    //     std::vector<std::string> avoid;
    //     std::set<std::string> reservedNodeIDs;
    //     const bool numericalIDs = oc.getBool("numerical-ids");
    //     if (oc.isSet("reserved-ids")) {
    //         NBHelpers::loadPrefixedIDsFomFile(oc.getString("reserved-ids"), "node:", reservedNodeIDs); // backward compatibility
    //         NBHelpers::loadPrefixedIDsFomFile(oc.getString("reserved-ids"), "junction:", reservedNodeIDs); // selection format
    //     }
    //     if (numericalIDs) {
    //         avoid = nc.getAllNames();
    //         std::vector<std::string> avoid2 = ec.getAllNames();
    //         avoid.insert(avoid.end(), avoid2.begin(), avoid2.end());
    //         avoid.insert(avoid.end(), reservedNodeIDs.begin(), reservedNodeIDs.end());
    //     }
    //     IDSupplier idSupplier("", avoid);
    //     for (const auto& edgeIt : ec) {
    //         const NBEdge* const e = edgeIt.second;
    //         PositionVector geom = e->getGeometry();
    //         if (geom.size() > 2) {
    //             // the import NIImporter_DlrNavteq checks for the presence of a
    //             // negated edge id to determine spread type. We may need to do some
    //             // shifting to make this consistent
    //             const bool hasOppositeID = ec.getOppositeByID(e->getID()) != nullptr;
    //             if (e->getLaneSpreadFunction() == LaneSpreadFunction::RIGHT && !hasOppositeID) {
    //                 // need to write center-line geometry instead
    //                 try {
    //                     geom.move2side(e->getTotalWidth() / 2);
    //                 } catch (InvalidArgument& exception) {
    //                     WRITE_WARNINGF(TL("Could not reconstruct shape for edge:'%' (%)."), e->getID(), exception.what());
    //                 }
    //             } else if (e->getLaneSpreadFunction() == LaneSpreadFunction::CENTER && hasOppositeID) {
    //                 // need to write left-border geometry instead
    //                 try {
    //                     geom.move2side(-e->getTotalWidth() / 2);
    //                 } catch (InvalidArgument& exception) {
    //                     WRITE_WARNINGF(TL("Could not reconstruct shape for edge:'%' (%)."), e->getID(), exception.what());
    //                 }
    //             }
    //
    //             if (geom.size() > 2) { // move2side might have changed the number of nodes
    //                 std::string internalNodeID = e->getID();
    //                 if (internalNodeID == UNDEFINED
    //                         || (nc.retrieve(internalNodeID) != nullptr)
    //                         || reservedNodeIDs.count(internalNodeID) > 0
    //                    ) {
    //                     // need to invent a new name to avoid clashing with the id of a 'real' node or a reserved name
    //                     if (numericalIDs) {
    //                         internalNodeID = idSupplier.getNext();
    //                     } else {
    //                         internalNodeID += "_geometry";
    //                     }
    //                 }
    //                 internalNodes[e] = internalNodeID;
    //                 device << internalNodeID << "\t1\t" << geom.size() - 2;
    //                 for (int ii = 1; ii < (int)geom.size() - 1; ++ii) {
    //                     Position pos = geom[(int)ii];
    //                     gch.cartesian2geo(pos);
    //                     pos.mul(geoScale);
    //                     device << "\t" << pos.x() << "\t" << pos.y();
    //                 }
    //                 device << "\n";
    //             }
    //         }
    //     }
    //     device.close();
    // }
    #[allow(unused_variables)]
    pub fn write_nodes_unsplitted(oc: &OptionsCont, nc: &NBNodeCont, ec: &NBEdgeCont, internalNodes: &mut BTreeMap<Ptr<NBEdge>, String>) {
        todo!("port NWWriter_DlrNavteq::writeNodesUnsplitted")
    }

    /// C++: `void NWWriter_DlrNavteq::writeLinksUnsplitted(const OptionsCont &, const NBEdgeCont &, const std::map<const NBEdge *, std::string> &)` [static, private]
    /// Writes the links_unsplitted file
    // void
    // NWWriter_DlrNavteq::writeLinksUnsplitted(const OptionsCont& oc, const NBEdgeCont& ec, const std::map<const NBEdge*, std::string>& internalNodes) {
    //     const int majorVersion = StringUtils::toInt(StringTokenizer(oc.getString("dlr-navteq.version"), ".").next());
    //     std::map<const std::string, std::string> nameIDs;
    //     OutputDevice& device = OutputDevice::getDevice(oc.getString("dlr-navteq-output") + "_links_unsplitted.txt");
    //     writeHeader(device, oc);
    //     // write format specifier
    //     device << "# LINK_ID\tNODE_ID_FROM\tNODE_ID_TO\tBETWEEN_NODE_ID\tLENGTH\tVEHICLE_TYPE\tFORM_OF_WAY\tBRUNNEL_TYPE\t"
    //            << "FUNCTIONAL_ROAD_CLASS\tSPEED_CATEGORY\tNUMBER_OF_LANES\tSPEED_LIMIT\tSPEED_RESTRICTION\t"
    //            << "NAME_ID1_REGIONAL\tNAME_ID2_LOCAL\tHOUSENUMBERS_RIGHT\tHOUSENUMBERS_LEFT\tZIP_CODE\t"
    //            << "AREA_ID\tSUBAREA_ID\tTHROUGH_TRAFFIC\tSPECIAL_RESTRICTIONS\tEXTENDED_NUMBER_OF_LANES\tISRAMP\tCONNECTION";
    //     if (majorVersion > 6) {
    //         device << "\tMAXHEIGHT\tMAXWIDTH\tMAXWEIGHT\tSURFACE";
    //     }
    //     device << "\n";
    //     // write edges
    //     for (const auto& edgeIt : ec) {
    //         const NBEdge* const e = edgeIt.second;
    //         const int kph = speedInKph(e->getSpeed());
    //         const auto& internalIt = internalNodes.find(e);
    //         const std::string& betweenNodeID = internalIt != internalNodes.end() ? internalIt->second : UNDEFINED;
    //         std::string nameID = UNDEFINED;
    //         std::string nameIDRegional = UNDEFINED;
    //         if (oc.getBool("output.street-names")) {
    //             const std::string& name = e->getStreetName();
    //             if (name != "") {
    //                 if (nameIDs.count(name) == 0) {
    //                     const int tmp = (int)nameIDs.size();
    //                     nameIDs[name] = toString(tmp);
    //                 }
    //                 nameID = nameIDs[name];
    //             }
    //             const std::string& name2 = e->getParameter("ref", "");
    //             if (name2 != "") {
    //                 if (nameIDs.count(name2) == 0) {
    //                     const int tmp = (int)nameIDs.size();
    //                     nameIDs[name2] = toString(tmp);
    //                 }
    //                 nameIDRegional = nameIDs[name2];
    //             }
    //         }
    //         device << e->getID() << "\t"
    //                << e->getFromNode()->getID() << "\t"
    //                << e->getToNode()->getID() << "\t"
    //                << betweenNodeID << "\t"
    //                << getGraphLength(e) << "\t"
    //                << getAllowedTypes(e->getPermissions()) << "\t"
    //                << getFormOfWay(e) << "\t"
    //                << getBrunnelType(e) << "\t"
    //                << getRoadClass(e) << "\t"
    //                << getSpeedCategory(kph) << "\t"
    //                << getNavteqLaneCode(e->getNumLanes()) << "\t"
    //                << getSpeedCategoryUpperBound(kph) << "\t"
    //                << kph << "\t"
    //                << nameIDRegional << "\t"
    //                << nameID << "\t" // NAME_ID2_LOCAL
    //                << UNDEFINED << "\t" // housenumbers_right
    //                << UNDEFINED << "\t" // housenumbers_left
    //                << getSinglePostalCode(e->getParameter("postal_code", UNDEFINED), e->getID()) << "\t" // ZIP_CODE
    //                << UNDEFINED << "\t" // AREA_ID
    //                << UNDEFINED << "\t" // SUBAREA_ID
    //                << "1\t" // through_traffic (allowed)
    //                << UNDEFINED << "\t" // special_restrictions
    //                << UNDEFINED << "\t" // extended_number_of_lanes
    //                << UNDEFINED << "\t" // isRamp
    //                << "0"; // connection (between nodes always in order)
    //         if (majorVersion > 6) {
    //             device << "\t" << e->getParameter("maxheight", UNDEFINED)
    //                    << "\t" << e->getParameter("maxwidth", UNDEFINED)
    //                    << "\t" << e->getParameter("maxweight", UNDEFINED)
    //                    << "\t" << e->getParameter("surface", UNDEFINED);
    //         }
    //         device << "\n";
    //     }
    //     if (oc.getBool("output.street-names")) {
    //         OutputDevice& namesDevice = OutputDevice::getDevice(oc.getString("dlr-navteq-output") + "_names.txt");
    //         writeHeader(namesDevice, oc);
    //         // write format specifier
    //         namesDevice << "# NAME_ID\tPERMANENT_ID_INFO\tName\n";
    //         namesDevice << "# [elements] " << nameIDs.size() << "\n";
    //         for (std::map<const std::string, std::string>::const_iterator i = nameIDs.begin(); i != nameIDs.end(); ++i) {
    //             namesDevice
    //                     << i->second << "\t"
    //                     << 0 << "\t"
    //                     << i->first << "\n";
    //         }
    //         namesDevice.close();
    //     }
    //     device.close();
    // }
    #[allow(unused_variables)]
    pub fn write_links_unsplitted(oc: &OptionsCont, ec: &NBEdgeCont, internalNodes: &BTreeMap<Ptr<NBEdge>, String>) {
        todo!("port NWWriter_DlrNavteq::writeLinksUnsplitted")
    }

    /// C++: `void NWWriter_DlrNavteq::writeTrafficSignals(const OptionsCont &, NBNodeCont &)` [static, private]
    /// Writes the traffic_signals file
    // void
    // NWWriter_DlrNavteq::writeTrafficSignals(const OptionsCont& oc, NBNodeCont& nc) {
    //     OutputDevice& device = OutputDevice::getDevice(oc.getString("dlr-navteq-output") + "_traffic_signals.txt");
    //     writeHeader(device, oc);
    //     const GeoConvHelper& gch = GeoConvHelper::getFinal();
    //     const bool haveGeo = gch.usingGeoProjection();
    //     const double geoScale = pow(10.0f, haveGeo ? 5 : 2); // see NIImporter_DlrNavteq::GEO_SCALE
    //     device.setPrecision(oc.getInt("dlr-navteq.precision"));
    //     // write format specifier
    //     device << "#Traffic signal related to LINK_ID and NODE_ID with location relative to driving direction.\n#column format like pointcollection.\n#DESCRIPTION->LOCATION: 1-rechts von LINK; 2-links von LINK; 3-oberhalb LINK -1-keineAngabe\n#RELATREC_ID\tPOICOL_TYPE\tDESCRIPTION\tLONGITUDE\tLATITUDE\tLINK_ID\n";
    //     // write record for every edge incoming to a tls controlled node
    //     for (std::map<std::string, NBNode*>::const_iterator i = nc.begin(); i != nc.end(); ++i) {
    //         NBNode* n = (*i).second;
    //         if (n->isTLControlled()) {
    //             Position pos = n->getPosition();
    //             gch.cartesian2geo(pos);
    //             pos.mul(geoScale);
    //             const EdgeVector& incoming = n->getIncomingEdges();
    //             for (EdgeVector::const_iterator it = incoming.begin(); it != incoming.end(); ++it) {
    //                 NBEdge* e = *it;
    //                 device << e->getID() << "\t"
    //                        << "12\t" // POICOL_TYPE
    //                        << "LSA;NODEIDS#" << n->getID() << "#;LOCATION#-1#;\t"
    //                        << pos.x() << "\t"
    //                        << pos.y() << "\t"
    //                        << e->getID() << "\n";
    //             }
    //         }
    //     }
    //     device.close();
    // }
    #[allow(unused_variables)]
    pub fn write_traffic_signals(oc: &OptionsCont, nc: &mut NBNodeCont) {
        todo!("port NWWriter_DlrNavteq::writeTrafficSignals")
    }

    /// C++: `void NWWriter_DlrNavteq::writeProhibitedManoeuvres(const OptionsCont &, const NBNodeCont &, const NBEdgeCont &)` [static, private]
    /// Writes the prohibited_manoeuvres file
    // void
    // NWWriter_DlrNavteq::writeProhibitedManoeuvres(const OptionsCont& oc, const NBNodeCont& nc, const NBEdgeCont& ec) {
    //     OutputDevice& device = OutputDevice::getDevice(oc.getString("dlr-navteq-output") + "_prohibited_manoeuvres.txt");
    //     writeHeader(device, oc);
    //     // need to invent id for relation
    //     std::set<std::string> reservedRelIDs;
    //     if (oc.isSet("reserved-ids")) {
    //         NBHelpers::loadPrefixedIDsFomFile(oc.getString("reserved-ids"), "rel:", reservedRelIDs);
    //     }
    //     std::vector<std::string> avoid = ec.getAllNames(); // already used for tls RELATREC_ID
    //     avoid.insert(avoid.end(), reservedRelIDs.begin(), reservedRelIDs.end());
    //     IDSupplier idSupplier("", avoid); // @note: use a global relRecIDsupplier if this is used more often
    //     // write format specifier
    //     device << "#No driving allowed from ID1 to ID2 or the complete chain from ID1 to IDn\n";
    //     device << "#RELATREC_ID\tPERMANENT_ID_INFO\tVALIDITY_PERIOD\tTHROUGH_TRAFFIC\tVEHICLE_TYPE\tNAVTEQ_LINK_ID1\t[NAVTEQ_LINK_ID2 ...]\n";
    //     // write record for every pair of incoming/outgoing edge that are not connected despite having common permissions
    //     for (std::map<std::string, NBNode*>::const_iterator i = nc.begin(); i != nc.end(); ++i) {
    //         NBNode* n = (*i).second;
    //         const EdgeVector& incoming = n->getIncomingEdges();
    //         const EdgeVector& outgoing = n->getOutgoingEdges();
    //         for (EdgeVector::const_iterator j = incoming.begin(); j != incoming.end(); ++j) {
    //             NBEdge* inEdge = *j;
    //             const SVCPermissions inPerm = inEdge->getPermissions();
    //             for (EdgeVector::const_iterator k = outgoing.begin(); k != outgoing.end(); ++k) {
    //                 NBEdge* outEdge = *k;
    //                 const SVCPermissions outPerm = outEdge->getPermissions();
    //                 const SVCPermissions commonPerm = inPerm & outPerm;
    //                 if (commonPerm != 0 && commonPerm != SVC_PEDESTRIAN && !inEdge->isConnectedTo(outEdge)) {
    //                     device
    //                             << idSupplier.getNext() << "\t"
    //                             << 1 << "\t" // permanent id
    //                             << UNDEFINED << "\t"
    //                             << 1 << "\t"
    //                             << getAllowedTypes(SVCAll) << "\t"
    //                             << inEdge->getID() << "\t" << outEdge->getID() << "\n";
    //                 }
    //             }
    //         }
    //     }
    //     device.close();
    // }
    #[allow(unused_variables)]
    pub fn write_prohibited_manoeuvres(oc: &OptionsCont, nc: &NBNodeCont, ec: &NBEdgeCont) {
        todo!("port NWWriter_DlrNavteq::writeProhibitedManoeuvres")
    }

    /// C++: `void NWWriter_DlrNavteq::writeConnectedLanes(const OptionsCont &, NBNodeCont &)` [static, private]
    /// Writes the connected_lanes file
    // void
    // NWWriter_DlrNavteq::writeConnectedLanes(const OptionsCont& oc, NBNodeCont& nc) {
    //     OutputDevice& device = OutputDevice::getDevice(oc.getString("dlr-navteq-output") + "_connected_lanes.txt");
    //     writeHeader(device, oc);
    //     // write format specifier
    //     device << "#Lane connections related to LINK-IDs and NODE-ID.\n";
    //     device << "#column format like pointcollection.\n";
    //     device << "#NODE-ID\tVEHICLE-TYPE\tFROM_LANE\tTO_LANE\tTHROUGH_TRAFFIC\tLINK_IDs[2..*]\n";
    //     // write record for every connection
    //     for (std::map<std::string, NBNode*>::const_iterator i = nc.begin(); i != nc.end(); ++i) {
    //         NBNode* n = (*i).second;
    //         const EdgeVector& incoming = n->getIncomingEdges();
    //         for (EdgeVector::const_iterator j = incoming.begin(); j != incoming.end(); ++j) {
    //             NBEdge* from = *j;
    //             const SVCPermissions fromPerm = from->getPermissions();
    //             const std::vector<NBEdge::Connection>& connections = from->getConnections();
    //             for (std::vector<NBEdge::Connection>::const_iterator it_c = connections.begin(); it_c != connections.end(); it_c++) {
    //                 const NBEdge::Connection& c = *it_c;
    //                 device
    //                         << n->getID() << "\t"
    //                         << getAllowedTypes(fromPerm & c.toEdge->getPermissions()) << "\t"
    //                         << c.fromLane + 1 << "\t" // one-based
    //                         << c.toLane + 1 << "\t" // one-based
    //                         << 1 << "\t" // no information regarding permissibility of through traffic
    //                         << from->getID() << "\t"
    //                         << c.toEdge->getID() << "\t"
    //                         << "\n";
    //             }
    //         }
    //     }
    //     device.close();
    // }
    #[allow(unused_variables)]
    pub fn write_connected_lanes(oc: &OptionsCont, nc: &mut NBNodeCont) {
        todo!("port NWWriter_DlrNavteq::writeConnectedLanes")
    }

    /// C++: `void NWWriter_DlrNavteq::writeHeader(OutputDevice &, const OptionsCont &)` [static, private]
    /// write header comments (input parameters, date, etc...)
    // void NWWriter_DlrNavteq::writeHeader(OutputDevice& device, const OptionsCont& oc) {
    //     device << "# Format matches Extraction version: V" << oc.getString("dlr-navteq.version") << " \n";
    //     std::stringstream tmp;
    //     oc.writeConfiguration(tmp, true, false, false);
    //     tmp.seekg(std::ios_base::beg);
    //     std::string line;
    //     while (!tmp.eof()) {
    //         std::getline(tmp, line);
    //         device << "# " << line << "\n";
    //     }
    //     device << "#\n";
    // }
    #[allow(unused_variables)]
    pub fn write_header(device: &mut OutputDevice, oc: &OptionsCont) {
        todo!("port NWWriter_DlrNavteq::writeHeader")
    }

    /// C++: `std::string NWWriter_DlrNavteq::getAllowedTypes(SVCPermissions)` [static, private]
    /// build the ascii-bit-vector for column vehicle_type
    // std::string
    // NWWriter_DlrNavteq::getAllowedTypes(SVCPermissions permissions) {
    //     if (permissions == SVCAll) {
    //         return "100000000000";
    //     }
    //     std::ostringstream oss;
    //     oss << "0";
    //     oss << ((permissions & SVC_PASSENGER)              > 0 ? 1 : 0);
    //     oss << ((permissions & SVC_PASSENGER)              > 0 ? 1 : 0); // residential
    //     oss << ((permissions & SVC_HOV)                    > 0 ? 1 : 0);
    //     oss << ((permissions & SVC_EMERGENCY)              > 0 ? 1 : 0);
    //     oss << ((permissions & SVC_TAXI)                   > 0 ? 1 : 0);
    //     oss << ((permissions & (SVC_BUS | SVC_COACH))      > 0 ? 1 : 0);
    //     oss << ((permissions & SVC_DELIVERY)               > 0 ? 1 : 0);
    //     oss << ((permissions & (SVC_TRUCK | SVC_TRAILER))  > 0 ? 1 : 0);
    //     oss << ((permissions & SVC_MOTORCYCLE)             > 0 ? 1 : 0);
    //     oss << ((permissions & SVC_BICYCLE)                > 0 ? 1 : 0);
    //     oss << ((permissions & SVC_PEDESTRIAN)             > 0 ? 1 : 0);
    //     return oss.str();
    // }
    #[allow(unused_variables)]
    pub fn get_allowed_types(permissions: SVCPermissions) -> String {
        todo!("port NWWriter_DlrNavteq::getAllowedTypes")
    }

    /// C++: `int NWWriter_DlrNavteq::getSpeedCategory(int)` [static, private]
    /// get the navteq speed class based on the speed in km/h
    // int
    // NWWriter_DlrNavteq::getSpeedCategory(int kph) {
    //     if ((kph) > 130) {
    //         return 1;
    //     }
    //     if ((kph) > 100) {
    //         return 2;
    //     }
    //     if ((kph) > 90) {
    //         return 3;
    //     }
    //     if ((kph) > 70) {
    //         return 4;
    //     }
    //     if ((kph) > 50) {
    //         return 5;
    //     }
    //     if ((kph) > 30) {
    //         return 6;
    //     }
    //     if ((kph) > 10) {
    //         return 7;
    //     }
    //     return 8;
    // }
    #[allow(unused_variables)]
    pub fn get_speed_category(kph: i32) -> i32 {
        todo!("port NWWriter_DlrNavteq::getSpeedCategory")
    }

    /// C++: `int NWWriter_DlrNavteq::getSpeedCategoryUpperBound(int)` [static, private]
    /// get the SPEED_LIMIT as defined by elmar (upper bound of speed category)
    // int
    // NWWriter_DlrNavteq::getSpeedCategoryUpperBound(int kph) {
    //     if ((kph) > 130) {
    //         return 131;
    //     }
    //     if ((kph) > 100) {
    //         return 130;
    //     }
    //     if ((kph) > 90) {
    //         return 100;
    //     }
    //     if ((kph) > 70) {
    //         return 90;
    //     }
    //     if ((kph) > 50) {
    //         return 70;
    //     }
    //     if ((kph) > 30) {
    //         return 50;
    //     }
    //     if ((kph) > 10) {
    //         return 30;
    //     }
    //     return 10;
    // }
    #[allow(unused_variables)]
    pub fn get_speed_category_upper_bound(kph: i32) -> i32 {
        todo!("port NWWriter_DlrNavteq::getSpeedCategoryUpperBound")
    }

    /// C++: `int NWWriter_DlrNavteq::getNavteqLaneCode(const int)` [static, private]
    /// get the lane number encoding
    // int
    // NWWriter_DlrNavteq::getNavteqLaneCode(const int numLanes) {
    //     const int code = (numLanes == 1 ? 1 :
    //                       (numLanes < 4 ?  2 : 3));
    //     return numLanes * 10 + code;
    // }
    #[allow(unused_variables)]
    pub fn get_navteq_lane_code(numLanes: i32) -> i32 {
        todo!("port NWWriter_DlrNavteq::getNavteqLaneCode")
    }

    /// C++: `double NWWriter_DlrNavteq::getGraphLength(const NBEdge *const)` [static, private]
    /// get the length of the edge when measured up to the junction center
    // double
    // NWWriter_DlrNavteq::getGraphLength(const NBEdge* const edge) {
    //     PositionVector geom = edge->getGeometry();
    //     geom.push_back_noDoublePos(edge->getToNode()->getPosition());
    //     geom.push_front_noDoublePos(edge->getFromNode()->getPosition());
    //     return geom.length();
    // }
    #[allow(unused_variables)]
    pub fn get_graph_length(edge: Ptr<NBEdge>) -> f64 {
        todo!("port NWWriter_DlrNavteq::getGraphLength")
    }

    /// C++: `std::string NWWriter_DlrNavteq::getSinglePostalCode(const std::string &, const std::string)` [static, private]
    // std::string
    // NWWriter_DlrNavteq::getSinglePostalCode(const std::string& zipCode, const std::string edgeID) {
    //     // might be multiple codes
    //     if (zipCode.find_first_of(" ,;") != std::string::npos) {
    //         WRITE_WARNINGF("ambiguous zip code '%' for edge '%'. (using first value)", zipCode, edgeID);
    //         StringTokenizer st(zipCode, " ,;", true);
    //         std::vector<std::string> ret = st.getVector();
    //         return ret[0];
    //     } else if (zipCode.size() > 16) {
    //         WRITE_WARNINGF("long zip code '%' for edge '%'", zipCode, edgeID);
    //     }
    //     return zipCode;
    // }
    #[allow(unused_variables)]
    pub fn get_single_postal_code(zipCode: &str, edgeID: String) -> String {
        todo!("port NWWriter_DlrNavteq::getSinglePostalCode")
    }

    /// C++: `int NWWriter_DlrNavteq::speedInKph(double)` [static, private]
    /// get edge speed rounded to kmh
    // static inline int speedInKph(double metersPerSecond) {
    //         return (int)std::floor(metersPerSecond * 3.6 + 0.5);
    //     }
    #[allow(unused_variables)]
    pub fn speed_in_kph(metersPerSecond: f64) -> i32 {
        todo!("port NWWriter_DlrNavteq::speedInKph")
    }

    /// C++: `double NWWriter_DlrNavteq::getMaxLaneWidth(const NBEdge *const)` [static, private]
    /// get the maximum width of all lanes
    // double
    // NWWriter_DlrNavteq::getMaxLaneWidth(const NBEdge* const edge) {
    //     double result = 0;
    //     for (int i = 0; i < edge->getNumLanes(); i++) {
    //         result = MAX2(result, edge->getLaneWidth(i));
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_max_lane_width(edge: Ptr<NBEdge>) -> f64 {
        todo!("port NWWriter_DlrNavteq::getMaxLaneWidth")
    }

}

