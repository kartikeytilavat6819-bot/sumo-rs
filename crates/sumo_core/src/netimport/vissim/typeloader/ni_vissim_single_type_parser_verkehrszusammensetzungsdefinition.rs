//! Port of netimport/vissim/typeloader/NIVissimSingleTypeParser_Verkehrszusammensetzungsdefinition.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `NIVissimSingleTypeParser_Verkehrszusammensetzungsdefinition` (netimport/vissim/typeloader/NIVissimSingleTypeParser_Verkehrszusammensetzungsdefinition.h)
#[repr(C)]
pub struct NIVissimSingleTypeParser_Verkehrszusammensetzungsdefinition {
    /// C++ base class `NIImporter_Vissim::VissimSingleTypeParser`
    pub base: NIImporter_Vissim_VissimSingleTypeParser,
}

impl std::ops::Deref for NIVissimSingleTypeParser_Verkehrszusammensetzungsdefinition {
    type Target = NIImporter_Vissim_VissimSingleTypeParser;
    fn deref(&self) -> &NIImporter_Vissim_VissimSingleTypeParser {
        &self.base
    }
}
impl std::ops::DerefMut for NIVissimSingleTypeParser_Verkehrszusammensetzungsdefinition {
    fn deref_mut(&mut self) -> &mut NIImporter_Vissim_VissimSingleTypeParser {
        &mut self.base
    }
}

impl NIVissimSingleTypeParser_Verkehrszusammensetzungsdefinition {
    /// C++: `void NIVissimSingleTypeParser_Verkehrszusammensetzungsdefinition::NIVissimSingleTypeParser_Verkehrszusammensetzungsdefinition(NIImporter_Vissim &)`
    /// Constructor
    // NIVissimSingleTypeParser_Verkehrszusammensetzungsdefinition::NIVissimSingleTypeParser_Verkehrszusammensetzungsdefinition(NIImporter_Vissim& parent)
    //     : NIImporter_Vissim::VissimSingleTypeParser(parent) {}
    #[allow(unused_variables)]
    pub fn new(parent: &mut NIImporter_Vissim) -> Self {
        Self { base: NIImporter_Vissim_VissimSingleTypeParser::new(parent) }
    }

    /// C++: `void NIVissimSingleTypeParser_Verkehrszusammensetzungsdefinition::~NIVissimSingleTypeParser_Verkehrszusammensetzungsdefinition() noexcept` [virtual]
    /// Destructor
    // NIVissimSingleTypeParser_Verkehrszusammensetzungsdefinition::~NIVissimSingleTypeParser_Verkehrszusammensetzungsdefinition() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `bool NIVissimSingleTypeParser_Verkehrszusammensetzungsdefinition::parse(std::istream &)` [virtual]
    /// Parses the data type from the given stream
    // bool
    // NIVissimSingleTypeParser_Verkehrszusammensetzungsdefinition::parse(std::istream& from) {
    //     // id
    //     int id;
    //     from >> id; // type-checking is missing!
    //     // name
    //     std::string tag;
    //     from >> tag;
    //     std::string name = readName(from);
    //     // assigned vehicle classes
    //     tag = myRead(from);
    //     if (tag == "temperatur") {
    //         tag = myRead(from);
    //         tag = myRead(from);
    //     }
    //     NIVissimVehicleClassVector assignedTypes;
    //     while (tag != "DATAEND") {
    //         int type;
    //         from >> type;
    //         double percentage;
    //         from >> tag;
    //         from >> percentage;
    //         int vwish;
    //         from >> tag;
    //         from >> vwish;
    //         assignedTypes.push_back(new NIVissimVehicleClass(vwish));
    //         tag = readEndSecure(from, "Fahrzeugtyp");
    //     }
    //     //
    //     return NIVissimTrafficDescription::dictionary(id, name, assignedTypes);
    // }
    #[allow(unused_variables)]
    pub fn parse(&mut self, from: &mut std::io::Cursor<Vec<u8>> /* std::istream */) -> bool {
        // id
        let id = read_i32(from); // type-checking is missing!
        // name
        read_token(from);
        let name = self.read_name(from);
        // assigned vehicle classes
        let mut tag = self.my_read(from);
        if tag == "temperatur" {
            tag = self.my_read(from);
            tag = self.my_read(from);
        }
        let mut assignedTypes: Vec<Ptr<NIVissimVehicleClass>> = Vec::new();
        while tag != "DATAEND" {
            let _type = read_i32(from);
            tag = read_token(from);
            let _percentage = read_f64(from);
            tag = read_token(from);
            let vwish = read_i32(from);
            // shortcut: the vehicle class is owned by the traffic description for the rest of the run (C++ never frees it), so leak the Box
            assignedTypes.push(Ptr::from_mut(Box::leak(Box::new(NIVissimVehicleClass::new(vwish)))));
            tag = self.read_end_secure(from, "Fahrzeugtyp");
        }
        //
        NIVissimTrafficDescription::dictionary(id, &name, &assignedTypes)
    }

}

/// C++ `from >> tag` for a std::string: skips whitespace, reads one whitespace-delimited token ("" at end of stream)
fn read_token(from: &mut std::io::Cursor<Vec<u8>>) -> String {
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

/// C++ `from >> int`: a token that does not parse gives 0 (C++11 behaviour)
// shortcut: numbers are read as whitespace-delimited tokens, C++ `>>` stops at the first character that does not fit
fn read_i32(from: &mut std::io::Cursor<Vec<u8>>) -> i32 {
    read_token(from).parse::<i32>().unwrap_or(0)
}

/// C++ `from >> double` (see read_i32)
fn read_f64(from: &mut std::io::Cursor<Vec<u8>>) -> f64 {
    read_token(from).parse::<f64>().unwrap_or(0.0)
}

