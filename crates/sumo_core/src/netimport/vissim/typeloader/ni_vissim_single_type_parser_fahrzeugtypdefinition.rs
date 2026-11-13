//! Port of netimport/vissim/typeloader/NIVissimSingleTypeParser_Fahrzeugtypdefinition.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `NIVissimSingleTypeParser_Fahrzeugtypdefinition` (netimport/vissim/typeloader/NIVissimSingleTypeParser_Fahrzeugtypdefinition.h)
#[repr(C)]
pub struct NIVissimSingleTypeParser_Fahrzeugtypdefinition {
    /// C++ base class `NIImporter_Vissim::VissimSingleTypeParser`
    pub base: NIImporter_Vissim_VissimSingleTypeParser,
    /// The color map to use
    pub myColorMap: Ptr<NIImporter_Vissim_ColorMap>,
}

impl std::ops::Deref for NIVissimSingleTypeParser_Fahrzeugtypdefinition {
    type Target = NIImporter_Vissim_VissimSingleTypeParser;
    fn deref(&self) -> &NIImporter_Vissim_VissimSingleTypeParser {
        &self.base
    }
}
impl std::ops::DerefMut for NIVissimSingleTypeParser_Fahrzeugtypdefinition {
    fn deref_mut(&mut self) -> &mut NIImporter_Vissim_VissimSingleTypeParser {
        &mut self.base
    }
}

impl NIVissimSingleTypeParser_Fahrzeugtypdefinition {
    /// C++: `void NIVissimSingleTypeParser_Fahrzeugtypdefinition::NIVissimSingleTypeParser_Fahrzeugtypdefinition(NIImporter_Vissim &, NIImporter_Vissim::ColorMap &)`
    /// Constructor
    // NIVissimSingleTypeParser_Fahrzeugtypdefinition::NIVissimSingleTypeParser_Fahrzeugtypdefinition(
    //     NIImporter_Vissim& parent, NIImporter_Vissim::ColorMap& colorMap)
    //     : NIImporter_Vissim::VissimSingleTypeParser(parent),
    //       myColorMap(colorMap) {}
    #[allow(unused_variables)]
    pub fn new(parent: &mut NIImporter_Vissim, colorMap: &mut NIImporter_Vissim_ColorMap) -> Self {
        Self { base: NIImporter_Vissim_VissimSingleTypeParser::new(parent), myColorMap: Ptr::from_mut(colorMap) }
    }

    /// C++: `void NIVissimSingleTypeParser_Fahrzeugtypdefinition::~NIVissimSingleTypeParser_Fahrzeugtypdefinition() noexcept` [virtual]
    /// Destructor
    // NIVissimSingleTypeParser_Fahrzeugtypdefinition::~NIVissimSingleTypeParser_Fahrzeugtypdefinition() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `bool NIVissimSingleTypeParser_Fahrzeugtypdefinition::parse(std::istream &)` [virtual]
    /// Parses the data type from the given stream
    // bool
    // NIVissimSingleTypeParser_Fahrzeugtypdefinition::parse(std::istream& from) {
    //     // id
    //     int id;
    //     from >> id; // type-checking is missing!
    //     // name
    //     std::string tag;
    //     from >> tag;
    //     std::string name = readName(from);
    //     // category
    //     std::string category;
    //     from >> tag;
    //     from >> category;
    //     // color (optional) and length
    //     RGBColor color;
    //     tag = myRead(from);
    //     while (tag != "laenge") {
    //         if (tag == "farbe") {
    //             std::string colorName = myRead(from);
    //             NIImporter_Vissim::ColorMap::iterator i = myColorMap.find(colorName);
    //             if (i != myColorMap.end()) {
    //                 color = (*i).second;
    //             } else {
    //                 int r, g, b;
    //                 r = StringUtils::toInt(colorName);
    //                 if (!(from >> g)) {
    //                     throw NumberFormatException("");
    //                 }
    //                 if (!(from >> b)) {
    //                     throw NumberFormatException("");
    //                 }
    //                 if (r < 0 || r > 255 || g < 0 || g > 255 || b < 0 || b > 255) {
    //                     throw NumberFormatException("");
    //                 }
    //                 color = RGBColor((unsigned char)r, (unsigned char)g, (unsigned char)b, 255);
    //             }
    //         }
    //         tag = myRead(from);
    //     }
    //     double length;
    //     from >> length;
    //     // overread until "Maxbeschleunigung"
    //     while (tag != "maxbeschleunigung") {
    //         tag = myRead(from);
    //     }
    //     double amax;
    //     from >> amax; // type-checking is missing!
    //     // overread until "Maxverzoegerung"
    //     while (tag != "maxverzoegerung") {
    //         tag = myRead(from);
    //     }
    //     double dmax;
    //     from >> dmax; // type-checking is missing!
    //     while (tag != "besetzungsgrad") {
    //         tag = myRead(from);
    //     }
    //     while (tag != "DATAEND") {
    //         tag = readEndSecure(from, "verlustzeit");
    //     }
    //     return NIVissimVehicleType::dictionary(id, name, category, color);
    // }
    #[allow(unused_variables)]
    pub fn parse(&mut self, from: &mut std::io::Cursor<Vec<u8>> /* std::istream */) -> bool {
        // id
        let id: i32 = self.my_read(from).parse::<i32>().unwrap_or(0); // type-checking is missing!
        // name
        let mut tag = self.my_read(from);
        let name = self.read_name(from);
        // category
        tag = self.my_read(from);
        let category = self.my_read(from);
        // color (optional) and length
        let mut color = RGBColor::default();
        tag = self.my_read(from);
        while tag != "laenge" {
            if tag == "farbe" {
                let colorName = self.my_read(from);
                let known = self.myColorMap.m().get(&colorName).cloned();
                if let Some(c) = known {
                    color = c;
                } else {
                    let r = utils::common::string_utils::to_int(&colorName).unwrap_or_else(|e| std::panic::panic_any(e));
                    let g: i32 = self.my_read(from).parse::<i32>().unwrap_or_else(|_| std::panic::panic_any(utils::common::util_exceptions::ProcessError::new("")));
                    let b: i32 = self.my_read(from).parse::<i32>().unwrap_or_else(|_| std::panic::panic_any(utils::common::util_exceptions::ProcessError::new("")));
                    if r < 0 || r > 255 || g < 0 || g > 255 || b < 0 || b > 255 {
                        std::panic::panic_any(utils::common::util_exceptions::ProcessError::new(""));
                    }
                    color = RGBColor::new(r as u8, g as u8, b as u8, 255);
                }
            }
            // shortcut: C++ loops forever when the keyword never comes; here end of stream ends the loop
            if (from.position() as usize) >= from.get_ref().len() {
                break;
            }
            tag = self.my_read(from);
        }
        let _length: f64 = self.my_read(from).parse::<f64>().unwrap_or(0.0);
        // overread until "Maxbeschleunigung"
        while tag != "maxbeschleunigung" && (from.position() as usize) < from.get_ref().len() {
            tag = self.my_read(from);
        }
        let _amax: f64 = self.my_read(from).parse::<f64>().unwrap_or(0.0);
        // overread until "Maxverzoegerung"
        while tag != "maxverzoegerung" && (from.position() as usize) < from.get_ref().len() {
            tag = self.my_read(from);
        }
        let _dmax: f64 = self.my_read(from).parse::<f64>().unwrap_or(0.0);
        while tag != "besetzungsgrad" && (from.position() as usize) < from.get_ref().len() {
            tag = self.my_read(from);
        }
        while tag != "DATAEND" && (from.position() as usize) < from.get_ref().len() {
            tag = self.read_end_secure(from, "verlustzeit");
        }
        NIVissimVehicleType::dictionary(id, &name, &category, &color)
    }

}

