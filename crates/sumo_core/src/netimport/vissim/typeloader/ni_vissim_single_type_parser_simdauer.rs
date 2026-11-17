//! Port of netimport/vissim/typeloader/NIVissimSingleTypeParser_Simdauer.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::msg_handler::write_error;
use utils::common::string_utils::to_double;

/// C++ class `NIVissimSingleTypeParser_Simdauer` (netimport/vissim/typeloader/NIVissimSingleTypeParser_Simdauer.h)
#[repr(C)]
pub struct NIVissimSingleTypeParser_Simdauer {
    /// C++ base class `NIImporter_Vissim::VissimSingleTypeParser`
    pub base: NIImporter_Vissim_VissimSingleTypeParser,
}

impl std::ops::Deref for NIVissimSingleTypeParser_Simdauer {
    type Target = NIImporter_Vissim_VissimSingleTypeParser;
    fn deref(&self) -> &NIImporter_Vissim_VissimSingleTypeParser {
        &self.base
    }
}
impl std::ops::DerefMut for NIVissimSingleTypeParser_Simdauer {
    fn deref_mut(&mut self) -> &mut NIImporter_Vissim_VissimSingleTypeParser {
        &mut self.base
    }
}

impl NIVissimSingleTypeParser_Simdauer {
    /// C++: `void NIVissimSingleTypeParser_Simdauer::NIVissimSingleTypeParser_Simdauer(NIImporter_Vissim &)`
    /// Constructor
    // NIVissimSingleTypeParser_Simdauer::NIVissimSingleTypeParser_Simdauer(NIImporter_Vissim& parent)
    //     : NIImporter_Vissim::VissimSingleTypeParser(parent) {}
    #[allow(unused_variables)]
    pub fn new(parent: &mut NIImporter_Vissim) -> Self {
        Self { base: NIImporter_Vissim_VissimSingleTypeParser::new(parent) }
    }

    /// C++: `void NIVissimSingleTypeParser_Simdauer::~NIVissimSingleTypeParser_Simdauer() noexcept` [virtual]
    /// Destructor
    // NIVissimSingleTypeParser_Simdauer::~NIVissimSingleTypeParser_Simdauer() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ body is empty; the base destructor runs next.
        self.base.drop_();
    }

    /// C++: `bool NIVissimSingleTypeParser_Simdauer::parse(std::istream &)` [virtual]
    /// Parses the data type from the given stream
    // bool
    // NIVissimSingleTypeParser_Simdauer::parse(std::istream& from) {
    //     std::string duration;
    //     from >> duration;
    //     // !!!
    //     try {
    //         StringUtils::toDouble(duration);
    //     } catch (...) {
    //         WRITE_ERROR(TL("Simulation duration could not be parsed"));
    //         return false;
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn parse(&mut self, from: &mut std::io::Cursor<Vec<u8>> /* std::istream */) -> bool {
        // from >> duration: skip whitespace, read one whitespace-delimited token
        let duration = {
            let data = from.get_ref();
            let mut i = from.position() as usize;
            let is_space = |c: u8| matches!(c, b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c);
            while i < data.len() && is_space(data[i]) {
                i += 1;
            }
            let start = i;
            while i < data.len() && !is_space(data[i]) {
                i += 1;
            }
            let tok = vissim_bytes_to_string(&data[start..i]);
            from.set_position(i as u64);
            tok
        };
        // !!!
        if to_double(&duration).is_err() {
            write_error("Simulation duration could not be parsed");
            return false;
        }
        true
    }

}

