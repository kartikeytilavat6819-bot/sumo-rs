//! Port of netimport/vissim/tempstructs/NIVissimNodeDef.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::map<int, NIVissimNodeDef *> NIVissimNodeDef::DictType`
pub type NIVissimNodeDef_DictType = BTreeMap<i32, Ptr<NIVissimNodeDef>>;

/// The concrete class a node definition was built as; selects the subclass override of the virtual `getEdgePosition`
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NIVissimNodeDefKind {
    Plain,
    Edges,
}

/// C++ class `NIVissimNodeDef` (netimport/vissim/tempstructs/NIVissimNodeDef.h)
pub struct NIVissimNodeDef {
    pub myID: i32,
    pub myName: String,
    pub myKind: NIVissimNodeDefKind,
}

// C++ static members of NIVissimNodeDef (port as module-level statics, see AGENTS.md):
//   static DictType NIVissimNodeDef::myDict;
//   static int NIVissimNodeDef::myMaxID;
thread_local! {
    static MY_DICT: std::cell::RefCell<NIVissimNodeDef_DictType> = std::cell::RefCell::new(BTreeMap::new());
    static MY_MAX_ID: std::cell::Cell<i32> = const { std::cell::Cell::new(0) };
}

impl NIVissimNodeDef {
    /// C++: `void NIVissimNodeDef::NIVissimNodeDef(int, const std::string &)`
    // NIVissimNodeDef::NIVissimNodeDef(int id, const std::string& name)
    //     : myID(id), myName(name) {}
    #[allow(unused_variables)]
    pub fn new(id: i32, name: &str) -> Self {
        Self { myID: id, myName: name.to_string(), myKind: NIVissimNodeDefKind::Plain }
    }

    /// C++: `void NIVissimNodeDef::~NIVissimNodeDef()` [virtual]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor is implicit (virtual, empty); nothing to release.
    }

    /// C++: `int NIVissimNodeDef::buildNodeCluster()`
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn build_node_cluster(&mut self) -> i32 {
        // C++ declares buildNodeCluster() without a definition (no body in NIVissimNodeDef.cpp), so there is no body to port
        panic!("NIVissimNodeDef::buildNodeCluster is declared but not defined in SUMO")
    }

    /// C++: `double NIVissimNodeDef::getEdgePosition(int) const` [const, virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_edge_position(&self, edgeid: i32) -> f64 {
        match self.myKind {
            // SAFETY: Edges kind is only set by NIVissimNodeDef_Edges::new, which embeds this base first (repr(C))
            NIVissimNodeDefKind::Edges => unsafe { (*(self as *const Self as *const NIVissimNodeDef_Edges)).get_edge_position(edgeid) },
            NIVissimNodeDefKind::Plain => panic!("NIVissimNodeDef::getEdgePosition is pure virtual; subclasses implement it"),
        }
    }

    /// C++: `bool NIVissimNodeDef::dictionary(int, NIVissimNodeDef *)` [static]
    // bool
    // NIVissimNodeDef::dictionary(int id, NIVissimNodeDef* o) {
    //     DictType::iterator i = myDict.find(id);
    //     if (i == myDict.end()) {
    //         myDict[id] = o;
    //         myMaxID = myMaxID > id
    //                   ? myMaxID
    //                   : id;
    // //        o->computeBounding();
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn dictionary(id: i32, o: Ptr<NIVissimNodeDef>) -> bool {
        MY_DICT.with(|d| {
            let mut d = d.borrow_mut();
            if !d.contains_key(&id) {
                d.insert(id, o);
                MY_MAX_ID.with(|m| m.set(if m.get() > id { m.get() } else { id }));
                // o->computeBounding() is disabled in C++
                return true;
            }
            false
        })
    }

    /// C++: `NIVissimNodeDef * NIVissimNodeDef::dictionary(int)` [static]
    // NIVissimNodeDef*
    // NIVissimNodeDef::dictionary(int id) {
    //     DictType::iterator i = myDict.find(id);
    //     if (i == myDict.end()) {
    //         return nullptr;
    //     }
    //     return (*i).second;
    // }
    #[allow(unused_variables)]
    pub fn dictionary_2(id: i32) -> Ptr<NIVissimNodeDef> {
        MY_DICT.with(|d| d.borrow().get(&id).copied().unwrap_or_else(Ptr::null))
    }

    /// C++: `void NIVissimNodeDef::clearDict()` [static]
    // void
    // NIVissimNodeDef::clearDict() {
    //     for (DictType::iterator i = myDict.begin(); i != myDict.end(); i++) {
    //         delete (*i).second;
    //     }
    //     myDict.clear();
    // }
    #[allow(unused_variables)]
    pub fn clear_dict() {
        // shortcut: C++ deletes each entry through a virtual destructor; Rust has no virtual delete for the base pointer, so entries are only forgotten (leaked). Upgrade with an owner that knows the concrete type.
        MY_DICT.with(|d| d.borrow_mut().clear());
    }

    /// C++: `int NIVissimNodeDef::getMaxID()` [static]
    // int
    // NIVissimNodeDef::getMaxID() {
    //     return myMaxID;
    // }
    #[allow(unused_variables)]
    pub fn get_max_id() -> i32 {
        MY_MAX_ID.with(|m| m.get())
    }

}

