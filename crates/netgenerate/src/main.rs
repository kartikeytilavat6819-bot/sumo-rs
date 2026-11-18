//! Port of netgen/netgen_main.cpp (main(); the rest lives in sumo_core::netgen::netgen_main)
use std::any::Any;
use std::collections::BTreeSet;
use std::panic::{self, AssertUnwindSafe};
use std::process::ExitCode;

use sumo_core::netbuild::nb_net_builder::NBNetBuilder;
use sumo_core::netgen::netgen_main::{build_network, check_options, fill_options};
use sumo_core::netimport::ni_type_loader::NITypeLoader;
use sumo_core::netimport::nixml_types_handler::NIXMLTypesHandler;
use sumo_core::netwrite::nw_frame::NWFrame;
use utils::common::msg_handler::{with_instance, write_error, write_message, MsgHandler, MsgType};
use utils::common::util_exceptions::ProcessError;
use utils::geom::boundary::Boundary;
use utils::geom::geo_conv_helper::GeoConvHelper;
use utils::geom::position::Position;
use utils::options::options_cont::with_options;
use utils::options::options_io;
use utils::xml::xml_sub_sys;

// same version string as the netconvert port (the reference build reports it)
const VERSION_STRING: &str = "v1.28.0-0000000000";

/// The try block of C++ `main()`; Ok(false) when a meta option ended the run
fn run(argv: &[String]) -> Result<bool, ProcessError> {
    xml_sub_sys::init();
    fill_options();
    options_io::set_args_argv(argv);
    options_io::get_options(false)?;
    if with_options(|oc| oc.process_meta_options(argv.len() < 2))? {
        utils::common::system_frame::close();
        return Ok(false);
    }
    with_options(|oc| -> Result<(), ProcessError> {
        xml_sub_sys::set_validation(&oc.get_string("xml-validation")?, "never", "never")
    })?;
    MsgHandler::init_output_options()?;
    if !check_options() {
        return Err(ProcessError::default_msg());
    }
    with_options(|oc| -> Result<(), ProcessError> {
        let off = Position::new(oc.get_float("offset.x")?, oc.get_float("offset.y")?);
        GeoConvHelper::init("!", off, Boundary::new(), Boundary::new(), 1.0);
        utils::common::rand_helper::init_rand(None, oc.get_bool("random")?, oc.get_int("seed")?);
        Ok(())
    })?;
    let mut nb = NBNetBuilder::new();
    with_options(|oc| nb.apply_options(oc));
    let type_files = with_options(|oc| -> Result<Option<Vec<String>>, ProcessError> {
        Ok(if oc.is_set("type-files", true)? { Some(oc.get_string_vector("type-files")?.clone()) } else { None })
    })?;
    if let Some(files) = type_files {
        let mut handler = NIXMLTypesHandler::new(nb.get_type_cont());
        NITypeLoader::load(&mut handler, &files, "types", false);
    }
    // build the netgen-network description
    let net = build_network(&mut nb);
    // ... and we have to do this...
    with_options(|oc| oc.reset_writable());
    // transfer to the netbuilding structures
    net.to_nb();
    // C++ `delete net`
    let mut boxed = unsafe { Box::from_raw(net.as_raw()) };
    boxed.drop_();
    drop(boxed);
    write_message(" Generation done;");
    write_message(&format!("   {} nodes generated.", nb.get_node_cont().size()));
    write_message(&format!("   {} edges generated.", nb.get_edge_cont().size()));
    if with_options(|oc| oc.get_bool("tls.discard-simple"))? {
        let tlc: *mut _ = nb.get_tl_logic_cont();
        nb.get_node_cont().discard_traffic_lights(unsafe { &mut *tlc }, true);
        let removed = nb.get_tl_logic_cont().get_num_extracted();
        if removed > 0 {
            write_message(&format!(" Removed {removed} traffic lights at geometry-like nodes"));
        }
    }
    with_options(|oc| nb.compute(oc, &BTreeSet::new(), true));
    nb.get_node_cont().print_built_nodes_statistics();
    with_options(|oc| NWFrame::write_network(oc, &mut nb));
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

/// C++ throws that unwound to main: ProcessError payloads, other panics as std::exception
fn report_panic(payload: Box<dyn Any + Send>) {
    if let Some(e) = payload.downcast_ref::<ProcessError>() {
        quit_on_process_error(e);
    } else if let Some(s) = payload.downcast_ref::<String>() {
        if !s.is_empty() {
            write_error(s);
        }
        with_instance(MsgType::Error, |h| h.inform("Quitting (on error).", false));
    } else if let Some(s) = payload.downcast_ref::<&str>() {
        if !s.is_empty() {
            write_error(s);
        }
        with_instance(MsgType::Error, |h| h.inform("Quitting (on error).", false));
    } else {
        with_instance(MsgType::Error, |h| h.inform("Quitting (on unknown error).", false));
    }
}

fn main() -> ExitCode {
    let argv: Vec<String> = std::env::args().collect();
    let default_hook = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        if !info.payload().is::<ProcessError>() {
            default_hook(info);
        }
    }));
    with_options(|oc| {
        oc.set_application_description("Synthetic network generator for the microscopic, multi-modal traffic simulation SUMO.");
        oc.set_application_name("netgenerate", &format!("Eclipse SUMO netgenerate {VERSION_STRING}"));
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
