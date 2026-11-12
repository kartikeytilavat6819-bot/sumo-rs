//! Port of od/ODDistrict.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// A district (origin/destination)
/// C++ class `ODDistrict` (od/ODDistrict.h)
#[repr(C)]
pub struct ODDistrict {
    /// C++ base class `Named`
    pub base: Named,
    /// Container of weighted sources
    pub mySources: RandomDistributor<String>,
    /// Container of weighted sinks
    pub mySinks: RandomDistributor<String>,
}

impl std::ops::Deref for ODDistrict {
    type Target = Named;
    fn deref(&self) -> &Named {
        &self.base
    }
}
impl std::ops::DerefMut for ODDistrict {
    fn deref_mut(&mut self) -> &mut Named {
        &mut self.base
    }
}

impl ODDistrict {
    /// C++: `void ODDistrict::ODDistrict(const std::string &)`
    /// Constructor
    // ODDistrict::ODDistrict(const std::string& id)
    //     : Named(id) {}
    #[allow(unused_variables)]
    pub fn new(id: &str) -> Self {
        ODDistrict { base: Named::new(id), mySources: RandomDistributor::default(), mySinks: RandomDistributor::default() }
    }

    /// C++: `void ODDistrict::~ODDistrict() noexcept` [virtual]
    /// Destructor
    // ODDistrict::~ODDistrict() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
    }

    /// C++: `void ODDistrict::addSource(const std::string &, double)`
    /// Adds a source connection
    // void
    // ODDistrict::addSource(const std::string& id, double weight) {
    //     mySources.add(id, weight, false);
    // }
    #[allow(unused_variables)]
    pub fn add_source(&mut self, id: &str, weight: f64) {
        self.mySources.add(id.to_string(), weight, false);
    }

    /// C++: `void ODDistrict::addSink(const std::string &, double)`
    /// Adds a sink connection
    // void
    // ODDistrict::addSink(const std::string& id, double weight) {
    //     mySinks.add(id, weight, false);
    // }
    #[allow(unused_variables)]
    pub fn add_sink(&mut self, id: &str, weight: f64) {
        self.mySinks.add(id.to_string(), weight, false);
    }

    /// C++: `std::string ODDistrict::getRandomSource() const` [const]
    /// Returns the id of a source to use
    // std::string
    // ODDistrict::getRandomSource() const {
    //     return mySources.get();
    // }
    #[allow(unused_variables)]
    pub fn get_random_source(&self) -> String {
        // shortcut: C++ get() throws ProcessError on an empty distribution and this signature has no error channel, so it panics, upgrade by returning Result
        self.mySources.get(None).unwrap()
    }

    /// C++: `std::string ODDistrict::getRandomSink() const` [const]
    /// Returns the id of a sink to use
    // std::string
    // ODDistrict::getRandomSink() const {
    //     return mySinks.get();
    // }
    #[allow(unused_variables)]
    pub fn get_random_sink(&self) -> String {
        // shortcut: C++ get() throws ProcessError on an empty distribution and this signature has no error channel, so it panics, upgrade by returning Result
        self.mySinks.get(None).unwrap()
    }

    /// C++: `int ODDistrict::sinkNumber() const` [const]
    /// Returns the number of sinks
    // int
    // ODDistrict::sinkNumber() const {
    //     return (int) mySinks.getVals().size();
    // }
    #[allow(unused_variables)]
    pub fn sink_number(&self) -> i32 {
        self.mySinks.get_vals().len() as i32
    }

    /// C++: `int ODDistrict::sourceNumber() const` [const]
    /// Returns the number of sources
    // int
    // ODDistrict::sourceNumber() const {
    //     return (int) mySources.getVals().size();
    // }
    #[allow(unused_variables)]
    pub fn source_number(&self) -> i32 {
        self.mySources.get_vals().len() as i32
    }

    /// C++: `void ODDistrict::ODDistrict(const ODDistrict &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &ODDistrict) -> Self {
        // C++ declares this copy constructor private and never defines it, so no caller exists
        panic!("ODDistrict copy constructor is private and undefined in C++")
    }

    /// C++: `ODDistrict & ODDistrict::operator=(const ODDistrict &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &ODDistrict) -> &mut ODDistrict {
        // C++ declares this assignment operator private and never defines it, so no caller exists
        panic!("ODDistrict::operator= is private and undefined in C++")
    }

}

