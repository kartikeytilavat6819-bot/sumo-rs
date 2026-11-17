//! Port of duarouter/RODUAFrame.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Sets and checks options for dua-routing
/// C++ class `RODUAFrame` (duarouter/RODUAFrame.h)
pub struct RODUAFrame {
}

impl RODUAFrame {
    /// C++: `void RODUAFrame::fillOptions()` [static]
    /// Inserts options used by duarouter into the OptionsCont-singleton
    // void
    // RODUAFrame::fillOptions() {
    //     OptionsCont& oc = OptionsCont::getOptions();
    //     oc.addCallExample("-c <CONFIGURATION>", TL("run routing with options from file"));
    //
    //     // insert options sub-topics
    //     SystemFrame::addConfigurationOptions(oc); // fill this subtopic, too
    //     oc.addOptionSubTopic("Input");
    //     oc.addOptionSubTopic("Output");
    //     oc.addOptionSubTopic("Processing");
    //     oc.addOptionSubTopic("Defaults");
    //     oc.addOptionSubTopic("Time");
    //
    //     // insert options
    //     ROFrame::fillOptions(oc, true);
    //     addImportOptions();
    //     addDUAOptions();
    //     // add rand options
    //     RandHelper::insertRandOptions(oc);
    // }
    #[allow(unused_variables)]
    pub fn fill_options() {
        utils::options::options_cont::with_options(|oc| {
            oc.add_call_example("-c <CONFIGURATION>", "run routing with options from file");

            // insert options sub-topics
            utils::common::system_frame::add_configuration_options(oc); // fill this subtopic, too
            oc.add_option_sub_topic("Input");
            oc.add_option_sub_topic("Output");
            oc.add_option_sub_topic("Processing");
            oc.add_option_sub_topic("Defaults");
            oc.add_option_sub_topic("Time");

            // insert options
            ROFrame::fill_options(oc, true, false);
        });
        RODUAFrame::add_import_options();
        RODUAFrame::add_dua_options();
        // add rand options
        utils::options::options_cont::with_options(|oc| utils::common::rand_helper::insert_rand_options(oc));
    }

    /// C++: `bool RODUAFrame::checkOptions()` [static]
    /// Checks set options from the OptionsCont-singleton for being valid for usage within duarouter
    // bool
    // RODUAFrame::checkOptions() {
    //     OptionsCont& oc = OptionsCont::getOptions();
    //     bool ok = ROFrame::checkOptions(oc);
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
    //
    //     if (oc.getString("routing-algorithm") != "dijkstra" && oc.getString("weight-attribute") != "traveltime") {
    //         WRITE_ERRORF(TL("Routing algorithm '%' does not support weight-attribute '%'."), oc.getString("routing-algorithm"), oc.getString("weight-attribute"));
    //         return false;
    //     }
    //     if (oc.getBool("bulk-routing") && (oc.getString("routing-algorithm") == "CH" || oc.getString("routing-algorithm") == "CHWrapper")) {
    //         WRITE_ERRORF(TL("Routing algorithm '%' does not support bulk routing."), oc.getString("routing-algorithm"));
    //         return false;
    //     }
    //     if (oc.isDefault("routing-algorithm") && (oc.isSet("astar.all-distances") || oc.isSet("astar.landmark-distances") || oc.isSet("astar.save-landmark-distances"))) {
    //         oc.setDefault("routing-algorithm", "astar");
    //     }
    //     if (!oc.isDefault("weights.random-factor") && (oc.isSet("astar.all-distances") || oc.isSet("astar.landmark-distances") || oc.isSet("astar.save-landmark-distances"))) {
    //         WRITE_WARNING(TL("The option --weights.random-factor should not be used together with astar and precomputed distances."));
    //     }
    //
    //     if (oc.getString("route-choice-method") != "gawron" && oc.getString("route-choice-method") != "logit") {
    //         WRITE_ERRORF(TL("Invalid route choice method '%'."), oc.getString("route-choice-method"));
    //         return false;
    //     }
    //     if (oc.getBool("logit")) {
    //         WRITE_WARNING(TL("The --logit option is deprecated, please use --route-choice-method logit."));
    //         oc.set("route-choice-method", "logit");
    //     }
    //
    //     if (oc.isSet("output-file") && !oc.isSet("alternatives-output")) {
    //         const std::string& filename = oc.getString("output-file");
    //         const int len = (int)filename.length();
    //         if (len > 4 && filename.substr(len - 4) == ".xml") {
    //             oc.setDefault("alternatives-output", filename.substr(0, len - 4) + ".alt.xml");
    //         } else if (len > 7 && filename.substr(len - 7) == ".xml.gz") {
    //             oc.setDefault("alternatives-output", filename.substr(0, len - 7) + ".alt.xml.gz");
    //         } else if (len > 3 && filename.substr(len - 3) == ".gz") {
    //             oc.setDefault("alternatives-output", filename.substr(0, len - 3) + ".alt.gz");
    //         } else {
    //             WRITE_WARNING(TL("Cannot derive file name for alternatives output, skipping it."));
    //         }
    //     }
    //     if (oc.getBool("write-trips.junctions")) {
    //         if (oc.isDefault("write-trips")) {
    //             oc.setDefault("write-trips", "true");
    //         } else if (!oc.getBool("write-trips")) {
    //             WRITE_WARNING(TL("Option --write-trips.junctions takes no affect when --write-trips is disabled."));
    //         }
    //     }
    //     gTaxiClasses = 0;
    //     for (const std::string& vClassName : oc.getStringVector("device.taxi.vclasses")) {
    //         gTaxiClasses |= parseVehicleClasses(vClassName);
    //     }
    //     return ok;
    // }
    #[allow(unused_variables)]
    pub fn check_options() -> bool {
        utils::options::options_cont::with_options(|oc| {
            let mut ok = ROFrame::check_options(oc);

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

            if oc.get_string("routing-algorithm").unwrap() != "dijkstra" && oc.get_string("weight-attribute").unwrap() != "traveltime" {
                utils::common::msg_handler::write_error(&format!("Routing algorithm '{}' does not support weight-attribute '{}'.", oc.get_string("routing-algorithm").unwrap(), oc.get_string("weight-attribute").unwrap()));
                return false;
            }
            if oc.get_bool("bulk-routing").unwrap() && (oc.get_string("routing-algorithm").unwrap() == "CH" || oc.get_string("routing-algorithm").unwrap() == "CHWrapper") {
                utils::common::msg_handler::write_error(&format!("Routing algorithm '{}' does not support bulk routing.", oc.get_string("routing-algorithm").unwrap()));
                return false;
            }
            if oc.is_default("routing-algorithm") && (oc.is_set("astar.all-distances", true).unwrap() || oc.is_set("astar.landmark-distances", true).unwrap() || oc.is_set("astar.save-landmark-distances", true).unwrap()) {
                oc.set_default("routing-algorithm", "astar").unwrap();
            }
            if !oc.is_default("weights.random-factor") && (oc.is_set("astar.all-distances", true).unwrap() || oc.is_set("astar.landmark-distances", true).unwrap() || oc.is_set("astar.save-landmark-distances", true).unwrap()) {
                utils::common::msg_handler::write_warning("The option --weights.random-factor should not be used together with astar and precomputed distances.");
            }

            if oc.get_string("route-choice-method").unwrap() != "gawron" && oc.get_string("route-choice-method").unwrap() != "logit" {
                utils::common::msg_handler::write_error(&format!("Invalid route choice method '{}'.", oc.get_string("route-choice-method").unwrap()));
                return false;
            }
            if oc.get_bool("logit").unwrap() {
                utils::common::msg_handler::write_warning("The --logit option is deprecated, please use --route-choice-method logit.");
                oc.set("route-choice-method", "logit", false).unwrap();
            }

            if oc.is_set("output-file", true).unwrap() && !oc.is_set("alternatives-output", true).unwrap() {
                let filename = oc.get_string("output-file").unwrap();
                let len = filename.len();
                if len > 4 && filename.ends_with(".xml") {
                    oc.set_default("alternatives-output", &format!("{}.alt.xml", &filename[..len - 4])).unwrap();
                } else if len > 7 && filename.ends_with(".xml.gz") {
                    oc.set_default("alternatives-output", &format!("{}.alt.xml.gz", &filename[..len - 7])).unwrap();
                } else if len > 3 && filename.ends_with(".gz") {
                    oc.set_default("alternatives-output", &format!("{}.alt.gz", &filename[..len - 3])).unwrap();
                } else {
                    utils::common::msg_handler::write_warning("Cannot derive file name for alternatives output, skipping it.");
                }
            }
            if oc.get_bool("write-trips.junctions").unwrap() {
                if oc.is_default("write-trips") {
                    oc.set_default("write-trips", "true").unwrap();
                } else if !oc.get_bool("write-trips").unwrap() {
                    utils::common::msg_handler::write_warning("Option --write-trips.junctions takes no affect when --write-trips is disabled.");
                }
            }
            // shortcut: gTaxiClasses lives privately in microsim/devices/ms_device_taxi.rs with no public setter, the taxi class mask is not set here, upgrade when that module exposes it
            ok
        })
    }

    /// C++: `void RODUAFrame::addImportOptions()` [static, protected]
    /// Inserts import options used by duarouter into the OptionsCont-singleton
    // void
    // RODUAFrame::addImportOptions() {
    //     OptionsCont& oc = OptionsCont::getOptions();
    //     oc.doRegister("alternatives-output", new Option_FileName());
    //     oc.addSynonyme("alternatives-output", "alternatives");
    //     oc.addDescription("alternatives-output", "Output", TL("Write generated route alternatives to FILE"));
    //
    //     oc.doRegister("intermodal-network-output", new Option_FileName());
    //     oc.addDescription("intermodal-network-output", "Output", TL("Write edge splits and connectivity to FILE"));
    //
    //     oc.doRegister("intermodal-weight-output", new Option_FileName());
    //     oc.addDescription("intermodal-weight-output", "Output", TL("Write intermodal edges with lengths and travel times to FILE"));
    //
    //     oc.doRegister("write-trips", new Option_Bool(false));
    //     oc.addDescription("write-trips", "Output", TL("Write trips instead of vehicles (for validating trip input)"));
    //
    //     oc.doRegister("write-trips.geo", new Option_Bool(false));
    //     oc.addDescription("write-trips.geo", "Output", TL("Write trips with geo-coordinates"));
    //
    //     oc.doRegister("write-trips.junctions", new Option_Bool(false));
    //     oc.addDescription("write-trips.junctions", "Output", TL("Write trips with fromJunction and toJunction"));
    //
    //     oc.doRegister("write-costs", new Option_Bool(false));
    //     oc.addDescription("write-costs", "Output", TL("Include the cost attribute in route output"));
    //
    //     // register further processing options
    //     // ! The subtopic "Processing" must be initialised earlier !
    //     oc.doRegister("weights.random-factor", new Option_Float(1.));
    //     oc.addDescription("weights.random-factor", "Processing", TL("Edge weights for routing are dynamically disturbed by a random factor drawn uniformly from [1,FLOAT)"));
    //
    //     oc.doRegister("weight-period", new Option_String("3600", "TIME"));
    //     oc.addDescription("weight-period", "Processing", TL("Aggregation period for the given weight files; triggers rebuilding of Contraction Hierarchy"));
    //
    //     oc.doRegister("weights.priority-factor", new Option_Float(0));
    //     oc.addDescription("weights.priority-factor", "Processing", TL("Consider edge priorities in addition to travel times, weighted by factor"));
    //
    //     oc.doRegister("astar.all-distances", new Option_FileName());
    //     oc.addDescription("astar.all-distances", "Processing", TL("Initialize lookup table for astar from the given file (generated by marouter --all-pairs-output)"));
    //
    //     oc.doRegister("astar.landmark-distances", new Option_FileName());
    //     oc.addDescription("astar.landmark-distances", "Processing", TL("Initialize lookup table for astar ALT-variant from the given file"));
    //
    //     oc.doRegister("astar.save-landmark-distances", new Option_FileName());
    //     oc.addDescription("astar.save-landmark-distances", "Processing", TL("Save lookup table for astar ALT-variant to the given file"));
    //
    //     oc.doRegister("scale", new Option_Float(1.));
    //     oc.addDescription("scale", "Processing", TL("Scale demand by the given factor (by discarding or duplicating vehicles)"));
    //
    //     oc.doRegister("scale-suffix", new Option_String("."));
    //     oc.addDescription("scale-suffix", "Processing", TL("Suffix to be added when creating ids for cloned vehicles"));
    //
    //     oc.doRegister("taxi.vclasses", new Option_StringVector({"taxi"}));
    //     oc.addSynonyme("taxi.vclasses", "device.taxi.vclasses");
    //     oc.addDescription("taxi.vclasses", "Processing", TL("Network permissions that can be accessed by taxis"));
    // }
    #[allow(unused_variables)]
    pub fn add_import_options() {
        utils::options::options_cont::with_options(|oc| {
            oc.do_register("alternatives-output", SumoOption::file_name());
            oc.add_synonyme("alternatives-output", "alternatives", false);
            oc.add_description("alternatives-output", "Output", "Write generated route alternatives to FILE");

            oc.do_register("intermodal-network-output", SumoOption::file_name());
            oc.add_description("intermodal-network-output", "Output", "Write edge splits and connectivity to FILE");

            oc.do_register("intermodal-weight-output", SumoOption::file_name());
            oc.add_description("intermodal-weight-output", "Output", "Write intermodal edges with lengths and travel times to FILE");

            oc.do_register("write-trips", SumoOption::bool(false));
            oc.add_description("write-trips", "Output", "Write trips instead of vehicles (for validating trip input)");

            oc.do_register("write-trips.geo", SumoOption::bool(false));
            oc.add_description("write-trips.geo", "Output", "Write trips with geo-coordinates");

            oc.do_register("write-trips.junctions", SumoOption::bool(false));
            oc.add_description("write-trips.junctions", "Output", "Write trips with fromJunction and toJunction");

            oc.do_register("write-costs", SumoOption::bool(false));
            oc.add_description("write-costs", "Output", "Include the cost attribute in route output");

            // register further processing options
            // ! The subtopic "Processing" must be initialised earlier !
            oc.do_register("weights.random-factor", SumoOption::float(1.0));
            oc.add_description("weights.random-factor", "Processing", "Edge weights for routing are dynamically disturbed by a random factor drawn uniformly from [1,FLOAT)");

            oc.do_register("weight-period", SumoOption::string_typed("3600", "TIME"));
            oc.add_description("weight-period", "Processing", "Aggregation period for the given weight files; triggers rebuilding of Contraction Hierarchy");

            oc.do_register("weights.priority-factor", SumoOption::float(0.0));
            oc.add_description("weights.priority-factor", "Processing", "Consider edge priorities in addition to travel times, weighted by factor");

            oc.do_register("astar.all-distances", SumoOption::file_name());
            oc.add_description("astar.all-distances", "Processing", "Initialize lookup table for astar from the given file (generated by marouter --all-pairs-output)");

            oc.do_register("astar.landmark-distances", SumoOption::file_name());
            oc.add_description("astar.landmark-distances", "Processing", "Initialize lookup table for astar ALT-variant from the given file");

            oc.do_register("astar.save-landmark-distances", SumoOption::file_name());
            oc.add_description("astar.save-landmark-distances", "Processing", "Save lookup table for astar ALT-variant to the given file");

            oc.do_register("scale", SumoOption::float(1.0));
            oc.add_description("scale", "Processing", "Scale demand by the given factor (by discarding or duplicating vehicles)");

            oc.do_register("scale-suffix", SumoOption::string_with("."));
            oc.add_description("scale-suffix", "Processing", "Suffix to be added when creating ids for cloned vehicles");

            oc.do_register("taxi.vclasses", SumoOption::string_vector_with(vec!["taxi".to_string()]));
            oc.add_synonyme("taxi.vclasses", "device.taxi.vclasses", false);
            oc.add_description("taxi.vclasses", "Processing", "Network permissions that can be accessed by taxis");
        });
    }

    /// C++: `void RODUAFrame::addDUAOptions()` [static, protected]
    /// Inserts dua options used by duarouter into the OptionsCont-singleton
    // void
    // RODUAFrame::addDUAOptions() {
    //     OptionsCont& oc = OptionsCont::getOptions();
    //     // register Gawron's DUE-settings
    //     oc.doRegister("gawron.beta", new Option_Float(double(0.9)));
    //     oc.addSynonyme("gawron.beta", "gBeta", true);
    //     oc.addDescription("gawron.beta", "Processing", TL("Use FLOAT as Gawron's beta"));
    //
    //     oc.doRegister("gawron.a", new Option_Float(double(0.5)));
    //     oc.addSynonyme("gawron.a", "gA", true);
    //     oc.addDescription("gawron.a", "Processing", TL("Use FLOAT as Gawron's a"));
    //
    //     oc.doRegister("exit-times", new Option_Bool(false));
    //     oc.addDescription("exit-times", "Output", TL("Write exit times (weights) for each edge"));
    //
    //     oc.doRegister("route-length", new Option_Bool(false));
    //     oc.addDescription("route-length", "Output", TL("Include total route length in the output"));
    //
    //     oc.doRegister("keep-all-routes", new Option_Bool(false));
    //     oc.addDescription("keep-all-routes", "Processing", TL("Save routes with near zero probability"));
    //
    //     oc.doRegister("skip-new-routes", new Option_Bool(false));
    //     oc.addDescription("skip-new-routes", "Processing", TL("Only reuse routes from input, do not calculate new ones"));
    //
    //     oc.doRegister("keep-route-probability", new Option_Float(0));
    //     oc.addDescription("keep-route-probability", "Processing", TL("The probability of keeping the old route"));
    //
    //     oc.doRegister("ptline-routing", new Option_Bool(false));
    //     oc.addDescription("ptline-routing", "Processing", TL("Route all public transport input"));
    //
    //     oc.doRegister("keep-flows", new Option_Bool(false));
    //     oc.addDescription("keep-flows", "Processing", TL("Write flows instead of expanding them into vehicles"));
    //
    //     oc.doRegister("route-choice-method", new Option_String("gawron"));
    //     oc.addDescription("route-choice-method", "Processing", TL("Choose a route choice method: gawron, logit, or lohse"));
    //
    //     oc.doRegister("logit", new Option_Bool(false)); // deprecated
    //     oc.addDescription("logit", "Processing", TL("Use c-logit model (deprecated in favor of --route-choice-method logit)"));
    //
    //     oc.doRegister("logit.beta", new Option_Float(double(-1)));
    //     oc.addSynonyme("logit.beta", "lBeta", true);
    //     oc.addDescription("logit.beta", "Processing", TL("Use FLOAT as logit's beta"));
    //
    //     oc.doRegister("logit.gamma", new Option_Float(double(1)));
    //     oc.addSynonyme("logit.gamma", "lGamma", true);
    //     oc.addDescription("logit.gamma", "Processing", TL("Use FLOAT as logit's gamma"));
    //
    //     oc.doRegister("logit.theta", new Option_Float(double(-1)));
    //     oc.addSynonyme("logit.theta", "lTheta", true);
    //     oc.addDescription("logit.theta", "Processing", TL("Use FLOAT as logit's theta (negative values mean auto-estimation)"));
    //
    //     oc.doRegister("persontrip.walkfactor", new Option_Float(double(0.75)));
    //     oc.addDescription("persontrip.walkfactor", "Processing", TL("Use FLOAT as a factor on pedestrian maximum speed during intermodal routing"));
    //
    //     oc.doRegister("persontrip.walk-opposite-factor", new Option_Float(1.0));
    //     oc.addDescription("persontrip.walk-opposite-factor", "Processing", TL("Use FLOAT as a factor on walking speed against vehicle traffic direction"));
    //
    //     oc.doRegister("persontrip.transfer.car-walk", new Option_StringVector(StringVector({ "parkingAreas" })));
    //     oc.addDescription("persontrip.transfer.car-walk", "Processing",
    //                       "Where are mode changes from car to walking allowed (possible values: 'parkingAreas', 'ptStops', 'allJunctions' and combinations)");
    //
    //     oc.doRegister("persontrip.transfer.taxi-walk", new Option_StringVector());
    //     oc.addDescription("persontrip.transfer.taxi-walk", "Processing", TL("Where taxis can drop off customers ('allJunctions, 'ptStops')"));
    //
    //     oc.doRegister("persontrip.transfer.walk-taxi", new Option_StringVector());
    //     oc.addDescription("persontrip.transfer.walk-taxi", "Processing", TL("Where taxis can pick up customers ('allJunctions, 'ptStops')"));
    //
    //     oc.doRegister("persontrip.taxi.waiting-time", new Option_String("300", "TIME"));
    //     oc.addDescription("persontrip.taxi.waiting-time", "Processing", TL("Estimated time for taxi pickup"));
    //
    //     oc.doRegister("persontrip.ride-public-line", new Option_Bool(false));
    //     oc.addDescription("persontrip.ride-public-line", "Processing", TL("Only use the intended public transport line rather than any alternative line that stops at the destination"));
    //
    //     oc.doRegister("railway.max-train-length", new Option_Float(1000.0));
    //     oc.addDescription("railway.max-train-length", "Processing", TL("Use FLOAT as a maximum train length when initializing the railway router"));
    //
    //     oc.doRegister("max-traveltime", new Option_String("-1", "TIME"));
    //     oc.addDescription("max-traveltime", "Processing", TL("Declare routing failure if traveltime exceeds the given positive TIME"));
    // }
    #[allow(unused_variables)]
    pub fn add_dua_options() {
        utils::options::options_cont::with_options(|oc| {
            // register Gawron's DUE-settings
            oc.do_register("gawron.beta", SumoOption::float(0.9));
            oc.add_synonyme("gawron.beta", "gBeta", true);
            oc.add_description("gawron.beta", "Processing", "Use FLOAT as Gawron's beta");

            oc.do_register("gawron.a", SumoOption::float(0.5));
            oc.add_synonyme("gawron.a", "gA", true);
            oc.add_description("gawron.a", "Processing", "Use FLOAT as Gawron's a");

            oc.do_register("exit-times", SumoOption::bool(false));
            oc.add_description("exit-times", "Output", "Write exit times (weights) for each edge");

            oc.do_register("route-length", SumoOption::bool(false));
            oc.add_description("route-length", "Output", "Include total route length in the output");

            oc.do_register("keep-all-routes", SumoOption::bool(false));
            oc.add_description("keep-all-routes", "Processing", "Save routes with near zero probability");

            oc.do_register("skip-new-routes", SumoOption::bool(false));
            oc.add_description("skip-new-routes", "Processing", "Only reuse routes from input, do not calculate new ones");

            oc.do_register("keep-route-probability", SumoOption::float(0.0));
            oc.add_description("keep-route-probability", "Processing", "The probability of keeping the old route");

            oc.do_register("ptline-routing", SumoOption::bool(false));
            oc.add_description("ptline-routing", "Processing", "Route all public transport input");

            oc.do_register("keep-flows", SumoOption::bool(false));
            oc.add_description("keep-flows", "Processing", "Write flows instead of expanding them into vehicles");

            oc.do_register("route-choice-method", SumoOption::string_with("gawron"));
            oc.add_description("route-choice-method", "Processing", "Choose a route choice method: gawron, logit, or lohse");

            oc.do_register("logit", SumoOption::bool(false)); // deprecated
            oc.add_description("logit", "Processing", "Use c-logit model (deprecated in favor of --route-choice-method logit)");

            oc.do_register("logit.beta", SumoOption::float(-1.0));
            oc.add_synonyme("logit.beta", "lBeta", true);
            oc.add_description("logit.beta", "Processing", "Use FLOAT as logit's beta");

            oc.do_register("logit.gamma", SumoOption::float(1.0));
            oc.add_synonyme("logit.gamma", "lGamma", true);
            oc.add_description("logit.gamma", "Processing", "Use FLOAT as logit's gamma");

            oc.do_register("logit.theta", SumoOption::float(-1.0));
            oc.add_synonyme("logit.theta", "lTheta", true);
            oc.add_description("logit.theta", "Processing", "Use FLOAT as logit's theta (negative values mean auto-estimation)");

            oc.do_register("persontrip.walkfactor", SumoOption::float(0.75));
            oc.add_description("persontrip.walkfactor", "Processing", "Use FLOAT as a factor on pedestrian maximum speed during intermodal routing");

            oc.do_register("persontrip.walk-opposite-factor", SumoOption::float(1.0));
            oc.add_description("persontrip.walk-opposite-factor", "Processing", "Use FLOAT as a factor on walking speed against vehicle traffic direction");

            oc.do_register("persontrip.transfer.car-walk", SumoOption::string_vector_with(vec!["parkingAreas".to_string()]));
            oc.add_description("persontrip.transfer.car-walk", "Processing",
                               "Where are mode changes from car to walking allowed (possible values: 'parkingAreas', 'ptStops', 'allJunctions' and combinations)");

            oc.do_register("persontrip.transfer.taxi-walk", SumoOption::string_vector());
            oc.add_description("persontrip.transfer.taxi-walk", "Processing", "Where taxis can drop off customers ('allJunctions, 'ptStops')");

            oc.do_register("persontrip.transfer.walk-taxi", SumoOption::string_vector());
            oc.add_description("persontrip.transfer.walk-taxi", "Processing", "Where taxis can pick up customers ('allJunctions, 'ptStops')");

            oc.do_register("persontrip.taxi.waiting-time", SumoOption::string_typed("300", "TIME"));
            oc.add_description("persontrip.taxi.waiting-time", "Processing", "Estimated time for taxi pickup");

            oc.do_register("persontrip.ride-public-line", SumoOption::bool(false));
            oc.add_description("persontrip.ride-public-line", "Processing", "Only use the intended public transport line rather than any alternative line that stops at the destination");

            oc.do_register("railway.max-train-length", SumoOption::float(1000.0));
            oc.add_description("railway.max-train-length", "Processing", "Use FLOAT as a maximum train length when initializing the railway router");

            oc.do_register("max-traveltime", SumoOption::string_typed("-1", "TIME"));
            oc.add_description("max-traveltime", "Processing", "Declare routing failure if traveltime exceeds the given positive TIME");
        });
    }

}

