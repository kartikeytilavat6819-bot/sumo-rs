//! Port of netimport/vissim/tempstructs/NIVissimSource.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::map<std::string, NIVissimSource *> NIVissimSource::DictType`
pub type NIVissimSource_DictType = BTreeMap<String, Ptr<NIVissimSource>>;

/// C++ class `NIVissimSource` (netimport/vissim/tempstructs/NIVissimSource.h)
pub struct NIVissimSource {
    pub myID: String,
    pub myName: String,
    pub myEdgeID: String,
}

// C++ static members of NIVissimSource (port as module-level statics, see AGENTS.md):
//   static DictType NIVissimSource::myDict;
thread_local! {
    static MY_DICT: std::cell::RefCell<NIVissimSource_DictType> = std::cell::RefCell::new(BTreeMap::new());
}

impl NIVissimSource {
    /// C++: `void NIVissimSource::NIVissimSource(const std::string &, const std::string &, const std::string &)`
    // NIVissimSource::NIVissimSource(const std::string& id, const std::string& name,
    //                                const std::string& edgeid)
    //     : myID(id), myName(name), myEdgeID(edgeid) {}
    #[allow(unused_variables)]
    pub fn new(id: &str, name: &str, edgeid: &str) -> Self {
        Self {
            myID: id.to_string(),
            myName: name.to_string(),
            myEdgeID: edgeid.to_string(),
        }
    }

    /// C++: `void NIVissimSource::~NIVissimSource()`
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor is implicit; nothing to release.
    }

    /// C++: `bool NIVissimSource::dictionary(const std::string &, const std::string &, const std::string &)` [static]
    // bool
    // NIVissimSource::dictionary(const std::string& id, const std::string& name,
    //                            const std::string& edgeid) {
    //     NIVissimSource* o = new NIVissimSource(id, name, edgeid);
    //     if (!dictionary(id, o)) {
    //         delete o;
    //         return false;
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn dictionary(id: &str, name: &str, edgeid: &str) -> bool {
        let o = Ptr::from_raw(Box::into_raw(Box::new(NIVissimSource::new(id, name, edgeid))));
        if !NIVissimSource::dictionary_2(id, o) {
            // delete o;
            drop(unsafe { Box::from_raw(o.as_raw()) });
            return false;
        }
        true
    }

    /// C++: `bool NIVissimSource::dictionary(const std::string &, NIVissimSource *)` [static]
    // bool
    // NIVissimSource::dictionary(const std::string& id, NIVissimSource* o) {
    //     DictType::iterator i = myDict.find(id);
    //     if (i == myDict.end()) {
    //         myDict[id] = o;
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn dictionary_2(id: &str, o: Ptr<NIVissimSource>) -> bool {
        MY_DICT.with(|d| {
            let mut d = d.borrow_mut();
            if !d.contains_key(id) {
                d.insert(id.to_string(), o);
                return true;
            }
            false
        })
    }

    /// C++: `NIVissimSource * NIVissimSource::dictionary(const std::string &)` [static]
    // NIVissimSource*
    // NIVissimSource::dictionary(const std::string& id) {
    //     DictType::iterator i = myDict.find(id);
    //     if (i == myDict.end()) {
    //         return nullptr;
    //     }
    //     return (*i).second;
    // }
    #[allow(unused_variables)]
    pub fn dictionary_3(id: &str) -> Ptr<NIVissimSource> {
        MY_DICT.with(|d| d.borrow().get(id).copied().unwrap_or_else(Ptr::null))
    }

    /// C++: `void NIVissimSource::clearDict()` [static]
    // void
    // NIVissimSource::clearDict() {
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

}

