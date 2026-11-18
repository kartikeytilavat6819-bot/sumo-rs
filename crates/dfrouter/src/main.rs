//! Port of dfrouter/dfrouter_main.cpp
use std::any::Any;
use std::panic::{self, AssertUnwindSafe};
use std::process::ExitCode;

use sumo_core::dfrouter::dfrouter_main::{read_detector_flows, read_detectors, start_computation};
use sumo_core::dfrouter::rodf_detector::RODFDetectorCon;
use sumo_core::dfrouter::rodf_detector_flow::RODFDetectorFlows;
use sumo_core::dfrouter::rodf_edge_builder::RODFEdgeBuilder;
use sumo_core::dfrouter::rodf_frame::RODFFrame;
use sumo_core::dfrouter::rodf_net::RODFNet;
use sumo_core::router::ro_loader::ROLoader;
use sumo_core::router::ro_net::RONet;
use utils::common::msg_handler::{with_instance, write_error, MsgHandler, MsgType};
use utils::common::ptr::Ptr;
use utils::common::time::string2time;
use utils::common::util_exceptions::ProcessError;
use utils::options::options_cont::{get_bool, get_int, with_options};
use utils::options::options_io;
use utils::xml::xml_sub_sys;

// C++ VERSION_STRING is generated into version.h as `<git describe>-<commit-ish>`; the port has no
// generated version info, so it uses the exact string the reference build reports.
const VERSION_STRING: &str = "v1.28.0-0000000000";

/// C++: `int main(int argc, char** argv)` try block; Ok(false) when a meta option ended the run
fn run(argv: &[String]) -> Result<bool, ProcessError> {
    xml_sub_sys::init();
    RODFFrame::fill_options();
    options_io::set_args_argv(argv);
    options_io::get_options(false)?;
    if with_options(|oc| oc.process_meta_options(argv.len() < 2))? {
        utils::common::system_frame::close();
        return Ok(false);
    }
    with_options(|oc| {
        utils::common::system_frame::check_options(oc);
        xml_sub_sys::set_validation(&oc.get_string("xml-validation")?, &oc.get_string("xml-validation.net")?, "never")
    })?;
    MsgHandler::init_output_options()?;
    if !RODFFrame::check_options() {
        return Err(ProcessError::default_msg());
    }
    utils::common::rand_helper::init_rand(None, get_bool("random"), get_int("seed"));
    with_options(|oc| -> Result<(), ProcessError> {
        let mut loader = ROLoader::new(oc, false, !get_bool("no-step-log"));
        let mut net = Box::new(RODFNet::new(get_bool("highway-mode")));
        RONet::register_instance(Ptr::from_mut(&mut net.base));
        let mut builder = RODFEdgeBuilder::new();
        loader.load_net(&mut net, &mut builder);
        net.build_approach_list();
        let netp = Ptr::from_box(&net);
        let mut detectors = RODFDetectorCon::new();
        read_detectors(&mut detectors, oc, netp);
        let mut flows = RODFDetectorFlows::new(string2time(&oc.get_string("begin")?)?, string2time(&oc.get_string("end")?)?, string2time(&oc.get_string("time-step")?)?);
        read_detector_flows(&mut flows, oc, &mut detectors);
        start_computation(netp, &mut flows, &mut detectors, oc);
        Ok(())
    })?;
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
        oc.set_application_description("Builds vehicle routes for SUMO using detector values.");
        oc.set_application_name("dfrouter", &format!("Eclipse SUMO dfrouter {VERSION_STRING}"));
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
