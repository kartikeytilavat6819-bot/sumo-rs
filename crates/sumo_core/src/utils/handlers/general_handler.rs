//! Port of utils/handlers/GeneralHandler.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `GeneralHandler` (utils/handlers/GeneralHandler.h)
#[repr(C)]
pub struct GeneralHandler {
    /// C++ base class `SUMOSAXHandler`
    pub base: GenericSAXHandler,
}

impl std::ops::Deref for GeneralHandler {
    type Target = GenericSAXHandler;
    fn deref(&self) -> &GenericSAXHandler {
        &self.base
    }
}
impl std::ops::DerefMut for GeneralHandler {
    fn deref_mut(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
}

impl GeneralHandler {
    /// C++: `void GeneralHandler::GeneralHandler(FileBucket *)`
    /// Constructor
    // GeneralHandler::GeneralHandler(FileBucket* fileBucket) :
    //     SUMOSAXHandler(fileBucket->getFilename()) {
    // }
    pub fn new(fileBucket: Ptr<utils::common::file_bucket::FileBucket>) -> Self {
        let file_name = fileBucket.get().expect("GeneralHandler: null FileBucket").get_filename();
        GeneralHandler { base: utils::xml::sumo_sax_handler::new(file_name, "") }
    }

    /// C++: `void GeneralHandler::~GeneralHandler() noexcept` [virtual]
    /// Destructor
    // GeneralHandler::~GeneralHandler() {}
    pub fn drop_(&mut self) {}

    /// C++: `bool GeneralHandler::parse()`
    /// parse
    // bool
    // GeneralHandler::parse() {
    //     return XMLSubSys::runParser(*this, getFileName());
    // }
    pub fn parse(&mut self) -> bool {
        let file = self.base.get_file_name().to_string();
        // run_parser only returns Err when catch_exceptions is false, and here it is true
        utils::xml::xml_sub_sys::run_parser(self, &file, false, false, false, true).unwrap_or(false)
    }

    /// C++: `void GeneralHandler::beginTag(SumoXMLTag, const SUMOSAXAttributes &)` [virtual, pure virtual, private]
    /// start element
    // (pure virtual: no C++ body; subclasses implement it)
    pub fn begin_tag(&mut self, tag: SumoXMLTag, attrs: &SUMOSAXAttributes) {}

    /// C++: `void GeneralHandler::endTag()` [virtual, pure virtual, private]
    /// end element
    // (pure virtual: no C++ body; subclasses implement it)
    pub fn end_tag(&mut self) {}

    /// C++: `void GeneralHandler::myStartElement(int, const SUMOSAXAttributes &)` [virtual, private]
    /// Called on the opening of a tag;
    // void
    // GeneralHandler::myStartElement(int element, const SUMOSAXAttributes& attrs) {
    //     // obtain tag
    //     const SumoXMLTag tag = (element == 0) ? SUMO_TAG_ROOTFILE : static_cast<SumoXMLTag>(element);
    //     // begin tag
    //     beginTag(tag, attrs);
    // }
    pub fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) {
        // obtain tag
        let tag: SumoXMLTag = if element == 0 { utils::xml::sumo_xml_definitions::SUMO_TAG_ROOTFILE } else { element };
        // begin tag
        GeneralHandler::begin_tag(self, tag, attrs);
    }

    /// C++: `void GeneralHandler::myEndElement(int)` [virtual, private]
    /// Called when a closing tag occurs
    // void
    // GeneralHandler::myEndElement(int /*element*/) {
    //     // end tag
    //     endTag();
    // }
    pub fn my_end_element(&mut self, element: i32) {
        // end tag
        GeneralHandler::end_tag(self);
    }
}

impl SaxHandler for GeneralHandler {
    fn handler(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
    fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) -> Result<(), ProcessError> {
        GeneralHandler::my_start_element(self, element, attrs);
        Ok(())
    }
    fn my_end_element(&mut self, element: i32) -> Result<(), ProcessError> {
        GeneralHandler::my_end_element(self, element);
        Ok(())
    }
}
