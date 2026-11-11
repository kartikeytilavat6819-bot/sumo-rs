//! Port of netimport/vissim/typeloader/NIVissimSingleTypeParser_VWunschentscheidungsdefinition.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `NIVissimSingleTypeParser_VWunschentscheidungsdefinition` (netimport/vissim/typeloader/NIVissimSingleTypeParser_VWunschentscheidungsdefinition.h)
pub struct NIVissimSingleTypeParser_VWunschentscheidungsdefinition {
    /// C++ base class `NIImporter_Vissim::VissimSingleTypeParser`
    pub base: NIImporter_Vissim_VissimSingleTypeParser,
}

impl std::ops::Deref for NIVissimSingleTypeParser_VWunschentscheidungsdefinition {
    type Target = NIImporter_Vissim_VissimSingleTypeParser;
    fn deref(&self) -> &NIImporter_Vissim_VissimSingleTypeParser {
        &self.base
    }
}
impl std::ops::DerefMut for NIVissimSingleTypeParser_VWunschentscheidungsdefinition {
    fn deref_mut(&mut self) -> &mut NIImporter_Vissim_VissimSingleTypeParser {
        &mut self.base
    }
}

impl NIVissimSingleTypeParser_VWunschentscheidungsdefinition {
    /// C++: `void NIVissimSingleTypeParser_VWunschentscheidungsdefinition::NIVissimSingleTypeParser_VWunschentscheidungsdefinition(NIImporter_Vissim &)`
    /// Constructor
    // NIVissimSingleTypeParser_VWunschentscheidungsdefinition::NIVissimSingleTypeParser_VWunschentscheidungsdefinition(NIImporter_Vissim& parent)
    //     : NIImporter_Vissim::VissimSingleTypeParser(parent) {}
    #[allow(unused_variables)]
    pub fn new(parent: &mut NIImporter_Vissim) -> Self {
        todo!("port NIVissimSingleTypeParser_VWunschentscheidungsdefinition::NIVissimSingleTypeParser_VWunschentscheidungsdefinition")
    }

    /// C++: `void NIVissimSingleTypeParser_VWunschentscheidungsdefinition::~NIVissimSingleTypeParser_VWunschentscheidungsdefinition() noexcept` [virtual]
    /// Destructor
    // NIVissimSingleTypeParser_VWunschentscheidungsdefinition::~NIVissimSingleTypeParser_VWunschentscheidungsdefinition() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NIVissimSingleTypeParser_VWunschentscheidungsdefinition::~NIVissimSingleTypeParser_VWunschentscheidungsdefinition")
    }

    /// C++: `bool NIVissimSingleTypeParser_VWunschentscheidungsdefinition::parse(std::istream &)` [virtual]
    /// Parses the data type from the given stream
    // bool
    // NIVissimSingleTypeParser_VWunschentscheidungsdefinition::parse(std::istream& from) {
    //     std::string tag;
    //     from >> tag; // id
    //     from >> tag; // name
    //     tag = readName(from);
    //     tag = overrideOptionalLabel(from);
    //     from >> tag; // strecke
    //     std::string edgeid;
    //     from >> edgeid;
    //     from >> tag; // spur
    //     std::string lane;
    //     from >> lane;
    //     from >> tag; // bei
    //     std::string pos;
    //     from >> pos;
    //     from >> tag; // fahrzeugklasse
    //     from >> tag; // <fahrzeugklasse>
    //     from >> tag; // vwunsch
    //     std::string vwunsch;
    //     from >> vwunsch; // vwunsch
    //     std::vector<std::string> tmp;
    //     tmp.push_back("zeit");
    //     tmp.push_back("fahrzeugklasse");
    //     tag = readEndSecure(from, tmp);
    //     while (tag != "DATAEND" && tag != "zeit") {
    //         from >> tag;
    //         from >> tag;
    //         from >> tag;
    //         tag = myRead(from);
    //     }
    //     if (tag == "zeit") {
    //         from >> tag;
    //         from >> tag;
    //         from >> tag;
    //         from >> tag;
    //     }
    //     int numid = StringUtils::toInt(edgeid);
    //     int numlane = StringUtils::toInt(lane) - 1;
    //     int numv = StringUtils::toInt(vwunsch);
    //     NIVissimEdge* e = NIVissimEdge::dictionary(numid);
    //     if (e == nullptr) {
    //         NIVissimConnection* c = NIVissimConnection::dictionary(numid);
    //         const std::vector<int>& lanes = c->getToLanes();
    //         e = NIVissimEdge::dictionary(c->getToEdgeID());
    //         for (std::vector<int>::const_iterator j = lanes.begin(); j != lanes.end(); j++) {
    //             e->setSpeed((*j), numv);
    //         }
    //         assert(e != 0);
    //     } else {
    //         e->setSpeed(numlane, numv);
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn parse(&mut self, from: &mut Unmapped /* std::istream */) -> bool {
        todo!("port NIVissimSingleTypeParser_VWunschentscheidungsdefinition::parse")
    }

}

