//! Port of netbuild/NBDistrictCont.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::map<std::string, NBDistrict *> NBDistrictCont::DistrictCont`
pub type NBDistrictCont_DistrictCont = BTreeMap<String, Ptr<NBDistrict>>;

/// A container for districts
/// C++ class `NBDistrictCont` (netbuild/NBDistrictCont.h)
pub struct NBDistrictCont {
    /// The instance of the dictionary
    pub myDistricts: NBDistrictCont_DistrictCont,
}

impl NBDistrictCont {
    /// C++: `void NBDistrictCont::NBDistrictCont()`
    /// Constructor
    // NBDistrictCont::NBDistrictCont() {}
    #[allow(unused_variables)]
    pub fn new() -> Self {
        Self { myDistricts: BTreeMap::new() }
    }

    /// C++: `void NBDistrictCont::~NBDistrictCont()`
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor has no body; the map and its Ptr entries need no explicit release
    }

    /// C++: `bool NBDistrictCont::insert(NBDistrict *const)`
    /// Adds a district to the dictionary
    // bool
    // NBDistrictCont::insert(NBDistrict* const district) {
    //     DistrictCont::const_iterator i = myDistricts.find(district->getID());
    //     if (i != myDistricts.end()) {
    //         return false;
    //     }
    //     myDistricts.insert(DistrictCont::value_type(district->getID(), district));
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn insert(&mut self, district: Ptr<NBDistrict>) -> bool {
        let id = district.get_id().to_string();
        if self.myDistricts.contains_key(&id) {
            return false;
        }
        self.myDistricts.insert(id, district);
        true
    }

    /// C++: `NBDistrict * NBDistrictCont::retrieve(const std::string &) const` [const]
    /// Returns the districts with the given id
    // NBDistrict*
    // NBDistrictCont::retrieve(const std::string& id) const {
    //     DistrictCont::const_iterator i = myDistricts.find(id);
    //     if (i == myDistricts.end()) {
    //         return nullptr;
    //     }
    //     return (*i).second;
    // }
    #[allow(unused_variables)]
    pub fn retrieve(&self, id: &str) -> Ptr<NBDistrict> {
        match self.myDistricts.get(id) {
            Some(d) => *d,
            None => Ptr::null(),
        }
    }

    /// C++: `std::map<std::string, NBDistrict *>::const_iterator NBDistrictCont::begin() const` [const]
    /// Returns the pointer to the begin of the stored districts
    // std::map<std::string, NBDistrict*>::const_iterator begin() const {
    //         return myDistricts.begin();
    //     }
    #[allow(unused_variables)]
    pub fn begin(&self) -> std::collections::btree_map::Iter<'_, String, Ptr<NBDistrict>> {
        self.myDistricts.iter()
    }

    /// C++: `std::map<std::string, NBDistrict *>::const_iterator NBDistrictCont::end() const` [const]
    /// Returns the pointer to the end of the stored districts
    // std::map<std::string, NBDistrict*>::const_iterator end() const {
    //         return myDistricts.end();
    //     }
    #[allow(unused_variables)]
    pub fn end(&self) -> std::collections::btree_map::Iter<'_, String, Ptr<NBDistrict>> {
        // C++ end(): an iterator past the last element. Rust iterators have no past-the-end
        // position, so return an exhausted iterator of the same type.
        let mut it = self.myDistricts.iter();
        while it.next().is_some() {}
        it
    }

    /// C++: `int NBDistrictCont::size() const` [const]
    /// Returns the number of districts inside the container
    // int
    // NBDistrictCont::size() const {
    //     return (int)myDistricts.size();
    // }
    #[allow(unused_variables)]
    pub fn size(&self) -> i32 {
        self.myDistricts.len() as i32
    }

    /// C++: `bool NBDistrictCont::addSource(const std::string &, NBEdge *const, double)`
    /// Adds a source to the named district
    // bool
    // NBDistrictCont::addSource(const std::string& dist, NBEdge* const source,
    //                           double weight) {
    //     NBDistrict* o = retrieve(dist);
    //     if (o == nullptr) {
    //         return false;
    //     }
    //     return o->addSource(source, weight);
    // }
    #[allow(unused_variables)]
    pub fn add_source(&mut self, dist: &str, source: Ptr<NBEdge>, weight: f64) -> bool {
        let o = self.retrieve(dist);
        if o.is_null() {
            return false;
        }
        o.m().add_source(source, weight)
    }

    /// C++: `bool NBDistrictCont::addSink(const std::string &, NBEdge *const, double)`
    /// Adds a sink to the named district
    // bool
    // NBDistrictCont::addSink(const std::string& dist, NBEdge* const destination,
    //                         double weight) {
    //     NBDistrict* o = retrieve(dist);
    //     if (o == nullptr) {
    //         return false;
    //     }
    //     return o->addSink(destination, weight);
    // }
    #[allow(unused_variables)]
    pub fn add_sink(&mut self, dist: &str, destination: Ptr<NBEdge>, weight: f64) -> bool {
        let o = self.retrieve(dist);
        if o.is_null() {
            return false;
        }
        o.m().add_sink(destination, weight)
    }

    /// C++: `void NBDistrictCont::removeFromSinksAndSources(NBEdge *const)`
    /// Removes the given edge from the lists of sources and sinks in all stored districts
    // void
    // NBDistrictCont::removeFromSinksAndSources(NBEdge* const e) {
    //     for (DistrictCont::iterator i = myDistricts.begin(); i != myDistricts.end(); i++) {
    //         (*i).second->removeFromSinksAndSources(e);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remove_from_sinks_and_sources(&mut self, e: Ptr<NBEdge>) {
        for d in self.myDistricts.values() {
            d.m().remove_from_sinks_and_sources(e);
        }
    }

    /// C++: `void NBDistrictCont::NBDistrictCont(const NBDistrictCont &)` [private]
    /// invalid copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &NBDistrictCont) -> Self {
        Self { myDistricts: s.myDistricts.clone() }
    }

    /// C++: `NBDistrictCont & NBDistrictCont::operator=(const NBDistrictCont &)` [private]
    /// invalid assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &NBDistrictCont) -> &mut NBDistrictCont {
        self.myDistricts = s.myDistricts.clone();
        self
    }

}

