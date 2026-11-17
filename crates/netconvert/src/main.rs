//! Port of netconvert_main.cpp
use std::any::Any;
use std::collections::BTreeSet;
use std::panic::{self, AssertUnwindSafe};
use std::process::ExitCode;

use sumo_core::netbuild::nb_frame::NBFrame;
use sumo_core::netbuild::nb_net_builder::NBNetBuilder;
use sumo_core::netimport::ni_frame::NIFrame;
use sumo_core::netimport::ni_loader::NILoader;
use sumo_core::netwrite::nw_frame::NWFrame;
use utils::common::msg_handler::{with_instance, write_error, MsgHandler, MsgType};
use utils::common::util_exceptions::ProcessError;
use utils::distribution::distribution_cont;
use utils::geom::geo_conv_helper::GeoConvHelper;
use utils::options::options_cont::{get_bool, with_options};
use utils::options::options_io;
use utils::xml::xml_sub_sys;

// C++ VERSION_STRING is generated into version.h as `<git describe>-<commit-ish>`; the port has no
// generated version info, so it uses the exact string the reference build reports.
const VERSION_STRING: &str = "v1.28.0-0000000000";

// ===========================================================================
// method definitions
// ===========================================================================
/// C++: `void fillOptions()`
fn fill_options() {
    with_options(|oc| {
        oc.add_call_example("-c <CONFIGURATION>", "generate net with options read from file");
        oc.add_call_example("-n ./nodes.xml -e ./edges.xml -v -t ./owntypes.xml", "generate net with given nodes, edges, and edge types doing verbose output");

        // insert options sub-topics
        utils::common::system_frame::add_configuration_options(oc);
        oc.add_option_sub_topic("Input");
        oc.add_option_sub_topic("Output");
        GeoConvHelper::add_projection_options(oc);
        oc.add_option_sub_topic("Processing");
        oc.add_option_sub_topic("Building Defaults");
        oc.add_option_sub_topic("TLS Building");
        oc.add_option_sub_topic("Ramp Guessing");
        oc.add_option_sub_topic("Edge Removal");
        oc.add_option_sub_topic("Unregulated Nodes");
        oc.add_option_sub_topic("Junctions");
        oc.add_option_sub_topic("Pedestrian");
        oc.add_option_sub_topic("Bicycle");
        oc.add_option_sub_topic("Railway");
        oc.add_option_sub_topic("Formats");

        NIFrame::fill_options(oc, false);
        NBFrame::fill_options(oc, false);
        NWFrame::fill_options(oc, false);
        utils::common::rand_helper::insert_rand_options(oc);
    });
}

/// C++: `bool checkOptions()`
fn check_options() -> bool {
    with_options(|oc| {
        let mut ok = NIFrame::check_options(oc);
        ok &= NBFrame::check_options(oc);
        ok &= NWFrame::check_options(oc);
        ok &= utils::common::system_frame::check_options(oc);
        ok
    })
}

fn close_system_frame() {
    utils::common::system_frame::close();
}

/// The try block of C++ `main()`; Ok(false) when a meta option (--help, --version, ...) ended the run
fn run(argv: &[String]) -> Result<bool, ProcessError> {
    xml_sub_sys::init();
    fill_options();
    options_io::set_args_argv(argv);
    options_io::get_options(false)?;
    if with_options(|oc| oc.process_meta_options(argv.len() < 2))? {
        close_system_frame();
        return Ok(false);
    }
    with_options(|oc| -> Result<(), ProcessError> {
        if oc.is_set("edge-files", true)? && !oc.is_set("type-files", true)? && oc.is_default("ignore-errors.edge-type") {
            oc.set_default("ignore-errors.edge-type", "true")?;
        }
        xml_sub_sys::set_validation(&oc.get_string("xml-validation")?, &oc.get_string("xml-validation.net")?, "never")?;
        if oc.is_default("aggregate-warnings") {
            oc.set_default("aggregate-warnings", "5")?;
        }
        Ok(())
    })?;
    MsgHandler::init_output_options()?;
    if !check_options() {
        return Err(ProcessError::default_msg());
    }
    // shortcut: RandHelper::initRandGlobal() is skipped because the random and seed options are not registered, the global RNG keeps its default seed, upgrade with insertRandOptions
    // build the projection
    if !with_options(|oc| GeoConvHelper::init_from_options(oc))? {
        return Err(ProcessError::new("Could not build projection!"));
    }
    let mut nb = NBNetBuilder::new();
    with_options(|oc| nb.apply_options(oc));
    // load data
    let mut nl = NILoader::new(&mut nb);
    with_options(|oc| nl.load(oc));
    // flush aggregated errors and optionally ignore them
    let ignore_errors = get_bool("ignore-errors");
    with_instance(MsgType::Error, |h| h.clear(ignore_errors));
    // check whether any errors occurred
    if with_instance(MsgType::Error, |h| h.was_informed()) {
        return Err(ProcessError::default_msg());
    }
    with_options(|oc| nb.compute(oc, &BTreeSet::new(), true));
    // check whether any errors occurred
    if with_instance(MsgType::Error, |h| h.was_informed()) {
        return Err(ProcessError::default_msg());
    }
    // report
    nb.get_node_cont().print_built_nodes_statistics();
    with_options(|oc| NWFrame::write_network(oc, &mut nb));
    Ok(true)
}

/// the catch (const ProcessError&) block
fn quit_on_process_error(e: &ProcessError) {
    with_instance(MsgType::Warning, |h| h.clear(false));
    with_instance(MsgType::Error, |h| h.clear(false));
    let what = e.to_string();
    if what != "Process Error" && !what.is_empty() {
        write_error(&what);
    }
    with_instance(MsgType::Error, |h| h.inform("Quitting (on error).", false));
}

/// the catch (const std::exception&) block
fn quit_on_exception(what: &str) {
    with_instance(MsgType::Warning, |h| h.clear(false));
    with_instance(MsgType::Error, |h| h.clear(false));
    if !what.is_empty() {
        write_error(what);
    }
    with_instance(MsgType::Error, |h| h.inform("Quitting (on error).", false));
}

/// the catch (...) block
fn quit_on_unknown_error() {
    with_instance(MsgType::Warning, |h| h.clear(false));
    with_instance(MsgType::Error, |h| h.clear(false));
    with_instance(MsgType::Error, |h| h.inform("Quitting (on unknown error).", false));
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
        quit_on_unknown_error();
    }
}

/// C++: `int main(int argc, char** argv)`
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
        oc.set_application_description("Network importer / builder for the microscopic, multi-modal traffic simulation SUMO.");
        oc.set_application_name("netconvert", &format!("Eclipse SUMO netconvert {VERSION_STRING}"));
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
    distribution_cont::clear();
    close_system_frame();
    // report about ending
    if ret == 0 {
        println!("Success.");
    }
    ExitCode::from(ret)
}
