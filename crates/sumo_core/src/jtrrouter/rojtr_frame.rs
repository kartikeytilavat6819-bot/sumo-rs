//! Port of jtrrouter/ROJTRFrame.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Sets and checks options for jtr-routing
/// C++ class `ROJTRFrame` (jtrrouter/ROJTRFrame.h)
pub struct ROJTRFrame {
}

impl ROJTRFrame {
    /// C++: `void ROJTRFrame::fillOptions()` [static]
    /// Inserts options used by jtrrouter into the OptionsCont-singleton
    // void
    // ROJTRFrame::fillOptions() {
    //     OptionsCont& oc = OptionsCont::getOptions();
    //     oc.addCallExample("-c <CONFIGURATION>", TL("run with routing options defined in file"));
    //
    //     // insert options sub-topics
    //     SystemFrame::addConfigurationOptions(oc);
    //     oc.addOptionSubTopic("Input");
    //     oc.addOptionSubTopic("Output");
    //     oc.addOptionSubTopic("Processing");
    //     oc.addOptionSubTopic("Defaults");
    //     oc.addOptionSubTopic("Time");
    //
    //     ROFrame::fillOptions(oc);
    //
    //     oc.doRegister("turn-ratio-files", 't', new Option_FileName());
    //     oc.addSynonyme("turn-ratio-files", "turns");
    //     oc.addDescription("turn-ratio-files", "Input", TL("Read turning ratios from FILE(s)"));
    //
    //     oc.doRegister("exit-times", new Option_Bool(false));
    //     oc.addDescription("exit-times", "Output", TL("Write exit times (weights) for each edge"));
    //
    //     oc.doRegister("max-edges-factor", new Option_Float(2.0));
    //     oc.addDescription("max-edges-factor", "Processing",
    //                       "Routes are cut off when the route edges to net edges ratio is larger than FLOAT");
    //
    //     oc.doRegister("turn-defaults", 'T', new Option_StringVector(StringVector({ "30", "50", "20" })));
    //     oc.addDescription("turn-defaults", "Processing", TL("Use STR[] as default turn definition"));
    //
    //     oc.doRegister("sink-edges", new Option_StringVector());
    //     oc.addSynonyme("sink-edges", "sinks");
    //     oc.addDescription("sink-edges", "Processing", TL("Use STR[] as list of sink edges"));
    //
    //     oc.doRegister("accept-all-destinations", 'A', new Option_Bool(false));
    //     oc.addDescription("accept-all-destinations", "Processing", TL("Whether all edges are allowed as sink edges"));
    //
    //     oc.doRegister("ignore-vclasses", 'i', new Option_Bool(false));
    //     oc.addDescription("ignore-vclasses", "Processing", TL("Ignore road restrictions based on vehicle class"));
    //
    //     oc.doRegister("allow-loops", new Option_Bool(false));
    //     oc.addDescription("allow-loops", "Processing", TL("Allow to re-use a road"));
    //
    //     oc.doRegister("sources-are-sinks", 'S', new Option_Bool(false));
    //     oc.addDescription("sources-are-sinks", "Processing", TL("Use all source edges as sink edges."));
    //
    //     oc.doRegister("discount-sources", 'D',  new Option_Bool(false));
    //     oc.addDescription("discount-sources", "Processing", TL("Subtract upstream flow when inserting a new flow. When option --sources-are-sinks is set, the upstream flow is limited to the value of the source flow and the remaining part terminates."));
    //
    //     // add rand options
    //     RandHelper::insertRandOptions(oc);
    // }
    #[allow(unused_variables)]
    pub fn fill_options() {
        todo!("port ROJTRFrame::fillOptions")
    }

    /// C++: `bool ROJTRFrame::checkOptions()` [static]
    /// Checks set options from the OptionsCont-singleton for being valid for usage within jtrrouter
    // bool
    // ROJTRFrame::checkOptions() {
    //     OptionsCont& oc = OptionsCont::getOptions();
    //     bool ok = ROFrame::checkOptions(oc);
    //     if (oc.isDefault("no-internal-links")) {
    //         oc.setDefault("no-internal-links", "true");
    //     }
    //
    //     SUMOVehicleParameter p;
    //     std::string error;
    //     if (oc.isSet("departlane") && !SUMOVehicleParameter::parseDepartLane(oc.getString("departlane"), "option", "departlane", p.departLane, p.departLaneProcedure, error)) {
    //         WRITE_ERROR(error);
    //         ok = false;
    //     }
    //     if (oc.isSet("departpos") && !SUMOVehicleParameter::parseDepartPos(oc.getString("departpos"), "option", "departpos", p.departPos, p.departPosProcedure, error)) {
    //         WRITE_ERROR(error);
    //         ok = false;
    //     }
    //     if (oc.isSet("departspeed") && !SUMOVehicleParameter::parseDepartSpeed(oc.getString("departspeed"), "option", "departspeed", p.departSpeed, p.departSpeedProcedure, error)) {
    //         WRITE_ERROR(error);
    //         ok = false;
    //     }
    //     if (oc.isSet("arrivallane") && !SUMOVehicleParameter::parseArrivalLane(oc.getString("arrivallane"), "option", "arrivallane", p.arrivalLane, p.arrivalLaneProcedure, error)) {
    //         WRITE_ERROR(error);
    //         ok = false;
    //     }
    //     if (oc.isSet("arrivalpos") && !SUMOVehicleParameter::parseArrivalPos(oc.getString("arrivalpos"), "option", "arrivalpos", p.arrivalPos, p.arrivalPosProcedure, error)) {
    //         WRITE_ERROR(error);
    //         ok = false;
    //     }
    //     if (oc.isSet("arrivalspeed") && !SUMOVehicleParameter::parseArrivalSpeed(oc.getString("arrivalspeed"), "option", "arrivalspeed", p.arrivalSpeed, p.arrivalSpeedProcedure, error)) {
    //         WRITE_ERROR(error);
    //         ok = false;
    //     }
    //     return ok;
    // }
    #[allow(unused_variables)]
    pub fn check_options() -> bool {
        todo!("port ROJTRFrame::checkOptions")
    }

}

