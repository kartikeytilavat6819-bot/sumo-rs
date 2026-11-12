//! Port of netimport/vissim/typeloader/NIVissimSingleTypeParser_Signalgruppendefinition.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `NIVissimSingleTypeParser_Signalgruppendefinition` (netimport/vissim/typeloader/NIVissimSingleTypeParser_Signalgruppendefinition.h)
#[repr(C)]
pub struct NIVissimSingleTypeParser_Signalgruppendefinition {
    /// C++ base class `NIImporter_Vissim::VissimSingleTypeParser`
    pub base: NIImporter_Vissim_VissimSingleTypeParser,
}

impl std::ops::Deref for NIVissimSingleTypeParser_Signalgruppendefinition {
    type Target = NIImporter_Vissim_VissimSingleTypeParser;
    fn deref(&self) -> &NIImporter_Vissim_VissimSingleTypeParser {
        &self.base
    }
}
impl std::ops::DerefMut for NIVissimSingleTypeParser_Signalgruppendefinition {
    fn deref_mut(&mut self) -> &mut NIImporter_Vissim_VissimSingleTypeParser {
        &mut self.base
    }
}

impl NIVissimSingleTypeParser_Signalgruppendefinition {
    /// C++: `void NIVissimSingleTypeParser_Signalgruppendefinition::NIVissimSingleTypeParser_Signalgruppendefinition(NIImporter_Vissim &)`
    /// Constructor
    // NIVissimSingleTypeParser_Signalgruppendefinition::NIVissimSingleTypeParser_Signalgruppendefinition(NIImporter_Vissim& parent)
    //     : NIImporter_Vissim::VissimSingleTypeParser(parent) {}
    #[allow(unused_variables)]
    pub fn new(parent: &mut NIImporter_Vissim) -> Self {
        todo!("port NIVissimSingleTypeParser_Signalgruppendefinition::NIVissimSingleTypeParser_Signalgruppendefinition")
    }

    /// C++: `void NIVissimSingleTypeParser_Signalgruppendefinition::~NIVissimSingleTypeParser_Signalgruppendefinition() noexcept` [virtual]
    /// Destructor
    // NIVissimSingleTypeParser_Signalgruppendefinition::~NIVissimSingleTypeParser_Signalgruppendefinition() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NIVissimSingleTypeParser_Signalgruppendefinition::~NIVissimSingleTypeParser_Signalgruppendefinition")
    }

    /// C++: `bool NIVissimSingleTypeParser_Signalgruppendefinition::parse(std::istream &)` [virtual]
    /// Parses the data type from the given stream
    // bool
    // NIVissimSingleTypeParser_Signalgruppendefinition::parse(std::istream& from) {
    //     //
    //     int id;
    //     from >> id; // type-checking is missing!
    //     //
    //     std::string tag;
    //     tag = myRead(from);
    //     std::string name;
    //     if (tag == "name") {
    //         name = readName(from);
    //         tag = myRead(from);
    //     }
    //     //
    //     int lsaid;
    //     from >> lsaid;
    //     NIVissimTL* tl = NIVissimTL::dictionary(lsaid);
    //     if (tl == nullptr) {
    //         WRITE_ERROR("A traffic light group with an unknown traffic light occurred.\n  Group-ID: " + toString<int>(id)
    //                     + "\n  TrafficLight-ID: " + toString<int>(lsaid));
    //         return false;
    //     }
    //     std::string type = tl->getType();
    //     if (type == "festzeit") {
    //         return parseFixedTime(id, name, lsaid, from);
    //     }
    //     if (type == "festzeit_fake") {
    //         return parseFixedTime(id, name, lsaid, from);
    // //        return parseExternFixedTime(id, name, lsaid, from);
    //     }
    //     if (type == "vas") {
    //         return parseVAS(id, name, lsaid, from);
    //     }
    //     if (type == "vsplus") {
    //         return parseVSPLUS(id, name, lsaid, from);
    //     }
    //     if (type == "trends") {
    //         return parseTRENDS(id, name, lsaid, from);
    //     }
    //     if (type == "vap") {
    //         return parseVAP(id, name, lsaid, from);
    //     }
    //     if (type == "tl") {
    //         return parseTL(id, name, lsaid, from);
    //     }
    //     if (type == "pos") {
    //         return parsePOS(id, name, lsaid, from);
    //     }
    //     WRITE_WARNINGF(TL("Unsupported LSA-Type '%' occurred."), type);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn parse(&mut self, from: &mut Unmapped /* std::istream */) -> bool {
        todo!("port NIVissimSingleTypeParser_Signalgruppendefinition::parse")
    }

    /// C++: `bool NIVissimSingleTypeParser_Signalgruppendefinition::parseFixedTime(int, const std::string &, int, std::istream &)` [private]
    /// Parses a traffic light group which belongs to a fixed time traffic light
    // bool
    // NIVissimSingleTypeParser_Signalgruppendefinition::parseFixedTime(
    //     int id, const std::string& name, int lsaid, std::istream& from) {
    //     //
    //     bool isGreenBegin;
    //     std::vector<SUMOTime> times;
    //     std::string tag = myRead(from);
    //     if (tag == "dauergruen") {
    //         isGreenBegin = true;
    //         from >> tag;
    //     } else if (tag == "dauerrot") {
    //         isGreenBegin = false;
    //         from >> tag;
    //     } else {
    //         // the first phase will be green
    //         isGreenBegin = true;
    //         while (tag == "rotende" || tag == "gruenanfang") {
    //             double point;
    //             from >> point; // type-checking is missing!
    //             times.push_back(TIME2STEPS(point));
    //             from >> tag;
    //             from >> point; // type-checking is missing!
    //             times.push_back(TIME2STEPS(point));
    //             tag = myRead(from);
    //         }
    //     }
    //     //
    //     double tredyellow, tyellow;
    //     from >> tredyellow;
    //     from >> tag;
    //     from >> tyellow;
    //     NIVissimTL::NIVissimTLSignalGroup* group =
    //         new NIVissimTL::NIVissimTLSignalGroup(
    //         id, name, isGreenBegin, times, TIME2STEPS(tredyellow), TIME2STEPS(tyellow));
    //     if (!NIVissimTL::NIVissimTLSignalGroup::dictionary(lsaid, id, group)) {
    //         throw 1; // !!!
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn parse_fixed_time(&mut self, id: i32, name: &str, lsaid: i32, from: &mut Unmapped /* std::istream */) -> bool {
        todo!("port NIVissimSingleTypeParser_Signalgruppendefinition::parseFixedTime")
    }

    /// C++: `bool NIVissimSingleTypeParser_Signalgruppendefinition::parseVAS(int, const std::string &, int, std::istream &)` [private]
    /// Parses a traffic light group which belongs to a VAS traffic light
    // bool
    // NIVissimSingleTypeParser_Signalgruppendefinition::parseVAS(
    //     int /*id*/, const std::string& /*name*/, int lsaid, std::istream& from) {
    //     WRITE_WARNINGF(TL("VAS traffic lights are not supported (lsa=%)"), toString<int>(lsaid));
    //     std::string tag;
    //     while (tag != "detektoren") {
    //         tag = myRead(from);
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn parse_vas(&mut self, id: i32, name: &str, lsaid: i32, from: &mut Unmapped /* std::istream */) -> bool {
        todo!("port NIVissimSingleTypeParser_Signalgruppendefinition::parseVAS")
    }

    /// C++: `bool NIVissimSingleTypeParser_Signalgruppendefinition::parseVSPLUS(int, const std::string &, int, std::istream &)` [private]
    /// Parses a traffic light group which belongs to a VSPLUS traffic light
    // bool
    // NIVissimSingleTypeParser_Signalgruppendefinition::parseVSPLUS(
    //     int /*id*/, const std::string&, int lsaid, std::istream&) {
    //     WRITE_WARNINGF(TL("VSPLUS traffic lights are not supported (lsa=%)"), toString<int>(lsaid));
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn parse_vsplus(&mut self, id: i32, name: &str, lsaid: i32, from: &mut Unmapped /* std::istream */) -> bool {
        todo!("port NIVissimSingleTypeParser_Signalgruppendefinition::parseVSPLUS")
    }

    /// C++: `bool NIVissimSingleTypeParser_Signalgruppendefinition::parseTRENDS(int, const std::string &, int, std::istream &)` [private]
    /// Parses a traffic light group which belongs to a TRENDS traffic light
    // bool
    // NIVissimSingleTypeParser_Signalgruppendefinition::parseTRENDS(
    //     int /*id*/, const std::string&, int lsaid, std::istream&) {
    //     WRITE_WARNINGF(TL("TRENDS traffic lights are not supported (lsa=%)"), toString<int>(lsaid));
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn parse_trends(&mut self, id: i32, name: &str, lsaid: i32, from: &mut Unmapped /* std::istream */) -> bool {
        todo!("port NIVissimSingleTypeParser_Signalgruppendefinition::parseTRENDS")
    }

    /// C++: `bool NIVissimSingleTypeParser_Signalgruppendefinition::parseVAP(int, const std::string &, int, std::istream &)` [private]
    /// Parses a traffic light group which belongs to a VAStraffic light
    // bool
    // NIVissimSingleTypeParser_Signalgruppendefinition::parseVAP(
    //     int /*id*/, const std::string&, int lsaid, std::istream&) {
    //     WRITE_WARNINGF(TL("VAS traffic lights are not supported (lsa=%)"), toString<int>(lsaid));
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn parse_vap(&mut self, id: i32, name: &str, lsaid: i32, from: &mut Unmapped /* std::istream */) -> bool {
        todo!("port NIVissimSingleTypeParser_Signalgruppendefinition::parseVAP")
    }

    /// C++: `bool NIVissimSingleTypeParser_Signalgruppendefinition::parseTL(int, const std::string &, int, std::istream &)` [private]
    /// Parses a traffic light group which belongs to a TL traffic light
    // bool
    // NIVissimSingleTypeParser_Signalgruppendefinition::parseTL(
    //     int /*id*/, const std::string&, int lsaid, std::istream&) {
    //     WRITE_WARNINGF(TL("TL traffic lights are not supported (lsa=%)"), toString<int>(lsaid));
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn parse_tl(&mut self, id: i32, name: &str, lsaid: i32, from: &mut Unmapped /* std::istream */) -> bool {
        todo!("port NIVissimSingleTypeParser_Signalgruppendefinition::parseTL")
    }

    /// C++: `bool NIVissimSingleTypeParser_Signalgruppendefinition::parsePOS(int, const std::string &, int, std::istream &)` [private]
    /// Parses a traffic light group which belongs to a POS traffic light
    // bool
    // NIVissimSingleTypeParser_Signalgruppendefinition::parsePOS(
    //     int /*id*/, const std::string&, int lsaid, std::istream&) {
    //     WRITE_WARNINGF(TL("POS traffic lights are not supported (lsa=%)"), toString<int>(lsaid));
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn parse_pos(&mut self, id: i32, name: &str, lsaid: i32, from: &mut Unmapped /* std::istream */) -> bool {
        todo!("port NIVissimSingleTypeParser_Signalgruppendefinition::parsePOS")
    }

    /// C++: `bool NIVissimSingleTypeParser_Signalgruppendefinition::parseExternFixedTime(int, const std::string &, int, std::istream &)` [private]
    /// Parses a traffic light group which belongs fixed time traffic light with an extern definition
    // bool
    // NIVissimSingleTypeParser_Signalgruppendefinition::parseExternFixedTime(
    //     int /*id*/, const std::string&, int lsaid, std::istream&) {
    //     WRITE_WARNINGF(TL("externally defined traffic lights are not supported (lsa=%)"), toString<int>(lsaid));
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn parse_extern_fixed_time(&mut self, id: i32, name: &str, lsaid: i32, from: &mut Unmapped /* std::istream */) -> bool {
        todo!("port NIVissimSingleTypeParser_Signalgruppendefinition::parseExternFixedTime")
    }

}

