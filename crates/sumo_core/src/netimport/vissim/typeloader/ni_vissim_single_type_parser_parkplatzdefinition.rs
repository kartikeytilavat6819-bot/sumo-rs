//! Port of netimport/vissim/typeloader/NIVissimSingleTypeParser_Parkplatzdefinition.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ `from >> tmp` for a std::string (mirrors the private helper in ni_importer_vissim.rs)
fn read_token(from: &mut std::io::Cursor<Vec<u8>>) -> String {
    let data = from.get_ref();
    let is_space = |c: u8| matches!(c, b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c);
    let mut i = from.position() as usize;
    while i < data.len() && is_space(data[i]) {
        i += 1;
    }
    let start = i;
    while i < data.len() && !is_space(data[i]) {
        i += 1;
    }
    let tok = String::from_utf8_lossy(&data[start..i]).into_owned();
    from.set_position(i as u64);
    tok
}

/// C++ `from >> int` (a number that does not parse gives 0)
fn read_i32(from: &mut std::io::Cursor<Vec<u8>>) -> i32 {
    crate::netimport::vissim::ni_importer_vissim::vissim_read_number(from, false).parse::<i32>().unwrap_or(0)
}

/// C++ `from >> double` (see read_i32)
fn read_f64(from: &mut std::io::Cursor<Vec<u8>>) -> f64 {
    crate::netimport::vissim::ni_importer_vissim::vissim_read_number(from, true).parse::<f64>().unwrap_or(0.0)
}

/// C++ class `NIVissimSingleTypeParser_Parkplatzdefinition` (netimport/vissim/typeloader/NIVissimSingleTypeParser_Parkplatzdefinition.h)
#[repr(C)]
pub struct NIVissimSingleTypeParser_Parkplatzdefinition {
    /// C++ base class `NIImporter_Vissim::VissimSingleTypeParser`
    pub base: NIImporter_Vissim_VissimSingleTypeParser,
}

impl std::ops::Deref for NIVissimSingleTypeParser_Parkplatzdefinition {
    type Target = NIImporter_Vissim_VissimSingleTypeParser;
    fn deref(&self) -> &NIImporter_Vissim_VissimSingleTypeParser {
        &self.base
    }
}
impl std::ops::DerefMut for NIVissimSingleTypeParser_Parkplatzdefinition {
    fn deref_mut(&mut self) -> &mut NIImporter_Vissim_VissimSingleTypeParser {
        &mut self.base
    }
}

impl NIVissimSingleTypeParser_Parkplatzdefinition {
    /// C++: `void NIVissimSingleTypeParser_Parkplatzdefinition::NIVissimSingleTypeParser_Parkplatzdefinition(NIImporter_Vissim &)`
    /// Constructor
    // NIVissimSingleTypeParser_Parkplatzdefinition::NIVissimSingleTypeParser_Parkplatzdefinition(NIImporter_Vissim& parent)
    //     : NIImporter_Vissim::VissimSingleTypeParser(parent) {}
    #[allow(unused_variables)]
    pub fn new(parent: &mut NIImporter_Vissim) -> Self {
        Self { base: NIImporter_Vissim_VissimSingleTypeParser::new(parent) }
    }

    /// C++: `void NIVissimSingleTypeParser_Parkplatzdefinition::~NIVissimSingleTypeParser_Parkplatzdefinition() noexcept` [virtual]
    /// Destructor
    // NIVissimSingleTypeParser_Parkplatzdefinition::~NIVissimSingleTypeParser_Parkplatzdefinition() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
    }

    /// C++: `bool NIVissimSingleTypeParser_Parkplatzdefinition::parse(std::istream &)` [virtual]
    /// Parses the data type from the given stream
    // bool
    // NIVissimSingleTypeParser_Parkplatzdefinition::parse(std::istream& from) {
    //     int id;
    //     from >> id;
    //
    //     std::string tag;
    //     from >> tag;
    //     std::string name = readName(from);
    //
    //     // parse the districts
    //     //  and allocate them if not done before
    //     //  A district may be already saved when another parking place with
    //     //  the same district was already build.
    //     std::vector<int> districts;
    //     std::vector<double> percentages;
    //     readUntil(from, "bezirke"); // "Bezirke"
    //     while (tag != "ort") {
    //         double perc = -1;
    //         int districtid;
    //         from >> districtid;
    //         tag = myRead(from);
    //         if (tag == "anteil") {
    //             from >> perc;
    //         }
    //         districts.push_back(districtid);
    //         percentages.push_back(perc);
    //         tag = myRead(from);
    //     }
    //
    //     from >> tag; // "Strecke"
    //     int edgeid;
    //     from >> edgeid;
    //
    //     double position;
    //     from >> tag; // "bei"
    //     from >> position;
    //
    //     double length;
    //     from >> tag;
    //     from >> length;
    //
    //     from >> tag; // "Kapazitaet"
    //     from >> tag; // "Kapazitaet"-value
    //
    //     tag = myRead(from);
    //     if (tag == "belegung") {
    //         from >> tag;
    //         tag = myRead(from); // "fahrzeugklasse"
    //     }
    //
    //     std::vector<std::pair<int, int> > assignedVehicles;
    //     while (tag != "default") {
    //         int vclass;
    //         from >> vclass;
    //         from >> tag; // "vwunsch"
    //         int vwunsch;
    //         from >> vwunsch; // "vwunsch"-value
    //         assignedVehicles.push_back(std::pair<int, int>(vclass, vwunsch));
    //         tag = myRead(from);
    //     }
    //
    //     from >> tag;
    //     from >> tag;
    // //    NIVissimEdge *e = NIVissimEdge::dictionary(edgeid);
    // //    e->addReferencedDistrict(id);
    //
    //     // build the district connection
    //     return NIVissimDistrictConnection::dictionary(id, name,
    //             districts, percentages, edgeid, position, assignedVehicles);
    // }
    #[allow(unused_variables)]
    pub fn parse(&mut self, from: &mut std::io::Cursor<Vec<u8>> /* std::istream */) -> bool {
        let id: i32 = read_i32(from);

        let mut tag: String = read_token(from);
        let name: String = self.read_name(from);

        // parse the districts
        //  and allocate them if not done before
        //  A district may be already saved when another parking place with
        //  the same district was already build.
        let mut districts: Vec<i32> = Vec::new();
        let mut percentages: Vec<f64> = Vec::new();
        self.read_until(from, "bezirke"); // "Bezirke"
        while tag != "ort" {
            let mut perc: f64 = -1.;
            let districtid: i32 = read_i32(from);
            tag = self.my_read(from);
            if tag == "anteil" {
                perc = read_f64(from);
            }
            districts.push(districtid);
            percentages.push(perc);
            tag = self.my_read(from);
        }

        tag = read_token(from); // "Strecke"
        let edgeid: i32 = read_i32(from);

        tag = read_token(from); // "bei"
        let position: f64 = read_f64(from);

        tag = read_token(from);
        let length: f64 = read_f64(from);

        tag = read_token(from); // "Kapazitaet"
        tag = read_token(from); // "Kapazitaet"-value

        tag = self.my_read(from);
        if tag == "belegung" {
            tag = read_token(from);
            tag = self.my_read(from); // "fahrzeugklasse"
        }

        let mut assignedVehicles: Vec<(i32, i32)> = Vec::new();
        while tag != "default" {
            let vclass: i32 = read_i32(from);
            tag = read_token(from); // "vwunsch"
            let vwunsch: i32 = read_i32(from); // "vwunsch"-value
            assignedVehicles.push((vclass, vwunsch));
            tag = self.my_read(from);
        }

        tag = read_token(from);
        tag = read_token(from);
        // let e = NIVissimEdge::dictionary(edgeid);
        // e.addReferencedDistrict(id);

        // build the district connection
        NIVissimDistrictConnection::dictionary(id, &name, &districts, &percentages, edgeid, position, &assignedVehicles)
    }

}

