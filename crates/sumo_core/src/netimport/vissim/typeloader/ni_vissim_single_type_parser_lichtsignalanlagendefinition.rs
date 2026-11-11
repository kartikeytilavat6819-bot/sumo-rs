//! Port of netimport/vissim/typeloader/NIVissimSingleTypeParser_Lichtsignalanlagendefinition.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `NIVissimSingleTypeParser_Lichtsignalanlagendefinition` (netimport/vissim/typeloader/NIVissimSingleTypeParser_Lichtsignalanlagendefinition.h)
pub struct NIVissimSingleTypeParser_Lichtsignalanlagendefinition {
    /// C++ base class `NIImporter_Vissim::VissimSingleTypeParser`
    pub base: NIImporter_Vissim_VissimSingleTypeParser,
}

impl std::ops::Deref for NIVissimSingleTypeParser_Lichtsignalanlagendefinition {
    type Target = NIImporter_Vissim_VissimSingleTypeParser;
    fn deref(&self) -> &NIImporter_Vissim_VissimSingleTypeParser {
        &self.base
    }
}
impl std::ops::DerefMut for NIVissimSingleTypeParser_Lichtsignalanlagendefinition {
    fn deref_mut(&mut self) -> &mut NIImporter_Vissim_VissimSingleTypeParser {
        &mut self.base
    }
}

impl NIVissimSingleTypeParser_Lichtsignalanlagendefinition {
    /// C++: `void NIVissimSingleTypeParser_Lichtsignalanlagendefinition::NIVissimSingleTypeParser_Lichtsignalanlagendefinition(NIImporter_Vissim &)`
    /// Constructor
    // NIVissimSingleTypeParser_Lichtsignalanlagendefinition::NIVissimSingleTypeParser_Lichtsignalanlagendefinition(NIImporter_Vissim& parent)
    //     : NIImporter_Vissim::VissimSingleTypeParser(parent) {}
    #[allow(unused_variables)]
    pub fn new(parent: &mut NIImporter_Vissim) -> Self {
        todo!("port NIVissimSingleTypeParser_Lichtsignalanlagendefinition::NIVissimSingleTypeParser_Lichtsignalanlagendefinition")
    }

    /// C++: `void NIVissimSingleTypeParser_Lichtsignalanlagendefinition::~NIVissimSingleTypeParser_Lichtsignalanlagendefinition() noexcept` [virtual]
    /// Destructor
    // NIVissimSingleTypeParser_Lichtsignalanlagendefinition::~NIVissimSingleTypeParser_Lichtsignalanlagendefinition() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NIVissimSingleTypeParser_Lichtsignalanlagendefinition::~NIVissimSingleTypeParser_Lichtsignalanlagendefinition")
    }

    /// C++: `bool NIVissimSingleTypeParser_Lichtsignalanlagendefinition::parse(std::istream &)` [virtual]
    /// Parses the data type from the given stream
    // bool
    // NIVissimSingleTypeParser_Lichtsignalanlagendefinition::parse(std::istream& from) {
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
    //     // type
    //     std::string type;
    //     type = myRead(from);
    //     if (type == "festzeit") {
    //         return parseFixedTime(id, name, from);
    //     }
    //     if (type == "vas") {
    //         return parseVAS(id, name, from);
    //     }
    //     if (type == "vsplus") {
    //         return parseRestActuated(id, name, from, type);
    //     }
    //     if (type == "trends") {
    //         return parseRestActuated(id, name, from, type);
    //     }
    //     if (type == "vap") {
    //         return parseRestActuated(id, name, from, type);
    //     }
    //     if (type == "tl") {
    //         return parseRestActuated(id, name, from, type);
    //     }
    //     if (type == "pos") {
    //         return parseRestActuated(id, name, from, type);
    //     }
    //     if (type == "nema") {
    //         return parseRestActuated(id, name, from, type);
    //     }
    //     if (type == "extern") {
    //         return parseRestActuated(id, name, from, type);
    //     }
    //     WRITE_ERRORF(TL("Unsupported LSA-Type '%' occurred."), type);
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn parse(&mut self, from: &mut Unmapped /* std::istream */) -> bool {
        todo!("port NIVissimSingleTypeParser_Lichtsignalanlagendefinition::parse")
    }

    /// C++: `bool NIVissimSingleTypeParser_Lichtsignalanlagendefinition::parseFixedTime(int, std::string, std::istream &)` [private]
    /// parses a traffic light with fixed times (no other types are supported by now)
    // bool
    // NIVissimSingleTypeParser_Lichtsignalanlagendefinition::parseFixedTime(
    //     int id, std::string name, std::istream& from) {
    //     std::string type = "festzeit";
    //     std::string tag;
    //     from >> tag;
    //     //
    //     double absdur;
    //     from >> absdur; // type-checking is missing!
    //     //
    //     tag = readEndSecure(from);
    //     double offset = 0;
    //     if (tag == "versatz") {
    //         from >> offset; // type-checking is missing!
    //     }
    //     if (tag != "szpkonfdatei" && tag != "DATAEND" && tag != "progdatei") {
    //         tag = readEndSecure(from);
    //         if (tag == "szpkonfdatei" || tag == "progdatei") {
    //             type = "festzeit_fake";
    //         }
    //     }
    //     return NIVissimTL::dictionary(id, type, name, TIME2STEPS(absdur), TIME2STEPS(offset));
    // }
    #[allow(unused_variables)]
    pub fn parse_fixed_time(&mut self, id: i32, name: String, from: &mut Unmapped /* std::istream */) -> bool {
        todo!("port NIVissimSingleTypeParser_Lichtsignalanlagendefinition::parseFixedTime")
    }

    /// C++: `bool NIVissimSingleTypeParser_Lichtsignalanlagendefinition::parseVAS(int, std::string, std::istream &)` [private]
    /// Parses a vas-traffic light; All other actuated traffic lights are parsed using "parseRestActuated" as they have a different format
    // bool
    // NIVissimSingleTypeParser_Lichtsignalanlagendefinition::parseVAS(
    //     int id, std::string name, std::istream& from) {
    //     std::string tag;
    //     from >> tag;
    //     //
    //     double absdur;
    //     from >> absdur; // type-checking is missing!
    //     //
    //     tag = readEndSecure(from);
    //     double offset = 0;
    //     if (tag == "versatz") {
    //         from >> offset; // type-checking is missing!
    //     }
    //     return NIVissimTL::dictionary(id, "vas", name, TIME2STEPS(absdur), TIME2STEPS(offset));
    // }
    #[allow(unused_variables)]
    pub fn parse_vas(&mut self, id: i32, name: String, from: &mut Unmapped /* std::istream */) -> bool {
        todo!("port NIVissimSingleTypeParser_Lichtsignalanlagendefinition::parseVAS")
    }

    /// C++: `bool NIVissimSingleTypeParser_Lichtsignalanlagendefinition::parseRestActuated(int, std::string, std::istream &, const std::string &)` [private]
    /// Parses actuated traffic lights (beside VAS)
    // bool
    // NIVissimSingleTypeParser_Lichtsignalanlagendefinition::parseRestActuated(
    //     int id, std::string name, std::istream& from, const std::string& type) {
    //     std::string tag;
    //     from >> tag;
    //     //
    //     double absdur;
    //     from >> absdur; // type-checking is missing!
    //     //
    //     tag = readEndSecure(from);
    //     double offset = 0;
    //     if (tag == "versatz") {
    //         from >> offset; // type-checking is missing!
    //     }
    //     while (tag != "datei") {
    //         tag = myRead(from);
    //     }
    //     return NIVissimTL::dictionary(id, type, name, TIME2STEPS(absdur), TIME2STEPS(offset));
    // }
    #[allow(unused_variables)]
    pub fn parse_rest_actuated(&mut self, id: i32, name: String, from: &mut Unmapped /* std::istream */, type_: &str) -> bool {
        todo!("port NIVissimSingleTypeParser_Lichtsignalanlagendefinition::parseRestActuated")
    }

}

