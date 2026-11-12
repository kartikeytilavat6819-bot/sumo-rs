//! Port of netimport/vissim/typeloader/NIVissimSingleTypeParser_Kantensperrung.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `NIVissimSingleTypeParser_Kantensperrung` (netimport/vissim/typeloader/NIVissimSingleTypeParser_Kantensperrung.h)
#[repr(C)]
pub struct NIVissimSingleTypeParser_Kantensperrung {
    /// C++ base class `NIImporter_Vissim::VissimSingleTypeParser`
    pub base: NIImporter_Vissim_VissimSingleTypeParser,
}

impl std::ops::Deref for NIVissimSingleTypeParser_Kantensperrung {
    type Target = NIImporter_Vissim_VissimSingleTypeParser;
    fn deref(&self) -> &NIImporter_Vissim_VissimSingleTypeParser {
        &self.base
    }
}
impl std::ops::DerefMut for NIVissimSingleTypeParser_Kantensperrung {
    fn deref_mut(&mut self) -> &mut NIImporter_Vissim_VissimSingleTypeParser {
        &mut self.base
    }
}

impl NIVissimSingleTypeParser_Kantensperrung {
    /// C++: `void NIVissimSingleTypeParser_Kantensperrung::NIVissimSingleTypeParser_Kantensperrung(NIImporter_Vissim &)`
    /// Constructor
    // NIVissimSingleTypeParser_Kantensperrung::NIVissimSingleTypeParser_Kantensperrung(NIImporter_Vissim& parent)
    //     : NIImporter_Vissim::VissimSingleTypeParser(parent) {}
    #[allow(unused_variables)]
    pub fn new(parent: &mut NIImporter_Vissim) -> Self {
        todo!("port NIVissimSingleTypeParser_Kantensperrung::NIVissimSingleTypeParser_Kantensperrung")
    }

    /// C++: `void NIVissimSingleTypeParser_Kantensperrung::~NIVissimSingleTypeParser_Kantensperrung() noexcept` [virtual]
    /// Destructor
    // NIVissimSingleTypeParser_Kantensperrung::~NIVissimSingleTypeParser_Kantensperrung() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NIVissimSingleTypeParser_Kantensperrung::~NIVissimSingleTypeParser_Kantensperrung")
    }

    /// C++: `bool NIVissimSingleTypeParser_Kantensperrung::parse(std::istream &)` [virtual]
    /// Parses the data type from the given stream
    // bool
    // NIVissimSingleTypeParser_Kantensperrung::parse(std::istream& from) {
    //     std::string tag;
    //     from >> tag;
    //     //
    //     std::string id;
    //     from >> id;
    //     //
    //     from >> tag;
    //     from >> tag;
    //     int from_node;
    //     from >> from_node;
    //     //
    //     from >> tag;
    //     from >> tag;
    //     int to_node;
    //     from >> to_node;
    //     //
    //     from >> tag;
    //     from >> tag;
    //     std::vector<int> edges;
    //     while (tag != "DATAEND") {
    //         tag = readEndSecure(from);
    //         if (tag != "DATAEND") {
    //             edges.push_back(StringUtils::toInt(tag));
    //         }
    //     }
    //     NIVissimClosures::dictionary(id, from_node, to_node, edges);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn parse(&mut self, from: &mut Unmapped /* std::istream */) -> bool {
        todo!("port NIVissimSingleTypeParser_Kantensperrung::parse")
    }

}

