//! Port of netwrite/NWWriter_Amitran.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Exporter writing networks using the Amitran format
/// C++ class `NWWriter_Amitran` (netwrite/NWWriter_Amitran.h)
pub struct NWWriter_Amitran {
}

impl NWWriter_Amitran {
    /// C++: `void NWWriter_Amitran::writeNetwork(const OptionsCont &, NBNetBuilder &)` [static]
    /// Writes the network into a Amitran-file
    // void
    // NWWriter_Amitran::writeNetwork(const OptionsCont& oc, NBNetBuilder& nb) {
    //     // check whether an amitran-file shall be generated
    //     if (!oc.isSet("amitran-output")) {
    //         return;
    //     }
    //     NBEdgeCont& ec = nb.getEdgeCont();
    //     OutputDevice& device = OutputDevice::getDevice(oc.getString("amitran-output"));
    //     device << "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n";
    //     device << "<network xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xsi:noNamespaceSchemaLocation=\"http://sumo.dlr.de/xsd/amitran/network.xsd\">\n";
    //     // write nodes
    //     int index = 0;
    //     NBNodeCont& nc = nb.getNodeCont();
    //     std::set<NBNode*> singleRoundaboutNodes;
    //     std::set<NBNode*> multiRoundaboutNodes;
    //     const std::set<EdgeSet>& roundabouts = ec.getRoundabouts();
    //     for (std::set<EdgeSet>::const_iterator i = roundabouts.begin(); i != roundabouts.end(); ++i) {
    //         for (EdgeSet::const_iterator j = (*i).begin(); j != (*i).end(); ++j) {
    //             if ((*j)->getNumLanes() > 1) {
    //                 multiRoundaboutNodes.insert((*j)->getFromNode());
    //             } else {
    //                 singleRoundaboutNodes.insert((*j)->getFromNode());
    //             }
    //         }
    //     }
    //     std::map<NBNode*, int> nodeIds;
    //     for (std::map<std::string, NBNode*>::const_iterator i = nc.begin(); i != nc.end(); ++i) {
    //         device << "    <node id=\"" << index;
    //         nodeIds[i->second] = index++;
    //         if (singleRoundaboutNodes.count(i->second) > 0) {
    //             device << "\" type=\"roundaboutSingle\"/>\n";
    //             continue;
    //         }
    //         if (multiRoundaboutNodes.count(i->second) > 0) {
    //             device << "\" type=\"roundaboutMulti\"/>\n";
    //             continue;
    //         }
    //         switch (i->second->getType()) {
    //             case SumoXMLNodeType::TRAFFIC_LIGHT:
    //             case SumoXMLNodeType::TRAFFIC_LIGHT_NOJUNCTION:
    //             case SumoXMLNodeType::TRAFFIC_LIGHT_RIGHT_ON_RED:
    //                 device << "\" type=\"trafficLight";
    //                 break;
    //             case SumoXMLNodeType::PRIORITY:
    //                 device << "\" type=\"priority";
    //                 break;
    //             case SumoXMLNodeType::PRIORITY_STOP:
    //                 device << "\" type=\"priorityStop";
    //                 break;
    //             case SumoXMLNodeType::RIGHT_BEFORE_LEFT:
    //                 device << "\" type=\"rightBeforeLeft";
    //                 break;
    //             case SumoXMLNodeType::LEFT_BEFORE_RIGHT:
    //                 device << "\" type=\"leftBeforeRight";
    //                 break;
    //             case SumoXMLNodeType::ALLWAY_STOP:
    //                 device << "\" type=\"allwayStop";
    //                 break;
    //             case SumoXMLNodeType::ZIPPER:
    //                 device << "\" type=\"zipper";
    //                 break;
    //             case SumoXMLNodeType::RAIL_SIGNAL:
    //                 device << "\" type=\"railSignal";
    //                 break;
    //             case SumoXMLNodeType::RAIL_CROSSING:
    //                 device << "\" type=\"railCrossing";
    //                 break;
    //             case SumoXMLNodeType::DEAD_END:
    //             case SumoXMLNodeType::DEAD_END_DEPRECATED:
    //                 device << "\" type=\"deadEnd";
    //                 break;
    //             case SumoXMLNodeType::DISTRICT:
    //             case SumoXMLNodeType::NOJUNCTION:
    //             case SumoXMLNodeType::INTERNAL:
    //             case SumoXMLNodeType::UNKNOWN:
    //                 break;
    //         }
    //         device << "\"/>\n";
    //     }
    //     // write edges
    //     index = 0;
    //     for (std::map<std::string, NBEdge*>::const_iterator i = ec.begin(); i != ec.end(); ++i) {
    //         device << "    <link id=\"" << index++
    //                << "\" from=\"" << nodeIds[i->second->getFromNode()]
    //                << "\" to=\"" << nodeIds[i->second->getToNode()]
    //                << "\" roadClass=\"" << NWWriter_DlrNavteq::getRoadClass((*i).second)
    //                << "\" length=\"" << int(1000 * i->second->getLoadedLength())
    //                << "\" speedLimitKmh=\"" << int(3.6 * (*i).second->getSpeed() + 0.5)
    //                << "\" laneNr=\"" << (*i).second->getNumLanes()
    //                << "\"/>\n";
    //     }
    //     device << "</network>\n";
    //     device.close();
    // }
    #[allow(unused_variables)]
    pub fn write_network(oc: &OptionsCont, nb: &mut NBNetBuilder) {
        // check whether an amitran-file shall be generated
        if !or_panic(oc.is_set("amitran-output", true)) {
            return;
        }
        let ec = &nb.myEdgeCont;
        let device_ref = OutputDevice::get_device(&or_panic(oc.get_string("amitran-output")), true).unwrap_or_else(|e| panic!("{e}"));
        let mut device = device_ref.borrow_mut();
        device.write("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n");
        device.write("<network xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xsi:noNamespaceSchemaLocation=\"http://sumo.dlr.de/xsd/amitran/network.xsd\">\n");
        // write nodes
        let mut index: i32 = 0;
        let nc = &nb.myNodeCont;
        let mut singleRoundaboutNodes: BTreeSet<Ptr<NBNode>> = BTreeSet::new();
        let mut multiRoundaboutNodes: BTreeSet<Ptr<NBNode>> = BTreeSet::new();
        let roundabouts = ec.get_roundabouts();
        for i in roundabouts.iter() {
            for j in i.iter() {
                if j.get_num_lanes() > 1 {
                    multiRoundaboutNodes.insert(j.get_from_node());
                } else {
                    singleRoundaboutNodes.insert(j.get_from_node());
                }
            }
        }
        let mut nodeIds: BTreeMap<Ptr<NBNode>, i32> = BTreeMap::new();
        for (_id, node) in nc.begin() {
            device.write("    <node id=\"").write(&index);
            nodeIds.insert(*node, index);
            index += 1;
            if singleRoundaboutNodes.contains(node) {
                device.write("\" type=\"roundaboutSingle\"/>\n");
                continue;
            }
            if multiRoundaboutNodes.contains(node) {
                device.write("\" type=\"roundaboutMulti\"/>\n");
                continue;
            }
            match node.get_type() {
                SumoXMLNodeType::TRAFFIC_LIGHT | SumoXMLNodeType::TRAFFIC_LIGHT_NOJUNCTION | SumoXMLNodeType::TRAFFIC_LIGHT_RIGHT_ON_RED => {
                    device.write("\" type=\"trafficLight");
                }
                SumoXMLNodeType::PRIORITY => {
                    device.write("\" type=\"priority");
                }
                SumoXMLNodeType::PRIORITY_STOP => {
                    device.write("\" type=\"priorityStop");
                }
                SumoXMLNodeType::RIGHT_BEFORE_LEFT => {
                    device.write("\" type=\"rightBeforeLeft");
                }
                SumoXMLNodeType::LEFT_BEFORE_RIGHT => {
                    device.write("\" type=\"leftBeforeRight");
                }
                SumoXMLNodeType::ALLWAY_STOP => {
                    device.write("\" type=\"allwayStop");
                }
                SumoXMLNodeType::ZIPPER => {
                    device.write("\" type=\"zipper");
                }
                SumoXMLNodeType::RAIL_SIGNAL => {
                    device.write("\" type=\"railSignal");
                }
                SumoXMLNodeType::RAIL_CROSSING => {
                    device.write("\" type=\"railCrossing");
                }
                SumoXMLNodeType::DEAD_END | SumoXMLNodeType::DEAD_END_DEPRECATED => {
                    device.write("\" type=\"deadEnd");
                }
                _ => {}
            }
            device.write("\"/>\n");
        }
        // write edges
        index = 0;
        for (_id, edge) in ec.begin() {
            device.write("    <link id=\"").write(&index);
            index += 1;
            // operator[] default-inserts 0 for unknown nodes, as in C++
            let from = *nodeIds.entry(edge.get_from_node()).or_insert(0);
            let to = *nodeIds.entry(edge.get_to_node()).or_insert(0);
            device.write("\" from=\"").write(&from)
                .write("\" to=\"").write(&to)
                .write("\" roadClass=\"").write(&NWWriter_DlrNavteq::get_road_class(*edge))
                .write("\" length=\"").write(&((1000.0 * edge.get_loaded_length()) as i32))
                .write("\" speedLimitKmh=\"").write(&((3.6 * edge.get_speed() + 0.5) as i32))
                .write("\" laneNr=\"").write(&edge.get_num_lanes())
                .write("\"/>\n");
        }
        device.write("</network>\n");
        device.close().unwrap_or_else(|e| panic!("{e}"));
    }

}

