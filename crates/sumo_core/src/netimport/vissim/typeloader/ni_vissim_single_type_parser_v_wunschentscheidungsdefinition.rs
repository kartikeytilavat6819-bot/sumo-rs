//! Port of netimport/vissim/typeloader/NIVissimSingleTypeParser_VWunschentscheidungsdefinition.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `NIVissimSingleTypeParser_VWunschentscheidungsdefinition` (netimport/vissim/typeloader/NIVissimSingleTypeParser_VWunschentscheidungsdefinition.h)
#[repr(C)]
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

/// C++ `from >> tag` for a std::string (same as NIImporter_Vissim's private vissim_read_token)
fn vissim_read_token(from: &mut std::io::Cursor<Vec<u8>>) -> String {
    let data = from.get_ref();
    let mut i = from.position() as usize;
    while i < data.len() && matches!(data[i], b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c) {
        i += 1;
    }
    let start = i;
    while i < data.len() && !matches!(data[i], b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c) {
        i += 1;
    }
    let tok = vissim_bytes_to_string(&data[start..i]);
    from.set_position(i as u64);
    tok
}

impl NIVissimSingleTypeParser_VWunschentscheidungsdefinition {
    /// C++: `void NIVissimSingleTypeParser_VWunschentscheidungsdefinition::NIVissimSingleTypeParser_VWunschentscheidungsdefinition(NIImporter_Vissim &)`
    /// Constructor
    // NIVissimSingleTypeParser_VWunschentscheidungsdefinition::NIVissimSingleTypeParser_VWunschentscheidungsdefinition(NIImporter_Vissim& parent)
    //     : NIImporter_Vissim::VissimSingleTypeParser(parent) {}
    #[allow(unused_variables)]
    pub fn new(parent: &mut NIImporter_Vissim) -> Self {
        Self { base: NIImporter_Vissim_VissimSingleTypeParser::new(parent) }
    }

    /// C++: `void NIVissimSingleTypeParser_VWunschentscheidungsdefinition::~NIVissimSingleTypeParser_VWunschentscheidungsdefinition() noexcept` [virtual]
    /// Destructor
    // NIVissimSingleTypeParser_VWunschentscheidungsdefinition::~NIVissimSingleTypeParser_VWunschentscheidungsdefinition() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // destructor: nothing to release explicitly, the base parser drops with the struct
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
    pub fn parse(&mut self, from: &mut std::io::Cursor<Vec<u8>> /* std::istream */) -> bool {
        let _ = vissim_read_token(from); // id
        let _ = vissim_read_token(from); // name
        let _ = self.read_name(from);
        let _ = self.override_optional_label(from, "");
        let _ = vissim_read_token(from); // strecke
        let edgeid = vissim_read_token(from);
        let _ = vissim_read_token(from); // spur
        let lane = vissim_read_token(from);
        let _ = vissim_read_token(from); // bei
        let _pos = vissim_read_token(from);
        let _ = vissim_read_token(from); // fahrzeugklasse
        let _ = vissim_read_token(from); // <fahrzeugklasse>
        let _ = vissim_read_token(from); // vwunsch
        let vwunsch = vissim_read_token(from);
        let tmp: Vec<String> = vec!["zeit".to_string(), "fahrzeugklasse".to_string()];
        let mut tag = self.read_end_secure_2(from, &tmp);
        while tag != "DATAEND" && tag != "zeit" {
            let _ = vissim_read_token(from);
            let _ = vissim_read_token(from);
            let _ = vissim_read_token(from);
            tag = self.my_read(from);
        }
        if tag == "zeit" {
            let _ = vissim_read_token(from);
            let _ = vissim_read_token(from);
            let _ = vissim_read_token(from);
            let _ = vissim_read_token(from);
        }
        let numid = utils::common::string_utils::to_int(&edgeid).unwrap_or_else(|e| panic!("{e}"));
        let numlane = utils::common::string_utils::to_int(&lane).unwrap_or_else(|e| panic!("{e}")) - 1;
        let numv = utils::common::string_utils::to_int(&vwunsch).unwrap_or_else(|e| panic!("{e}"));
        let mut e = NIVissimEdge::dictionary_3(numid);
        if e.is_null() {
            let c = NIVissimConnection::dictionary_2(numid);
            let lanes = c.get_to_lanes();
            e = NIVissimEdge::dictionary_3(c.get_to_edge_id());
            for j in lanes.iter() {
                e.m().set_speed(*j, numv);
            }
            assert!(!e.is_null());
        } else {
            e.m().set_speed(numlane, numv);
        }
        true
    }

}

