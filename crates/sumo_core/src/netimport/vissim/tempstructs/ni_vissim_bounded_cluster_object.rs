//! Port of netimport/vissim/tempstructs/NIVissimBoundedClusterObject.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::set<NIVissimBoundedClusterObject *> NIVissimBoundedClusterObject::ContType`
pub type NIVissimBoundedClusterObject_ContType = BTreeSet<Ptr<NIVissimBoundedClusterObject>>;

/// C++ class `NIVissimBoundedClusterObject` (netimport/vissim/tempstructs/NIVissimBoundedClusterObject.h)
pub struct NIVissimBoundedClusterObject {
    pub myBoundary: Ptr<Boundary>,
    pub myClusterID: i32,
}

// C++ static members of NIVissimBoundedClusterObject (port as module-level statics, see AGENTS.md):
//   static ContType NIVissimBoundedClusterObject::myDict;
thread_local! {
    // determinism: C++ std::set orders by pointer address; BTreeMap orders by Ptr's Ord (address), same relative order.
    // The value is the subclass's virtual computeBounding(), since the base pointer cannot dispatch it.
    static MY_DICT: std::cell::RefCell<BTreeMap<Ptr<NIVissimBoundedClusterObject>, fn(&mut NIVissimBoundedClusterObject)>> = std::cell::RefCell::new(BTreeMap::new());
}

impl NIVissimBoundedClusterObject {
    /// C++: `void NIVissimBoundedClusterObject::NIVissimBoundedClusterObject()`
    // NIVissimBoundedClusterObject::NIVissimBoundedClusterObject()
    //     : myBoundary(nullptr), myClusterID(-1) {
    //     myDict.insert(this);
    // }
    #[allow(unused_variables)]
    pub fn new() -> Box<Self> {
        // the constructor registers `this` in myDict, so the object must stay at a fixed address: return the Box
        let mut b = Box::new(Self { myBoundary: Ptr::null(), myClusterID: -1 });
        Self::register(Ptr::from_mut(&mut *b), |o| o.compute_bounding());
        b
    }

    /// C++: the base constructor's `myDict.insert(this)`, called by subclasses that build their own base.
    /// `compute` is the subclass's computeBounding() override, which closeLoading() must reach virtually.
    pub fn register(this: Ptr<NIVissimBoundedClusterObject>, compute: fn(&mut NIVissimBoundedClusterObject)) {
        MY_DICT.with(|d| d.borrow_mut().insert(this, compute));
    }

    /// C++: `void NIVissimBoundedClusterObject::~NIVissimBoundedClusterObject() noexcept` [virtual]
    // NIVissimBoundedClusterObject::~NIVissimBoundedClusterObject() {
    //     delete myBoundary;
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        if !self.myBoundary.is_null() {
            // SAFETY: myBoundary is a Box::leak'd or Box-owned Boundary created by its owner; delete frees it once
            drop(unsafe { Box::from_raw(self.myBoundary.as_raw()) });
        }
    }

    /// C++: `void NIVissimBoundedClusterObject::computeBounding()` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn compute_bounding(&mut self) {
        panic!("NIVissimBoundedClusterObject::computeBounding is pure virtual; subclasses implement it")
    }

    /// C++: `bool NIVissimBoundedClusterObject::crosses(const AbstractPoly &, double) const` [const]
    // bool
    // NIVissimBoundedClusterObject::crosses(const AbstractPoly& poly,
    //                                       double offset) const {
    //     assert(myBoundary != 0 && myBoundary->xmax() >= myBoundary->xmin());
    //     return myBoundary->overlapsWith(poly, offset);
    // }
    #[allow(unused_variables)]
    pub fn crosses(&self, poly: &dyn AbstractPoly, offset: f64) -> bool {
        debug_assert!(!self.myBoundary.is_null() && self.myBoundary.xmax() >= self.myBoundary.xmin());
        self.myBoundary.overlaps_with(poly, offset)
    }

    /// C++: `void NIVissimBoundedClusterObject::inCluster(int)`
    // void
    // NIVissimBoundedClusterObject::inCluster(int id) {
    //     myClusterID = id;
    // }
    #[allow(unused_variables)]
    pub fn in_cluster(&mut self, id: i32) {
        self.myClusterID = id;
    }

    /// C++: `bool NIVissimBoundedClusterObject::clustered() const` [const]
    // bool
    // NIVissimBoundedClusterObject::clustered() const {
    //     return myClusterID > 0;
    // }
    #[allow(unused_variables)]
    pub fn clustered(&self) -> bool {
        self.myClusterID > 0
    }

    /// C++: `const Boundary & NIVissimBoundedClusterObject::getBoundary() const` [const]
    // const Boundary&
    // NIVissimBoundedClusterObject::getBoundary() const {
    //     return *myBoundary;
    // }
    #[allow(unused_variables)]
    pub fn get_boundary(&self) -> &Boundary {
        self.myBoundary.get().expect("NIVissimBoundedClusterObject::getBoundary on a null boundary")
    }

    /// C++: `void NIVissimBoundedClusterObject::closeLoading()` [static]
    // void
    // NIVissimBoundedClusterObject::closeLoading() {
    //     for (ContType::iterator i = myDict.begin(); i != myDict.end(); i++) {
    //         (*i)->computeBounding();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn close_loading() {
        // snapshot first: a computeBounding() must not hold the dictionary borrow
        let entries: Vec<(Ptr<NIVissimBoundedClusterObject>, fn(&mut NIVissimBoundedClusterObject))> =
            MY_DICT.with(|d| d.borrow().iter().map(|(o, f)| (*o, *f)).collect());
        for (o, compute) in entries {
            compute(o.m());
        }
    }

}

