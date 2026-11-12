//! Port of activitygen/AGFrame.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Sets and checks options for ActivityGen
/// C++ class `AGFrame` (activitygen/AGFrame.h)
pub struct AGFrame {
}

impl AGFrame {
    /// C++: `void AGFrame::fillOptions()` [static]
    /// Inserts options used by ActivityGen into the OptionsCont singleton
    // void AGFrame::fillOptions() {
    //     OptionsCont& oc = OptionsCont::getOptions();
    //     // Options handling
    //     oc.addCallExample("--net-file <INPUT>.net.xml --stat-file <INPUT>.stat.xml --output <OUTPUT>.rou.xml --rand",
    //                       TL("generate a trips file from a stats file on a given net using arbitrary random seed"));
    //     oc.addCallExample("--net-file <INPUT>.net.xml --stat-file <INPUT>.stat.xml --output <OUTPUT>.rou.xml --duration-d <NBR_OF_DAYS>",
    //                       TL("generate a trips file from a stats file on a given net for numerous days (with fixed random seed)"));
    //
    //     // Add categories and insert the standard options
    //     SystemFrame::addConfigurationOptions(oc);
    //     oc.addOptionSubTopic("Input");
    //     oc.addOptionSubTopic("Output");
    //     oc.addOptionSubTopic("Processing");
    //     oc.addOptionSubTopic("Time");
    //
    //     // Insert options
    //     oc.doRegister("net-file", 'n', new Option_FileName());
    //     oc.addSynonyme("net-file", "net");
    //     oc.addDescription("net-file", "Input", TL("Use FILE as SUMO-network to create trips for"));
    //
    //     oc.doRegister("stat-file", 's', new Option_FileName());
    //     oc.addDescription("stat-file", "Input", TL("Loads the SUMO-statistics FILE"));
    //
    //     // need to do this here to be able to check for network and route input options
    //     SystemFrame::addReportOptions(oc);
    //     RandHelper::insertRandOptions(oc);
    //
    //     oc.doRegister("output-file", 'o', new Option_FileName());
    //     oc.addSynonyme("output-file", "output", true);
    //     oc.addDescription("output-file", "Output", TL("Write generated trips to FILE"));
    //
    //     oc.doRegister("debug", new Option_Bool(false));
    //     oc.addDescription("debug", "Report",
    //                       "Detailed messages about every single step");
    //
    //     // TODO: What time options are consistent with other parts of SUMO and
    //     // useful for the user?
    //     oc.doRegister("begin", 'b', new Option_Integer(0));
    //     oc.addDescription("begin", "Time", TL("Sets the time of beginning of the simulation during the first day (in seconds)"));
    //
    //     oc.doRegister("end", 'e', new Option_Integer(0));
    //     oc.addDescription("end", "Time", TL("Sets the time of ending of the simulation during the last day (in seconds)"));
    //
    //     oc.doRegister("duration-d", new Option_Integer(1));
    //     oc.addDescription("duration-d", "Time", TL("Sets the duration of the simulation in days"));
    // }
    #[allow(unused_variables)]
    pub fn fill_options() {
        todo!("port AGFrame::fillOptions")
    }

    /// C++: `bool AGFrame::checkOptions()` [static]
    /// Checks set options from the OptionsCont singleton for being valid for usage within ActivityGen
    // bool AGFrame::checkOptions() {
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn check_options() -> bool {
        true
    }

}

