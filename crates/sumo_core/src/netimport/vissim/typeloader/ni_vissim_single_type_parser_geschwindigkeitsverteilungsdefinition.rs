//! Port of netimport/vissim/typeloader/NIVissimSingleTypeParser_Geschwindigkeitsverteilungsdefinition.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `NIVissimSingleTypeParser_Geschwindigkeitsverteilungsdefinition` (netimport/vissim/typeloader/NIVissimSingleTypeParser_Geschwindigkeitsverteilungsdefinition.h)
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

impl NIVissimSingleTypeParser_Geschwindigkeitsverteilungsdefinition {
    /// C++: `void NIVissimSingleTypeParser_Geschwindigkeitsverteilungsdefinition::NIVissimSingleTypeParser_Geschwindigkeitsverteilungsdefinition(NIImporter_Vissim &)`
    /// Constructor
    // NIVissimSingleTypeParser_Geschwindigkeitsverteilungsdefinition::NIVissimSingleTypeParser_Geschwindigkeitsverteilungsdefinition(NIImporter_Vissim& parent)
    //     : NIImporter_Vissim::VissimSingleTypeParser(parent) {}
    #[allow(unused_variables)]
    pub fn new(parent: &mut NIImporter_Vissim) -> Self {
        todo!("port NIVissimSingleTypeParser_Geschwindigkeitsverteilungsdefinition::NIVissimSingleTypeParser_Geschwindigkeitsverteilungsdefinition")
    }

    /// C++: `void NIVissimSingleTypeParser_Geschwindigkeitsverteilungsdefinition::~NIVissimSingleTypeParser_Geschwindigkeitsverteilungsdefinition() noexcept` [virtual]
    /// Destructor
    // NIVissimSingleTypeParser_Geschwindigkeitsverteilungsdefinition::~NIVissimSingleTypeParser_Geschwindigkeitsverteilungsdefinition() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NIVissimSingleTypeParser_Geschwindigkeitsverteilungsdefinition::~NIVissimSingleTypeParser_Geschwindigkeitsverteilungsdefinition")
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
    pub fn parse(&mut self, from: &mut Unmapped /* std::istream */) -> bool {
        todo!("port NIVissimSingleTypeParser_Geschwindigkeitsverteilungsdefinition::parse")
    }

}

