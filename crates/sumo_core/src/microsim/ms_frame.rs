//! Port of microsim/MSFrame.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Sets and checks options for microsim; inits global outputs and settings
/// C++ class `MSFrame` (microsim/MSFrame.h)
pub struct MSFrame {
}

// ----- helpers for MSFrame::checkOptions (the OptionsCont is only reachable inside with_options) -----

fn or_panic<T>(r: Result<T, utils::common::util_exceptions::ProcessError>) -> T {
    r.unwrap_or_else(|e| panic!("{e}"))
}

fn opt_bool(oc: &utils::options::options_cont::OptionsCont, name: &str) -> bool {
    or_panic(oc.get_bool(name))
}

fn opt_float(oc: &utils::options::options_cont::OptionsCont, name: &str) -> f64 {
    or_panic(oc.get_float(name))
}

fn opt_int(oc: &utils::options::options_cont::OptionsCont, name: &str) -> i32 {
    or_panic(oc.get_int(name))
}

fn opt_str(oc: &utils::options::options_cont::OptionsCont, name: &str) -> String {
    or_panic(oc.get_string(name))
}

fn opt_strs(oc: &utils::options::options_cont::OptionsCont, name: &str) -> Vec<String> {
    or_panic(oc.get_string_vector(name)).clone()
}

fn opt_is_set(oc: &utils::options::options_cont::OptionsCont, name: &str) -> bool {
    or_panic(oc.is_set(name, true))
}

/// C++ `gTaxiClasses` is an `int`, so each `|=` keeps only the low 32 bits of the 64-bit SVCPermissions
fn g_taxi_classes_mask(names: &[String]) -> i32 {
    let mut taxiClasses: i32 = 0;
    for vClassName in names {
        taxiClasses |= utils::common::sumo_vehicle_class::parse_vehicle_classes(vClassName) as i32;
    }
    taxiClasses
}

#[cfg(test)]
mod tests {
    use super::g_taxi_classes_mask;

    #[test]
    fn taxi_mask_is_int_wide() {
        let names = |v: &[&str]| v.iter().map(|s| s.to_string()).collect::<Vec<String>>();
        // SVC_TAXI = 1 << 8 is kept
        assert_eq!(g_taxi_classes_mask(&names(&["taxi"])), 1 << 8);
        // SVC_CUSTOM1 = 1 << 31 becomes INT_MIN in C++ int
        assert_eq!(g_taxi_classes_mask(&names(&["custom1"])), i32::MIN);
        // SVC_CUSTOM2 = 1 << 32 is dropped entirely
        assert_eq!(g_taxi_classes_mask(&names(&["custom2"])), 0);
        assert_eq!(g_taxi_classes_mask(&names(&["taxi", "custom2"])), 1 << 8);
    }
}

fn opt_is_default(oc: &utils::options::options_cont::OptionsCont, name: &str) -> bool {
    oc.is_default(name)
}

/// C++ string2time(...) that throws ProcessError
fn st(s: &str) -> utils::common::time::SUMOTime {
    or_panic(utils::common::time::string2time(s))
}

use utils::common::time::check_step_length_multiple;

impl MSFrame {
    /// C++: `void MSFrame::fillOptions()` [static]
    /// Inserts options used by the simulation into the OptionsCont-singleton
    // void
    // MSFrame::fillOptions() {
    //     OptionsCont& oc = OptionsCont::getOptions();
    //     oc.addCallExample("-b 0 -e 1000 -n net.xml -r routes.xml", TL("start a simulation from time 0 to 1000 with given net and routes"));
    //     oc.addCallExample("-c munich_config.cfg", TL("start with a configuration file"));
    //     oc.addCallExample("--help", TL("print help"));
    //
    //     // insert options sub-topics
    //     SystemFrame::addConfigurationOptions(oc); // fill this subtopic, too
    //     oc.addOptionSubTopic("Input");
    //     oc.addOptionSubTopic("Output");
    //     oc.addOptionSubTopic("Time");
    //     oc.addOptionSubTopic("Processing");
    //     oc.addOptionSubTopic("Routing");
    //     oc.addOptionSubTopic("Persons");
    //
    //     // register configuration options
    //     //  register input options
    //     oc.doRegister("net-file", 'n', new Option_FileName());
    //     oc.addSynonyme("net-file", "net");
    //     oc.addDescription("net-file", "Input", TL("Load road network description from FILE"));
    //     oc.addXMLDefault("net-file", "net");
    //
    //     oc.doRegister("route-files", 'r', new Option_FileName());
    //     oc.addSynonyme("route-files", "routes");
    //     oc.addDescription("route-files", "Input", TL("Load routes descriptions from FILE(s)"));
    //
    //     oc.doRegister("additional-files", 'a', new Option_FileName());
    //     oc.addSynonyme("additional-files", "additional");
    //     oc.addDescription("additional-files", "Input", TL("Load further descriptions from FILE(s)"));
    //
    //     oc.doRegister("weight-files", 'w', new Option_FileName());
    //     oc.addSynonyme("weight-files", "weights");
    //     oc.addDescription("weight-files", "Input", TL("Load edge/lane weights for online rerouting from FILE"));
    //     oc.doRegister("weight-attribute", 'x', new Option_String("traveltime"));
    //     oc.addSynonyme("weight-attribute", "measure", true);
    //     oc.addDescription("weight-attribute", "Input", TL("Name of the xml attribute which gives the edge weight"));
    //
    //     oc.doRegister("load-state", new Option_FileName());//!!! check, describe
    //     oc.addDescription("load-state", "Input", TL("Loads a network state from FILE"));
    //     oc.doRegister("load-state.offset", new Option_String("0", "TIME"));//!!! check, describe
    //     oc.addDescription("load-state.offset", "Input", TL("Shifts all times loaded from a saved state by the given offset"));
    //     oc.doRegister("load-state.remove-vehicles", new Option_StringVector(StringVector({})));
    //     oc.addDescription("load-state.remove-vehicles", "Input", TL("Removes vehicles with the given IDs from the loaded state"));
    //
    //     oc.doRegister("junction-taz", new Option_Bool(false));
    //     oc.addDescription("junction-taz", "Input", TL("Initialize a TAZ for every junction to use attributes toJunction and fromJunction"));
    //
    //     // need to do this here to be able to check for network and route input options
    //     SystemFrame::addReportOptions(oc);
    //
    //     //  register output options
    //     oc.doRegister("netstate-dump", new Option_FileName());
    //     oc.addSynonyme("netstate-dump", "ndump");
    //     oc.addSynonyme("netstate-dump", "netstate");
    //     oc.addSynonyme("netstate-dump", "netstate-output");
    //     oc.addDescription("netstate-dump", "Output", TL("Save complete network states into FILE"));
    //     oc.doRegister("netstate-dump.empty-edges", new Option_Bool(false));
    //     oc.addSynonyme("netstate-dump.empty-edges", "netstate.empty-edges");
    //     oc.addSynonyme("netstate-dump.empty-edges", "netstate-output.empty-edges");
    //     oc.addSynonyme("netstate-dump.empty-edges", "dump-empty-edges", true);
    //     oc.addDescription("netstate-dump.empty-edges", "Output", TL("Write also empty edges completely when dumping"));
    //     oc.doRegister("netstate-dump.precision", new Option_Integer(2));
    //     oc.addSynonyme("netstate-dump.precision", "netstate.precision");
    //     oc.addSynonyme("netstate-dump.precision", "netstate-output.precision");
    //     oc.addSynonyme("netstate-dump.precision", "dump-precision", true);
    //     oc.addDescription("netstate-dump.precision", "Output", TL("Write positions and speeds with the given precision (default 2)"));
    //
    //     oc.doRegister("emission-output", new Option_FileName());
    //     oc.addDescription("emission-output", "Output", TL("Save the emission values of each vehicle"));
    //     oc.doRegister("emission-output.precision", new Option_Integer(2));
    //     oc.addDescription("emission-output.precision", "Output", TL("Write emission values with the given precision (default 2)"));
    //     oc.doRegister("emission-output.geo", new Option_Bool(false));
    //     oc.addDescription("emission-output.geo", "Output", TL("Save the positions in emission output using geo-coordinates (lon/lat)"));
    //     oc.doRegister("emission-output.step-scaled", new Option_Bool(false));
    //     oc.addDescription("emission-output.step-scaled", "Output", TL("Write emission values scaled to the step length rather than as per-second values"));
    //     oc.doRegister("emission-output.attributes", new Option_StringVector());
    //     oc.addDescription("emission-output.attributes", "Output", TL("List attributes that should be included in the emission output"));
    //
    //     oc.doRegister("battery-output", new Option_FileName());
    //     oc.addDescription("battery-output", "Output", TL("Save the battery values of each vehicle"));
    //     oc.doRegister("battery-output.precision", new Option_Integer(2));
    //     oc.addDescription("battery-output.precision", "Output", TL("Write battery values with the given precision (default 2)"));
    //
    //     oc.doRegister("elechybrid-output", new Option_FileName());
    //     oc.addDescription("elechybrid-output", "Output", TL("Save the elecHybrid values of each vehicle"));
    //     oc.doRegister("elechybrid-output.precision", new Option_Integer(2));
    //     oc.addDescription("elechybrid-output.precision", "Output", TL("Write elecHybrid values with the given precision (default 2)"));
    //     oc.doRegister("elechybrid-output.aggregated", new Option_Bool(false));
    //     oc.addDescription("elechybrid-output.aggregated", "Output", TL("Write elecHybrid values into one aggregated file"));
    //
    //     oc.doRegister("chargingstations-output", new Option_FileName());
    //     oc.addDescription("chargingstations-output", "Output", TL("Write data of charging stations"));
    //     oc.doRegister("chargingstations-output.aggregated", new Option_Bool(false));
    //     oc.addDescription("chargingstations-output.aggregated", "Output", TL("Write aggregated charging event data instead of single time steps"));
    //     oc.doRegister("chargingstations-output.aggregated.write-unfinished", new Option_Bool(false));
    //     oc.addDescription("chargingstations-output.aggregated.write-unfinished", "Output", TL("Write aggregated charging event data for vehicles which have not arrived at simulation end"));
    //
    //     oc.doRegister("overheadwiresegments-output", new Option_FileName());
    //     oc.addDescription("overheadwiresegments-output", "Output", TL("Write data of overhead wire segments"));
    //
    //     oc.doRegister("substations-output", new Option_FileName());
    //     oc.addDescription("substations-output", "Output", TL("Write data of electrical substation stations"));
    //     oc.doRegister("substations-output.precision", new Option_Integer(2));
    //     oc.addDescription("substations-output.precision", "Output", TL("Write substation values with the given precision (default 2)"));
    //
    //     oc.doRegister("fcd-output", new Option_FileName());
    //     oc.addDescription("fcd-output", "Output", TL("Save the Floating Car Data"));
    //     oc.doRegister("fcd-output.geo", new Option_Bool(false));
    //     oc.addDescription("fcd-output.geo", "Output", TL("Save the Floating Car Data using geo-coordinates (lon/lat)"));
    //     oc.doRegister("fcd-output.utm", new Option_Bool(false));
    //     oc.addDescription("fcd-output.utm", "Output", TL("Save the Floating Car Data using utm/unshifted coordinates (x/y)"));
    //     oc.doRegister("fcd-output.signals", new Option_Bool(false));
    //     oc.addDescription("fcd-output.signals", "Output", TL("Add the vehicle signal state to the FCD output (brake lights etc.)"));
    //     oc.doRegister("fcd-output.distance", new Option_Bool(false));
    //     oc.addDescription("fcd-output.distance", "Output", TL("Add kilometrage to the FCD output (linear referencing)"));
    //     oc.doRegister("fcd-output.acceleration", new Option_Bool(false));
    //     oc.addDescription("fcd-output.acceleration", "Output", TL("Add acceleration to the FCD output"));
    //     oc.doRegister("fcd-output.speed-relative", new Option_Bool(false));
    //     oc.addDescription("fcd-output.speed-relative", "Output", TL("Add relative speed (vehicle speed / edge speed limit) to the FCD output"));
    //     oc.doRegister("fcd-output.max-leader-distance", new Option_Float(-1));
    //     oc.addDescription("fcd-output.max-leader-distance", "Output", TL("Add leader vehicle information to the FCD output (within the given distance)"));
    //     oc.doRegister("fcd-output.params", new Option_StringVector());
    //     oc.addDescription("fcd-output.params", "Output", TL("Add generic parameter values to the FCD output"));
    //     oc.doRegister("fcd-output.filter-edges.input-file", new Option_FileName());
    //     oc.addDescription("fcd-output.filter-edges.input-file", "Output", TL("Restrict fcd output to the edge selection from the given input file"));
    //     oc.doRegister("fcd-output.attributes", new Option_StringVector());
    //     oc.addDescription("fcd-output.attributes", "Output", TL("List attributes that should be included in the FCD output"));
    //     oc.doRegister("fcd-output.filter-shapes", new Option_StringVector());
    //     oc.addDescription("fcd-output.filter-shapes", "Output", TL("List shape names that should be used to filter the FCD output"));
    //     oc.doRegister("fcd-output.skip-empty", new Option_Bool(false));
    //     oc.addDescription("fcd-output.skip-empty", "Output", TL("Do not save data for time steps which have no vehicles / transportables"));
    //
    //     oc.doRegister("person-fcd-output", new Option_FileName());
    //     oc.addSynonyme("person-fcd-output", "person-fcd");
    //     oc.addDescription("person-fcd-output", "Output", TL("Save fcd for persons and container to separate FILE"));
    //
    //     oc.doRegister("device.ssm.filter-edges.input-file", new Option_FileName());
    //     oc.addDescription("device.ssm.filter-edges.input-file", "Output", TL("Restrict SSM device output to the edge selection from the given input file"));
    //
    //     oc.doRegister("full-output", new Option_FileName());
    //     oc.addDescription("full-output", "Output", TL("Save a lot of information for each timestep (very redundant)"));
    //
    //     oc.doRegister("queue-output", new Option_FileName());
    //     oc.addDescription("queue-output", "Output", TL("Save the vehicle queues at the junctions (experimental)"));
    //     oc.doRegister("queue-output.period", new Option_String("-1", "TIME"));
    //     oc.addDescription("queue-output.period", "Output", TL("Save vehicle queues with the given period"));
    //     oc.doRegister("queue-output.aggregation", new Option_String("-1", "TIME"));
    //     oc.addDescription("queue-output.aggregation", "Output", TL("Write aggregated queue length statistics (max, median, 95th percentile) per edge for the given period (e.g. a traffic light cycle) instead of per-timestep data"));
    //     oc.doRegister("queue-output.speed-threshold", new Option_Float(5.0 / 3.6));
    //     oc.addDescription("queue-output.speed-threshold", "Output", TL("Maximum speed for counting a vehicle as queued"));
    //     oc.doRegister("queue-output.percentile", new Option_Float(95.));
    //     oc.addDescription("queue-output.percentile", "Output", TL("The percentile (0-100) to report as percentileQueueLength in aggregated queue output"));
    //     oc.doRegister("queue-output.skip-empty", new Option_Bool(false));
    //     oc.addDescription("queue-output.skip-empty", "Output", TL("Do not save data for time steps / intervals which have no queue (required for column based output formats)"));
    //
    //     oc.doRegister("vtk-output", new Option_FileName());
    //     oc.addDescription("vtk-output", "Output", TL("Save complete vehicle positions inclusive speed values in the VTK Format (usage: /path/out will produce /path/out_$TIMESTEP$.vtp files)"));
    //     oc.doRegister("amitran-output", new Option_FileName());
    //     oc.addDescription("amitran-output", "Output", TL("Save the vehicle trajectories in the Amitran format"));
    //
    //
    //     oc.doRegister("summary-output", new Option_FileName());
    //     oc.addSynonyme("summary-output", "summary");
    //     oc.addDescription("summary-output", "Output", TL("Save aggregated vehicle departure info into FILE"));
    //
    //     oc.doRegister("summary-output.period", new Option_String("-1", "TIME"));
    //     oc.addDescription("summary-output.period", "Output", TL("Save summary-output with the given period"));
    //
    //     oc.doRegister("person-summary-output", new Option_FileName());
    //     oc.addDescription("person-summary-output", "Output", TL("Save aggregated person counts into FILE"));
    //
    //     oc.doRegister("tripinfo-output", new Option_FileName());
    //     oc.addSynonyme("tripinfo-output", "tripinfo");
    //     oc.addDescription("tripinfo-output", "Output", TL("Save single vehicle trip info into FILE"));
    //
    //     oc.doRegister("tripinfo-output.write-unfinished", new Option_Bool(false));
    //     oc.addDescription("tripinfo-output.write-unfinished", "Output", TL("Write tripinfo output for vehicles which have not arrived at simulation end"));
    //
    //     oc.doRegister("tripinfo-output.write-undeparted", new Option_Bool(false));
    //     oc.addDescription("tripinfo-output.write-undeparted", "Output", TL("Write tripinfo output for vehicles which have not departed at simulation end because of depart delay"));
    //
    //     oc.doRegister("personinfo-output", new Option_FileName());
    //     oc.addSynonyme("personinfo-output", "personinfo");
    //     oc.addDescription("personinfo-output", "Output", TL("Save personinfo and containerinfo to separate FILE"));
    //
    //     oc.doRegister("vehroute-output", new Option_FileName());
    //     oc.addSynonyme("vehroute-output", "vehroutes");
    //     oc.addDescription("vehroute-output", "Output", TL("Save single vehicle route info into FILE"));
    //
    //     oc.doRegister("vehroute-output.exit-times", new Option_Bool(false));
    //     oc.addSynonyme("vehroute-output.exit-times", "vehroutes.exit-times");
    //     oc.addDescription("vehroute-output.exit-times", "Output", TL("Write the exit times for all edges"));
    //
    //     oc.doRegister("vehroute-output.last-route", new Option_Bool(false));
    //     oc.addSynonyme("vehroute-output.last-route", "vehroutes.last-route");
    //     oc.addDescription("vehroute-output.last-route", "Output", TL("Write the last route only"));
    //
    //     oc.doRegister("vehroute-output.sorted", new Option_Bool(false));
    //     oc.addSynonyme("vehroute-output.sorted", "vehroutes.sorted");
    //     oc.addDescription("vehroute-output.sorted", "Output", TL("Sorts the output by departure time"));
    //
    //     oc.doRegister("vehroute-output.dua", new Option_Bool(false));
    //     oc.addSynonyme("vehroute-output.dua", "vehroutes.dua");
    //     oc.addDescription("vehroute-output.dua", "Output", TL("Write the output in the duarouter alternatives style"));
    //
    //     oc.doRegister("vehroute-output.cost", new Option_Bool(false));
    //     oc.addDescription("vehroute-output.cost", "Output", TL("Write costs for all routes"));
    //
    //     oc.doRegister("vehroute-output.intended-depart", new Option_Bool(false));
    //     oc.addSynonyme("vehroute-output.intended-depart", "vehroutes.intended-depart");
    //     oc.addDescription("vehroute-output.intended-depart", "Output", TL("Write the output with the intended instead of the real departure time"));
    //
    //     oc.doRegister("vehroute-output.route-length", new Option_Bool(false));
    //     oc.addSynonyme("vehroute-output.route-length", "vehroutes.route-length");
    //     oc.addDescription("vehroute-output.route-length", "Output", TL("Include total route length in the output"));
    //
    //     oc.doRegister("vehroute-output.write-unfinished", new Option_Bool(false));
    //     oc.addDescription("vehroute-output.write-unfinished", "Output", TL("Write vehroute output for vehicles which have not arrived at simulation end"));
    //
    //     oc.doRegister("vehroute-output.skip-ptlines", new Option_Bool(false));
    //     oc.addDescription("vehroute-output.skip-ptlines", "Output", TL("Skip vehroute output for public transport vehicles"));
    //
    //     oc.doRegister("vehroute-output.incomplete", new Option_Bool(false));
    //     oc.addDescription("vehroute-output.incomplete", "Output", TL("Include invalid routes and route stubs in vehroute output"));
    //
    //     oc.doRegister("vehroute-output.stop-edges", new Option_Bool(false));
    //     oc.addDescription("vehroute-output.stop-edges", "Output", TL("Include information about edges between stops"));
    //
    //     oc.doRegister("vehroute-output.speedfactor", new Option_Bool(false));
    //     oc.addDescription("vehroute-output.speedfactor", "Output", TL("Write the vehicle speedFactor (defaults to 'true' if departSpeed is written)"));
    //
    //     oc.doRegister("vehroute-output.internal", new Option_Bool(false));
    //     oc.addDescription("vehroute-output.internal", "Output", TL("Include internal edges in the output"));
    //
    //     oc.doRegister("personroute-output", new Option_FileName());
    //     oc.addSynonyme("personroute-output", "personroutes");
    //     oc.addDescription("personroute-output", "Output", TL("Save person and container routes to separate FILE"));
    //
    //     oc.doRegister("link-output", new Option_FileName());
    //     oc.addDescription("link-output", "Output", TL("Save links states into FILE"));
    //
    //     oc.doRegister("railsignal-block-output", new Option_FileName());
    //     oc.addDescription("railsignal-block-output", "Output", TL("Save railsignal-blocks into FILE"));
    //
    //     oc.doRegister("railsignal-vehicle-output", new Option_FileName());
    //     oc.addDescription("railsignal-vehicle-output", "Output", TL("Record entry and exit times of vehicles for railsignal blocks into FILE"));
    //
    //     oc.doRegister("bt-output", new Option_FileName());
    //     oc.addDescription("bt-output", "Output", TL("Save bluetooth visibilities into FILE (in conjunction with device.btreceiver and device.btsender)"));
    //
    //     oc.doRegister("lanechange-output", new Option_FileName());
    //     oc.addDescription("lanechange-output", "Output", TL("Record lane changes and their motivations for all vehicles into FILE"));
    //
    //     oc.doRegister("lanechange-output.started", new Option_Bool(false));
    //     oc.addDescription("lanechange-output.started", "Output", TL("Record start of lane change manoeuvres"));
    //
    //     oc.doRegister("lanechange-output.ended", new Option_Bool(false));
    //     oc.addDescription("lanechange-output.ended", "Output", TL("Record end of lane change manoeuvres"));
    //
    //     oc.doRegister("lanechange-output.xy", new Option_Bool(false));
    //     oc.addDescription("lanechange-output.xy", "Output", TL("Record coordinates of lane change manoeuvres"));
    //
    //     oc.doRegister("stop-output", new Option_FileName());
    //     oc.addDescription("stop-output", "Output", TL("Record stops and loading/unloading of passenger and containers for all vehicles into FILE"));
    //     oc.doRegister("stop-output.write-unfinished", new Option_Bool(false));
    //     oc.addDescription("stop-output.write-unfinished", "Output", TL("Write stop output for stops which have not ended at simulation end"));
    //
    //     oc.doRegister("collision-output", new Option_FileName());
    //     oc.addDescription("collision-output", "Output", TL("Write collision information into FILE"));
    //
    //     oc.doRegister("edgedata-output", new Option_FileName());
    //     oc.addDescription("edgedata-output", "Output", TL("Write aggregated traffic statistics for all edges into FILE"));
    //     oc.doRegister("lanedata-output", new Option_FileName());
    //     oc.addDescription("lanedata-output", "Output", TL("Write aggregated traffic statistics for all lanes into FILE"));
    //
    //     oc.doRegister("statistic-output", new Option_FileName());
    //     oc.addSynonyme("statistic-output", "statistics-output");
    //     oc.addDescription("statistic-output", "Output", TL("Write overall statistics into FILE"));
    //
    //     oc.doRegister("deadlock-output", new Option_FileName());
    //     oc.addDescription("deadlock-output", "Output", TL("Write reports on deadlocks FILE"));
    //
    // #ifdef _DEBUG
    //     oc.doRegister("movereminder-output", new Option_FileName());
    //     oc.addDescription("movereminder-output", "Output", TL("Save movereminder states of selected vehicles into FILE"));
    //     oc.doRegister("movereminder-output.vehicles", new Option_StringVector());
    //     oc.addDescription("movereminder-output.vehicles", "Output", TL("List of vehicle ids which shall save their movereminder states"));
    // #endif
    //
    //     oc.doRegister("save-state.times", new Option_StringVector());
    //     oc.addDescription("save-state.times", "Output", TL("Use TIME[] as times at which a network state written"));
    //     oc.doRegister("save-state.period", new Option_String("-1", "TIME"));
    //     oc.addDescription("save-state.period", "Output", TL("save state repeatedly after TIME period"));
    //     oc.doRegister("save-state.period.keep", new Option_Integer(0));
    //     oc.addDescription("save-state.period.keep", "Output", TL("Keep only the last INT periodic state files"));
    //     oc.doRegister("save-state.prefix", new Option_FileName(StringVector({ "state" })));
    //     oc.addDescription("save-state.prefix", "Output", TL("Prefix for network states"));
    //     oc.doRegister("save-state.suffix", new Option_String(".xml.gz"));
    //     oc.addDescription("save-state.suffix", "Output", TL("Suffix for network states (.xml.gz or .xml)"));
    //     oc.doRegister("save-state.files", new Option_FileName());
    //     oc.addDescription("save-state.files", "Output", TL("Files for network states"));
    //     oc.doRegister("save-state.rng", new Option_Bool(false));
    //     oc.addDescription("save-state.rng", "Output", TL("Save random number generator states"));
    //     oc.doRegister("save-state.transportables", new Option_Bool(false));
    //     oc.addDescription("save-state.transportables", "Output", TL("Save person and container states (experimental)"));
    //     oc.doRegister("save-state.constraints", new Option_Bool(false));
    //     oc.addDescription("save-state.constraints", "Output", TL("Save rail signal constraints"));
    //     oc.doRegister("save-state.precision", new Option_Integer(2));
    //     oc.addDescription("save-state.precision", "Output", TL("Write internal state values with the given precision (default 2)"));
    //
    //     // register the simulation settings
    //     oc.doRegister("begin", 'b', new Option_String("0", "TIME"));
    //     oc.addDescription("begin", "Time", TL("Defines the begin time in seconds; The simulation starts at this time"));
    //
    //     oc.doRegister("end", 'e', new Option_String("-1", "TIME"));
    //     oc.addDescription("end", "Time", TL("Defines the end time in seconds; The simulation ends at this time"));
    //
    //     oc.doRegister("step-length", new Option_String("1", "TIME"));
    //     oc.addDescription("step-length", "Time", TL("Defines the step duration in seconds"));
    //
    //     oc.doRegister("step-method.ballistic", new Option_Bool(false));
    //     oc.addDescription("step-method.ballistic", "Processing", TL("Whether to use ballistic method for the positional update of vehicles (default is a semi-implicit Euler method)."));
    //
    //     oc.doRegister("extrapolate-departpos", new Option_Bool(false));
    //     oc.addDescription("extrapolate-departpos", "Processing", TL("Whether vehicles that depart between simulation steps should extrapolate the depart position"));
    //
    //     oc.doRegister("threads", new Option_Integer(1));
    //     oc.addDescription("threads", "Processing", TL("Defines the number of threads for parallel simulation"));
    //
    //     oc.doRegister("lateral-resolution", new Option_Float(-1));
    //     oc.addDescription("lateral-resolution", "Processing", TL("Defines the resolution in m when handling lateral positioning within a lane (with -1 all vehicles drive at the center of their lane"));
    //
    //     // register the processing options
    //     oc.doRegister("route-steps", 's', new Option_String("200", "TIME"));
    //     oc.addDescription("route-steps", "Processing", TL("Load routes for the next number of seconds ahead"));
    //
    //     oc.doRegister("no-internal-links", new Option_Bool(false));
    //     oc.addDescription("no-internal-links", "Processing", TL("Disable (junction) internal links"));
    //
    //     oc.doRegister("ignore-junction-blocker", new Option_String("-1", "TIME"));
    //     oc.addDescription("ignore-junction-blocker", "Processing", TL("Ignore vehicles which block the junction after they have been standing for SECONDS (-1 means never ignore)"));
    //
    //     oc.doRegister("ignore-route-errors", new Option_Bool(false));
    //     oc.addDescription("ignore-route-errors", "Processing", TL("Do not check whether routes are connected"));
    //
    //     oc.doRegister("ignore-accidents", new Option_Bool(false));
    //     oc.addDescription("ignore-accidents", "Processing", TL("Do not check whether accidents occur"));
    //
    //     oc.doRegister("collision.action", new Option_String("teleport"));
    //     oc.addDescription("collision.action", "Processing", TL("How to deal with collisions: [none,warn,teleport,remove]"));
    //
    //     oc.doRegister("intermodal-collision.action", new Option_String("warn"));
    //     oc.addDescription("intermodal-collision.action", "Processing", TL("How to deal with collisions between vehicle and pedestrian: [none,warn,teleport,remove]"));
    //
    //     oc.doRegister("collision.stoptime", new Option_String("0", "TIME"));
    //     oc.addDescription("collision.stoptime", "Processing", TL("Let vehicle stop for TIME before performing collision.action (except for action 'none')"));
    //
    //     oc.doRegister("intermodal-collision.stoptime", new Option_String("0", "TIME"));
    //     oc.addDescription("intermodal-collision.stoptime", "Processing", TL("Let vehicle stop for TIME before performing intermodal-collision.action (except for action 'none')"));
    //
    //     oc.doRegister("collision.check-junctions", new Option_Bool(false));
    //     oc.addDescription("collision.check-junctions", "Processing", TL("Enables collisions checks on junctions"));
    //
    //     oc.doRegister("collision.check-junctions.mingap", new Option_Float(0));
    //     oc.addDescription("collision.check-junctions.mingap", "Processing", TL("Increase or decrease sensitivity for junction collision check"));
    //
    //     oc.doRegister("collision.mingap-factor", new Option_Float(-1));
    //     oc.addDescription("collision.mingap-factor", "Processing", TL("Sets the fraction of minGap that must be maintained to avoid collision detection. If a negative value is given, the carFollowModel parameter is used"));
    //
    //     oc.doRegister("keep-after-arrival", new Option_String("0", "TIME"));
    //     oc.addDescription("keep-after-arrival", "Processing", TL("After a vehicle arrives, keep it in memory for the given TIME (for TraCI access)"));
    //
    //     oc.doRegister("max-num-vehicles", new Option_Integer(-1));
    //     oc.addDescription("max-num-vehicles", "Processing", TL("Delay vehicle insertion to stay within the given maximum number"));
    //
    //     oc.doRegister("max-num-persons", new Option_Integer(-1));
    //     oc.addDescription("max-num-persons", "Persons", TL("Delay person insertion to stay within the given maximum number"));
    //
    //     oc.doRegister("max-num-teleports", new Option_Integer(-1));
    //     oc.addDescription("max-num-teleports", "Processing", TL("Abort the simulation if the given maximum number of teleports is exceeded"));
    //
    //     oc.doRegister("scale", new Option_Float(1.));
    //     oc.addDescription("scale", "Processing", TL("Scale demand by the given factor (by discarding or duplicating vehicles)"));
    //
    //     oc.doRegister("scale-suffix", new Option_String("."));
    //     oc.addDescription("scale-suffix", "Processing", TL("Suffix to be added when creating ids for cloned vehicles"));
    //
    //     oc.doRegister("time-to-teleport", new Option_String("300", "TIME"));
    //     oc.addDescription("time-to-teleport", "Processing", TL("Specify how long a vehicle may wait until being teleported, defaults to 300, non-positive values disable teleporting"));
    //
    //     oc.doRegister("time-to-teleport.highways", new Option_String("0", "TIME"));
    //     oc.addDescription("time-to-teleport.highways", "Processing", TL("The waiting time after which vehicles on a fast road (speed > 69km/h) are teleported if they are on a non-continuing lane"));
    //
    //     oc.doRegister("time-to-teleport.highways.min-speed", new Option_Float(69 / 3.6));
    //     oc.addDescription("time-to-teleport.highways.min-speed", "Processing", TL("The waiting time after which vehicles on a fast road (default: speed > 69km/h) are teleported if they are on a non-continuing lane"));
    //
    //     oc.doRegister("time-to-teleport.disconnected", new Option_String("-1", "TIME"));
    //     oc.addDescription("time-to-teleport.disconnected", "Processing", TL("The waiting time after which vehicles with a disconnected route are teleported. Negative values disable teleporting"));
    //
    //     oc.doRegister("time-to-teleport.remove", new Option_Bool(false));
    //     oc.addDescription("time-to-teleport.remove", "Processing", TL("Whether vehicles shall be removed after waiting too long instead of being teleported"));
    //
    //     oc.doRegister("time-to-teleport.remove-constraint", new Option_Bool(false));
    //     oc.addDescription("time-to-teleport.remove-constraint", "Processing", TL("Whether rail-signal-constraint based deadlocks shall be cleared by removing a constraint"));
    //
    //     oc.doRegister("time-to-teleport.ride", new Option_String("-1", "TIME"));
    //     oc.addDescription("time-to-teleport.ride", "Persons", TL("The waiting time after which persons / containers waiting for a pickup are teleported. Negative values disable teleporting"));
    //
    //     oc.doRegister("time-to-teleport.bidi", new Option_String("-1", "TIME"));
    //     oc.addDescription("time-to-teleport.bidi", "Processing", TL("The waiting time after which vehicles on bidirectional edges are teleported"));
    //
    //     oc.doRegister("time-to-teleport.railsignal-deadlock", new Option_String("-1", "TIME"));
    //     oc.addDescription("time-to-teleport.railsignal-deadlock", "Processing", TL("The waiting time after which vehicles in a rail-signal based deadlock are teleported"));
    //
    //     oc.doRegister("waiting-time-memory", new Option_String("100", "TIME"));
    //     oc.addDescription("waiting-time-memory", "Processing", TL("Length of time interval, over which accumulated waiting time is taken into account (default is 100s.)"));
    //
    //     oc.doRegister("startup-wait-threshold", new Option_String("2", "TIME"));
    //     oc.addDescription("startup-wait-threshold", "Processing", TL("Minimum consecutive waiting time before applying startupDelay"));
    //
    //     oc.doRegister("max-depart-delay", new Option_String("-1", "TIME"));
    //     oc.addDescription("max-depart-delay", "Processing", TL("How long vehicles wait for departure before being skipped, defaults to -1 which means vehicles are never skipped"));
    //
    //     oc.doRegister("sloppy-insert", new Option_Bool(false));
    //     oc.addDescription("sloppy-insert", "Processing", TL("Whether insertion on an edge shall not be repeated in same step once failed"));
    //
    //     oc.doRegister("eager-insert", new Option_Bool(false));
    //     oc.addDescription("eager-insert", "Processing", TL("Whether each vehicle is checked separately for insertion on an edge"));
    //
    //     oc.doRegister("emergency-insert", new Option_Bool(false));
    //     oc.addDescription("emergency-insert", "Processing", TL("Allow inserting a vehicle in a situation which requires emergency braking"));
    //
    //     oc.doRegister("insertion-checks", new Option_String("all"));
    //     oc.addDescription("insertion-checks", "Processing", TL("Override default value for vehicle attribute insertionChecks"));
    //
    //     oc.doRegister("random-depart-offset", new Option_String("0", "TIME"));
    //     oc.addDescription("random-depart-offset", "Processing", TL("Each vehicle receives a random offset to its depart value drawn uniformly from [0, TIME]"));
    //
    //     oc.doRegister("lanechange.duration", new Option_String("0", "TIME"));
    //     oc.addDescription("lanechange.duration", "Processing", TL("Duration of a lane change maneuver (default 0)"));
    //
    //     oc.doRegister("lanechange.overtake-right", new Option_Bool(false));
    //     oc.addDescription("lanechange.overtake-right", "Processing", TL("Whether overtaking on the right on motorways is permitted"));
    //
    //     oc.doRegister("tls.all-off", new Option_Bool(false));
    //     oc.addDescription("tls.all-off", "Processing", TL("Switches off all traffic lights."));
    //
    //     oc.doRegister("tls.actuated.show-detectors", new Option_Bool(false));
    //     oc.addDescription("tls.actuated.show-detectors", "Processing", TL("Sets default visibility for actuation detectors"));
    //
    //     oc.doRegister("tls.actuated.jam-threshold", new Option_Float(-1));
    //     oc.addDescription("tls.actuated.jam-threshold", "Processing", TL("Sets default jam-threshold parameter for all actuation detectors"));
    //
    //     oc.doRegister("tls.actuated.detector-length", new Option_Float(0));
    //     oc.addDescription("tls.actuated.detector-length", "Processing", TL("Sets default detector length parameter for all actuation detectors"));
    //
    //     oc.doRegister("tls.delay_based.detector-range", new Option_Float(100));
    //     oc.addDescription("tls.delay_based.detector-range", "Processing", TL("Sets default range for detecting delayed vehicles"));
    //
    //     oc.doRegister("tls.yellow.min-decel", new Option_Float(3.0));
    //     oc.addDescription("tls.yellow.min-decel", "Processing", TL("Minimum deceleration when braking at yellow"));
    //
    //     oc.doRegister("railsignal-moving-block", new Option_Bool(false));
    //     oc.addDescription("railsignal-moving-block", "Processing", TL("Let railsignals operate in moving-block mode by default"));
    //     oc.addSynonyme("railsignal-moving-block", "railsignal.moving-block");
    //
    //     oc.doRegister("railsignal.moving-block.default-classes", new Option_StringVector(StringVector({"tram", "cable_car"})));
    //     oc.addDescription("railsignal.moving-block.default-classes", "Processing", TL("List vehicle classes that default to moving-block operations"));
    //
    //     oc.doRegister("railsignal.moving-block.max-dist", new Option_Float(200));
    //     oc.addDescription("railsignal.moving-block.max-dist", "Processing", TL("Maximum signal distance above which zipper conflicts are ignored"));
    //
    //     oc.doRegister("railsignal.max-block-length", new Option_Float(2e4));
    //     oc.addDescription("railsignal.max-block-length", "Processing", TL("Do not build blocks longer than FLOAT and issue a warning instead"));
    //
    //     oc.doRegister("railsignal.default-classes", new Option_StringVector(StringVector({"rail", "rail_fast", "rail_electric", "rail_urban", "subway"})));
    //     oc.addDescription("railsignal.default-classes", "Processing", TL("List vehicle classes that uses block-based insertion checks even when the network has no rail signals for them"));
    //
    //     oc.doRegister("slope-centered", new Option_Bool(false));
    //     oc.addDescription("slope-centered", "Processing", TL("Compute slope at the vehicle center of mass instead of integrating over front and back"));
    //
    //     oc.doRegister("time-to-impatience", new Option_String("180", "TIME"));
    //     oc.addDescription("time-to-impatience", "Processing", TL("Specify how long a vehicle may wait until impatience grows from 0 to 1, defaults to 300, non-positive values disable impatience growth"));
    //
    //     oc.doRegister("default.departspeed", new Option_String("avg"));
    //     oc.addDescription("default.departspeed", "Processing", TL("Select default depart speed"));
    //
    //     oc.doRegister("default.departlane", new Option_String("best_prob"));
    //     oc.addDescription("default.departlane", "Processing", TL("Select default depart lane"));
    //
    //     oc.doRegister("default.action-step-length", new Option_Float(0.0));
    //     oc.addDescription("default.action-step-length", "Processing", TL("Length of the default interval length between action points for the car-following and lane-change models (in seconds). If not specified, the simulation step-length is used per default. Vehicle- or VType-specific settings override the default. Must be a multiple of the simulation step-length."));
    //
    //     oc.doRegister("default.carfollowmodel", new Option_String("Krauss"));
    //     oc.addDescription("default.carfollowmodel", "Processing", TL("Select default car following model (Krauss, IDM, ...)"));
    //     oc.addSynonyme("default.carfollowmodel", "carfollow.model");
    //
    //     oc.doRegister("default.speeddev", new Option_Float(-1));
    //     oc.addDescription("default.speeddev", "Processing", TL("Select default speed deviation. A negative value implies vClass specific defaults (0.1 for the default passenger class)"));
    //
    //     oc.doRegister("default.emergencydecel", new Option_String("default"));
    //     oc.addDescription("default.emergencydecel", "Processing", TL("Select default emergencyDecel value among ('decel', 'default', FLOAT) which sets the value either to the same as the deceleration value, a vClass-class specific default or the given FLOAT in m/s^2"));
    //
    //     oc.doRegister("overhead-wire.solver", new Option_Bool(true));
    //     oc.addDescription("overhead-wire.solver", "Processing", TL("Use Kirchhoff's laws for solving overhead wire circuit"));
    //
    //     oc.doRegister("overhead-wire.recuperation", new Option_Bool(true));
    //     oc.addDescription("overhead-wire.recuperation", "Processing", TL("Enable recuperation from the vehicle equipped with elecHybrid device into the overhead wire."));
    //
    //     oc.doRegister("overhead-wire.substation-current-limits", new Option_Bool(true));
    //     oc.addDescription("overhead-wire.substation-current-limits", "Processing", TL("Enable current limits of traction substation during solving the overhead wire electrical circuit."));
    //
    //     oc.doRegister("emergencydecel.warning-threshold", new Option_Float(1));
    //     oc.addDescription("emergencydecel.warning-threshold", "Processing", TL("Sets the fraction of emergency decel capability that must be used to trigger a warning."));
    //
    //     oc.doRegister("parking.maneuver", new Option_Bool(false));
    //     oc.addDescription("parking.maneuver", "Processing", TL("Whether parking simulation includes maneuvering time and associated lane blocking"));
    //
    //     oc.doRegister("use-stop-ended", new Option_Bool(false));
    //     oc.addDescription("use-stop-ended", "Processing", TL("Override stop until times with stop ended times when given"));
    //
    //     oc.doRegister("use-stop-started", new Option_Bool(false));
    //     oc.addDescription("use-stop-started", "Processing", TL("Override stop arrival times with stop started times when given"));
    //
    //     // pedestrian model
    //     oc.doRegister("pedestrian.model", new Option_String("striping"));
    //     oc.addDescription("pedestrian.model", "Persons", TL("Select among pedestrian models ['nonInteracting', 'striping', 'jupedsim']"));
    //
    //     oc.doRegister("pedestrian.timegap-crossing", new Option_Float(2.));
    //     oc.addDescription("pedestrian.timegap-crossing", "Persons", TL("Minimal acceptable gap (in seconds) between two vehicles before starting to cross"));
    //
    //     oc.doRegister("pedestrian.striping.stripe-width", new Option_Float(0.64));
    //     oc.addDescription("pedestrian.striping.stripe-width", "Persons", TL("Width of parallel stripes for segmenting a sidewalk (meters) for use with model 'striping'"));
    //
    //     oc.doRegister("pedestrian.striping.dawdling", new Option_Float(0.2));
    //     oc.addDescription("pedestrian.striping.dawdling", "Persons", TL("Factor for random slow-downs [0,1] for use with model 'striping'"));
    //
    //     oc.doRegister("pedestrian.striping.mingap-to-vehicle", new Option_Float(0.25));
    //     oc.addDescription("pedestrian.striping.mingap-to-vehicle", "Persons", TL("Minimal gap / safety buffer (in meters) from a pedestrian to another vehicle for use with model 'striping'"));
    //
    //     oc.doRegister("pedestrian.striping.jamtime", new Option_String("300", "TIME"));
    //     oc.addDescription("pedestrian.striping.jamtime", "Persons", TL("Time in seconds after which pedestrians start squeezing through a jam when using model 'striping' (non-positive values disable squeezing)"));
    //     oc.doRegister("pedestrian.striping.jamtime.crossing", new Option_String("10", "TIME"));
    //     oc.addDescription("pedestrian.striping.jamtime.crossing", "Persons", TL("Time in seconds after which pedestrians start squeezing through a jam while on a pedestrian crossing when using model 'striping' (non-positive values disable squeezing)"));
    //     oc.doRegister("pedestrian.striping.jamtime.narrow", new Option_String("1", "TIME"));
    //     oc.addDescription("pedestrian.striping.jamtime.narrow", "Persons", TL("Time in seconds after which pedestrians start squeezing through a jam while on a narrow lane when using model 'striping'"));
    //
    //     oc.doRegister("pedestrian.striping.jamfactor", new Option_Float(0.25));
    //     oc.addDescription("pedestrian.striping.jamfactor", "Persons", TL("Factor for reducing speed of pedestrian in jammed state"));
    //
    //     oc.doRegister("pedestrian.striping.reserve-oncoming", new Option_Float(0.0));
    //     oc.addDescription("pedestrian.striping.reserve-oncoming", "Persons", TL("Fraction of stripes to reserve for oncoming pedestrians"));
    //
    //     oc.doRegister("pedestrian.striping.reserve-oncoming.junctions", new Option_Float(0.34));
    //     oc.addDescription("pedestrian.striping.reserve-oncoming.junctions", "Persons", TL("Fraction of stripes to reserve for oncoming pedestrians on crossings and walkingareas"));
    //
    //     oc.doRegister("pedestrian.striping.reserve-oncoming.max", new Option_Float(1.28));
    //     oc.addDescription("pedestrian.striping.reserve-oncoming.max", "Persons", TL("Maximum width in m to reserve for oncoming pedestrians"));
    //
    //     oc.doRegister("pedestrian.striping.legacy-departposlat", new Option_Bool(false));
    //     oc.addDescription("pedestrian.striping.legacy-departposlat", "Persons", TL("Interpret departPosLat for walks in legacy style"));
    //
    //     oc.doRegister("pedestrian.striping.walkingarea-detail", new Option_Integer(4));
    //     oc.addDescription("pedestrian.striping.walkingarea-detail", "Persons", TL("Generate INT intermediate points to smooth out lanes within the walkingarea"));
    //
    // #if defined(HAVE_JUPEDSIM_GRPC) || defined(HAVE_JUPEDSIM_LEGACY)
    //     oc.doRegister("pedestrian.jupedsim.step-length", new Option_String("0.01", "TIME"));
    //     oc.addDescription("pedestrian.jupedsim.step-length", "Persons", TL("The update interval of the JuPedSim simulation (in seconds)"));
    //     oc.doRegister("pedestrian.jupedsim.exit-tolerance", new Option_Float(1.));
    //     oc.addDescription("pedestrian.jupedsim.exit-tolerance", "Persons", TL("The distance to accept the JuPedSim arrival point (in meters)"));
    //     oc.doRegister("pedestrian.jupedsim.model", new Option_String("CollisionFreeSpeedModel"));
    //     oc.addDescription("pedestrian.jupedsim.model", "Persons", TL("The submodel to use in JuPedSim (default: 'CollisionFreeSpeedModel')"));
    //     oc.doRegister("pedestrian.jupedsim.strength-neighbor-repulsion", new Option_Float(8.));
    //     oc.addDescription("pedestrian.jupedsim.strength-neighbor-repulsion", "Persons", TL("The neighbor repulsion strength of the JuPedSim model"));
    //     oc.doRegister("pedestrian.jupedsim.range-neighbor-repulsion", new Option_Float(.1));
    //     oc.addDescription("pedestrian.jupedsim.range-neighbor-repulsion", "Persons", TL("The neighbor repulsion range of the JuPedSim model (in meters)"));
    //     oc.doRegister("pedestrian.jupedsim.strength-geometry-repulsion", new Option_Float(5.));
    //     oc.addDescription("pedestrian.jupedsim.strength-geometry-repulsion", "Persons", TL("The geometry repulsion strength of the JuPedSim model"));
    //     oc.doRegister("pedestrian.jupedsim.range-geometry-repulsion", new Option_Float(.02));
    //     oc.addDescription("pedestrian.jupedsim.range-geometry-repulsion", "Persons", TL("The geometry repulsion range of the JuPedSim model (in meters)"));
    //     oc.doRegister("pedestrian.jupedsim.wkt", new Option_FileName());
    //     oc.addDescription("pedestrian.jupedsim.wkt", "Output", TL("The filename to output the JuPedSim network as WKT"));
    //     oc.doRegister("pedestrian.jupedsim.wkt.geo", new Option_Bool(false));
    //     oc.addDescription("pedestrian.jupedsim.wkt.geo", "Output", TL("Whether to output JuPedSim network as WKT using geo-coordinates (lon/lat)"));
    //     oc.doRegister("pedestrian.jupedsim.py", new Option_FileName());
    //     oc.addDescription("pedestrian.jupedsim.py", "Output", TL("The filename to output the JuPedSim setup as Python script"));
    // #ifdef HAVE_JUPEDSIM_GRPC
    //     oc.doRegister("pedestrian.jupedsim.address", new Option_String(""));
    //     oc.addDescription("pedestrian.jupedsim.address", "Persons", TL("The address to connect to the JuPedSim gRPC server"));
    // #endif
    // #endif
    //
    //     oc.doRegister("ride.stop-tolerance", new Option_Float(10.));
    //     oc.addDescription("ride.stop-tolerance", "Persons", TL("Tolerance to apply when matching pedestrian and vehicle positions on boarding at individual stops"));
    //
    //     oc.doRegister("mapmatch.distance", new Option_Float(100));
    //     oc.addDescription("mapmatch.distance", "Processing", TL("Maximum distance when mapping input coordinates (fromXY etc.) to the road network"));
    //
    //     oc.doRegister("mapmatch.junctions", new Option_Bool(false));
    //     oc.addDescription("mapmatch.junctions", "Processing", TL("Match positions to junctions instead of edges"));
    //
    //     oc.doRegister("mapmatch.taz", new Option_Bool(false));
    //     oc.addDescription("mapmatch.taz", "Processing", TL("Match positions to taz instead of edges"));
    //
    //     // generic routing options
    //     oc.doRegister("routing-algorithm", new Option_String("dijkstra"));
    //     oc.addDescription("routing-algorithm", "Routing",
    //                       "Select among routing algorithms ['dijkstra', 'astar', 'CH', 'CHWrapper', 'CCH']");
    //
    //     oc.doRegister("weights.random-factor", new Option_Float(1.));
    //     oc.addDescription("weights.random-factor", "Routing", TL("Edge weights for routing are dynamically disturbed by a random factor drawn uniformly from [1,FLOAT)"));
    //
    //     oc.doRegister("weights.random-factor.dynamic", new Option_Bool(false));
    //     oc.addDescription("weights.random-factor.dynamic", "Routing", TL("When using option --weights.random-factor, vary the randomness over time"));
    //
    //     oc.doRegister("weights.minor-penalty", new Option_Float(1.5));
    //     oc.addDescription("weights.minor-penalty", "Routing", TL("Apply the given time penalty when computing minimum routing costs for minor-link internal lanes"));
    //
    //     oc.doRegister("weights.tls-penalty", new Option_Float(0));
    //     oc.addDescription("weights.tls-penalty", "Routing", TL("Apply scaled travel time penalties based on green split when computing minimum routing costs for internal lanes at traffic lights"));
    //
    //     oc.doRegister("weights.turnaround-penalty", new Option_Float(5.0));
    //     oc.addDescription("weights.turnaround-penalty", "Processing", TL("Apply the given time penalty when computing routing costs for turnaround internal lanes"));
    //
    //     oc.doRegister("weights.reversal-penalty", new Option_Float(60));
    //     oc.addDescription("weights.reversal-penalty", "Processing", TL("Apply the given time penalty when computing routing costs for train reversal. Negative values disable reversal"));
    //
    //     oc.doRegister("weights.priority-factor", new Option_Float(0));
    //     oc.addDescription("weights.priority-factor", "Routing", TL("Consider edge priorities in addition to travel times, weighted by factor"));
    //
    //     oc.doRegister("weights.separate-turns", new Option_Float(0));
    //     oc.addDescription("weights.separate-turns", "Routing", TL("Distinguish travel time by turn direction and shift a fraction of the estimated time loss ahead of the intersection onto the internal edges"));
    //
    //     oc.doRegister("astar.all-distances", new Option_FileName());
    //     oc.addDescription("astar.all-distances", "Routing", TL("Initialize lookup table for astar from the given file (generated by marouter --all-pairs-output)"));
    //
    //     oc.doRegister("astar.landmark-distances", new Option_FileName());
    //     oc.addDescription("astar.landmark-distances", "Routing", TL("Initialize lookup table for astar ALT-variant from the given file"));
    //
    //     oc.doRegister("persontrip.walkfactor", new Option_Float(double(0.75)));
    //     oc.addDescription("persontrip.walkfactor", "Persons", TL("Use FLOAT as a factor on pedestrian maximum speed during intermodal routing"));
    //
    //     oc.doRegister("persontrip.walk-opposite-factor", new Option_Float(1.0));
    //     oc.addDescription("persontrip.walk-opposite-factor", "Persons", TL("Use FLOAT as a factor on walking speed against vehicle traffic direction"));
    //
    //     oc.doRegister("persontrip.transfer.car-walk", new Option_StringVector(StringVector({ "parkingAreas" })));
    //     oc.addDescription("persontrip.transfer.car-walk", "Persons",
    //                       "Where are mode changes from car to walking allowed (possible values: 'parkingAreas', 'ptStops', 'allJunctions' and combinations)");
    //
    //     oc.doRegister("persontrip.transfer.taxi-walk", new Option_StringVector());
    //     oc.addDescription("persontrip.transfer.taxi-walk", "Persons", TL("Where taxis can drop off customers ('allJunctions, 'ptStops', 'parkingAreas')"));
    //
    //     oc.doRegister("persontrip.transfer.walk-taxi", new Option_StringVector());
    //     oc.addDescription("persontrip.transfer.walk-taxi", "Persons", TL("Where taxis can pick up customers ('allJunctions, 'ptStops', 'parkingAreas')"));
    //
    //     oc.doRegister("persontrip.default.group", new Option_String());
    //     oc.addDescription("persontrip.default.group", "Persons", TL("When set, trips between the same origin and destination will share a taxi by default"));
    //
    //     oc.doRegister("persontrip.taxi.waiting-time", new Option_String("300", "TIME"));
    //     oc.addDescription("persontrip.taxi.waiting-time", "Persons", TL("Estimated time for taxi pickup"));
    //
    //     oc.doRegister("persontrip.ride-public-line", new Option_Bool(false));
    //     oc.addDescription("persontrip.ride-public-line", "Persons", TL("Only use the intended public transport line rather than any alternative line that stops at the destination"));
    //
    //     oc.doRegister("railway.max-train-length", new Option_Float(1000.0));
    //     oc.addDescription("railway.max-train-length", "Routing", TL("Use FLOAT as a maximum train length when initializing the railway router"));
    //
    //     oc.doRegister("replay-rerouting", new Option_Bool(false));
    //     oc.addDescription("replay-rerouting", "Routing", TL("Replay exact rerouting sequence from vehroute-output"));
    //
    //     // devices
    //     oc.addOptionSubTopic("Emissions");
    //     oc.doRegister("emissions.volumetric-fuel", new Option_Bool(false));
    //     oc.addDescription("emissions.volumetric-fuel", "Emissions", TL("Return fuel consumption values in (legacy) unit l instead of mg"));
    //
    //     oc.doRegister("phemlight-path", new Option_FileName(StringVector({ "./PHEMlight/" })));
    //     oc.addDescription("phemlight-path", "Emissions", TL("Determines where to load PHEMlight definitions from"));
    //
    //     oc.doRegister("phemlight-year", new Option_Integer(0));
    //     oc.addDescription("phemlight-year", "Emissions", TL("Enable fleet age modeling with the given reference year in PHEMlight5"));
    //
    //     oc.doRegister("phemlight-temperature", new Option_Float(INVALID_DOUBLE));
    //     oc.addDescription("phemlight-temperature", "Emissions", TL("Set ambient temperature to correct NOx emissions in PHEMlight5"));
    //
    //     oc.addOptionSubTopic("Communication");
    //     oc.addOptionSubTopic("Battery");
    //     MSDevice::insertOptions(oc);
    //
    //     // register report options
    //     oc.doRegister("duration-log.disable", new Option_Bool(false));
    //     oc.addSynonyme("duration-log.disable", "no-duration-log", false);
    //     oc.addDescription("duration-log.disable", "Report", TL("Disable performance reports for individual simulation steps"));
    //
    //     oc.doRegister("duration-log.statistics", 't', new Option_Bool(false));
    //     oc.addDescription("duration-log.statistics", "Report", TL("Enable statistics on vehicle trips"));
    //
    //     oc.doRegister("no-step-log", new Option_Bool(false));
    //     oc.addDescription("no-step-log", "Report", TL("Disable console output of current simulation step"));
    //
    //     oc.doRegister("step-log.period", new Option_Integer(100));
    //     oc.addDescription("step-log.period", "Report", TL("Number of simulation steps between step-log outputs"));
    //
    //     //remote port 0 if not used
    //     oc.addOptionSubTopic("TraCI Server");
    //     oc.doRegister("remote-port", new Option_Integer(0));
    //     oc.addDescription("remote-port", "TraCI Server", TL("Enables TraCI Server if set"));
    //     oc.doRegister("num-clients", new Option_Integer(1));
    //     oc.addDescription("num-clients", "TraCI Server", TL("Expected number of connecting clients"));
    //
    //     oc.addOptionSubTopic("Mesoscopic");
    //     oc.doRegister("mesosim", new Option_Bool(false));
    //     oc.addDescription("mesosim", "Mesoscopic", TL("Enables mesoscopic simulation"));
    //     oc.doRegister("meso-ltm", new Option_Bool(false));
    //     oc.addDescription("meso-ltm", "Mesoscopic", TL("Enables the meso-LTM (LIFT) model"));
    //     oc.doRegister("meso-edgelength", new Option_Float(98.0f));
    //     oc.addDescription("meso-edgelength", "Mesoscopic", TL("Length of an edge segment in mesoscopic simulation"));
    //     oc.doRegister("meso-tauff", new Option_String("1.13", "TIME"));
    //     oc.addDescription("meso-tauff", "Mesoscopic", TL("Factor for calculating the net free-free headway time"));
    //     oc.doRegister("meso-taufj", new Option_String("1.13", "TIME"));
    //     oc.addDescription("meso-taufj", "Mesoscopic", TL("Factor for calculating the net free-jam headway time"));
    //     oc.doRegister("meso-taujf", new Option_String("1.73", "TIME"));
    //     oc.addDescription("meso-taujf", "Mesoscopic", TL("Factor for calculating the jam-free headway time"));
    //     oc.doRegister("meso-taujj", new Option_String("1.4", "TIME"));
    //     oc.addDescription("meso-taujj", "Mesoscopic", TL("Factor for calculating the jam-jam headway time"));
    //     oc.doRegister("meso-jam-threshold", new Option_Float(-1));
    //     oc.addDescription("meso-jam-threshold", "Mesoscopic",
    //                       "Minimum percentage of occupied space to consider a segment jammed. A negative argument causes thresholds to be computed based on edge speed and tauff (default)");
    //     oc.doRegister("meso-multi-queue", new Option_Bool(true));
    //     oc.addDescription("meso-multi-queue", "Mesoscopic", TL("Enable multiple queues at edge ends"));
    //     oc.doRegister("meso-lane-queue", new Option_Bool(false));
    //     oc.addDescription("meso-lane-queue", "Mesoscopic", TL("Enable separate queues for every lane"));
    //     oc.doRegister("meso-ignore-lanes-by-vclass", new Option_StringVector(StringVector({ "pedestrian", "bicycle" })));
    //     oc.addDescription("meso-ignore-lanes-by-vclass", "Mesoscopic", TL("Do not build queues (or reduce capacity) for lanes allowing only the given vclasses"));
    //     oc.addSynonyme("meso-ignore-lanes-by-vclass", "meso.ignore-lanes.by-vclass");
    //     oc.doRegister("meso-junction-control", new Option_Bool(false));
    //     oc.addDescription("meso-junction-control", "Mesoscopic", TL("Enable mesoscopic traffic light and priority junction handling"));
    //     oc.doRegister("meso-junction-control.limited", new Option_Bool(false));
    //     oc.addDescription("meso-junction-control.limited", "Mesoscopic",
    //                       "Enable mesoscopic traffic light and priority junction handling for saturated links. This prevents faulty traffic lights from hindering flow in low-traffic situations");
    //     oc.doRegister("meso-tls-penalty", new Option_Float(0));
    //     oc.addDescription("meso-tls-penalty", "Mesoscopic",
    //                       "Apply scaled travel time penalties when driving across tls controlled junctions based on green split instead of checking actual phases");
    //     oc.doRegister("meso-tls-flow-penalty", new Option_Float(0));
    //     oc.addDescription("meso-tls-flow-penalty", "Mesoscopic",
    //                       "Apply scaled headway penalties when driving across tls controlled junctions based on green split instead of checking actual phases");
    //     oc.doRegister("meso-minor-penalty", new Option_String("0", "TIME"));
    //     oc.addDescription("meso-minor-penalty", "Mesoscopic",
    //                       "Apply fixed time penalty when driving across a minor link. When using --meso-junction-control.limited, the penalty is not applied whenever limited control is active.");
    //     oc.doRegister("meso-overtaking", new Option_Bool(false));
    //     oc.addDescription("meso-overtaking", "Mesoscopic", TL("Enable mesoscopic overtaking"));
    //     oc.doRegister("meso-recheck", new Option_String("0", "TIME"));
    //     oc.addDescription("meso-recheck", "Mesoscopic", TL("Time interval for rechecking insertion into the next segment after failure"));
    //     oc.doRegister("meso-interpolate-pos", new Option_Bool(false));
    //     oc.addDescription("meso-interpolate-pos", "Mesoscopic", TL("Enable mesoscopic position interpolation"));
    //
    //     // add rand options
    //     RandHelper::insertRandOptions(oc);
    //     oc.doRegister("thread-rngs", new Option_Integer(64));
    //     oc.addDescription("thread-rngs", "Random Number",
    //                       "Number of pre-allocated random number generators to ensure repeatable multi-threaded simulations (should be at least the number of threads for repeatable simulations).");
    //
    //     // add GUI options
    //     // the reason that we include them in vanilla sumo as well is to make reusing config files easy
    //     oc.addOptionSubTopic("GUI Only");
    //     oc.doRegister("gui-settings-file", 'g', new Option_FileName());
    //     oc.addDescription("gui-settings-file", "GUI Only", TL("Load visualization settings from FILE"));
    //
    //     oc.doRegister("quit-on-end", 'Q', new Option_Bool(false));
    //     oc.addDescription("quit-on-end", "GUI Only", TL("Quits the GUI when the simulation stops"));
    //
    //     oc.doRegister("game", 'G', new Option_Bool(false));
    //     oc.addDescription("game", "GUI Only", TL("Start the GUI in gaming mode"));
    //
    //     oc.doRegister("game.mode", new Option_String("tls"));
    //     oc.addDescription("game.mode", "GUI Only", TL("Select the game type ('tls', 'drt')"));
    //
    //     oc.doRegister("start", 'S', new Option_Bool(false));
    //     oc.addDescription("start", "GUI Only", TL("Start the simulation after loading"));
    //
    //     oc.doRegister("delay", 'd', new Option_Float(0.0));
    //     oc.addDescription("delay", "GUI Only", TL("Use FLOAT in ms as delay between simulation steps"));
    //
    //     oc.doRegister("breakpoints", 'B', new Option_StringVector());
    //     oc.addDescription("breakpoints", "GUI Only", TL("Use TIME[] as times when the simulation should halt"));
    //
    //     oc.doRegister("edgedata-files", 'm', new Option_FileName());
    //     oc.addSynonyme("edgedata-files", "data-files");
    //     oc.addDescription("edgedata-files", "GUI Only", TL("Load edge/lane weights for visualization from FILE"));
    //
    //     oc.doRegister("alternative-net-file", 'N', new Option_FileName());
    //     oc.addDescription("alternative-net-file", "GUI Only", TL("Load a secondary road network for abstract visualization from FILE"));
    //
    //     oc.doRegister("selection-file", new Option_FileName());
    //     oc.addDescription("selection-file", "GUI Only", TL("Load pre-selected elements from FILE"));
    //
    //     oc.doRegister("demo", 'D', new Option_Bool(false));
    //     oc.addDescription("demo", "GUI Only", TL("Restart the simulation after ending (demo mode)"));
    //
    //     oc.doRegister("disable-textures", 'T', new Option_Bool(false));
    //     oc.addDescription("disable-textures", "GUI Only", TL("Do not load background pictures"));
    //
    //     oc.doRegister("registry-viewport", new Option_Bool(false));
    //     oc.addDescription("registry-viewport", "GUI Only", TL("Load current viewport from registry"));
    //
    //     oc.doRegister("window-size", new Option_StringVector());
    //     oc.addDescription("window-size", "GUI Only", TL("Create initial window with the given x,y size"));
    //
    //     oc.doRegister("window-pos", new Option_StringVector());
    //     oc.addDescription("window-pos", "GUI Only", TL("Create initial window at the given x,y position"));
    //
    //     oc.doRegister("tracker-interval", new Option_String("1", "TIME"));
    //     oc.addDescription("tracker-interval", "GUI Only", TL("The aggregation period for value tracker windows"));
    //
    // #ifdef HAVE_OSG
    //     oc.doRegister("osg-view", new Option_Bool(false));
    //     oc.addDescription("osg-view", "GUI Only", TL("Start with an OpenSceneGraph view instead of the regular 2D view"));
    // #endif
    //
    //     // gui testing
    //     oc.doRegister("gui-testing", new Option_Bool(false));
    //     oc.addDescription("gui-testing", "GUI Only", TL("Enable overlay for screen recognition"));
    //
    //     // gui testing - debug
    //     oc.doRegister("gui-testing-debug", new Option_Bool(false));
    //     oc.addDescription("gui-testing-debug", "GUI Only", TL("Enable output messages during GUI-Testing"));
    //
    //     // gui testing - settings output
    //     oc.doRegister("gui-testing.setting-output", new Option_FileName());
    //     oc.addDescription("gui-testing.setting-output", "GUI Only", TL("Save gui settings in the given settings output file"));
    // }
    #[allow(unused_variables)]
    pub fn fill_options() {
        utils::options::options_cont::with_options(|oc| {
            oc.add_call_example("-b 0 -e 1000 -n net.xml -r routes.xml", "start a simulation from time 0 to 1000 with given net and routes");
            oc.add_call_example("-c munich_config.cfg", "start with a configuration file");
            oc.add_call_example("--help", "print help");
            utils::common::system_frame::add_configuration_options(oc);
            oc.add_option_sub_topic("Input");
            oc.add_option_sub_topic("Output");
            oc.add_option_sub_topic("Time");
            oc.add_option_sub_topic("Processing");
            oc.add_option_sub_topic("Routing");
            oc.do_register_abbr("net-file", 'n', SumoOption::file_name());
            oc.add_synonyme("net-file", "net", false);
            oc.add_description("net-file", "Input", "Load road network description from FILE");
            oc.add_xml_default("net-file", "net");
            oc.do_register_abbr("route-files", 'r', SumoOption::file_name());
            oc.add_synonyme("route-files", "routes", false);
            oc.add_description("route-files", "Input", "Load routes descriptions from FILE(s)");
            oc.do_register_abbr("additional-files", 'a', SumoOption::file_name());
            oc.add_synonyme("additional-files", "additional", false);
            oc.add_description("additional-files", "Input", "Load further descriptions from FILE(s)");
            oc.do_register_abbr("weight-files", 'w', SumoOption::file_name());
            oc.add_synonyme("weight-files", "weights", false);
            oc.add_description("weight-files", "Input", "Load edge/lane weights for online rerouting from FILE");
            oc.do_register_abbr("weight-attribute", 'x', SumoOption::string_with("traveltime"));
            oc.add_synonyme("weight-attribute", "measure", true);
            oc.add_description("weight-attribute", "Input", "Name of the xml attribute which gives the edge weight");
            oc.do_register("load-state", SumoOption::file_name());
            oc.add_description("load-state", "Input", "Loads a network state from FILE");
            oc.do_register("load-state.offset", SumoOption::string_typed("0", "TIME"));
            oc.add_description("load-state.offset", "Input", "Shifts all times loaded from a saved state by the given offset");
            oc.do_register("load-state.remove-vehicles", SumoOption::string_vector_with(Vec::new()));
            oc.add_description("load-state.remove-vehicles", "Input", "Removes vehicles with the given IDs from the loaded state");
            oc.do_register("junction-taz", SumoOption::bool(false));
            oc.add_description("junction-taz", "Input", "Initialize a TAZ for every junction to use attributes toJunction and fromJunction");
            utils::common::system_frame::add_report_options(oc);
            oc.do_register("netstate-dump", SumoOption::file_name());
            oc.add_synonyme("netstate-dump", "ndump", false);
            oc.add_synonyme("netstate-dump", "netstate", false);
            oc.add_synonyme("netstate-dump", "netstate-output", false);
            oc.add_description("netstate-dump", "Output", "Save complete network states into FILE");
            oc.do_register("netstate-dump.empty-edges", SumoOption::bool(false));
            oc.add_synonyme("netstate-dump.empty-edges", "netstate.empty-edges", false);
            oc.add_synonyme("netstate-dump.empty-edges", "netstate-output.empty-edges", false);
            oc.add_synonyme("netstate-dump.empty-edges", "dump-empty-edges", true);
            oc.add_description("netstate-dump.empty-edges", "Output", "Write also empty edges completely when dumping");
            oc.do_register("netstate-dump.precision", SumoOption::integer(2));
            oc.add_synonyme("netstate-dump.precision", "netstate.precision", false);
            oc.add_synonyme("netstate-dump.precision", "netstate-output.precision", false);
            oc.add_synonyme("netstate-dump.precision", "dump-precision", true);
            oc.add_description("netstate-dump.precision", "Output", "Write positions and speeds with the given precision (default 2)");
            oc.do_register("emission-output", SumoOption::file_name());
            oc.add_description("emission-output", "Output", "Save the emission values of each vehicle");
            oc.do_register("emission-output.precision", SumoOption::integer(2));
            oc.add_description("emission-output.precision", "Output", "Write emission values with the given precision (default 2)");
            oc.do_register("emission-output.geo", SumoOption::bool(false));
            oc.add_description("emission-output.geo", "Output", "Save the positions in emission output using geo-coordinates (lon/lat)");
            oc.do_register("emission-output.step-scaled", SumoOption::bool(false));
            oc.add_description("emission-output.step-scaled", "Output", "Write emission values scaled to the step length rather than as per-second values");
            oc.do_register("emission-output.attributes", SumoOption::string_vector());
            oc.add_description("emission-output.attributes", "Output", "List attributes that should be included in the emission output");
            oc.do_register("battery-output", SumoOption::file_name());
            oc.add_description("battery-output", "Output", "Save the battery values of each vehicle");
            oc.do_register("battery-output.precision", SumoOption::integer(2));
            oc.add_description("battery-output.precision", "Output", "Write battery values with the given precision (default 2)");
            oc.do_register("elechybrid-output", SumoOption::file_name());
            oc.add_description("elechybrid-output", "Output", "Save the elecHybrid values of each vehicle");
            oc.do_register("elechybrid-output.precision", SumoOption::integer(2));
            oc.add_description("elechybrid-output.precision", "Output", "Write elecHybrid values with the given precision (default 2)");
            oc.do_register("elechybrid-output.aggregated", SumoOption::bool(false));
            oc.add_description("elechybrid-output.aggregated", "Output", "Write elecHybrid values into one aggregated file");
            oc.do_register("chargingstations-output", SumoOption::file_name());
            oc.add_description("chargingstations-output", "Output", "Write data of charging stations");
            oc.do_register("chargingstations-output.aggregated", SumoOption::bool(false));
            oc.add_description("chargingstations-output.aggregated", "Output", "Write aggregated charging event data instead of single time steps");
            oc.do_register("chargingstations-output.aggregated.write-unfinished", SumoOption::bool(false));
            oc.add_description("chargingstations-output.aggregated.write-unfinished", "Output", "Write aggregated charging event data for vehicles which have not arrived at simulation end");
            oc.do_register("overheadwiresegments-output", SumoOption::file_name());
            oc.add_description("overheadwiresegments-output", "Output", "Write data of overhead wire segments");
            oc.do_register("substations-output", SumoOption::file_name());
            oc.add_description("substations-output", "Output", "Write data of electrical substation stations");
            oc.do_register("substations-output.precision", SumoOption::integer(2));
            oc.add_description("substations-output.precision", "Output", "Write substation values with the given precision (default 2)");
            oc.do_register("fcd-output", SumoOption::file_name());
            oc.add_description("fcd-output", "Output", "Save the Floating Car Data");
            oc.do_register("fcd-output.geo", SumoOption::bool(false));
            oc.add_description("fcd-output.geo", "Output", "Save the Floating Car Data using geo-coordinates (lon/lat)");
            oc.do_register("fcd-output.utm", SumoOption::bool(false));
            oc.add_description("fcd-output.utm", "Output", "Save the Floating Car Data using utm/unshifted coordinates (x/y)");
            oc.do_register("fcd-output.signals", SumoOption::bool(false));
            oc.add_description("fcd-output.signals", "Output", "Add the vehicle signal state to the FCD output (brake lights etc.)");
            oc.do_register("fcd-output.distance", SumoOption::bool(false));
            oc.add_description("fcd-output.distance", "Output", "Add kilometrage to the FCD output (linear referencing)");
            oc.do_register("fcd-output.acceleration", SumoOption::bool(false));
            oc.add_description("fcd-output.acceleration", "Output", "Add acceleration to the FCD output");
            oc.do_register("fcd-output.speed-relative", SumoOption::bool(false));
            oc.add_description("fcd-output.speed-relative", "Output", "Add relative speed (vehicle speed / edge speed limit) to the FCD output");
            oc.do_register("fcd-output.max-leader-distance", SumoOption::float(-1.0));
            oc.add_description("fcd-output.max-leader-distance", "Output", "Add leader vehicle information to the FCD output (within the given distance)");
            oc.do_register("fcd-output.params", SumoOption::string_vector());
            oc.add_description("fcd-output.params", "Output", "Add generic parameter values to the FCD output");
            oc.do_register("fcd-output.filter-edges.input-file", SumoOption::file_name());
            oc.add_description("fcd-output.filter-edges.input-file", "Output", "Restrict fcd output to the edge selection from the given input file");
            oc.do_register("fcd-output.attributes", SumoOption::string_vector());
            oc.add_description("fcd-output.attributes", "Output", "List attributes that should be included in the FCD output");
            oc.do_register("fcd-output.filter-shapes", SumoOption::string_vector());
            oc.add_description("fcd-output.filter-shapes", "Output", "List shape names that should be used to filter the FCD output");
            oc.do_register("fcd-output.skip-empty", SumoOption::bool(false));
            oc.add_description("fcd-output.skip-empty", "Output", "Do not save data for time steps which have no vehicles / transportables");
            oc.do_register("person-fcd-output", SumoOption::file_name());
            oc.add_synonyme("person-fcd-output", "person-fcd", false);
            oc.add_description("person-fcd-output", "Output", "Save fcd for persons and container to separate FILE");
            oc.do_register("device.ssm.filter-edges.input-file", SumoOption::file_name());
            oc.add_description("device.ssm.filter-edges.input-file", "Output", "Restrict SSM device output to the edge selection from the given input file");
            oc.do_register("full-output", SumoOption::file_name());
            oc.add_description("full-output", "Output", "Save a lot of information for each timestep (very redundant)");
            oc.do_register("queue-output", SumoOption::file_name());
            oc.add_description("queue-output", "Output", "Save the vehicle queues at the junctions (experimental)");
            oc.do_register("queue-output.period", SumoOption::string_typed("-1", "TIME"));
            oc.add_description("queue-output.period", "Output", "Save vehicle queues with the given period");
            oc.do_register("queue-output.aggregation", SumoOption::string_typed("-1", "TIME"));
            oc.add_description("queue-output.aggregation", "Output", "Write aggregated queue length statistics (max, median, 95th percentile) per edge for the given period (e.g. a traffic light cycle) instead of per-timestep data");
            oc.do_register("queue-output.speed-threshold", SumoOption::float(5.0 / 3.6));
            oc.add_description("queue-output.speed-threshold", "Output", "Maximum speed for counting a vehicle as queued");
            oc.do_register("queue-output.percentile", SumoOption::float(95.0));
            oc.add_description("queue-output.percentile", "Output", "The percentile (0-100) to report as percentileQueueLength in aggregated queue output");
            oc.do_register("queue-output.skip-empty", SumoOption::bool(false));
            oc.add_description("queue-output.skip-empty", "Output", "Do not save data for time steps / intervals which have no queue (required for column based output formats)");
            oc.do_register("vtk-output", SumoOption::file_name());
            oc.add_description("vtk-output", "Output", "Save complete vehicle positions inclusive speed values in the VTK Format (usage: /path/out will produce /path/out_$TIMESTEP$.vtp files)");
            oc.do_register("amitran-output", SumoOption::file_name());
            oc.add_description("amitran-output", "Output", "Save the vehicle trajectories in the Amitran format");
            oc.do_register("summary-output", SumoOption::file_name());
            oc.add_synonyme("summary-output", "summary", false);
            oc.add_description("summary-output", "Output", "Save aggregated vehicle departure info into FILE");
            oc.do_register("summary-output.period", SumoOption::string_typed("-1", "TIME"));
            oc.add_description("summary-output.period", "Output", "Save summary-output with the given period");
            oc.do_register("person-summary-output", SumoOption::file_name());
            oc.add_description("person-summary-output", "Output", "Save aggregated person counts into FILE");
            oc.do_register("tripinfo-output", SumoOption::file_name());
            oc.add_synonyme("tripinfo-output", "tripinfo", false);
            oc.add_description("tripinfo-output", "Output", "Save single vehicle trip info into FILE");
            oc.do_register("tripinfo-output.write-unfinished", SumoOption::bool(false));
            oc.add_description("tripinfo-output.write-unfinished", "Output", "Write tripinfo output for vehicles which have not arrived at simulation end");
            oc.do_register("tripinfo-output.write-undeparted", SumoOption::bool(false));
            oc.add_description("tripinfo-output.write-undeparted", "Output", "Write tripinfo output for vehicles which have not departed at simulation end because of depart delay");
            oc.do_register("personinfo-output", SumoOption::file_name());
            oc.add_synonyme("personinfo-output", "personinfo", false);
            oc.add_description("personinfo-output", "Output", "Save personinfo and containerinfo to separate FILE");
            oc.do_register("vehroute-output", SumoOption::file_name());
            oc.add_synonyme("vehroute-output", "vehroutes", false);
            oc.add_description("vehroute-output", "Output", "Save single vehicle route info into FILE");
            oc.do_register("vehroute-output.exit-times", SumoOption::bool(false));
            oc.add_synonyme("vehroute-output.exit-times", "vehroutes.exit-times", false);
            oc.add_description("vehroute-output.exit-times", "Output", "Write the exit times for all edges");
            oc.do_register("vehroute-output.last-route", SumoOption::bool(false));
            oc.add_synonyme("vehroute-output.last-route", "vehroutes.last-route", false);
            oc.add_description("vehroute-output.last-route", "Output", "Write the last route only");
            oc.do_register("vehroute-output.sorted", SumoOption::bool(false));
            oc.add_synonyme("vehroute-output.sorted", "vehroutes.sorted", false);
            oc.add_description("vehroute-output.sorted", "Output", "Sorts the output by departure time");
            oc.do_register("vehroute-output.dua", SumoOption::bool(false));
            oc.add_synonyme("vehroute-output.dua", "vehroutes.dua", false);
            oc.add_description("vehroute-output.dua", "Output", "Write the output in the duarouter alternatives style");
            oc.do_register("vehroute-output.cost", SumoOption::bool(false));
            oc.add_description("vehroute-output.cost", "Output", "Write costs for all routes");
            oc.do_register("vehroute-output.intended-depart", SumoOption::bool(false));
            oc.add_synonyme("vehroute-output.intended-depart", "vehroutes.intended-depart", false);
            oc.add_description("vehroute-output.intended-depart", "Output", "Write the output with the intended instead of the real departure time");
            oc.do_register("vehroute-output.route-length", SumoOption::bool(false));
            oc.add_synonyme("vehroute-output.route-length", "vehroutes.route-length", false);
            oc.add_description("vehroute-output.route-length", "Output", "Include total route length in the output");
            oc.do_register("vehroute-output.write-unfinished", SumoOption::bool(false));
            oc.add_description("vehroute-output.write-unfinished", "Output", "Write vehroute output for vehicles which have not arrived at simulation end");
            oc.do_register("vehroute-output.skip-ptlines", SumoOption::bool(false));
            oc.add_description("vehroute-output.skip-ptlines", "Output", "Skip vehroute output for public transport vehicles");
            oc.do_register("vehroute-output.incomplete", SumoOption::bool(false));
            oc.add_description("vehroute-output.incomplete", "Output", "Include invalid routes and route stubs in vehroute output");
            oc.do_register("vehroute-output.stop-edges", SumoOption::bool(false));
            oc.add_description("vehroute-output.stop-edges", "Output", "Include information about edges between stops");
            oc.do_register("vehroute-output.speedfactor", SumoOption::bool(false));
            oc.add_description("vehroute-output.speedfactor", "Output", "Write the vehicle speedFactor (defaults to 'true' if departSpeed is written)");
            oc.do_register("vehroute-output.internal", SumoOption::bool(false));
            oc.add_description("vehroute-output.internal", "Output", "Include internal edges in the output");
            oc.do_register("personroute-output", SumoOption::file_name());
            oc.add_synonyme("personroute-output", "personroutes", false);
            oc.add_description("personroute-output", "Output", "Save person and container routes to separate FILE");
            oc.do_register("link-output", SumoOption::file_name());
            oc.add_description("link-output", "Output", "Save links states into FILE");
            oc.do_register("railsignal-block-output", SumoOption::file_name());
            oc.add_description("railsignal-block-output", "Output", "Save railsignal-blocks into FILE");
            oc.do_register("railsignal-vehicle-output", SumoOption::file_name());
            oc.add_description("railsignal-vehicle-output", "Output", "Record entry and exit times of vehicles for railsignal blocks into FILE");
            oc.do_register("bt-output", SumoOption::file_name());
            oc.add_description("bt-output", "Output", "Save bluetooth visibilities into FILE (in conjunction with device.btreceiver and device.btsender)");
            oc.do_register("lanechange-output", SumoOption::file_name());
            oc.add_description("lanechange-output", "Output", "Record lane changes and their motivations for all vehicles into FILE");
            oc.do_register("lanechange-output.started", SumoOption::bool(false));
            oc.add_description("lanechange-output.started", "Output", "Record start of lane change manoeuvres");
            oc.do_register("lanechange-output.ended", SumoOption::bool(false));
            oc.add_description("lanechange-output.ended", "Output", "Record end of lane change manoeuvres");
            oc.do_register("lanechange-output.xy", SumoOption::bool(false));
            oc.add_description("lanechange-output.xy", "Output", "Record coordinates of lane change manoeuvres");
            oc.do_register("stop-output", SumoOption::file_name());
            oc.add_description("stop-output", "Output", "Record stops and loading/unloading of passenger and containers for all vehicles into FILE");
            oc.do_register("stop-output.write-unfinished", SumoOption::bool(false));
            oc.add_description("stop-output.write-unfinished", "Output", "Write stop output for stops which have not ended at simulation end");
            oc.do_register("collision-output", SumoOption::file_name());
            oc.add_description("collision-output", "Output", "Write collision information into FILE");
            oc.do_register("edgedata-output", SumoOption::file_name());
            oc.add_description("edgedata-output", "Output", "Write aggregated traffic statistics for all edges into FILE");
            oc.do_register("lanedata-output", SumoOption::file_name());
            oc.add_description("lanedata-output", "Output", "Write aggregated traffic statistics for all lanes into FILE");
            oc.do_register("statistic-output", SumoOption::file_name());
            oc.add_synonyme("statistic-output", "statistics-output", false);
            oc.add_description("statistic-output", "Output", "Write overall statistics into FILE");
            oc.do_register("deadlock-output", SumoOption::file_name());
            oc.add_description("deadlock-output", "Output", "Write reports on deadlocks FILE");
            oc.do_register("save-state.times", SumoOption::string_vector());
            oc.add_description("save-state.times", "Output", "Use TIME[] as times at which a network state written");
            oc.do_register("save-state.period", SumoOption::string_typed("-1", "TIME"));
            oc.add_description("save-state.period", "Output", "save state repeatedly after TIME period");
            oc.do_register("save-state.period.keep", SumoOption::integer(0));
            oc.add_description("save-state.period.keep", "Output", "Keep only the last INT periodic state files");
            oc.do_register("save-state.prefix", SumoOption::file_name_with(vec!["state".to_string()]));
            oc.add_description("save-state.prefix", "Output", "Prefix for network states");
            oc.do_register("save-state.suffix", SumoOption::string_with(".xml.gz"));
            oc.add_description("save-state.suffix", "Output", "Suffix for network states (.xml.gz or .xml)");
            oc.do_register("save-state.files", SumoOption::file_name());
            oc.add_description("save-state.files", "Output", "Files for network states");
            oc.do_register("save-state.rng", SumoOption::bool(false));
            oc.add_description("save-state.rng", "Output", "Save random number generator states");
            oc.do_register("save-state.transportables", SumoOption::bool(false));
            oc.add_description("save-state.transportables", "Output", "Save person and container states (experimental)");
            oc.do_register("save-state.constraints", SumoOption::bool(false));
            oc.add_description("save-state.constraints", "Output", "Save rail signal constraints");
            oc.do_register("save-state.precision", SumoOption::integer(2));
            oc.add_description("save-state.precision", "Output", "Write internal state values with the given precision (default 2)");
            oc.do_register_abbr("begin", 'b', SumoOption::string_typed("0", "TIME"));
            oc.add_description("begin", "Time", "Defines the begin time in seconds; The simulation starts at this time");
            oc.do_register_abbr("end", 'e', SumoOption::string_typed("-1", "TIME"));
            oc.add_description("end", "Time", "Defines the end time in seconds; The simulation ends at this time");
            oc.do_register("step-length", SumoOption::string_typed("1", "TIME"));
            oc.add_description("step-length", "Time", "Defines the step duration in seconds");
            oc.do_register("step-method.ballistic", SumoOption::bool(false));
            oc.add_description("step-method.ballistic", "Processing", "Whether to use ballistic method for the positional update of vehicles (default is a semi-implicit Euler method).");
            oc.do_register("extrapolate-departpos", SumoOption::bool(false));
            oc.add_description("extrapolate-departpos", "Processing", "Whether vehicles that depart between simulation steps should extrapolate the depart position");
            oc.do_register("threads", SumoOption::integer(1));
            oc.add_description("threads", "Processing", "Defines the number of threads for parallel simulation");
            oc.do_register("lateral-resolution", SumoOption::float(-1.0));
            oc.add_description("lateral-resolution", "Processing", "Defines the resolution in m when handling lateral positioning within a lane (with -1 all vehicles drive at the center of their lane");
            oc.do_register_abbr("route-steps", 's', SumoOption::string_typed("200", "TIME"));
            oc.add_description("route-steps", "Processing", "Load routes for the next number of seconds ahead");
            oc.do_register("no-internal-links", SumoOption::bool(false));
            oc.add_description("no-internal-links", "Processing", "Disable (junction) internal links");
            oc.do_register("ignore-junction-blocker", SumoOption::string_typed("-1", "TIME"));
            oc.add_description("ignore-junction-blocker", "Processing", "Ignore vehicles which block the junction after they have been standing for SECONDS (-1 means never ignore)");
            oc.do_register("ignore-route-errors", SumoOption::bool(false));
            oc.add_description("ignore-route-errors", "Processing", "Do not check whether routes are connected");
            oc.do_register("ignore-accidents", SumoOption::bool(false));
            oc.add_description("ignore-accidents", "Processing", "Do not check whether accidents occur");
            oc.do_register("collision.action", SumoOption::string_with("teleport"));
            oc.add_description("collision.action", "Processing", "How to deal with collisions: [none,warn,teleport,remove]");
            oc.do_register("intermodal-collision.action", SumoOption::string_with("warn"));
            oc.add_description("intermodal-collision.action", "Processing", "How to deal with collisions between vehicle and pedestrian: [none,warn,teleport,remove]");
            oc.do_register("collision.stoptime", SumoOption::string_typed("0", "TIME"));
            oc.add_description("collision.stoptime", "Processing", "Let vehicle stop for TIME before performing collision.action (except for action 'none')");
            oc.do_register("intermodal-collision.stoptime", SumoOption::string_typed("0", "TIME"));
            oc.add_description("intermodal-collision.stoptime", "Processing", "Let vehicle stop for TIME before performing intermodal-collision.action (except for action 'none')");
            oc.do_register("collision.check-junctions", SumoOption::bool(false));
            oc.add_description("collision.check-junctions", "Processing", "Enables collisions checks on junctions");
            oc.do_register("collision.check-junctions.mingap", SumoOption::float(0.0));
            oc.add_description("collision.check-junctions.mingap", "Processing", "Increase or decrease sensitivity for junction collision check");
            oc.do_register("collision.mingap-factor", SumoOption::float(-1.0));
            oc.add_description("collision.mingap-factor", "Processing", "Sets the fraction of minGap that must be maintained to avoid collision detection. If a negative value is given, the carFollowModel parameter is used");
            oc.do_register("keep-after-arrival", SumoOption::string_typed("0", "TIME"));
            oc.add_description("keep-after-arrival", "Processing", "After a vehicle arrives, keep it in memory for the given TIME (for TraCI access)");
            oc.do_register("max-num-vehicles", SumoOption::integer(-1));
            oc.add_description("max-num-vehicles", "Processing", "Delay vehicle insertion to stay within the given maximum number");
            oc.do_register("max-num-persons", SumoOption::integer(-1));
            oc.add_description("max-num-persons", "Processing", "Delay person insertion to stay within the given maximum number");
            oc.do_register("max-num-teleports", SumoOption::integer(-1));
            oc.add_description("max-num-teleports", "Processing", "Abort the simulation if the given maximum number of teleports is exceeded");
            oc.do_register("scale", SumoOption::float(1.0));
            oc.add_description("scale", "Processing", "Scale demand by the given factor (by discarding or duplicating vehicles)");
            oc.do_register("scale-suffix", SumoOption::string_with("."));
            oc.add_description("scale-suffix", "Processing", "Suffix to be added when creating ids for cloned vehicles");
            oc.do_register("time-to-teleport", SumoOption::string_typed("300", "TIME"));
            oc.add_description("time-to-teleport", "Processing", "Specify how long a vehicle may wait until being teleported, defaults to 300, non-positive values disable teleporting");
            oc.do_register("time-to-teleport.highways", SumoOption::string_typed("0", "TIME"));
            oc.add_description("time-to-teleport.highways", "Processing", "The waiting time after which vehicles on a fast road (speed > 69km/h) are teleported if they are on a non-continuing lane");
            oc.do_register("time-to-teleport.highways.min-speed", SumoOption::float(69.0 / 3.6));
            oc.add_description("time-to-teleport.highways.min-speed", "Processing", "The waiting time after which vehicles on a fast road (default: speed > 69km/h) are teleported if they are on a non-continuing lane");
            oc.do_register("time-to-teleport.disconnected", SumoOption::string_typed("-1", "TIME"));
            oc.add_description("time-to-teleport.disconnected", "Processing", "The waiting time after which vehicles with a disconnected route are teleported. Negative values disable teleporting");
            oc.do_register("time-to-teleport.remove", SumoOption::bool(false));
            oc.add_description("time-to-teleport.remove", "Processing", "Whether vehicles shall be removed after waiting too long instead of being teleported");
            oc.do_register("time-to-teleport.remove-constraint", SumoOption::bool(false));
            oc.add_description("time-to-teleport.remove-constraint", "Processing", "Whether rail-signal-constraint based deadlocks shall be cleared by removing a constraint");
            oc.do_register("time-to-teleport.ride", SumoOption::string_typed("-1", "TIME"));
            oc.add_description("time-to-teleport.ride", "Processing", "The waiting time after which persons / containers waiting for a pickup are teleported. Negative values disable teleporting");
            oc.do_register("time-to-teleport.bidi", SumoOption::string_typed("-1", "TIME"));
            oc.add_description("time-to-teleport.bidi", "Processing", "The waiting time after which vehicles on bidirectional edges are teleported");
            oc.do_register("time-to-teleport.railsignal-deadlock", SumoOption::string_typed("-1", "TIME"));
            oc.add_description("time-to-teleport.railsignal-deadlock", "Processing", "The waiting time after which vehicles in a rail-signal based deadlock are teleported");
            oc.do_register("waiting-time-memory", SumoOption::string_typed("100", "TIME"));
            oc.add_description("waiting-time-memory", "Processing", "Length of time interval, over which accumulated waiting time is taken into account (default is 100s.)");
            oc.do_register("startup-wait-threshold", SumoOption::string_typed("2", "TIME"));
            oc.add_description("startup-wait-threshold", "Processing", "Minimum consecutive waiting time before applying startupDelay");
            oc.do_register("max-depart-delay", SumoOption::string_typed("-1", "TIME"));
            oc.add_description("max-depart-delay", "Processing", "How long vehicles wait for departure before being skipped, defaults to -1 which means vehicles are never skipped");
            oc.do_register("sloppy-insert", SumoOption::bool(false));
            oc.add_description("sloppy-insert", "Processing", "Whether insertion on an edge shall not be repeated in same step once failed");
            oc.do_register("eager-insert", SumoOption::bool(false));
            oc.add_description("eager-insert", "Processing", "Whether each vehicle is checked separately for insertion on an edge");
            oc.do_register("emergency-insert", SumoOption::bool(false));
            oc.add_description("emergency-insert", "Processing", "Allow inserting a vehicle in a situation which requires emergency braking");
            oc.do_register("insertion-checks", SumoOption::string_with("all"));
            oc.add_description("insertion-checks", "Processing", "Override default value for vehicle attribute insertionChecks");
            oc.do_register("random-depart-offset", SumoOption::string_typed("0", "TIME"));
            oc.add_description("random-depart-offset", "Processing", "Each vehicle receives a random offset to its depart value drawn uniformly from [0, TIME]");
            oc.do_register("lanechange.duration", SumoOption::string_typed("0", "TIME"));
            oc.add_description("lanechange.duration", "Processing", "Duration of a lane change maneuver (default 0)");
            oc.do_register("lanechange.overtake-right", SumoOption::bool(false));
            oc.add_description("lanechange.overtake-right", "Processing", "Whether overtaking on the right on motorways is permitted");
            oc.do_register("tls.all-off", SumoOption::bool(false));
            oc.add_description("tls.all-off", "Processing", "Switches off all traffic lights.");
            oc.do_register("tls.actuated.show-detectors", SumoOption::bool(false));
            oc.add_description("tls.actuated.show-detectors", "Processing", "Sets default visibility for actuation detectors");
            oc.do_register("tls.actuated.jam-threshold", SumoOption::float(-1.0));
            oc.add_description("tls.actuated.jam-threshold", "Processing", "Sets default jam-threshold parameter for all actuation detectors");
            oc.do_register("tls.actuated.detector-length", SumoOption::float(0.0));
            oc.add_description("tls.actuated.detector-length", "Processing", "Sets default detector length parameter for all actuation detectors");
            oc.do_register("tls.delay_based.detector-range", SumoOption::float(100.0));
            oc.add_description("tls.delay_based.detector-range", "Processing", "Sets default range for detecting delayed vehicles");
            oc.do_register("tls.yellow.min-decel", SumoOption::float(3.0));
            oc.add_description("tls.yellow.min-decel", "Processing", "Minimum deceleration when braking at yellow");
            oc.do_register("railsignal-moving-block", SumoOption::bool(false));
            oc.add_description("railsignal-moving-block", "Processing", "Let railsignals operate in moving-block mode by default");
            oc.add_synonyme("railsignal-moving-block", "railsignal.moving-block", false);
            oc.do_register("railsignal.moving-block.default-classes", SumoOption::string_vector_with(vec!["tram".to_string(), "cable_car".to_string()]));
            oc.add_description("railsignal.moving-block.default-classes", "Processing", "List vehicle classes that default to moving-block operations");
            oc.do_register("railsignal.moving-block.max-dist", SumoOption::float(200.0));
            oc.add_description("railsignal.moving-block.max-dist", "Processing", "Maximum signal distance above which zipper conflicts are ignored");
            oc.do_register("railsignal.max-block-length", SumoOption::float(2e4));
            oc.add_description("railsignal.max-block-length", "Processing", "Do not build blocks longer than FLOAT and issue a warning instead");
            oc.do_register("railsignal.default-classes", SumoOption::string_vector_with(vec!["rail".to_string(), "rail_fast".to_string(), "rail_electric".to_string(), "rail_urban".to_string(), "subway".to_string()]));
            oc.add_description("railsignal.default-classes", "Processing", "List vehicle classes that uses block-based insertion checks even when the network has no rail signals for them");
            oc.do_register("slope-centered", SumoOption::bool(false));
            oc.add_description("slope-centered", "Processing", "Compute slope at the vehicle center of mass instead of integrating over front and back");
            oc.do_register("time-to-impatience", SumoOption::string_typed("180", "TIME"));
            oc.add_description("time-to-impatience", "Processing", "Specify how long a vehicle may wait until impatience grows from 0 to 1, defaults to 300, non-positive values disable impatience growth");
            oc.do_register("default.departspeed", SumoOption::string_with("avg"));
            oc.add_description("default.departspeed", "Processing", "Select default depart speed");
            oc.do_register("default.departlane", SumoOption::string_with("best_prob"));
            oc.add_description("default.departlane", "Processing", "Select default depart lane");
            oc.do_register("default.action-step-length", SumoOption::float(0.0));
            oc.add_description("default.action-step-length", "Processing", "Length of the default interval length between action points for the car-following and lane-change models (in seconds). If not specified, the simulation step-length is used per default. Vehicle- or VType-specific settings override the default. Must be a multiple of the simulation step-length.");
            oc.do_register("default.carfollowmodel", SumoOption::string_with("Krauss"));
            oc.add_description("default.carfollowmodel", "Processing", "Select default car following model (Krauss, IDM, ...)");
            oc.add_synonyme("default.carfollowmodel", "carfollow.model", false);
            oc.do_register("default.speeddev", SumoOption::float(-1.0));
            oc.add_description("default.speeddev", "Processing", "Select default speed deviation. A negative value implies vClass specific defaults (0.1 for the default passenger class)");
            oc.do_register("default.emergencydecel", SumoOption::string_with("default"));
            oc.add_description("default.emergencydecel", "Processing", "Select default emergencyDecel value among ('decel', 'default', FLOAT) which sets the value either to the same as the deceleration value, a vClass-class specific default or the given FLOAT in m/s^2");
            oc.do_register("overhead-wire.solver", SumoOption::bool(true));
            oc.add_description("overhead-wire.solver", "Processing", "Use Kirchhoff's laws for solving overhead wire circuit");
            oc.do_register("overhead-wire.recuperation", SumoOption::bool(true));
            oc.add_description("overhead-wire.recuperation", "Processing", "Enable recuperation from the vehicle equipped with elecHybrid device into the overhead wire.");
            oc.do_register("overhead-wire.substation-current-limits", SumoOption::bool(true));
            oc.add_description("overhead-wire.substation-current-limits", "Processing", "Enable current limits of traction substation during solving the overhead wire electrical circuit.");
            oc.do_register("emergencydecel.warning-threshold", SumoOption::float(1.0));
            oc.add_description("emergencydecel.warning-threshold", "Processing", "Sets the fraction of emergency decel capability that must be used to trigger a warning.");
            oc.do_register("parking.maneuver", SumoOption::bool(false));
            oc.add_description("parking.maneuver", "Processing", "Whether parking simulation includes maneuvering time and associated lane blocking");
            oc.do_register("use-stop-ended", SumoOption::bool(false));
            oc.add_description("use-stop-ended", "Processing", "Override stop until times with stop ended times when given");
            oc.do_register("use-stop-started", SumoOption::bool(false));
            oc.add_description("use-stop-started", "Processing", "Override stop arrival times with stop started times when given");
            oc.do_register("pedestrian.model", SumoOption::string_with("striping"));
            oc.add_description("pedestrian.model", "Processing", "Select among pedestrian models ['nonInteracting', 'striping', 'jupedsim', 'remote']");
            oc.do_register("pedestrian.timegap-crossing", SumoOption::float(2.0));
            oc.add_description("pedestrian.timegap-crossing", "Processing", "Minimal acceptable gap (in seconds) between two vehicles before starting to cross");
            oc.do_register("pedestrian.striping.stripe-width", SumoOption::float(0.64));
            oc.add_description("pedestrian.striping.stripe-width", "Processing", "Width of parallel stripes for segmenting a sidewalk (meters) for use with model 'striping'");
            oc.do_register("pedestrian.striping.dawdling", SumoOption::float(0.2));
            oc.add_description("pedestrian.striping.dawdling", "Processing", "Factor for random slow-downs [0,1] for use with model 'striping'");
            oc.do_register("pedestrian.striping.mingap-to-vehicle", SumoOption::float(0.25));
            oc.add_description("pedestrian.striping.mingap-to-vehicle", "Processing", "Minimal gap / safety buffer (in meters) from a pedestrian to another vehicle for use with model 'striping'");
            oc.do_register("pedestrian.striping.jamtime", SumoOption::string_typed("300", "TIME"));
            oc.add_description("pedestrian.striping.jamtime", "Processing", "Time in seconds after which pedestrians start squeezing through a jam when using model 'striping' (non-positive values disable squeezing)");
            oc.do_register("pedestrian.striping.jamtime.crossing", SumoOption::string_typed("10", "TIME"));
            oc.add_description("pedestrian.striping.jamtime.crossing", "Processing", "Time in seconds after which pedestrians start squeezing through a jam while on a pedestrian crossing when using model 'striping' (non-positive values disable squeezing)");
            oc.do_register("pedestrian.striping.jamtime.narrow", SumoOption::string_typed("1", "TIME"));
            oc.add_description("pedestrian.striping.jamtime.narrow", "Processing", "Time in seconds after which pedestrians start squeezing through a jam while on a narrow lane when using model 'striping'");
            oc.do_register("pedestrian.striping.jamfactor", SumoOption::float(0.25));
            oc.add_description("pedestrian.striping.jamfactor", "Processing", "Factor for reducing speed of pedestrian in jammed state");
            oc.do_register("pedestrian.striping.reserve-oncoming", SumoOption::float(0.0));
            oc.add_description("pedestrian.striping.reserve-oncoming", "Processing", "Fraction of stripes to reserve for oncoming pedestrians");
            oc.do_register("pedestrian.striping.reserve-oncoming.junctions", SumoOption::float(0.34));
            oc.add_description("pedestrian.striping.reserve-oncoming.junctions", "Processing", "Fraction of stripes to reserve for oncoming pedestrians on crossings and walkingareas");
            oc.do_register("pedestrian.striping.reserve-oncoming.max", SumoOption::float(1.28));
            oc.add_description("pedestrian.striping.reserve-oncoming.max", "Processing", "Maximum width in m to reserve for oncoming pedestrians");
            oc.do_register("pedestrian.striping.legacy-departposlat", SumoOption::bool(false));
            oc.add_description("pedestrian.striping.legacy-departposlat", "Processing", "Interpret departPosLat for walks in legacy style");
            oc.do_register("pedestrian.striping.walkingarea-detail", SumoOption::integer(4));
            oc.add_description("pedestrian.striping.walkingarea-detail", "Processing", "Generate INT intermediate points to smooth out lanes within the walkingarea");
            oc.do_register("pedestrian.jupedsim.step-length", SumoOption::string_typed("0.01", "TIME"));
            oc.add_description("pedestrian.jupedsim.step-length", "Processing", "The update interval of the JuPedSim simulation (in seconds)");
            oc.do_register("pedestrian.jupedsim.exit-tolerance", SumoOption::float(1.0));
            oc.add_description("pedestrian.jupedsim.exit-tolerance", "Processing", "The distance to accept the JuPedSim arrival point (in meters)");
            oc.do_register("pedestrian.jupedsim.model", SumoOption::string_with("CollisionFreeSpeed"));
            oc.add_description("pedestrian.jupedsim.model", "Processing", "The submodel to use in JuPedSim ('CollisionFreeSpeed', 'CollisionFreeSpeedV2', 'GeneralizedCentrifugalForce', 'SocialForce')");
            oc.do_register("pedestrian.jupedsim.strength-neighbor-repulsion", SumoOption::float(8.0));
            oc.add_description("pedestrian.jupedsim.strength-neighbor-repulsion", "Processing", "The neighbor repulsion strength of the JuPedSim model");
            oc.do_register("pedestrian.jupedsim.range-neighbor-repulsion", SumoOption::float(0.1));
            oc.add_description("pedestrian.jupedsim.range-neighbor-repulsion", "Processing", "The neighbor repulsion range of the JuPedSim model (in meters)");
            oc.do_register("pedestrian.jupedsim.strength-geometry-repulsion", SumoOption::float(5.0));
            oc.add_description("pedestrian.jupedsim.strength-geometry-repulsion", "Processing", "The geometry repulsion strength of the JuPedSim model");
            oc.do_register("pedestrian.jupedsim.range-geometry-repulsion", SumoOption::float(0.02));
            oc.add_description("pedestrian.jupedsim.range-geometry-repulsion", "Processing", "The geometry repulsion range of the JuPedSim model (in meters)");
            oc.do_register("pedestrian.jupedsim.wkt", SumoOption::file_name());
            oc.add_description("pedestrian.jupedsim.wkt", "Output", "The filename to output the JuPedSim network as WKT");
            oc.do_register("pedestrian.jupedsim.wkt.geo", SumoOption::bool(false));
            oc.add_description("pedestrian.jupedsim.wkt.geo", "Output", "Whether to output JuPedSim network as WKT using geo-coordinates (lon/lat)");
            oc.do_register("pedestrian.jupedsim.py", SumoOption::file_name());
            oc.add_description("pedestrian.jupedsim.py", "Output", "The filename to output the JuPedSim setup as Python script");
            oc.do_register("ride.stop-tolerance", SumoOption::float(10.0));
            oc.add_description("ride.stop-tolerance", "Processing", "Tolerance to apply when matching pedestrian and vehicle positions on boarding at individual stops");
            oc.do_register("mapmatch.distance", SumoOption::float(100.0));
            oc.add_description("mapmatch.distance", "Processing", "Maximum distance when mapping input coordinates (fromXY etc.) to the road network");
            oc.do_register("mapmatch.junctions", SumoOption::bool(false));
            oc.add_description("mapmatch.junctions", "Processing", "Match positions to junctions instead of edges");
            oc.do_register("mapmatch.taz", SumoOption::bool(false));
            oc.add_description("mapmatch.taz", "Processing", "Match positions to taz instead of edges");
            oc.do_register("routing-algorithm", SumoOption::string_with("dijkstra"));
            oc.add_description("routing-algorithm", "Routing", "Select among routing algorithms ['dijkstra', 'astar', 'CH', 'CHWrapper', 'CCH']");
            oc.do_register("weights.random-factor", SumoOption::float(1.0));
            oc.add_description("weights.random-factor", "Routing", "Edge weights for routing are dynamically disturbed by a random factor drawn uniformly from [1,FLOAT)");
            oc.do_register("weights.random-factor.dynamic", SumoOption::bool(false));
            oc.add_description("weights.random-factor.dynamic", "Routing", "When using option --weights.random-factor, vary the randomness over time");
            oc.do_register("weights.minor-penalty", SumoOption::float(1.5));
            oc.add_description("weights.minor-penalty", "Routing", "Apply the given time penalty when computing minimum routing costs for minor-link internal lanes");
            oc.do_register("weights.tls-penalty", SumoOption::float(0.0));
            oc.add_description("weights.tls-penalty", "Routing", "Apply scaled travel time penalties based on green split when computing minimum routing costs for internal lanes at traffic lights");
            oc.do_register("weights.turnaround-penalty", SumoOption::float(5.0));
            oc.add_description("weights.turnaround-penalty", "Processing", "Apply the given time penalty when computing routing costs for turnaround internal lanes");
            oc.do_register("weights.reversal-penalty", SumoOption::float(60.0));
            oc.add_description("weights.reversal-penalty", "Processing", "Apply the given time penalty when computing routing costs for train reversal. Negative values disable reversal");
            oc.do_register("weights.priority-factor", SumoOption::float(0.0));
            oc.add_description("weights.priority-factor", "Routing", "Consider edge priorities in addition to travel times, weighted by factor");
            oc.do_register("weights.separate-turns", SumoOption::float(0.0));
            oc.add_description("weights.separate-turns", "Routing", "Distinguish travel time by turn direction and shift a fraction of the estimated time loss ahead of the intersection onto the internal edges");
            oc.do_register("astar.all-distances", SumoOption::file_name());
            oc.add_description("astar.all-distances", "Routing", "Initialize lookup table for astar from the given file (generated by marouter --all-pairs-output)");
            oc.do_register("astar.landmark-distances", SumoOption::file_name());
            oc.add_description("astar.landmark-distances", "Routing", "Initialize lookup table for astar ALT-variant from the given file");
            oc.do_register("persontrip.walkfactor", SumoOption::float(0.75));
            oc.add_description("persontrip.walkfactor", "Routing", "Use FLOAT as a factor on pedestrian maximum speed during intermodal routing");
            oc.do_register("persontrip.walk-opposite-factor", SumoOption::float(1.0));
            oc.add_description("persontrip.walk-opposite-factor", "Processing", "Use FLOAT as a factor on walking speed against vehicle traffic direction");
            oc.do_register("persontrip.transfer.car-walk", SumoOption::string_vector_with(vec!["parkingAreas".to_string()]));
            oc.add_description("persontrip.transfer.car-walk", "Routing", "Where are mode changes from car to walking allowed (possible values: 'parkingAreas', 'ptStops', 'allJunctions' and combinations)");
            oc.do_register("persontrip.transfer.taxi-walk", SumoOption::string_vector());
            oc.add_description("persontrip.transfer.taxi-walk", "Routing", "Where taxis can drop off customers ('allJunctions, 'ptStops', 'parkingAreas')");
            oc.do_register("persontrip.transfer.walk-taxi", SumoOption::string_vector());
            oc.add_description("persontrip.transfer.walk-taxi", "Routing", "Where taxis can pick up customers ('allJunctions, 'ptStops', 'parkingAreas')");
            oc.do_register("persontrip.default.group", SumoOption::string());
            oc.add_description("persontrip.default.group", "Routing", "When set, trips between the same origin and destination will share a taxi by default");
            oc.do_register("persontrip.taxi.waiting-time", SumoOption::string_typed("300", "TIME"));
            oc.add_description("persontrip.taxi.waiting-time", "Routing", "Estimated time for taxi pickup");
            oc.do_register("persontrip.ride-public-line", SumoOption::bool(false));
            oc.add_description("persontrip.ride-public-line", "Routing", "Only use the intended public transport line rather than any alternative line that stops at the destination");
            oc.do_register("railway.max-train-length", SumoOption::float(1000.0));
            oc.add_description("railway.max-train-length", "Routing", "Use FLOAT as a maximum train length when initializing the railway router");
            oc.do_register("replay-rerouting", SumoOption::bool(false));
            oc.add_description("replay-rerouting", "Routing", "Replay exact rerouting sequence from vehroute-output");
            oc.add_option_sub_topic("Emissions");
            oc.do_register("emissions.volumetric-fuel", SumoOption::bool(false));
            oc.add_description("emissions.volumetric-fuel", "Emissions", "Return fuel consumption values in (legacy) unit l instead of mg");
            oc.do_register("phemlight-path", SumoOption::file_name_with(vec!["./PHEMlight/".to_string()]));
            oc.add_description("phemlight-path", "Emissions", "Determines where to load PHEMlight definitions from");
            oc.do_register("phemlight-year", SumoOption::integer(0));
            oc.add_description("phemlight-year", "Emissions", "Enable fleet age modelling with the given reference year in PHEMlight5");
            oc.do_register("phemlight-temperature", SumoOption::float(f64::MAX));
            oc.add_description("phemlight-temperature", "Emissions", "Set ambient temperature to correct NOx emissions in PHEMlight5");
            oc.add_option_sub_topic("Communication");
            oc.add_option_sub_topic("Battery");
            crate::microsim::devices::ms_device::MSDevice::insert_options(oc);
            oc.do_register("duration-log.disable", SumoOption::bool(false));
            oc.add_synonyme("duration-log.disable", "no-duration-log", false);
            oc.add_description("duration-log.disable", "Report", "Disable performance reports for individual simulation steps");
            oc.do_register_abbr("duration-log.statistics", 't', SumoOption::bool(false));
            oc.add_description("duration-log.statistics", "Report", "Enable statistics on vehicle trips");
            oc.do_register("no-step-log", SumoOption::bool(false));
            oc.add_description("no-step-log", "Report", "Disable console output of current simulation step");
            oc.do_register("step-log.period", SumoOption::integer(100));
            oc.add_description("step-log.period", "Report", "Number of simulation steps between step-log outputs");
            oc.add_option_sub_topic("TraCI Server");
            oc.do_register("remote-port", SumoOption::integer(0));
            oc.add_description("remote-port", "TraCI Server", "Enables TraCI Server if set");
            oc.do_register("num-clients", SumoOption::integer(1));
            oc.add_description("num-clients", "TraCI Server", "Expected number of connecting clients");
            oc.add_option_sub_topic("Mesoscopic");
            oc.do_register("mesosim", SumoOption::bool(false));
            oc.add_description("mesosim", "Mesoscopic", "Enables mesoscopic simulation");
            oc.do_register("meso-ltm", SumoOption::bool(false));
            oc.add_description("meso-ltm", "Mesoscopic", "Enables the meso-LTM (LIFT) model");
            oc.do_register("meso-edgelength", SumoOption::float(98.0));
            oc.add_description("meso-edgelength", "Mesoscopic", "Length of an edge segment in mesoscopic simulation");
            oc.do_register("meso-tauff", SumoOption::string_typed("1.13", "TIME"));
            oc.add_description("meso-tauff", "Mesoscopic", "Factor for calculating the net free-free headway time");
            oc.do_register("meso-taufj", SumoOption::string_typed("1.13", "TIME"));
            oc.add_description("meso-taufj", "Mesoscopic", "Factor for calculating the net free-jam headway time");
            oc.do_register("meso-taujf", SumoOption::string_typed("1.73", "TIME"));
            oc.add_description("meso-taujf", "Mesoscopic", "Factor for calculating the jam-free headway time");
            oc.do_register("meso-taujj", SumoOption::string_typed("1.4", "TIME"));
            oc.add_description("meso-taujj", "Mesoscopic", "Factor for calculating the jam-jam headway time");
            oc.do_register("meso-jam-threshold", SumoOption::float(-1.0));
            oc.add_description("meso-jam-threshold", "Mesoscopic", "Minimum percentage of occupied space to consider a segment jammed. A negative argument causes thresholds to be computed based on edge speed and tauff (default)");
            oc.do_register("meso-multi-queue", SumoOption::bool(true));
            oc.add_description("meso-multi-queue", "Mesoscopic", "Enable multiple queues at edge ends");
            oc.do_register("meso-lane-queue", SumoOption::bool(false));
            oc.add_description("meso-lane-queue", "Mesoscopic", "Enable separate queues for every lane");
            oc.do_register("meso-ignore-lanes-by-vclass", SumoOption::string_vector_with(vec!["pedestrian".to_string(), "bicycle".to_string()]));
            oc.add_description("meso-ignore-lanes-by-vclass", "Mesoscopic", "Do not build queues (or reduce capacity) for lanes allowing only the given vclasses");
            oc.add_synonyme("meso-ignore-lanes-by-vclass", "meso.ignore-lanes.by-vclass", false);
            oc.do_register("meso-junction-control", SumoOption::bool(false));
            oc.add_description("meso-junction-control", "Mesoscopic", "Enable mesoscopic traffic light and priority junction handling");
            oc.do_register("meso-junction-control.limited", SumoOption::bool(false));
            oc.add_description("meso-junction-control.limited", "Mesoscopic", "Enable mesoscopic traffic light and priority junction handling for saturated links. This prevents faulty traffic lights from hindering flow in low-traffic situations");
            oc.do_register("meso-tls-penalty", SumoOption::float(0.0));
            oc.add_description("meso-tls-penalty", "Mesoscopic", "Apply scaled travel time penalties when driving across tls controlled junctions based on green split instead of checking actual phases");
            oc.do_register("meso-tls-flow-penalty", SumoOption::float(0.0));
            oc.add_description("meso-tls-flow-penalty", "Mesoscopic", "Apply scaled headway penalties when driving across tls controlled junctions based on green split instead of checking actual phases");
            oc.do_register("meso-minor-penalty", SumoOption::string_typed("0", "TIME"));
            oc.add_description("meso-minor-penalty", "Mesoscopic", "Apply fixed time penalty when driving across a minor link. When using --meso-junction-control.limited, the penalty is not applied whenever limited control is active.");
            oc.do_register("meso-overtaking", SumoOption::bool(false));
            oc.add_description("meso-overtaking", "Mesoscopic", "Enable mesoscopic overtaking");
            oc.do_register("meso-recheck", SumoOption::string_typed("0", "TIME"));
            oc.add_description("meso-recheck", "Mesoscopic", "Time interval for rechecking insertion into the next segment after failure");
            oc.do_register("meso-interpolate-pos", SumoOption::bool(false));
            oc.add_description("meso-interpolate-pos", "Mesoscopic", "Enable mesoscopic position interpolation");
            utils::common::rand_helper::insert_rand_options(oc);
            oc.do_register("thread-rngs", SumoOption::integer(64));
            oc.add_description("thread-rngs", "Random Number", "Number of pre-allocated random number generators to ensure repeatable multi-threaded simulations (should be at least the number of threads for repeatable simulations).");
            oc.add_option_sub_topic("GUI Only");
            oc.do_register_abbr("gui-settings-file", 'g', SumoOption::file_name());
            oc.add_description("gui-settings-file", "GUI Only", "Load visualisation settings from FILE");
            oc.do_register_abbr("quit-on-end", 'Q', SumoOption::bool(false));
            oc.add_description("quit-on-end", "GUI Only", "Quits the GUI when the simulation stops");
            oc.do_register_abbr("game", 'G', SumoOption::bool(false));
            oc.add_description("game", "GUI Only", "Start the GUI in gaming mode");
            oc.do_register("game.mode", SumoOption::string_with("tls"));
            oc.add_description("game.mode", "GUI Only", "Select the game type ('tls', 'drt')");
            oc.do_register_abbr("start", 'S', SumoOption::bool(false));
            oc.add_description("start", "GUI Only", "Start the simulation after loading");
            oc.do_register_abbr("delay", 'd', SumoOption::float(0.0));
            oc.add_description("delay", "GUI Only", "Use FLOAT in ms as delay between simulation steps");
            oc.do_register_abbr("breakpoints", 'B', SumoOption::string_vector());
            oc.add_description("breakpoints", "GUI Only", "Use TIME[] as times when the simulation should halt");
            oc.do_register_abbr("edgedata-files", 'm', SumoOption::file_name());
            oc.add_synonyme("edgedata-files", "data-files", false);
            oc.add_description("edgedata-files", "GUI Only", "Load edge/lane weights for visualization from FILE");
            oc.do_register_abbr("alternative-net-file", 'N', SumoOption::file_name());
            oc.add_description("alternative-net-file", "GUI Only", "Load a secondary road network for abstract visualization from FILE");
            oc.do_register("selection-file", SumoOption::file_name());
            oc.add_description("selection-file", "GUI Only", "Load pre-selected elements from FILE");
            oc.do_register_abbr("demo", 'D', SumoOption::bool(false));
            oc.add_description("demo", "GUI Only", "Restart the simulation after ending (demo mode)");
            oc.do_register_abbr("disable-textures", 'T', SumoOption::bool(false));
            oc.add_description("disable-textures", "GUI Only", "Do not load background pictures");
            oc.do_register("registry-viewport", SumoOption::bool(false));
            oc.add_description("registry-viewport", "GUI Only", "Load current viewport from registry");
            oc.do_register("window-size", SumoOption::string_vector());
            oc.add_description("window-size", "GUI Only", "Create initial window with the given x,y size");
            oc.do_register("window-pos", SumoOption::string_vector());
            oc.add_description("window-pos", "GUI Only", "Create initial window at the given x,y position");
            oc.do_register("tracker-interval", SumoOption::string_typed("1", "TIME"));
            oc.add_description("tracker-interval", "GUI Only", "The aggregation period for value tracker windows");
            // C++ registers osg-view under #ifdef HAVE_OSG and movereminder-output under
            // #ifdef _DEBUG. The scoring build has neither, so both are absent from its
            // --help / --save-template output; do not register them here.
            oc.do_register("gui-testing", SumoOption::bool(false));
            oc.add_description("gui-testing", "GUI Only", "Enable overlay for screen recognition");
            oc.do_register("gui-testing-debug", SumoOption::bool(false));
            oc.add_description("gui-testing-debug", "GUI Only", "Enable output messages during GUI-Testing");
            oc.do_register("gui-testing.setting-output", SumoOption::file_name());
            oc.add_description("gui-testing.setting-output", "GUI Only", "Save gui settings in the given settings output file");
        });
    }

    /// C++: `void MSFrame::buildStreams()` [static]
    /// Builds the streams used possibly by the simulation
    // void
    // MSFrame::buildStreams() {
    //     // standard outputs
    //     if (OutputDevice::createDeviceByOption("netstate-dump", "netstate", "netstate_file.xsd")) {
    //         WRITE_WARNING(TL("Raw netstate dumps are deprecated. Use fcd-output instead."));
    //     }
    //     OutputDevice::createDeviceByOption("summary-output", "summary", "summary_file.xsd");
    //     OutputDevice::createDeviceByOption("person-summary-output", "personSummary", "person_summary_file.xsd");
    //     OutputDevice::createDeviceByOption("tripinfo-output", "tripinfos", "tripinfo_file.xsd");
    //
    //     //extended
    //     OutputDevice::createDeviceByOption("fcd-output", "fcd-export", "fcd_file.xsd");
    //     OutputDevice::createDeviceByOption("person-fcd-output", "fcd-export", "fcd_file.xsd");
    //     OutputDevice::createDeviceByOption("emission-output", "emission-export", "emission_file.xsd");
    //     OutputDevice::createDeviceByOption("battery-output", "battery-export", "battery_file.xsd");
    //     if (OptionsCont::getOptions().getBool("elechybrid-output.aggregated")) {
    //         // RICE_TODO: Add path to elechybrid-output.aggregated xsd file
    //         OutputDevice::createDeviceByOption("elechybrid-output", "elecHybrid-export-aggregated", "\" recuperationEnabled=\"" + toString(MSGlobals::gOverheadWireRecuperation));
    //     }
    //     //OutputDevice::createDeviceByOption("elecHybrid-output", "elecHybrid-export");
    //     OutputDevice::createDeviceByOption("chargingstations-output", "chargingstations-export");
    //     OutputDevice::createDeviceByOption("overheadwiresegments-output", "overheadWireSegments-export");
    //     OutputDevice::createDeviceByOption("substations-output", "substations-export");
    //     OutputDevice::createDeviceByOption("full-output", "full-export", "full_file.xsd");
    //     OutputDevice::createDeviceByOption("queue-output", "queue-export", "queue_file.xsd", 3);
    //     OutputDevice::createDeviceByOption("amitran-output", "trajectories", "amitran/trajectories.xsd\" timeStepSize=\"" + toString(STEPS2MS(DELTA_T)));
    //
    //     //OutputDevice::createDeviceByOption("vtk-output", "vtk-export");
    //     OutputDevice::createDeviceByOption("link-output", "link-output");
    //     OutputDevice::createDeviceByOption("railsignal-block-output", "railsignal-block-output");
    //     OutputDevice::createDeviceByOption("railsignal-vehicle-output", "railsignal-vehicle-output");
    //     OutputDevice::createDeviceByOption("bt-output", "bt-output");
    //     OutputDevice::createDeviceByOption("lanechange-output", "lanechanges");
    //     OutputDevice::createDeviceByOption("stop-output", "stops", "stopinfo_file.xsd");
    //     OutputDevice::createDeviceByOption("collision-output", "collisions", "collision_file.xsd");
    //     OutputDevice::createDeviceByOption("statistic-output", "statistics", "statistic_file.xsd");
    //     OutputDevice::createDeviceByOption("deadlock-output", "additional", "additional_file.xsd");
    //
    // #ifdef _DEBUG
    //     OutputDevice::createDeviceByOption("movereminder-output", "movereminder-output");
    // #endif
    //
    //     MSDevice_Vehroutes::init();
    //     MSStopOut::init();
    // }
    #[allow(unused_variables)]
    pub fn build_streams() {
        // C++ OutputDevice::createDeviceByOption defaults: rootElement "", schemaFile "", maximumDepth 2
        let create = |option: &str, root: &str, schema: &str, depth: i32| -> bool {
            utils::iodevices::output_device::OutputDevice::create_device_by_option(option, root, schema, depth).unwrap_or_else(|e| panic!("{e}"))
        };
        // standard outputs
        if create("netstate-dump", "netstate", "netstate_file.xsd", 2) {
            utils::common::msg_handler::write_warning("Raw netstate dumps are deprecated. Use fcd-output instead.");
        }
        create("summary-output", "summary", "summary_file.xsd", 2);
        create("person-summary-output", "personSummary", "person_summary_file.xsd", 2);
        create("tripinfo-output", "tripinfos", "tripinfo_file.xsd", 2);

        //extended
        create("fcd-output", "fcd-export", "fcd_file.xsd", 2);
        create("person-fcd-output", "fcd-export", "fcd_file.xsd", 2);
        create("emission-output", "emission-export", "emission_file.xsd", 2);
        create("battery-output", "battery-export", "battery_file.xsd", 2);
        if utils::options::options_cont::get_bool("elechybrid-output.aggregated") {
            // RICE_TODO: Add path to elechybrid-output.aggregated xsd file
            create(
                "elechybrid-output",
                "elecHybrid-export-aggregated",
                &format!("\" recuperationEnabled=\"{}", utils::common::to_string::to_string(&crate::microsim::ms_globals::overhead_wire_recuperation())),
                2,
            );
        }
        //OutputDevice::createDeviceByOption("elecHybrid-output", "elecHybrid-export");
        create("chargingstations-output", "chargingstations-export", "", 2);
        create("overheadwiresegments-output", "overheadWireSegments-export", "", 2);
        create("substations-output", "substations-export", "", 2);
        create("full-output", "full-export", "full_file.xsd", 2);
        create("queue-output", "queue-export", "queue_file.xsd", 3);
        create(
            "amitran-output",
            "trajectories",
            &format!("amitran/trajectories.xsd\" timeStepSize=\"{}", utils::common::to_string::to_string(&utils::common::time::delta_t())),
            2,
        );

        //OutputDevice::createDeviceByOption("vtk-output", "vtk-export");
        create("link-output", "link-output", "", 2);
        create("railsignal-block-output", "railsignal-block-output", "", 2);
        create("railsignal-vehicle-output", "railsignal-vehicle-output", "", 2);
        create("bt-output", "bt-output", "", 2);
        create("lanechange-output", "lanechanges", "", 2);
        create("stop-output", "stops", "stopinfo_file.xsd", 2);
        create("collision-output", "collisions", "collision_file.xsd", 2);
        create("statistic-output", "statistics", "statistic_file.xsd", 2);
        create("deadlock-output", "additional", "additional_file.xsd", 2);
        // C++ _DEBUG-only: compiled out of the release reference build, not ported (movereminder-output device)

        crate::microsim::devices::ms_device_vehroutes::MSDevice_Vehroutes::init();
        crate::microsim::output::ms_stop_out::MSStopOut::init();
    }

    /// C++: `bool MSFrame::checkOptions()` [static]
    /// Checks the set options.
    // bool
    // MSFrame::checkOptions() {
    //     OptionsCont& oc = OptionsCont::getOptions();
    //     bool ok = true;
    //     if (oc.getBool("ignore.routeelements")) {
    //         oc.resetWritable("route-files");
    //         oc.resetDefault("route-files");
    //     }
    //     if (!oc.isSet("net-file") && oc.isDefault("remote-port")) {
    //         WRITE_ERROR(TL("No network file (-n) specified."));
    //         ok = false;
    //     }
    //     if (oc.getFloat("scale") < 0.) {
    //         WRITE_ERROR(TL("Invalid scaling factor."));
    //         ok = false;
    //     }
    //     if (oc.getBool("vehroute-output.exit-times") && !oc.isSet("vehroute-output")) {
    //         WRITE_ERROR(TL("A vehroute-output file is needed for exit times."));
    //         ok = false;
    //     }
    //     if (oc.isSet("gui-settings-file") &&
    //             oc.getString("gui-settings-file") != "" &&
    //             !oc.isUsableFileList("gui-settings-file")) {
    //         ok = false;
    //     }
    //     if (oc.getBool("demo") && oc.isDefault("start")) {
    //         oc.setDefault("start", "true");
    //     }
    //     if (oc.getBool("demo") && oc.getBool("quit-on-end")) {
    //         WRITE_ERROR(TL("You can either restart or quit on end."));
    //         ok = false;
    //     }
    //     if (oc.getBool("meso-junction-control.limited") && !oc.getBool("meso-junction-control")) {
    //         if (!oc.isDefault("meso-junction-control")) {
    //             WRITE_WARNING(TL("The option 'meso-junction-control.limited' implies 'meso-junction-control'."))
    //         }
    //         oc.setDefault("meso-junction-control", "true");
    //     }
    //     if (oc.getBool("meso-ltm") && oc.isDefault("mesosim")) {
    //         oc.setDefault("mesosim", "true");
    //     }
    //     if (oc.getBool("mesosim")) {
    //         if (oc.isDefault("pedestrian.model")) {
    //             oc.setDefault("pedestrian.model", "nonInteracting");
    //         }
    //     }
    //     if (string2time(oc.getString("device.fcd.begin")) < 0) {
    //         oc.setDefault("device.fcd.begin", oc.getString("begin"));
    //     }
    //     if (string2time(oc.getString("device.emissions.begin")) < 0) {
    //         oc.setDefault("device.emissions.begin", oc.getString("begin"));
    //     }
    //     const SUMOTime begin = string2time(oc.getString("begin"));
    //     const SUMOTime end = string2time(oc.getString("end"));
    //     if (begin < 0) {
    //         WRITE_ERROR(TL("The begin time should not be negative."));
    //         ok = false;
    //     }
    //     // DELTA_T not yet initialized
    //     const SUMOTime deltaT = MAX2((SUMOTime)1, string2time(oc.getString("step-length")));
    //     if (begin < TIME2STEPS(1)) {
    //         checkStepLengthMultiple(begin, " for begin", deltaT);
    //     }
    //     if (end != string2time("-1")) {
    //         if (end < begin) {
    //             WRITE_ERROR(TL("The end time should be after the begin time."));
    //             ok = false;
    //         }
    //     }
    //     if (string2time(oc.getString("step-length")) <= 0) {
    //         WRITE_ERROR(TL("the minimum step-length is 0.001."));
    //         ok = false;
    //     }
    //     const SUMOTime period = string2time(oc.getString("device.fcd.period"));
    //     if (period > 0) {
    //         checkStepLengthMultiple(period, " for device.fcd.period", deltaT);
    //     }
    //     const SUMOTime statePeriod = string2time(oc.getString("save-state.period"));
    //     if (statePeriod > 0) {
    //         checkStepLengthMultiple(statePeriod, " for save-state.period", deltaT);
    //     }
    //     for (const std::string& timeStr : oc.getStringVector("save-state.times")) {
    //         try {
    //             const SUMOTime saveT = string2time(timeStr);
    //             if (end > 0 && saveT >= end) {
    //                 WRITE_WARNINGF(TL("The save-state time=% will not be used before simulation end at %."), timeStr, time2string(end));
    //             } else {
    //                 checkStepLengthMultiple(saveT, " for save-state.times", deltaT, begin);
    //             }
    //         } catch (ProcessError& e) {
    //             WRITE_ERROR("Invalid time '" + timeStr + "' for option 'save-state.times'. " + e.what());
    //             ok = false;
    //         }
    //     }
    //
    // #ifdef _DEBUG
    //     if (oc.isSet("movereminder-output.vehicles") && !oc.isSet("movereminder-output")) {
    //         WRITE_ERROR(TL("option movereminder-output.vehicles requires option movereminder-output to be set"));
    //         ok = false;
    //     }
    // #endif
    //     if (oc.getBool("sloppy-insert")) {
    //         WRITE_WARNING(TL("The option 'sloppy-insert' is deprecated, because it is now activated by default, see the new option 'eager-insert'."));
    //     }
    //     if (string2time(oc.getString("lanechange.duration")) > 0 && oc.getFloat("lateral-resolution") > 0) {
    //         WRITE_ERROR(TL("Only one of the options 'lanechange.duration' or 'lateral-resolution' may be given."));
    //         ok = false;
    //     }
    //     if (oc.getBool("mesosim") && (oc.getFloat("lateral-resolution") > 0 || string2time(oc.getString("lanechange.duration")) > 0)) {
    //         WRITE_ERROR(TL("Sublane dynamics are not supported by mesoscopic simulation."));
    //         ok = false;
    //     }
    //     if (oc.getBool("ignore-accidents")) {
    //         WRITE_WARNING(TL("The option 'ignore-accidents' is deprecated. Use 'collision.action none' instead."));
    //     }
    //     if (oc.getBool("duration-log.statistics") && oc.isDefault("verbose")) {
    //         oc.setDefault("verbose", "true");
    //     }
    //     if (oc.isDefault("precision") && string2time(oc.getString("step-length")) % 10 != 0) {
    //         oc.setDefault("precision", "3");
    //     }
    //     if (oc.isDefault("tracker-interval") && !oc.isDefault("step-length")) {
    //         oc.setDefault("tracker-interval", oc.getString("step-length"));
    //     }
    //     if (oc.getBool("tripinfo-output.write-undeparted")) {
    //         if (!oc.isDefault("tripinfo-output.write-unfinished") && !oc.getBool("tripinfo-output.write-unfinished")) {
    //             WRITE_WARNING(TL("The option tripinfo-output.write-undeparted implies tripinfo-output.write-unfinished."));
    //         }
    //         oc.setDefault("tripinfo-output.write-unfinished", "true");
    //     }
    //     if (oc.getInt("precision") > 2) {
    //         if (oc.isDefault("netstate-dump.precision")) {
    //             oc.setDefault("netstate-dump.precision", toString(oc.getInt("precision")));
    //         }
    //         if (oc.isDefault("emission-output.precision")) {
    //             oc.setDefault("emission-output.precision", toString(oc.getInt("precision")));
    //         }
    //         if (oc.isDefault("battery-output.precision")) {
    //             oc.setDefault("battery-output.precision", toString(oc.getInt("precision")));
    //         }
    //         if (oc.isDefault("elechybrid-output.precision")) {
    //             oc.setDefault("elechybrid-output.precision", toString(oc.getInt("precision")));
    //         }
    //         if (oc.isDefault("substations-output.precision")) {
    //             oc.setDefault("substations-output.precision", toString(oc.getInt("precision")));
    //         }
    //     }
    //     if (!SUMOXMLDefinitions::CarFollowModels.hasString(oc.getString("carfollow.model"))) {
    //         WRITE_ERRORF(TL("Unknown model '%' for option 'carfollow.model'."), oc.getString("carfollow.model"));
    //         ok = false;
    //     }
    //     if (oc.isSet("default.emergencydecel")) {
    //         const std::string val = oc.getString("default.emergencydecel");
    //         if (val != "default" && val != "decel") {
    //             try {
    //                 StringUtils::toDouble(val);
    //             } catch (NumberFormatException&) {
    //                 WRITE_ERRORF(TL("Invalid value '%' for option 'default.emergencydecel'. Must be a FLOAT or 'default' or 'decel'."), val);
    //                 ok = false;
    //             }
    //         }
    //     }
    //
    //     if (oc.isSet("time-to-teleport.railsignal-deadlock")) {
    //         SUMOTime t1 = string2time(oc.getString("time-to-teleport"));
    //         SUMOTime t2 = string2time(oc.getString("time-to-teleport.railsignal-deadlock"));
    //         if (t1 > 0 && t2 > 0 && t1 <= t2) {
    //             WRITE_WARNINGF(TL("Railsignal-deadlock will not be detected because time-to-teleport (%) is lower than time-to-teleport.railsignal-deadlock (%)."), time2string(t1), time2string(t2));
    //         }
    //     }
    //
    //     oc.doRegister("", new Option_String("-1", "TIME"));
    //
    //
    //     if (oc.getFloat("delay") < 0.0) {
    //         WRITE_ERROR(TL("You need a non-negative delay."));
    //         ok = false;
    //     }
    //     for (const std::string& val : oc.getStringVector("breakpoints")) {
    //         try {
    //             string2time(val);
    //         } catch (ProcessError& e) {
    //             WRITE_ERROR("Invalid time '" + val + "' for option 'breakpoints'. " + e.what());
    //             ok = false;
    //         }
    //     }
    // #ifndef HAVE_FOX
    //     if (oc.getInt("threads") > 1) {
    //         WRITE_ERROR(TL("Parallel simulation is only possible when compiled with Fox."));
    //         ok = false;
    //     }
    // #endif
    //     if (oc.getInt("threads") < 1) {
    //         WRITE_ERROR(TL("You need at least one thread."));
    //         ok = false;
    //     }
    //     if (oc.getInt("threads") > oc.getInt("thread-rngs")) {
    //         WRITE_WARNING(TL("Number of threads exceeds number of thread-rngs. Simulation runs with the same seed may produce different results."));
    //     }
    //     if (oc.getString("game.mode") != "tls" && oc.getString("game.mode") != "drt") {
    //         WRITE_ERROR(TL("game.mode must be one of ['tls', 'drt']"));
    //         ok = false;
    //     }
    //
    //     if (oc.isSet("persontrip.transfer.car-walk")) {
    //         for (const std::string& opt : OptionsCont::getOptions().getStringVector("persontrip.transfer.car-walk")) {
    //             if (opt != "parkingAreas" && opt != "ptStops" && opt != "allJunctions") {
    //                 WRITE_ERRORF(TL("Invalid transfer option '%'. Must be one of 'parkingAreas', 'ptStops' or 'allJunctions'."), opt);
    //                 ok = false;
    //             }
    //         }
    //     }
    //     if (!oc.isDefault("weights.random-factor") && (oc.isSet("astar.all-distances") || oc.isSet("astar.landmark-distances"))) {
    //         WRITE_WARNING(TL("The option --weights.random-factor should not be used together with astar and precomputed distances."));
    //     }
    //     if (oc.getInt("threads") > 1) {
    //         WRITE_WARNING(TL("The option --threads has known problems and does NOT provide meaningful speedup at this time (https://github.com/eclipse-sumo/sumo/issues/4767). Using it is not recommended!"));
    //     }
    //     if (oc.getBool("mapmatch.junctions") && oc.isDefault("junction-taz")) {
    //         oc.setDefault("junction-taz", "true");
    //     }
    //
    //     ok &= MSDevice::checkOptions(oc);
    //     ok &= SystemFrame::checkOptions(oc);
    //
    //     return ok;
    // }
    #[allow(unused_variables)]
    pub fn check_options() -> bool {
        utils::options::options_cont::with_options(|oc| {
            let mut ok = true;
            if !opt_is_set(oc, "net-file") && opt_is_default(oc, "remote-port") {
                utils::common::msg_handler::write_error("No network file (-n) specified.");
                ok = false;
            }
            if opt_float(oc, "scale") < 0. {
                utils::common::msg_handler::write_error("Invalid scaling factor.");
                ok = false;
            }
            if opt_bool(oc, "vehroute-output.exit-times") && !opt_is_set(oc, "vehroute-output") {
                utils::common::msg_handler::write_error("A vehroute-output file is needed for exit times.");
                ok = false;
            }
            if opt_is_set(oc, "gui-settings-file") && opt_str(oc, "gui-settings-file") != "" && !or_panic(oc.is_usable_file_list("gui-settings-file")) {
                ok = false;
            }
            if opt_bool(oc, "demo") && opt_is_default(oc, "start") {
                or_panic(oc.set_default("start", "true"));
            }
            if opt_bool(oc, "demo") && opt_bool(oc, "quit-on-end") {
                utils::common::msg_handler::write_error("You can either restart or quit on end.");
                ok = false;
            }
            if opt_bool(oc, "meso-junction-control.limited") && !opt_bool(oc, "meso-junction-control") {
                if !opt_is_default(oc, "meso-junction-control") {
                    utils::common::msg_handler::write_warning("The option 'meso-junction-control.limited' implies 'meso-junction-control'.");
                }
                or_panic(oc.set_default("meso-junction-control", "true"));
            }
            if opt_bool(oc, "meso-ltm") && opt_is_default(oc, "mesosim") {
                or_panic(oc.set_default("mesosim", "true"));
            }
            if opt_bool(oc, "mesosim") && opt_is_default(oc, "pedestrian.model") {
                or_panic(oc.set_default("pedestrian.model", "nonInteracting"));
            }
            if st(&opt_str(oc, "device.fcd.begin")) < 0 {
                let v = opt_str(oc, "begin");
                or_panic(oc.set_default("device.fcd.begin", &v));
            }
            if st(&opt_str(oc, "device.emissions.begin")) < 0 {
                let v = opt_str(oc, "begin");
                or_panic(oc.set_default("device.emissions.begin", &v));
            }
            let begin = st(&opt_str(oc, "begin"));
            let end = st(&opt_str(oc, "end"));
            if begin < 0 {
                utils::common::msg_handler::write_error("The begin time should not be negative.");
                ok = false;
            }
            // DELTA_T not yet initialized
            let deltaT = std::cmp::max(1, st(&opt_str(oc, "step-length")));
            if begin < utils::common::time::time2steps(1.0) {
                check_step_length_multiple(begin, " for begin", deltaT, 0);
            }
            if end != st("-1") && end < begin {
                utils::common::msg_handler::write_error("The end time should be after the begin time.");
                ok = false;
            }
            if st(&opt_str(oc, "step-length")) <= 0 {
                utils::common::msg_handler::write_error("the minimum step-length is 0.001.");
                ok = false;
            }
            let period = st(&opt_str(oc, "device.fcd.period"));
            if period > 0 {
                check_step_length_multiple(period, " for device.fcd.period", deltaT, 0);
            }
            let statePeriod = st(&opt_str(oc, "save-state.period"));
            if statePeriod > 0 {
                check_step_length_multiple(statePeriod, " for save-state.period", deltaT, 0);
            }
            for timeStr in opt_strs(oc, "save-state.times") {
                match utils::common::time::string2time(&timeStr) {
                    Ok(saveT) => {
                        if end > 0 && saveT >= end {
                            utils::common::msg_handler::write_warning(&format!(
                                "The save-state time={} will not be used before simulation end at {}.",
                                timeStr,
                                utils::common::time::time2string(end)
                            ));
                        } else {
                            check_step_length_multiple(saveT, " for save-state.times", deltaT, begin);
                        }
                    }
                    Err(e) => {
                        utils::common::msg_handler::write_error(&format!("Invalid time '{}' for option 'save-state.times'. {}", timeStr, e));
                        ok = false;
                    }
                }
            }
            // C++ _DEBUG-only: compiled out of the release reference build, not ported (movereminder-output check)
            if opt_bool(oc, "sloppy-insert") {
                utils::common::msg_handler::write_warning("The option 'sloppy-insert' is deprecated, because it is now activated by default, see the new option 'eager-insert'.");
            }
            if st(&opt_str(oc, "lanechange.duration")) > 0 && opt_float(oc, "lateral-resolution") > 0. {
                utils::common::msg_handler::write_error("Only one of the options 'lanechange.duration' or 'lateral-resolution' may be given.");
                ok = false;
            }
            if opt_bool(oc, "mesosim") && (opt_float(oc, "lateral-resolution") > 0. || st(&opt_str(oc, "lanechange.duration")) > 0) {
                utils::common::msg_handler::write_error("Sublane dynamics are not supported by mesoscopic simulation.");
                ok = false;
            }
            if opt_bool(oc, "ignore-accidents") {
                utils::common::msg_handler::write_warning("The option 'ignore-accidents' is deprecated. Use 'collision.action none' instead.");
            }
            if opt_bool(oc, "duration-log.statistics") && opt_is_default(oc, "verbose") {
                or_panic(oc.set_default("verbose", "true"));
            }
            if opt_is_default(oc, "precision") && st(&opt_str(oc, "step-length")) % 10 != 0 {
                or_panic(oc.set_default("precision", "3"));
            }
            if opt_is_default(oc, "tracker-interval") && !opt_is_default(oc, "step-length") {
                let v = opt_str(oc, "step-length");
                or_panic(oc.set_default("tracker-interval", &v));
            }
            if opt_bool(oc, "tripinfo-output.write-undeparted") {
                if !opt_is_default(oc, "tripinfo-output.write-unfinished") && !opt_bool(oc, "tripinfo-output.write-unfinished") {
                    utils::common::msg_handler::write_warning("The option tripinfo-output.write-undeparted implies tripinfo-output.write-unfinished.");
                }
                or_panic(oc.set_default("tripinfo-output.write-unfinished", "true"));
            }
            if opt_int(oc, "precision") > 2 {
                for name in [
                    "netstate-dump.precision",
                    "emission-output.precision",
                    "battery-output.precision",
                    "elechybrid-output.precision",
                    "substations-output.precision",
                ] {
                    if opt_is_default(oc, name) {
                        let v = opt_int(oc, "precision").to_string();
                        or_panic(oc.set_default(name, &v));
                    }
                }
            }
            let carFollowModel = opt_str(oc, "carfollow.model");
            if !utils::xml::sumo_xml_definitions::CarFollowModels.has_string(&carFollowModel) {
                utils::common::msg_handler::write_error(&format!("Unknown model '{}' for option 'carfollow.model'.", carFollowModel));
                ok = false;
            }
            if opt_is_set(oc, "default.emergencydecel") {
                let val = opt_str(oc, "default.emergencydecel");
                if val != "default" && val != "decel" && utils::common::string_utils::to_double(&val).is_err() {
                    utils::common::msg_handler::write_error(&format!(
                        "Invalid value '{}' for option 'default.emergencydecel'. Must be a FLOAT or 'default' or 'decel'.",
                        val
                    ));
                    ok = false;
                }
            }
            if opt_is_set(oc, "time-to-teleport.railsignal-deadlock") {
                let t1 = st(&opt_str(oc, "time-to-teleport"));
                let t2 = st(&opt_str(oc, "time-to-teleport.railsignal-deadlock"));
                if t1 > 0 && t2 > 0 && t1 <= t2 {
                    utils::common::msg_handler::write_warning(&format!(
                        "Railsignal-deadlock will not be detected because time-to-teleport ({}) is lower than time-to-teleport.railsignal-deadlock ({}).",
                        utils::common::time::time2string(t1),
                        utils::common::time::time2string(t2)
                    ));
                }
            }
            oc.do_register("", utils::options::option::SumoOption::string_typed("-1", "TIME"));
            if opt_float(oc, "delay") < 0.0 {
                utils::common::msg_handler::write_error("You need a non-negative delay.");
                ok = false;
            }
            for val in opt_strs(oc, "breakpoints") {
                if let Err(e) = utils::common::time::string2time(&val) {
                    utils::common::msg_handler::write_error(&format!("Invalid time '{}' for option 'breakpoints'. {}", val, e));
                    ok = false;
                }
            }
            // shortcut: the C++ #ifndef HAVE_FOX check is applied because this port is built without Fox
            if opt_int(oc, "threads") > 1 {
                utils::common::msg_handler::write_error("Parallel simulation is only possible when compiled with Fox.");
                ok = false;
            }
            if opt_int(oc, "threads") < 1 {
                utils::common::msg_handler::write_error("You need at least one thread.");
                ok = false;
            }
            if opt_int(oc, "threads") > opt_int(oc, "thread-rngs") {
                utils::common::msg_handler::write_warning("Number of threads exceeds number of thread-rngs. Simulation runs with the same seed may produce different results.");
            }
            if opt_str(oc, "game.mode") != "tls" && opt_str(oc, "game.mode") != "drt" {
                utils::common::msg_handler::write_error("game.mode must be one of ['tls', 'drt']");
                ok = false;
            }
            if opt_is_set(oc, "persontrip.transfer.car-walk") {
                for opt in opt_strs(oc, "persontrip.transfer.car-walk") {
                    if opt != "parkingAreas" && opt != "ptStops" && opt != "allJunctions" {
                        utils::common::msg_handler::write_error(&format!(
                            "Invalid transfer option '{}'. Must be one of 'parkingAreas', 'ptStops' or 'allJunctions'.",
                            opt
                        ));
                        ok = false;
                    }
                }
            }
            if !opt_is_default(oc, "weights.random-factor") && (opt_is_set(oc, "astar.all-distances") || opt_is_set(oc, "astar.landmark-distances")) {
                utils::common::msg_handler::write_warning("The option --weights.random-factor should not be used together with astar and precomputed distances.");
            }
            if opt_int(oc, "threads") > 1 {
                utils::common::msg_handler::write_warning("The option --threads has known problems and does NOT provide meaningful speedup at this time (https://github.com/eclipse-sumo/sumo/issues/4767). Using it is not recommended!");
            }
            if opt_bool(oc, "mapmatch.junctions") && opt_is_default(oc, "junction-taz") {
                or_panic(oc.set_default("junction-taz", "true"));
            }
            ok &= crate::microsim::devices::ms_device::MSDevice::check_options(oc);
            ok &= utils::common::system_frame::check_options(oc);
            ok
        })
    }

    /// C++: `void MSFrame::setMSGlobals(OptionsCont &)` [static]
    /// Sets the global microsim-options
    // void
    // MSFrame::setMSGlobals(OptionsCont& oc) {
    //     // pre-initialise the network
    //     // set whether empty edges shall be printed on dump
    //     MSGlobals::gOmitEmptyEdgesOnDump = !oc.getBool("netstate-dump.empty-edges");
    //     // set whether internal lanes shall be used
    //     MSGlobals::gUsingInternalLanes = !oc.getBool("no-internal-links");
    //     MSGlobals::gIgnoreJunctionBlocker = string2time(oc.getString("ignore-junction-blocker")) < 0 ?
    //                                         std::numeric_limits<SUMOTime>::max() : string2time(oc.getString("ignore-junction-blocker"));
    //     // set the grid lock time
    //     MSGlobals::gTimeToGridlock = string2time(oc.getString("time-to-teleport")) < 0 ? 0 : string2time(oc.getString("time-to-teleport"));
    //     MSGlobals::gTimeToImpatience = string2time(oc.getString("time-to-impatience"));
    //     MSGlobals::gTimeToGridlockHighways = string2time(oc.getString("time-to-teleport.highways")) < 0 ? 0 : string2time(oc.getString("time-to-teleport.highways"));
    //     MSGlobals::gGridlockHighwaysSpeed = oc.getFloat("time-to-teleport.highways.min-speed");
    //     MSGlobals::gTimeToTeleportDisconnected = string2time(oc.getString("time-to-teleport.disconnected"));
    //     MSGlobals::gTimeToTeleportBidi = string2time(oc.getString("time-to-teleport.bidi"));
    //     MSGlobals::gTimeToTeleportRSDeadlock = string2time(oc.getString("time-to-teleport.railsignal-deadlock"));
    //     MSGlobals::gRemoveGridlocked = oc.getBool("time-to-teleport.remove");
    //     MSGlobals::gCheck4Accidents = !oc.getBool("ignore-accidents");
    //     MSGlobals::gCheckRoutes = !oc.getBool("ignore-route-errors");
    //     MSGlobals::gEmergencyInsert = oc.getBool("emergency-insert");
    //     MSGlobals::gWeightsSeparateTurns = oc.getFloat("weights.separate-turns");
    //     MSGlobals::gStartupWaitThreshold = string2time(oc.getString("startup-wait-threshold"));
    //     MSGlobals::gLaneChangeDuration = string2time(oc.getString("lanechange.duration"));
    //     MSGlobals::gLateralResolution = oc.getFloat("lateral-resolution");
    //     MSGlobals::gSublane = (MSGlobals::gLaneChangeDuration > 0 || MSGlobals::gLateralResolution > 0);
    //     MSGlobals::gStateLoaded = oc.isSet("load-state");
    //     MSGlobals::gUseMesoSim = oc.getBool("mesosim");
    //     MSGlobals::gMesoLimitedJunctionControl = oc.getBool("meso-junction-control.limited");
    //     MSGlobals::gMesoInterpolatePos = oc.getBool("meso-interpolate-pos");
    //     MSGlobals::gWaitingTimeMemory = string2time(oc.getString("waiting-time-memory"));
    //     MSAbstractLaneChangeModel::initGlobalOptions(oc);
    //     MSGlobals::gOverheadWireSolver = oc.getBool("overhead-wire.solver");
    //     MSGlobals::gOverheadWireRecuperation = oc.getBool("overhead-wire.recuperation");
    //     Circuit::enforceCurrentLimits(oc.getBool("overhead-wire.substation-current-limits"));
    //     MSGlobals::gInsertionChecks = SUMOVehicleParameter::parseInsertionChecks(oc.getString("insertion-checks"));
    //     MSGlobals::gMaxRailSignalBlockLength = oc.getFloat("railsignal.max-block-length");
    //
    //     MSLane::initCollisionOptions(oc);
    //
    //     DELTA_T = string2time(oc.getString("step-length"));
    //
    //     const bool integrationMethodSet = !oc.isDefault("step-method.ballistic");
    //     const bool actionStepLengthSet  = !oc.isDefault("default.action-step-length");
    //     MSGlobals::gSemiImplicitEulerUpdate = !oc.getBool("step-method.ballistic");
    //     // Init default value for gActionStepLength
    //     if (MSGlobals::gSemiImplicitEulerUpdate && actionStepLengthSet && !integrationMethodSet) {
    //         WRITE_MESSAGE(TL("Integration method was set to 'ballistic', since a default action step length was specified."));
    //         MSGlobals::gSemiImplicitEulerUpdate = false;
    //     }
    //     double givenDefaultActionStepLength = oc.getFloat("default.action-step-length");
    //     MSGlobals::gActionStepLength = SUMOVehicleParserHelper::processActionStepLength(givenDefaultActionStepLength);
    //
    //     const std::string defaultEmergencyDecelOption = OptionsCont::getOptions().getString("default.emergencydecel");
    //     if (defaultEmergencyDecelOption == "default") {
    //         MSGlobals::gDefaultEmergencyDecel = VTYPEPARS_DEFAULT_EMERGENCYDECEL_DEFAULT;
    //     } else if (defaultEmergencyDecelOption == "decel") {
    //         MSGlobals::gDefaultEmergencyDecel = VTYPEPARS_DEFAULT_EMERGENCYDECEL_DECEL;
    //     } else {
    //         // value already checked in checkOptions()
    //         MSGlobals::gDefaultEmergencyDecel = StringUtils::toDouble(defaultEmergencyDecelOption);
    //     }
    //     MSGlobals::gNumSimThreads = oc.getInt("threads");
    //     MSGlobals::gNumThreads = MAX2(MSGlobals::gNumSimThreads, oc.getInt("device.rerouting.threads"));
    //
    //     MSGlobals::gEmergencyDecelWarningThreshold = oc.getFloat("emergencydecel.warning-threshold");
    //     MSGlobals::gMinorPenalty = oc.getFloat("weights.minor-penalty");
    //     MSGlobals::gTLSPenalty = oc.getFloat("weights.tls-penalty");
    //     MSGlobals::gTurnaroundPenalty = oc.getFloat("weights.turnaround-penalty");
    //
    //     MSGlobals::gModelParkingManoeuver = oc.getBool("parking.maneuver");
    //
    //     MSGlobals::gStopTolerance = oc.getFloat("ride.stop-tolerance");
    //     MSGlobals::gTLSYellowMinDecel = oc.getFloat("tls.yellow.min-decel");
    //     MSGlobals::gUseStopEnded = oc.getBool("use-stop-ended");
    //     MSGlobals::gUseStopStarted = oc.getBool("use-stop-started");
    //     MSGlobals::gSlopeCentered = oc.getBool("slope-centered");
    //
    //     SVCPermissions defaultClasses = 0;
    //     for (const std::string& vClassName : oc.getStringVector("railsignal.default-classes")) {
    //         defaultClasses |= parseVehicleClasses(vClassName);
    //     }
    //     SVCPermissions mBdefaultClasses = 0;
    //     for (const std::string& vClassName : oc.getStringVector("railsignal.moving-block.default-classes")) {
    //         mBdefaultClasses |= parseVehicleClasses(vClassName);
    //     }
    //     MSRailSignalControl::initSignalized(defaultClasses, mBdefaultClasses);
    //     gTaxiClasses = 0;
    //     for (const std::string& vClassName : oc.getStringVector("device.taxi.vclasses")) {
    //         gTaxiClasses |= parseVehicleClasses(vClassName);
    //     }
    //
    //     std::string error;
    //     if (!SUMOVehicleParameter::parseDepartLane(oc.getString("default.departlane"), "options", "",
    //             MSEdge::getDefaultDepartLane(),
    //             MSEdge::getDefaultDepartLaneDefinition(), error)) {
    //         WRITE_ERROR(error);
    //     }
    //     if (!SUMOVehicleParameter::parseDepartSpeed(oc.getString("default.departspeed"), "options", "",
    //             MSLane::getDefaultDepartSpeed(),
    //             MSLane::getDefaultDepartSpeedDefinition(), error)) {
    //         WRITE_ERROR(error);
    //     }
    //
    // #ifdef _DEBUG
    //     if (oc.isSet("movereminder-output")) {
    //         MSBaseVehicle::initMoveReminderOutput(oc);
    //     }
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn set_ms_globals(oc: &mut OptionsCont) {
        use crate::microsim::ms_globals as g;
        // pre-initialise the network
        // set whether empty edges shall be printed on dump
        g::set_omit_empty_edges_on_dump(!opt_bool(oc, "netstate-dump.empty-edges"));
        // set whether internal lanes shall be used
        g::set_using_internal_lanes(!opt_bool(oc, "no-internal-links"));
        let ignoreJunctionBlocker = st(&opt_str(oc, "ignore-junction-blocker"));
        g::set_ignore_junction_blocker(if ignoreJunctionBlocker < 0 { i64::MAX } else { ignoreJunctionBlocker });
        // set the grid lock time
        let timeToTeleport = st(&opt_str(oc, "time-to-teleport"));
        g::set_time_to_gridlock(if timeToTeleport < 0 { 0 } else { timeToTeleport });
        g::set_time_to_impatience(st(&opt_str(oc, "time-to-impatience")));
        let timeToTeleportHighways = st(&opt_str(oc, "time-to-teleport.highways"));
        g::set_time_to_gridlock_highways(if timeToTeleportHighways < 0 { 0 } else { timeToTeleportHighways });
        g::set_gridlock_highways_speed(opt_float(oc, "time-to-teleport.highways.min-speed"));
        g::set_time_to_teleport_disconnected(st(&opt_str(oc, "time-to-teleport.disconnected")));
        g::set_time_to_teleport_bidi(st(&opt_str(oc, "time-to-teleport.bidi")));
        g::set_time_to_teleport_rs_deadlock(st(&opt_str(oc, "time-to-teleport.railsignal-deadlock")));
        g::set_remove_gridlocked(opt_bool(oc, "time-to-teleport.remove"));
        g::set_check4_accidents(!opt_bool(oc, "ignore-accidents"));
        g::set_check_routes(!opt_bool(oc, "ignore-route-errors"));
        g::set_emergency_insert(opt_bool(oc, "emergency-insert"));
        g::set_weights_separate_turns(opt_float(oc, "weights.separate-turns"));
        g::set_startup_wait_threshold(st(&opt_str(oc, "startup-wait-threshold")));
        g::set_lane_change_duration(st(&opt_str(oc, "lanechange.duration")));
        g::set_lateral_resolution(opt_float(oc, "lateral-resolution"));
        g::set_sublane(g::lane_change_duration() > 0 || g::lateral_resolution() > 0.);
        g::set_state_loaded(opt_is_set(oc, "load-state"));
        g::set_use_meso_sim(opt_bool(oc, "mesosim"));
        g::set_meso_limited_junction_control(opt_bool(oc, "meso-junction-control.limited"));
        g::set_meso_interpolate_pos(opt_bool(oc, "meso-interpolate-pos"));
        g::set_waiting_time_memory(st(&opt_str(oc, "waiting-time-memory")));
        crate::microsim::lcmodels::ms_abstract_lane_change_model::MSAbstractLaneChangeModel::init_global_options(oc);
        g::set_overhead_wire_solver(opt_bool(oc, "overhead-wire.solver"));
        g::set_overhead_wire_recuperation(opt_bool(oc, "overhead-wire.recuperation"));
        Circuit::enforce_current_limits(opt_bool(oc, "overhead-wire.substation-current-limits"));
        g::set_insertion_checks(crate::utils::vehicle::sumo_vehicle_parameter::SUMOVehicleParameter::parse_insertion_checks(&opt_str(oc, "insertion-checks")));
        g::set_max_rail_signal_block_length(opt_float(oc, "railsignal.max-block-length"));
        crate::microsim::ms_lane::MSLane::init_collision_options(oc);
        utils::common::time::set_delta_t(st(&opt_str(oc, "step-length")));
        let integrationMethodSet = !opt_is_default(oc, "step-method.ballistic");
        let actionStepLengthSet = !opt_is_default(oc, "default.action-step-length");
        g::set_semi_implicit_euler_update(!opt_bool(oc, "step-method.ballistic"));
        // Init default value for gActionStepLength
        if g::semi_implicit_euler_update() && actionStepLengthSet && !integrationMethodSet {
            utils::common::msg_handler::write_message("Integration method was set to 'ballistic', since a default action step length was specified.");
            g::set_semi_implicit_euler_update(false);
        }
        let givenDefaultActionStepLength = opt_float(oc, "default.action-step-length");
        g::set_action_step_length(crate::utils::vehicle::sumo_vehicle_parser_helper::SUMOVehicleParserHelper::process_action_step_length(givenDefaultActionStepLength));
        let defaultEmergencyDecelOption = opt_str(oc, "default.emergencydecel");
        if defaultEmergencyDecelOption == "default" {
            // VTYPEPARS_DEFAULT_EMERGENCYDECEL_DEFAULT (SUMOVTypeParameter.h)
            g::set_default_emergency_decel(-1.0);
        } else if defaultEmergencyDecelOption == "decel" {
            // VTYPEPARS_DEFAULT_EMERGENCYDECEL_DECEL (SUMOVTypeParameter.h)
            g::set_default_emergency_decel(-2.0);
        } else {
            // value already checked in checkOptions()
            g::set_default_emergency_decel(or_panic(utils::common::string_utils::to_double(&defaultEmergencyDecelOption)));
        }
        g::set_num_sim_threads(opt_int(oc, "threads"));
        g::set_num_threads(std::cmp::max(g::num_sim_threads(), opt_int(oc, "device.rerouting.threads")));
        g::set_emergency_decel_warning_threshold(opt_float(oc, "emergencydecel.warning-threshold"));
        g::set_minor_penalty(opt_float(oc, "weights.minor-penalty"));
        g::set_tls_penalty(opt_float(oc, "weights.tls-penalty"));
        g::set_turnaround_penalty(opt_float(oc, "weights.turnaround-penalty"));
        g::set_model_parking_manoeuver(opt_bool(oc, "parking.maneuver"));
        g::set_stop_tolerance(opt_float(oc, "ride.stop-tolerance"));
        g::set_tls_yellow_min_decel(opt_float(oc, "tls.yellow.min-decel"));
        g::set_use_stop_ended(opt_bool(oc, "use-stop-ended"));
        g::set_use_stop_started(opt_bool(oc, "use-stop-started"));
        g::set_slope_centered(opt_bool(oc, "slope-centered"));
        let mut defaultClasses: utils::common::sumo_vehicle_class::SVCPermissions = 0;
        for vClassName in opt_strs(oc, "railsignal.default-classes") {
            defaultClasses |= utils::common::sumo_vehicle_class::parse_vehicle_classes(&vClassName);
        }
        let mut mBdefaultClasses: utils::common::sumo_vehicle_class::SVCPermissions = 0;
        for vClassName in opt_strs(oc, "railsignal.moving-block.default-classes") {
            mBdefaultClasses |= utils::common::sumo_vehicle_class::parse_vehicle_classes(&vClassName);
        }
        crate::microsim::traffic_lights::ms_rail_signal_control::MSRailSignalControl::init_signalized(defaultClasses, mBdefaultClasses);
        utils::common::std_defs::set_g_taxi_classes(g_taxi_classes_mask(&opt_strs(oc, "device.taxi.vclasses")));
        let mut error = String::new();
        if !crate::utils::vehicle::sumo_vehicle_parameter::SUMOVehicleParameter::parse_depart_lane(
            &opt_str(oc, "default.departlane"),
            "options",
            "",
            crate::microsim::ms_edge::MSEdge::get_default_depart_lane(),
            crate::microsim::ms_edge::MSEdge::get_default_depart_lane_definition(),
            &mut error,
        ) {
            utils::common::msg_handler::write_error(&error);
        }
        if !crate::utils::vehicle::sumo_vehicle_parameter::SUMOVehicleParameter::parse_depart_speed(
            &opt_str(oc, "default.departspeed"),
            "options",
            "",
            crate::microsim::ms_lane::MSLane::get_default_depart_speed(),
            crate::microsim::ms_lane::MSLane::get_default_depart_speed_definition(),
            &mut error,
        ) {
            utils::common::msg_handler::write_error(&error);
        }
        // C++ _DEBUG-only: compiled out of the release reference build, not ported (MSBaseVehicle::initMoveReminderOutput)
    }

}

