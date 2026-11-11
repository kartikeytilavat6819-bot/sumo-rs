//! Port of utils/xml/XMLSubSys.{h,cpp}
//! No reader pool (C++ myReaders): every runParser call builds a fresh SUMOSAXReader, and
//! transcode* are not needed since Rust strings are UTF-8.
// shortcut: no grammar pool / schema loading, validation schemes are only checked and stored
use super::generic_sax_handler::SaxHandler;
use super::stand_ins::{clear_errors, error_was_informed, is_readable, write_error, write_message, write_warning, ProcessError};
use super::sumo_sax_reader::SUMOSAXReader;
use std::sync::Mutex;

struct Schemes {
    validation: String,
    net: String,
    route: String,
    needs_validation_warning: bool,
}

static SCHEMES: Mutex<Schemes> = Mutex::new(Schemes { validation: String::new(), net: String::new(), route: String::new(), needs_validation_warning: true });

fn schemes() -> std::sync::MutexGuard<'static, Schemes> {
    let mut s = SCHEMES.lock().unwrap_or_else(|e| e.into_inner());
    if s.validation.is_empty() {
        // C++ static initialisers
        s.validation = "local".into();
        s.net = "local".into();
        s.route = "local".into();
    }
    s
}

/// Xerces initialisation; nothing to do
pub fn init() {}

fn warn_local_scheme(s: &mut Schemes, new_scheme: &str, have_sumo_home: bool) -> Result<String, ProcessError> {
    if !["never", "auto", "always", "local"].contains(&new_scheme) {
        return Err(ProcessError::Process(format!("Unknown xml validation scheme + '{new_scheme}'.")));
    }
    if !have_sumo_home && new_scheme == "local" {
        if s.needs_validation_warning {
            write_warning("Environment variable SUMO_HOME is not set properly, disabling XML validation. Set 'auto' or 'always' for web lookups.");
            s.needs_validation_warning = false;
        }
        return Ok("never".into());
    }
    Ok(new_scheme.to_string())
}

pub fn set_validation(validation_scheme: &str, net_validation_scheme: &str, route_validation_scheme: &str) -> Result<(), ProcessError> {
    let have_sumo_home = std::env::var("SUMO_HOME").is_ok_and(|p| is_readable(&format!("{p}/data/xsd/net_file.xsd")));
    let mut s = schemes();
    s.validation = warn_local_scheme(&mut s, validation_scheme, have_sumo_home)?;
    s.net = warn_local_scheme(&mut s, net_validation_scheme, have_sumo_home)?;
    s.route = warn_local_scheme(&mut s, route_validation_scheme, have_sumo_home)?;
    if (s.validation != "never" || s.net != "never" || s.route != "never") && !have_sumo_home && s.needs_validation_warning {
        write_warning("Environment variable SUMO_HOME is not set properly, XML validation will fail or use slow website lookups.");
        s.needs_validation_warning = false;
    }
    Ok(())
}

pub fn close() {}

fn scheme_for(is_net: bool, is_route: bool) -> String {
    let s = schemes();
    if is_route {
        s.route.clone()
    } else if is_net {
        s.net.clone()
    } else {
        s.validation.clone()
    }
}

/// getSAXReader: the handler is passed to the parse calls instead
pub fn get_sax_reader(is_net: bool, is_route: bool) -> SUMOSAXReader {
    SUMOSAXReader::new(&scheme_for(is_net, is_route))
}

/// Runs the parser on `file`; true if no error was reported.
pub fn run_parser(handler: &mut dyn SaxHandler, file: &str, is_net: bool, is_route: bool, is_external: bool, catch_exceptions: bool) -> Result<bool, ProcessError> {
    clear_errors();
    let mut validation_scheme = scheme_for(is_net, is_route);
    if is_external && validation_scheme == "local" {
        write_message(&format!("Disabling XML validation for external file '{file}'. Use 'auto' or 'always' to enable."));
        validation_scheme = "never".into();
    }
    let mut reader = SUMOSAXReader::new(&validation_scheme);
    let prev_file = handler.handler().get_file_name().to_string();
    handler.handler().set_file_name(file);
    match reader.parse(handler, file) {
        Ok(()) => handler.handler().set_file_name(&prev_file),
        Err(e) => {
            if !catch_exceptions {
                return Err(e);
            }
            let what = e.to_string();
            write_error(if what.is_empty() { "Process Error" } else { &what });
        }
    }
    Ok(!error_was_informed())
}
