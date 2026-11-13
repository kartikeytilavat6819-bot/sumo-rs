//! Port of utils/handlers/MeanDataHandler.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use super::common_handler::{attr_str, tag_str};
use utils::xml::common_xml_structure::CommonXMLStructure_SumoBaseObject;
use utils::xml::sumo_xml_definitions::{ExcludeEmptys, MeanDataTypes};

/// unwrap a CommonXMLStructure getter; the C++ getters throw when the attribute is missing
fn or_throw<T>(r: Result<T, utils::common::util_exceptions::ProcessError>) -> T {
    r.unwrap_or_else(|e| panic!("{}", e))
}
fn str_attr(obj: Ptr<CommonXMLStructure_SumoBaseObject>, attr: SumoXMLAttr) -> String {
    or_throw(obj.get_string_attribute(attr)).clone()
}
fn str_list_attr(obj: Ptr<CommonXMLStructure_SumoBaseObject>, attr: SumoXMLAttr) -> Vec<String> {
    or_throw(obj.get_string_list_attribute(attr)).clone()
}
fn double_attr(obj: Ptr<CommonXMLStructure_SumoBaseObject>, attr: SumoXMLAttr) -> f64 {
    or_throw(obj.get_double_attribute(attr))
}
fn bool_attr(obj: Ptr<CommonXMLStructure_SumoBaseObject>, attr: SumoXMLAttr) -> bool {
    or_throw(obj.get_bool_attribute(attr))
}
fn time_attr(obj: Ptr<CommonXMLStructure_SumoBaseObject>, attr: SumoXMLAttr) -> SUMOTime {
    or_throw(obj.get_time_attribute(attr))
}

/// C++ class `MeanDataHandler` (utils/handlers/MeanDataHandler.h)
#[repr(C)]
pub struct MeanDataHandler {
    /// C++ base class `CommonHandler`
    pub base: CommonHandler,
}

impl std::ops::Deref for MeanDataHandler {
    type Target = CommonHandler;
    fn deref(&self) -> &CommonHandler {
        &self.base
    }
}
impl std::ops::DerefMut for MeanDataHandler {
    fn deref_mut(&mut self) -> &mut CommonHandler {
        &mut self.base
    }
}

impl MeanDataHandler {
    /// C++: `void MeanDataHandler::MeanDataHandler(FileBucket *)`
    /// Constructor
    // MeanDataHandler::MeanDataHandler(FileBucket* fileBucket) :
    //     CommonHandler(fileBucket) {
    // }
    #[allow(unused_variables)]
    pub fn new(fileBucket: Ptr<Unmapped /* FileBucket */>) -> Self {
        Self { base: CommonHandler::new(fileBucket) }
    }

    /// C++: `void MeanDataHandler::~MeanDataHandler() noexcept` [virtual]
    /// Destructor
    // MeanDataHandler::~MeanDataHandler() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor has an empty body
    }

    /// C++: `bool MeanDataHandler::beginParseAttributes(SumoXMLTag, const SUMOSAXAttributes &)`
    /// begin parse attributes
    // bool
    // MeanDataHandler::beginParseAttributes(SumoXMLTag tag, const SUMOSAXAttributes& attrs) {
    //     // open SUMOBaseOBject
    //     myCommonXMLStructure.openSUMOBaseOBject();
    //     // check tag
    //     try {
    //         switch (tag) {
    //             case SUMO_TAG_MEANDATA_EDGE:
    //                 parseEdgeMeanData(attrs);
    //                 break;
    //             case SUMO_TAG_MEANDATA_LANE:
    //                 parseLaneMeanData(attrs);
    //                 break;
    //             case SUMO_TAG_PARAM:
    //                 WRITE_WARNING(TL("MeanData elements cannot load attributes as params"));
    //                 break;
    //             default:
    //                 // tag cannot be parsed in MeanDataHandler
    //                 myCommonXMLStructure.abortSUMOBaseOBject();
    //                 return false;
    //         }
    //     } catch (InvalidArgument& e) {
    //         writeError(e.what());
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn begin_parse_attributes(&mut self, tag: SumoXMLTag, attrs: &SUMOSAXAttributes) -> bool {
        // open SUMOBaseOBject
        self.myCommonXMLStructure.open_sumo_base_object();
        // check tag
        // shortcut: C++ try/catch(InvalidArgument) around the switch is not reproduced; attribute errors surface as panics from the getters, upgrade when the getters return Result
        match tag {
            SUMO_TAG_MEANDATA_EDGE => self.parse_edge_mean_data(attrs),
            SUMO_TAG_MEANDATA_LANE => self.parse_lane_mean_data(attrs),
            SUMO_TAG_PARAM => {
                utils::common::msg_handler::write_warning("MeanData elements cannot load attributes as params");
            }
            _ => {
                // tag cannot be parsed in MeanDataHandler
                self.myCommonXMLStructure.abort_sumo_base_object();
                return false;
            }
        }
        true
    }

    /// C++: `void MeanDataHandler::endParseAttributes()`
    /// end parse attributes
    // void
    // MeanDataHandler::endParseAttributes() {
    //     // get last inserted object
    //     CommonXMLStructure::SumoBaseObject* obj = myCommonXMLStructure.getCurrentSumoBaseObject();
    //     // close SUMOBaseOBject
    //     if (obj) {
    //         myCommonXMLStructure.closeSUMOBaseOBject();
    //         // check tag
    //         switch (obj->getTag()) {
    //             case SUMO_TAG_MEANDATA_EDGE:
    //             case SUMO_TAG_MEANDATA_LANE:
    //                 parseSumoBaseObject(obj);
    //                 // delete object
    //                 delete obj;
    //                 break;
    //             default:
    //                 break;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn end_parse_attributes(&mut self) {
        // get last inserted object
        let obj = self.myCommonXMLStructure.get_current_sumo_base_object();
        // close SUMOBaseOBject
        if !obj.is_null() {
            self.myCommonXMLStructure.close_sumo_base_object();
            // check tag
            match obj.get_tag() {
                SUMO_TAG_MEANDATA_EDGE | SUMO_TAG_MEANDATA_LANE => {
                    self.parse_sumo_base_object(obj);
                    // delete object: the parent drops its owned child
                    let parent = obj.get_parent_sumo_base_object();
                    if !parent.is_null() {
                        parent.m().remove_sumo_base_object_child(obj);
                    }
                    // shortcut: root objects (no parent) are not freed here, upgrade with root ownership
                }
                _ => {}
            }
        }
    }

    /// C++: `void MeanDataHandler::parseSumoBaseObject(CommonXMLStructure::SumoBaseObject *)`
    /// parse SumoBaseObject (it's called recursivelly)
    // void
    // MeanDataHandler::parseSumoBaseObject(CommonXMLStructure::SumoBaseObject* obj) {
    //     // check if loading was aborted
    //     if (!myAbortLoading) {
    //         // switch tag
    //         switch (obj->getTag()) {
    //             case SUMO_TAG_MEANDATA_EDGE:
    //                 if (buildEdgeMeanData(obj,
    //                                       obj->getStringAttribute(SUMO_ATTR_ID),
    //                                       obj->getStringAttribute(SUMO_ATTR_FILE),
    //                                       obj->getStringAttribute(SUMO_ATTR_TYPE),
    //                                       obj->getTimeAttribute(SUMO_ATTR_PERIOD),
    //                                       obj->getTimeAttribute(SUMO_ATTR_BEGIN),
    //                                       obj->getTimeAttribute(SUMO_ATTR_END),
    //                                       obj->getBoolAttribute(SUMO_ATTR_TRACK_VEHICLES),
    //                                       obj->getStringListAttribute(SUMO_ATTR_WRITE_ATTRIBUTES),
    //                                       obj->getStringAttribute(SUMO_ATTR_AGGREGATE),
    //                                       obj->getStringListAttribute(SUMO_ATTR_EDGES),
    //                                       obj->getStringAttribute(SUMO_ATTR_EDGESFILE),
    //                                       obj->getStringAttribute(SUMO_ATTR_EXCLUDE_EMPTY),
    //                                       obj->getBoolAttribute(SUMO_ATTR_WITH_INTERNAL),
    //                                       obj->getStringListAttribute(SUMO_ATTR_DETECT_PERSONS),
    //                                       obj->getDoubleAttribute(SUMO_ATTR_MIN_SAMPLES),
    //                                       obj->getDoubleAttribute(SUMO_ATTR_MAX_TRAVELTIME),
    //                                       obj->getStringListAttribute(SUMO_ATTR_VTYPES),
    //                                       obj->getDoubleAttribute(SUMO_ATTR_HALTING_SPEED_THRESHOLD))) {
    //                     obj->markAsCreated();
    //                 };
    //                 break;
    //             case SUMO_TAG_MEANDATA_LANE:
    //                 if (buildLaneMeanData(obj,
    //                                       obj->getStringAttribute(SUMO_ATTR_ID),
    //                                       obj->getStringAttribute(SUMO_ATTR_FILE),
    //                                       obj->getStringAttribute(SUMO_ATTR_TYPE),
    //                                       obj->getTimeAttribute(SUMO_ATTR_PERIOD),
    //                                       obj->getTimeAttribute(SUMO_ATTR_BEGIN),
    //                                       obj->getTimeAttribute(SUMO_ATTR_END),
    //                                       obj->getBoolAttribute(SUMO_ATTR_TRACK_VEHICLES),
    //                                       obj->getStringListAttribute(SUMO_ATTR_WRITE_ATTRIBUTES),
    //                                       obj->getStringAttribute(SUMO_ATTR_AGGREGATE),
    //                                       obj->getStringListAttribute(SUMO_ATTR_EDGES),
    //                                       obj->getStringAttribute(SUMO_ATTR_EDGESFILE),
    //                                       obj->getStringAttribute(SUMO_ATTR_EXCLUDE_EMPTY),
    //                                       obj->getBoolAttribute(SUMO_ATTR_WITH_INTERNAL),
    //                                       obj->getStringListAttribute(SUMO_ATTR_DETECT_PERSONS),
    //                                       obj->getDoubleAttribute(SUMO_ATTR_MIN_SAMPLES),
    //                                       obj->getDoubleAttribute(SUMO_ATTR_MAX_TRAVELTIME),
    //                                       obj->getStringListAttribute(SUMO_ATTR_VTYPES),
    //                                       obj->getDoubleAttribute(SUMO_ATTR_HALTING_SPEED_THRESHOLD))) {
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
    pub fn parse_sumo_base_object(&mut self, obj: Ptr<CommonXMLStructure_SumoBaseObject>) {
        // check if loading was aborted
        if (!self.myAbortLoading) {
            // switch tag
            match obj.get_tag() {
                SUMO_TAG_MEANDATA_EDGE => {
                    if (self.build_edge_mean_data(obj, &str_attr(obj, SUMO_ATTR_ID), &str_attr(obj, SUMO_ATTR_FILE), &str_attr(obj, SUMO_ATTR_TYPE), time_attr(obj, SUMO_ATTR_PERIOD), time_attr(obj, SUMO_ATTR_BEGIN), time_attr(obj, SUMO_ATTR_END), bool_attr(obj, SUMO_ATTR_TRACK_VEHICLES), &str_list_attr(obj, SUMO_ATTR_WRITE_ATTRIBUTES), &str_attr(obj, SUMO_ATTR_AGGREGATE), &str_list_attr(obj, SUMO_ATTR_EDGES), &str_attr(obj, SUMO_ATTR_EDGESFILE), &str_attr(obj, SUMO_ATTR_EXCLUDE_EMPTY), bool_attr(obj, SUMO_ATTR_WITH_INTERNAL), &str_list_attr(obj, SUMO_ATTR_DETECT_PERSONS), double_attr(obj, SUMO_ATTR_MIN_SAMPLES), double_attr(obj, SUMO_ATTR_MAX_TRAVELTIME), &str_list_attr(obj, SUMO_ATTR_VTYPES), double_attr(obj, SUMO_ATTR_HALTING_SPEED_THRESHOLD))) {
                        obj.m().mark_as_created();
                    };
                }
                SUMO_TAG_MEANDATA_LANE => {
                    if (self.build_lane_mean_data(obj, &str_attr(obj, SUMO_ATTR_ID), &str_attr(obj, SUMO_ATTR_FILE), &str_attr(obj, SUMO_ATTR_TYPE), time_attr(obj, SUMO_ATTR_PERIOD), time_attr(obj, SUMO_ATTR_BEGIN), time_attr(obj, SUMO_ATTR_END), bool_attr(obj, SUMO_ATTR_TRACK_VEHICLES), &str_list_attr(obj, SUMO_ATTR_WRITE_ATTRIBUTES), &str_attr(obj, SUMO_ATTR_AGGREGATE), &str_list_attr(obj, SUMO_ATTR_EDGES), &str_attr(obj, SUMO_ATTR_EDGESFILE), &str_attr(obj, SUMO_ATTR_EXCLUDE_EMPTY), bool_attr(obj, SUMO_ATTR_WITH_INTERNAL), &str_list_attr(obj, SUMO_ATTR_DETECT_PERSONS), double_attr(obj, SUMO_ATTR_MIN_SAMPLES), double_attr(obj, SUMO_ATTR_MAX_TRAVELTIME), &str_list_attr(obj, SUMO_ATTR_VTYPES), double_attr(obj, SUMO_ATTR_HALTING_SPEED_THRESHOLD))) {
                        obj.m().mark_as_created();
                    }
                }
                _ => {}
            }
            // now iterate over childrens
            for child in obj.get_sumo_base_object_children().iter() {
                // call this function recursively
                self.parse_sumo_base_object(Ptr::from_box(child));
            }
        }
    }

    /// C++: `bool MeanDataHandler::buildEdgeMeanData(const CommonXMLStructure::SumoBaseObject *, const std::string &, const std::string &, const std::string &, const SUMOTime, const SUMOTime, const SUMOTime, const bool, const std::vector<std::string> &, const std::string &, const std::vector<std::string> &, const std::string &, const std::string &, const bool, const std::vector<std::string> &, const double, const double, const std::vector<std::string> &, const double)` [virtual, pure virtual]
    /// Builds edgeMeanData
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_edge_mean_data(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, ID: &str, file: &str, type_: &str, period: SUMOTime, begin: SUMOTime, end: SUMOTime, trackVehicles: bool, writtenAttributes: &Vec<String>, aggregate: &str, edges: &Vec<String>, edgeFile: &str, excludeEmpty: &str, withInternal: bool, detectPersons: &Vec<String>, minSamples: f64, maxTravelTime: f64, vTypes: &Vec<String>, speedThreshold: f64) -> bool {
        panic!("MeanDataHandler::buildEdgeMeanData is pure virtual in C++")
    }

    /// C++: `bool MeanDataHandler::buildLaneMeanData(const CommonXMLStructure::SumoBaseObject *, const std::string &, const std::string &, const std::string &, const SUMOTime, const SUMOTime, const SUMOTime, const bool, const std::vector<std::string> &, const std::string &, const std::vector<std::string> &, const std::string &, const std::string &, const bool, const std::vector<std::string> &, const double, const double, const std::vector<std::string> &, const double)` [virtual, pure virtual]
    /// Builds laneMeanData
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_lane_mean_data(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, ID: &str, file: &str, type_: &str, period: SUMOTime, begin: SUMOTime, end: SUMOTime, trackVehicles: bool, writtenAttributes: &Vec<String>, aggregate: &str, edges: &Vec<String>, edgeFile: &str, excludeEmpty: &str, withInternal: bool, detectPersons: &Vec<String>, minSamples: f64, maxTravelTime: f64, vTypes: &Vec<String>, speedThreshold: f64) -> bool {
        panic!("MeanDataHandler::buildLaneMeanData is pure virtual in C++")
    }

    /// C++: `void MeanDataHandler::parseEdgeMeanData(const SUMOSAXAttributes &)` [private]
    /// parse edgeMeanData attributes
    // void
    // MeanDataHandler::parseEdgeMeanData(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // needed attributes
    //     const std::string id = attrs.get<std::string>(SUMO_ATTR_ID, "", parsedOk);
    //     const std::string file = attrs.get<std::string>(SUMO_ATTR_FILE, id.c_str(), parsedOk);
    //     // optional attributes
    //     std::string type = attrs.getOpt<std::string>(SUMO_ATTR_TYPE, id.c_str(), parsedOk, "");
    //     const SUMOTime period = attrs.getOptSUMOTimeReporting(SUMO_ATTR_PERIOD, id.c_str(), parsedOk, TIME2STEPS(-1));
    //     const SUMOTime begin = attrs.getOptSUMOTimeReporting(SUMO_ATTR_BEGIN, id.c_str(), parsedOk, TIME2STEPS(-1));
    //     const SUMOTime end = attrs.getOptSUMOTimeReporting(SUMO_ATTR_END, id.c_str(), parsedOk, TIME2STEPS(-1));
    //     const bool trackVehicles = attrs.getOpt<bool>(SUMO_ATTR_TRACK_VEHICLES, id.c_str(), parsedOk, false);
    //     const std::vector<std::string> writeAttributes = attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_WRITE_ATTRIBUTES, id.c_str(), parsedOk, {});
    //     const std::string aggregate = attrs.getOpt<std::string>(SUMO_ATTR_AGGREGATE, id.c_str(), parsedOk, "false");
    //     const std::vector<std::string> edges = attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_EDGES, id.c_str(), parsedOk, {});
    //     const std::string edgeFile = attrs.getOpt<std::string>(SUMO_ATTR_EDGESFILE, id.c_str(), parsedOk, "");
    //     const std::string excludeEmpty = attrs.getOpt<std::string>(SUMO_ATTR_EXCLUDE_EMPTY, id.c_str(), parsedOk, SUMOXMLDefinitions::ExcludeEmptys.getString(ExcludeEmpty::FALSES));
    //     const bool withInternal = attrs.getOpt<bool>(SUMO_ATTR_WITH_INTERNAL, id.c_str(), parsedOk, false);
    //     const std::vector<std::string> detectPersons = attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_DETECT_PERSONS, id.c_str(), parsedOk, {});
    //     const double minSamples = attrs.getOpt<double>(SUMO_ATTR_MIN_SAMPLES, id.c_str(), parsedOk, 0);
    //     const double maxTravel = attrs.getOpt<double>(SUMO_ATTR_MAX_TRAVELTIME, id.c_str(), parsedOk, 100000);
    //     const std::vector<std::string> vTypes = attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_VTYPES, id.c_str(), parsedOk, {});
    //     const double speedThreshold = attrs.getOpt<double>(SUMO_ATTR_HALTING_SPEED_THRESHOLD, id.c_str(), parsedOk, 0.1);
    //     // adjust type
    //     if (type == "performance") {
    //         type = "";
    //     } else if (type == "hbefa") {
    //         WRITE_WARNING(TL("The netstate type 'hbefa' is deprecated. Using 'emissions' instead."));
    //         type = "emissions";
    //     }
    //     // continue if flag is ok
    //     if (parsedOk && checkType(SUMO_TAG_MEANDATA_EDGE, id, type)) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_MEANDATA_EDGE);
    //         // add all attributes
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_ID, id);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_FILE, file);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_TYPE, type);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addTimeAttribute(SUMO_ATTR_PERIOD, period);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addTimeAttribute(SUMO_ATTR_BEGIN, begin);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addTimeAttribute(SUMO_ATTR_END, end);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addBoolAttribute(SUMO_ATTR_TRACK_VEHICLES, trackVehicles);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringListAttribute(SUMO_ATTR_WRITE_ATTRIBUTES, writeAttributes);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_AGGREGATE, aggregate);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringListAttribute(SUMO_ATTR_EDGES, edges);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_EDGESFILE, edgeFile);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_EXCLUDE_EMPTY, excludeEmpty);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addBoolAttribute(SUMO_ATTR_WITH_INTERNAL, withInternal);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringListAttribute(SUMO_ATTR_DETECT_PERSONS, detectPersons);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_MIN_SAMPLES, minSamples);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_MAX_TRAVELTIME, maxTravel);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringListAttribute(SUMO_ATTR_VTYPES, vTypes);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_HALTING_SPEED_THRESHOLD, speedThreshold);
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_edge_mean_data(&mut self, attrs: &SUMOSAXAttributes) {
        // declare Ok Flag
        let mut parsedOk: bool = true;
        // needed attributes
        let id: String = attrs.get::<String>(SUMO_ATTR_ID, "", &mut parsedOk, true);
        let file: String = attrs.get::<String>(SUMO_ATTR_FILE, &id, &mut parsedOk, true);
        // optional attributes
        let mut type_: String = attrs.get_opt::<String>(SUMO_ATTR_TYPE, &id, &mut parsedOk, String::new(), true);
        let period: SUMOTime = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_PERIOD, &id, &mut parsedOk, time2steps(-1.0), true);
        let begin: SUMOTime = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_BEGIN, &id, &mut parsedOk, time2steps(-1.0), true);
        let end: SUMOTime = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_END, &id, &mut parsedOk, time2steps(-1.0), true);
        let trackVehicles: bool = attrs.get_opt::<bool>(SUMO_ATTR_TRACK_VEHICLES, &id, &mut parsedOk, false, true);
        let writeAttributes: Vec<String> = attrs.get_opt::<Vec<String>>(SUMO_ATTR_WRITE_ATTRIBUTES, &id, &mut parsedOk, Vec::new(), true);
        let aggregate: String = attrs.get_opt::<String>(SUMO_ATTR_AGGREGATE, &id, &mut parsedOk, "false".to_string(), true);
        let edges: Vec<String> = attrs.get_opt::<Vec<String>>(SUMO_ATTR_EDGES, &id, &mut parsedOk, Vec::new(), true);
        let edgeFile: String = attrs.get_opt::<String>(SUMO_ATTR_EDGESFILE, &id, &mut parsedOk, String::new(), true);
        let excludeEmpty: String = attrs.get_opt::<String>(SUMO_ATTR_EXCLUDE_EMPTY, &id, &mut parsedOk, ExcludeEmptys.get_string(ExcludeEmpty::FALSES).to_string(), true);
        let withInternal: bool = attrs.get_opt::<bool>(SUMO_ATTR_WITH_INTERNAL, &id, &mut parsedOk, false, true);
        let detectPersons: Vec<String> = attrs.get_opt::<Vec<String>>(SUMO_ATTR_DETECT_PERSONS, &id, &mut parsedOk, Vec::new(), true);
        let minSamples: f64 = attrs.get_opt::<f64>(SUMO_ATTR_MIN_SAMPLES, &id, &mut parsedOk, 0.0, true);
        let maxTravel: f64 = attrs.get_opt::<f64>(SUMO_ATTR_MAX_TRAVELTIME, &id, &mut parsedOk, 100000.0, true);
        let vTypes: Vec<String> = attrs.get_opt::<Vec<String>>(SUMO_ATTR_VTYPES, &id, &mut parsedOk, Vec::new(), true);
        let speedThreshold: f64 = attrs.get_opt::<f64>(SUMO_ATTR_HALTING_SPEED_THRESHOLD, &id, &mut parsedOk, 0.1, true);
        // adjust type
        if type_ == "performance" {
            type_ = String::new();
        } else if type_ == "hbefa" {
            utils::common::msg_handler::write_warning("The netstate type 'hbefa' is deprecated. Using 'emissions' instead.");
            type_ = "emissions".to_string();
        }
        // continue if flag is ok
        if parsedOk && self.check_type(SUMO_TAG_MEANDATA_EDGE, &id, &type_) {
            // set tag
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_MEANDATA_EDGE);
            // add all attributes
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_ID, &id);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_FILE, &file);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_TYPE, &type_);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_time_attribute(SUMO_ATTR_PERIOD, period);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_time_attribute(SUMO_ATTR_BEGIN, begin);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_time_attribute(SUMO_ATTR_END, end);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_bool_attribute(SUMO_ATTR_TRACK_VEHICLES, trackVehicles);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_list_attribute(SUMO_ATTR_WRITE_ATTRIBUTES, &writeAttributes);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_AGGREGATE, &aggregate);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_list_attribute(SUMO_ATTR_EDGES, &edges);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_EDGESFILE, &edgeFile);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_EXCLUDE_EMPTY, &excludeEmpty);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_bool_attribute(SUMO_ATTR_WITH_INTERNAL, withInternal);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_list_attribute(SUMO_ATTR_DETECT_PERSONS, &detectPersons);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_MIN_SAMPLES, minSamples);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_MAX_TRAVELTIME, maxTravel);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_list_attribute(SUMO_ATTR_VTYPES, &vTypes);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_HALTING_SPEED_THRESHOLD, speedThreshold);
        } else {
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_ERROR);
        }
    }

    /// C++: `void MeanDataHandler::parseLaneMeanData(const SUMOSAXAttributes &)` [private]
    /// parse laneMeanData attributes
    // void
    // MeanDataHandler::parseLaneMeanData(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // needed attributes
    //     const std::string id = attrs.get<std::string>(SUMO_ATTR_ID, "", parsedOk);
    //     const std::string file = attrs.get<std::string>(SUMO_ATTR_FILE, id.c_str(), parsedOk);
    //     // optional attributes
    //     const std::string type = attrs.getOpt<std::string>(SUMO_ATTR_TYPE, id.c_str(), parsedOk, "");
    //     const SUMOTime period = attrs.getOptSUMOTimeReporting(SUMO_ATTR_PERIOD, id.c_str(), parsedOk, TIME2STEPS(-1));
    //     const SUMOTime begin = attrs.getOptSUMOTimeReporting(SUMO_ATTR_BEGIN, id.c_str(), parsedOk, TIME2STEPS(-1));
    //     const SUMOTime end = attrs.getOptSUMOTimeReporting(SUMO_ATTR_END, id.c_str(), parsedOk, TIME2STEPS(-1));
    //     const bool trackVehicles = attrs.getOpt<bool>(SUMO_ATTR_TRACK_VEHICLES, id.c_str(), parsedOk, false);
    //     const std::vector<std::string> writeAttributes = attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_WRITE_ATTRIBUTES, id.c_str(), parsedOk, {});
    //     const std::string aggregate = attrs.getOpt<std::string>(SUMO_ATTR_AGGREGATE, id.c_str(), parsedOk, "false");
    //     const std::vector<std::string> edges = attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_EDGES, id.c_str(), parsedOk, {});
    //     const std::string edgeFile = attrs.getOpt<std::string>(SUMO_ATTR_EDGESFILE, id.c_str(), parsedOk, "");
    //     const std::string excludeEmpty = attrs.getOpt<std::string>(SUMO_ATTR_EXCLUDE_EMPTY, id.c_str(), parsedOk, SUMOXMLDefinitions::ExcludeEmptys.getString(ExcludeEmpty::FALSES));
    //     const bool withInternal = attrs.getOpt<bool>(SUMO_ATTR_WITH_INTERNAL, id.c_str(), parsedOk, false);
    //     const std::vector<std::string> detectPersons = attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_DETECT_PERSONS, id.c_str(), parsedOk, {});
    //     const double minSamples = attrs.getOpt<double>(SUMO_ATTR_MIN_SAMPLES, id.c_str(), parsedOk, 0);
    //     const double maxTravel = attrs.getOpt<double>(SUMO_ATTR_MAX_TRAVELTIME, id.c_str(), parsedOk, 100000);
    //     const std::vector<std::string> vTypes = attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_VTYPES, id.c_str(), parsedOk, {});
    //     const double speedThreshold = attrs.getOpt<double>(SUMO_ATTR_HALTING_SPEED_THRESHOLD, id.c_str(), parsedOk, 0.1);
    //     // continue if flag is ok
    //     if (parsedOk && checkType(SUMO_TAG_MEANDATA_LANE, id, type)) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_MEANDATA_LANE);
    //         // add all attributes
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_ID, id);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_FILE, file);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_TYPE, type);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addTimeAttribute(SUMO_ATTR_PERIOD, period);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addTimeAttribute(SUMO_ATTR_BEGIN, begin);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addTimeAttribute(SUMO_ATTR_END, end);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addBoolAttribute(SUMO_ATTR_TRACK_VEHICLES, trackVehicles);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringListAttribute(SUMO_ATTR_WRITE_ATTRIBUTES, writeAttributes);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_AGGREGATE, aggregate);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringListAttribute(SUMO_ATTR_EDGES, edges);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_EDGESFILE, edgeFile);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_EXCLUDE_EMPTY, excludeEmpty);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addBoolAttribute(SUMO_ATTR_WITH_INTERNAL, withInternal);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringListAttribute(SUMO_ATTR_DETECT_PERSONS, detectPersons);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_MIN_SAMPLES, minSamples);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_MAX_TRAVELTIME, maxTravel);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringListAttribute(SUMO_ATTR_VTYPES, vTypes);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_HALTING_SPEED_THRESHOLD, speedThreshold);
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_lane_mean_data(&mut self, attrs: &SUMOSAXAttributes) {
        // declare Ok Flag
        let mut parsedOk: bool = true;
        // needed attributes
        let id: String = attrs.get::<String>(SUMO_ATTR_ID, "", &mut parsedOk, true);
        let file: String = attrs.get::<String>(SUMO_ATTR_FILE, &id, &mut parsedOk, true);
        // optional attributes
        let type_: String = attrs.get_opt::<String>(SUMO_ATTR_TYPE, &id, &mut parsedOk, String::new(), true);
        let period: SUMOTime = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_PERIOD, &id, &mut parsedOk, time2steps(-1.0), true);
        let begin: SUMOTime = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_BEGIN, &id, &mut parsedOk, time2steps(-1.0), true);
        let end: SUMOTime = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_END, &id, &mut parsedOk, time2steps(-1.0), true);
        let trackVehicles: bool = attrs.get_opt::<bool>(SUMO_ATTR_TRACK_VEHICLES, &id, &mut parsedOk, false, true);
        let writeAttributes: Vec<String> = attrs.get_opt::<Vec<String>>(SUMO_ATTR_WRITE_ATTRIBUTES, &id, &mut parsedOk, Vec::new(), true);
        let aggregate: String = attrs.get_opt::<String>(SUMO_ATTR_AGGREGATE, &id, &mut parsedOk, "false".to_string(), true);
        let edges: Vec<String> = attrs.get_opt::<Vec<String>>(SUMO_ATTR_EDGES, &id, &mut parsedOk, Vec::new(), true);
        let edgeFile: String = attrs.get_opt::<String>(SUMO_ATTR_EDGESFILE, &id, &mut parsedOk, String::new(), true);
        let excludeEmpty: String = attrs.get_opt::<String>(SUMO_ATTR_EXCLUDE_EMPTY, &id, &mut parsedOk, ExcludeEmptys.get_string(ExcludeEmpty::FALSES).to_string(), true);
        let withInternal: bool = attrs.get_opt::<bool>(SUMO_ATTR_WITH_INTERNAL, &id, &mut parsedOk, false, true);
        let detectPersons: Vec<String> = attrs.get_opt::<Vec<String>>(SUMO_ATTR_DETECT_PERSONS, &id, &mut parsedOk, Vec::new(), true);
        let minSamples: f64 = attrs.get_opt::<f64>(SUMO_ATTR_MIN_SAMPLES, &id, &mut parsedOk, 0.0, true);
        let maxTravel: f64 = attrs.get_opt::<f64>(SUMO_ATTR_MAX_TRAVELTIME, &id, &mut parsedOk, 100000.0, true);
        let vTypes: Vec<String> = attrs.get_opt::<Vec<String>>(SUMO_ATTR_VTYPES, &id, &mut parsedOk, Vec::new(), true);
        let speedThreshold: f64 = attrs.get_opt::<f64>(SUMO_ATTR_HALTING_SPEED_THRESHOLD, &id, &mut parsedOk, 0.1, true);
        // continue if flag is ok
        if parsedOk && self.check_type(SUMO_TAG_MEANDATA_LANE, &id, &type_) {
            // set tag
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_MEANDATA_LANE);
            // add all attributes
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_ID, &id);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_FILE, &file);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_TYPE, &type_);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_time_attribute(SUMO_ATTR_PERIOD, period);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_time_attribute(SUMO_ATTR_BEGIN, begin);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_time_attribute(SUMO_ATTR_END, end);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_bool_attribute(SUMO_ATTR_TRACK_VEHICLES, trackVehicles);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_list_attribute(SUMO_ATTR_WRITE_ATTRIBUTES, &writeAttributes);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_AGGREGATE, &aggregate);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_list_attribute(SUMO_ATTR_EDGES, &edges);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_EDGESFILE, &edgeFile);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_attribute(SUMO_ATTR_EXCLUDE_EMPTY, &excludeEmpty);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_bool_attribute(SUMO_ATTR_WITH_INTERNAL, withInternal);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_list_attribute(SUMO_ATTR_DETECT_PERSONS, &detectPersons);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_MIN_SAMPLES, minSamples);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_MAX_TRAVELTIME, maxTravel);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_string_list_attribute(SUMO_ATTR_VTYPES, &vTypes);
            self.myCommonXMLStructure.get_current_sumo_base_object().m().add_double_attribute(SUMO_ATTR_HALTING_SPEED_THRESHOLD, speedThreshold);
        } else {
            self.myCommonXMLStructure.get_current_sumo_base_object().m().set_tag(SUMO_TAG_ERROR);
        }
    }

    /// C++: `bool MeanDataHandler::checkType(const SumoXMLTag, const std::string &, const std::string &)` [private]
    /// check mean data type
    // bool
    // MeanDataHandler::checkType(const SumoXMLTag currentTag, const std::string& id, const std::string& type) {
    //     if (SUMOXMLDefinitions::MeanDataTypes.hasString(type)) {
    //         return true;
    //     } else {
    //         writeError(TLF("Attribute '%' defined in % with id '%' doesn't have a valid value (given '%').", toString(SUMO_ATTR_TYPE), toString(currentTag), id, type));
    //         return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn check_type(&mut self, currentTag: SumoXMLTag, id: &str, type_: &str) -> bool {
        if MeanDataTypes.has_string(type_) {
            return true;
        } else {
            self.write_error(&format!("Attribute '{}' defined in {} with id '{}' doesn't have a valid value (given '{}').", attr_str(SUMO_ATTR_TYPE), tag_str(currentTag), id, type_));
            return false;
        }
    }

    /// C++: `void MeanDataHandler::MeanDataHandler()` [private]
    /// invalidate default onstructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2() -> Self {
        panic!("MeanDataHandler() is private and never defined in C++")
    }

    /// C++: `void MeanDataHandler::MeanDataHandler(const MeanDataHandler &)` [private]
    /// invalidate copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_3(s: &MeanDataHandler) -> Self {
        panic!("MeanDataHandler(const MeanDataHandler&) is private and never defined in C++")
    }

    /// C++: `MeanDataHandler & MeanDataHandler::operator=(const MeanDataHandler &)` [private]
    /// invalidate assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &MeanDataHandler) -> &mut MeanDataHandler {
        panic!("MeanDataHandler::operator= is private and never defined in C++")
    }

}

