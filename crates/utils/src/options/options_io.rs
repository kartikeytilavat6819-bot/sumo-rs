//! Port of utils/options/OptionsIO.{h,cpp}
// shortcut: gzipped files given as the single argument are not decompressed in get_root, add flate2 when needed
use super::options_cont::with_options;
use super::options_loader::OptionsLoader;
use super::options_parser;
use crate::common::file_helpers;
use crate::common::msg_handler::{progress_begin_message, progress_done_message};
use crate::common::util_exceptions::ProcessError;
use crate::tlf;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

static ARGS: Mutex<Vec<String>> = Mutex::new(Vec::new());
static LOAD_TIME: Mutex<SystemTime> = Mutex::new(UNIX_EPOCH);

fn args() -> Vec<String> {
    ARGS.lock().unwrap_or_else(|e| e.into_inner()).clone()
}

/// setArgs(argc, argv): all arguments including the program name
pub fn set_args_argv(argv: &[String]) {
    *ARGS.lock().unwrap_or_else(|e| e.into_inner()) = argv.to_vec();
}

/// setArgs(vector): arguments without the program name (an empty one is inserted if missing)
pub fn set_args(args: &[String]) {
    let mut a = ARGS.lock().unwrap_or_else(|e| e.into_inner());
    a.resize(1, String::new());
    a.extend_from_slice(args);
}

pub fn get_arg_c() -> usize {
    ARGS.lock().unwrap_or_else(|e| e.into_inner()).len()
}

pub fn get_load_time() -> SystemTime {
    *LOAD_TIME.lock().unwrap_or_else(|e| e.into_inner())
}

/// Parses the command line arguments and loads the configuration
pub fn get_options(command_line_only: bool) -> Result<(), ProcessError> {
    *LOAD_TIME.lock().unwrap_or_else(|e| e.into_inner()) = SystemTime::now();
    let my_args = args();
    if my_args.len() == 2 && !my_args[1].starts_with('-') {
        // special case only one parameter, check who can handle it
        let root = get_root(&my_args[1])?;
        if with_options(|oc| oc.set_by_root_element(&root, &my_args[1]))? {
            if !command_line_only {
                load_configuration()?;
            }
            return Ok(());
        }
    }
    // preparse the options
    //  (maybe another configuration file was chosen)
    if !options_parser::parse(&my_args, true) {
        return Err(ProcessError::new("Could not parse commandline options."));
    }
    if !command_line_only || with_options(|oc| oc.is_set("save-configuration", false))? {
        // read the configuration when everything's ok
        load_configuration()?;
    }
    Ok(())
}

/// Loads and parses the configuration, then reparses the command line
pub fn load_configuration() -> Result<(), ProcessError> {
    let my_args = args();
    with_options(|oc| {
        if oc.exists("configuration-file") && oc.is_set("configuration-file", true)? {
            let path = oc.get_string("configuration-file")?;
            if !file_helpers::is_readable(&path) {
                return Err(ProcessError::new(tlf!("Could not access configuration '%'.", path)));
            }
            let verbose = !oc.exists("verbose") || oc.get_bool("verbose")?;
            if verbose {
                progress_begin_message("Loading configuration");
            }
            oc.reset_writable();
            let mut handler = OptionsLoader::new(false);
            if let Err(e) = handler.parse_file(oc, &path, false) {
                return Err(ProcessError::new(format!("Could not load configuration '{path}':\n {e}")));
            }
            if handler.error_occurred() {
                return Err(ProcessError::new(tlf!("Could not load configuration '%'.", path)));
            }
            oc.relocate_files(&path);
            if verbose {
                progress_done_message();
            }
        }
        if my_args.len() > 2 {
            // reparse the options (overwrite the settings from the configuration file)
            oc.reset_writable();
            if !options_parser::parse_with(oc, &my_args, false) {
                return Err(ProcessError::new("Could not parse commandline options."));
            }
        }
        Ok(())
    })
}

/// Retrieves the XML root element of a supposed configuration or net
pub fn get_root(filename: &str) -> Result<String, ProcessError> {
    if !file_helpers::is_readable(filename) || file_helpers::is_directory(filename)? {
        return Err(ProcessError::new(tlf!("Could not open '%'.", filename)));
    }
    let mut handler = OptionsLoader::new(false);
    with_options(|oc| handler.parse_file(oc, filename, true))
        .map_err(|e| ProcessError::new(format!("Could not load '{filename}':\n {e}")))?;
    if handler.error_occurred() {
        return Err(ProcessError::new(tlf!("Could not load '%'.", filename)));
    }
    Ok(handler.get_item().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn args_handling() {
        // only touches ARGS, no other test uses it
        set_args_argv(&["prog".into(), "-v".into()]);
        assert_eq!(get_arg_c(), 2);
        set_args(&["-a".into()]);
        assert_eq!(args(), ["prog", "-a"]);
    }
}
