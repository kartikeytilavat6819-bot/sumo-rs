//! Port of netimport/NITypeLoader.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;



/// C++ class `NITypeLoader` (netimport/NITypeLoader.h)
pub struct NITypeLoader {
}

impl NITypeLoader {
    /// C++: `bool NITypeLoader::load(SUMOSAXHandler &, const int &, const std::string &, const bool)` [static]
    /// loads data from the list of xml-files of certain type
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn load(handler: &mut dyn SaxHandler, files: &[String], type_: &str, stringParse: bool) -> bool {
        use utils::common::file_helpers::is_readable;
        use utils::common::msg_handler::{progress_begin_message, progress_done_message, write_error};
        use utils::xml::xml_sub_sys::{get_sax_reader, run_parser};
        let mut exceptMsg = String::new();
        let mut fileName = String::new();
        // build parser and start the parsing
        let mut ok = true;
        let res: Result<Option<bool>, ProcessError> = (|| {
            for file in files {
                fileName = file.clone();
                if stringParse {
                    fileName = "built in type map".into();
                    handler.handler().set_file_name(&fileName);
                    let mut reader = get_sax_reader(false, false);
                    reader.parse_string(handler, file)?;
                    continue;
                }
                if !is_readable(&fileName) {
                    write_error(&format!("Could not open {type_}-file '{fileName}'."));
                    return Ok(Some(false));
                }
                progress_begin_message(&format!("Parsing {type_} from '{fileName}'"));
                ok &= run_parser(handler, &fileName, false, false, false, true)?;
                progress_done_message();
            }
            Ok(None)
        })();
        match res {
            Ok(Some(early)) => return early,
            Ok(None) => {}
            // shortcut: C++ catch(...) also catches non-ProcessError exceptions (e.g. Xerces); panics are not caught here
            Err(e) => {
                exceptMsg = format!("{e}\n  ");
                std::panic::panic_any(ProcessError::new(format!("{exceptMsg}The {type_} could not be loaded from '{fileName}'.")));
            }
        }
        ok
    }

}

