//! Port of sumo_main.cpp
// shortcut: signal handlers (SIGINT/SIGTERM graceful stop, SIGUSR1 statistics) are not installed; they need libc signal() and MSNet's instance is thread-local so a Windows handler thread cannot see it; upgrade when MSNet's instance is process-global and signal FFI is added
// C++ VERSION_STRING is generated into version.h as `<git describe>-<commit-ish>`; the port has no generated version info, so it uses the exact string the reference build reports.
const VERSION_STRING: &str = "1.28.0";
use std::any::Any;
use std::panic::AssertUnwindSafe;

use sumo_core::microsim::ms_net::{MSNet, MSNet_SimulationState};
use sumo_core::netload::nl_builder::NLBuilder;
use sumo_core::traci_server::tra_ci_server::TraCIServer;
use utils::common::msg_handler::{MsgHandler, MsgType, with_instance, write_error};
use utils::common::ptr::Ptr;
use utils::common::std_defs::set_g_simulation;
use utils::common::time::string2time;
use utils::common::util_exceptions::ProcessError;
use utils::iodevices::output_device::OutputDevice;
use utils::options::options_cont::{get_string, with_options};
use utils::options::options_io::set_args_argv;
use utils::xml::xml_sub_sys;

/// C++ `delete net`; deleting null is a no-op
fn delete_net(net: Ptr<MSNet>) {
    if !net.is_null() {
        // ~MSNet: writes pending output (Command_SaveTLSProgram etc.) before the memory goes
        net.m().drop_();
        unsafe { drop(Box::from_raw(net.as_raw())) }
    }
}

fn inform_error(msg: &str) {
    with_instance(MsgType::Error, |h| h.inform(msg, false));
}

/// Text of a non-ProcessError panic, the Rust stand-in for `std::exception::what()`
fn panic_text(payload: &(dyn Any + Send)) -> Option<String> {
    payload.downcast_ref::<String>().cloned().or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
}

/// C++ `main`: returns the process exit code
fn sumo_main(args: Vec<String>) -> i32 {
    // oc97: temporary trace-enable marker (removed with the tracing code)
    if std::env::var_os("OC97_TRACE").is_some() {
        eprintln!("OC97 trace active");
    }
    // Panics stand in for C++ exceptions and are caught/converted to error
    // status or written by the handlers below. The default hook would print
    // "thread panicked at ..." for every caught one, which C++ never does.
    // Keep a backtrace available for debugging when explicitly requested.
    let default_hook = std::panic::take_hook();
    let show_panics = std::env::var_os("RUST_BACKTRACE").is_some();
    std::panic::set_hook(Box::new(move |info| {
        if show_panics {
            default_hook(info);
        }
    }));
    with_options(|oc| {
        oc.set_application_description("A microscopic, multi-modal traffic simulation.");
        oc.set_application_name("sumo", &format!("Eclipse SUMO sumo {VERSION_STRING}"));
    });
    set_g_simulation(true);
    let mut ret = 0;
    let mut net: Ptr<MSNet> = Ptr::default();
    let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
        // initialise subsystems
        xml_sub_sys::init();
        set_args_argv(&args);
        // load the net
        let mut state = MSNet_SimulationState::SIMSTATE_LOADING;
        while state == MSNet_SimulationState::SIMSTATE_LOADING {
            net = NLBuilder::init(false);
            if !net.is_null() {
                let begin = string2time(&get_string("begin")).unwrap_or_else(|e| std::panic::panic_any(e));
                let end = string2time(&get_string("end")).unwrap_or_else(|e| std::panic::panic_any(e));
                state = net.m().simulate(begin, end);
                delete_net(net);
                // C++ keeps the dangling pointer; null it so the error path does not delete it twice
                net = Ptr::default();
            } else {
                break;
            }
            // flush warnings and prepare reinit of all outputs
            with_instance(MsgType::Warning, |h| h.clear(true));
            OutputDevice::close_all(false);
            MsgHandler::cleanup_on_end();
        }
    }));
    if let Err(payload) = result {
        if let Some(e) = payload.downcast_ref::<ProcessError>() {
            let msg = e.to_string();
            if msg != "Process Error" && !msg.is_empty() {
                write_error(&msg);
            }
            inform_error("Quitting (on error).");
            // we need to delete the network explicitly to trigger the cleanup in the correct order
            delete_net(net);
            ret = 1;
        } else if let Some(msg) = panic_text(&*payload) {
            if !msg.is_empty() {
                write_error(&msg);
            }
            inform_error("Quitting (on error).");
            ret = 1;
        } else {
            inform_error("Quitting (on unknown error).");
            ret = 1;
        }
    }
    TraCIServer::close();
    utils::common::system_frame::close();
    ret
}

fn main() {
    std::process::exit(sumo_main(std::env::args().collect()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn panic_text_reads_string_payloads() {
        assert_eq!(panic_text(&"boom"), Some("boom".to_string()));
        assert_eq!(panic_text(&String::from("bang")), Some("bang".to_string()));
        assert_eq!(panic_text(&ProcessError::default_msg()), None);
    }
}
