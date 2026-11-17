//! Port of netwrite/NWWriter_MATSim.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::to_string::f64_to_string_prec;

/// C++ `OptionsCont` getters throw ProcessError; the options used here are always registered
fn or_panic<T, E: std::fmt::Display>(r: Result<T, E>) -> T {
    r.unwrap_or_else(|e| panic!("{e}"))
}

/// Exporter writing networks using the MATSim format
/// C++ class `NWWriter_MATSim` (netwrite/NWWriter_MATSim.h)
pub struct NWWriter_MATSim {
}

impl NWWriter_MATSim {
    /// C++: `void NWWriter_MATSim::writeNetwork(const OptionsCont &, NBNetBuilder &)` [static]
    /// Writes the network into a MATSim-file
    // void
    // NWWriter_MATSim::writeNetwork(const OptionsCont& oc, NBNetBuilder& nb) {
    //     // check whether a matsim-file shall be generated
    //     if (!oc.isSet("matsim-output")) {
    //         return;
    //     }
    //     OutputDevice& device = OutputDevice::getDevice(oc.getString("matsim-output"));
    //     device << "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n";
    //     device << "<!DOCTYPE network SYSTEM \"http://www.matsim.org/files/dtd/network_v1.dtd\">\n\n";
    //     device << "<network name=\"NAME\">\n"; // !!! name
    //     // write nodes
    //     device << "   <nodes>\n";
    //     NBNodeCont& nc = nb.getNodeCont();
    //     for (std::map<std::string, NBNode*>::const_iterator i = nc.begin(); i != nc.end(); ++i) {
    //         device << "      <node id=\"" << (*i).first
    //                << "\" x=\"" << (*i).second->getPosition().x()
    //                << "\" y=\"" << (*i).second->getPosition().y()
    //                << "\"/>\n";
    //     }
    //     device << "   </nodes>\n";
    //     // write edges
    //     device << "   <links capperiod=\"01:00:00\">\n";
    //     NBEdgeCont& ec = nb.getEdgeCont();
    //     for (std::map<std::string, NBEdge*>::const_iterator i = ec.begin(); i != ec.end(); ++i) {
    //         device << "      <link id=\"" << (*i).first
    //                << "\" from=\"" << (*i).second->getFromNode()->getID()
    //                << "\" to=\"" << (*i).second->getToNode()->getID()
    //                << "\" length=\"" << (*i).second->getLoadedLength()
    //                << "\" capacity=\"" << (oc.getFloat("lanes-from-capacity.norm") * (*i).second->getNumLanes())
    //                << "\" freespeed=\"" << (*i).second->getSpeed()
    //                << "\" permlanes=\"" << (*i).second->getNumLanes()
    //                << "\"/>\n";
    //     }
    //     device << "   </links>\n";
    //     //
    //     device << "</network>\n"; // !!! name
    //     device.close();
    // }
    #[allow(unused_variables)]
    pub fn write_network(oc: &OptionsCont, nb: &mut NBNetBuilder) {
        // check whether a matsim-file shall be generated
        if !or_panic(oc.is_set("matsim-output", true)) {
            return;
        }
        let device = or_panic(OutputDevice::get_device(&or_panic(oc.get_string("matsim-output")), true));
        let mut dev = device.borrow_mut();
        let prec = dev.get_precision();
        dev.write("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n");
        dev.write("<!DOCTYPE network SYSTEM \"http://www.matsim.org/files/dtd/network_v1.dtd\">\n\n");
        dev.write("<network name=\"NAME\">\n"); // !!! name
        // write nodes
        dev.write("   <nodes>\n");
        let nc = nb.get_node_cont();
        for (id, node) in nc.begin() {
            dev.write("      <node id=\"").write(id.as_str())
                .write("\" x=\"").write(&f64_to_string_prec(node.get_position().x(), prec))
                .write("\" y=\"").write(&f64_to_string_prec(node.get_position().y(), prec))
                .write("\"/>\n");
        }
        dev.write("   </nodes>\n");
        // write edges
        dev.write("   <links capperiod=\"01:00:00\">\n");
        let ec = nb.get_edge_cont();
        for (id, edge) in ec.begin() {
            dev.write("      <link id=\"").write(id.as_str())
                .write("\" from=\"").write(edge.get_from_node().get_id())
                .write("\" to=\"").write(edge.get_to_node().get_id())
                .write("\" length=\"").write(&f64_to_string_prec(edge.get_loaded_length(), prec))
                .write("\" capacity=\"").write(&f64_to_string_prec(or_panic(oc.get_float("lanes-from-capacity.norm")) * edge.get_num_lanes() as f64, prec))
                .write("\" freespeed=\"").write(&f64_to_string_prec(edge.get_speed(), prec))
                .write("\" permlanes=\"").write(&format!("{}", edge.get_num_lanes()))
                .write("\"/>\n");
        }
        dev.write("   </links>\n");
        //
        dev.write("</network>\n"); // !!! name
        let _ = dev.close();
    }

}

