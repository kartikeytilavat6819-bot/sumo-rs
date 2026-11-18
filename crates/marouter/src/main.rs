//! Port of marouter/marouter_main.cpp
use std::any::Any;
use std::panic::{self, AssertUnwindSafe};
use std::process::ExitCode;

use sumo_core::marouter::marouter_main::{compute_all_pairs, compute_routes};
use sumo_core::marouter::roma_edge_builder::ROMAEdgeBuilder;
use sumo_core::marouter::roma_frame::ROMAFrame;
use sumo_core::marouter::roma_route_handler::ROMARouteHandler;
use sumo_core::od::od_district_cont::ODDistrictCont;
use sumo_core::od::od_matrix::ODMatrix;
use sumo_core::router::ro_loader::ROLoader;
use sumo_core::router::ro_net::RONet;
use utils::common::msg_handler::{with_instance, write_error, write_message, write_warning, MsgHandler, MsgType};
use utils::common::to_string::to_string;
use utils::common::util_exceptions::ProcessError;
use utils::options::options_cont::{get_bool, get_float, get_int, with_options, OptionsCont};
use utils::options::options_io;
use utils::xml::xml_sub_sys;

// see duarouter/src/main.rs: the exact string the reference build reports
const VERSION_STRING: &str = "v1.28.0-0000000000";

/// C++ initNet: loads the net and the weights
fn init_net(net: &mut RONet, loader: &mut ROLoader, oc: &mut OptionsCont) {
    let mut builder = ROMAEdgeBuilder::new();
    sumo_core::router::ro_edge::ROEdge::set_global_options(oc.get_bool("weights.interpolate").unwrap());
    loader.load_net(net, &mut builder);
    if oc.is_set("weight-files", true).unwrap() {
        loader.load_weights(net, "weight-files", &oc.get_string("weight-attribute").unwrap(), false, oc.get_bool("weights.expand").unwrap());
    }
    if oc.is_set("lane-weight-files", true).unwrap() {
        loader.load_weights(net, "lane-weight-files", &oc.get_string("weight-attribute").unwrap(), true, oc.get_bool("weights.expand").unwrap());
    }
}

/// C++: `int main(int argc, char** argv)` try block; Ok(false) when a meta option ended the run
fn run(argv: &[String]) -> Result<bool, ProcessError> {
    xml_sub_sys::init();
    ROMAFrame::fill_options();
    options_io::set_args_argv(argv);
    options_io::get_options(false)?;
    if with_options(|oc| oc.process_meta_options(argv.len() < 2))? {
        utils::common::system_frame::close();
        return Ok(false);
    }
    with_options(|oc| {
        utils::common::system_frame::check_options(oc);
        xml_sub_sys::set_validation(&oc.get_string("xml-validation")?, &oc.get_string("xml-validation.net")?, &oc.get_string("xml-validation.routes")?)
    })?;
    MsgHandler::init_output_options()?;
    if !ROMAFrame::check_options() {
        return Err(ProcessError::default_msg());
    }
    utils::common::rand_helper::init_rand(None, get_bool("random"), get_int("seed"));
    with_options(|oc| -> Result<(), ProcessError> {
        let mut loader = ROLoader::new(oc, false, false);
        let mut net = Box::new(RONet::new());
        RONet::register_instance(utils::common::ptr::Ptr::from_box(&net));
        init_net(&mut net, &mut loader, oc);
        if oc.is_set("all-pairs-output", true)? {
            compute_all_pairs(&mut net, oc);
            if net.get_districts().is_empty() {
                return Ok(());
            }
        }
        if net.get_districts().is_empty() {
            write_warning("No districts loaded, will use edge ids!");
        }
        let mut districts = ODDistrictCont::new();
        districts.make_districts(&net.get_districts().clone());
        let mut matrix = ODMatrix::new(&districts, get_float("scale"));
        matrix.load_matrix(oc);
        let mut handler = ROMARouteHandler::new(&mut matrix);
        matrix.load_routes(oc, &mut handler);
        if matrix.get_num_loaded() == matrix.get_num_discarded() {
            return Err(ProcessError::new("No valid vehicles loaded.".to_string()));
        }
        if with_instance(MsgType::Error, |h| h.was_informed()) && !get_bool("ignore-errors") {
            return Err(ProcessError::new("Loading failed.".to_string()));
        }
        with_instance(MsgType::Error, |h| h.clear(true));
        write_message(&format!("{} valid vehicles loaded (total seen: {}).", to_string(&(matrix.get_num_loaded() - matrix.get_num_discarded())), to_string(&matrix.get_num_loaded())));
        compute_routes(&mut net, oc, &mut matrix);
        if with_instance(MsgType::Error, |h| h.was_informed()) {
            return Err(ProcessError::default_msg());
        }
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
        oc.set_application_description("Import O/D-matrices for macroscopic traffic assignment to generate SUMO routes.");
        oc.set_application_name("marouter", &format!("Eclipse SUMO marouter {VERSION_STRING}"));
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
