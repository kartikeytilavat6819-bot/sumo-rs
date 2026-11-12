//! Port of netimport/vissim/typeloader/NIVissimSingleTypeParser_Knotendefinition.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `NIVissimSingleTypeParser_Knotendefinition` (netimport/vissim/typeloader/NIVissimSingleTypeParser_Knotendefinition.h)
#[repr(C)]
pub struct NIVissimSingleTypeParser_Knotendefinition {
    /// C++ base class `NIImporter_Vissim::VissimSingleTypeParser`
    pub base: NIImporter_Vissim_VissimSingleTypeParser,
}

impl std::ops::Deref for NIVissimSingleTypeParser_Knotendefinition {
    type Target = NIImporter_Vissim_VissimSingleTypeParser;
    fn deref(&self) -> &NIImporter_Vissim_VissimSingleTypeParser {
        &self.base
    }
}
impl std::ops::DerefMut for NIVissimSingleTypeParser_Knotendefinition {
    fn deref_mut(&mut self) -> &mut NIImporter_Vissim_VissimSingleTypeParser {
        &mut self.base
    }
}

impl NIVissimSingleTypeParser_Knotendefinition {
    /// C++: `void NIVissimSingleTypeParser_Knotendefinition::NIVissimSingleTypeParser_Knotendefinition(NIImporter_Vissim &)`
    /// Constructor
    // NIVissimSingleTypeParser_Knotendefinition::NIVissimSingleTypeParser_Knotendefinition(NIImporter_Vissim& parent)
    //     : NIImporter_Vissim::VissimSingleTypeParser(parent) {}
    #[allow(unused_variables)]
    pub fn new(parent: &mut NIImporter_Vissim) -> Self {
        todo!("port NIVissimSingleTypeParser_Knotendefinition::NIVissimSingleTypeParser_Knotendefinition")
    }

    /// C++: `void NIVissimSingleTypeParser_Knotendefinition::~NIVissimSingleTypeParser_Knotendefinition() noexcept` [virtual]
    /// Destructor
    // NIVissimSingleTypeParser_Knotendefinition::~NIVissimSingleTypeParser_Knotendefinition() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NIVissimSingleTypeParser_Knotendefinition::~NIVissimSingleTypeParser_Knotendefinition")
    }

    /// C++: `bool NIVissimSingleTypeParser_Knotendefinition::parse(std::istream &)` [virtual]
    /// Parses the data type from the given stream
    // bool
    // NIVissimSingleTypeParser_Knotendefinition::parse(std::istream& from) {
    //     //
    //     int id;
    //     from >> id;
    //     //
    //     std::string tag;
    //     from >> tag;
    //     std::string name = readName(from);
    //     //
    //     tag = overrideOptionalLabel(from);
    //     //
    //     while (tag != "netzausschnitt") {
    //         tag = myRead(from);
    //     }
    //     //
    //     tag = myRead(from);
    //     if (tag == "strecke") {
    //         NIVissimNodeParticipatingEdgeVector edges;
    //         while (tag == "strecke") {
    //             int edgeid;
    //             double from_pos, to_pos;
    //             from_pos = to_pos = -1.0;
    //             from >> edgeid;
    //             tag = readEndSecure(from, "strecke");
    //             if (tag == "von") {
    //                 from >> from_pos; // type-checking is missing!
    //                 from >> tag;
    //                 from >> to_pos; // type-checking is missing!
    //                 tag = readEndSecure(from, "strecke");
    //             }
    //             edges.push_back(new NIVissimNodeParticipatingEdge(edgeid, from_pos, to_pos));
    //         }
    //         NIVissimNodeDef_Edges::dictionary(id, name, edges);
    //     } else {
    //         int no = StringUtils::toInt(tag);
    //         PositionVector poly;
    //         for (int i = 0; i < no; i++) {
    //             poly.push_back(getPosition(from));
    //         }
    //         poly.closePolygon();
    //         NIVissimNodeDef_Poly::dictionary(id, name, poly);
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn parse(&mut self, from: &mut Unmapped /* std::istream */) -> bool {
        todo!("port NIVissimSingleTypeParser_Knotendefinition::parse")
    }

}

