//! Port of dfrouter/RODFFrame.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Sets and checks options for df-routing
/// C++ class `RODFFrame` (dfrouter/RODFFrame.h)
pub struct RODFFrame {
}

impl RODFFrame {
    /// C++: `void RODFFrame::fillOptions()` [static]
    /// Inserts options used by dfrouter into the OptionsCont-singleton
    // void
    // RODFFrame::fillOptions() {
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
    //     // register the options
    //     // register input-options
    //     oc.doRegister("net-file", 'n', new Option_FileName());
    //     oc.addSynonyme("net-file", "net");
    //     oc.addDescription("net-file", "Input", TL("Loads the SUMO-network FILE"));
    //
    //     /* never used
    //     oc.doRegister("route-files", 'r', new Option_FileName());
    //     oc.addDescription("route-files", "Input", TL("Loads SUMO-routes from FILE"));
    //     */
    //
    //     oc.doRegister("detector-files", 'd', new Option_FileName());
    //     oc.addSynonyme("detector-files", "detectors");
    //     oc.addDescription("detector-files", "Input", TL("Loads detector descriptions from FILE"));
    //
    //     oc.doRegister("measure-files", 'f', new Option_FileName());
    //     oc.addSynonyme("measure-files", "detflows");
    //     oc.addSynonyme("measure-files", "detector-flow-files", true);
    //     oc.addDescription("measure-files", "Input", TL("Loads detector flows from FILE(s)"));
    //
    //     // need to do this here to be able to check for network and route input options
    //     SystemFrame::addReportOptions(oc);
    //
    //     // register output options
    //     oc.doRegister("routes-output", 'o', new Option_FileName());
    //     oc.addDescription("routes-output", "Output", TL("Saves computed routes to FILE"));
    //
    //     oc.doRegister("routes-for-all", new Option_Bool(false));
    //     oc.addDescription("routes-for-all", "Output", TL("Forces dfrouter to compute routes for in-between detectors"));
    //
    //     oc.doRegister("detector-output", new Option_FileName());
    //     oc.addSynonyme("detector-output", "detectors-output", true);
    //     oc.addDescription("detector-output", "Output", TL("Saves typed detectors to FILE"));
    //
    //     oc.doRegister("detectors-poi-output", new Option_FileName());
    //     oc.addDescription("detectors-poi-output", "Output", TL("Saves detector positions as pois to FILE"));
    //
    //     oc.doRegister("emitters-output", new Option_FileName());
    //     oc.addDescription("emitters-output", "Output", TL("Saves emitter definitions for source detectors to FILE"));
    //
    //     oc.doRegister("vtype", new Option_Bool(false));
    //     oc.addDescription("vtype", "Output", TL("Add vehicle types to the emitters file (PKW, LKW)"));
    //
    //     oc.doRegister("vtype-output", new Option_FileName(StringVector({ "" })));
    //     oc.addDescription("vtype-output", "Output", TL("Write generated vehicle types into separate FILE instead of including them into the emitters-output"));
    //
    //     oc.doRegister("emitters-poi-output", new Option_FileName()); // !!! describe
    //     oc.addDescription("emitters-poi-output", "Output", TL("Saves emitter positions as pois to FILE"));
    //
    //     oc.doRegister("variable-speed-sign-output", new Option_FileName());
    //     oc.addSynonyme("variable-speed-sign-output", "speed-trigger-output", true);
    //     oc.addDescription("variable-speed-sign-output", "Output", TL("Saves variable seed sign definitions for sink detectors to FILE"));
    //
    //     oc.doRegister("end-reroute-output", new Option_FileName());
    //     oc.addDescription("end-reroute-output", "Output", TL("Saves rerouter definitions for sink detectors to FILE"));
    //
    //     oc.doRegister("validation-output", new Option_FileName());
    //     oc.addDescription("validation-output", "Output", TL("Write induction loop definitions for the given detector positions to FILE"));
    //
    //     oc.doRegister("validation-output.add-sources", new Option_Bool(false));
    //     oc.addDescription("validation-output.add-sources", "Output", TL("Add source detectors to the validation output"));
    //
    //
    //     // register processing options
    //     // to guess empty flows
    //     oc.doRegister("guess-empty-flows", new Option_Bool(false));
    //     oc.addDescription("guess-empty-flows", "Processing", TL("Derive missing flow values from upstream or downstream (not working!)"));
    //
    //     // for guessing source/sink detectors
    //     oc.doRegister("highway-mode", 'h', new Option_Bool(false)); // !!! describe
    //     oc.addDescription("highway-mode", "Processing", TL("Switches to highway-mode"));
    //
    //     // for detector reading
    //     oc.doRegister("ignore-invalid-detectors", new Option_Bool(false));
    //     oc.addDescription("ignore-invalid-detectors", "Processing", TL("Only warn about unparseable detectors"));
    //
    //     // for detector type computation
    //     oc.doRegister("revalidate-detectors", new Option_Bool(false));
    //     oc.addDescription("revalidate-detectors", "Processing", TL("Recomputes detector types even if given"));
    //
    //     // for route computation
    //     oc.doRegister("revalidate-routes", new Option_Bool(false));
    //     oc.addDescription("revalidate-routes", "Processing", TL("Recomputes routes even if given"));
    //
    //     oc.doRegister("keep-unfinished-routes", new Option_Bool(false));
    //     oc.addSynonyme("keep-unfinished-routes", "keep-unfound-ends", true);
    //     oc.addDescription("keep-unfinished-routes", "Processing", TL("Keeps routes even if they have exhausted max-search-depth"));
    //
    //     oc.doRegister("keep-longer-routes", new Option_Bool(false));
    //     oc.addDescription("keep-longer-routes", "Processing", TL("Keeps routes even if a shorter one exists"));
    //
    //     oc.doRegister("max-search-depth", new Option_Integer(30));
    //     oc.addSynonyme("max-search-depth", "max-nodet-follower", true);
    //     oc.addDescription("max-search-depth", "Processing", TL("Number of edges to follow a route without passing a detector"));
    //
    //     oc.doRegister("emissions-only", new Option_Bool(false));
    //     oc.addDescription("emissions-only", "Processing", TL("Writes only emission times"));
    //
    //     oc.doRegister("disallowed-edges", new Option_StringVector(StringVector({ "" })));
    //     oc.addDescription("disallowed-edges", "Processing", TL("Do not route on these edges"));
    //
    //     oc.doRegister("vclass", new Option_String("ignoring"));
    //     oc.addDescription("vclass", "Processing", TL("Only route on edges allowing the given vclass"));
    //
    //     oc.doRegister("keep-turnarounds", new Option_Bool(false));
    //     oc.addDescription("keep-turnarounds", "Processing", TL("Allow turnarounds as route continuations"));
    //
    //     oc.doRegister("min-route-length", new Option_Float(-1));
    //     oc.addSynonyme("min-route-length", "min-dist", true);
    //     oc.addSynonyme("min-route-length", "min-distance", true);
    //     oc.addDescription("min-route-length", "Processing", TL("Minimum distance in meters between start and end node of every route"));
    //
    //     oc.doRegister("randomize-flows", new Option_Bool(false));
    //     oc.addDescription("randomize-flows", "Processing", TL("generate random departure times for emitted vehicles"));
    //
    //     // flow reading
    //     oc.doRegister("time-factor", new Option_String("60", "TIME"));
    //     oc.addDescription("time-factor", "Processing", TL("Multiply flow times with TIME to get seconds"));
    //
    //     oc.doRegister("time-offset", new Option_String("0", "TIME"));
    //     oc.addDescription("time-offset", "Processing", TL("Subtracts TIME seconds from (scaled) flow times"));
    //
    //     oc.doRegister("time-step", new Option_String("60", "TIME"));
    //     oc.addDescription("time-step", "Processing", TL("Expected distance between two successive data sets"));
    //
    //     // saving further structures
    //     oc.doRegister("calibrator-output", new Option_Bool(false));
    //     oc.addSynonyme("calibrator-output", "write-calibrators", true);
    //     oc.addDescription("calibrator-output", "Processing", TL("Write calibrators to FILE"));
    //
    //     oc.doRegister("include-unused-routes", new Option_Bool(false));
    //     oc.addDescription("include-unused-routes", "Processing", TL("Include routes in the output which have no vehicles"));
    //
    //     //
    //     oc.doRegister("revalidate-flows", new Option_Bool(false));
    //     oc.addDescription("revalidate-flows", "Processing", TL("Checks whether detectors with calculated flow 0 can attract additional traffic"));
    //
    //     oc.doRegister("remove-empty-detectors", new Option_Bool(false));
    //     oc.addDescription("remove-empty-detectors", "Processing", TL("Removes empty detectors from the list"));
    //
    //     oc.doRegister("strict-sources", new Option_Bool(false));
    //     oc.addDescription("strict-sources", "Processing", TL("Whether edges with unknown status can prevent an edge from becoming a source"));
    //
    //     oc.doRegister("respect-concurrent-inflows", new Option_Bool(false));
    //     oc.addDescription("respect-concurrent-inflows", "Processing", TL("Try to determine further inflows to an inbetween detector when computing split probabilities"));
    //
    //     //
    //     oc.doRegister("scale", new Option_Float(1.));
    //     oc.addDescription("scale", "Processing", TL("Scale factor for flows"));
    //
    //     // register defaults options
    //     oc.doRegister("departlane", new Option_String());
    //     oc.addDescription("departlane", "Defaults", TL("Assigns a default depart lane"));
    //
    //     oc.doRegister("departpos", new Option_String());
    //     oc.addDescription("departpos", "Defaults", TL("Assigns a default depart position"));
    //
    //     oc.doRegister("departspeed", new Option_String());
    //     oc.addDescription("departspeed", "Defaults", TL("Assigns a default depart speed"));
    //
    //     oc.doRegister("arrivallane", new Option_String());
    //     oc.addDescription("arrivallane", "Defaults", TL("Assigns a default arrival lane"));
    //
    //     oc.doRegister("arrivalpos", new Option_String());
    //     oc.addDescription("arrivalpos", "Defaults", TL("Assigns a default arrival position"));
    //
    //     oc.doRegister("arrivalspeed", new Option_String());
    //     oc.addDescription("arrivalspeed", "Defaults", TL("Assigns a default arrival speed"));
    //
    //     oc.doRegister("speeddev", new Option_Float(0.1));
    //     oc.addDescription("speeddev", "Defaults", TL("The default speed deviation of vehicles"));
    //
    //     // register the simulation settings
    //     oc.doRegister("begin", 'b', new Option_String("0", "TIME"));
    //     oc.addDescription("begin", "Time", TL("Defines the begin time; Previous defs will be discarded"));
    //
    //     oc.doRegister("end", 'e', new Option_String("86400", "TIME"));
    //     oc.addDescription("end", "Time", TL("Defines the end time; Later defs will be discarded; Defaults to one day"));
    //
    //
    //     // register report options
    //     oc.doRegister("report-empty-detectors", new Option_Bool(false));
    //     oc.addDescription("report-empty-detectors", "Report", TL("Lists detectors with no flow (enable -v)"));
    //
    //     oc.doRegister("print-absolute-flows", new Option_Bool(false));
    //     oc.addDescription("print-absolute-flows", "Report", TL("Prints aggregated detector flows"));
    //
    //     // register report options
    //     oc.doRegister("no-step-log", new Option_Bool(false));
    //     oc.addDescription("no-step-log", "Report", TL("Disable console output of route parsing step"));
    //
    //     RandHelper::insertRandOptions(oc);
    // }
    #[allow(unused_variables)]
    pub fn fill_options() {
        utils::options::options_cont::with_options(|oc| {
            // insert options sub-topics
            utils::common::system_frame::add_configuration_options(oc);
            oc.add_option_sub_topic("Input");
            oc.add_option_sub_topic("Output");
            oc.add_option_sub_topic("Processing");
            oc.add_option_sub_topic("Defaults");
            oc.add_option_sub_topic("Time");

            // register the options
            // register input-options
            oc.do_register_abbr("net-file", 'n', SumoOption::file_name());
            oc.add_synonyme("net-file", "net", false);
            oc.add_description("net-file", "Input", "Loads the SUMO-network FILE");

            oc.do_register_abbr("detector-files", 'd', SumoOption::file_name());
            oc.add_synonyme("detector-files", "detectors", false);
            oc.add_description("detector-files", "Input", "Loads detector descriptions from FILE");

            oc.do_register_abbr("measure-files", 'f', SumoOption::file_name());
            oc.add_synonyme("measure-files", "detflows", false);
            oc.add_synonyme("measure-files", "detector-flow-files", true);
            oc.add_description("measure-files", "Input", "Loads detector flows from FILE(s)");

            // need to do this here to be able to check for network and route input options
            utils::common::system_frame::add_report_options(oc);

            // register output options
            oc.do_register_abbr("routes-output", 'o', SumoOption::file_name());
            oc.add_description("routes-output", "Output", "Saves computed routes to FILE");

            oc.do_register("routes-for-all", SumoOption::bool(false));
            oc.add_description("routes-for-all", "Output", "Forces dfrouter to compute routes for in-between detectors");

            oc.do_register("detector-output", SumoOption::file_name());
            oc.add_synonyme("detector-output", "detectors-output", true);
            oc.add_description("detector-output", "Output", "Saves typed detectors to FILE");

            oc.do_register("detectors-poi-output", SumoOption::file_name());
            oc.add_description("detectors-poi-output", "Output", "Saves detector positions as pois to FILE");

            oc.do_register("emitters-output", SumoOption::file_name());
            oc.add_description("emitters-output", "Output", "Saves emitter definitions for source detectors to FILE");

            oc.do_register("vtype", SumoOption::bool(false));
            oc.add_description("vtype", "Output", "Add vehicle types to the emitters file (PKW, LKW)");

            oc.do_register("vtype-output", SumoOption::file_name_with(vec![String::new()]));
            oc.add_description("vtype-output", "Output", "Write generated vehicle types into separate FILE instead of including them into the emitters-output");

            oc.do_register("emitters-poi-output", SumoOption::file_name()); // !!! describe
            oc.add_description("emitters-poi-output", "Output", "Saves emitter positions as pois to FILE");

            oc.do_register("variable-speed-sign-output", SumoOption::file_name());
            oc.add_synonyme("variable-speed-sign-output", "speed-trigger-output", true);
            oc.add_description("variable-speed-sign-output", "Output", "Saves variable seed sign definitions for sink detectors to FILE");

            oc.do_register("end-reroute-output", SumoOption::file_name());
            oc.add_description("end-reroute-output", "Output", "Saves rerouter definitions for sink detectors to FILE");

            oc.do_register("validation-output", SumoOption::file_name());
            oc.add_description("validation-output", "Output", "Write induction loop definitions for the given detector positions to FILE");

            oc.do_register("validation-output.add-sources", SumoOption::bool(false));
            oc.add_description("validation-output.add-sources", "Output", "Add source detectors to the validation output");

            // register processing options
            // to guess empty flows
            oc.do_register("guess-empty-flows", SumoOption::bool(false));
            oc.add_description("guess-empty-flows", "Processing", "Derive missing flow values from upstream or downstream (not working!)");

            // for guessing source/sink detectors
            oc.do_register_abbr("highway-mode", 'h', SumoOption::bool(false)); // !!! describe
            oc.add_description("highway-mode", "Processing", "Switches to highway-mode");

            // for detector reading
            oc.do_register("ignore-invalid-detectors", SumoOption::bool(false));
            oc.add_description("ignore-invalid-detectors", "Processing", "Only warn about unparseable detectors");

            // for detector type computation
            oc.do_register("revalidate-detectors", SumoOption::bool(false));
            oc.add_description("revalidate-detectors", "Processing", "Recomputes detector types even if given");

            // for route computation
            oc.do_register("revalidate-routes", SumoOption::bool(false));
            oc.add_description("revalidate-routes", "Processing", "Recomputes routes even if given");

            oc.do_register("keep-unfinished-routes", SumoOption::bool(false));
            oc.add_synonyme("keep-unfinished-routes", "keep-unfound-ends", true);
            oc.add_description("keep-unfinished-routes", "Processing", "Keeps routes even if they have exhausted max-search-depth");

            oc.do_register("keep-longer-routes", SumoOption::bool(false));
            oc.add_description("keep-longer-routes", "Processing", "Keeps routes even if a shorter one exists");

            oc.do_register("max-search-depth", SumoOption::integer(30));
            oc.add_synonyme("max-search-depth", "max-nodet-follower", true);
            oc.add_description("max-search-depth", "Processing", "Number of edges to follow a route without passing a detector");

            oc.do_register("emissions-only", SumoOption::bool(false));
            oc.add_description("emissions-only", "Processing", "Writes only emission times");

            oc.do_register("disallowed-edges", SumoOption::string_vector_with(vec![String::new()]));
            oc.add_description("disallowed-edges", "Processing", "Do not route on these edges");

            oc.do_register("vclass", SumoOption::string_with("ignoring"));
            oc.add_description("vclass", "Processing", "Only route on edges allowing the given vclass");

            oc.do_register("keep-turnarounds", SumoOption::bool(false));
            oc.add_description("keep-turnarounds", "Processing", "Allow turnarounds as route continuations");

            oc.do_register("min-route-length", SumoOption::float(-1.0));
            oc.add_synonyme("min-route-length", "min-dist", true);
            oc.add_synonyme("min-route-length", "min-distance", true);
            oc.add_description("min-route-length", "Processing", "Minimum distance in meters between start and end node of every route");

            oc.do_register("randomize-flows", SumoOption::bool(false));
            oc.add_description("randomize-flows", "Processing", "generate random departure times for emitted vehicles");

            // flow reading
            oc.do_register("time-factor", SumoOption::string_typed("60", "TIME"));
            oc.add_description("time-factor", "Processing", "Multiply flow times with TIME to get seconds");

            oc.do_register("time-offset", SumoOption::string_typed("0", "TIME"));
            oc.add_description("time-offset", "Processing", "Subtracts TIME seconds from (scaled) flow times");

            oc.do_register("time-step", SumoOption::string_typed("60", "TIME"));
            oc.add_description("time-step", "Processing", "Expected distance between two successive data sets");

            // saving further structures
            oc.do_register("calibrator-output", SumoOption::bool(false));
            oc.add_synonyme("calibrator-output", "write-calibrators", true);
            oc.add_description("calibrator-output", "Processing", "Write calibrators to FILE");

            oc.do_register("include-unused-routes", SumoOption::bool(false));
            oc.add_description("include-unused-routes", "Processing", "Include routes in the output which have no vehicles");

            //
            oc.do_register("revalidate-flows", SumoOption::bool(false));
            oc.add_description("revalidate-flows", "Processing", "Checks whether detectors with calculated flow 0 can attract additional traffic");

            oc.do_register("remove-empty-detectors", SumoOption::bool(false));
            oc.add_description("remove-empty-detectors", "Processing", "Removes empty detectors from the list");

            oc.do_register("strict-sources", SumoOption::bool(false));
            oc.add_description("strict-sources", "Processing", "Whether edges with unknown status can prevent an edge from becoming a source");

            oc.do_register("respect-concurrent-inflows", SumoOption::bool(false));
            oc.add_description("respect-concurrent-inflows", "Processing", "Try to determine further inflows to an inbetween detector when computing split probabilities");

            //
            oc.do_register("scale", SumoOption::float(1.0));
            oc.add_description("scale", "Processing", "Scale factor for flows");

            // register defaults options
            oc.do_register("departlane", SumoOption::string());
            oc.add_description("departlane", "Defaults", "Assigns a default depart lane");

            oc.do_register("departpos", SumoOption::string());
            oc.add_description("departpos", "Defaults", "Assigns a default depart position");

            oc.do_register("departspeed", SumoOption::string());
            oc.add_description("departspeed", "Defaults", "Assigns a default depart speed");

            oc.do_register("arrivallane", SumoOption::string());
            oc.add_description("arrivallane", "Defaults", "Assigns a default arrival lane");

            oc.do_register("arrivalpos", SumoOption::string());
            oc.add_description("arrivalpos", "Defaults", "Assigns a default arrival position");

            oc.do_register("arrivalspeed", SumoOption::string());
            oc.add_description("arrivalspeed", "Defaults", "Assigns a default arrival speed");

            oc.do_register("speeddev", SumoOption::float(0.1));
            oc.add_description("speeddev", "Defaults", "The default speed deviation of vehicles");

            // register the simulation settings
            oc.do_register_abbr("begin", 'b', SumoOption::string_typed("0", "TIME"));
            oc.add_description("begin", "Time", "Defines the begin time; Previous defs will be discarded");

            oc.do_register_abbr("end", 'e', SumoOption::string_typed("86400", "TIME"));
            oc.add_description("end", "Time", "Defines the end time; Later defs will be discarded; Defaults to one day");

            //
            // register report options
            oc.do_register("report-empty-detectors", SumoOption::bool(false));
            oc.add_description("report-empty-detectors", "Report", "Lists detectors with no flow (enable -v)");

            oc.do_register("print-absolute-flows", SumoOption::bool(false));
            oc.add_description("print-absolute-flows", "Report", "Prints aggregated detector flows");

            // register report options
            oc.do_register("no-step-log", SumoOption::bool(false));
            oc.add_description("no-step-log", "Report", "Disable console output of route parsing step");

            utils::common::rand_helper::insert_rand_options(oc);
        });
    }

    /// C++: `bool RODFFrame::checkOptions()` [static]
    /// Checks set options from the OptionsCont-singleton for being valid for usage within dfrouter
    // bool
    // RODFFrame::checkOptions() {
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn check_options() -> bool {
        true
    }

}

