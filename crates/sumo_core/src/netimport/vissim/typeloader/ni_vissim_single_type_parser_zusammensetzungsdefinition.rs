//! Port of netimport/vissim/typeloader/NIVissimSingleTypeParser_Zusammensetzungsdefinition.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `NIVissimSingleTypeParser_Zusammensetzungsdefinition` (netimport/vissim/typeloader/NIVissimSingleTypeParser_Zusammensetzungsdefinition.h)
pub struct NIVissimSingleTypeParser_Zusammensetzungsdefinition {
    /// C++ base class `NIImporter_Vissim::VissimSingleTypeParser`
    pub base: NIImporter_Vissim_VissimSingleTypeParser,
}

impl std::ops::Deref for NIVissimSingleTypeParser_Zusammensetzungsdefinition {
    type Target = NIImporter_Vissim_VissimSingleTypeParser;
    fn deref(&self) -> &NIImporter_Vissim_VissimSingleTypeParser {
        &self.base
    }
}
impl std::ops::DerefMut for NIVissimSingleTypeParser_Zusammensetzungsdefinition {
    fn deref_mut(&mut self) -> &mut NIImporter_Vissim_VissimSingleTypeParser {
        &mut self.base
    }
}

impl NIVissimSingleTypeParser_Zusammensetzungsdefinition {
    /// C++: `void NIVissimSingleTypeParser_Zusammensetzungsdefinition::NIVissimSingleTypeParser_Zusammensetzungsdefinition(NIImporter_Vissim &)`
    /// Constructor
    // NIVissimSingleTypeParser_Zusammensetzungsdefinition::NIVissimSingleTypeParser_Zusammensetzungsdefinition(NIImporter_Vissim& parent)
    //     : NIImporter_Vissim::VissimSingleTypeParser(parent) {}
    #[allow(unused_variables)]
    pub fn new(parent: &mut NIImporter_Vissim) -> Self {
        todo!("port NIVissimSingleTypeParser_Zusammensetzungsdefinition::NIVissimSingleTypeParser_Zusammensetzungsdefinition")
    }

    /// C++: `void NIVissimSingleTypeParser_Zusammensetzungsdefinition::~NIVissimSingleTypeParser_Zusammensetzungsdefinition() noexcept` [virtual]
    /// Destructor
    // NIVissimSingleTypeParser_Zusammensetzungsdefinition::~NIVissimSingleTypeParser_Zusammensetzungsdefinition() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NIVissimSingleTypeParser_Zusammensetzungsdefinition::~NIVissimSingleTypeParser_Zusammensetzungsdefinition")
    }

    /// C++: `bool NIVissimSingleTypeParser_Zusammensetzungsdefinition::parse(std::istream &)` [virtual]
    /// Parses the data type from the given stream
    // bool
    // NIVissimSingleTypeParser_Zusammensetzungsdefinition::parse(std::istream& from) {
    //     std::string tag = myRead(from);
    //     while (tag != "fahrzeugtyp") {
    //         tag = readEndSecure(from, "fahrzeugtyp");
    //     }
    //     do {
    //         tag = myRead(from); // id
    //         tag = myRead(from); // "anteil"
    //         tag = myRead(from); // value
    //         tag = myRead(from); // "VWunsch"
    //         tag = myRead(from); // value
    //         tag = readEndSecure(from, "fahrzeugtyp"); // "fahrzeugtyp"?
    //     } while (tag == "fahrzeugtyp");
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn parse(&mut self, from: &mut Unmapped /* std::istream */) -> bool {
        todo!("port NIVissimSingleTypeParser_Zusammensetzungsdefinition::parse")
    }

}

