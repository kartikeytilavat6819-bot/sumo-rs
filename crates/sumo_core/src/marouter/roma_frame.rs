//! Port of marouter/ROMAFrame.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::msg_handler::{write_error, write_warning};

/// Sets and checks options for dua-routing
/// C++ class `ROMAFrame` (marouter/ROMAFrame.h)
pub struct ROMAFrame {
}

impl ROMAFrame {
    /// C++: `void ROMAFrame::fillOptions()` [static]
    /// Inserts options used by duarouter into the OptionsCont-singleton
    // void
    // ROMAFrame::fillOptions() {
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
    //     ROFrame::fillOptions(oc, false, true);
    //     addImportOptions();
    //     addAssignmentOptions();
    //     // add rand options
    //     RandHelper::insertRandOptions(oc);
    // }
    #[allow(unused_variables)]
    pub fn fill_options() {
        utils::options::options_cont::with_options(|oc| {
            oc.add_call_example("-c <CONFIGURATION>", "run routing with options from file");

            // insert options sub-topics
            // shortcut: SystemFrame::addConfigurationOptions is not ported yet, its configuration options are missing, upgrade when system_frame is ported
            oc.add_option_sub_topic("Input");
            oc.add_option_sub_topic("Output");
            oc.add_option_sub_topic("Processing");
            oc.add_option_sub_topic("Defaults");
            oc.add_option_sub_topic("Time");

            // insert options
            ROFrame::fill_options(oc, false, true);
        });
        Self::add_import_options();
        Self::add_assignment_options();
        // add rand options
        utils::options::options_cont::with_options(|oc| utils::common::rand_helper::insert_rand_options(oc));
    }

    /// C++: `bool ROMAFrame::checkOptions()` [static]
    /// Checks set options from the OptionsCont-singleton for being valid for usage within duarouter
    // bool
    // ROMAFrame::checkOptions() {
    //     OptionsCont& oc = OptionsCont::getOptions();
    //     if (oc.isSet("assignment-method") && oc.getString("assignment-method") != "incremental" && oc.getString("assignment-method") != "UE" && oc.getString("assignment-method") != "SUE") {
    //         WRITE_ERRORF(TL("Invalid assignment method '%'."), oc.getString("assignment-method"));
    //         return false;
    //     }
    //     if (oc.getString("route-choice-method") != "gawron" && oc.getString("route-choice-method") != "logit" && oc.getString("route-choice-method") != "lohse") {
    //         WRITE_ERRORF(TL("Invalid route choice method '%'."), oc.getString("route-choice-method"));
    //         return false;
    //     }
    //     if (oc.getInt("paths") > 1 && (oc.getString("routing-algorithm") == "CH" || oc.getString("routing-algorithm") == "CHWrapper" || oc.getString("routing-algorithm") == "CCH")) {
    //         WRITE_WARNING(TL("Contraction hierarchies do not work with k shortest path search (please use a different routing algorithm)!"));
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn check_options() -> bool {
        use utils::options::options_cont::{get_int, get_string, is_set};
        if is_set("assignment-method") && get_string("assignment-method") != "incremental" && get_string("assignment-method") != "UE" && get_string("assignment-method") != "SUE" {
            write_error(&format!("Invalid assignment method '{}'.", get_string("assignment-method")));
            return false;
        }
        if get_string("route-choice-method") != "gawron" && get_string("route-choice-method") != "logit" && get_string("route-choice-method") != "lohse" {
            write_error(&format!("Invalid route choice method '{}'.", get_string("route-choice-method")));
            return false;
        }
        if get_int("paths") > 1 && (get_string("routing-algorithm") == "CH" || get_string("routing-algorithm") == "CHWrapper" || get_string("routing-algorithm") == "CCH") {
            write_warning("Contraction hierarchies do not work with k shortest path search (please use a different routing algorithm)!");
        }
        true
    }

    /// C++: `void ROMAFrame::addImportOptions()` [static, protected]
    /// Inserts import options used by duarouter into the OptionsCont-singleton
    // void
    // ROMAFrame::addImportOptions() {
    //     OptionsCont& oc = OptionsCont::getOptions();
    //     // register import options
    //     oc.doRegister("ignore-vehicle-type", new Option_Bool(false));
    //     oc.addSynonyme("ignore-vehicle-type", "no-vtype", true);
    //     oc.addDescription("ignore-vehicle-type", "Output", TL("Does not save vtype information"));
    //
    //     oc.doRegister("netload-output", new Option_FileName());
    //     oc.addDescription("netload-output", "Output", TL("Writes edge loads and final costs into FILE"));
    //
    //     oc.doRegister("all-pairs-output", new Option_FileName());
    //     oc.addDescription("all-pairs-output", "Output", TL("Writes complete distance matrix into FILE"));
    //
    //     oc.doRegister("od-matrix-files", 'm', new Option_FileName());
    //     oc.addSynonyme("od-matrix-files", "od-files");
    //     oc.addDescription("od-matrix-files", "Input", TL("Loads O/D-files from FILE(s)"));
    //
    //     oc.doRegister("od-amitran-files", new Option_FileName());
    //     oc.addSynonyme("od-amitran-files", "amitran-files");
    //     oc.addSynonyme("od-amitran-files", "amitran");
    //     oc.addDescription("od-amitran-files", "Input", TL("Loads O/D-matrix in Amitran format from FILE(s)"));
    //
    //     oc.doRegister("tazrelation-files", 'z', new Option_FileName());
    //     oc.addDescription("tazrelation-files", "Input", TL("Loads O/D-matrix in tazRelation format from FILE(s)"));
    //
    //     oc.doRegister("tazrelation-attribute", new Option_String("count"));
    //     oc.addSynonyme("tazrelation-attribute", "attribute");
    //     oc.addDescription("tazrelation-attribute", "Input", TL("Define data attribute for loading counts (default 'count')"));
    //
    //     oc.doRegister("weight-adaption", new Option_Float(0.));
    //     oc.addDescription("weight-adaption", "Input", TL("The travel time influence of prior intervals"));
    //
    //     oc.doRegister("taz-param", new Option_StringVector());
    //     oc.addDescription("taz-param", "Input", TL("Parameter key(s) defining source (and sink) taz"));
    //
    //     oc.doRegister("ignore-taz", new Option_Bool(false));
    //     oc.addDescription("ignore-taz", "Input", TL("Ignore attributes 'fromTaz' and 'toTaz'"));
    //
    //     // register the processing options
    //     oc.doRegister("aggregation-interval", new Option_String("3600", "TIME"));
    //     oc.addDescription("aggregation-interval", "Processing", TL("Defines the time interval when aggregating single vehicle input; Defaults to one hour"));
    //
    //     oc.doRegister("capacities.default", new Option_Bool(false));
    //     oc.addDescription("capacities.default", "Processing", TL("Ignore edge priorities when calculating capacities and restraints"));
    //
    //     oc.doRegister("weights.priority-factor", new Option_Float(0));
    //     oc.addDescription("weights.priority-factor", "Processing", TL("Consider edge priorities in addition to travel times, weighted by factor"));
    //
    //     oc.doRegister("bulk-routing.vtypes", new Option_Bool(false));
    //     oc.addDescription("bulk-routing.vtypes", "Processing", TL("Aggregate routing queries with the same origin for different vehicle types"));
    //
    //     oc.doRegister("weight-period", new Option_String("3600", "TIME"));
    //     oc.addDescription("weight-period", "Processing", TL("Aggregation period for the given weight files; triggers rebuilding of Contraction Hierarchy"));
    //
    //     // register defaults options
    //     oc.doRegister("flow-output.departlane", new Option_String("free"));
    //     oc.addSynonyme("flow-output.departlane", "departlane");
    //     oc.addDescription("flow-output.departlane", "Defaults", TL("Assigns a default depart lane"));
    //
    //     oc.doRegister("flow-output.departpos", new Option_String());
    //     oc.addSynonyme("flow-output.departpos", "departpos");
    //     oc.addDescription("flow-output.departpos", "Defaults", TL("Assigns a default depart position"));
    //
    //     oc.doRegister("flow-output.departspeed", new Option_String("max"));
    //     oc.addSynonyme("flow-output.departspeed", "departspeed");
    //     oc.addDescription("flow-output.departspeed", "Defaults", TL("Assigns a default depart speed"));
    //
    //     oc.doRegister("flow-output.arrivallane", new Option_String());
    //     oc.addSynonyme("flow-output.arrivallane", "arrivallane");
    //     oc.addDescription("flow-output.arrivallane", "Defaults", TL("Assigns a default arrival lane"));
    //
    //     oc.doRegister("flow-output.arrivalpos", new Option_String());
    //     oc.addSynonyme("flow-output.arrivalpos", "arrivalpos");
    //     oc.addDescription("flow-output.arrivalpos", "Defaults", TL("Assigns a default arrival position"));
    //
    //     oc.doRegister("flow-output.arrivalspeed", new Option_String());
    //     oc.addSynonyme("flow-output.arrivalspeed", "arrivalspeed");
    //     oc.addDescription("flow-output.arrivalspeed", "Defaults", TL("Assigns a default arrival speed"));
    //
    // }
    #[allow(unused_variables)]
    pub fn add_import_options() {
        utils::options::options_cont::with_options(|oc| {
            // register import options
            oc.do_register("ignore-vehicle-type", SumoOption::bool(false));
            oc.add_synonyme("ignore-vehicle-type", "no-vtype", true);
            oc.add_description("ignore-vehicle-type", "Output", "Does not save vtype information");

            oc.do_register("netload-output", SumoOption::file_name());
            oc.add_description("netload-output", "Output", "Writes edge loads and final costs into FILE");

            oc.do_register("all-pairs-output", SumoOption::file_name());
            oc.add_description("all-pairs-output", "Output", "Writes complete distance matrix into FILE");

            oc.do_register_abbr("od-matrix-files", 'm', SumoOption::file_name());
            oc.add_synonyme("od-matrix-files", "od-files", false);
            oc.add_description("od-matrix-files", "Input", "Loads O/D-files from FILE(s)");

            oc.do_register("od-amitran-files", SumoOption::file_name());
            oc.add_synonyme("od-amitran-files", "amitran-files", false);
            oc.add_synonyme("od-amitran-files", "amitran", false);
            oc.add_description("od-amitran-files", "Input", "Loads O/D-matrix in Amitran format from FILE(s)");

            oc.do_register_abbr("tazrelation-files", 'z', SumoOption::file_name());
            oc.add_description("tazrelation-files", "Input", "Loads O/D-matrix in tazRelation format from FILE(s)");

            oc.do_register("tazrelation-attribute", SumoOption::string_with("count"));
            oc.add_synonyme("tazrelation-attribute", "attribute", false);
            oc.add_description("tazrelation-attribute", "Input", "Define data attribute for loading counts (default 'count')");

            oc.do_register("weight-adaption", SumoOption::float(0.));
            oc.add_description("weight-adaption", "Input", "The travel time influence of prior intervals");

            oc.do_register("taz-param", SumoOption::string_vector());
            oc.add_description("taz-param", "Input", "Parameter key(s) defining source (and sink) taz");

            oc.do_register("ignore-taz", SumoOption::bool(false));
            oc.add_description("ignore-taz", "Input", "Ignore attributes 'fromTaz' and 'toTaz'");

            // register the processing options
            oc.do_register("aggregation-interval", SumoOption::string_typed("3600", "TIME"));
            oc.add_description("aggregation-interval", "Processing", "Defines the time interval when aggregating single vehicle input; Defaults to one hour");

            oc.do_register("capacities.default", SumoOption::bool(false));
            oc.add_description("capacities.default", "Processing", "Ignore edge priorities when calculating capacities and restraints");

            oc.do_register("weights.priority-factor", SumoOption::float(0.));
            oc.add_description("weights.priority-factor", "Processing", "Consider edge priorities in addition to travel times, weighted by factor");

            oc.do_register("bulk-routing.vtypes", SumoOption::bool(false));
            oc.add_description("bulk-routing.vtypes", "Processing", "Aggregate routing queries with the same origin for different vehicle types");

            oc.do_register("weight-period", SumoOption::string_typed("3600", "TIME"));
            oc.add_description("weight-period", "Processing", "Aggregation period for the given weight files; triggers rebuilding of Contraction Hierarchy");

            // register defaults options
            oc.do_register("flow-output.departlane", SumoOption::string_with("free"));
            oc.add_synonyme("flow-output.departlane", "departlane", false);
            oc.add_description("flow-output.departlane", "Defaults", "Assigns a default depart lane");

            oc.do_register("flow-output.departpos", SumoOption::string());
            oc.add_synonyme("flow-output.departpos", "departpos", false);
            oc.add_description("flow-output.departpos", "Defaults", "Assigns a default depart position");

            oc.do_register("flow-output.departspeed", SumoOption::string_with("max"));
            oc.add_synonyme("flow-output.departspeed", "departspeed", false);
            oc.add_description("flow-output.departspeed", "Defaults", "Assigns a default depart speed");

            oc.do_register("flow-output.arrivallane", SumoOption::string());
            oc.add_synonyme("flow-output.arrivallane", "arrivallane", false);
            oc.add_description("flow-output.arrivallane", "Defaults", "Assigns a default arrival lane");

            oc.do_register("flow-output.arrivalpos", SumoOption::string());
            oc.add_synonyme("flow-output.arrivalpos", "arrivalpos", false);
            oc.add_description("flow-output.arrivalpos", "Defaults", "Assigns a default arrival position");

            oc.do_register("flow-output.arrivalspeed", SumoOption::string());
            oc.add_synonyme("flow-output.arrivalspeed", "arrivalspeed", false);
            oc.add_description("flow-output.arrivalspeed", "Defaults", "Assigns a default arrival speed");
        });
    }

    /// C++: `void ROMAFrame::addAssignmentOptions()` [static, protected]
    /// Inserts dua options used by duarouter into the OptionsCont-singleton
    // void
    // ROMAFrame::addAssignmentOptions() {
    //     OptionsCont& oc = OptionsCont::getOptions();
    //     // register the data processing options
    //     oc.doRegister("scale", 's', new Option_Float(1));
    //     oc.addDescription("scale", "Processing", TL("Scales the loaded flows by FLOAT"));
    //
    //     oc.doRegister("vtype", new Option_String(""));
    //     oc.addDescription("vtype", "Processing", TL("Defines the name of the vehicle type to use"));
    //
    //     oc.doRegister("prefix", new Option_String(""));
    //     oc.addDescription("prefix", "Processing", TL("Defines the prefix for vehicle flow names"));
    //
    //     oc.doRegister("timeline", new Option_StringVector());
    //     oc.addDescription("timeline", "Processing", TL("Uses STR[] as a timeline definition"));
    //
    //     oc.doRegister("timeline.day-in-hours", new Option_Bool(false));
    //     oc.addDescription("timeline.day-in-hours", "Processing", TL("Uses STR as a 24h-timeline definition"));
    //
    //     oc.doRegister("additive-traffic", new Option_Bool(false));
    //     oc.addDescription("additive-traffic", "Processing", TL("Keep traffic flows of all time slots in the net"));
    //
    //     // register macroscopic SUE-settings
    //     oc.doRegister("assignment-method", new Option_String("incremental"));
    //     oc.addDescription("assignment-method", "Processing", TL("Choose a assignment method: incremental, UE or SUE"));
    //
    //     oc.doRegister("tolerance", new Option_Float(double(0.001)));
    //     oc.addDescription("tolerance", "Processing", TL("Use FLOAT as tolerance when checking for SUE stability"));
    //
    //     oc.doRegister("left-turn-penalty", new Option_Float(0.));
    //     oc.addDescription("left-turn-penalty", "Processing", TL("Use left-turn penalty FLOAT to calculate link travel time when searching routes"));
    //
    //     oc.doRegister("paths",  new Option_Integer(1));
    //     oc.addDescription("paths", "Processing", TL("Use INTEGER as the number of paths needed to be searched for each OD pair at each iteration"));
    //
    //     oc.doRegister("paths.penalty", new Option_Float(double(1)));
    //     oc.addDescription("paths.penalty", "Processing", TL("Penalize existing routes with FLOAT to find secondary routes"));
    //
    //     oc.doRegister("upperbound", new Option_Float(double(0.5)));
    //     oc.addSynonyme("upperbound", "upper", true);
    //     oc.addDescription("upperbound", "Processing", TL("Use FLOAT as the upper bound to determine auxiliary link cost"));
    //
    //     oc.doRegister("lowerbound", new Option_Float(double(0.15)));
    //     oc.addSynonyme("lowerbound", "lower", true);
    //     oc.addDescription("lowerbound", "Processing", TL("Use FLOAT as the lower bound to determine auxiliary link cost"));
    //
    //     oc.doRegister("max-iterations", 'i', new Option_Integer(20));
    //     oc.addDescription("max-iterations", "Processing", TL("maximal number of iterations for new route searching in incremental and stochastic user assignment"));
    //
    //     oc.doRegister("max-inner-iterations", new Option_Integer(1000));
    //     oc.addDescription("max-inner-iterations", "Processing", TL("maximal number of inner iterations for user equilibrium calculation in the stochastic user assignment"));
    //
    //     // register route choice settings
    //     oc.doRegister("route-choice-method", new Option_String("logit"));
    //     oc.addDescription("route-choice-method", "Processing", TL("Choose a route choice method: gawron, logit, or lohse"));
    //
    //     oc.doRegister("gawron.beta", new Option_Float(double(0.3)));
    //     oc.addSynonyme("gawron.beta", "gBeta", true);
    //     oc.addDescription("gawron.beta", "Processing", TL("Use FLOAT as Gawron's beta"));
    //
    //     oc.doRegister("gawron.a", new Option_Float(double(0.05)));
    //     oc.addSynonyme("gawron.a", "gA", true);
    //     oc.addDescription("gawron.a", "Processing", TL("Use FLOAT as Gawron's a"));
    //
    //     oc.doRegister("exit-times", new Option_Bool(false));
    //     oc.addDescription("exit-times", "Output", TL("Write exit times (weights) for each edge"));
    //
    //     oc.doRegister("keep-all-routes", new Option_Bool(false));
    //     oc.addDescription("keep-all-routes", "Processing", TL("Save routes with near zero probability"));
    //
    //     oc.doRegister("skip-new-routes", new Option_Bool(false));
    //     oc.addDescription("skip-new-routes", "Processing", TL("Only reuse routes from input, do not calculate new ones"));
    //
    //     oc.doRegister("logit.beta", new Option_Float(double(0.15)));  // check: remove the default?
    //     oc.addSynonyme("logit.beta", "lBeta", true);
    //     oc.addDescription("logit.beta", "Processing", TL("Use FLOAT as (c-)logit's beta for the commonality factor"));
    //
    //     oc.doRegister("logit.gamma", new Option_Float(double(1)));
    //     oc.addSynonyme("logit.gamma", "lGamma", true);
    //     oc.addDescription("logit.gamma", "Processing", TL("Use FLOAT as (c-)logit's gamma for the commonality factor"));
    //
    //     oc.doRegister("logit.theta", new Option_Float(double(0.01)));
    //     oc.addSynonyme("logit.theta", "lTheta", true);
    //     oc.addDescription("logit.theta", "Processing", TL("Use FLOAT as (c-)logit's theta"));
    // }
    #[allow(unused_variables)]
    pub fn add_assignment_options() {
        utils::options::options_cont::with_options(|oc| {
            // register the data processing options
            oc.do_register_abbr("scale", 's', SumoOption::float(1.));
            oc.add_description("scale", "Processing", "Scales the loaded flows by FLOAT");

            oc.do_register("vtype", SumoOption::string_with(""));
            oc.add_description("vtype", "Processing", "Defines the name of the vehicle type to use");

            oc.do_register("prefix", SumoOption::string_with(""));
            oc.add_description("prefix", "Processing", "Defines the prefix for vehicle flow names");

            oc.do_register("timeline", SumoOption::string_vector());
            oc.add_description("timeline", "Processing", "Uses STR[] as a timeline definition");

            oc.do_register("timeline.day-in-hours", SumoOption::bool(false));
            oc.add_description("timeline.day-in-hours", "Processing", "Uses STR as a 24h-timeline definition");

            oc.do_register("additive-traffic", SumoOption::bool(false));
            oc.add_description("additive-traffic", "Processing", "Keep traffic flows of all time slots in the net");

            // register macroscopic SUE-settings
            oc.do_register("assignment-method", SumoOption::string_with("incremental"));
            oc.add_description("assignment-method", "Processing", "Choose a assignment method: incremental, UE or SUE");

            oc.do_register("tolerance", SumoOption::float(0.001));
            oc.add_description("tolerance", "Processing", "Use FLOAT as tolerance when checking for SUE stability");

            oc.do_register("left-turn-penalty", SumoOption::float(0.));
            oc.add_description("left-turn-penalty", "Processing", "Use left-turn penalty FLOAT to calculate link travel time when searching routes");

            oc.do_register("paths", SumoOption::integer(1));
            oc.add_description("paths", "Processing", "Use INTEGER as the number of paths needed to be searched for each OD pair at each iteration");

            oc.do_register("paths.penalty", SumoOption::float(1.));
            oc.add_description("paths.penalty", "Processing", "Penalize existing routes with FLOAT to find secondary routes");

            oc.do_register("upperbound", SumoOption::float(0.5));
            oc.add_synonyme("upperbound", "upper", true);
            oc.add_description("upperbound", "Processing", "Use FLOAT as the upper bound to determine auxiliary link cost");

            oc.do_register("lowerbound", SumoOption::float(0.15));
            oc.add_synonyme("lowerbound", "lower", true);
            oc.add_description("lowerbound", "Processing", "Use FLOAT as the lower bound to determine auxiliary link cost");

            oc.do_register_abbr("max-iterations", 'i', SumoOption::integer(20));
            oc.add_description("max-iterations", "Processing", "maximal number of iterations for new route searching in incremental and stochastic user assignment");

            oc.do_register("max-inner-iterations", SumoOption::integer(1000));
            oc.add_description("max-inner-iterations", "Processing", "maximal number of inner iterations for user equilibrium calculation in the stochastic user assignment");

            // register route choice settings
            oc.do_register("route-choice-method", SumoOption::string_with("logit"));
            oc.add_description("route-choice-method", "Processing", "Choose a route choice method: gawron, logit, or lohse");

            oc.do_register("gawron.beta", SumoOption::float(0.3));
            oc.add_synonyme("gawron.beta", "gBeta", true);
            oc.add_description("gawron.beta", "Processing", "Use FLOAT as Gawron's beta");

            oc.do_register("gawron.a", SumoOption::float(0.05));
            oc.add_synonyme("gawron.a", "gA", true);
            oc.add_description("gawron.a", "Processing", "Use FLOAT as Gawron's a");

            oc.do_register("exit-times", SumoOption::bool(false));
            oc.add_description("exit-times", "Output", "Write exit times (weights) for each edge");

            oc.do_register("keep-all-routes", SumoOption::bool(false));
            oc.add_description("keep-all-routes", "Processing", "Save routes with near zero probability");

            oc.do_register("skip-new-routes", SumoOption::bool(false));
            oc.add_description("skip-new-routes", "Processing", "Only reuse routes from input, do not calculate new ones");

            oc.do_register("logit.beta", SumoOption::float(0.15)); // check: remove the default?
            oc.add_synonyme("logit.beta", "lBeta", true);
            oc.add_description("logit.beta", "Processing", "Use FLOAT as (c-)logit's beta for the commonality factor");

            oc.do_register("logit.gamma", SumoOption::float(1.));
            oc.add_synonyme("logit.gamma", "lGamma", true);
            oc.add_description("logit.gamma", "Processing", "Use FLOAT as (c-)logit's gamma for the commonality factor");

            oc.do_register("logit.theta", SumoOption::float(0.01));
            oc.add_synonyme("logit.theta", "lTheta", true);
            oc.add_description("logit.theta", "Processing", "Use FLOAT as (c-)logit's theta");
        });
    }

}

