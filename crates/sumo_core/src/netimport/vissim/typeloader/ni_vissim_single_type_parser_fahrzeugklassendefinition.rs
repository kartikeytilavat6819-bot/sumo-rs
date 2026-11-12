//! Port of netimport/vissim/typeloader/NIVissimSingleTypeParser_Fahrzeugklassendefinition.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `NIVissimSingleTypeParser_Fahrzeugklassendefinition` (netimport/vissim/typeloader/NIVissimSingleTypeParser_Fahrzeugklassendefinition.h)
#[repr(C)]
pub struct NIVissimSingleTypeParser_Fahrzeugklassendefinition {
    /// C++ base class `NIImporter_Vissim::VissimSingleTypeParser`
    pub base: NIImporter_Vissim_VissimSingleTypeParser,
    /// color definitions
    pub myColorMap: Ptr<NIImporter_Vissim_ColorMap>,
}

impl std::ops::Deref for NIVissimSingleTypeParser_Fahrzeugklassendefinition {
    type Target = NIImporter_Vissim_VissimSingleTypeParser;
    fn deref(&self) -> &NIImporter_Vissim_VissimSingleTypeParser {
        &self.base
    }
}
impl std::ops::DerefMut for NIVissimSingleTypeParser_Fahrzeugklassendefinition {
    fn deref_mut(&mut self) -> &mut NIImporter_Vissim_VissimSingleTypeParser {
        &mut self.base
    }
}

impl NIVissimSingleTypeParser_Fahrzeugklassendefinition {
    /// C++: `void NIVissimSingleTypeParser_Fahrzeugklassendefinition::NIVissimSingleTypeParser_Fahrzeugklassendefinition(NIImporter_Vissim &, NIImporter_Vissim::ColorMap &)`
    /// Constructor
    // NIVissimSingleTypeParser_Fahrzeugklassendefinition::NIVissimSingleTypeParser_Fahrzeugklassendefinition(
    //     NIImporter_Vissim& parent, NIImporter_Vissim::ColorMap& colorMap)
    //     : NIImporter_Vissim::VissimSingleTypeParser(parent),
    //       myColorMap(colorMap) {}
    #[allow(unused_variables)]
    pub fn new(parent: &mut NIImporter_Vissim, colorMap: &mut NIImporter_Vissim_ColorMap) -> Self {
        todo!("port NIVissimSingleTypeParser_Fahrzeugklassendefinition::NIVissimSingleTypeParser_Fahrzeugklassendefinition")
    }

    /// C++: `void NIVissimSingleTypeParser_Fahrzeugklassendefinition::~NIVissimSingleTypeParser_Fahrzeugklassendefinition() noexcept` [virtual]
    /// Destructor
    // NIVissimSingleTypeParser_Fahrzeugklassendefinition::~NIVissimSingleTypeParser_Fahrzeugklassendefinition() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NIVissimSingleTypeParser_Fahrzeugklassendefinition::~NIVissimSingleTypeParser_Fahrzeugklassendefinition")
    }

    /// C++: `bool NIVissimSingleTypeParser_Fahrzeugklassendefinition::parse(std::istream &)` [virtual]
    /// Parses the data type from the given stream
    // bool
    // NIVissimSingleTypeParser_Fahrzeugklassendefinition::parse(std::istream& from) {
    //     // id
    //     int id;
    //     from >> id; // type-checking is missing!
    //     // name
    //     std::string tag;
    //     from >> tag;
    //     std::string name = readName(from);
    //     // color
    //     from >> tag;
    //     std::string colorName = myRead(from);
    //     RGBColor color;
    //     NIImporter_Vissim::ColorMap::iterator i = myColorMap.find(colorName);
    //     if (i != myColorMap.end()) {
    //         color = (*i).second;
    //     } else {
    //         int r, g, b;
    //         r = StringUtils::toInt(colorName);
    //         if (!(from >> g)) {
    //             throw NumberFormatException("");
    //         }
    //         if (!(from >> b)) {
    //             throw NumberFormatException("");
    //         }
    //         if (r < 0 || r > 255 || g < 0 || g > 255 || b < 0 || b > 255) {
    //             throw NumberFormatException("");
    //         }
    //         color = RGBColor((unsigned char)r, (unsigned char)g, (unsigned char)b, 255);
    //     }
    //     // types
    //     from >> tag;
    //     if (tag == "ANM_ID") {
    //         readName(from);
    //         from >> tag;
    //     }
    //     std::vector<int> types;
    //     from >> tag;
    //     do {
    //         types.push_back(StringUtils::toInt(tag));
    //         tag = readEndSecure(from);
    //     } while (tag != "DATAEND");
    //     return NIVissimVehTypeClass::dictionary(id, name, color, types);
    // }
    #[allow(unused_variables)]
    pub fn parse(&mut self, from: &mut Unmapped /* std::istream */) -> bool {
        todo!("port NIVissimSingleTypeParser_Fahrzeugklassendefinition::parse")
    }

}

