//! Port of netwrite/NWWriter_MATSim.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

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
        todo!("port NWWriter_MATSim::writeNetwork")
    }

}

