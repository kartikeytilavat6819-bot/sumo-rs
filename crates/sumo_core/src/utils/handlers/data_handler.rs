//! Port of utils/handlers/DataHandler.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::file_bucket::FileBucket;
use utils::xml::common_xml_structure::CommonXMLStructure_SumoBaseObject;
use utils::xml::generic_sax_handler::SaxHandler;
use utils::common::msg_handler::{write_error, write_warning};
use utils::xml::sumo_xml_definitions::{
    ATTRS, SUMO_ATTR_BEGIN, SUMO_ATTR_END, SUMO_ATTR_FROM, SUMO_ATTR_ID, SUMO_ATTR_TO, SUMO_TAG_EDGE, SUMO_TAG_EDGEREL, SUMO_TAG_ERROR, SUMO_TAG_INTERVAL,
    SUMO_TAG_PARAM, SUMO_TAG_ROOTFILE, SUMO_TAG_TAZREL, TAGS,
};
use utils::common::time::steps2time;

/// The XML-Handler for network loading
/// C++ class `DataHandler` (utils/handlers/DataHandler.h)
#[repr(C)]
pub struct DataHandler {
    /// C++ base class `CommonHandler`
    pub base: CommonHandler,
    /// C++ base class `SUMOSAXHandler`
    pub base2: GenericSAXHandler,
}

/// C++ `toString(SumoXMLTag)`: the XML name of a tag (first table entry with that value)
fn tag_name(tag: SumoXMLTag) -> &'static str {
    TAGS.iter().find(|(_, v)| *v == tag).map(|(n, _)| *n).unwrap_or("")
}

/// C++ `toString(SumoXMLAttr)`: the XML name of an attribute (first table entry with that value)
fn attr_name(attr: SumoXMLAttr) -> &'static str {
    ATTRS.iter().find(|(_, v)| *v == attr).map(|(n, _)| *n).unwrap_or("")
}

impl std::ops::Deref for DataHandler {
    type Target = CommonHandler;
    fn deref(&self) -> &CommonHandler {
        &self.base
    }
}
impl std::ops::DerefMut for DataHandler {
    fn deref_mut(&mut self) -> &mut CommonHandler {
        &mut self.base
    }
}

impl DataHandler {
    /// C++: `void DataHandler::DataHandler(FileBucket *)`
    /// Constructor
    // DataHandler::DataHandler(FileBucket* fileBucket) :
    //     CommonHandler(fileBucket),
    //     SUMOSAXHandler(fileBucket->getFilename()) {
    // }
    pub fn new(fileBucket: Ptr<FileBucket>) -> Self {
        let file_name = fileBucket.get_filename().to_string();
        Self {
            base: CommonHandler::new(fileBucket),
            base2: utils::xml::sumo_sax_handler::new(&file_name, ""),
        }
    }

    /// C++: `void DataHandler::~DataHandler() noexcept` [virtual]
    /// Destructor
    // DataHandler::~DataHandler() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty; Rust drops the fields.
    }

    /// C++: `bool DataHandler::parse()`
    /// parse
    // bool
    // DataHandler::parse() {
    //     // run parser and return result
    //     return XMLSubSys::runParser(*this, getFileName());
    // }
    #[allow(unused_variables)]
    pub fn parse(&mut self) -> bool {
        // run parser and return result
        let file = self.base2.get_file_name().to_string();
        utils::xml::xml_sub_sys::run_parser(self, &file, false, false, false, true).unwrap_or_else(|e| {
            write_error(&e.to_string());
            false
        })
    }

    /// C++: `void DataHandler::parseSumoBaseObject(CommonXMLStructure::SumoBaseObject *)`
    /// parse SumoBaseObject (it's called recursivelly)
    // void
    // DataHandler::parseSumoBaseObject(CommonXMLStructure::SumoBaseObject* obj) {
    //     // check if loading was aborted
    //     if (!myAbortLoading) {
    //         // switch tag
    //         switch (obj->getTag()) {
    //             // Stopping Places
    //             case SUMO_TAG_INTERVAL:
    //                 if (buildDataInterval(obj,
    //                                       obj->getStringAttribute(SUMO_ATTR_ID),
    //                                       obj->getDoubleAttribute(SUMO_ATTR_BEGIN),
    //                                       obj->getDoubleAttribute(SUMO_ATTR_END))) {
    //                     obj->markAsCreated();
    //                 }
    //                 break;
    //             case SUMO_TAG_EDGE:
    //                 if (buildEdgeData(obj,
    //                                   obj->getStringAttribute(SUMO_ATTR_ID),
    //                                   obj->getParameters())) {
    //                     obj->markAsCreated();
    //                 }
    //                 break;
    //             case SUMO_TAG_EDGEREL:
    //                 if (buildEdgeRelationData(obj,
    //                                           obj->getStringAttribute(SUMO_ATTR_FROM),
    //                                           obj->getStringAttribute(SUMO_ATTR_TO),
    //                                           obj->getParameters())) {
    //                     obj->markAsCreated();
    //                 }
    //                 break;
    //             case SUMO_TAG_TAZREL:
    //                 if (buildTAZRelationData(obj,
    //                                          obj->getStringAttribute(SUMO_ATTR_FROM),
    //                                          obj->getStringAttribute(SUMO_ATTR_TO),
    //                                          obj->getParameters())) {
    //                     obj->markAsCreated();
    //                 }
    //                 break;
    //             default:
    //                 break;
    //         }
    //         // now iterate over childrens
    //         for (const auto& child : obj->getSumoBaseObjectChildren()) {
    //             // call this function recursively
    //             parseSumoBaseObject(child);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_sumo_base_object(&mut self, obj: Ptr<CommonXMLStructure_SumoBaseObject>) -> Result<(), ProcessError> {
        // check if loading was aborted
        if !self.myAbortLoading {
            // switch tag
            match obj.get_tag() {
                // Stopping Places
                SUMO_TAG_INTERVAL => {
                    if self.build_data_interval(obj, obj.get_string_attribute(SUMO_ATTR_ID)?, obj.get_double_attribute(SUMO_ATTR_BEGIN)?, obj.get_double_attribute(SUMO_ATTR_END)?) {
                        obj.m().mark_as_created();
                    }
                }
                SUMO_TAG_EDGE => {
                    if self.build_edge_data(obj, obj.get_string_attribute(SUMO_ATTR_ID)?, obj.get_parameters()) {
                        obj.m().mark_as_created();
                    }
                }
                SUMO_TAG_EDGEREL => {
                    if self.build_edge_relation_data(obj, obj.get_string_attribute(SUMO_ATTR_FROM)?, obj.get_string_attribute(SUMO_ATTR_TO)?, obj.get_parameters()) {
                        obj.m().mark_as_created();
                    }
                }
                SUMO_TAG_TAZREL => {
                    if self.build_taz_relation_data(obj, obj.get_string_attribute(SUMO_ATTR_FROM)?, obj.get_string_attribute(SUMO_ATTR_TO)?, obj.get_parameters()) {
                        obj.m().mark_as_created();
                    }
                }
                _ => {}
            }
            // now iterate over childrens
            for child in obj.get_sumo_base_object_children().iter() {
                // call this function recursively
                self.parse_sumo_base_object(Ptr::from_box(child))?;
            }
        }
        Ok(())
    }

    /// C++: `bool DataHandler::buildDataInterval(const CommonXMLStructure::SumoBaseObject *, const std::string &, const double, const double)` [virtual, pure virtual]
    /// Builds DataInterval
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_data_interval(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, dataSetID: &str, begin: f64, end: f64) -> bool {
        // pure virtual in C++ (no base body); subclasses override. Nothing to build at this level.
        false
    }

    /// C++: `bool DataHandler::buildEdgeData(const CommonXMLStructure::SumoBaseObject *, const std::string &, const Parameterised::Map &)` [virtual, pure virtual]
    /// Builds edgeData
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_edge_data(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, edgeID: &str, parameters: &BTreeMap<String, String>) -> bool {
        // pure virtual in C++ (no base body); subclasses override. Nothing to build at this level.
        false
    }

    /// C++: `bool DataHandler::buildEdgeRelationData(const CommonXMLStructure::SumoBaseObject *, const std::string &, const std::string &, const Parameterised::Map &)` [virtual, pure virtual]
    /// Builds edgeRelationData
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_edge_relation_data(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, fromEdgeID: &str, toEdgeID: &str, parameters: &BTreeMap<String, String>) -> bool {
        // pure virtual in C++ (no base body); subclasses override. Nothing to build at this level.
        false
    }

    /// C++: `bool DataHandler::buildTAZRelationData(const CommonXMLStructure::SumoBaseObject *, const std::string &, const std::string &, const Parameterised::Map &)` [virtual, pure virtual]
    /// Builds TAZRelationData
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_taz_relation_data(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, fromTAZID: &str, toTAZID: &str, parameters: &BTreeMap<String, String>) -> bool {
        // pure virtual in C++ (no base body); subclasses override. Nothing to build at this level.
        false
    }

    /// C++: `void DataHandler::myStartElement(int, const SUMOSAXAttributes &)` [virtual, private]
    /// Called on the opening of a tag;
    // void
    // DataHandler::myStartElement(int element, const SUMOSAXAttributes& attrs) {
    //     // obtain tag
    //     const SumoXMLTag tag = (element == 0) ? SUMO_TAG_ROOTFILE : static_cast<SumoXMLTag>(element);
    //     // open SUMOBaseOBject
    //     myCommonXMLStructure.openSUMOBaseOBject();
    //     // check tag
    //     try {
    //         switch (tag) {
    //             // interval
    //             case SUMO_TAG_INTERVAL:
    //                 parseInterval(attrs);
    //                 break;
    //             // datas
    //             case SUMO_TAG_EDGE:
    //                 parseEdgeData(attrs);
    //                 break;
    //             case SUMO_TAG_EDGEREL:
    //                 parseEdgeRelationData(attrs);
    //                 break;
    //             case SUMO_TAG_TAZREL:
    //                 parseTAZRelationData(attrs);
    //                 break;
    //             case SUMO_TAG_PARAM:
    //                 WRITE_WARNING(TL("Data elements cannot load attributes as params"));
    //                 myCommonXMLStructure.abortSUMOBaseOBject();
    //                 break;
    //             default:
    //                 // tag cannot be parsed in routeHandler
    //                 myCommonXMLStructure.abortSUMOBaseOBject();
    //                 break;
    //         }
    //     } catch (InvalidArgument& e) {
    //         writeError(e.what());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) {
        // obtain tag
        let tag = if element == 0 { SUMO_TAG_ROOTFILE } else { element };
        // open SUMOBaseOBject
        self.myCommonXMLStructure.open_sumo_base_object();
        // check tag
        // (C++ catches InvalidArgument and calls writeError(e.what()); the parse_* helpers here use get<T>, which does not throw)
        match tag {
            // interval
            SUMO_TAG_INTERVAL => self.parse_interval(attrs),
            // datas
            SUMO_TAG_EDGE => self.parse_edge_data(attrs),
            SUMO_TAG_EDGEREL => self.parse_edge_relation_data(attrs),
            SUMO_TAG_TAZREL => self.parse_taz_relation_data(attrs),
            SUMO_TAG_PARAM => {
                write_warning("Data elements cannot load attributes as params");
                self.myCommonXMLStructure.abort_sumo_base_object();
            }
            // tag cannot be parsed in routeHandler
            _ => {
                self.myCommonXMLStructure.abort_sumo_base_object();
            }
        }
    }

    /// C++: `void DataHandler::myEndElement(int)` [virtual, private]
    /// Called when a closing tag occurs
    // void
    // DataHandler::myEndElement(int element) {
    //     // obtain tag
    //     const SumoXMLTag tag = static_cast<SumoXMLTag>(element);
    //     // get last inserted object
    //     CommonXMLStructure::SumoBaseObject* obj = myCommonXMLStructure.getCurrentSumoBaseObject();
    //     // close SUMOBaseOBject
    //     myCommonXMLStructure.closeSUMOBaseOBject();
    //     if (obj) {
    //         // check tag
    //         switch (tag) {
    //             // only interval
    //             case SUMO_TAG_INTERVAL:
    //                 // parse object and all their childrens
    //                 parseSumoBaseObject(obj);
    //                 // delete object (and all of their childrens)
    //                 delete obj;
    //                 break;
    //             default:
    //                 break;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_end_element(&mut self, element: i32) -> Result<(), ProcessError> {
        // obtain tag
        let tag = element;
        // get last inserted object
        let obj = self.myCommonXMLStructure.get_current_sumo_base_object();
        // close SUMOBaseOBject
        self.myCommonXMLStructure.close_sumo_base_object();
        if !obj.is_null() {
            // check tag
            match tag {
                // only interval
                SUMO_TAG_INTERVAL => {
                    // parse object and all their childrens
                    self.parse_sumo_base_object(obj)?;
                    // delete object (and all of their childrens): the parent drops its owned child
                    let parent = obj.get_parent_sumo_base_object();
                    if !parent.is_null() {
                        parent.m().remove_sumo_base_object_child(obj);
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    /// C++: `void DataHandler::parseInterval(const SUMOSAXAttributes &)` [private]
    /// parse interval attributes
    // void
    // DataHandler::parseInterval(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // needed attributes
    //     const std::string id = attrs.get<std::string>(SUMO_ATTR_ID, "", parsedOk);
    //     const double begin = STEPS2TIME(attrs.getSUMOTimeReporting(SUMO_ATTR_BEGIN, "", parsedOk));
    //     const double end = STEPS2TIME(attrs.getSUMOTimeReporting(SUMO_ATTR_END, "", parsedOk));
    //     // continue if flag is ok
    //     if (parsedOk) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_INTERVAL);
    //         // add all attributes
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_ID, id);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_BEGIN, begin);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_END, end);
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_interval(&mut self, attrs: &SUMOSAXAttributes) {
        // declare Ok Flag
        let mut parsedOk = true;
        // needed attributes
        let id = attrs.get::<String>(SUMO_ATTR_ID, "", &mut parsedOk, true);
        let begin = steps2time(attrs.get_sumo_time_reporting(SUMO_ATTR_BEGIN, "", &mut parsedOk, true));
        let end = steps2time(attrs.get_sumo_time_reporting(SUMO_ATTR_END, "", &mut parsedOk, true));
        // continue if flag is ok
        if parsedOk {
            // set tag
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_INTERVAL);
            // add all attributes
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_ID, &id);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_BEGIN, begin);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_END, end);
        } else {
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_ERROR);
        }
    }

    /// C++: `void DataHandler::parseEdgeData(const SUMOSAXAttributes &)` [private]
    /// parse edgeData attributes
    // void
    // DataHandler::parseEdgeData(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // needed attributes
    //     const std::string id = attrs.get<std::string>(SUMO_ATTR_ID, "", parsedOk);
    //     // fill attributes
    //     getAttributes(attrs, {SUMO_ATTR_ID});
    //     // continue if flag is ok
    //     if (parsedOk) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_EDGE);
    //         // add all attributes
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_ID, id);
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_edge_data(&mut self, attrs: &SUMOSAXAttributes) {
        // declare Ok Flag
        let mut parsedOk = true;
        // needed attributes
        let id = attrs.get::<String>(SUMO_ATTR_ID, "", &mut parsedOk, true);
        // fill attributes
        self.get_attributes(attrs, vec![SUMO_ATTR_ID]);
        // continue if flag is ok
        if parsedOk {
            // set tag
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_EDGE);
            // add all attributes
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_ID, &id);
        } else {
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_ERROR);
        }
    }

    /// C++: `void DataHandler::parseEdgeRelationData(const SUMOSAXAttributes &)` [private]
    /// parse edgeRelationData attributes
    // void
    // DataHandler::parseEdgeRelationData(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // needed attributes
    //     const std::string from = attrs.get<std::string>(SUMO_ATTR_FROM, "", parsedOk);
    //     const std::string to = attrs.get<std::string>(SUMO_ATTR_TO, "", parsedOk);
    //     // fill attributes
    //     getAttributes(attrs, {SUMO_ATTR_FROM, SUMO_ATTR_TO});
    //     // continue if flag is ok
    //     if (parsedOk) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_EDGEREL);
    //         // add all attributes
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_FROM, from);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_TO, to);
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_edge_relation_data(&mut self, attrs: &SUMOSAXAttributes) {
        // declare Ok Flag
        let mut parsedOk = true;
        // needed attributes
        let from = attrs.get::<String>(SUMO_ATTR_FROM, "", &mut parsedOk, true);
        let to = attrs.get::<String>(SUMO_ATTR_TO, "", &mut parsedOk, true);
        // fill attributes
        self.get_attributes(attrs, vec![SUMO_ATTR_FROM, SUMO_ATTR_TO]);
        // continue if flag is ok
        if parsedOk {
            // set tag
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_EDGEREL);
            // add all attributes
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_FROM, &from);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_TO, &to);
        } else {
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_ERROR);
        }
    }

    /// C++: `void DataHandler::parseTAZRelationData(const SUMOSAXAttributes &)` [private]
    /// parse TAZRelationData attributes
    // void
    // DataHandler::parseTAZRelationData(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // needed attributes
    //     const std::string from = attrs.get<std::string>(SUMO_ATTR_FROM, "", parsedOk);
    //     const std::string to = attrs.get<std::string>(SUMO_ATTR_TO, "", parsedOk);
    //     // fill attributes
    //     getAttributes(attrs, {SUMO_ATTR_FROM, SUMO_ATTR_TO});
    //     // continue if flag is ok
    //     if (parsedOk) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_TAZREL);
    //         // add all attributes
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_FROM, from);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_TO, to);
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_taz_relation_data(&mut self, attrs: &SUMOSAXAttributes) {
        // declare Ok Flag
        let mut parsedOk = true;
        // needed attributes
        let from = attrs.get::<String>(SUMO_ATTR_FROM, "", &mut parsedOk, true);
        let to = attrs.get::<String>(SUMO_ATTR_TO, "", &mut parsedOk, true);
        // fill attributes
        self.get_attributes(attrs, vec![SUMO_ATTR_FROM, SUMO_ATTR_TO]);
        // continue if flag is ok
        if parsedOk {
            // set tag
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_TAZREL);
            // add all attributes
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_FROM, &from);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_TO, &to);
        } else {
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_ERROR);
        }
    }

    /// C++: `void DataHandler::getAttributes(const SUMOSAXAttributes &, const std::vector<SumoXMLAttr>) const` [const, private]
    /// parse attributes as parameters
    // void
    // DataHandler::getAttributes(const SUMOSAXAttributes& attrs, const std::vector<SumoXMLAttr> avoidAttributes) const {
    //     // transform avoidAttributes to strings
    //     std::vector<std::string> avoidAttributesStr;
    //     for (const SumoXMLAttr& avoidAttribute : avoidAttributes) {
    //         avoidAttributesStr.push_back(toString(avoidAttribute));
    //     }
    //     // iterate over attributes and fill parameters map
    //     for (const std::string& attribute : attrs.getAttributeNames()) {
    //         if (std::find(avoidAttributesStr.begin(), avoidAttributesStr.end(), attribute) == avoidAttributesStr.end()) {
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->addParameter(attribute, attrs.getStringSecure(attribute, ""));
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_attributes(&self, attrs: &SUMOSAXAttributes, avoidAttributes: Vec<SumoXMLAttr>) {
        // transform avoidAttributes to strings
        let avoidAttributesStr: Vec<&str> = avoidAttributes.iter().map(|a| attr_name(*a)).collect();
        // iterate over attributes and fill parameters map
        for attribute in attrs.get_attribute_names() {
            if !avoidAttributesStr.contains(&attribute.as_str()) {
                let value = attrs.get_string_secure_name(&attribute, "");
                self.myCommonXMLStructure.get_current_sumo_base_object().m().add_parameter(&attribute, &value);
            }
        }
    }

    /// C++: `void DataHandler::checkParent(const SumoXMLTag, const SumoXMLTag, bool &)` [private]
    /// check parents
    // void
    // DataHandler::checkParent(const SumoXMLTag currentTag, const SumoXMLTag parentTag, bool& ok) {
    //     // check that parent SUMOBaseObject's tag is the parentTag
    //     if ((myCommonXMLStructure.getCurrentSumoBaseObject()->getParentSumoBaseObject() &&
    //             (myCommonXMLStructure.getCurrentSumoBaseObject()->getParentSumoBaseObject()->getTag() == parentTag)) == false) {
    //         writeError(toString(currentTag) + " must be defined within the definition of a " + toString(parentTag));
    //         ok = false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn check_parent(&mut self, currentTag: SumoXMLTag, parentTag: SumoXMLTag, ok: &mut bool) {
        // check that parent SUMOBaseObject's tag is the parentTag
        let parent = self.myCommonXMLStructure.get_current_sumo_base_object().get_parent_sumo_base_object();
        if parent.is_null() || parent.get_tag() != parentTag {
            self.write_error(&format!("{} must be defined within the definition of a {}", tag_name(currentTag), tag_name(parentTag)));
            *ok = false;
        }
    }

    /// C++: `void DataHandler::DataHandler()` [private]
    /// invalidate default onstructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2() -> Self {
        panic!("DataHandler() is deleted in C++ (private, invalidated)")
    }

    /// C++: `void DataHandler::DataHandler(const DataHandler &)` [private]
    /// invalidate copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_3(s: &DataHandler) -> Self {
        panic!("DataHandler(const DataHandler&) is deleted in C++ (private, invalidated)")
    }

    /// C++: `DataHandler & DataHandler::operator=(const DataHandler &)` [private]
    /// invalidate assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &DataHandler) -> &mut DataHandler {
        panic!("DataHandler::operator= is deleted in C++ (private, invalidated)")
    }

}

/// C++: `DataHandler` is a `SUMOSAXHandler`, so the XML reader can drive it (XMLSubSys::runParser(*this, ...)).
impl SaxHandler for DataHandler {
    fn handler(&mut self) -> &mut GenericSAXHandler {
        &mut self.base2
    }
    fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) -> Result<(), ProcessError> {
        DataHandler::my_start_element(self, element, attrs);
        Ok(())
    }
    fn my_end_element(&mut self, element: i32) -> Result<(), ProcessError> {
        DataHandler::my_end_element(self, element)
    }
}

