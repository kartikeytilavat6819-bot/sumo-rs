//! Port of utils/handlers/CommonHandler.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Every method is now ported from the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::xml::common_xml_structure::{CommonXMLStructure, CommonXMLStructure_SumoBaseObject};
use utils::xml::sumo_xml_definitions::{
    SUMO_ATTR_FROM, SUMO_ATTR_FROM_JUNCTION, SUMO_ATTR_FROM_TAZ, SUMO_ATTR_ID, SUMO_ATTR_KEY, SUMO_ATTR_ROUTE, SUMO_ATTR_TO,
    SUMO_ATTR_TO_JUNCTION, SUMO_ATTR_TO_TAZ, SUMO_ATTR_VALUE, SUMO_TAG_CONTAINER, SUMO_TAG_CONTAINERFLOW, SUMO_TAG_ERROR, SUMO_TAG_FLOW,
    SUMO_TAG_PARAM, SUMO_TAG_PERSON, SUMO_TAG_PERSONFLOW, SUMO_TAG_ROOTFILE, SUMO_TAG_ROUTE, SUMO_TAG_ROUTE_DISTRIBUTION, SUMO_TAG_TRIP,
    SUMO_TAG_VEHICLE, SUMO_TAG_VTYPE_DISTRIBUTION,
};

/// C++ `toString(SumoXMLTag)`: reverse lookup in the generated tag table.
// shortcut: linear reverse lookup of the generated TAGS table, C++ uses a map; upgrade by generating a reverse map
pub(crate) fn tag_str(tag: SumoXMLTag) -> String {
    utils::xml::sumo_xml_definitions::TAGS.iter().find(|(_, t)| *t == tag).map(|(n, _)| n.to_string()).unwrap_or_default()
}

/// C++ `toString(SumoXMLAttr)`: reverse lookup in the generated attribute table.
// shortcut: linear reverse lookup of the generated ATTRS table, C++ uses a map; upgrade by generating a reverse map
pub(crate) fn attr_str(attr: SumoXMLAttr) -> String {
    utils::xml::sumo_xml_definitions::ATTRS.iter().find(|(_, a)| *a == attr).map(|(n, _)| n.to_string()).unwrap_or_default()
}

/// C++ class `CommonHandler` (utils/handlers/CommonHandler.h)
pub struct CommonHandler {
    /// fileBucket
    pub myFileBucket: Ptr<Unmapped /* FileBucket */>,
    /// common XML Structure
    pub myCommonXMLStructure: CommonXMLStructure,
    /// flag for mark if a element wasn't created
    pub myErrorCreatingElement: bool,
    /// overwrite elements
    pub myOverwriteElements: bool,
    /// remain elements
    pub myRemainElements: bool,
    /// abort loading
    pub myAbortLoading: bool,
}

impl CommonHandler {
    /// C++: `void CommonHandler::CommonHandler(FileBucket *)`
    /// Constructor
    // CommonHandler::CommonHandler(FileBucket* fileBucket) :
    //     myFileBucket(fileBucket) {
    // }
    #[allow(unused_variables)]
    pub fn new(fileBucket: Ptr<Unmapped /* FileBucket */>) -> Self {
        Self { myFileBucket: fileBucket, myCommonXMLStructure: CommonXMLStructure::default(), myErrorCreatingElement: false, myOverwriteElements: false, myRemainElements: false, myAbortLoading: false }
    }

    /// C++: `void CommonHandler::~CommonHandler() noexcept` [virtual]
    /// Destructor
    // CommonHandler::~CommonHandler() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor has an empty body
    }

    /// C++: `void CommonHandler::forceOverwriteElements()`
    /// force overwritte elements (used if we're reloading elements)
    // void
    // CommonHandler::forceOverwriteElements() {
    //     myOverwriteElements = true;
    // }
    #[allow(unused_variables)]
    pub fn force_overwrite_elements(&mut self) {
        self.myOverwriteElements = true;
    }

    /// C++: `void CommonHandler::forceRemainElements()`
    /// force remain elements (used if we're reloading elements)
    // void
    // CommonHandler::forceRemainElements() {
    //     myRemainElements = true;
    // }
    #[allow(unused_variables)]
    pub fn force_remain_elements(&mut self) {
        self.myRemainElements = true;
    }

    /// C++: `void CommonHandler::abortLoading()`
    /// abort loading
    // void
    // CommonHandler::abortLoading() {
    //     myAbortLoading = true;
    // }
    #[allow(unused_variables)]
    pub fn abort_loading(&mut self) {
        self.myAbortLoading = true;
    }

    /// C++: `bool CommonHandler::isErrorCreatingElement() const` [const]
    /// get flag for mark if a element wasn't created
    // bool
    // CommonHandler::isErrorCreatingElement() const {
    //     return myErrorCreatingElement;
    // }
    #[allow(unused_variables)]
    pub fn is_error_creating_element(&self) -> bool {
        self.myErrorCreatingElement
    }

    /// C++: `bool CommonHandler::isForceOverwriteElements() const` [const]
    /// force overwritte elements (used if we're reloading elements)
    // bool
    // CommonHandler::isForceOverwriteElements() const {
    //     return myOverwriteElements;
    // }
    #[allow(unused_variables)]
    pub fn is_force_overwrite_elements(&self) -> bool {
        self.myOverwriteElements
    }

    /// C++: `bool CommonHandler::isForceRemainElements() const` [const]
    /// force remain elements (used if we're reloading elements)
    // bool
    // CommonHandler::isForceRemainElements() const {
    //     return myRemainElements;
    // }
    #[allow(unused_variables)]
    pub fn is_force_remain_elements(&self) -> bool {
        self.myRemainElements
    }

    /// C++: `bool CommonHandler::isAbortLoading() const` [const]
    /// abort loading
    // bool
    // CommonHandler::isAbortLoading() const {
    //     return myAbortLoading;
    // }
    #[allow(unused_variables)]
    pub fn is_abort_loading(&self) -> bool {
        self.myAbortLoading
    }

    /// C++: `void CommonHandler::parseParameters(const SUMOSAXAttributes &)` [protected]
    /// parse generic parameters
    // void
    // CommonHandler::parseParameters(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // get key
    //     const std::string key = attrs.get<std::string>(SUMO_ATTR_KEY, nullptr, parsedOk);
    //     // get SumoBaseObject parent
    //     CommonXMLStructure::SumoBaseObject* SumoBaseObjectParent = myCommonXMLStructure.getCurrentSumoBaseObject()->getParentSumoBaseObject();
    //     // check parent
    //     if ((SumoBaseObjectParent == nullptr) || (SumoBaseObjectParent->getTag() == SUMO_TAG_ROOTFILE)) {
    //         writeError(TL("Parameters must be defined within an object"));
    //     } else if (SumoBaseObjectParent->getTag() == SUMO_TAG_PARAM) {
    //         writeError(TL("Parameters cannot be defined within another parameter."));
    //     } else if ((SumoBaseObjectParent->getTag() != SUMO_TAG_ERROR) && parsedOk) {
    //         // get tag str
    //         const std::string parentTagStr = toString(SumoBaseObjectParent->getTag());
    //         // circumventing empty string value
    //         const std::string value = attrs.hasAttribute(SUMO_ATTR_VALUE) ? attrs.getString(SUMO_ATTR_VALUE) : "";
    //         // show warnings if values are invalid
    //         if (key.empty()) {
    //             writeError(TLF("Error parsing key from % generic parameter. Key cannot be empty", parentTagStr));
    //         } else if (!SUMOXMLDefinitions::isValidParameterKey(key)) {
    //             writeError(TLF("Error parsing key from % generic parameter. Key contains invalid characters", parentTagStr));
    //         } else {
    //             // insert parameter in SumoBaseObjectParent
    //             SumoBaseObjectParent->addParameter(key, value);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_parameters(&mut self, attrs: &SUMOSAXAttributes) {
        // declare Ok Flag
        let mut parsedOk = true;
        // get key
        let key: String = attrs.get::<String>(SUMO_ATTR_KEY, "", &mut parsedOk, true);
        // get SumoBaseObject parent
        let SumoBaseObjectParent = self.myCommonXMLStructure.get_current_sumo_base_object().get_parent_sumo_base_object();
        // check parent
        if SumoBaseObjectParent.is_null() || SumoBaseObjectParent.get_tag() == SUMO_TAG_ROOTFILE {
            self.write_error("Parameters must be defined within an object");
        } else if SumoBaseObjectParent.get_tag() == SUMO_TAG_PARAM {
            self.write_error("Parameters cannot be defined within another parameter.");
        } else if (SumoBaseObjectParent.get_tag() != SUMO_TAG_ERROR) && parsedOk {
            // get tag str
            let parentTagStr = tag_str(SumoBaseObjectParent.get_tag());
            // circumventing empty string value
            let value: String = if attrs.has_attribute(SUMO_ATTR_VALUE) { attrs.get_string(SUMO_ATTR_VALUE) } else { String::new() };
            // show warnings if values are invalid
            if key.is_empty() {
                self.write_error(&format!("Error parsing key from {} generic parameter. Key cannot be empty", parentTagStr));
            } else if !utils::xml::sumo_xml_definitions::is_valid_parameter_key(&key) {
                self.write_error(&format!("Error parsing key from {} generic parameter. Key contains invalid characters", parentTagStr));
            } else {
                // insert parameter in SumoBaseObjectParent
                SumoBaseObjectParent.m().add_parameter(&key, &value);
            }
        }
    }

    /// C++: `CommonXMLStructure::SumoBaseObject * CommonHandler::getEmbeddedRoute(const CommonXMLStructure::SumoBaseObject *) const` [const, protected]
    /// get embedded route from children
    // CommonXMLStructure::SumoBaseObject*
    // CommonHandler::getEmbeddedRoute(const CommonXMLStructure::SumoBaseObject* sumoBaseObject) const {
    //     // locate route in childrens
    //     for (const auto& embeddedRoute : sumoBaseObject->getSumoBaseObjectChildren()) {
    //         if ((embeddedRoute->getTag() == SUMO_TAG_ROUTE) && (!embeddedRoute->hasStringAttribute(SUMO_ATTR_ID))) {
    //             return embeddedRoute;
    //         }
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_embedded_route(&self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>) -> Ptr<CommonXMLStructure_SumoBaseObject> {
        // locate route in childrens
        for embeddedRoute in sumoBaseObject.get_sumo_base_object_children().iter() {
            if (embeddedRoute.get_tag() == SUMO_TAG_ROUTE) && (!embeddedRoute.has_string_attribute(SUMO_ATTR_ID)) {
                return Ptr::from_box(embeddedRoute).cast();
            }
        }
        Ptr::null()
    }

    /// C++: `void CommonHandler::checkParsedParent(const SumoXMLTag, const std::vector<SumoXMLTag> &, bool &)` [protected]
    /// check parsed parents
    // void
    // CommonHandler::checkParsedParent(const SumoXMLTag currentTag, const std::vector<SumoXMLTag>& parentTags, bool& ok) {
    //     if (parentTags.size() > 0) {
    //         std::string tagsStr;
    //         for (auto it = parentTags.begin(); it != parentTags.end(); it++) {
    //             tagsStr.append(toString(*it));
    //             if ((it + 1) != parentTags.end()) {
    //                 if ((it + 2) != parentTags.end()) {
    //                     tagsStr.append(", ");
    //                 } else {
    //                     tagsStr.append(" or ");
    //                 }
    //             }
    //         }
    //         // obtain parent
    //         CommonXMLStructure::SumoBaseObject* const parent = myCommonXMLStructure.getCurrentSumoBaseObject()->getParentSumoBaseObject();
    //         if (parent == nullptr) {
    //             ok = writeError(TLF("'%' must be defined within the definition of a %.", toString(currentTag), tagsStr));
    //         } else if ((parent->getTag() != SUMO_TAG_ERROR) && std::find(parentTags.begin(), parentTags.end(), parent->getTag()) == parentTags.end()) {
    //             if (parent->hasStringAttribute(SUMO_ATTR_ID)) {
    //                 ok = writeError(TLF("'%' must be defined within the definition of a '%' (found % '%').", toString(currentTag), tagsStr,
    //                                     toString(parent->getTag()), parent->getStringAttribute(SUMO_ATTR_ID)));
    //             } else {
    //                 ok = writeError(TLF("'%' must be defined within the definition of a '%' (found %).", toString(currentTag), tagsStr,
    //                                     toString(parent->getTag())));
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn check_parsed_parent(&mut self, currentTag: SumoXMLTag, parentTags: &Vec<SumoXMLTag>, ok: &mut bool) {
        if parentTags.len() > 0 {
            let mut tagsStr = String::new();
            let n = parentTags.len();
            for (i, parentTag) in parentTags.iter().enumerate() {
                tagsStr.push_str(&tag_str(*parentTag));
                if i + 1 != n {
                    if i + 2 != n {
                        tagsStr.push_str(", ");
                    } else {
                        tagsStr.push_str(" or ");
                    }
                }
            }
            // obtain parent
            let parent = self.myCommonXMLStructure.get_current_sumo_base_object().get_parent_sumo_base_object();
            if parent.is_null() {
                *ok = self.write_error(&format!("'{}' must be defined within the definition of a {}.", tag_str(currentTag), tagsStr));
            } else if (parent.get_tag() != SUMO_TAG_ERROR) && !parentTags.contains(&parent.get_tag()) {
                if parent.has_string_attribute(SUMO_ATTR_ID) {
                    let parentID = parent.get_string_attribute(SUMO_ATTR_ID).cloned().unwrap_or_default();
                    *ok = self.write_error(&format!(
                        "'{}' must be defined within the definition of a '{}' (found {} '{}').",
                        tag_str(currentTag),
                        tagsStr,
                        tag_str(parent.get_tag()),
                        parentID
                    ));
                } else {
                    *ok = self.write_error(&format!(
                        "'{}' must be defined within the definition of a '{}' (found {}).",
                        tag_str(currentTag),
                        tagsStr,
                        tag_str(parent.get_tag())
                    ));
                }
            }
        }
    }

    /// C++: `bool CommonHandler::checkListOfVehicleTypes(const SumoXMLTag, const std::string &, const std::vector<std::string> &)` [protected]
    /// check list of IDs
    // bool
    // CommonHandler::checkListOfVehicleTypes(const SumoXMLTag tag, const std::string& id, const std::vector<std::string>& vTypeIDs) {
    //     for (const auto& vTypeID : vTypeIDs) {
    //         if (!SUMOXMLDefinitions::isValidTypeID(vTypeID)) {
    //             return writeError(TLF("Could not build % with ID '%' in netedit; '%' ist not a valid vType ID.", toString(tag), id, vTypeID));
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn check_list_of_vehicle_types(&mut self, tag: SumoXMLTag, id: &str, vTypeIDs: &Vec<String>) -> bool {
        for vTypeID in vTypeIDs.iter() {
            if !utils::xml::sumo_xml_definitions::is_valid_type_id(vTypeID) {
                return self.write_error(&format!("Could not build {} with ID '{}' in netedit; '{}' ist not a valid vType ID.", tag_str(tag), id, vTypeID));
            }
        }
        true
    }

    /// C++: `bool CommonHandler::checkWithinDistribution(CommonXMLStructure::SumoBaseObject *)` [protected]
    /// check if the given object is within a distribution (VType or routes)
    // bool
    // CommonHandler::checkWithinDistribution(CommonXMLStructure::SumoBaseObject* obj) {
    //     if (obj->getParentSumoBaseObject() == nullptr) {
    //         return false;
    //     } else if (obj->getParentSumoBaseObject()->getTag() == SUMO_TAG_ROUTE_DISTRIBUTION) {
    //         return true;
    //     } else if (obj->getParentSumoBaseObject()->getTag() == SUMO_TAG_VTYPE_DISTRIBUTION) {
    //         return true;
    //     } else {
    //         return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn check_within_distribution(&mut self, obj: Ptr<CommonXMLStructure_SumoBaseObject>) -> bool {
        if obj.get_parent_sumo_base_object().is_null() {
            return false;
        } else if obj.get_parent_sumo_base_object().get_tag() == SUMO_TAG_ROUTE_DISTRIBUTION {
            return true;
        } else if obj.get_parent_sumo_base_object().get_tag() == SUMO_TAG_VTYPE_DISTRIBUTION {
            return true;
        } else {
            return false;
        }
    }

    /// C++: `bool CommonHandler::checkVehicleParents(CommonXMLStructure::SumoBaseObject *)` [protected]
    /// check vehicle parents
    // bool
    // CommonHandler::checkVehicleParents(CommonXMLStructure::SumoBaseObject* obj) {
    //     if (obj == nullptr) {
    //         return false;
    //     } else if (!obj->hasStringAttribute(SUMO_ATTR_ID)) {
    //         return false;
    //     } else {
    //         SumoXMLTag tag = obj->getTag();
    //         const std::string id = obj->getStringAttribute(SUMO_ATTR_ID);
    //         const bool hasRoute = obj->hasStringAttribute(SUMO_ATTR_ROUTE);
    //         const bool hasEmbeddedRoute = (getEmbeddedRoute(obj) != nullptr);
    //         const bool overEdges = obj->hasStringAttribute(SUMO_ATTR_FROM) && obj->hasStringAttribute(SUMO_ATTR_TO);
    //         const bool overJunctions = obj->hasStringAttribute(SUMO_ATTR_FROM_JUNCTION) && obj->hasStringAttribute(SUMO_ATTR_TO_JUNCTION);
    //         const bool overTAZs = obj->hasStringAttribute(SUMO_ATTR_FROM_TAZ) && obj->hasStringAttribute(SUMO_ATTR_TO_TAZ);
    //         if (hasRoute && hasEmbeddedRoute) {
    //             return writeError(TLF("Could not build % with ID '%' in netedit; Cannot have an external route and an embedded route in the same definition.", toString(tag), id));
    //         }
    //         if ((overEdges + overJunctions + overTAZs) > 1) {
    //             return writeError(TLF("Could not build % with ID '%' in netedit; Cannot have multiple from-to attributes.", toString(tag), id));
    //         }
    //         if ((hasRoute + hasEmbeddedRoute + overEdges + overJunctions + overTAZs) > 1) {
    //             return writeError(TLF("Could not build % with ID '%' in netedit; Cannot have from-to attributes and route attributes in the same definition.", toString(tag), id));
    //         }
    //         if ((hasRoute + hasEmbeddedRoute + overEdges + overJunctions + overTAZs) == 0) {
    //             return writeError(TLF("Could not build % with ID '%' in netedit; Requires either a route or an embedded route or a from-to attribute (Edges, junctions or TAZs).", toString(tag), id));
    //         }
    //         return true;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn check_vehicle_parents(&mut self, obj: Ptr<CommonXMLStructure_SumoBaseObject>) -> bool {
        let sbo = obj;
        if obj.is_null() {
            return false;
        } else if !sbo.has_string_attribute(SUMO_ATTR_ID) {
            return false;
        } else {
            let tag = sbo.get_tag();
            let id = sbo.get_string_attribute(SUMO_ATTR_ID).cloned().unwrap_or_default();
            let hasRoute = sbo.has_string_attribute(SUMO_ATTR_ROUTE);
            let hasEmbeddedRoute = !self.get_embedded_route(obj).is_null();
            let overEdges = sbo.has_string_attribute(SUMO_ATTR_FROM) && sbo.has_string_attribute(SUMO_ATTR_TO);
            let overJunctions = sbo.has_string_attribute(SUMO_ATTR_FROM_JUNCTION) && sbo.has_string_attribute(SUMO_ATTR_TO_JUNCTION);
            let overTAZs = sbo.has_string_attribute(SUMO_ATTR_FROM_TAZ) && sbo.has_string_attribute(SUMO_ATTR_TO_TAZ);
            if hasRoute && hasEmbeddedRoute {
                return self.write_error(&format!("Could not build {} with ID '{}' in netedit; Cannot have an external route and an embedded route in the same definition.", tag_str(tag), id));
            }
            if (overEdges as i32 + overJunctions as i32 + overTAZs as i32) > 1 {
                return self.write_error(&format!("Could not build {} with ID '{}' in netedit; Cannot have multiple from-to attributes.", tag_str(tag), id));
            }
            if (hasRoute as i32 + hasEmbeddedRoute as i32 + overEdges as i32 + overJunctions as i32 + overTAZs as i32) > 1 {
                return self.write_error(&format!("Could not build {} with ID '{}' in netedit; Cannot have from-to attributes and route attributes in the same definition.", tag_str(tag), id));
            }
            if (hasRoute as i32 + hasEmbeddedRoute as i32 + overEdges as i32 + overJunctions as i32 + overTAZs as i32) == 0 {
                return self.write_error(&format!("Could not build {} with ID '{}' in netedit; Requires either a route or an embedded route or a from-to attribute (Edges, junctions or TAZs).", tag_str(tag), id));
            }
            return true;
        }
    }

    /// C++: `bool CommonHandler::checkPersonPlanParents(CommonXMLStructure::SumoBaseObject *)` [protected]
    /// check person plan parents
    // bool
    // CommonHandler::checkPersonPlanParents(CommonXMLStructure::SumoBaseObject* obj) {
    //     const auto parent = obj->getParentSumoBaseObject();
    //     if (parent == nullptr) {
    //         return false;
    //     } else if (!parent->wasCreated()) {
    //         return false;
    //     } else if ((parent->getTag() == SUMO_TAG_PERSON) || (parent->getTag() == SUMO_TAG_PERSONFLOW)) {
    //         return true;
    //     } else {
    //         return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn check_person_plan_parents(&mut self, obj: Ptr<CommonXMLStructure_SumoBaseObject>) -> bool {
        let parent = obj.get_parent_sumo_base_object();
        if parent.is_null() {
            return false;
        } else if !parent.was_created() {
            return false;
        } else if (parent.get_tag() == SUMO_TAG_PERSON) || (parent.get_tag() == SUMO_TAG_PERSONFLOW) {
            return true;
        } else {
            return false;
        }
    }

    /// C++: `bool CommonHandler::checkContainerPlanParents(CommonXMLStructure::SumoBaseObject *)` [protected]
    /// check container plan parents
    // bool
    // CommonHandler::checkContainerPlanParents(CommonXMLStructure::SumoBaseObject* obj) {
    //     const auto parent = obj->getParentSumoBaseObject();
    //     if (parent == nullptr) {
    //         return false;
    //     } else if (!parent->wasCreated()) {
    //         return false;
    //     } else if ((parent->getTag() == SUMO_TAG_CONTAINER) || (parent->getTag() == SUMO_TAG_CONTAINERFLOW)) {
    //         return true;
    //     } else {
    //         return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn check_container_plan_parents(&mut self, obj: Ptr<CommonXMLStructure_SumoBaseObject>) -> bool {
        let parent = obj.get_parent_sumo_base_object();
        if parent.is_null() {
            return false;
        } else if !parent.was_created() {
            return false;
        } else if (parent.get_tag() == SUMO_TAG_CONTAINER) || (parent.get_tag() == SUMO_TAG_CONTAINERFLOW) {
            return true;
        } else {
            return false;
        }
    }

    /// C++: `bool CommonHandler::checkStopParents(CommonXMLStructure::SumoBaseObject *)` [protected]
    /// check stop parents
    // bool
    // CommonHandler::checkStopParents(CommonXMLStructure::SumoBaseObject* obj) {
    //     const auto parent = obj->getParentSumoBaseObject();
    //     if (parent == nullptr) {
    //         return false;
    //     } else if (!parent->wasCreated()) {
    //         return false;
    //     } else if ((parent->getTag() == SUMO_TAG_ROUTE) || (parent->getTag() == SUMO_TAG_TRIP) ||
    //                (parent->getTag() == SUMO_TAG_VEHICLE) || (parent->getTag() == SUMO_TAG_FLOW) ||
    //                (parent->getTag() == SUMO_TAG_PERSON) || (parent->getTag() == SUMO_TAG_PERSONFLOW) ||
    //                (parent->getTag() == SUMO_TAG_CONTAINER) || (parent->getTag() == SUMO_TAG_CONTAINERFLOW)) {
    //         return true;
    //     } else {
    //         return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn check_stop_parents(&mut self, obj: Ptr<CommonXMLStructure_SumoBaseObject>) -> bool {
        let parent = obj.get_parent_sumo_base_object();
        if parent.is_null() {
            return false;
        } else if !parent.was_created() {
            return false;
        } else if (parent.get_tag() == SUMO_TAG_ROUTE)
            || (parent.get_tag() == SUMO_TAG_TRIP)
            || (parent.get_tag() == SUMO_TAG_VEHICLE)
            || (parent.get_tag() == SUMO_TAG_FLOW)
            || (parent.get_tag() == SUMO_TAG_PERSON)
            || (parent.get_tag() == SUMO_TAG_PERSONFLOW)
            || (parent.get_tag() == SUMO_TAG_CONTAINER)
            || (parent.get_tag() == SUMO_TAG_CONTAINERFLOW)
        {
            return true;
        } else {
            return false;
        }
    }

    /// C++: `bool CommonHandler::checkNegative(const SumoXMLTag, const std::string &, const SumoXMLAttr, const int, const bool)` [protected]
    /// check if the given int value is NOT negative
    // bool
    // CommonHandler::checkNegative(const SumoXMLTag tag, const std::string& id, const SumoXMLAttr attribute, const int value, const bool canBeZero) {
    //     if (canBeZero) {
    //         if (value < 0) {
    //             return writeError(TLF("Could not build % with ID '%' in netedit; Attribute % cannot be negative.", toString(tag), id, toString(attribute)));
    //         } else {
    //             return true;
    //         }
    //     } else {
    //         if (value <= 0) {
    //             return writeError(TLF("Could not build % with ID '%' in netedit; Attribute % must be greather than zero.", toString(tag), id, toString(attribute)));
    //         } else {
    //             return true;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn check_negative(&mut self, tag: SumoXMLTag, id: &str, attribute: SumoXMLAttr, value: i32, canBeZero: bool) -> bool {
        if canBeZero {
            if value < 0 {
                return self.write_error(&format!("Could not build {} with ID '{}' in netedit; Attribute {} cannot be negative.", tag_str(tag), id, attr_str(attribute)));
            } else {
                return true;
            }
        } else {
            if value <= 0 {
                return self.write_error(&format!("Could not build {} with ID '{}' in netedit; Attribute {} must be greather than zero.", tag_str(tag), id, attr_str(attribute)));
            } else {
                return true;
            }
        }
    }

    /// C++: `bool CommonHandler::checkNegative(const SumoXMLTag, const std::string &, const SumoXMLAttr, const double, const bool)` [protected]
    /// check if the given double value is NOT negative
    // bool
    // CommonHandler::checkNegative(const SumoXMLTag tag, const std::string& id, const SumoXMLAttr attribute, const double value, const bool canBeZero) {
    //     if (canBeZero) {
    //         if (value < 0) {
    //             return writeError(TLF("Could not build % with ID '%' in netedit; Attribute % cannot be negative (%).", toString(tag), id, toString(attribute), toString(value)));
    //         } else {
    //             return true;
    //         }
    //     } else {
    //         if (value <= 0) {
    //             return writeError(TLF("Could not build % with ID '%' in netedit; Attribute % must be greather than zero (%).", toString(tag), id, toString(attribute), toString(value)));
    //         } else {
    //             return true;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn check_negative_2(&mut self, tag: SumoXMLTag, id: &str, attribute: SumoXMLAttr, value: f64, canBeZero: bool) -> bool {
        if canBeZero {
            if value < 0.0 {
                return self.write_error(&format!("Could not build {} with ID '{}' in netedit; Attribute {} cannot be negative ({}).", tag_str(tag), id, attr_str(attribute), utils::common::to_string::to_string(&value)));
            } else {
                return true;
            }
        } else {
            if value <= 0.0 {
                return self.write_error(&format!("Could not build {} with ID '{}' in netedit; Attribute {} must be greather than zero ({}).", tag_str(tag), id, attr_str(attribute), utils::common::to_string::to_string(&value)));
            } else {
                return true;
            }
        }
    }

    /// C++: `bool CommonHandler::checkNegative(const SumoXMLTag, const std::string &, const SumoXMLAttr, const SUMOTime, const bool)` [protected]
    /// check if the given SUMOTime value is NOT negative
    // bool
    // CommonHandler::checkNegative(const SumoXMLTag tag, const std::string& id, const SumoXMLAttr attribute, const SUMOTime value, const bool canBeZero) {
    //     if (canBeZero) {
    //         if (value < 0) {
    //             return writeError(TLF("Could not build % with ID '%' in netedit; Attribute % cannot be negative (%).", toString(tag), id, toString(attribute), time2string(value)));
    //         } else {
    //             return true;
    //         }
    //     } else {
    //         if (value <= 0) {
    //             return writeError(TLF("Could not build % with ID '%' in netedit; Attribute % must be greather than zero (%).", toString(tag), id, toString(attribute), time2string(value)));
    //         } else {
    //             return true;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn check_negative_3(&mut self, tag: SumoXMLTag, id: &str, attribute: SumoXMLAttr, value: SUMOTime, canBeZero: bool) -> bool {
        if canBeZero {
            if value < 0 {
                return self.write_error(&format!("Could not build {} with ID '{}' in netedit; Attribute {} cannot be negative ({}).", tag_str(tag), id, attr_str(attribute), time2string(value)));
            } else {
                return true;
            }
        } else {
            if value <= 0 {
                return self.write_error(&format!("Could not build {} with ID '{}' in netedit; Attribute {} must be greather than zero ({}).", tag_str(tag), id, attr_str(attribute), time2string(value)));
            } else {
                return true;
            }
        }
    }

    /// C++: `bool CommonHandler::checkFileName(const SumoXMLTag, const std::string &, const SumoXMLAttr, const std::string &)` [protected]
    /// check if the given filename is valid
    // bool
    // CommonHandler::checkFileName(const SumoXMLTag tag, const std::string& id, const SumoXMLAttr attribute, const std::string& value) {
    //     if (SUMOXMLDefinitions::isValidFilename(value)) {
    //         return true;
    //     } else {
    //         return writeError(TLF("Could not build % with ID '%' in netedit; % is invalid % ()", toString(tag), id, toString(attribute), value));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn check_file_name(&mut self, tag: SumoXMLTag, id: &str, attribute: SumoXMLAttr, value: &str) -> bool {
        if utils::xml::sumo_xml_definitions::is_valid_filename(value) {
            true
        } else {
            self.write_error(&format!("Could not build {} with ID '{}' in netedit; {} is invalid {} ()", tag_str(tag), id, attr_str(attribute), value))
        }
    }

    /// C++: `bool CommonHandler::checkValidAdditionalID(const SumoXMLTag, const std::string &)` [protected]
    /// check if the given additional ID is valid
    // bool
    // CommonHandler::checkValidAdditionalID(const SumoXMLTag tag, const std::string& value) {
    //     if (value.empty()) {
    //         return writeError(TLF("Could not build %; ID cannot be empty", toString(tag)));
    //     } else if (!SUMOXMLDefinitions::isValidVehicleID(value)) {
    //         return writeError(TLF("Could not build % with ID '%' in netedit; ID contains invalid characters.", toString(tag), value));
    //     } else {
    //         return true;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn check_valid_additional_id(&mut self, tag: SumoXMLTag, value: &str) -> bool {
        if value.is_empty() {
            self.write_error(&format!("Could not build {}; ID cannot be empty", tag_str(tag)))
        } else if !utils::xml::sumo_xml_definitions::is_valid_vehicle_id(value) {
            self.write_error(&format!("Could not build {} with ID '{}' in netedit; ID contains invalid characters.", tag_str(tag), value))
        } else {
            true
        }
    }

    /// C++: `bool CommonHandler::checkValidDetectorID(const SumoXMLTag, const std::string &)` [protected]
    /// check if the given detector ID is valid
    // bool
    // CommonHandler::checkValidDetectorID(const SumoXMLTag tag, const std::string& value) {
    //     if (value.empty()) {
    //         return writeError(TLF("Could not build %; ID cannot be empty", toString(tag)));
    //     } else if (!SUMOXMLDefinitions::isValidDetectorID(value)) {
    //         return writeError(TLF("Could not build % with ID '%' in netedit; detector ID contains invalid characters.", toString(tag), value));
    //     } else {
    //         return true;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn check_valid_detector_id(&mut self, tag: SumoXMLTag, value: &str) -> bool {
        if value.is_empty() {
            self.write_error(&format!("Could not build {}; ID cannot be empty", tag_str(tag)))
        } else if !utils::xml::sumo_xml_definitions::is_valid_detector_id(value) {
            self.write_error(&format!("Could not build {} with ID '{}' in netedit; detector ID contains invalid characters.", tag_str(tag), value))
        } else {
            true
        }
    }

    /// C++: `bool CommonHandler::checkValidDemandElementID(const SumoXMLTag, const std::string &)` [protected]
    /// check if the given demand elmement ID is valid
    // bool
    // CommonHandler::checkValidDemandElementID(const SumoXMLTag tag, const std::string& value) {
    //     if (value.empty()) {
    //         return writeError(TLF("Could not build %; ID cannot be empty", toString(tag)));
    //     } else if (!SUMOXMLDefinitions::isValidVehicleID(value)) {
    //         return writeError(TLF("Could not build % with ID '%' in netedit; ID contains invalid characters.", toString(tag), value));
    //     } else {
    //         return true;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn check_valid_demand_element_id(&mut self, tag: SumoXMLTag, value: &str) -> bool {
        if value.is_empty() {
            self.write_error(&format!("Could not build {}; ID cannot be empty", tag_str(tag)))
        } else if !utils::xml::sumo_xml_definitions::is_valid_vehicle_id(value) {
            self.write_error(&format!("Could not build {} with ID '{}' in netedit; ID contains invalid characters.", tag_str(tag), value))
        } else {
            true
        }
    }

    /// C++: `void CommonHandler::writeWarningOverwriting(const SumoXMLTag, const std::string &)` [protected]
    /// write warning overwritting element
    // void
    // CommonHandler::writeWarningOverwriting(const SumoXMLTag tag, const std::string& id) {
    //     WRITE_WARNING(TLF("Overwriting % with ID '%'", toString(tag), id));
    // }
    #[allow(unused_variables)]
    pub fn write_warning_overwriting(&mut self, tag: SumoXMLTag, id: &str) {
        utils::common::msg_handler::write_warning(&format!("Overwriting {} with ID '{}'", tag_str(tag), id));
    }

    /// C++: `bool CommonHandler::writeWarningDuplicated(const SumoXMLTag, const std::string &, const SumoXMLTag)` [protected]
    /// write warning duplicated element
    // bool
    // CommonHandler::writeWarningDuplicated(const SumoXMLTag tag, const std::string& id, const SumoXMLTag checkedTag) {
    //     WRITE_WARNING(TLF("Could not build % with ID '%' in netedit; Found another % with the same ID.", toString(tag), id, toString(checkedTag)));
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn write_warning_duplicated(&mut self, tag: SumoXMLTag, id: &str, checkedTag: SumoXMLTag) -> bool {
        utils::common::msg_handler::write_warning(&format!("Could not build {} with ID '{}' in netedit; Found another {} with the same ID.", tag_str(tag), id, tag_str(checkedTag)));
        false
    }

    /// C++: `bool CommonHandler::writeError(const std::string &)` [protected]
    /// write error and enable error creating element
    // bool
    // CommonHandler::writeError(const std::string& error) {
    //     WRITE_ERROR(error);
    //     myErrorCreatingElement = true;
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn write_error(&mut self, error: &str) -> bool {
        utils::common::msg_handler::write_error(error);
        self.myErrorCreatingElement = true;
        false
    }

    /// C++: `bool CommonHandler::writeErrorInvalidPosition(const SumoXMLTag, const std::string &)` [protected]
    /// write error "invalid position"
    // bool
    // CommonHandler::writeErrorInvalidPosition(const SumoXMLTag tag, const std::string& id) {
    //     return writeError(TLF("Could not build % with ID '%' in netedit; Invalid position over lane.", toString(tag), id));
    // }
    #[allow(unused_variables)]
    pub fn write_error_invalid_position(&mut self, tag: SumoXMLTag, id: &str) -> bool {
        self.write_error(&format!("Could not build {} with ID '{}' in netedit; Invalid position over lane.", tag_str(tag), id))
    }

    /// C++: `bool CommonHandler::writeErrorInvalidLanes(const SumoXMLTag, const std::string &)` [protected]
    /// write error "invalid list of lanes"
    // bool
    // CommonHandler::writeErrorInvalidLanes(const SumoXMLTag tag, const std::string& id) {
    //     return writeError(TLF("Could not build % with ID '%' in netedit; List of lanes isn't valid.", toString(tag), id));
    // }
    #[allow(unused_variables)]
    pub fn write_error_invalid_lanes(&mut self, tag: SumoXMLTag, id: &str) -> bool {
        self.write_error(&format!("Could not build {} with ID '{}' in netedit; List of lanes isn't valid.", tag_str(tag), id))
    }

    /// C++: `bool CommonHandler::writeErrorInvalidParent(const SumoXMLTag, const std::string &, const std::vector<SumoXMLTag>, const std::string &)` [protected]
    /// write error "invalid parent element" giving ids of current and parent element
    // bool
    // CommonHandler::writeErrorInvalidParent(const SumoXMLTag tag, const std::string& id, const std::vector<SumoXMLTag> parentTags, const std::string& parentID) {
    //     if (parentTags.size() == 1) {
    //         return writeError(TLF("Could not build % with ID '%' in netedit; % parent with ID '%' doesn't exist.", toString(tag), id, toString(parentTags.front()), parentID));
    //     } else {
    //         return writeError(TLF("Could not build % with ID '%' in netedit; % parent with ID '%' doesn't exist.", toString(tag), id, parseParentTags(parentTags), parentID));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_error_invalid_parent(&mut self, tag: SumoXMLTag, id: &str, parentTags: Vec<SumoXMLTag>, parentID: &str) -> bool {
        if parentTags.len() == 1 {
            self.write_error(&format!("Could not build {} with ID '{}' in netedit; {} parent with ID '{}' doesn't exist.", tag_str(tag), id, tag_str(parentTags[0]), parentID))
        } else {
            let parents = self.parse_parent_tags(parentTags);
            self.write_error(&format!("Could not build {} with ID '{}' in netedit; {} parent with ID '{}' doesn't exist.", tag_str(tag), id, parents, parentID))
        }
    }

    /// C++: `bool CommonHandler::writeErrorInvalidParent(const SumoXMLTag, const std::vector<SumoXMLTag>, const std::string &)` [protected]
    /// write error "invalid parent element" giving only the id of parent element
    // bool
    // CommonHandler::writeErrorInvalidParent(const SumoXMLTag tag, const std::vector<SumoXMLTag> parentTags, const std::string& parentID) {
    //     if (parentTags.size() == 1) {
    //         return writeError(TLF("Could not build % in netedit; % parent with ID '%' doesn't exist.", toString(tag), toString(parentTags.front()), parentID));
    //     } else {
    //         return writeError(TLF("Could not build % in netedit; % parent with ID '%' doesn't exist.", toString(tag), parseParentTags(parentTags), parentID));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_error_invalid_parent_2(&mut self, tag: SumoXMLTag, parentTags: Vec<SumoXMLTag>, parentID: &str) -> bool {
        if parentTags.len() == 1 {
            self.write_error(&format!("Could not build {} in netedit; {} parent with ID '{}' doesn't exist.", tag_str(tag), tag_str(parentTags[0]), parentID))
        } else {
            let parents = self.parse_parent_tags(parentTags);
            self.write_error(&format!("Could not build {} in netedit; {} parent with ID '{}' doesn't exist.", tag_str(tag), parents, parentID))
        }
    }

    /// C++: `bool CommonHandler::writeErrorInvalidParent(const SumoXMLTag, const std::vector<SumoXMLTag>)` [protected]
    /// write error "invalid parent element" without giving IDs
    // bool
    // CommonHandler::writeErrorInvalidParent(const SumoXMLTag tag, const std::vector<SumoXMLTag> parentTags) {
    //     if (parentTags.size() == 1) {
    //         return writeError(TLF("Could not build % in netedit; % parent doesn't exist.", toString(tag), toString(parentTags.front())));
    //     } else {
    //         return writeError(TLF("Could not build % in netedit; % parent doesn't exist.", toString(tag), parseParentTags(parentTags)));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_error_invalid_parent_3(&mut self, tag: SumoXMLTag, parentTags: Vec<SumoXMLTag>) -> bool {
        if parentTags.len() == 1 {
            self.write_error(&format!("Could not build {} in netedit; {} parent doesn't exist.", tag_str(tag), tag_str(parentTags[0])))
        } else {
            let parents = self.parse_parent_tags(parentTags);
            self.write_error(&format!("Could not build {} in netedit; {} parent doesn't exist.", tag_str(tag), parents))
        }
    }

    /// C++: `std::string CommonHandler::parseParentTags(std::vector<SumoXMLTag>) const` [const, private]
    /// parse list of parent tags
    // std::string
    // CommonHandler::parseParentTags(std::vector<SumoXMLTag> parentTags) const {
    //     std::string parentTagsStr;
    //     // Join all but the last with ", ", then add " or " + last
    //     std::string result;
    //     for (int i = 0; i < ((int)parentTags.size() - 1); i++) {
    //         if (i != 0) {
    //             result.append(", ");
    //         }
    //         result.append(toString(parentTags[i]));
    //     }
    //     result.append(TL(" or "));
    //     result.append(toString(parentTags.back()));
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn parse_parent_tags(&self, parentTags: Vec<SumoXMLTag>) -> String {
        let mut result = String::new();
        // Join all but the last with ", ", then add " or " + last
        for i in 0..(parentTags.len() as i64 - 1) {
            if i != 0 {
                result.push_str(", ");
            }
            result.push_str(&tag_str(parentTags[i as usize]));
        }
        result.push_str(" or ");
        // shortcut: C++ parentTags.back() on an empty vector is undefined; empty list yields no last tag here
        if let Some(last) = parentTags.last() {
            result.push_str(&tag_str(*last));
        }
        result
    }

    /// C++: `void CommonHandler::CommonHandler()` [private]
    /// invalidate default onstructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2() -> Self {
        panic!("CommonHandler() is private and never defined in C++")
    }

    /// C++: `void CommonHandler::CommonHandler(const CommonHandler &)` [private]
    /// invalidate copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_3(s: &CommonHandler) -> Self {
        panic!("CommonHandler(const CommonHandler&) is private and never defined in C++")
    }

    /// C++: `CommonHandler & CommonHandler::operator=(const CommonHandler &)` [private]
    /// invalidate assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &CommonHandler) -> &mut CommonHandler {
        panic!("CommonHandler::operator= is private and never defined in C++")
    }

}

