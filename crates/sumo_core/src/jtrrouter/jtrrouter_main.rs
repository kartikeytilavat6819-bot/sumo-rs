//! Port of jtrrouter/jtrrouter_main.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++: `int (OptionsCont &) getTurningDefaults`
// std::vector<double>
// getTurningDefaults(OptionsCont& oc) {
//     std::vector<double> ret;
//     std::vector<std::string> defs = oc.getStringVector("turn-defaults");
//     if (defs.size() < 2) {
//         throw ProcessError(TL("The defaults for turnings must be a tuple of at least two numbers divided by ','."));
//     }
//     for (std::vector<std::string>::const_iterator i = defs.begin(); i != defs.end(); ++i) {
//         try {
//             double val = StringUtils::toDouble(*i);
//             ret.push_back(val);
//         } catch (NumberFormatException&) {
//             throw ProcessError(TL("A turn default is not numeric."));
//         }
//     }
//     return ret;
// }
#[allow(unused_variables)]
pub fn get_turning_defaults(oc: &mut OptionsCont) -> i32 {
    todo!("port getTurningDefaults")
}

/// C++: `void (RONet &, ROLoader &, const int &) initNet`
// void
// initNet(RONet& net, ROLoader& loader,
//         const std::vector<double>& turnDefs) {
//     // load the net
//     ROJTREdgeBuilder builder;
//     loader.loadNet(net, builder);
//     // set the turn defaults
//     for (const auto& i : net.getEdgeMap()) {
//         static_cast<ROJTREdge*>(i.second)->setTurnDefaults(turnDefs);
//     }
// }
#[allow(unused_variables)]
pub fn init_net(net: &mut RONet, loader: &mut ROLoader, turnDefs: &i32) {
    todo!("port initNet")
}

/// C++: `void (RONet &, OptionsCont &) loadJTRDefinitions`
// void
// loadJTRDefinitions(RONet& net, OptionsCont& oc) {
//     // load the turning definitions (and possible sink definition)
//     if (oc.isSet("turn-ratio-files")) {
//         ROJTRTurnDefLoader loader(net);
//         std::vector<std::string> ratio_files = oc.getStringVector("turn-ratio-files");
//         for (std::vector<std::string>::const_iterator i = ratio_files.begin(); i != ratio_files.end(); ++i) {
//             if (!XMLSubSys::runParser(loader, *i)) {
//                 throw ProcessError();
//             }
//         }
//     }
//     if (oc.getBool("sources-are-sinks") || oc.getBool("discount-sources")) {
//         // load all route-files and additional files to discover sink edges and flow discount values
//         ROJTRTurnDefLoader loader(net);
//         for (std::string fileOption : {
//                     "route-files", "additional-files"
//                 }) {
//             for (std::string file : oc.getStringVector(fileOption)) {
//                 if (!XMLSubSys::runParser(loader, file)) {
//                     throw ProcessError();
//                 }
//             }
//         }
//     }
//
//     if (MsgHandler::getErrorInstance()->wasInformed() && oc.getBool("ignore-errors")) {
//         MsgHandler::getErrorInstance()->clear();
//     }
//     // parse sink edges specified at the input/within the configuration
//     if (oc.isSet("sink-edges")) {
//         std::vector<std::string> edges = oc.getStringVector("sink-edges");
//         for (std::vector<std::string>::const_iterator i = edges.begin(); i != edges.end(); ++i) {
//             ROJTREdge* edge = static_cast<ROJTREdge*>(net.getEdge(*i));
//             if (edge == nullptr) {
//                 throw ProcessError(TLF("The edge '%' declared as a sink is not known.", *i));
//             }
//             edge->setSink();
//         }
//     }
// }
#[allow(unused_variables)]
pub fn load_jtr_definitions(net: &mut RONet, oc: &mut OptionsCont) {
    todo!("port loadJTRDefinitions")
}

