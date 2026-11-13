//! Port of netimport/vissim/typeloader/NIVissimSingleTypeParser_Signalgeberdefinition.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ `from >> tmp` for a std::string: skips whitespace and reads one token without lower-casing it
fn read_raw_token(from: &mut std::io::Cursor<Vec<u8>>) -> String {
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

/// C++ `from >> int`: a failed conversion yields 0 (C++11 semantics)
fn read_int(from: &mut std::io::Cursor<Vec<u8>>) -> i32 {
    read_raw_token(from).parse::<i32>().unwrap_or(0)
}

/// C++ `from >> double`: a failed conversion yields 0 (C++11 semantics)
fn read_double(from: &mut std::io::Cursor<Vec<u8>>) -> f64 {
    read_raw_token(from).parse::<f64>().unwrap_or(0.)
}

/// C++ class `NIVissimSingleTypeParser_Signalgeberdefinition` (netimport/vissim/typeloader/NIVissimSingleTypeParser_Signalgeberdefinition.h)
#[repr(C)]
pub struct NIVissimSingleTypeParser_Signalgeberdefinition {
    /// C++ base class `NIImporter_Vissim::VissimSingleTypeParser`
    pub base: NIImporter_Vissim_VissimSingleTypeParser,
}

impl std::ops::Deref for NIVissimSingleTypeParser_Signalgeberdefinition {
    type Target = NIImporter_Vissim_VissimSingleTypeParser;
    fn deref(&self) -> &NIImporter_Vissim_VissimSingleTypeParser {
        &self.base
    }
}
impl std::ops::DerefMut for NIVissimSingleTypeParser_Signalgeberdefinition {
    fn deref_mut(&mut self) -> &mut NIImporter_Vissim_VissimSingleTypeParser {
        &mut self.base
    }
}

impl NIVissimSingleTypeParser_Signalgeberdefinition {
    /// C++: `void NIVissimSingleTypeParser_Signalgeberdefinition::NIVissimSingleTypeParser_Signalgeberdefinition(NIImporter_Vissim &)`
    /// Constructor
    // NIVissimSingleTypeParser_Signalgeberdefinition::NIVissimSingleTypeParser_Signalgeberdefinition(NIImporter_Vissim& parent)
    //     : NIImporter_Vissim::VissimSingleTypeParser(parent) {}
    #[allow(unused_variables)]
    pub fn new(parent: &mut NIImporter_Vissim) -> Self {
        Self { base: NIImporter_Vissim_VissimSingleTypeParser::new(parent) }
    }

    /// C++: `void NIVissimSingleTypeParser_Signalgeberdefinition::~NIVissimSingleTypeParser_Signalgeberdefinition() noexcept` [virtual]
    /// Destructor
    // NIVissimSingleTypeParser_Signalgeberdefinition::~NIVissimSingleTypeParser_Signalgeberdefinition() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `bool NIVissimSingleTypeParser_Signalgeberdefinition::parse(std::istream &)` [virtual]
    /// Parses the data type from the given stream
    // bool
    // NIVissimSingleTypeParser_Signalgeberdefinition::parse(std::istream& from) {
    //     //
    //     int id;
    //     from >> id;
    //     //
    //     std::string tag, name;
    //     tag = myRead(from);
    //     if (tag == "name") {
    //         name = readName(from);
    //         tag = myRead(from);
    //     }
    //     // skip optional "Beschriftung"
    //     tag = overrideOptionalLabel(from, tag);
    //     //
    //     int lsaid;
    //     std::vector<int> groupids;
    //     if (tag == "lsa") {
    //         int groupid;
    //         from >> lsaid; // type-checking is missing!
    //         from >> tag; // "Gruppe"
    //         do {
    //             from >> groupid;
    //             groupids.push_back(groupid);
    //             tag = myRead(from);
    //         } while (tag == "oder");
    //         //
    //     } else {
    //         from >> tag; // strecke
    //         WRITE_WARNING(TL("Omitting unknown traffic light."));
    //         return true;
    //     }
    //     if (tag == "typ") {
    //         from >> tag; // typ-value
    //         from >> tag; // "ort"
    //     }
    //
    //     //
    //     from >> tag;
    //     int edgeid;
    //     from >> edgeid;
    //
    //     from >> tag;
    //     int laneno;
    //     from >> laneno;
    //
    //     from >> tag;
    //     double position;
    //     from >> position;
    //     //
    //     while (tag != "fahrzeugklassen") {
    //         tag = myRead(from);
    //     }
    //     std::vector<int> assignedVehicleTypes = parseAssignedVehicleTypes(from, "N/A");
    //     //
    //     NIVissimTL::dictionary(lsaid); // !!! check whether someting is really done here
    //     NIVissimTL::NIVissimTLSignal* signal =
    //         new NIVissimTL::NIVissimTLSignal(id, name, groupids, edgeid,
    //                                          laneno, position, assignedVehicleTypes);
    //     if (!NIVissimTL::NIVissimTLSignal::dictionary(lsaid, id, signal)) {
    //         throw 1; // !!!
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn parse(&mut self, from: &mut std::io::Cursor<Vec<u8>> /* std::istream */) -> bool {
        //
        let id = read_int(from);
        //
        let mut tag = self.my_read(from);
        let mut name = String::new();
        if tag == "name" {
            name = self.read_name(from);
            tag = self.my_read(from);
        }
        // skip optional "Beschriftung"
        tag = self.override_optional_label(from, &tag);
        //
        let mut lsaid: i32 = 0;
        let mut groupids: Vec<i32> = Vec::new();
        if tag == "lsa" {
            lsaid = read_int(from); // type-checking is missing!
            read_raw_token(from); // "Gruppe"
            loop {
                let groupid = read_int(from);
                groupids.push(groupid);
                tag = self.my_read(from);
                if tag != "oder" {
                    break;
                }
            }
            //
        } else {
            read_raw_token(from); // strecke
            utils::common::msg_handler::write_warning("Omitting unknown traffic light.");
            return true;
        }
        if tag == "typ" {
            read_raw_token(from); // typ-value
            read_raw_token(from); // "ort"
        }

        //
        read_raw_token(from);
        let edgeid = read_int(from);

        read_raw_token(from);
        let laneno = read_int(from);

        read_raw_token(from);
        let position = read_double(from);
        //
        while tag != "fahrzeugklassen" {
            tag = self.my_read(from);
        }
        let assignedVehicleTypes = self.parse_assigned_vehicle_types(from, "N/A");
        //
        NIVissimTL::dictionary_3(lsaid); // !!! check whether someting is really done here
        let signal = Box::leak(Box::new(NIVissimTL_NIVissimTLSignal::new(id, &name, &groupids, edgeid, laneno, position, &assignedVehicleTypes)));
        if !NIVissimTL_NIVissimTLSignal::dictionary(lsaid, id, Ptr::from_mut(signal)) {
            std::panic::panic_any(ProcessError::new("")); // !!!
        }
        true
    }

}

