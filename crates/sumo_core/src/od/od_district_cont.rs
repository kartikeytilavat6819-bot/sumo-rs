//! Port of od/ODDistrictCont.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// A container for districts
/// C++ class `ODDistrictCont` (od/ODDistrictCont.h)
#[repr(C)]
pub struct ODDistrictCont {
    /// C++ base class `NamedObjectCont<ODDistrict *>` (a std::map keyed by id, see utils/shapes/shape_container.rs)
    pub base: BTreeMap<String, Ptr<ODDistrict>>,
}

impl std::ops::Deref for ODDistrictCont {
    type Target = BTreeMap<String, Ptr<ODDistrict>>;
    fn deref(&self) -> &BTreeMap<String, Ptr<ODDistrict>> {
        &self.base
    }
}
impl std::ops::DerefMut for ODDistrictCont {
    fn deref_mut(&mut self) -> &mut BTreeMap<String, Ptr<ODDistrict>> {
        &mut self.base
    }
}

impl ODDistrictCont {
    /// C++: `void ODDistrictCont::ODDistrictCont()`
    /// Constructor
    // ODDistrictCont::ODDistrictCont() {}
    #[allow(unused_variables)]
    pub fn new() -> Self {
        Self { base: BTreeMap::new() }
    }

    /// C++: `void ODDistrictCont::~ODDistrictCont() noexcept` [virtual]
    /// Destructor
    // ODDistrictCont::~ODDistrictCont() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ body is empty: the container never deletes the districts it holds
    }

    /// C++: `std::string ODDistrictCont::getRandomSourceFromDistrict(const std::string &) const` [const]
    /// Returns the id of a random source from the named district
    // std::string
    // ODDistrictCont::getRandomSourceFromDistrict(const std::string& name) const {
    //     ODDistrict* district = get(name);
    //     if (district == nullptr) {
    //         throw InvalidArgument("There is no district '" + name + "'.");
    //     }
    //     std::string randomSource;
    //     try {
    //         randomSource = district->getRandomSource();
    //     } catch (OutOfBoundsException&) {
    //         throw ProcessError(TLF("District '%' does not provide any valid source.", name));
    //     }
    //     return randomSource;
    // }
    #[allow(unused_variables)]
    pub fn get_random_source_from_district(&self, name: &str) -> String {
        let Some(district) = self.base.get(name) else {
            panic!("{}", ProcessError::InvalidArgument(format!("There is no district '{}'.", name)));
        };
        // shortcut: C++ catches OutOfBoundsException from getRandomSource() and rethrows ProcessError; ODDistrict::get_random_source panics instead of returning an error, so no catch here, upgrade when ODDistrict returns Result
        district.get_random_source()
    }

    /// C++: `std::string ODDistrictCont::getRandomSinkFromDistrict(const std::string &) const` [const]
    /// Returns the id of a random sink from the named district
    // std::string
    // ODDistrictCont::getRandomSinkFromDistrict(const std::string& name) const {
    //     ODDistrict* district = get(name);
    //     if (district == nullptr) {
    //         throw InvalidArgument("There is no district '" + name + "'.");
    //     }
    //     std::string randomSink;
    //     try {
    //         randomSink = district->getRandomSink();
    //     } catch (OutOfBoundsException&) {
    //         throw ProcessError(TLF("District '%' does not provide any valid sink.", name));
    //     }
    //     return randomSink;
    // }
    #[allow(unused_variables)]
    pub fn get_random_sink_from_district(&self, name: &str) -> String {
        let Some(district) = self.base.get(name) else {
            panic!("{}", ProcessError::InvalidArgument(format!("There is no district '{}'.", name)));
        };
        // shortcut: C++ catches OutOfBoundsException from getRandomSink() and rethrows ProcessError; ODDistrict::get_random_sink panics instead of returning an error, so no catch here, upgrade when ODDistrict returns Result
        district.get_random_sink()
    }

    /// C++: `void ODDistrictCont::loadDistricts(std::vector<std::string>)`
    /// load districts from files
    // void
    // ODDistrictCont::loadDistricts(std::vector<std::string> files) {
    //     for (std::vector<std::string>::iterator i = files.begin(); i != files.end(); ++i) {
    //         const std::string& districtfile = *i;
    //         if (!FileHelpers::isReadable(districtfile)) {
    //             throw ProcessError(TLF("Could not access network file '%' to load.", districtfile));
    //         }
    //         PROGRESS_BEGIN_MESSAGE("Loading districts from '" + districtfile + "'");
    //         // build the xml-parser and handler
    //         ODDistrictHandler handler(*this, districtfile);
    //         if (!XMLSubSys::runParser(handler, districtfile, true)) {
    //             PROGRESS_FAILED_MESSAGE();
    //         } else {
    //             PROGRESS_DONE_MESSAGE();
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn load_districts(&mut self, files: Vec<String>) {
        for districtfile in &files {
            if !utils::common::file_helpers::is_readable(districtfile) {
                panic!("{}", ProcessError::new(utils::tlf!("Could not access network file '%' to load.", districtfile)));
            }
            utils::common::msg_handler::progress_begin_message(&format!("Loading districts from '{}'", districtfile));
            // build the xml-parser and handler
            let mut handler = ODDistrictHandler::new(self, districtfile);
            if !utils::xml::xml_sub_sys::run_parser(&mut handler, districtfile, true, false, false, true).unwrap_or_else(|e| panic!("{e}")) {
                utils::common::msg_handler::progress_failed_message();
            } else {
                utils::common::msg_handler::progress_done_message();
            }
        }
    }

    /// C++: `void ODDistrictCont::makeDistricts(const std::map<std::string, std::pair<std::vector<std::string>, std::vector<std::string>>> &)`
    /// create districts from description
    // void
    // ODDistrictCont::makeDistricts(const std::map<std::string, std::pair<std::vector<std::string>, std::vector<std::string> > >& districts) {
    //     for (std::map<std::string, std::pair<std::vector<std::string>, std::vector<std::string> > >::const_iterator it = districts.begin(); it != districts.end(); ++it) {
    //         ODDistrict* current = new ODDistrict(it->first);
    //         const std::vector<std::string>& sources = it->second.first;
    //         for (std::vector<std::string>::const_iterator i = sources.begin(); i != sources.end(); ++i) {
    //             current->addSource(*i, 1.);
    //         }
    //         const std::vector<std::string>& sinks = it->second.second;
    //         for (std::vector<std::string>::const_iterator i = sinks.begin(); i != sinks.end(); ++i) {
    //             current->addSink(*i, 1.);
    //         }
    //         add(current->getID(), current);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn make_districts(&mut self, districts: &BTreeMap<String, (Vec<String>, Vec<String>)>) {
        // std::map iteration order is key order, same as BTreeMap
        for (id, (sources, sinks)) in districts {
            let mut current = Box::new(ODDistrict::new(id));
            for source in sources {
                current.add_source(source, 1.);
            }
            for sink in sinks {
                current.add_sink(sink, 1.);
            }
            let currentID = current.get_id().to_string();
            // C++ `new` with no matching delete: the container holds the district for the rest of the run
            let current: Ptr<ODDistrict> = Ptr::from_mut(Box::leak(current));
            // C++ add(): insert only if the id is free
            self.base.entry(currentID).or_insert(current);
        }
    }

    /// C++: `void ODDistrictCont::ODDistrictCont(const ODDistrictCont &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &ODDistrictCont) -> Self {
        panic!("ODDistrictCont copy constructor is private and never defined in C++")
    }

    /// C++: `ODDistrictCont & ODDistrictCont::operator=(const ODDistrictCont &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &ODDistrictCont) -> &mut ODDistrictCont {
        panic!("ODDistrictCont assignment operator is private and never defined in C++")
    }

}

