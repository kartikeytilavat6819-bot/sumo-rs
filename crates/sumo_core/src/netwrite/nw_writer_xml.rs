//! Port of netwrite/NWWriter_XML.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::msg_handler::write_warning;
use utils::common::std_defs::{g_precision, g_precision_geo, NETWORK_VERSION};
use utils::common::string_utils::escape_xml;
use utils::common::sumo_vehicle_class::{get_vehicle_class_names, write_permissions, SVCPermissions, SVC_ALL, SVC_IGNORING, SVC_PEDESTRIAN, SVC_UNSPECIFIED};
use utils::common::to_string::{join_to_string, mm_version_to_string};
use utils::iodevices::output_device::OutputDeviceRef;
use utils::xml::sumo_xml_definitions::{LaneSpreadFunctions, TrafficLightTypes};

// NBNode::UNSPECIFIED_RADIUS and NBEdge::UNSPECIFIED_* (C++ static members, NBNode.cpp / NBEdge.cpp)
const NBNODE_UNSPECIFIED_RADIUS: f64 = -1.0;
const NBEDGE_UNSPECIFIED_WIDTH: f64 = -1.0;
const NBEDGE_UNSPECIFIED_OFFSET: f64 = 0.0;
const NBEDGE_UNSPECIFIED_SPEED: f64 = -1.0;
const NBEDGE_UNSPECIFIED_FRICTION: f64 = 1.0;
const NBEDGE_UNSPECIFIED_CONTPOS: f64 = -1.0;
const NBEDGE_UNSPECIFIED_LOADED_LENGTH: f64 = -1.0;

/// C++ `OptionsCont` getters throw ProcessError; the options used here are always registered
fn or_panic<T, E: std::fmt::Display>(r: Result<T, E>) -> T {
    // C++ getters throw ProcessError; keep that payload type so netconvert's
    // panic hook suppresses the Rust-only "thread panicked at ..." line.
    r.unwrap_or_else(|e| std::panic::panic_any(utils::common::util_exceptions::ProcessError::new(e.to_string())))
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

/// GeoConvHelper::writeLocation(OutputDevice&)
pub fn write_location(into: &mut OutputDevice) {
    get_final_geo().write_location(into);
}

/// writePreferences(OutputDevice&, SVCPermissions) (utils/common/SUMOVehicleClass.cpp)
fn write_preferences(into: &mut OutputDevice, preferred: SVCPermissions) {
    utils::common::sumo_vehicle_class::write_preferences(into, preferred);
}

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
        // check whether plain-output files shall be generated
        if prefix != "" {
            let haveTypes = nb.myTypeCont.size() > 0;
            Self::write_nodes(oc, prefix, &mut nb.myNodeCont);
            if haveTypes {
                Self::write_types(prefix, &mut nb.myEdgeCont, &mut nb.myTypeCont);
            }
            Self::write_edges_and_connections(oc, prefix, &mut nb.myNodeCont, &mut nb.myEdgeCont);
            Self::write_traffic_lights(prefix, &mut nb.myTLLCont, &mut nb.myEdgeCont);
            Self::write_config(oc, prefix, haveTypes);
        }
        if or_panic(oc.is_set("junctions.join-output", true)) && opt_str(oc, "junctions.join-output").len() > 0 {
            Self::write_joined_junctions(&opt_str(oc, "junctions.join-output"), &mut nb.myNodeCont);
        }
        if or_panic(oc.is_set("street-sign-output", true)) && opt_str(oc, "street-sign-output").len() > 0 {
            Self::write_street_signs(oc, &mut nb.myEdgeCont);
        }
        if oc.exists("ptstop-output") && or_panic(oc.is_set("ptstop-output", true)) && opt_str(oc, "ptstop-output").len() > 0 {
            Self::write_pt_stops(oc, &mut nb.myPTStopCont);
        }
        if oc.exists("ptline-output") && or_panic(oc.is_set("ptline-output", true)) && opt_str(oc, "ptline-output").len() > 0 {
            Self::write_pt_lines(oc, &mut nb.myPTLineCont);
        }
        if oc.exists("parking-output") && or_panic(oc.is_set("parking-output", true)) && opt_str(oc, "parking-output").len() > 0 {
            Self::write_parking_areas(oc, &mut nb.myParkingCont, &mut nb.myEdgeCont);
        }
        if oc.exists("taz-output") && or_panic(oc.is_set("taz-output", true)) && opt_str(oc, "taz-output").len() > 0 {
            Self::write_districts(oc, &mut nb.myDistrictCont);
        }
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
        let mut attrs: BTreeMap<SumoXMLAttr, String> = BTreeMap::new();
        attrs.insert(SUMO_ATTR_VERSION, mm_version_to_string(NETWORK_VERSION));
        let device_ref = get_device(filename);
        let mut device = device_ref.borrow_mut();
        device.write_xml_header("nodes", "nodes_file.xsd", attrs, true);
        for joined in nc.get_joined_clusters() {
            assert!(!joined.is_empty());
            device.open_tag_id(SUMO_TAG_JOIN).write_attr(SUMO_ATTR_NODES, &join_to_string(joined, " ")).close_tag("");
        }
        or_panic(device.close());
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
        let device_ref = get_device(&opt_str(oc, "street-sign-output"));
        let mut device = device_ref.borrow_mut();
        device.write_xml_header("additional", "additional_file.xsd", BTreeMap::new(), true);
        for e in ec.myEdges.values() {
            for sign in e.get_signs() {
                sign.write_as_poi(&mut device, *e);
            }
        }
        or_panic(device.close());
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
        if !oc.exists("node-files") {
            // do not write configuration for netgen
            return;
        }
        let ext = opt_str(oc, "output.format");
        let mut tmp = oc.clone_options();
        or_panic(tmp.set("node-files", &format!("{prefix}.nod.{ext}"), false));
        or_panic(tmp.set("edge-files", &format!("{prefix}.edg.{ext}"), false));
        or_panic(tmp.set("connection-files", &format!("{prefix}.con.{ext}"), false));
        or_panic(tmp.set("tllogic-files", &format!("{prefix}.tll.{ext}"), false));
        if haveTypes {
            or_panic(tmp.set("type-files", &format!("{prefix}.typ.{ext}"), false));
        }
        or_panic(tmp.set_default("sumo-net-file", ""));
        or_panic(tmp.set_default("plain-output-prefix", ""));

        let configPath = format!("{prefix}.netccfg");
        let mut out = std::fs::File::create(&configPath).unwrap_or_else(|_| panic!("Could not save configuration to '{configPath}'"));
        let mut os = String::new();
        or_panic(tmp.write_configuration(&mut os, true, false, false, "", false, false, ""));
        std::io::Write::write_all(&mut out, os.as_bytes()).unwrap_or_else(|e| panic!("{e}"));
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
        let ext = opt_str(oc, "output.format");
        let gch = get_final_geo();
        let mut useGeo = oc.exists("proj.plain-geo") && or_panic(oc.get_bool("proj.plain-geo"));
        if useGeo && !gch.using_geo_projection() {
            write_warning("Ignoring option \"proj.plain-geo\" because no geo-conversion has been defined");
            useGeo = false;
        }
        let geoAccuracy = useGeo || gch.using_inverse_geo_projection();

        let device_ref = get_device(&format!("{prefix}.nod.{ext}"));
        let mut device = device_ref.borrow_mut();
        let mut attrs: BTreeMap<SumoXMLAttr, String> = BTreeMap::new();
        attrs.insert(SUMO_ATTR_VERSION, mm_version_to_string(NETWORK_VERSION));
        device.write_xml_header("nodes", "nodes_file.xsd", attrs, true);

        // write network offsets and projection to allow reconstruction of original coordinates
        if !useGeo {
            write_location(&mut device);
        }

        // write nodes
        let tlsDefaultType = or_panic(TrafficLightTypes.get(&opt_str(oc, "tls.default-type")));
        for n in nc.myNodes.values() {
            device.open_tag_id(SUMO_TAG_NODE);
            device.write_attr(SUMO_ATTR_ID, n.get_id());
            // write position
            let mut pos: Position = n.get_position().clone();
            if useGeo {
                gch.cartesian2geo(&mut pos);
            }
            if geoAccuracy {
                device.set_precision(g_precision_geo());
            }
            NWFrame::write_position_long(&pos, &mut device);
            if geoAccuracy {
                device.set_precision(g_precision());
            }

            device.write_attr(SUMO_ATTR_TYPE, &n.get_type());
            // std::set is already sorted, so the output is stable
            let mut tlsIDs: BTreeSet<String> = BTreeSet::new();
            let mut controlledInnerEdges: BTreeSet<String> = BTreeSet::new();
            let mut tlType = String::new();
            for tl in n.get_controlling_tls() {
                tlsIDs.insert(tl.get_id().to_string());
                controlledInnerEdges.extend(tl.get_controlled_inner_edges());
                if tl.get_type() != tlsDefaultType {
                    tlType = TrafficLightTypes.get_string(tl.get_type()).to_string();
                }
            }
            device.write_optional_attr(SUMO_ATTR_TLID, &join_to_string(&tlsIDs, " "), tlsIDs.is_empty());
            device.write_optional_attr(SUMO_ATTR_TLTYPE, &tlType, tlType == "");
            device.write_optional_attr(SUMO_ATTR_CONTROLLED_INNER, &join_to_string(&controlledInnerEdges, " "), controlledInnerEdges.is_empty());
            NWWriter_SUMO::write_shape(&mut device, &gch, n.get_shape().clone(), SUMO_ATTR_SHAPE, useGeo, geoAccuracy, n.has_custom_shape());
            device.write_optional_attr(SUMO_ATTR_RADIUS, &n.get_radius(), n.get_radius() == NBNODE_UNSPECIFIED_RADIUS);
            device.write_optional_attr::<bool>(SUMO_ATTR_KEEP_CLEAR, &n.get_keep_clear(), n.get_keep_clear());
            device.write_optional_attr(SUMO_ATTR_RIGHT_OF_WAY, &n.get_right_of_way(), n.get_right_of_way() == RightOfWay::DEFAULT);
            device.write_optional_attr(SUMO_ATTR_FRINGE, &n.get_fringe_type(), n.get_fringe_type() == FringeType::DEFAULT);
            device.write_optional_attr(SUMO_ATTR_ROUNDABOUT, &n.get_roundabout_type(), n.get_roundabout_type() == RoundaboutType::DEFAULT);
            device.write_optional_attr(SUMO_ATTR_NAME, &escape_xml(n.get_name(), false), n.get_name() == "");
            n.base2.write_params(&mut device);
            device.close_tag("");
        }
        or_panic(device.close());
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
        let ext = utils::options::options_cont::get_string("output.format");
        let device_ref = get_device(&format!("{prefix}.typ.{ext}"));
        let mut device = device_ref.borrow_mut();
        let mut attrs: BTreeMap<SumoXMLAttr, String> = BTreeMap::new();
        attrs.insert(SUMO_ATTR_VERSION, mm_version_to_string(NETWORK_VERSION));
        device.write_xml_header("types", "types_file.xsd", attrs, true);
        let usedTypes = ec.get_used_types();
        tc.write_edge_types(&mut device, &usedTypes);
        or_panic(device.close());
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
        let ext = opt_str(oc, "output.format");
        let gch = get_final_geo();
        let mut useGeo = oc.exists("proj.plain-geo") && or_panic(oc.get_bool("proj.plain-geo"));
        let geoAccuracy = useGeo || gch.using_inverse_geo_projection();
        let dlrNavteq = oc.exists("dlr-navteq.plain") && or_panic(oc.get_bool("dlr-navteq.plain"));

        let mut attrs: BTreeMap<SumoXMLAttr, String> = BTreeMap::new();
        attrs.insert(SUMO_ATTR_VERSION, mm_version_to_string(NETWORK_VERSION));
        let edevice_ref = get_device(&format!("{prefix}.edg.{ext}"));
        let mut edevice = edevice_ref.borrow_mut();
        edevice.write_xml_header("edges", "edges_file.xsd", attrs.clone(), true);
        let cdevice_ref = get_device(&format!("{prefix}.con.{ext}"));
        let mut cdevice = cdevice_ref.borrow_mut();
        cdevice.write_xml_header("connections", "connections_file.xsd", attrs, true);
        let writeNames = or_panic(oc.get_bool("output.street-names"));
        let writeLanes = or_panic(oc.get_bool("plain-output.lanes"));

        // write network offsets and projection to allow reconstruction of original coordinates at least for geo-referenced networks
        if !useGeo && gch.using_geo_projection() {
            write_location(&mut edevice);
        }
        let defaultSpread = or_panic(LaneSpreadFunctions.get(&opt_str(oc, "default.spreadtype")));
        for e_ref in ec.myEdges.values() {
            // write the edge itself to the edges-files
            let e: Ptr<NBEdge> = *e_ref;
            edevice.open_tag_id(SUMO_TAG_EDGE);
            edevice.write_attr(SUMO_ATTR_ID, e.get_id());
            edevice.write_attr(SUMO_ATTR_FROM, e.get_from_node().get_id());
            edevice.write_attr(SUMO_ATTR_TO, e.get_to_node().get_id());
            edevice.write_optional_attr(SUMO_ATTR_NAME, &escape_xml(e.get_street_name(), false), !writeNames || e.get_street_name() == "");
            edevice.write_attr(SUMO_ATTR_PRIORITY, &e.get_priority());
            // write the type if given
            edevice.write_optional_attr(SUMO_ATTR_TYPE, e.get_type_id(), e.get_type_id() == "");
            edevice.write_optional_attr(SUMO_ATTR_ROUTINGTYPE, e.get_routing_type(), e.get_routing_type() == "");
            edevice.write_attr(SUMO_ATTR_NUMLANES, &e.get_num_lanes());
            edevice.write_optional_attr(SUMO_ATTR_SPEED, &e.get_speed(), e.has_lane_specific_speed());
            edevice.write_optional_attr(SUMO_ATTR_FRICTION, &e.get_friction(), e.get_friction() == NBEDGE_UNSPECIFIED_FRICTION || e.has_lane_specific_friction());
            // write non-default geometry
            NWWriter_SUMO::write_shape(&mut edevice, &gch, e.get_geometry().clone(), SUMO_ATTR_SHAPE, useGeo, geoAccuracy, !e.has_default_geometry());
            // write the spread type if not default ("right")
            edevice.write_optional_attr(SUMO_ATTR_SPREADTYPE, &e.get_lane_spread_function(), e.get_lane_spread_function() == defaultSpread);
            // write the length if it was specified
            edevice.write_optional_attr(SUMO_ATTR_LENGTH, &e.get_loaded_length(), !e.has_loaded_length());
            // some attributes can be set by edge default or per lane. Write as default if possible (efficiency)
            edevice.write_optional_attr(SUMO_ATTR_WIDTH, &e.get_lane_width(), e.get_lane_width() == NBEDGE_UNSPECIFIED_WIDTH || e.has_lane_specific_width());
            edevice.write_optional_attr(SUMO_ATTR_ENDOFFSET, &e.get_end_offset(), e.get_end_offset() == NBEDGE_UNSPECIFIED_OFFSET || e.has_lane_specific_end_offset());
            // SVC_UNSPECIFIED is here a shortcut for do not write
            write_permissions(&mut edevice, if e.has_lane_specific_permissions() { SVC_UNSPECIFIED } else { e.get_permissions(0) });
            if !e.has_lane_specific_stop_offsets() && e.get_edge_stop_offset().is_defined() {
                NWWriter_SUMO::write_stop_offsets(&mut edevice, e.get_edge_stop_offset());
            }
            edevice.write_optional_attr(SUMO_ATTR_DISTANCE, &e.get_distance(), e.get_distance() == 0.0);
            let bidi = e.get_bidi_edge();
            let bidi_id = if bidi.is_null() { String::new() } else { bidi.get_id().to_string() };
            edevice.write_optional_attr(SUMO_ATTR_BIDI, &bidi_id, bidi.is_null());
            if dlrNavteq {
                NWWriter_DlrNavteq::write_edge_attrs(&mut edevice, e);
            }
            if e.needs_lane_specific_output() || writeLanes {
                let mut idx: i32 = 0;
                for lane in e.get_lanes() {
                    edevice.open_tag_id(SUMO_TAG_LANE);
                    edevice.write_attr(SUMO_ATTR_INDEX, &idx);
                    idx += 1;
                    write_permissions(&mut edevice, if e.has_lane_specific_permissions() || writeLanes { lane.permissions } else { SVC_UNSPECIFIED });
                    write_preferences(&mut edevice, lane.preferred);
                    edevice.write_optional_attr(SUMO_ATTR_WIDTH, &lane.width, lane.width == NBEDGE_UNSPECIFIED_WIDTH || !(e.has_lane_specific_width() || writeLanes));
                    edevice.write_optional_attr(SUMO_ATTR_ENDOFFSET, &lane.endOffset, lane.endOffset == NBEDGE_UNSPECIFIED_OFFSET || !(e.has_lane_specific_end_offset() || writeLanes));
                    edevice.write_optional_attr(SUMO_ATTR_SPEED, &lane.speed, !(e.has_lane_specific_speed() || writeLanes));
                    edevice.write_optional_attr(SUMO_ATTR_ACCELERATION, &lane.accelRamp, !lane.accelRamp);
                    let laneShape = if lane.customShape.len() > 0 { lane.customShape.clone() } else { lane.shape.clone() };
                    NWWriter_SUMO::write_shape(&mut edevice, &gch, laneShape, SUMO_ATTR_SHAPE, useGeo, geoAccuracy, lane.customShape.len() > 0 || writeLanes);
                    edevice.write_optional_attr(SUMO_ATTR_TYPE, &lane.type_, lane.type_ == "");
                    let changeLeft = lane.changeLeft != SVC_ALL && lane.changeLeft != SVC_UNSPECIFIED && lane.changeLeft != SVC_IGNORING;
                    edevice.write_optional_attr(SUMO_ATTR_CHANGE_LEFT, &get_vehicle_class_names(lane.changeLeft, false), !changeLeft);
                    let changeRight = lane.changeRight != SVC_ALL && lane.changeRight != SVC_UNSPECIFIED && lane.changeRight != SVC_IGNORING;
                    edevice.write_optional_attr(SUMO_ATTR_CHANGE_RIGHT, &get_vehicle_class_names(lane.changeRight, false), !changeRight);
                    if lane.oppositeID != "" {
                        edevice.open_tag_id(SUMO_TAG_NEIGH);
                        edevice.write_attr(SUMO_ATTR_LANE, lane.oppositeID.as_str());
                        edevice.close_tag("");
                    }
                    lane.base.write_params(&mut edevice);
                    NWWriter_SUMO::write_stop_offsets(&mut edevice, &lane.laneStopOffset);
                    edevice.close_tag("");
                }
            }
            e.base2.write_params(&mut edevice);
            edevice.close_tag("");
            // write this edge's connections to the connections-files
            let connections = e.get_connections();
            if connections.is_empty() {
                // if there are no connections and this appears to be customized, preserve the information
                let numOutgoing = e.get_to_node().get_outgoing_edges().len() as i32;
                if numOutgoing > 0 {
                    let inPerm = e.get_permissions(-1);
                    let mut outPerm: SVCPermissions = 0;
                    for out in e.get_to_node().get_outgoing_edges() {
                        outPerm |= out.get_permissions(-1);
                    }
                    if (inPerm & outPerm) != 0 && (inPerm & outPerm) != SVC_PEDESTRIAN {
                        cdevice.open_tag_id(SUMO_TAG_CONNECTION);
                        cdevice.write_attr(SUMO_ATTR_FROM, e.get_id());
                        cdevice.close_tag("");
                        cdevice.lf();
                    }
                }
            } else {
                for c in connections {
                    NWWriter_SUMO::write_connection(
                        &mut cdevice,
                        &e,
                        c,
                        false,
                        if dlrNavteq { NWWriter_SUMO_ConnectionStyle::PLAIN_NAVTEQ } else { NWWriter_SUMO_ConnectionStyle::PLAIN },
                        useGeo,
                        geoAccuracy,
                    );
                }
                cdevice.lf();
            }
        }
        // write roundabout information to the edges-files
        let roundabouts = ec.get_roundabouts();
        if roundabouts.len() > 0 {
            edevice.lf();
            NWWriter_SUMO::write_roundabouts(&mut edevice, &roundabouts, ec);
        }

        // write loaded prohibitions to the connections-file
        for n in nc.myNodes.values() {
            NWWriter_SUMO::write_prohibitions(&mut cdevice, n.m().get_prohibitions(), ec);
        }
        // write pedestrian crossings to the connections-file
        for n in nc.myNodes.values() {
            for c in n.get_crossings() {
                cdevice.open_tag_id(SUMO_TAG_CROSSING);
                cdevice.write_attr(SUMO_ATTR_NODE, n.get_id());
                // determinism: C++ Crossing::edges is a vector of edge IDs, written in order
                let edgeIDs: Vec<String> = c.edges.iter().map(|e| e.get_id().to_string()).collect();
                cdevice.write_attr(SUMO_ATTR_EDGES, &join_to_string(&edgeIDs, " "));
                cdevice.write_attr(SUMO_ATTR_PRIORITY, &c.priority);
                cdevice.write_optional_attr(SUMO_ATTR_WIDTH, &c.customWidth, c.customWidth == NBEDGE_UNSPECIFIED_WIDTH);
                cdevice.write_optional_attr(SUMO_ATTR_TLLINKINDEX, &c.customTLIndex, c.customTLIndex == -1);
                cdevice.write_optional_attr(SUMO_ATTR_TLLINKINDEX2, &c.customTLIndex2, c.customTLIndex2 == -1);
                NWWriter_SUMO::write_shape(&mut cdevice, &gch, c.customShape.clone(), SUMO_ATTR_SHAPE, useGeo, geoAccuracy, c.customShape.len() != 0);
                NWWriter_SUMO::write_shape(&mut cdevice, &gch, c.outlineShape.clone(), SUMO_ATTR_OUTLINESHAPE, useGeo, geoAccuracy, c.outlineShape.len() != 0);
                c.base.write_params(&mut cdevice);
                cdevice.close_tag("");
            }
        }
        // write custom walkingarea shapes to the connections file
        for (nodeID, n) in nc.myNodes.iter() {
            for wacs in n.get_walking_area_custom_shapes() {
                cdevice.open_tag_id(SUMO_TAG_WALKINGAREA);
                cdevice.write_attr(SUMO_ATTR_NODE, nodeID.as_str());
                // determinism: C++ std::set<const NBEdge*, ComparatorIdLess> is ordered by edge ID, not by pointer
                let mut edgeIDs: Vec<String> = wacs.edges.iter().map(|e| e.get_id().to_string()).collect();
                edgeIDs.sort();
                cdevice.write_attr(SUMO_ATTR_EDGES, &join_to_string(&edgeIDs, " "));
                NWWriter_SUMO::write_shape(&mut cdevice, &gch, wacs.shape.clone(), SUMO_ATTR_SHAPE, useGeo, geoAccuracy, wacs.shape.len() != 0);
                cdevice.write_optional_attr(SUMO_ATTR_WIDTH, &wacs.width, wacs.width == NBEDGE_UNSPECIFIED_WIDTH);
                cdevice.close_tag("");
            }
        }

        or_panic(edevice.close());
        or_panic(cdevice.close());

        if dlrNavteq {
            let cdevice2_ref = get_device(&format!("{prefix}.cpr.{ext}"));
            let mut cdevice2 = cdevice2_ref.borrow_mut();
            cdevice2.write_xml_header("connections", "connections_file.xsd", BTreeMap::new(), true);
            NWWriter_DlrNavteq::write_prohibited_manoeuvres_attrs(&mut cdevice2, nc);
            or_panic(cdevice2.close());
        }
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
        let ext = utils::options::options_cont::get_string("output.format");
        let mut attrs: BTreeMap<SumoXMLAttr, String> = BTreeMap::new();
        attrs.insert(SUMO_ATTR_VERSION, mm_version_to_string(NETWORK_VERSION));
        let device_ref = get_device(&format!("{prefix}.tll.{ext}"));
        let mut device = device_ref.borrow_mut();
        device.write_xml_header("tlLogics", "tllogic_file.xsd", attrs, true);
        let dlrNavteq = utils::options::options_cont::with_options(|oc| oc.exists("dlr-navteq.plain") && or_panic(oc.get_bool("dlr-navteq.plain")));
        NWWriter_SUMO::write_traffic_lights(
            &mut device,
            tc,
            if dlrNavteq { NWWriter_SUMO_ConnectionStyle::PLAIN_NAVTEQ } else { NWWriter_SUMO_ConnectionStyle::PLAIN },
        );
        // we also need to remember the associations between tlLogics and connections
        // since the information in con.xml is insufficient
        for e in ec.myEdges.values() {
            // write this edge's tl-controlled connections
            for c in e.get_connections() {
                if c.tlID != "" {
                    NWWriter_SUMO::write_connection(&mut device, e, c, false, NWWriter_SUMO_ConnectionStyle::TLL, false, false);
                }
            }
        }
        or_panic(device.close());
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
        let device_ref = get_device(&opt_str(oc, "ptstop-output"));
        let mut device = device_ref.borrow_mut();
        device.write_xml_header("additional", "additional_file.xsd", BTreeMap::new(), true);
        for stop in ec.get_stops().values() {
            stop.write(&mut device);
        }
        or_panic(device.close());
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
        let device_ref = get_device(&opt_str(cont, "ptline-output"));
        let mut device = device_ref.borrow_mut();
        device.write_xml_header("ptLines", "ptlines_file.xsd", BTreeMap::new(), true);
        for item in lc.get_lines().values() {
            item.m().write(&mut device);
        }
        or_panic(device.close());
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
        let device_ref = get_device(&opt_str(cont, "parking-output"));
        let mut device = device_ref.borrow_mut();
        device.write_xml_header("additional", "additional_file.xsd", BTreeMap::new(), true);
        for p in pc.iter() {
            p.write(&mut device, ec);
        }
        or_panic(device.close());
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
        let device_ref = get_device(&opt_str(oc, "taz-output"));
        let mut device = device_ref.borrow_mut();
        device.write_xml_header("additional", "additional_file.xsd", BTreeMap::new(), true);
        for d in dc.myDistricts.values() {
            NWWriter_SUMO::write_district(&mut device, d);
        }
    }

}

