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
pub fn get_turning_defaults(oc: &mut OptionsCont) -> Vec<f64> {
    let mut ret: Vec<f64> = Vec::new();
    let defs = oc.get_string_vector("turn-defaults").unwrap().clone();
    if defs.len() < 2 {
        panic!("{}", ProcessError::new("The defaults for turnings must be a tuple of at least two numbers divided by ','."));
    }
    for i in defs.iter() {
        match utils::common::string_utils::to_double(i) {
            Ok(val) => ret.push(val),
            Err(e) if e.is_format() => panic!("{}", ProcessError::new("A turn default is not numeric.")),
            Err(e) => panic!("{}", e),
        }
    }
    ret
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
pub fn init_net(net: &mut RONet, loader: &mut ROLoader, turnDefs: &Vec<f64>) {
    // load the net
    let mut builder = crate::jtrrouter::rojtr_edge_builder::ROJTREdgeBuilder::new();
    loader.load_net(net, &mut builder);
    // set the turn defaults
    for (_, i) in net.get_edge_map().iter() {
        i.cast::<crate::jtrrouter::rojtr_edge::ROJTREdge>().m().set_turn_defaults(turnDefs);
    }
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
    // load the turning definitions (and possible sink definition)
    if oc.is_set("turn-ratio-files", true).unwrap() {
        let mut loader = crate::jtrrouter::rojtr_turn_def_loader::ROJTRTurnDefLoader::new(net);
        let ratio_files = oc.get_string_vector("turn-ratio-files").unwrap().clone();
        for i in ratio_files.iter() {
            if !utils::xml::xml_sub_sys::run_parser(&mut loader, i, false, false, false, true).unwrap() {
                panic!("{}", ProcessError::default_msg());
            }
        }
    }
    if oc.get_bool("sources-are-sinks").unwrap() || oc.get_bool("discount-sources").unwrap() {
        // load all route-files and additional files to discover sink edges and flow discount values
        let mut loader = crate::jtrrouter::rojtr_turn_def_loader::ROJTRTurnDefLoader::new(net);
        for fileOption in ["route-files", "additional-files"] {
            for file in oc.get_string_vector(fileOption).unwrap().clone() {
                if !utils::xml::xml_sub_sys::run_parser(&mut loader, &file, false, false, false, true).unwrap() {
                    panic!("{}", ProcessError::default_msg());
                }
            }
        }
    }

    if utils::common::msg_handler::with_instance(utils::common::msg_handler::MsgType::Error, |h| h.was_informed()) && oc.get_bool("ignore-errors").unwrap() {
        utils::common::msg_handler::with_instance(utils::common::msg_handler::MsgType::Error, |h| h.clear(true));
    }
    // parse sink edges specified at the input/within the configuration
    if oc.is_set("sink-edges", true).unwrap() {
        let edges = oc.get_string_vector("sink-edges").unwrap().clone();
        for i in edges.iter() {
            let edge = net.get_edge(i).cast::<crate::jtrrouter::rojtr_edge::ROJTREdge>();
            if edge.is_null() {
                panic!("{}", ProcessError::new(format!("The edge '{}' declared as a sink is not known.", i)));
            }
            edge.m().set_sink(true);
        }
    }
}

