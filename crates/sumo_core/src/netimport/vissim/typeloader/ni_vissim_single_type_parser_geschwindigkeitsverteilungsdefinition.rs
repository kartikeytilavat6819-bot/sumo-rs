//! Port of netimport/vissim/typeloader/NIVissimSingleTypeParser_Geschwindigkeitsverteilungsdefinition.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `NIVissimSingleTypeParser_Geschwindigkeitsverteilungsdefinition` (netimport/vissim/typeloader/NIVissimSingleTypeParser_Geschwindigkeitsverteilungsdefinition.h)
#[repr(C)]
pub struct NIVissimSingleTypeParser_Geschwindigkeitsverteilungsdefinition {
    /// C++ base class `NIImporter_Vissim::VissimSingleTypeParser`
    pub base: NIImporter_Vissim_VissimSingleTypeParser,
}

impl std::ops::Deref for NIVissimSingleTypeParser_Geschwindigkeitsverteilungsdefinition {
    type Target = NIImporter_Vissim_VissimSingleTypeParser;
    fn deref(&self) -> &NIImporter_Vissim_VissimSingleTypeParser {
        &self.base
    }
}
impl std::ops::DerefMut for NIVissimSingleTypeParser_Geschwindigkeitsverteilungsdefinition {
    fn deref_mut(&mut self) -> &mut NIImporter_Vissim_VissimSingleTypeParser {
        &mut self.base
    }
}

/// C++ `from >> tag` for a std::string (same as NIImporter_Vissim's private vissim_read_token)
fn vissim_read_token(from: &mut std::io::Cursor<Vec<u8>>) -> String {
    let data = from.get_ref();
    let mut i = from.position() as usize;
    while i < data.len() && matches!(data[i], b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c) {
        i += 1;
    }
    let start = i;
    while i < data.len() && !matches!(data[i], b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c) {
        i += 1;
    }
    let tok = vissim_bytes_to_string(&data[start..i]);
    from.set_position(i as u64);
    tok
}

impl NIVissimSingleTypeParser_Geschwindigkeitsverteilungsdefinition {
    /// C++: `void NIVissimSingleTypeParser_Geschwindigkeitsverteilungsdefinition::NIVissimSingleTypeParser_Geschwindigkeitsverteilungsdefinition(NIImporter_Vissim &)`
    /// Constructor
    // NIVissimSingleTypeParser_Geschwindigkeitsverteilungsdefinition::NIVissimSingleTypeParser_Geschwindigkeitsverteilungsdefinition(NIImporter_Vissim& parent)
    //     : NIImporter_Vissim::VissimSingleTypeParser(parent) {}
    #[allow(unused_variables)]
    pub fn new(parent: &mut NIImporter_Vissim) -> Self {
        Self { base: NIImporter_Vissim_VissimSingleTypeParser::new(parent) }
    }

    /// C++: `void NIVissimSingleTypeParser_Geschwindigkeitsverteilungsdefinition::~NIVissimSingleTypeParser_Geschwindigkeitsverteilungsdefinition() noexcept` [virtual]
    /// Destructor
    // NIVissimSingleTypeParser_Geschwindigkeitsverteilungsdefinition::~NIVissimSingleTypeParser_Geschwindigkeitsverteilungsdefinition() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // destructor: nothing to release explicitly, the base parser drops with the struct
    }

    /// C++: `bool NIVissimSingleTypeParser_Geschwindigkeitsverteilungsdefinition::parse(std::istream &)` [virtual]
    /// Parses the data type from the given stream
    // bool
    // NIVissimSingleTypeParser_Geschwindigkeitsverteilungsdefinition::parse(std::istream& from) {
    //     // id
    //     std::string id;
    //     from >> id;
    //     // list of points
    //     Distribution_Points* points = new Distribution_Points(id);
    //     std::string tag;
    //     do {
    //         tag = readEndSecure(from);
    //         if (tag == "name") {
    //             readName(from);
    //             tag = readEndSecure(from);
    //         }
    //         if (tag != "DATAEND") {
    //             const double p1 = StringUtils::toDouble(tag);
    //             from >> tag;
    //             const double p2 = StringUtils::toDouble(tag);
    //             points->add(p1, p2);
    //         }
    //     } while (tag != "DATAEND");
    //     DistributionCont::dictionary("speed", id, points);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn parse(&mut self, from: &mut std::io::Cursor<Vec<u8>> /* std::istream */) -> bool {
        // id
        let id = vissim_read_token(from);
        // list of points
        let mut points = DistributionPoints::new(&id);
        let mut tag;
        loop {
            tag = self.read_end_secure(from, "");
            if tag == "name" {
                let _ = self.read_name(from);
                tag = self.read_end_secure(from, "");
            }
            if tag != "DATAEND" {
                let p1 = utils::common::string_utils::to_double(&tag).unwrap_or_else(|e| panic!("{e}"));
                tag = vissim_read_token(from);
                let p2 = utils::common::string_utils::to_double(&tag).unwrap_or_else(|e| panic!("{e}"));
                points.add(p1, p2, true);
            }
            if tag == "DATAEND" {
                break;
            }
        }
        utils::distribution::distribution_cont::dictionary_add("speed", &id, std::sync::Arc::new(points));
        true
    }

}

