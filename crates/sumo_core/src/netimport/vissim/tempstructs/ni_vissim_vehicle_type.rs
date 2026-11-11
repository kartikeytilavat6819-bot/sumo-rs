//! Port of netimport/vissim/tempstructs/NIVissimVehicleType.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::map<int, NIVissimVehicleType *> NIVissimVehicleType::DictType`
pub type NIVissimVehicleType_DictType = BTreeMap<i32, Ptr<NIVissimVehicleType>>;

/// C++ class `NIVissimVehicleType` (netimport/vissim/tempstructs/NIVissimVehicleType.h)
pub struct NIVissimVehicleType {
    pub myName: String,
    pub myCategory: String,
    pub myColor: RGBColor,
}

// C++ static members of NIVissimVehicleType (port as module-level statics, see AGENTS.md):
//   static DictType NIVissimVehicleType::myDict;

impl NIVissimVehicleType {
    /// C++: `void NIVissimVehicleType::NIVissimVehicleType(const std::string &, const std::string &, const RGBColor &)`
    // NIVissimVehicleType::NIVissimVehicleType(const std::string& name,
    //         const std::string& category, const RGBColor& color)
    //     : myName(name), myCategory(category),
    //       myColor(color) {}
    #[allow(unused_variables)]
    pub fn new(name: &str, category: &str, color: &RGBColor) -> Self {
        todo!("port NIVissimVehicleType::NIVissimVehicleType")
    }

    /// C++: `void NIVissimVehicleType::~NIVissimVehicleType()`
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NIVissimVehicleType::~NIVissimVehicleType")
    }

    /// C++: `bool NIVissimVehicleType::dictionary(int, const std::string &, const std::string &, const RGBColor &)` [static]
    // bool
    // NIVissimVehicleType::dictionary(int id, const std::string& name, const std::string& category,
    //                                 const RGBColor& color) {
    //     NIVissimVehicleType* o = new NIVissimVehicleType(name, category, color);
    //     if (!dictionary(id, o)) {
    //         delete o;
    //         return false;
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn dictionary(id: i32, name: &str, category: &str, color: &RGBColor) -> bool {
        todo!("port NIVissimVehicleType::dictionary")
    }

    /// C++: `bool NIVissimVehicleType::dictionary(int, NIVissimVehicleType *)` [static]
    // bool
    // NIVissimVehicleType::dictionary(int id, NIVissimVehicleType* o) {
    //     DictType::iterator i = myDict.find(id);
    //     if (i == myDict.end()) {
    //         myDict[id] = o;
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn dictionary_2(id: i32, o: Ptr<NIVissimVehicleType>) -> bool {
        todo!("port NIVissimVehicleType::dictionary")
    }

    /// C++: `NIVissimVehicleType * NIVissimVehicleType::dictionary(int)` [static]
    // NIVissimVehicleType*
    // NIVissimVehicleType::dictionary(int id) {
    //     DictType::iterator i = myDict.find(id);
    //     if (i == myDict.end()) {
    //         return nullptr;
    //     }
    //     return (*i).second;
    // }
    #[allow(unused_variables)]
    pub fn dictionary_3(id: i32) -> Ptr<NIVissimVehicleType> {
        todo!("port NIVissimVehicleType::dictionary")
    }

    /// C++: `void NIVissimVehicleType::clearDict()` [static]
    // void
    // NIVissimVehicleType::clearDict() {
    //     for (DictType::iterator i = myDict.begin(); i != myDict.end(); i++) {
    //         delete (*i).second;
    //     }
    //     myDict.clear();
    // }
    #[allow(unused_variables)]
    pub fn clear_dict() {
        todo!("port NIVissimVehicleType::clearDict")
    }

}

