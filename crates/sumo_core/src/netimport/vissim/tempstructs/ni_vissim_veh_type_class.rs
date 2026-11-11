//! Port of netimport/vissim/tempstructs/NIVissimVehTypeClass.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::map<int, NIVissimVehTypeClass *> NIVissimVehTypeClass::DictType`
pub type NIVissimVehTypeClass_DictType = BTreeMap<i32, Ptr<NIVissimVehTypeClass>>;

/// C++ class `NIVissimVehTypeClass` (netimport/vissim/tempstructs/NIVissimVehTypeClass.h)
pub struct NIVissimVehTypeClass {
    pub myID: i32,
    pub myName: String,
    pub myColor: RGBColor,
    pub myTypes: Vec<i32>,
}

// C++ static members of NIVissimVehTypeClass (port as module-level statics, see AGENTS.md):
//   static DictType NIVissimVehTypeClass::myDict;
thread_local! {
    static MY_DICT: std::cell::RefCell<NIVissimVehTypeClass_DictType> = std::cell::RefCell::new(BTreeMap::new());
}

impl NIVissimVehTypeClass {
    /// C++: `void NIVissimVehTypeClass::NIVissimVehTypeClass(int, const std::string &, const RGBColor &, std::vector<int> &)`
    // NIVissimVehTypeClass::NIVissimVehTypeClass(int id,
    //         const std::string& name,
    //         const RGBColor& color,
    //         std::vector<int>& types)
    //     : myID(id), myName(name), myColor(color), myTypes(types) {}
    #[allow(unused_variables)]
    pub fn new(id: i32, name: &str, color: &RGBColor, types: &mut Vec<i32>) -> Self {
        Self {
            myID: id,
            myName: name.to_string(),
            myColor: *color,
            myTypes: types.clone(),
        }
    }

    /// C++: `void NIVissimVehTypeClass::~NIVissimVehTypeClass()`
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor is implicit; nothing to release.
    }

    /// C++: `bool NIVissimVehTypeClass::dictionary(int, const std::string &, const RGBColor &, std::vector<int> &)` [static]
    // bool
    // NIVissimVehTypeClass::dictionary(int id, const std::string& name,
    //                                  const RGBColor& color,
    //                                  std::vector<int>& types) {
    //     NIVissimVehTypeClass* o = new NIVissimVehTypeClass(id, name, color, types);
    //     if (!dictionary(id, o)) {
    //         delete o;
    //         return false;
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn dictionary(id: i32, name: &str, color: &RGBColor, types: &mut Vec<i32>) -> bool {
        let o = Ptr::from_raw(Box::into_raw(Box::new(NIVissimVehTypeClass::new(id, name, color, types))));
        if !NIVissimVehTypeClass::dictionary_2(id, o) {
            // delete o;
            drop(unsafe { Box::from_raw(o.as_raw()) });
            return false;
        }
        true
    }

    /// C++: `bool NIVissimVehTypeClass::dictionary(int, NIVissimVehTypeClass *)` [static]
    // bool
    // NIVissimVehTypeClass::dictionary(int name, NIVissimVehTypeClass* o) {
    //     DictType::iterator i = myDict.find(name);
    //     if (i == myDict.end()) {
    //         myDict[name] = o;
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn dictionary_2(id: i32, o: Ptr<NIVissimVehTypeClass>) -> bool {
        MY_DICT.with(|d| {
            let mut d = d.borrow_mut();
            if !d.contains_key(&id) {
                d.insert(id, o);
                return true;
            }
            false
        })
    }

    /// C++: `NIVissimVehTypeClass * NIVissimVehTypeClass::dictionary(int)` [static]
    // NIVissimVehTypeClass*
    // NIVissimVehTypeClass::dictionary(int name) {
    //     DictType::iterator i = myDict.find(name);
    //     if (i == myDict.end()) {
    //         return nullptr;
    //     }
    //     return (*i).second;
    // }
    #[allow(unused_variables)]
    pub fn dictionary_3(name: i32) -> Ptr<NIVissimVehTypeClass> {
        MY_DICT.with(|d| d.borrow().get(&name).copied().unwrap_or_else(Ptr::null))
    }

    /// C++: `void NIVissimVehTypeClass::clearDict()` [static]
    // void
    // NIVissimVehTypeClass::clearDict() {
    //     for (DictType::iterator i = myDict.begin(); i != myDict.end(); i++) {
    //         delete (*i).second;
    //     }
    //     myDict.clear();
    // }
    #[allow(unused_variables)]
    pub fn clear_dict() {
        MY_DICT.with(|d| {
            let mut d = d.borrow_mut();
            for (_, o) in d.iter() {
                // delete (*i).second;
                drop(unsafe { Box::from_raw(o.as_raw()) });
            }
            d.clear();
        })
    }

    /// C++: `double NIVissimVehTypeClass::meanSpeed() const` [const]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn mean_speed(&self) -> f64 {
        // shortcut: declared as double meanSpeed() const in NIVissimVehTypeClass.h but never defined in SUMO; upgrade only if SUMO adds a definition.
        panic!("NIVissimVehTypeClass::meanSpeed is declared but not defined in SUMO")
    }

}

