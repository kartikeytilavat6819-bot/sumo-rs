//! Port of polyconvert/PCTypeMap.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ enum `PCTypeMap::Filltype`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum PCTypeMap_Filltype {
    NOFILL = 0,
    FILL = 1,
    FORCE = 2,
}

/// A storage for type mappings
/// C++ class `PCTypeMap` (polyconvert/PCTypeMap.h)
pub struct PCTypeMap {
    /// A map of type names to type definitions
    pub myTypes: BTreeMap<String, PCTypeMap_TypeDef>,
    pub myDefaultType: PCTypeMap_TypeDef,
}

impl PCTypeMap {
    /// C++: `void PCTypeMap::PCTypeMap(const OptionsCont &)`
    /// Constructor. The default type is constructed based on the given options
    // PCTypeMap::PCTypeMap(const OptionsCont& oc) {
    //     myDefaultType.id = oc.getString("type");
    //     myDefaultType.color = RGBColor::parseColor(oc.getString("color"));
    //     myDefaultType.icon = oc.getString("icon");
    //     myDefaultType.layer = oc.getFloat("layer");
    //     myDefaultType.discard = oc.getBool("discard");
    //     myDefaultType.allowFill = oc.getBool("fill") ? Filltype::FILL : Filltype::NOFILL;
    //     myDefaultType.prefix = oc.getString("prefix");
    // }
    #[allow(unused_variables)]
    pub fn new(oc: &OptionsCont) -> Self {
        fn or_panic<T>(r: Result<T, ProcessError>) -> T {
            r.unwrap_or_else(|e| panic!("{e}"))
        }
        // shortcut: C++ leaves myDefaultType.angle and imgFile uninitialized; zero and empty are used here
        let myDefaultType = PCTypeMap_TypeDef {
            id: or_panic(oc.get_string("type")),
            color: RGBColor::parse_color(&or_panic(oc.get_string("color"))).unwrap_or_else(|e| panic!("{e}")),
            icon: or_panic(oc.get_string("icon")),
            layer: or_panic(oc.get_float("layer")),
            angle: 0.,
            imgFile: String::new(),
            discard: or_panic(oc.get_bool("discard")),
            allowFill: if or_panic(oc.get_bool("fill")) { PCTypeMap_Filltype::FILL } else { PCTypeMap_Filltype::NOFILL },
            prefix: or_panic(oc.get_string("prefix")),
        };
        Self { myTypes: BTreeMap::new(), myDefaultType }
    }

    /// C++: `void PCTypeMap::~PCTypeMap()`
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ body is empty: the default destructor only releases the map
    }

    /// C++: `bool PCTypeMap::add(const std::string &, const std::string &, const std::string &, const std::string &, const std::string &, double, double, const std::string &, bool, Filltype)`
    /// Adds a type definition
    // bool
    // PCTypeMap::add(const std::string& id, const std::string& newid, const std::string& color,
    //                const std::string& prefix, const std::string& icon, double layer,
    //                double angle, const std::string& imgFile, bool discard, Filltype allowFill) {
    //     if (has(id)) {
    //         return false;
    //     }
    //     TypeDef td;
    //     td.id = newid;
    //     td.color = RGBColor::parseColor(color);
    //     td.icon = icon;
    //     td.layer = layer;
    //     td.angle = angle;
    //     td.imgFile = imgFile;
    //     td.discard = discard;
    //     td.allowFill = allowFill;
    //     td.prefix = prefix;
    //     myTypes[id] = td;
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn add(&mut self, id: &str, newid: &str, color: &str, prefix: &str, icon: &str, layer: f64, angle: f64, imgFile: &str, discard: bool, allowFill: PCTypeMap_Filltype) -> bool {
        if self.has(id) {
            return false;
        }
        let td = PCTypeMap_TypeDef {
            id: newid.to_string(),
            color: RGBColor::parse_color(color).unwrap_or_else(|e| panic!("{e}")),
            icon: icon.to_string(),
            layer,
            angle,
            imgFile: imgFile.to_string(),
            discard,
            allowFill,
            prefix: prefix.to_string(),
        };
        self.myTypes.insert(id.to_string(), td);
        true
    }

    /// C++: `const TypeDef & PCTypeMap::get(const std::string &)`
    /// Returns a type definition
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get(&mut self, id: &str) -> &PCTypeMap_TypeDef {
        // C++ dereferences find(id)->second without checking, which is undefined for an unknown id; panic instead
        &self.myTypes[id]
    }

    /// C++: `bool PCTypeMap::has(const std::string &)`
    /// Returns the information whether the named type is known
    // bool
    // PCTypeMap::has(const std::string& id) {
    //     return myTypes.find(id) != myTypes.end();
    // }
    #[allow(unused_variables)]
    pub fn has(&mut self, id: &str) -> bool {
        self.myTypes.contains_key(id)
    }

    /// C++: `const TypeDef & PCTypeMap::getDefault()`
    /// get the default type according to the given options
    // const TypeDef& getDefault() {
    //         return myDefaultType;
    //     }
    #[allow(unused_variables)]
    pub fn get_default(&mut self) -> &PCTypeMap_TypeDef {
        &self.myDefaultType
    }

    /// C++: `const std::map<std::string, TypeDef> & PCTypeMap::getTypes() const` [const]
    /// retrieve all known types
    // const std::map<std::string, TypeDef>& getTypes() const {
    //         return myTypes;
    //     }
    #[allow(unused_variables)]
    pub fn get_types(&self) -> &BTreeMap<String, PCTypeMap_TypeDef> {
        &self.myTypes
    }

}

/// A single definition of values that shall be used for a given type
/// C++ class `PCTypeMap::TypeDef` (polyconvert/PCTypeMap.h)
pub struct PCTypeMap_TypeDef {
    /// The new type id to use
    pub id: String,
    /// The color to use
    pub color: RGBColor,
    /// The prefix to use
    pub prefix: String,
    /// the icon to use
    pub icon: String,
    /// The layer to use
    pub layer: f64,
    /// The angle to use
    pub angle: f64,
    /// The image file to use
    pub imgFile: String,
    /// Information whether polygons of this type shall be discarded
    pub discard: bool,
    /// Information whether polygons of this type can be filled
    pub allowFill: PCTypeMap_Filltype,
}

impl PCTypeMap_TypeDef {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filltype_values_match_release() {
        // C++ polyconvert/PCTypeMap.h:52-54 — the values are written to output XML.
        assert_eq!(PCTypeMap_Filltype::NOFILL as i64, 0);
        assert_eq!(PCTypeMap_Filltype::FILL as i64, 1);
        assert_eq!(PCTypeMap_Filltype::FORCE as i64, 2);
    }
}

