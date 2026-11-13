//! Port of netimport/vissim/typeloader/NIVissimSingleTypeParser_Lichtsignalanlagendefinition.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
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

/// C++ `from >> double`: a failed conversion yields 0 (C++11 semantics)
fn read_double(from: &mut std::io::Cursor<Vec<u8>>) -> f64 {
    read_raw_token(from).parse::<f64>().unwrap_or(0.)
}

/// C++ class `NIVissimSingleTypeParser_Lichtsignalanlagendefinition` (netimport/vissim/typeloader/NIVissimSingleTypeParser_Lichtsignalanlagendefinition.h)
#[repr(C)]
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
        Self { base: NIImporter_Vissim_VissimSingleTypeParser::new(parent) }
    }

    /// C++: `void NIVissimSingleTypeParser_Lichtsignalanlagendefinition::~NIVissimSingleTypeParser_Lichtsignalanlagendefinition() noexcept` [virtual]
    /// Destructor
    // NIVissimSingleTypeParser_Lichtsignalanlagendefinition::~NIVissimSingleTypeParser_Lichtsignalanlagendefinition() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
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
    pub fn parse(&mut self, from: &mut std::io::Cursor<Vec<u8>> /* std::istream */) -> bool {
        //
        let id = read_raw_token(from).parse::<i32>().unwrap_or(0); // type-checking is missing!
        //
        let mut tag = self.my_read(from);
        let mut name = String::new();
        if tag == "name" {
            name = self.read_name(from);
            tag = self.my_read(from);
        }
        // type
        let type_ = self.my_read(from);
        if type_ == "festzeit" {
            return self.parse_fixed_time(id, name, from);
        }
        if type_ == "vas" {
            return self.parse_vas(id, name, from);
        }
        if type_ == "vsplus" {
            return self.parse_rest_actuated(id, name, from, &type_);
        }
        if type_ == "trends" {
            return self.parse_rest_actuated(id, name, from, &type_);
        }
        if type_ == "vap" {
            return self.parse_rest_actuated(id, name, from, &type_);
        }
        if type_ == "tl" {
            return self.parse_rest_actuated(id, name, from, &type_);
        }
        if type_ == "pos" {
            return self.parse_rest_actuated(id, name, from, &type_);
        }
        if type_ == "nema" {
            return self.parse_rest_actuated(id, name, from, &type_);
        }
        if type_ == "extern" {
            return self.parse_rest_actuated(id, name, from, &type_);
        }
        write_errorf!("Unsupported LSA-Type '%' occurred.", type_);
        false
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
    pub fn parse_fixed_time(&mut self, id: i32, name: String, from: &mut std::io::Cursor<Vec<u8>> /* std::istream */) -> bool {
        let mut type_ = "festzeit".to_string();
        read_raw_token(from);
        //
        let absdur = read_double(from); // type-checking is missing!
        //
        let mut tag = self.read_end_secure(from, "");
        let mut offset = 0.;
        if tag == "versatz" {
            offset = read_double(from); // type-checking is missing!
        }
        if tag != "szpkonfdatei" && tag != "DATAEND" && tag != "progdatei" {
            tag = self.read_end_secure(from, "");
            if tag == "szpkonfdatei" || tag == "progdatei" {
                type_ = "festzeit_fake".to_string();
            }
        }
        NIVissimTL::dictionary(id, &type_, &name, utils::common::time::time2steps(absdur), utils::common::time::time2steps(offset))
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
    pub fn parse_vas(&mut self, id: i32, name: String, from: &mut std::io::Cursor<Vec<u8>> /* std::istream */) -> bool {
        read_raw_token(from);
        //
        let absdur = read_double(from); // type-checking is missing!
        //
        let tag = self.read_end_secure(from, "");
        let mut offset = 0.;
        if tag == "versatz" {
            offset = read_double(from); // type-checking is missing!
        }
        NIVissimTL::dictionary(id, "vas", &name, utils::common::time::time2steps(absdur), utils::common::time::time2steps(offset))
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
    pub fn parse_rest_actuated(&mut self, id: i32, name: String, from: &mut std::io::Cursor<Vec<u8>> /* std::istream */, type_: &str) -> bool {
        read_raw_token(from);
        //
        let absdur = read_double(from); // type-checking is missing!
        //
        let mut tag = self.read_end_secure(from, "");
        let mut offset = 0.;
        if tag == "versatz" {
            offset = read_double(from); // type-checking is missing!
        }
        while tag != "datei" {
            tag = self.my_read(from);
        }
        NIVissimTL::dictionary(id, type_, &name, utils::common::time::time2steps(absdur), utils::common::time::time2steps(offset))
    }

}

