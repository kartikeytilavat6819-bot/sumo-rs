//! Port of utils/handlers/DataHandler.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// The XML-Handler for network loading
/// C++ class `DataHandler` (utils/handlers/DataHandler.h)
pub struct DataHandler {
    /// C++ base class `CommonHandler`
    pub base: CommonHandler,
    /// C++ base class `SUMOSAXHandler`
    pub base2: Unmapped /* SUMOSAXHandler */,
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
    #[allow(unused_variables)]
    pub fn new(fileBucket: Ptr<Unmapped /* FileBucket */>) -> Self {
        todo!("port DataHandler::DataHandler")
    }

    /// C++: `void DataHandler::~DataHandler() noexcept` [virtual]
    /// Destructor
    // DataHandler::~DataHandler() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port DataHandler::~DataHandler")
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
        todo!("port DataHandler::parse")
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
    pub fn parse_sumo_base_object(&mut self, obj: Ptr<Unmapped /* CommonXMLStructure::SumoBaseObject */>) {
        todo!("port DataHandler::parseSumoBaseObject")
    }

    /// C++: `bool DataHandler::buildDataInterval(const CommonXMLStructure::SumoBaseObject *, const std::string &, const double, const double)` [virtual, pure virtual]
    /// Builds DataInterval
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_data_interval(&mut self, sumoBaseObject: Ptr<Unmapped /* CommonXMLStructure::SumoBaseObject */>, dataSetID: &str, begin: f64, end: f64) -> bool {
        todo!("port DataHandler::buildDataInterval")
    }

    /// C++: `bool DataHandler::buildEdgeData(const CommonXMLStructure::SumoBaseObject *, const std::string &, const Parameterised::Map &)` [virtual, pure virtual]
    /// Builds edgeData
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_edge_data(&mut self, sumoBaseObject: Ptr<Unmapped /* CommonXMLStructure::SumoBaseObject */>, edgeID: &str, parameters: &Unmapped /* Parameterised::Map */) -> bool {
        todo!("port DataHandler::buildEdgeData")
    }

    /// C++: `bool DataHandler::buildEdgeRelationData(const CommonXMLStructure::SumoBaseObject *, const std::string &, const std::string &, const Parameterised::Map &)` [virtual, pure virtual]
    /// Builds edgeRelationData
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_edge_relation_data(&mut self, sumoBaseObject: Ptr<Unmapped /* CommonXMLStructure::SumoBaseObject */>, fromEdgeID: &str, toEdgeID: &str, parameters: &Unmapped /* Parameterised::Map */) -> bool {
        todo!("port DataHandler::buildEdgeRelationData")
    }

    /// C++: `bool DataHandler::buildTAZRelationData(const CommonXMLStructure::SumoBaseObject *, const std::string &, const std::string &, const Parameterised::Map &)` [virtual, pure virtual]
    /// Builds TAZRelationData
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_taz_relation_data(&mut self, sumoBaseObject: Ptr<Unmapped /* CommonXMLStructure::SumoBaseObject */>, fromTAZID: &str, toTAZID: &str, parameters: &Unmapped /* Parameterised::Map */) -> bool {
        todo!("port DataHandler::buildTAZRelationData")
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
        todo!("port DataHandler::myStartElement")
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
    pub fn my_end_element(&mut self, element: i32) {
        todo!("port DataHandler::myEndElement")
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
        todo!("port DataHandler::parseInterval")
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
        todo!("port DataHandler::parseEdgeData")
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
        todo!("port DataHandler::parseEdgeRelationData")
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
        todo!("port DataHandler::parseTAZRelationData")
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
        todo!("port DataHandler::getAttributes")
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
        todo!("port DataHandler::checkParent")
    }

    /// C++: `void DataHandler::DataHandler()` [private]
    /// invalidate default onstructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2() -> Self {
        todo!("port DataHandler::DataHandler")
    }

    /// C++: `void DataHandler::DataHandler(const DataHandler &)` [private]
    /// invalidate copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_3(s: &DataHandler) -> Self {
        todo!("port DataHandler::DataHandler")
    }

    /// C++: `DataHandler & DataHandler::operator=(const DataHandler &)` [private]
    /// invalidate assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &DataHandler) -> &mut DataHandler {
        todo!("port DataHandler::operator=")
    }

}

