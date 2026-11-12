//! Port of netimport/vissim/tempstructs/NIVissimClosures.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::map<std::string, NIVissimClosures *> NIVissimClosures::DictType`
pub type NIVissimClosures_DictType = BTreeMap<String, Ptr<NIVissimClosures>>;

/// C++ class `NIVissimClosures` (netimport/vissim/tempstructs/NIVissimClosures.h)
pub struct NIVissimClosures {
    pub myID: String,
    pub myFromNode: i32,
    pub myToNode: i32,
    pub myOverEdges: Vec<i32>,
}

// C++ static members of NIVissimClosures (port as module-level statics, see AGENTS.md):
//   static DictType NIVissimClosures::myDict;
thread_local! {
    static MY_DICT: std::cell::RefCell<NIVissimClosures_DictType> = std::cell::RefCell::new(BTreeMap::new());
}

impl NIVissimClosures {
    /// C++: `void NIVissimClosures::NIVissimClosures(const std::string &, int, int, std::vector<int> &)`
    // NIVissimClosures::NIVissimClosures(const std::string& id,
    //                                    int from_node, int to_node,
    //                                    std::vector<int>& overEdges)
    //     : myID(id), myFromNode(from_node), myToNode(to_node),
    //       myOverEdges(overEdges) {}
    #[allow(unused_variables)]
    pub fn new(id: &str, from_node: i32, to_node: i32, overEdges: &mut Vec<i32>) -> Self {
        Self {
            myID: id.to_string(),
            myFromNode: from_node,
            myToNode: to_node,
            myOverEdges: overEdges.clone(),
        }
    }

    /// C++: `void NIVissimClosures::~NIVissimClosures()`
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ body is implicit: nothing to release beyond the members
    }

    /// C++: `bool NIVissimClosures::dictionary(const std::string &, int, int, std::vector<int> &)` [static]
    // bool
    // NIVissimClosures::dictionary(const std::string& id,
    //                              int from_node, int to_node,
    //                              std::vector<int>& overEdges) {
    //     NIVissimClosures* o = new NIVissimClosures(id, from_node, to_node,
    //             overEdges);
    //     if (!dictionary(id, o)) {
    //         delete o;
    //         return false;
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn dictionary(id: &str, from_node: i32, to_node: i32, overEdges: &mut Vec<i32>) -> bool {
        // C++ `new` followed by `delete` on failure: the Box is dropped when the insert is refused
        let mut o = Box::new(Self::new(id, from_node, to_node, overEdges));
        if !Self::dictionary_2(id, Ptr::from_mut(&mut *o)) {
            return false;
        }
        // kept alive for the dictionary, freed by clear_dict
        Box::leak(o);
        true
    }

    /// C++: `bool NIVissimClosures::dictionary(const std::string &, NIVissimClosures *)` [static]
    // bool
    // NIVissimClosures::dictionary(const std::string& name, NIVissimClosures* o) {
    //     DictType::iterator i = myDict.find(name);
    //     if (i == myDict.end()) {
    //         myDict[name] = o;
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn dictionary_2(name: &str, o: Ptr<NIVissimClosures>) -> bool {
        MY_DICT.with(|d| {
            let mut d = d.borrow_mut();
            if d.contains_key(name) {
                return false;
            }
            d.insert(name.to_string(), o);
            true
        })
    }

    /// C++: `NIVissimClosures * NIVissimClosures::dictionary(const std::string &)` [static]
    // NIVissimClosures*
    // NIVissimClosures::dictionary(const std::string& name) {
    //     DictType::iterator i = myDict.find(name);
    //     if (i == myDict.end()) {
    //         return nullptr;
    //     }
    //     return (*i).second;
    // }
    #[allow(unused_variables)]
    pub fn dictionary_3(name: &str) -> Ptr<NIVissimClosures> {
        MY_DICT.with(|d| d.borrow().get(name).copied().unwrap_or_default())
    }

    /// C++: `void NIVissimClosures::clearDict()` [static]
    // void
    // NIVissimClosures::clearDict() {
    //     for (DictType::iterator i = myDict.begin(); i != myDict.end(); i++) {
    //         delete (*i).second;
    //     }
    //     myDict.clear();
    // }
    #[allow(unused_variables)]
    pub fn clear_dict() {
        MY_DICT.with(|d| {
            for (_, o) in std::mem::take(&mut *d.borrow_mut()) {
                // SAFETY: every entry is created by dictionary() through Box::leak, so Box::from_raw frees it once
                drop(unsafe { Box::from_raw(o.as_raw()) });
            }
        });
    }

    /// C++: `void NIVissimClosures::NIVissimClosures(const NIVissimClosures &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &NIVissimClosures) -> Self {
        panic!("NIVissimClosures copy constructor is private and never defined in C++")
    }

    /// C++: `NIVissimClosures & NIVissimClosures::operator=(const NIVissimClosures &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &NIVissimClosures) -> &mut NIVissimClosures {
        panic!("NIVissimClosures assignment operator is private and never defined in C++")
    }

}

