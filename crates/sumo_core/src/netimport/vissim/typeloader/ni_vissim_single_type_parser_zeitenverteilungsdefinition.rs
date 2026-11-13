//! Port of netimport/vissim/typeloader/NIVissimSingleTypeParser_Zeitenverteilungsdefinition.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `NIVissimSingleTypeParser_Zeitenverteilungsdefinition` (netimport/vissim/typeloader/NIVissimSingleTypeParser_Zeitenverteilungsdefinition.h)
#[repr(C)]
pub struct NIVissimSingleTypeParser_Zeitenverteilungsdefinition {
    /// C++ base class `NIImporter_Vissim::VissimSingleTypeParser`
    pub base: NIImporter_Vissim_VissimSingleTypeParser,
}

impl std::ops::Deref for NIVissimSingleTypeParser_Zeitenverteilungsdefinition {
    type Target = NIImporter_Vissim_VissimSingleTypeParser;
    fn deref(&self) -> &NIImporter_Vissim_VissimSingleTypeParser {
        &self.base
    }
}
impl std::ops::DerefMut for NIVissimSingleTypeParser_Zeitenverteilungsdefinition {
    fn deref_mut(&mut self) -> &mut NIImporter_Vissim_VissimSingleTypeParser {
        &mut self.base
    }
}

impl NIVissimSingleTypeParser_Zeitenverteilungsdefinition {
    /// C++: `void NIVissimSingleTypeParser_Zeitenverteilungsdefinition::NIVissimSingleTypeParser_Zeitenverteilungsdefinition(NIImporter_Vissim &)`
    /// Constructor
    // NIVissimSingleTypeParser_Zeitenverteilungsdefinition::NIVissimSingleTypeParser_Zeitenverteilungsdefinition(NIImporter_Vissim& parent)
    //     : NIImporter_Vissim::VissimSingleTypeParser(parent) {}
    #[allow(unused_variables)]
    pub fn new(parent: &mut NIImporter_Vissim) -> Self {
        Self { base: NIImporter_Vissim_VissimSingleTypeParser::new(parent) }
    }

    /// C++: `void NIVissimSingleTypeParser_Zeitenverteilungsdefinition::~NIVissimSingleTypeParser_Zeitenverteilungsdefinition() noexcept` [virtual]
    /// Destructor
    // NIVissimSingleTypeParser_Zeitenverteilungsdefinition::~NIVissimSingleTypeParser_Zeitenverteilungsdefinition() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `bool NIVissimSingleTypeParser_Zeitenverteilungsdefinition::parse(std::istream &)` [virtual]
    /// Parses the data type from the given stream
    // bool
    // NIVissimSingleTypeParser_Zeitenverteilungsdefinition::parse(std::istream& from) {
    //     // id
    //     std::string id;
    //     from >> id;
    //     // list of points
    //     Distribution_Points* points = new Distribution_Points(id);
    //     std::string tag;
    //     do {
    //         tag = readEndSecure(from);
    //         if (tag == "mittelwert") {
    //             double mean, deviation;
    //             from >> mean;
    //             from >> tag;
    //             from >> deviation;
    //             delete points;
    //             return DistributionCont::dictionary("times", id,
    //                                                 new Distribution_Parameterized(id, mean, deviation));
    //         }
    //         if (tag != "DATAEND") {
    //             double p1 = StringUtils::toDouble(tag);
    //             from >> tag;
    //             double p2 = StringUtils::toDouble(tag);
    //             points->add(p1, p2);
    //         }
    //     } while (tag != "DATAEND");
    //     return DistributionCont::dictionary("times", id, points);
    // }
    #[allow(unused_variables)]
    pub fn parse(&mut self, from: &mut std::io::Cursor<Vec<u8>> /* std::istream */) -> bool {
        // id
        let id: String = vissim_raw_token(from);
        // list of points
        let mut points = utils::distribution::distribution_points::DistributionPoints::new(&id);
        let mut tag: String;
        loop {
            tag = self.read_end_secure(from, "");
            if tag == "mittelwert" {
                let mean = vissim_read_f64(from);
                tag = vissim_raw_token(from);
                let deviation = vissim_read_f64(from);
                return utils::distribution::distribution_cont::dictionary_add(
                    "times",
                    &id,
                    std::sync::Arc::new(utils::distribution::distribution_parameterized::DistributionParameterized::with_mean_deviation(&id, mean, deviation)),
                );
            }
            if tag != "DATAEND" {
                let p1 = utils::common::string_utils::to_double(&tag).unwrap_or_else(|e| panic!("{e}"));
                tag = vissim_raw_token(from);
                let p2 = utils::common::string_utils::to_double(&tag).unwrap_or_else(|e| panic!("{e}"));
                points.add(p1, p2, true);
            }
            if tag == "DATAEND" {
                break;
            }
        }
        utils::distribution::distribution_cont::dictionary_add("times", &id, std::sync::Arc::new(points))
    }

}

/// C++ `from >> tmp` for a std::string: skips whitespace, reads one whitespace-delimited token ("" at end of stream)
fn vissim_raw_token(from: &mut std::io::Cursor<Vec<u8>>) -> String {
    let data = from.get_ref();
    let mut i = from.position() as usize;
    while i < data.len() && matches!(data[i], b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c) {
        i += 1;
    }
    let start = i;
    while i < data.len() && !matches!(data[i], b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c) {
        i += 1;
    }
    let tok = String::from_utf8_lossy(&data[start..i]).into_owned();
    from.set_position(i as u64);
    tok
}

/// C++ `from >> double`: a token that does not parse gives 0 (C++11 behaviour)
// shortcut: numbers are read as whitespace-delimited tokens, C++ `>>` stops at the first character that does not fit
fn vissim_read_f64(from: &mut std::io::Cursor<Vec<u8>>) -> f64 {
    vissim_raw_token(from).parse::<f64>().unwrap_or(0.0)
}

