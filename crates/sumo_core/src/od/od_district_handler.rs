//! Port of od/ODDistrictHandler.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use utils::xml::generic_sax_handler::GenericSAXHandler;
use crate::prelude::*;

/// An XML-Handler for districts
/// C++ class `ODDistrictHandler` (od/ODDistrictHandler.h)
#[repr(C)]
pub struct ODDistrictHandler {
    /// C++ base class `SUMOSAXHandler`
    pub base: GenericSAXHandler,
    /// The container to add read districts to
    pub myContainer: Ptr<ODDistrictCont>,
    /// The currently parsed district
    pub myCurrentDistrict: Ptr<ODDistrict>,
}

impl std::ops::Deref for ODDistrictHandler {
    type Target = GenericSAXHandler;
    fn deref(&self) -> &GenericSAXHandler {
        &self.base
    }
}
impl std::ops::DerefMut for ODDistrictHandler {
    fn deref_mut(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
}

impl ODDistrictHandler {
    /// C++: `void ODDistrictHandler::ODDistrictHandler(ODDistrictCont &, const std::string &)`
    /// Constructor
    // ODDistrictHandler::ODDistrictHandler(ODDistrictCont& cont,
    //                                      const std::string& file)
    //     : SUMOSAXHandler(file), myContainer(cont), myCurrentDistrict(nullptr) {}
    #[allow(unused_variables)]
    pub fn new(cont: &mut ODDistrictCont, file: &str) -> Self {
        ODDistrictHandler {
            base: GenericSAXHandler::new(TAGS, SUMO_TAG_NOTHING, ATTRS, SUMO_ATTR_NOTHING, file, ""),
            myContainer: Ptr::from_mut(cont),
            myCurrentDistrict: Ptr::from_raw(std::ptr::null_mut()),
        }
    }

    /// C++: `void ODDistrictHandler::~ODDistrictHandler()` [virtual]
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // implicit C++ destructor, nothing to release
    }

    /// C++: `void ODDistrictHandler::myStartElement(int, const SUMOSAXAttributes &)` [virtual, protected]
    /// Called when an opening-tag occurs
    // void
    // ODDistrictHandler::myStartElement(int element,
    //                                   const SUMOSAXAttributes& attrs) {
    //     switch (element) {
    //         case SUMO_TAG_TAZ:
    //             openDistrict(attrs);
    //             break;
    //         case SUMO_TAG_TAZSOURCE:
    //             addSource(attrs);
    //             break;
    //         case SUMO_TAG_TAZSINK:
    //             addSink(attrs);
    //             break;
    //         default:
    //             break;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) {
        match element {
            SUMO_TAG_TAZ => self.open_district(attrs),
            SUMO_TAG_TAZSOURCE => self.add_source(attrs),
            SUMO_TAG_TAZSINK => self.add_sink(attrs),
            _ => {}
        }
    }

    /// C++: `void ODDistrictHandler::myEndElement(int)` [virtual, protected]
    /// Called when a closing tag occurs
    // void
    // ODDistrictHandler::myEndElement(int element) {
    //     if (element == SUMO_TAG_TAZ) {
    //         closeDistrict();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_end_element(&mut self, element: i32) {
        if element == SUMO_TAG_TAZ {
            self.close_district();
        }
    }

    /// C++: `void ODDistrictHandler::openDistrict(const SUMOSAXAttributes &)` [private]
    /// Begins the parsing of a district
    // void
    // ODDistrictHandler::openDistrict(const SUMOSAXAttributes& attrs) {
    //     myCurrentDistrict = nullptr;
    //     // get the id, report an error if not given or empty...
    //     bool ok = true;
    //     std::string id = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     if (!ok) {
    //         return;
    //     }
    //     myCurrentDistrict = new ODDistrict(id);
    //     if (attrs.hasAttribute(SUMO_ATTR_EDGES)) {
    //         const std::vector<std::string>& desc = attrs.get<std::vector<std::string> >(SUMO_ATTR_EDGES, id.c_str(), ok);
    //         for (const std::string& eID : desc) {
    //             myCurrentDistrict->addSource(eID, 1.);
    //             myCurrentDistrict->addSink(eID, 1.);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn open_district(&mut self, attrs: &SUMOSAXAttributes) {
        self.myCurrentDistrict = Ptr::from_raw(std::ptr::null_mut());
        // get the id, report an error if not given or empty...
        let mut ok = true;
        let id = attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
        if !ok {
            return;
        }
        // shortcut: C++ `new ODDistrict` is owned by the container and deleted there; leaked here, ODDistrictCont has no owning drop yet
        self.myCurrentDistrict = Ptr::from_mut(Box::leak(Box::new(ODDistrict::new(&id))));
        if attrs.has_attribute(SUMO_ATTR_EDGES) {
            let desc = attrs.get::<Vec<String>>(SUMO_ATTR_EDGES, &id, &mut ok, true);
            for eID in desc.iter() {
                self.myCurrentDistrict.add_source(eID, 1.);
                self.myCurrentDistrict.add_sink(eID, 1.);
            }
        }
    }

    /// C++: `void ODDistrictHandler::addSource(const SUMOSAXAttributes &)` [private]
    /// Adds a read source to the current district
    // void
    // ODDistrictHandler::addSource(const SUMOSAXAttributes& attrs) {
    //     std::pair<std::string, double> vals = parseTAZ(attrs);
    //     if (vals.second >= 0) {
    //         myCurrentDistrict->addSource(vals.first, vals.second);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_source(&mut self, attrs: &SUMOSAXAttributes) {
        let vals = self.parse_taz(attrs);
        if vals.1 >= 0. {
            self.myCurrentDistrict.add_source(&vals.0, vals.1);
        }
    }

    /// C++: `void ODDistrictHandler::addSink(const SUMOSAXAttributes &)` [private]
    /// Adds a read sink to the current district
    // void
    // ODDistrictHandler::addSink(const SUMOSAXAttributes& attrs) {
    //     std::pair<std::string, double> vals = parseTAZ(attrs);
    //     if (vals.second >= 0) {
    //         myCurrentDistrict->addSink(vals.first, vals.second);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_sink(&mut self, attrs: &SUMOSAXAttributes) {
        let vals = self.parse_taz(attrs);
        if vals.1 >= 0. {
            self.myCurrentDistrict.add_sink(&vals.0, vals.1);
        }
    }

    /// C++: `void ODDistrictHandler::closeDistrict()` [private]
    /// Closes the processing of the current district
    // void
    // ODDistrictHandler::closeDistrict() {
    //     if (myCurrentDistrict != nullptr) {
    //         myContainer.add(myCurrentDistrict->getID(), myCurrentDistrict);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn close_district(&mut self) {
        if !self.myCurrentDistrict.is_null() {
            let id = self.myCurrentDistrict.get_id().to_string();
            // NamedObjectCont::add: keeps the existing entry on a duplicate id
            let cont = self.myContainer.m();
            if !cont.contains_key(&id) {
                cont.insert(id, self.myCurrentDistrict);
            }
        }
    }

    /// C++: `std::pair<std::string, double> ODDistrictHandler::parseTAZ(const SUMOSAXAttributes &)` [private]
    /// Returns the id and weight for a taz/tazSink/tazSource
    // std::pair<std::string, double>
    // ODDistrictHandler::parseTAZ(const SUMOSAXAttributes& attrs) {
    //     // check the current district first
    //     if (myCurrentDistrict == nullptr) {
    //         return std::pair<std::string, double>("", -1);
    //     }
    //     // get the id, report an error if not given or empty...
    //     bool ok = true;
    //     std::string id = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     if (!ok) {
    //         return std::pair<std::string, double>("", -1);
    //     }
    //     // get the weight
    //     double weight = attrs.get<double>(SUMO_ATTR_WEIGHT, id.c_str(), ok);
    //     if (ok) {
    //         if (weight < 0) {
    //             WRITE_ERRORF(TL("'probability' must be positive (in definition of % '%')."), attrs.getObjectType(), id);
    //         } else {
    //             return std::pair<std::string, double>(id, weight);
    //         }
    //     }
    //     return std::pair<std::string, double>("", -1);
    // }
    #[allow(unused_variables)]
    pub fn parse_taz(&mut self, attrs: &SUMOSAXAttributes) -> (String, f64) {
        // check the current district first
        if self.myCurrentDistrict.is_null() {
            return (String::new(), -1.);
        }
        // get the id, report an error if not given or empty...
        let mut ok = true;
        let id = attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
        if !ok {
            return (String::new(), -1.);
        }
        // get the weight
        let weight = attrs.get::<f64>(SUMO_ATTR_WEIGHT, &id, &mut ok, true);
        if ok {
            if weight < 0. {
                write_errorf!("'probability' must be positive (in definition of % '%').", attrs.get_object_type().to_string(), id);
            } else {
                return (id, weight);
            }
        }
        (String::new(), -1.)
    }

    /// C++: `void ODDistrictHandler::ODDistrictHandler(const ODDistrictHandler &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &ODDistrictHandler) -> Self {
        panic!("ODDistrictHandler copy constructor is private and undefined in C++")
    }

    /// C++: `ODDistrictHandler & ODDistrictHandler::operator=(const ODDistrictHandler &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &ODDistrictHandler) -> &mut ODDistrictHandler {
        panic!("ODDistrictHandler assignment operator is private and undefined in C++")
    }

}

const TAGS: &[(&str, i32)] = &[
    ("taz", SUMO_TAG_TAZ),
    ("tazSource", SUMO_TAG_TAZSOURCE),
    ("tazSink", SUMO_TAG_TAZSINK),
    ("", SUMO_TAG_NOTHING),
];
const ATTRS: &[(&str, i32)] = &[
    ("id", SUMO_ATTR_ID),
    ("edges", SUMO_ATTR_EDGES),
    ("weight", SUMO_ATTR_WEIGHT),
    ("", SUMO_ATTR_NOTHING),
];

impl SaxHandler for ODDistrictHandler {
    fn handler(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
    fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) -> Result<(), ProcessError> {
        ODDistrictHandler::my_start_element(self, element, attrs);
        Ok(())
    }
    fn my_end_element(&mut self, element: i32) -> Result<(), ProcessError> {
        ODDistrictHandler::my_end_element(self, element);
        Ok(())
    }
}

