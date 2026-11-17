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
        utils::options::options_cont::with_options(|oc| {
            oc.add_call_example("-c <CONFIGURATION>", "run with routing options defined in file");

            // insert options sub-topics
            utils::common::system_frame::add_configuration_options(oc);
            oc.add_option_sub_topic("Input");
            oc.add_option_sub_topic("Output");
            oc.add_option_sub_topic("Processing");
            oc.add_option_sub_topic("Defaults");
            oc.add_option_sub_topic("Time");

            ROFrame::fill_options(oc, false, false);

            oc.do_register_abbr("turn-ratio-files", 't', SumoOption::file_name());
            oc.add_synonyme("turn-ratio-files", "turns", false);
            oc.add_description("turn-ratio-files", "Input", "Read turning ratios from FILE(s)");

            oc.do_register("exit-times", SumoOption::bool(false));
            oc.add_description("exit-times", "Output", "Write exit times (weights) for each edge");

            oc.do_register("max-edges-factor", SumoOption::float(2.0));
            oc.add_description("max-edges-factor", "Processing",
                               "Routes are cut off when the route edges to net edges ratio is larger than FLOAT");

            oc.do_register_abbr("turn-defaults", 'T', SumoOption::string_vector_with(vec!["30".to_string(), "50".to_string(), "20".to_string()]));
            oc.add_description("turn-defaults", "Processing", "Use STR[] as default turn definition");

            oc.do_register("sink-edges", SumoOption::string_vector());
            oc.add_synonyme("sink-edges", "sinks", false);
            oc.add_description("sink-edges", "Processing", "Use STR[] as list of sink edges");

            oc.do_register_abbr("accept-all-destinations", 'A', SumoOption::bool(false));
            oc.add_description("accept-all-destinations", "Processing", "Whether all edges are allowed as sink edges");

            oc.do_register_abbr("ignore-vclasses", 'i', SumoOption::bool(false));
            oc.add_description("ignore-vclasses", "Processing", "Ignore road restrictions based on vehicle class");

            oc.do_register("allow-loops", SumoOption::bool(false));
            oc.add_description("allow-loops", "Processing", "Allow to re-use a road");

            oc.do_register_abbr("sources-are-sinks", 'S', SumoOption::bool(false));
            oc.add_description("sources-are-sinks", "Processing", "Use all source edges as sink edges.");

            oc.do_register_abbr("discount-sources", 'D', SumoOption::bool(false));
            oc.add_description("discount-sources", "Processing", "Subtract upstream flow when inserting a new flow. When option --sources-are-sinks is set, the upstream flow is limited to the value of the source flow and the remaining part terminates.");

            // add rand options
            utils::common::rand_helper::insert_rand_options(oc);
        });
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
        utils::options::options_cont::with_options(|oc| {
            let mut ok = ROFrame::check_options(oc);
            if oc.is_default("no-internal-links") {
                oc.set_default("no-internal-links", "true").unwrap();
            }

            let mut p = SUMOVehicleParameter::new();
            let mut error = String::new();
            if oc.is_set("departlane", true).unwrap() && !SUMOVehicleParameter::parse_depart_lane(&oc.get_string("departlane").unwrap(), "option", "departlane", &mut p.departLane, &mut p.departLaneProcedure, &mut error) {
                utils::common::msg_handler::write_error(&error);
                ok = false;
            }
            if oc.is_set("departpos", true).unwrap() && !SUMOVehicleParameter::parse_depart_pos(&oc.get_string("departpos").unwrap(), "option", "departpos", &mut p.departPos, &mut p.departPosProcedure, &mut error) {
                utils::common::msg_handler::write_error(&error);
                ok = false;
            }
            if oc.is_set("departspeed", true).unwrap() && !SUMOVehicleParameter::parse_depart_speed(&oc.get_string("departspeed").unwrap(), "option", "departspeed", &mut p.departSpeed, &mut p.departSpeedProcedure, &mut error) {
                utils::common::msg_handler::write_error(&error);
                ok = false;
            }
            if oc.is_set("arrivallane", true).unwrap() && !SUMOVehicleParameter::parse_arrival_lane(&oc.get_string("arrivallane").unwrap(), "option", "arrivallane", &mut p.arrivalLane, &mut p.arrivalLaneProcedure, &mut error) {
                utils::common::msg_handler::write_error(&error);
                ok = false;
            }
            if oc.is_set("arrivalpos", true).unwrap() && !SUMOVehicleParameter::parse_arrival_pos(&oc.get_string("arrivalpos").unwrap(), "option", "arrivalpos", &mut p.arrivalPos, &mut p.arrivalPosProcedure, &mut error) {
                utils::common::msg_handler::write_error(&error);
                ok = false;
            }
            if oc.is_set("arrivalspeed", true).unwrap() && !SUMOVehicleParameter::parse_arrival_speed(&oc.get_string("arrivalspeed").unwrap(), "option", "arrivalspeed", &mut p.arrivalSpeed, &mut p.arrivalSpeedProcedure, &mut error) {
                utils::common::msg_handler::write_error(&error);
                ok = false;
            }
            ok
        })
    }

}

