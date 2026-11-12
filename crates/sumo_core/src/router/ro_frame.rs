//! Port of router/ROFrame.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Sets and checks options for routing
/// C++ class `ROFrame` (router/ROFrame.h)
pub struct ROFrame {
}

impl ROFrame {
    /// C++: `void ROFrame::fillOptions(OptionsCont &, const bool, const bool)` [static]
    /// Inserts options used by routing applications into the OptionsCont-singleton
    // void
    // ROFrame::fillOptions(OptionsCont& oc, const bool isDUA, const bool isMA) {
    //     // register options
    //     // register I/O options
    //     oc.doRegister("output-file", 'o', new Option_FileName());
    //     oc.addSynonyme("output-file", "output");
    //     oc.addDescription("output-file", "Output", TL("Write generated routes to FILE"));
    //
    //     oc.doRegister("vtype-output", new Option_FileName());
    //     if (!isMA) {
    //         oc.addSynonyme("vtype-output", "vtype");
    //     }
    //     oc.addDescription("vtype-output", "Output", TL("Write used vehicle types into separate FILE"));
    //
    //     oc.doRegister("keep-vtype-distributions", new Option_Bool(false));
    //     oc.addDescription("keep-vtype-distributions", "Output", TL("Keep vTypeDistribution ids when writing vehicles and their types"));
    //
    //     oc.doRegister("emissions.volumetric-fuel", new Option_Bool(false));
    //     oc.addDescription("emissions.volumetric-fuel", "Output", TL("Return fuel consumption values in (legacy) unit l instead of mg"));
    //
    //     oc.doRegister("net-file", 'n', new Option_FileName());
    //     oc.addSynonyme("net-file", "net");
    //     oc.addDescription("net-file", "Input", TL("Use FILE as SUMO-network to route on"));
    //
    //     oc.doRegister("named-routes", new Option_Bool(false));
    //     oc.addDescription("named-routes", "Output", TL("Write vehicles that reference routes by their id"));
    //
    //     oc.doRegister("additional-files", 'a', new Option_FileName());
    //     oc.addSynonyme("additional-files", "d", true);
    //     oc.addSynonyme("additional-files", "additional");
    //     oc.addSynonyme("additional-files", "taz-files");
    //     oc.addSynonyme("additional-files", "districts", true);
    //     oc.addDescription("additional-files", "Input", TL("Read additional network data (districts, bus stops) from FILE(s)"));
    //
    //     oc.doRegister("route-files", 'r', new Option_FileName());
    //     oc.addSynonyme("route-files", "flow-files", true);
    //     oc.addSynonyme("route-files", "flows", true);
    //     oc.addSynonyme("route-files", "f", true);
    //     oc.addSynonyme("route-files", "alternative-files", true);
    //     oc.addSynonyme("route-files", "alternatives-files", true);
    //     oc.addSynonyme("route-files", "trip-files", true);
    //     oc.addSynonyme("route-files", "trips", true);
    //     if (isDUA) {
    //         // to make the transition from --trip-files easier, but has a conflict with jtrrouter
    //         oc.addSynonyme("route-files", "t", true);
    //     }
    //     oc.addDescription("route-files", "Input", TL("Read sumo routes, alternatives, flows, and trips from FILE(s)"));
    //
    //     oc.doRegister("phemlight-path", new Option_FileName(StringVector({ "./PHEMlight/" })));
    //     oc.addDescription("phemlight-path", "Input", TL("Determines where to load PHEMlight definitions from"));
    //
    //     oc.doRegister("phemlight-year", new Option_Integer(0));
    //     oc.addDescription("phemlight-year", "Input", TL("Enable fleet age modeling with the given reference year in PHEMlight5"));
    //
    //     oc.doRegister("phemlight-temperature", new Option_Float(INVALID_DOUBLE));
    //     oc.addDescription("phemlight-temperature", "Input", TL("Set ambient temperature to correct NOx emissions in PHEMlight5"));
    //
    //     if (isDUA || isMA) {
    //         oc.doRegister("weight-files", 'w', new Option_FileName());
    //         oc.addSynonyme("weight-files", "weights");
    //         oc.addDescription("weight-files", "Input", TL("Read network weights from FILE(s)"));
    //
    //         oc.doRegister("lane-weight-files", new Option_FileName());
    //         oc.addDescription("lane-weight-files", "Input", TL("Read lane-based network weights from FILE(s)"));
    //
    //         oc.doRegister("weight-attribute", 'x', new Option_String("traveltime"));
    //         oc.addSynonyme("weight-attribute", "measure", true);
    //         oc.addDescription("weight-attribute", "Input", TL("Name of the xml attribute which gives the edge weight"));
    //     }
    //
    //     // need to do this here to be able to check for network and route input options
    //     SystemFrame::addReportOptions(oc);
    //
    //     // register the time settings
    //     oc.doRegister("begin", 'b', new Option_String("0", "TIME"));
    //     oc.addDescription("begin", "Time", TL("Defines the begin time; Previous trips will be discarded"));
    //
    //     oc.doRegister("end", 'e', new Option_String("-1", "TIME"));
    //     oc.addDescription("end", "Time", TL("Defines the end time; Later trips will be discarded; Defaults to the maximum time that SUMO can represent"));
    //
    //     // register the processing options
    //     oc.doRegister("ignore-errors", new Option_Bool(false));
    //     oc.addSynonyme("ignore-errors", "continue-on-unbuild", true);
    //     oc.addSynonyme("ignore-errors", "dismiss-loading-errors", true);
    //     oc.addDescription("ignore-errors", "Report", TL("Continue if a route could not be build"));
    //
    //     oc.doRegister("max-alternatives", new Option_Integer(5));
    //     oc.addDescription("max-alternatives", "Processing", TL("Prune the number of alternatives to INT"));
    //
    //     oc.doRegister("with-taz", new Option_Bool(false));
    //     oc.addDescription("with-taz", "Processing", TL("Use origin and destination zones (districts) for in- and output"));
    //
    //     oc.doRegister("junction-taz", new Option_Bool(false));
    //     oc.addDescription("junction-taz", "Input", TL("Initialize a TAZ for every junction to use attributes toJunction and fromJunction"));
    //
    //     if (!isMA) {
    //         oc.doRegister("unsorted-input", new Option_Bool(false));
    //         oc.addSynonyme("unsorted-input", "unsorted");
    //         oc.addDescription("unsorted-input", "Processing", TL("Assume input is unsorted"));
    //
    //         oc.doRegister("route-steps", 's', new Option_String("200", "TIME"));
    //         oc.addDescription("route-steps", "Processing", TL("Load routes for the next number of seconds ahead"));
    //
    //         oc.doRegister("no-internal-links", new Option_Bool(false));
    //         oc.addDescription("no-internal-links", "Processing", TL("Disable (junction) internal links"));
    //
    //         oc.doRegister("randomize-flows", new Option_Bool(false));
    //         oc.addDescription("randomize-flows", "Processing", TL("generate random departure times for flow input"));
    //
    //         oc.doRegister("remove-loops", new Option_Bool(false));
    //         oc.addDescription("remove-loops", "Processing", TL("Remove loops within the route; Remove turnarounds at start and end of the route"));
    //
    //         oc.doRegister("repair", new Option_Bool(false));
    //         oc.addDescription("repair", "Processing", TL("Tries to correct a false route"));
    //
    //         oc.doRegister("repair.from", new Option_Bool(false));
    //         oc.addDescription("repair.from", "Processing", TL("Tries to correct an invalid starting edge by using the first usable edge instead"));
    //
    //         oc.doRegister("repair.to", new Option_Bool(false));
    //         oc.addDescription("repair.to", "Processing", TL("Tries to correct an invalid destination edge by using the last usable edge instead"));
    //
    //         oc.doRegister("repair.max-detour-factor", new Option_Float(10));
    //         oc.addDescription("repair.max-detour-factor", "Processing", TL("Backtrack on route if the detour is longer than the gap by FACTOR"));
    //
    //         oc.doRegister("mapmatch.distance", new Option_Float(100));
    //         oc.addDescription("mapmatch.distance", "Processing", TL("Maximum distance when mapping input coordinates (fromXY etc.) to the road network"));
    //
    //         oc.doRegister("mapmatch.junctions", new Option_Bool(false));
    //         oc.addDescription("mapmatch.junctions", "Processing", TL("Match positions to junctions instead of edges"));
    //
    //         oc.doRegister("mapmatch.taz", new Option_Bool(false));
    //         oc.addDescription("mapmatch.taz", "Processing", TL("Match positions to taz instead of edges"));
    //
    //         oc.doRegister("bulk-routing", new Option_Bool(false));
    //         oc.addDescription("bulk-routing", "Processing", TL("Aggregate routing queries with the same origin"));
    //     }
    //
    //     oc.doRegister("routing-threads", new Option_Integer(0));
    //     oc.addDescription("routing-threads", "Processing", TL("The number of parallel execution threads used for routing"));
    //
    //     if (isDUA || isMA) {
    //         oc.doRegister("routing-algorithm", new Option_String("dijkstra"));
    //         oc.addDescription("routing-algorithm", "Processing", TL("Select among routing algorithms ['dijkstra', 'astar', 'CH', 'CHWrapper', 'CCH']"));
    //     }
    //
    //     oc.doRegister("restriction-params", new Option_StringVector());
    //     oc.addDescription("restriction-params", "Processing", TL("Comma separated list of param keys to compare for additional restrictions"));
    //
    //     if (isDUA || isMA) {
    //         oc.doRegister("weights.interpolate", new Option_Bool(false));
    //         oc.addSynonyme("weights.interpolate", "interpolate", true);
    //         oc.addDescription("weights.interpolate", "Processing", TL("Interpolate edge weights at interval boundaries"));
    //
    //         oc.doRegister("weights.expand", new Option_Bool(false));
    //         oc.addSynonyme("weights.expand", "expand-weights", true);
    //         oc.addDescription("weights.expand", "Processing", TL("Expand the end of the last loaded weight interval to infinity"));
    //     }
    //
    //     oc.doRegister("weights.minor-penalty", new Option_Float(1.5));
    //     oc.addDescription("weights.minor-penalty", "Processing", TL("Apply the given time penalty when computing routing costs for minor-link internal lanes"));
    //
    //     oc.doRegister("weights.tls-penalty", new Option_Float(0));
    //     oc.addDescription("weights.tls-penalty", "Processing", TL("Apply the given time penalty when computing routing costs across a traffic light"));
    //
    //     oc.doRegister("weights.turnaround-penalty", new Option_Float(5.0));
    //     oc.addDescription("weights.turnaround-penalty", "Processing", TL("Apply the given time penalty when computing routing costs for turnaround internal lanes"));
    //
    //     oc.doRegister("weights.reversal-penalty", new Option_Float(60));
    //     oc.addDescription("weights.reversal-penalty", "Processing", TL("Apply the given time penalty when computing routing costs for train reversal. Negative values disable reversal"));
    //
    //     if (!isMA) {
    //         // register defaults options
    //         oc.doRegister("departlane", new Option_String());
    //         oc.addDescription("departlane", "Defaults", TL("Assigns a default depart lane"));
    //
    //         oc.doRegister("departpos", new Option_String());
    //         oc.addDescription("departpos", "Defaults", TL("Assigns a default depart position"));
    //
    //         oc.doRegister("departspeed", new Option_String());
    //         oc.addDescription("departspeed", "Defaults", TL("Assigns a default depart speed"));
    //
    //         oc.doRegister("arrivallane", new Option_String());
    //         oc.addDescription("arrivallane", "Defaults", TL("Assigns a default arrival lane"));
    //
    //         oc.doRegister("arrivalpos", new Option_String());
    //         oc.addDescription("arrivalpos", "Defaults", TL("Assigns a default arrival position"));
    //
    //         oc.doRegister("arrivalspeed", new Option_String());
    //         oc.addDescription("arrivalspeed", "Defaults", TL("Assigns a default arrival speed"));
    //
    //     }
    //     oc.doRegister("defaults-override", new Option_Bool(false));
    //     oc.addDescription("defaults-override", "Defaults", TL("Defaults will override given values"));
    //
    //     // register report options
    //     oc.doRegister("stats-period", new Option_Integer(-1));
    //     oc.addDescription("stats-period", "Report", TL("Defines how often statistics shall be printed"));
    //
    //     oc.doRegister("no-step-log", new Option_Bool(false));
    //     oc.addDescription("no-step-log", "Report", TL("Disable console output of route parsing step"));
    // }
    #[allow(unused_variables)]
    pub fn fill_options(oc: &mut OptionsCont, isDUA: bool, isMA: bool) {
        // register options
        // register I/O options
        oc.do_register_abbr("output-file", 'o', SumoOption::file_name());
        oc.add_synonyme("output-file", "output", false);
        oc.add_description("output-file", "Output", "Write generated routes to FILE");

        oc.do_register("vtype-output", SumoOption::file_name());
        if !isMA {
            oc.add_synonyme("vtype-output", "vtype", false);
        }
        oc.add_description("vtype-output", "Output", "Write used vehicle types into separate FILE");

        oc.do_register("keep-vtype-distributions", SumoOption::bool(false));
        oc.add_description("keep-vtype-distributions", "Output", "Keep vTypeDistribution ids when writing vehicles and their types");

        oc.do_register("emissions.volumetric-fuel", SumoOption::bool(false));
        oc.add_description("emissions.volumetric-fuel", "Output", "Return fuel consumption values in (legacy) unit l instead of mg");

        oc.do_register_abbr("net-file", 'n', SumoOption::file_name());
        oc.add_synonyme("net-file", "net", false);
        oc.add_description("net-file", "Input", "Use FILE as SUMO-network to route on");

        oc.do_register("named-routes", SumoOption::bool(false));
        oc.add_description("named-routes", "Output", "Write vehicles that reference routes by their id");

        oc.do_register_abbr("additional-files", 'a', SumoOption::file_name());
        oc.add_synonyme("additional-files", "d", true);
        oc.add_synonyme("additional-files", "additional", false);
        oc.add_synonyme("additional-files", "taz-files", false);
        oc.add_synonyme("additional-files", "districts", true);
        oc.add_description("additional-files", "Input", "Read additional network data (districts, bus stops) from FILE(s)");

        oc.do_register_abbr("route-files", 'r', SumoOption::file_name());
        oc.add_synonyme("route-files", "flow-files", true);
        oc.add_synonyme("route-files", "flows", true);
        oc.add_synonyme("route-files", "f", true);
        oc.add_synonyme("route-files", "alternative-files", true);
        oc.add_synonyme("route-files", "alternatives-files", true);
        oc.add_synonyme("route-files", "trip-files", true);
        oc.add_synonyme("route-files", "trips", true);
        if isDUA {
            // to make the transition from --trip-files easier, but has a conflict with jtrrouter
            oc.add_synonyme("route-files", "t", true);
        }
        oc.add_description("route-files", "Input", "Read sumo routes, alternatives, flows, and trips from FILE(s)");

        oc.do_register("phemlight-path", SumoOption::file_name_with(vec!["./PHEMlight/".to_string()]));
        oc.add_description("phemlight-path", "Input", "Determines where to load PHEMlight definitions from");

        oc.do_register("phemlight-year", SumoOption::integer(0));
        oc.add_description("phemlight-year", "Input", "Enable fleet age modeling with the given reference year in PHEMlight5");

        oc.do_register("phemlight-temperature", SumoOption::float(utils::common::std_defs::INVALID_DOUBLE));
        oc.add_description("phemlight-temperature", "Input", "Set ambient temperature to correct NOx emissions in PHEMlight5");

        if isDUA || isMA {
            oc.do_register_abbr("weight-files", 'w', SumoOption::file_name());
            oc.add_synonyme("weight-files", "weights", false);
            oc.add_description("weight-files", "Input", "Read network weights from FILE(s)");

            oc.do_register("lane-weight-files", SumoOption::file_name());
            oc.add_description("lane-weight-files", "Input", "Read lane-based network weights from FILE(s)");

            oc.do_register_abbr("weight-attribute", 'x', SumoOption::string_with("traveltime"));
            oc.add_synonyme("weight-attribute", "measure", true);
            oc.add_description("weight-attribute", "Input", "Name of the xml attribute which gives the edge weight");
        }

        // shortcut: SystemFrame::addReportOptions not ported yet (report options such as log/verbose are missing), add when SystemFrame lands
        // register the time settings
        oc.do_register_abbr("begin", 'b', SumoOption::string_typed("0", "TIME"));
        oc.add_description("begin", "Time", "Defines the begin time; Previous trips will be discarded");

        oc.do_register_abbr("end", 'e', SumoOption::string_typed("-1", "TIME"));
        oc.add_description("end", "Time", "Defines the end time; Later trips will be discarded; Defaults to the maximum time that SUMO can represent");

        // register the processing options
        oc.do_register("ignore-errors", SumoOption::bool(false));
        oc.add_synonyme("ignore-errors", "continue-on-unbuild", true);
        oc.add_synonyme("ignore-errors", "dismiss-loading-errors", true);
        oc.add_description("ignore-errors", "Report", "Continue if a route could not be build");

        oc.do_register("max-alternatives", SumoOption::integer(5));
        oc.add_description("max-alternatives", "Processing", "Prune the number of alternatives to INT");

        oc.do_register("with-taz", SumoOption::bool(false));
        oc.add_description("with-taz", "Processing", "Use origin and destination zones (districts) for in- and output");

        oc.do_register("junction-taz", SumoOption::bool(false));
        oc.add_description("junction-taz", "Input", "Initialize a TAZ for every junction to use attributes toJunction and fromJunction");

        if !isMA {
            oc.do_register("unsorted-input", SumoOption::bool(false));
            oc.add_synonyme("unsorted-input", "unsorted", false);
            oc.add_description("unsorted-input", "Processing", "Assume input is unsorted");

            oc.do_register_abbr("route-steps", 's', SumoOption::string_typed("200", "TIME"));
            oc.add_description("route-steps", "Processing", "Load routes for the next number of seconds ahead");

            oc.do_register("no-internal-links", SumoOption::bool(false));
            oc.add_description("no-internal-links", "Processing", "Disable (junction) internal links");

            oc.do_register("randomize-flows", SumoOption::bool(false));
            oc.add_description("randomize-flows", "Processing", "generate random departure times for flow input");

            oc.do_register("remove-loops", SumoOption::bool(false));
            oc.add_description("remove-loops", "Processing", "Remove loops within the route; Remove turnarounds at start and end of the route");

            oc.do_register("repair", SumoOption::bool(false));
            oc.add_description("repair", "Processing", "Tries to correct a false route");

            oc.do_register("repair.from", SumoOption::bool(false));
            oc.add_description("repair.from", "Processing", "Tries to correct an invalid starting edge by using the first usable edge instead");

            oc.do_register("repair.to", SumoOption::bool(false));
            oc.add_description("repair.to", "Processing", "Tries to correct an invalid destination edge by using the last usable edge instead");

            oc.do_register("repair.max-detour-factor", SumoOption::float(10.));
            oc.add_description("repair.max-detour-factor", "Processing", "Backtrack on route if the detour is longer than the gap by FACTOR");

            oc.do_register("mapmatch.distance", SumoOption::float(100.));
            oc.add_description("mapmatch.distance", "Processing", "Maximum distance when mapping input coordinates (fromXY etc.) to the road network");

            oc.do_register("mapmatch.junctions", SumoOption::bool(false));
            oc.add_description("mapmatch.junctions", "Processing", "Match positions to junctions instead of edges");

            oc.do_register("mapmatch.taz", SumoOption::bool(false));
            oc.add_description("mapmatch.taz", "Processing", "Match positions to taz instead of edges");

            oc.do_register("bulk-routing", SumoOption::bool(false));
            oc.add_description("bulk-routing", "Processing", "Aggregate routing queries with the same origin");
        }

        oc.do_register("routing-threads", SumoOption::integer(0));
        oc.add_description("routing-threads", "Processing", "The number of parallel execution threads used for routing");

        if isDUA || isMA {
            oc.do_register("routing-algorithm", SumoOption::string_with("dijkstra"));
            oc.add_description("routing-algorithm", "Processing", "Select among routing algorithms ['dijkstra', 'astar', 'CH', 'CHWrapper', 'CCH']");
        }

        oc.do_register("restriction-params", SumoOption::string_vector());
        oc.add_description("restriction-params", "Processing", "Comma separated list of param keys to compare for additional restrictions");

        if isDUA || isMA {
            oc.do_register("weights.interpolate", SumoOption::bool(false));
            oc.add_synonyme("weights.interpolate", "interpolate", true);
            oc.add_description("weights.interpolate", "Processing", "Interpolate edge weights at interval boundaries");

            oc.do_register("weights.expand", SumoOption::bool(false));
            oc.add_synonyme("weights.expand", "expand-weights", true);
            oc.add_description("weights.expand", "Processing", "Expand the end of the last loaded weight interval to infinity");
        }

        oc.do_register("weights.minor-penalty", SumoOption::float(1.5));
        oc.add_description("weights.minor-penalty", "Processing", "Apply the given time penalty when computing routing costs for minor-link internal lanes");

        oc.do_register("weights.tls-penalty", SumoOption::float(0.));
        oc.add_description("weights.tls-penalty", "Processing", "Apply the given time penalty when computing routing costs across a traffic light");

        oc.do_register("weights.turnaround-penalty", SumoOption::float(5.0));
        oc.add_description("weights.turnaround-penalty", "Processing", "Apply the given time penalty when computing routing costs for turnaround internal lanes");

        oc.do_register("weights.reversal-penalty", SumoOption::float(60.));
        oc.add_description("weights.reversal-penalty", "Processing", "Apply the given time penalty when computing routing costs for train reversal. Negative values disable reversal");

        if !isMA {
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
        }
        oc.do_register("defaults-override", SumoOption::bool(false));
        oc.add_description("defaults-override", "Defaults", "Defaults will override given values");

        // register report options
        oc.do_register("stats-period", SumoOption::integer(-1));
        oc.add_description("stats-period", "Report", "Defines how often statistics shall be printed");

        oc.do_register("no-step-log", SumoOption::bool(false));
        oc.add_description("no-step-log", "Report", "Disable console output of route parsing step");
    }

    /// C++: `bool ROFrame::checkOptions(OptionsCont &)` [static]
    /// Checks whether options are valid
    // bool
    // ROFrame::checkOptions(OptionsCont& oc) {
    //     // check whether the output is valid and can be build
    //     if (!oc.isSet("output-file")) {
    //         WRITE_ERROR(TL("No output specified."));
    //         return false;
    //     }
    //     //
    //     if (oc.getInt("max-alternatives") < 1) {
    //         WRITE_ERROR(TL("A positive number of alternatives must be enabled."));
    //         return false;
    //     }
    // #ifndef HAVE_FOX
    //     if (oc.getInt("routing-threads") > 1) {
    //         WRITE_ERROR(TL("Parallel routing is only possible when compiled with Fox."));
    //         return false;
    //     }
    // #endif
    //     if (oc.exists("mapmatch.junctions") && oc.getBool("mapmatch.junctions") && oc.isDefault("junction-taz")) {
    //         oc.setDefault("junction-taz", "true");
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn check_options(oc: &mut OptionsCont) -> bool {
        // check whether the output is valid and can be build
        if !oc.is_set("output-file", true).unwrap() {
            utils::common::msg_handler::write_error("No output specified.");
            return false;
        }
        //
        if oc.get_int("max-alternatives").unwrap() < 1 {
            utils::common::msg_handler::write_error("A positive number of alternatives must be enabled.");
            return false;
        }
        // the port is built without HAVE_FOX (headless), so the parallel routing check applies
        if oc.get_int("routing-threads").unwrap() > 1 {
            utils::common::msg_handler::write_error("Parallel routing is only possible when compiled with Fox.");
            return false;
        }
        if oc.exists("mapmatch.junctions") && oc.get_bool("mapmatch.junctions").unwrap() && oc.is_default("junction-taz") {
            oc.set_default("junction-taz", "true").unwrap();
        }
        true
    }

}

