//! Port of od2trips_main.cpp
use std::any::Any;
use std::panic::{self, AssertUnwindSafe};
use std::process::ExitCode;

use sumo_core::od::od_district_cont::ODDistrictCont;
use sumo_core::od::od_matrix::ODMatrix;
use sumo_core::utils::vehicle::sumo_vehicle_parameter::SUMOVehicleParameter;
use utils::common::msg_handler::{with_instance, write_error, write_message, MsgHandler, MsgType};
use utils::common::to_string::to_string;
use utils::common::util_exceptions::ProcessError;
use utils::iodevices::output_device::OutputDevice;
use utils::options::option::SumoOption;
use utils::options::options_cont::{get_bool, get_float, get_int, get_string, get_string_vector, is_default, is_set, with_options, OptionsCont};
use utils::options::options_io;
use utils::xml::xml_sub_sys;

// see duarouter/src/main.rs: the exact string the reference build reports
const VERSION_STRING: &str = "v1.28.0-0000000000";

fn fill_options() {
    with_options(|oc| {
        oc.add_call_example("-c <CONFIGURATION>", "run with configuration file");
        // insert options sub-topics
        utils::common::system_frame::add_configuration_options(oc);
        for t in ["Input", "Output", "Time", "Processing", "Defaults"] {
            oc.add_option_sub_topic(t);
        }
        utils::common::system_frame::add_report_options(oc);
        // register the file input options
        oc.do_register_abbr("taz-files", 'n', SumoOption::file_name());
        for s in ["taz", "net-file", "net"] {
            oc.add_synonyme("taz-files", s, false);
        }
        oc.add_description("taz-files", "Input", "Loads TAZ (districts; also from networks) from FILE(s)");
        oc.do_register_abbr("od-matrix-files", 'd', SumoOption::file_name());
        oc.add_synonyme("od-matrix-files", "od-files", false);
        oc.add_synonyme("od-matrix-files", "od", false);
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
        // register the file output options
        oc.do_register_abbr("output-file", 'o', SumoOption::file_name());
        oc.add_synonyme("output-file", "output", true);
        oc.add_description("output-file", "Output", "Writes trip definitions into FILE");
        oc.do_register("flow-output", SumoOption::file_name());
        oc.add_description("flow-output", "Output", "Writes flow definitions into FILE");
        oc.do_register("flow-output.probability", SumoOption::bool(false));
        oc.add_description("flow-output.probability", "Output", "Writes probabilistic flow instead of evenly spaced flow");
        oc.do_register("flow-output.poisson", SumoOption::bool(false));
        oc.add_description("flow-output.poisson", "Output", "Writes poisson distributed flow instead of evenly spaced flow");
        oc.do_register("pedestrians", SumoOption::bool(false));
        oc.add_description("pedestrians", "Output", "Writes pedestrians instead of vehicles");
        oc.do_register("persontrips", SumoOption::bool(false));
        oc.add_description("persontrips", "Output", "Writes persontrips instead of vehicles");
        oc.do_register("persontrips.modes", SumoOption::string_vector());
        oc.add_description("persontrips.modes", "Output", "Add modes attribute to personTrips");
        oc.do_register("ignore-vehicle-type", SumoOption::bool(false));
        oc.add_synonyme("ignore-vehicle-type", "no-vtype", true);
        oc.add_description("ignore-vehicle-type", "Output", "Does not save vtype information");
        oc.do_register("junctions", SumoOption::bool(false));
        oc.add_description("junctions", "Output", "Writes trips between junctions");
        // register the time settings
        oc.do_register_abbr("begin", 'b', SumoOption::string_typed("0", "TIME"));
        oc.add_description("begin", "Time", "Defines the begin time; Previous trips will be discarded");
        oc.do_register_abbr("end", 'e', SumoOption::string_typed("-1", "TIME"));
        oc.add_description("end", "Time", "Defines the end time; Later trips will be discarded; Defaults to the maximum time that SUMO can represent");
        // register the data processing options
        oc.do_register_abbr("scale", 's', SumoOption::float(1.));
        oc.add_description("scale", "Processing", "Scales the loaded flows by FLOAT");
        oc.do_register("spread.uniform", SumoOption::bool(false));
        oc.add_description("spread.uniform", "Processing", "Spreads trips uniformly over each time period");
        oc.do_register("different-source-sink", SumoOption::bool(false));
        oc.add_description("different-source-sink", "Processing", "Always choose source and sink edge which are not identical");
        oc.do_register("vtype", SumoOption::string_with(""));
        oc.add_description("vtype", "Processing", "Defines the name of the vehicle type to use");
        oc.do_register("prefix", SumoOption::string_with(""));
        oc.add_description("prefix", "Processing", "Defines the prefix for vehicle names");
        oc.do_register("timeline", SumoOption::string_vector());
        oc.add_description("timeline", "Processing", "Uses STR[] as a timeline definition");
        oc.do_register("timeline.day-in-hours", SumoOption::bool(false));
        oc.add_description("timeline.day-in-hours", "Processing", "Uses STR as a 24h-timeline definition");
        oc.do_register("ignore-errors", SumoOption::bool(false));
        oc.add_synonyme("ignore-errors", "dismiss-loading-errors", true);
        oc.add_description("ignore-errors", "Report", "Continue on broken input");
        oc.do_register("no-step-log", SumoOption::bool(false));
        oc.add_description("no-step-log", "Processing", "Disable console output of current time step");
        // register defaults options
        oc.do_register("departlane", SumoOption::string_with("free"));
        oc.add_description("departlane", "Defaults", "Assigns a default depart lane");
        oc.do_register("departpos", SumoOption::string());
        oc.add_description("departpos", "Defaults", "Assigns a default depart position");
        oc.do_register("departspeed", SumoOption::string_with("max"));
        oc.add_description("departspeed", "Defaults", "Assigns a default depart speed");
        oc.do_register("arrivallane", SumoOption::string());
        oc.add_description("arrivallane", "Defaults", "Assigns a default arrival lane");
        oc.do_register("arrivalpos", SumoOption::string());
        oc.add_description("arrivalpos", "Defaults", "Assigns a default arrival position");
        oc.do_register("arrivalspeed", SumoOption::string());
        oc.add_description("arrivalspeed", "Defaults", "Assigns a default arrival speed");
        // add rand options
        utils::common::rand_helper::insert_rand_options(oc);
    });
}

fn check_options(oc: &mut OptionsCont) -> bool {
    let mut ok = true;
    if !is_set("taz-files") {
        write_error("No TAZ input file (-n) specified.");
        ok = false;
    }
    if !is_set("od-matrix-files") && !is_set("od-amitran-files") && !is_set("tazrelation-files") {
        write_error("No input specified.");
        ok = false;
    }
    if !is_set("output-file") && !is_set("flow-output") {
        write_error("No trip table output file (-o) or flow-output is specified.");
        ok = false;
    }
    if get_bool("flow-output.probability") && get_bool("flow-output.poisson") {
        write_error("Only one of the options 'flow-output.probability' and 'flow-output.poisson' may be set.");
        ok = false;
    }
    if get_bool("pedestrians") && get_bool("persontrips") {
        write_error("Only one of the Options 'pedestrians' and 'persontrips' may be set.");
        ok = false;
    }
    let mut p = SUMOVehicleParameter::new();
    let mut error = String::new();
    if is_set("departlane") && !SUMOVehicleParameter::parse_depart_lane(&get_string("departlane"), "option", "departlane", &mut p.departLane, &mut p.departLaneProcedure, &mut error) {
        write_error(&error);
        ok = false;
    }
    if is_set("departpos") && !SUMOVehicleParameter::parse_depart_pos(&get_string("departpos"), "option", "departpos", &mut p.departPos, &mut p.departPosProcedure, &mut error) {
        write_error(&error);
        ok = false;
    }
    if is_set("departspeed") && !SUMOVehicleParameter::parse_depart_speed(&get_string("departspeed"), "option", "departspeed", &mut p.departSpeed, &mut p.departSpeedProcedure, &mut error) {
        write_error(&error);
        ok = false;
    }
    if is_set("arrivallane") && !SUMOVehicleParameter::parse_arrival_lane(&get_string("arrivallane"), "option", "arrivallane", &mut p.arrivalLane, &mut p.arrivalLaneProcedure, &mut error) {
        write_error(&error);
        ok = false;
    }
    if is_set("arrivalpos") && !SUMOVehicleParameter::parse_arrival_pos(&get_string("arrivalpos"), "option", "arrivalpos", &mut p.arrivalPos, &mut p.arrivalPosProcedure, &mut error) {
        write_error(&error);
        ok = false;
    }
    if is_set("arrivalspeed") && !SUMOVehicleParameter::parse_arrival_speed(&get_string("arrivalspeed"), "option", "arrivalspeed", &mut p.arrivalSpeed, &mut p.arrivalSpeedProcedure, &mut error) {
        write_error(&error);
        ok = false;
    }
    ok &= utils::common::system_frame::check_options(oc);
    ok
}

/// C++: `int main(int argc, char** argv)` try block; Ok(false) when a meta option ended the run
fn run(argv: &[String]) -> Result<bool, ProcessError> {
    xml_sub_sys::init();
    fill_options();
    options_io::set_args_argv(argv);
    options_io::get_options(false)?;
    if with_options(|oc| oc.process_meta_options(argv.len() < 2))? {
        utils::common::system_frame::close();
        return Ok(false);
    }
    xml_sub_sys::set_validation(&get_string("xml-validation"), "never", "never")?;
    MsgHandler::init_output_options()?;
    if !with_options(check_options) {
        return Err(ProcessError::default_msg());
    }
    utils::common::rand_helper::init_rand(None, get_bool("random"), get_int("seed"));
    if !is_set("taz-files") {
        return Err(ProcessError::new("You must supply a TAZ, network or districts file ('-n').".to_string()));
    }
    let mut districts = ODDistrictCont::new();
    districts.load_districts(get_string_vector("taz-files"));
    if districts.len() == 0 {
        return Err(ProcessError::new("No districts loaded.".to_string()));
    }
    let mut matrix = ODMatrix::new(&districts, get_float("scale"));
    with_options(|oc| matrix.load_matrix(oc));
    if matrix.get_num_loaded() == 0. {
        return Err(ProcessError::new("No vehicles loaded.".to_string()));
    }
    if with_instance(MsgType::Error, |h| h.was_informed()) && !get_bool("ignore-errors") {
        return Err(ProcessError::new("Loading failed.".to_string()));
    }
    write_message(&format!("{} vehicles loaded.", to_string(&matrix.get_num_loaded())));
    if is_set("timeline") {
        let ps = matrix.parse_time_line(&get_string_vector("timeline"), get_bool("timeline.day-in-hours"));
        matrix.apply_curve(&ps);
    }
    let modes = to_string(&get_string_vector("persontrips.modes"));
    let mut have_output = false;
    let end = if is_default("end") { utils::common::time::SUMOTIME_MAX } else { utils::common::time::string2time(&get_string("end"))? };
    let begin = utils::common::time::string2time(&get_string("begin"))?;
    if OutputDevice::create_device_by_option("output-file", "routes", "routes_file.xsd", -1)? {
        let dev = OutputDevice::get_device_by_option("output-file")?;
        matrix.write(begin, end, &mut dev.borrow_mut(), get_bool("spread.uniform"), get_bool("different-source-sink"), get_bool("ignore-vehicle-type"), &get_string("prefix"), !get_bool("no-step-log"), get_bool("pedestrians"), get_bool("persontrips"), &modes);
        have_output = true;
    }
    if OutputDevice::create_device_by_option("flow-output", "routes", "routes_file.xsd", -1)? {
        let dev = OutputDevice::get_device_by_option("flow-output")?;
        matrix.write_flows(begin, end, &mut dev.borrow_mut(), get_bool("ignore-vehicle-type"), &get_string("prefix"), get_bool("flow-output.probability"), get_bool("flow-output.poisson"), get_bool("pedestrians"), get_bool("persontrips"), &modes);
        have_output = true;
    }
    if !have_output {
        return Err(ProcessError::new("No output file given.".to_string()));
    }
    write_message(&format!("{} vehicles discarded.", to_string(&matrix.get_num_discarded())));
    write_message(&format!("{} vehicles written.", to_string(&matrix.get_num_written())));
    Ok(true)
}

/// the catch (const ProcessError&) block
fn quit_on_process_error(e: &ProcessError) {
    let what = e.to_string();
    if what != "Process Error" && !what.is_empty() {
        write_error(&what);
    }
    with_instance(MsgType::Error, |h| h.inform("Quitting (on error).", false));
}

/// the catch (const std::exception&) block
fn quit_on_exception(what: &str) {
    if !what.is_empty() {
        write_error(what);
    }
    with_instance(MsgType::Error, |h| h.inform("Quitting (on error).", false));
}

/// C++ throws that unwound to main: ProcessError payloads, other panics as std::exception
fn report_panic(payload: Box<dyn Any + Send>) {
    if let Some(e) = payload.downcast_ref::<ProcessError>() {
        quit_on_process_error(e);
    } else if let Some(s) = payload.downcast_ref::<String>() {
        quit_on_exception(s);
    } else if let Some(s) = payload.downcast_ref::<&str>() {
        quit_on_exception(s);
    } else {
        with_instance(MsgType::Error, |h| h.inform("Quitting (on unknown error).", false));
    }
}

fn main() -> ExitCode {
    let argv: Vec<String> = std::env::args().collect();
    let default_hook = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        // throws are reported by report_panic, only real panics reach the default hook
        if !info.payload().is::<ProcessError>() {
            default_hook(info);
        }
    }));
    with_options(|oc| {
        oc.set_application_description("Importer of O/D-matrices for the microscopic, multi-modal traffic simulation SUMO.");
        oc.set_application_name("od2trips", &format!("Eclipse SUMO od2trips {VERSION_STRING}"));
    });
    let ret: u8 = match panic::catch_unwind(AssertUnwindSafe(|| run(&argv))) {
        Ok(Ok(true)) => 0,
        Ok(Ok(false)) => return ExitCode::SUCCESS,
        Ok(Err(e)) => {
            quit_on_process_error(&e);
            1
        }
        Err(payload) => {
            report_panic(payload);
            1
        }
    };
    utils::common::system_frame::close();
    if ret == 0 {
        println!("Success.");
    }
    ExitCode::from(ret)
}
