//! Port of netwrite/NWWriter_DlrNavteq.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::msg_handler::write_warning;
use utils::common::string_utils::{starts_with, to_int};
use utils::iodevices::output_device::OutputDeviceRef;

/// C++ `OptionsCont` getters throw ProcessError; the options used here are always registered
fn or_panic<T, E: std::fmt::Display>(r: Result<T, E>) -> T {
    r.unwrap_or_else(|e| panic!("{e}"))
}

/// OptionsCont::getOptions().getString(name)
fn opt_str(oc: &OptionsCont, name: &str) -> String {
    or_panic(oc.get_string(name))
}

/// OutputDevice::getDevice(name)
fn get_device(name: &str) -> OutputDeviceRef {
    or_panic(OutputDevice::get_device(name, true))
}

/// GeoConvHelper::getFinal() (copied; the closure form cannot be held across the writers)
fn get_final_geo() -> GeoConvHelper {
    GeoConvHelper::with_final(|g| g.clone())
}

/// Port of utils/common/IDSupplier.{h,cpp} (no shared port exists yet)
struct IDSupplier {
    my_current: i64,
    my_prefix: String,
}

impl IDSupplier {
    fn new(prefix: &str, known_ids: &[String]) -> Self {
        let mut s = Self { my_current: 0, my_prefix: prefix.to_string() };
        for id in known_ids {
            s.avoid(id);
        }
        s
    }

    fn get_next(&mut self) -> String {
        let r = format!("{}{}", self.my_prefix, self.my_current);
        self.my_current += 1;
        r
    }

    fn avoid(&mut self, id: &str) {
        // does it start with prefix?
        if let Some(rest) = id.strip_prefix(self.my_prefix.as_str()) {
            // does it continue with a number? (istream >> long long)
            let t = rest.trim_start();
            let (sign, digits) = if let Some(r) = t.strip_prefix('-') {
                (-1i64, r)
            } else if let Some(r) = t.strip_prefix('+') {
                (1i64, r)
            } else {
                (1i64, t)
            };
            let n = digits.bytes().take_while(|b| b.is_ascii_digit()).count();
            if n > 0 {
                if let Ok(v) = digits[..n].parse::<i64>() {
                    let number = sign * v;
                    self.my_current = self.my_current.max(number + 1);
                }
            }
        }
    }
}

/// Exporter writing networks using XML (native input) format
/// C++ class `NWWriter_DlrNavteq` (netwrite/NWWriter_DlrNavteq.h)
pub struct NWWriter_DlrNavteq {
}

// C++ static members of NWWriter_DlrNavteq (port as module-level statics, see AGENTS.md):
//   static const std::string NWWriter_DlrNavteq::UNDEFINED;
const UNDEFINED: &str = "-1";

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
        if !or_panic(oc.is_set("dlr-navteq-output", true)) {
            return;
        }
        let mut internal_nodes: BTreeMap<Ptr<NBEdge>, String> = BTreeMap::new();
        Self::write_nodes_unsplitted(oc, &nb.myNodeCont, &nb.myEdgeCont, &mut internal_nodes);
        Self::write_links_unsplitted(oc, &nb.myEdgeCont, &internal_nodes);
        Self::write_traffic_signals(oc, &mut nb.myNodeCont);
        Self::write_prohibited_manoeuvres(oc, &nb.myNodeCont, &nb.myEdgeCont);
        Self::write_connected_lanes(oc, &mut nb.myNodeCont);
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
        let mut type_ = edge.get_type_id().to_string();
        if starts_with(&type_, "highway.") {
            type_ = type_[8..].to_string();
        }
        if starts_with(&type_, "motorway") {
            return 0;
        } else if starts_with(&type_, "trunk") {
            return 1;
        } else if starts_with(&type_, "primary") {
            return 1;
        } else if starts_with(&type_, "secondary") {
            return 2;
        } else if starts_with(&type_, "tertiary") {
            return 3;
        } else if type_ == "unclassified" {
            return 3;
        } else if type_ == "living_street" || type_ == "residential" || type_ == "road" || type_ == "service" || type_ == "track" || type_ == "cycleway" || type_ == "path" || type_ == "footway" {
            return 4;
        }
        let kph = Self::speed_in_kph(edge.get_speed());
        if kph > 100 {
            return 0;
        }
        if kph > 70 {
            return 1;
        }
        if kph > 50 {
            return if edge.get_num_lanes() > 1 { 2 } else { 3 };
        }
        if kph > 30 {
            return 3;
        }
        4
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
        if edge.base2.has_parameter("bridge") {
            return 1;
        } else if edge.base2.has_parameter("tunnel") {
            return 4;
        } else if edge.get_type_id() == "route.ferry" {
            return 10;
        }
        -1 // UNDEFINED
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
        if edge.get_permissions(-1) == SVC_PEDESTRIAN {
            return 15;
        } else if edge.get_junction_priority(edge.get_to_node()) == NBEdge_JunctionPriority::ROUNDABOUT as i32 {
            return 4;
        } else if edge.get_type_id() == "highway.service" {
            return 14;
        } else if edge.get_type_id().contains("_link") {
            return 10;
        }
        3 // speed category 1-8;
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
        let kph = Self::speed_in_kph(edge.get_speed());
        device.write_attr("speedCategory", &Self::get_speed_category(kph));
        device.write_attr("speedRestriction", &Self::get_speed_category_upper_bound(kph));
        device.write_attr("maxWidth", &Self::get_max_lane_width(edge));
        device.write_attr("FRC", &Self::get_road_class(edge));
        device.write_attr("vehicleTypeCode", &Self::get_allowed_types(edge.get_permissions(-1)));
        device.write_attr("formOfWay", &Self::get_form_of_way(edge));
        device.write_attr("isRamp", &edge.is_off_ramp());
        device.write_attr("brunnelType", &Self::get_brunnel_type(edge));
        device.write_attr("postalCode", &Self::get_single_postal_code(&edge.base2.get_parameter("postal_code", UNDEFINED), edge.get_id().to_string()));
        device.write_attr("surface", &edge.base2.get_parameter("surface", UNDEFINED));
    }

    /// C++: `void NWWriter_DlrNavteq::writeConnectionAttrs(OutputDevice &, const NBEdge::Connection &)` [static]
    // void
    // NWWriter_DlrNavteq::writeConnectionAttrs(OutputDevice& device, const NBEdge::Connection& c) {
    //     device.writeAttr(SUMO_ATTR_NODE, c.toEdge->getFromNode()->getID());
    //     device.writeAttr("vehicleTypeCode", getAllowedTypes(c.permissions));
    // }
    #[allow(unused_variables)]
    pub fn write_connection_attrs(device: &mut OutputDevice, c: &NBEdge_Connection) {
        device.write_attr(SUMO_ATTR_NODE, c.toEdge.get_from_node().get_id());
        device.write_attr("vehicleTypeCode", &Self::get_allowed_types(c.permissions));
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
        for n in nc.myNodes.values() {
            let incoming = n.get_incoming_edges();
            let outgoing = n.get_outgoing_edges();
            for in_edge in incoming {
                let in_perm = in_edge.get_permissions(-1);
                for out_edge in outgoing {
                    let out_perm = out_edge.get_permissions(-1);
                    let common_perm = in_perm & out_perm;
                    if common_perm != 0 && common_perm != SVC_PEDESTRIAN && !in_edge.is_connected_to(*out_edge, false) {
                        device.open_tag_id(SUMO_TAG_PROHIBITED_CONNECTION);
                        device.write_attr(SUMO_ATTR_FROM, in_edge.get_id());
                        device.write_attr(SUMO_ATTR_TO, out_edge.get_id());
                        device.close_tag("");
                    }
                }
            }
        }
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
        let device_ref = get_device(&format!("{}_nodes_unsplitted.txt", opt_str(oc, "dlr-navteq-output")));
        let mut device = device_ref.borrow_mut();
        Self::write_header(&mut device, oc);
        let gch = get_final_geo();
        let have_geo = gch.using_geo_projection();
        let geo_scale = 10.0f64.powf(if have_geo { 5.0 } else { 2.0 }); // see NIImporter_DlrNavteq::GEO_SCALE
        device.set_precision(or_panic(oc.get_int("dlr-navteq.precision")));
        if !have_geo {
            write_warning("DlrNavteq node data will be written in (floating point) cartesian coordinates");
        }
        // write format specifier
        device.write("# NODE_ID\tIS_BETWEEN_NODE\tamount_of_geocoordinates\tx1\ty1\t[x2 y2  ... xn  yn]\n");
        // write header
        let boundary = gch.get_conv_boundary();
        let mut min = Position::new(boundary.xmin(), boundary.ymin());
        let mut max = Position::new(boundary.xmax(), boundary.ymax());
        gch.cartesian2geo(&mut min);
        min.mul(geo_scale);
        gch.cartesian2geo(&mut max);
        max.mul(geo_scale);
        let mut multinodes: i32 = 0;
        for e in ec.myEdges.values() {
            if e.get_geometry().len() > 2 {
                multinodes += 1;
            }
        }
        device.write("# [xmin_region] ").write(&min.x()).write("\n");
        device.write("# [xmax_region] ").write(&max.x()).write("\n");
        device.write("# [ymin_region] ").write(&min.y()).write("\n");
        device.write("# [ymax_region] ").write(&max.y()).write("\n");
        device.write("# [elements_multinode] ").write(&multinodes).write("\n");
        device.write("# [elements_normalnode] ").write(&nc.size()).write("\n");
        device.write("# [xmin] ").write(&min.x()).write("\n");
        device.write("# [xmax] ").write(&max.x()).write("\n");
        device.write("# [ymin] ").write(&min.y()).write("\n");
        device.write("# [ymax] ").write(&max.y()).write("\n");
        // write normal nodes
        for n in nc.myNodes.values() {
            let mut pos = *n.get_position();
            gch.cartesian2geo(&mut pos);
            pos.mul(geo_scale);
            device.write(n.get_id()).write("\t0\t1\t").write(&pos.x()).write("\t").write(&pos.y()).write("\n");
        }
        // write "internal" nodes
        let numerical_ids = or_panic(oc.get_bool("numerical-ids"));
        let mut reserved_node_ids: BTreeSet<String> = BTreeSet::new();
        if or_panic(oc.is_set("reserved-ids", true)) {
            NBHelpers::load_prefixed_i_ds_fom_file(&or_panic(oc.get_string("reserved-ids")), "node:".to_string(), &mut reserved_node_ids); // backward compatibility
            NBHelpers::load_prefixed_i_ds_fom_file(&or_panic(oc.get_string("reserved-ids")), "junction:".to_string(), &mut reserved_node_ids); // selection format
        }
        let avoid: Vec<String> = if numerical_ids {
            let mut a = nc.get_all_names();
            a.extend(ec.get_all_names());
            a.extend(reserved_node_ids.iter().cloned());
            a
        } else {
            Vec::new()
        };
        let mut id_supplier = IDSupplier::new("", &avoid);
        for edge_it in ec.myEdges.values() {
            let e = edge_it;
            let mut geom: PositionVector = e.get_geometry().clone();
            if geom.len() > 2 {
                // the import NIImporter_DlrNavteq checks for the presence of a
                // negated edge id to determine spread type. We may need to do some
                // shifting to make this consistent
                let has_opposite_id = !ec.get_opposite_by_id(e.get_id()).is_null();
                // C++ try/catch dropped: PositionVector::move2side(double) does not throw
                if e.get_lane_spread_function() == LaneSpreadFunction::RIGHT && !has_opposite_id {
                    // need to write center-line geometry instead
                    geom.move2side(e.get_total_width() / 2.0, 100.0);
                } else if e.get_lane_spread_function() == LaneSpreadFunction::CENTER && has_opposite_id {
                    // need to write left-border geometry instead
                    geom.move2side(-e.get_total_width() / 2.0, 100.0);
                }

                if geom.len() > 2 { // move2side might have changed the number of nodes
                    let mut internal_node_id = e.get_id().to_string();
                    if internal_node_id == UNDEFINED
                        || !nc.retrieve(&internal_node_id).is_null()
                        || reserved_node_ids.contains(&internal_node_id)
                    {
                        // need to invent a new name to avoid clashing with the id of a 'real' node or a reserved name
                        if numerical_ids {
                            internal_node_id = id_supplier.get_next();
                        } else {
                            internal_node_id += "_geometry";
                        }
                    }
                    internalNodes.insert(*e, internal_node_id.clone());
                    device.write(&internal_node_id).write("\t1\t").write(&(geom.len() - 2));
                    for ii in 1..geom.len() - 1 {
                        let mut pos = geom[ii];
                        gch.cartesian2geo(&mut pos);
                        pos.mul(geo_scale);
                        device.write("\t").write(&pos.x()).write("\t").write(&pos.y());
                    }
                    device.write("\n");
                }
            }
        }
        or_panic(device.close());
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
        let major_version = or_panic(to_int(&or_panic(StringTokenizer::with_token(&opt_str(oc, "dlr-navteq.version"), ".", false).next())));
        let mut name_ids: BTreeMap<String, String> = BTreeMap::new();
        let device_ref = get_device(&format!("{}_links_unsplitted.txt", opt_str(oc, "dlr-navteq-output")));
        let mut device = device_ref.borrow_mut();
        Self::write_header(&mut device, oc);
        // write format specifier
        device.write("# LINK_ID\tNODE_ID_FROM\tNODE_ID_TO\tBETWEEN_NODE_ID\tLENGTH\tVEHICLE_TYPE\tFORM_OF_WAY\tBRUNNEL_TYPE\t")
            .write("FUNCTIONAL_ROAD_CLASS\tSPEED_CATEGORY\tNUMBER_OF_LANES\tSPEED_LIMIT\tSPEED_RESTRICTION\t")
            .write("NAME_ID1_REGIONAL\tNAME_ID2_LOCAL\tHOUSENUMBERS_RIGHT\tHOUSENUMBERS_LEFT\tZIP_CODE\t")
            .write("AREA_ID\tSUBAREA_ID\tTHROUGH_TRAFFIC\tSPECIAL_RESTRICTIONS\tEXTENDED_NUMBER_OF_LANES\tISRAMP\tCONNECTION");
        if major_version > 6 {
            device.write("\tMAXHEIGHT\tMAXWIDTH\tMAXWEIGHT\tSURFACE");
        }
        device.write("\n");
        // write edges
        for e in ec.myEdges.values() {
            let kph = Self::speed_in_kph(e.get_speed());
            let between_node_id: String = match internalNodes.get(e) {
                Some(id) => id.clone(),
                None => UNDEFINED.to_string(),
            };
            let mut name_id = UNDEFINED.to_string();
            let mut name_id_regional = UNDEFINED.to_string();
            if or_panic(oc.get_bool("output.street-names")) {
                let name = e.get_street_name().to_string();
                if !name.is_empty() {
                    if !name_ids.contains_key(&name) {
                        let tmp = name_ids.len() as i32;
                        name_ids.insert(name.clone(), tmp.to_string());
                    }
                    name_id = name_ids[&name].clone();
                }
                let name2 = e.base2.get_parameter("ref", "");
                if !name2.is_empty() {
                    if !name_ids.contains_key(&name2) {
                        let tmp = name_ids.len() as i32;
                        name_ids.insert(name2.clone(), tmp.to_string());
                    }
                    name_id_regional = name_ids[&name2].clone();
                }
            }
            device.write(e.get_id()).write("\t")
                .write(e.get_from_node().get_id()).write("\t")
                .write(e.get_to_node().get_id()).write("\t")
                .write(between_node_id.as_str()).write("\t")
                .write(&Self::get_graph_length(*e)).write("\t")
                .write(&Self::get_allowed_types(e.get_permissions(-1))).write("\t")
                .write(&Self::get_form_of_way(*e)).write("\t")
                .write(&Self::get_brunnel_type(*e)).write("\t")
                .write(&Self::get_road_class(*e)).write("\t")
                .write(&Self::get_speed_category(kph)).write("\t")
                .write(&Self::get_navteq_lane_code(e.get_num_lanes())).write("\t")
                .write(&Self::get_speed_category_upper_bound(kph)).write("\t")
                .write(&kph).write("\t")
                .write(name_id_regional.as_str()).write("\t")
                .write(name_id.as_str()).write("\t") // NAME_ID2_LOCAL
                .write(UNDEFINED).write("\t") // housenumbers_right
                .write(UNDEFINED).write("\t") // housenumbers_left
                .write(&Self::get_single_postal_code(&e.base2.get_parameter("postal_code", UNDEFINED), e.get_id().to_string())).write("\t") // ZIP_CODE
                .write(UNDEFINED).write("\t") // AREA_ID
                .write(UNDEFINED).write("\t") // SUBAREA_ID
                .write("1\t") // through_traffic (allowed)
                .write(UNDEFINED).write("\t") // special_restrictions
                .write(UNDEFINED).write("\t") // extended_number_of_lanes
                .write(UNDEFINED).write("\t") // isRamp
                .write("0"); // connection (between nodes always in order)
            if major_version > 6 {
                device.write("\t").write(&e.base2.get_parameter("maxheight", UNDEFINED))
                    .write("\t").write(&e.base2.get_parameter("maxwidth", UNDEFINED))
                    .write("\t").write(&e.base2.get_parameter("maxweight", UNDEFINED))
                    .write("\t").write(&e.base2.get_parameter("surface", UNDEFINED));
            }
            device.write("\n");
        }
        if or_panic(oc.get_bool("output.street-names")) {
            let names_device_ref = get_device(&format!("{}_names.txt", opt_str(oc, "dlr-navteq-output")));
            let mut names_device = names_device_ref.borrow_mut();
            Self::write_header(&mut names_device, oc);
            // write format specifier
            names_device.write("# NAME_ID\tPERMANENT_ID_INFO\tName\n");
            names_device.write("# [elements] ").write(&name_ids.len()).write("\n");
            for (name, id) in &name_ids {
                names_device.write(id.as_str()).write("\t").write(&0).write("\t").write(name.as_str()).write("\n");
            }
            or_panic(names_device.close());
        }
        or_panic(device.close());
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
        let device_ref = get_device(&format!("{}_traffic_signals.txt", opt_str(oc, "dlr-navteq-output")));
        let mut device = device_ref.borrow_mut();
        Self::write_header(&mut device, oc);
        let gch = get_final_geo();
        let have_geo = gch.using_geo_projection();
        let geo_scale = 10.0f64.powf(if have_geo { 5.0 } else { 2.0 }); // see NIImporter_DlrNavteq::GEO_SCALE
        device.set_precision(or_panic(oc.get_int("dlr-navteq.precision")));
        // write format specifier
        device.write("#Traffic signal related to LINK_ID and NODE_ID with location relative to driving direction.\n#column format like pointcollection.\n#DESCRIPTION->LOCATION: 1-rechts von LINK; 2-links von LINK; 3-oberhalb LINK -1-keineAngabe\n#RELATREC_ID\tPOICOL_TYPE\tDESCRIPTION\tLONGITUDE\tLATITUDE\tLINK_ID\n");
        // write record for every edge incoming to a tls controlled node
        for n in nc.myNodes.values() {
            if n.is_tl_controlled() {
                let mut pos = *n.get_position();
                gch.cartesian2geo(&mut pos);
                pos.mul(geo_scale);
                for e in n.get_incoming_edges() {
                    device.write(e.get_id()).write("\t")
                        .write("12\t") // POICOL_TYPE
                        .write("LSA;NODEIDS#").write(n.get_id()).write("#;LOCATION#-1#;\t")
                        .write(&pos.x()).write("\t")
                        .write(&pos.y()).write("\t")
                        .write(e.get_id()).write("\n");
                }
            }
        }
        or_panic(device.close());
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
        let device_ref = get_device(&format!("{}_prohibited_manoeuvres.txt", opt_str(oc, "dlr-navteq-output")));
        let mut device = device_ref.borrow_mut();
        Self::write_header(&mut device, oc);
        // need to invent id for relation
        let mut reserved_rel_ids: BTreeSet<String> = BTreeSet::new();
        if or_panic(oc.is_set("reserved-ids", true)) {
            NBHelpers::load_prefixed_i_ds_fom_file(&or_panic(oc.get_string("reserved-ids")), "rel:".to_string(), &mut reserved_rel_ids);
        }
        let mut avoid: Vec<String> = ec.get_all_names(); // already used for tls RELATREC_ID
        avoid.extend(reserved_rel_ids.iter().cloned());
        let mut id_supplier = IDSupplier::new("", &avoid); // @note: use a global relRecIDsupplier if this is used more often
        // write format specifier
        device.write("#No driving allowed from ID1 to ID2 or the complete chain from ID1 to IDn\n");
        device.write("#RELATREC_ID\tPERMANENT_ID_INFO\tVALIDITY_PERIOD\tTHROUGH_TRAFFIC\tVEHICLE_TYPE\tNAVTEQ_LINK_ID1\t[NAVTEQ_LINK_ID2 ...]\n");
        // write record for every pair of incoming/outgoing edge that are not connected despite having common permissions
        for n in nc.myNodes.values() {
            let incoming = n.get_incoming_edges();
            let outgoing = n.get_outgoing_edges();
            for in_edge in incoming {
                let in_perm = in_edge.get_permissions(-1);
                for out_edge in outgoing {
                    let out_perm = out_edge.get_permissions(-1);
                    let common_perm = in_perm & out_perm;
                    if common_perm != 0 && common_perm != SVC_PEDESTRIAN && !in_edge.is_connected_to(*out_edge, false) {
                        device.write(&id_supplier.get_next()).write("\t")
                            .write(&1).write("\t") // permanent id
                            .write(UNDEFINED).write("\t")
                            .write(&1).write("\t")
                            .write(&Self::get_allowed_types(SVC_ALL)).write("\t")
                            .write(in_edge.get_id()).write("\t").write(out_edge.get_id()).write("\n");
                    }
                }
            }
        }
        or_panic(device.close());
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
        let device_ref = get_device(&format!("{}_connected_lanes.txt", opt_str(oc, "dlr-navteq-output")));
        let mut device = device_ref.borrow_mut();
        Self::write_header(&mut device, oc);
        // write format specifier
        device.write("#Lane connections related to LINK-IDs and NODE-ID.\n");
        device.write("#column format like pointcollection.\n");
        device.write("#NODE-ID\tVEHICLE-TYPE\tFROM_LANE\tTO_LANE\tTHROUGH_TRAFFIC\tLINK_IDs[2..*]\n");
        // write record for every connection
        for n in nc.myNodes.values() {
            for from in n.get_incoming_edges() {
                let from_perm = from.get_permissions(-1);
                for c in from.get_connections() {
                    device.write(n.get_id()).write("\t")
                        .write(&Self::get_allowed_types(from_perm & c.toEdge.get_permissions(-1))).write("\t")
                        .write(&(c.fromLane + 1)).write("\t") // one-based
                        .write(&(c.toLane + 1)).write("\t") // one-based
                        .write(&1).write("\t") // no information regarding permissibility of through traffic
                        .write(from.get_id()).write("\t")
                        .write(c.toEdge.get_id()).write("\t")
                        .write("\n");
                }
            }
        }
        or_panic(device.close());
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
        device.write("# Format matches Extraction version: V").write(&opt_str(oc, "dlr-navteq.version")).write(" \n");
        let mut tmp = String::new();
        or_panic(oc.write_configuration(&mut tmp, true, false, false, "", false, false, ""));
        // emulates the std::getline loop: a trailing newline yields one more (empty) line
        let mut remaining: &str = &tmp;
        let mut eof = false;
        while !eof {
            let line = match remaining.find('\n') {
                Some(i) => {
                    let l = &remaining[..i];
                    remaining = &remaining[i + 1..];
                    l
                }
                None => {
                    eof = true;
                    let l = remaining;
                    remaining = "";
                    l
                }
            };
            device.write("# ").write(line).write("\n");
        }
        device.write("#\n");
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
        if permissions == SVC_ALL {
            return "100000000000".to_string();
        }
        let bit = |m: SVCPermissions| -> &'static str { if (permissions & m) > 0 { "1" } else { "0" } };
        let mut oss = String::from("0");
        oss += bit(SVC_PASSENGER);
        oss += bit(SVC_PASSENGER); // residential
        oss += bit(SVC_HOV);
        oss += bit(SVC_EMERGENCY);
        oss += bit(SVC_TAXI);
        oss += bit(SVC_BUS | SVC_COACH);
        oss += bit(SVC_DELIVERY);
        oss += bit(SVC_TRUCK | SVC_TRAILER);
        oss += bit(SVC_MOTORCYCLE);
        oss += bit(SVC_BICYCLE);
        oss += bit(SVC_PEDESTRIAN);
        oss
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
        if kph > 130 {
            return 1;
        }
        if kph > 100 {
            return 2;
        }
        if kph > 90 {
            return 3;
        }
        if kph > 70 {
            return 4;
        }
        if kph > 50 {
            return 5;
        }
        if kph > 30 {
            return 6;
        }
        if kph > 10 {
            return 7;
        }
        8
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
        if kph > 130 {
            return 131;
        }
        if kph > 100 {
            return 130;
        }
        if kph > 90 {
            return 100;
        }
        if kph > 70 {
            return 90;
        }
        if kph > 50 {
            return 70;
        }
        if kph > 30 {
            return 50;
        }
        if kph > 10 {
            return 30;
        }
        10
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
        let code = if numLanes == 1 {
            1
        } else if numLanes < 4 {
            2
        } else {
            3
        };
        numLanes * 10 + code
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
        let mut geom = edge.get_geometry().clone();
        geom.push_back_no_double_pos(edge.get_to_node().get_position());
        geom.push_front_no_double_pos(edge.get_from_node().get_position());
        geom.length()
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
        // might be multiple codes
        if zipCode.contains([' ', ',', ';']) {
            write_warningf!("ambiguous zip code '%' for edge '%'. (using first value)", zipCode.to_string(), edgeID.clone());
            let mut st = StringTokenizer::with_token(zipCode, " ,;", true);
            let ret = st.get_vector();
            return ret[0].clone();
        } else if zipCode.len() > 16 {
            write_warningf!("long zip code '%' for edge '%'", zipCode.to_string(), edgeID.clone());
        }
        zipCode.to_string()
    }

    /// C++: `int NWWriter_DlrNavteq::speedInKph(double)` [static, private]
    /// get edge speed rounded to kmh
    // static inline int speedInKph(double metersPerSecond) {
    //         return (int)std::floor(metersPerSecond * 3.6 + 0.5);
    //     }
    #[allow(unused_variables)]
    pub fn speed_in_kph(metersPerSecond: f64) -> i32 {
        (metersPerSecond * 3.6 + 0.5).floor() as i32
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
        let mut result: f64 = 0.0;
        for i in 0..edge.get_num_lanes() {
            let w = edge.get_lane_width_2(i);
            result = if result > w { result } else { w };
        }
        result
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allowed_types_bit_vector() {
        assert_eq!(NWWriter_DlrNavteq::get_allowed_types(SVC_ALL), "100000000000");
        assert_eq!(NWWriter_DlrNavteq::get_allowed_types(SVC_PASSENGER), "011000000000");
        assert_eq!(NWWriter_DlrNavteq::get_allowed_types(SVC_PASSENGER | SVC_PEDESTRIAN), "011000000001");
    }

    #[test]
    fn speed_categories_and_lane_code() {
        assert_eq!(NWWriter_DlrNavteq::get_speed_category(131), 1);
        assert_eq!(NWWriter_DlrNavteq::get_speed_category(100), 3);
        assert_eq!(NWWriter_DlrNavteq::get_speed_category(10), 8);
        assert_eq!(NWWriter_DlrNavteq::get_speed_category_upper_bound(100), 100);
        assert_eq!(NWWriter_DlrNavteq::get_navteq_lane_code(1), 11);
        assert_eq!(NWWriter_DlrNavteq::get_navteq_lane_code(3), 32);
        assert_eq!(NWWriter_DlrNavteq::get_navteq_lane_code(4), 43);
    }

    #[test]
    fn speed_in_kph_rounds_half_up() {
        assert_eq!(NWWriter_DlrNavteq::speed_in_kph(13.888), 50);
        assert_eq!(NWWriter_DlrNavteq::speed_in_kph(0.0), 0);
    }

    #[test]
    fn id_supplier_skips_known_ids() {
        let mut s = IDSupplier::new("id", &["id5".to_string(), "idx".to_string(), "id2x".to_string()]);
        assert_eq!(s.get_next(), "id6");
        assert_eq!(s.get_next(), "id7");
    }
}

