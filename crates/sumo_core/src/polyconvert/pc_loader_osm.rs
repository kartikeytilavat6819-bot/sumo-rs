//! Port of polyconvert/PCLoaderOSM.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use std::sync::LazyLock;
use std::sync::atomic::Ordering;
use utils::common::file_helpers::is_readable;
use utils::common::msg_handler::{progress_begin_message, progress_time_message, with_instance, MsgType};
use utils::common::string_utils::{escape_xml, parse_dist};
use utils::common::to_string::to_string;
use utils::options::options_cont::{get_bool, get_float};
use utils::xml::generic_sax_handler::SaxHandler;
use utils::xml::xml_sub_sys::run_parser;

// C++ static members of PCLoaderOSM (see AGENTS.md: module-level statics)
static MY_KEYS_TO_INCLUDE: LazyLock<BTreeSet<String>> = LazyLock::new(PCLoaderOSM::init_my_keys_to_include);
static MY_FULL_TYPE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

// owned objects that C++ allocates with new and frees with delete
fn leak_box<T>(v: T) -> Ptr<T> {
    Ptr::from_raw(Box::into_raw(Box::new(v)))
}

fn free_box<T>(p: Ptr<T>) {
    unsafe { drop(Box::from_raw(p.as_raw())) }
}

fn project(pos: &mut Position) -> bool {
    GeoConvHelper::with_processing(|g| g.x2cartesian(pos, true))
}

fn or_panic<T, E: std::fmt::Display>(r: Result<T, E>) -> T {
    r.unwrap_or_else(|e| panic!("{e}"))
}

/// SysUtils::getCurrentMillis()
fn current_millis() -> i64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as i64).unwrap_or(0)
}

/// C++ typedef `std::map<long long, PCOSMEdge *> PCLoaderOSM::EdgeMap`
pub type PCLoaderOSM_EdgeMap = BTreeMap<i64, Ptr<PCLoaderOSM_PCOSMEdge>>;

/// C++ typedef `std::vector<PCOSMRelation *> PCLoaderOSM::Relations`
pub type PCLoaderOSM_Relations = Vec<Ptr<PCLoaderOSM_PCOSMRelation>>;

/// C++ typedef `std::map<long long, PCOSMRelation *> PCLoaderOSM::RelationsMap`
pub type PCLoaderOSM_RelationsMap = BTreeMap<i64, Ptr<PCLoaderOSM_PCOSMRelation>>;

/// A class which extracts OSM-edges from a parsed OSM-file
/// C++ class `PCLoaderOSM::EdgesHandler` (polyconvert/PCLoaderOSM.h)
#[repr(C)]
pub struct PCLoaderOSM_EdgesHandler {
    /// C++ base class `SUMOSAXHandler`
    pub base: GenericSAXHandler,
    /// Whether all attributes shall be stored
    pub myWithAttributes: bool,
    /// The handler to report errors to (will be the WarningsHandler if --ignore-errors was set)
    pub myErrorHandler: Ptr<MsgHandler>,
    /// The previously parsed nodes
    pub myOSMNodes: Ptr<BTreeMap<i64, Ptr<PCLoaderOSM_PCOSMNode>>>,
    /// A map of built edges
    pub myEdgeMap: Ptr<PCLoaderOSM_EdgeMap>,
    /// additional ways which are reference by relations
    pub myAdditionalWays: Ptr<PCLoaderOSM_RelationsMap>,
    /// The currently built edge
    pub myCurrentEdge: Ptr<PCLoaderOSM_PCOSMEdge>,
    /// Current path in order to know to what occuring values belong
    pub myParentElements: Vec<i32>,
    /// whether the last edge (way) should be kept because it had a key from the inclusion list
    pub myKeep: bool,
}

impl std::ops::Deref for PCLoaderOSM_EdgesHandler {
    type Target = GenericSAXHandler;
    fn deref(&self) -> &GenericSAXHandler {
        &self.base
    }
}
impl std::ops::DerefMut for PCLoaderOSM_EdgesHandler {
    fn deref_mut(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
}

impl PCLoaderOSM_EdgesHandler {
    /// C++: `void EdgesHandler::EdgesHandler(const std::map<long long, PCOSMNode *> &, EdgeMap &, const RelationsMap &, bool, MsgHandler &)`
    /// Constructor
    // PCLoaderOSM::EdgesHandler::EdgesHandler(const std::map<long long int, PCOSMNode*>& osmNodes,
    //                                         EdgeMap& toFill,
    //                                         const RelationsMap& additionalWays,
    //                                         bool withAttributes, MsgHandler& errorHandler) :
    //     SUMOSAXHandler("osm - file"),
    //     myWithAttributes(withAttributes),
    //     myErrorHandler(errorHandler),
    //     myOSMNodes(osmNodes),
    //     myEdgeMap(toFill),
    //     myAdditionalWays(additionalWays) {
    // }
    #[allow(unused_variables)]
    pub fn new(osmNodes: &BTreeMap<i64, Ptr<PCLoaderOSM_PCOSMNode>>, toFill: &mut PCLoaderOSM_EdgeMap, additionalWays: &PCLoaderOSM_RelationsMap, withAttributes: bool, errorHandler: &mut MsgHandler) -> Self {
        PCLoaderOSM_EdgesHandler {
            base: utils::xml::sumo_sax_handler::new("osm - file", ""),
            myWithAttributes: withAttributes,
            myErrorHandler: Ptr::from_mut(errorHandler),
            myOSMNodes: Ptr::new(osmNodes),
            myEdgeMap: Ptr::from_mut(toFill),
            myAdditionalWays: Ptr::new(additionalWays),
            myCurrentEdge: Ptr::null(),
            myParentElements: Vec::new(),
            myKeep: false,
        }
    }

    /// C++: `void EdgesHandler::~EdgesHandler() noexcept` [virtual]
    /// Destructor
    // PCLoaderOSM::EdgesHandler::~EdgesHandler() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `void EdgesHandler::myStartElement(int, const SUMOSAXAttributes &)` [virtual, protected]
    /// Called on the opening of a tag;
    // void
    // PCLoaderOSM::EdgesHandler::myStartElement(int element, const SUMOSAXAttributes& attrs) {
    //     myParentElements.push_back(element);
    //     // parse "way" elements
    //     if (element == SUMO_TAG_WAY) {
    //         bool ok = true;
    //         const long long int id = attrs.get<long long int>(SUMO_ATTR_ID, nullptr, ok);
    //         const std::string& action = attrs.getOpt<std::string>(SUMO_ATTR_ACTION, nullptr, ok);
    //         if (action == "delete" || !ok) {
    //             myCurrentEdge = nullptr;
    //             return;
    //         }
    //         myCurrentEdge = new PCOSMEdge();
    //         myCurrentEdge->id = id;
    //         myCurrentEdge->myIsClosed = false;
    //         myCurrentEdge->standalone = false;
    //         myKeep = (myAdditionalWays.find(id) != myAdditionalWays.end());
    //     }
    //     // parse "nd" (node) elements
    //     if (element == SUMO_TAG_ND && myCurrentEdge != nullptr) {
    //         bool ok = true;
    //         const long long int ref = attrs.get<long long int>(SUMO_ATTR_REF, nullptr, ok);
    //         if (ok) {
    //             if (myOSMNodes.find(ref) == myOSMNodes.end()) {
    //                 WRITE_WARNINGF(TL("The referenced geometry information (ref='%') is not known"), ref);
    //                 return;
    //             }
    //             myCurrentEdge->myCurrentNodes.push_back(ref);
    //         }
    //     }
    //     // parse values
    //     if (element == SUMO_TAG_TAG && myParentElements.size() > 2 && myParentElements[myParentElements.size() - 2] == SUMO_TAG_WAY
    //             && myCurrentEdge != nullptr) {
    //         bool ok = true;
    //         std::string key = attrs.getOpt<std::string>(SUMO_ATTR_K, toString(myCurrentEdge->id).c_str(), ok, "", false);
    //         std::string value = attrs.getOpt<std::string>(SUMO_ATTR_V, toString(myCurrentEdge->id).c_str(), ok, "", false);
    //         if (key == "") {
    //             myErrorHandler.inform("Empty key in a a tag while parsing way '" + toString(myCurrentEdge->id) + "' occurred.");
    //             ok = false;
    //         }
    //         if (!ok) {
    //             return;
    //         }
    //         if (key == "name") {
    //             myCurrentEdge->name = value;
    //         } else if (MyKeysToInclude.count(key) > 0) {
    //             myKeep = true;
    //             myCurrentEdge->standalone = true;
    //         }
    //         myCurrentEdge->myAttributes[key] = value;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) {
        self.myParentElements.push(element);
        // parse "way" elements
        if element == SUMO_TAG_WAY {
            let mut ok = true;
            let id: i64 = attrs.get(SUMO_ATTR_ID, "", &mut ok, true);
            let action: String = attrs.get_opt(SUMO_ATTR_ACTION, "", &mut ok, String::new(), true);
            if action == "delete" || !ok {
                self.myCurrentEdge = Ptr::null();
                return;
            }
            self.myCurrentEdge = leak_box(PCLoaderOSM_PCOSMEdge {
                id,
                name: String::new(),
                myIsClosed: false,
                myCurrentNodes: Vec::new(),
                myAttributes: BTreeMap::new(),
                standalone: false,
            });
            self.myKeep = self.myAdditionalWays.contains_key(&id);
        }
        // parse "nd" (node) elements
        if element == SUMO_TAG_ND && !self.myCurrentEdge.is_null() {
            let mut ok = true;
            let refid: i64 = attrs.get(SUMO_ATTR_REF, "", &mut ok, true);
            if ok {
                if !self.myOSMNodes.contains_key(&refid) {
                    write_warningf!("The referenced geometry information (ref='%') is not known", refid);
                    return;
                }
                self.myCurrentEdge.m().myCurrentNodes.push(refid);
            }
        }
        // parse values
        if element == SUMO_TAG_TAG
            && self.myParentElements.len() > 2
            && self.myParentElements[self.myParentElements.len() - 2] == SUMO_TAG_WAY
            && !self.myCurrentEdge.is_null()
        {
            let mut ok = true;
            let edgeID = self.myCurrentEdge.id.to_string();
            let key: String = attrs.get_opt(SUMO_ATTR_K, &edgeID, &mut ok, String::new(), false);
            let value: String = attrs.get_opt(SUMO_ATTR_V, &edgeID, &mut ok, String::new(), false);
            if key == "" {
                self.myErrorHandler.m().inform(&format!("Empty key in a a tag while parsing way '{}' occurred.", edgeID), true);
                ok = false;
            }
            if !ok {
                return;
            }
            if key == "name" {
                self.myCurrentEdge.m().name = value.clone();
            } else if MY_KEYS_TO_INCLUDE.contains(&key) {
                self.myKeep = true;
                self.myCurrentEdge.m().standalone = true;
            }
            self.myCurrentEdge.m().myAttributes.insert(key, value);
        }
    }

    /// C++: `void EdgesHandler::myEndElement(int)` [virtual, protected]
    /// Called when a closing tag occurs
    // void
    // PCLoaderOSM::EdgesHandler::myEndElement(int element) {
    //     myParentElements.pop_back();
    //     if (element == SUMO_TAG_WAY && myCurrentEdge != nullptr) {
    //         if (myKeep) {
    //             RelationsMap::const_iterator it = myAdditionalWays.find(myCurrentEdge->id);
    //             if (it != myAdditionalWays.end()) {
    //                 myCurrentEdge->myAttributes.insert((*it).second->myAttributes.begin(), (*it).second->myAttributes.end());
    //             }
    //             myEdgeMap[myCurrentEdge->id] = myCurrentEdge;
    //         } else {
    //             delete myCurrentEdge;
    //         }
    //         myCurrentEdge = nullptr;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_end_element(&mut self, element: i32) {
        self.myParentElements.pop();
        if element == SUMO_TAG_WAY && !self.myCurrentEdge.is_null() {
            if self.myKeep {
                if let Some(rel) = (*self.myAdditionalWays).get(&self.myCurrentEdge.id) {
                    for (k, v) in rel.myAttributes.iter() {
                        self.myCurrentEdge.m().myAttributes.entry(k.clone()).or_insert_with(|| v.clone());
                    }
                }
                let id = self.myCurrentEdge.id;
                self.myEdgeMap.m().insert(id, self.myCurrentEdge);
            } else {
                free_box(self.myCurrentEdge);
            }
            self.myCurrentEdge = Ptr::null();
        }
    }

    /// C++: `void EdgesHandler::EdgesHandler(const EdgesHandler &)` [private]
    /// Invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &NIImporter_DlrNavteq_EdgesHandler) -> Self {
        panic!("PCLoaderOSM::EdgesHandler copy constructor is private and undefined in C++ (invalidated)")
    }

    /// C++: `EdgesHandler & EdgesHandler::operator=(const EdgesHandler &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &NIImporter_DlrNavteq_EdgesHandler) -> &mut NIImporter_DlrNavteq_EdgesHandler {
        panic!("PCLoaderOSM::EdgesHandler assignment operator is private and undefined in C++ (invalidated)")
    }

}

impl SaxHandler for PCLoaderOSM_EdgesHandler {
    fn handler(&mut self) -> &mut utils::xml::generic_sax_handler::GenericSAXHandler {
        &mut self.base
    }
    fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) -> Result<(), ProcessError> {
        PCLoaderOSM_EdgesHandler::my_start_element(self, element, attrs);
        Ok(())
    }
    fn my_end_element(&mut self, element: i32) -> Result<(), ProcessError> {
        PCLoaderOSM_EdgesHandler::my_end_element(self, element);
        Ok(())
    }
}

/// A class which extracts OSM-nodes from a parsed OSM-file
/// C++ class `PCLoaderOSM::NodesHandler` (polyconvert/PCLoaderOSM.h)
#[repr(C)]
pub struct PCLoaderOSM_NodesHandler {
    /// C++ base class `SUMOSAXHandler`
    pub base: GenericSAXHandler,
    /// Whether all attributes shall be stored
    pub myWithAttributes: bool,
    /// The handler to report errors to (will be the WarningsHandler if --ignore-errors was set)
    pub myErrorHandler: Ptr<MsgHandler>,
    /// The nodes container to fill
    pub myToFill: Ptr<BTreeMap<i64, Ptr<PCLoaderOSM_PCOSMNode>>>,
    /// Current path in order to know to what occuring values belong
    pub myParentElements: Vec<i32>,
    /// The id of the last parsed node
    pub myLastNodeID: i64,
}

impl std::ops::Deref for PCLoaderOSM_NodesHandler {
    type Target = GenericSAXHandler;
    fn deref(&self) -> &GenericSAXHandler {
        &self.base
    }
}
impl std::ops::DerefMut for PCLoaderOSM_NodesHandler {
    fn deref_mut(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
}

impl PCLoaderOSM_NodesHandler {
    /// C++: `void NodesHandler::NodesHandler(std::map<long long, PCOSMNode *> &, bool, MsgHandler &)`
    /// Contructor
    // PCLoaderOSM::NodesHandler::NodesHandler(std::map<long long int, PCOSMNode*>& toFill,
    //                                         bool withAttributes, MsgHandler& errorHandler) :
    //     SUMOSAXHandler("osm - file"),
    //     myWithAttributes(withAttributes),
    //     myErrorHandler(errorHandler),
    //     myToFill(toFill), myLastNodeID(-1) {}
    #[allow(unused_variables)]
    pub fn new(toFill: &mut BTreeMap<i64, Ptr<PCLoaderOSM_PCOSMNode>>, withAttributes: bool, errorHandler: &mut MsgHandler) -> Self {
        PCLoaderOSM_NodesHandler {
            base: utils::xml::sumo_sax_handler::new("osm - file", ""),
            myWithAttributes: withAttributes,
            myErrorHandler: Ptr::from_mut(errorHandler),
            myToFill: Ptr::from_mut(toFill),
            myParentElements: Vec::new(),
            myLastNodeID: -1,
        }
    }

    /// C++: `void NodesHandler::~NodesHandler() noexcept` [virtual]
    /// Destructor
    // PCLoaderOSM::NodesHandler::~NodesHandler() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `void NodesHandler::myStartElement(int, const SUMOSAXAttributes &)` [virtual, protected]
    /// Called on the opening of a tag;
    // void
    // PCLoaderOSM::NodesHandler::myStartElement(int element, const SUMOSAXAttributes& attrs) {
    //     myParentElements.push_back(element);
    //     if (element == SUMO_TAG_NODE) {
    //         bool ok = true;
    //         long long int id = attrs.get<long long int>(SUMO_ATTR_ID, nullptr, ok);
    //         if (!ok) {
    //             return;
    //         }
    //         myLastNodeID = -1;
    //         if (myToFill.find(id) == myToFill.end()) {
    //             myLastNodeID = id;
    //             // assume we are loading multiple files...
    //             //  ... so we won't report duplicate nodes
    //             PCOSMNode* toAdd = new PCOSMNode();
    //             toAdd->id = id;
    //             toAdd->lon = attrs.get<double>(SUMO_ATTR_LON, toString(id).c_str(), ok);
    //             toAdd->lat = attrs.get<double>(SUMO_ATTR_LAT, toString(id).c_str(), ok);
    //             if (!ok) {
    //                 delete toAdd;
    //                 return;
    //             }
    //             myToFill[toAdd->id] = toAdd;
    //         }
    //     }
    //     if (element == SUMO_TAG_TAG && myParentElements.size() > 2 && myParentElements[myParentElements.size() - 2] == SUMO_TAG_NODE
    //             && myLastNodeID != -1) {
    //         bool ok = true;
    //         std::string key = attrs.getOpt<std::string>(SUMO_ATTR_K, toString(myLastNodeID).c_str(), ok, "", false);
    //         std::string value = attrs.getOpt<std::string>(SUMO_ATTR_V, toString(myLastNodeID).c_str(), ok, "", false);
    //         if (key == "name") {
    //             myToFill[myLastNodeID]->name = value;
    //         } else if (key == "") {
    //             myErrorHandler.inform("Empty key in a a tag while parsing node '" + toString(myLastNodeID) + "' occurred.");
    //             ok = false;
    //         }
    //         if (!ok) {
    //             return;
    //         }
    //         myToFill[myLastNodeID]->myAttributes[key] = value;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) {
        self.myParentElements.push(element);
        if element == SUMO_TAG_NODE {
            let mut ok = true;
            let id: i64 = attrs.get(SUMO_ATTR_ID, "", &mut ok, true);
            if !ok {
                return;
            }
            self.myLastNodeID = -1;
            if !self.myToFill.contains_key(&id) {
                self.myLastNodeID = id;
                // assume we are loading multiple files...
                //  ... so we won't report duplicate nodes
                let idStr = id.to_string();
                let lon: f64 = attrs.get(SUMO_ATTR_LON, &idStr, &mut ok, true);
                let lat: f64 = attrs.get(SUMO_ATTR_LAT, &idStr, &mut ok, true);
                if !ok {
                    return;
                }
                let toAdd = PCLoaderOSM_PCOSMNode { id, lon, lat, name: String::new(), myAttributes: BTreeMap::new() };
                self.myToFill.m().insert(id, leak_box(toAdd));
            }
        }
        if element == SUMO_TAG_TAG
            && self.myParentElements.len() > 2
            && self.myParentElements[self.myParentElements.len() - 2] == SUMO_TAG_NODE
            && self.myLastNodeID != -1
        {
            let mut ok = true;
            let lastID = self.myLastNodeID.to_string();
            let key: String = attrs.get_opt(SUMO_ATTR_K, &lastID, &mut ok, String::new(), false);
            let value: String = attrs.get_opt(SUMO_ATTR_V, &lastID, &mut ok, String::new(), false);
            if key == "name" {
                self.myToFill.m()[&self.myLastNodeID].m().name = value.clone();
            } else if key == "" {
                self.myErrorHandler.m().inform(&format!("Empty key in a a tag while parsing node '{}' occurred.", lastID), true);
                ok = false;
            }
            if !ok {
                return;
            }
            let nodeID = self.myLastNodeID;
            self.myToFill.m()[&nodeID].m().myAttributes.insert(key, value);
        }
    }

    /// C++: `void NodesHandler::myEndElement(int)` [virtual, protected]
    /// Called when a closing tag occurs
    // void
    // PCLoaderOSM::NodesHandler::myEndElement(int element) {
    //     if (element == SUMO_TAG_NODE) {
    //         myLastNodeID = -1;
    //     }
    //     myParentElements.pop_back();
    // }
    #[allow(unused_variables)]
    pub fn my_end_element(&mut self, element: i32) {
        if element == SUMO_TAG_NODE {
            self.myLastNodeID = -1;
        }
        self.myParentElements.pop();
    }

    /// C++: `void NodesHandler::NodesHandler(const NodesHandler &)` [private]
    /// Invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &PCLoaderOSM_NodesHandler) -> Self {
        panic!("PCLoaderOSM::NodesHandler copy constructor is private and undefined in C++ (invalidated)")
    }

    /// C++: `NodesHandler & NodesHandler::operator=(const NodesHandler &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &PCLoaderOSM_NodesHandler) -> &mut PCLoaderOSM_NodesHandler {
        panic!("PCLoaderOSM::NodesHandler assignment operator is private and undefined in C++ (invalidated)")
    }

}

impl SaxHandler for PCLoaderOSM_NodesHandler {
    fn handler(&mut self) -> &mut utils::xml::generic_sax_handler::GenericSAXHandler {
        &mut self.base
    }
    fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) -> Result<(), ProcessError> {
        PCLoaderOSM_NodesHandler::my_start_element(self, element, attrs);
        Ok(())
    }
    fn my_end_element(&mut self, element: i32) -> Result<(), ProcessError> {
        PCLoaderOSM_NodesHandler::my_end_element(self, element);
        Ok(())
    }
}

/// A reader of pois and polygons stored in OSM-format
/// C++ class `PCLoaderOSM` (polyconvert/PCLoaderOSM.h)
#[repr(C)]
pub struct PCLoaderOSM {
    /// C++ base class `SUMOSAXHandler`
    pub base: GenericSAXHandler,
}

impl std::ops::Deref for PCLoaderOSM {
    type Target = GenericSAXHandler;
    fn deref(&self) -> &GenericSAXHandler {
        &self.base
    }
}
impl std::ops::DerefMut for PCLoaderOSM {
    fn deref_mut(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
}

// C++ static members of PCLoaderOSM (port as module-level statics, see AGENTS.md):
//   static const std::set<std::string> PCLoaderOSM::MyKeysToInclude;
//   static bool PCLoaderOSM::myFullType;

impl PCLoaderOSM {
    /// C++: `void PCLoaderOSM::loadIfSet(OptionsCont &, PCPolyContainer &, PCTypeMap &)` [static]
    /// Loads pois/polygons assumed to be stored as OSM-XML
    // void
    // PCLoaderOSM::loadIfSet(OptionsCont& oc, PCPolyContainer& toFill, PCTypeMap& tm) {
    //     if (!oc.isSet("osm-files")) {
    //         return;
    //     }
    //     // parse file(s)
    //     std::vector<std::string> files = oc.getStringVector("osm-files");
    //     // load nodes, first
    //     std::map<long long int, PCOSMNode*> nodes;
    //     bool withAttributes = oc.getBool("all-attributes");
    //     myFullType = oc.getBool("osm.keep-full-type");
    //     MsgHandler* m = OptionsCont::getOptions().getBool("ignore-errors") ? MsgHandler::getWarningInstance() : MsgHandler::getErrorInstance();
    //     NodesHandler nodesHandler(nodes, withAttributes, *m);
    //     for (std::vector<std::string>::const_iterator file = files.begin(); file != files.end(); ++file) {
    //         // nodes
    //         if (!FileHelpers::isReadable(*file)) {
    //             WRITE_ERRORF(TL("Could not open osm-file '%'."), *file);
    //             return;
    //         }
    //         const long before = PROGRESS_BEGIN_TIME_MESSAGE("Parsing nodes from osm-file '" + *file + "'");
    //         if (!XMLSubSys::runParser(nodesHandler, *file)) {
    //             for (std::map<long long int, PCOSMNode*>::const_iterator i = nodes.begin(); i != nodes.end(); ++i) {
    //                 delete (*i).second;
    //             }
    //             throw ProcessError();
    //         }
    //         PROGRESS_TIME_MESSAGE(before);
    //     }
    //     // load relations to see which additional ways may be relevant
    //     Relations relations;
    //     RelationsMap additionalWays;
    //     std::set<long long int> innerEdges;
    //     RelationsHandler relationsHandler(additionalWays, relations, innerEdges, withAttributes, *m);
    //     for (std::vector<std::string>::const_iterator file = files.begin(); file != files.end(); ++file) {
    //         // edges
    //         const long before = PROGRESS_BEGIN_TIME_MESSAGE("Parsing relations from osm-file '" + *file + "'");
    //         XMLSubSys::runParser(relationsHandler, *file);
    //         PROGRESS_TIME_MESSAGE(before);
    //     }
    //
    //     // load ways
    //     EdgeMap edges;
    //     EdgesHandler edgesHandler(nodes, edges, additionalWays, withAttributes, *m);
    //     for (std::vector<std::string>::const_iterator file = files.begin(); file != files.end(); ++file) {
    //         // edges
    //         const long before = PROGRESS_BEGIN_TIME_MESSAGE("Parsing edges from osm-file '" + *file + "'");
    //         XMLSubSys::runParser(edgesHandler, *file);
    //         PROGRESS_TIME_MESSAGE(before);
    //     }
    //
    //     // build all
    //     const bool useName = oc.getBool("osm.use-name");
    //     const double mergeRelationsThreshold = OptionsCont::getOptions().getFloat("osm.merge-relations");
    //     // create polygons from relations
    //     if (mergeRelationsThreshold >= 0) {
    //         for (PCOSMRelation* rel : relations) {
    //             if (!rel->keep || rel->myWays.empty()) {
    //                 continue;
    //             }
    //             // filter unknown and empty ways
    //             int numNodes = 0;
    //             for (auto it = rel->myWays.begin(); it != rel->myWays.end();) {
    //                 if (edges.count(*it) == 0 || edges[*it]->myCurrentNodes.empty()) {
    //                     it = rel->myWays.erase(it);
    //                 } else if (innerEdges.count(*it) > 0) {
    //                     // @note: it would be a good idea to merge inner shapes but
    //                     // it's difficult since there may be more than one
    //                     // independent inner shape
    //                     edges[*it]->standalone = true;
    //                     it = rel->myWays.erase(it);
    //                 } else {
    //                     numNodes += (int)edges[*it]->myCurrentNodes.size();
    //                     it++;
    //                 }
    //             }
    //             if (numNodes == 0) {
    //                 WRITE_WARNINGF(TL("Could not import polygon from relation '%' (missing ways)"), rel->id);
    //                 continue;
    //             }
    //             PCOSMEdge* e = new PCOSMEdge();
    //             e->id = rel->id;
    //             e->name = rel->name;
    //             e->myAttributes = rel->myAttributes;
    //             e->myIsClosed = false;
    //             e->standalone = true;
    //
    //             std::vector<std::vector<long long int> > snippets;
    //             for (const long long int wayID : rel->myWays) {
    //                 PCOSMEdge* edge = edges[wayID];
    //                 snippets.push_back(edge->myCurrentNodes);
    //             }
    //             double maxDist = 0.;
    //             bool ok = true;
    //             while (snippets.size() > 1) {
    //                 maxDist = MAX2(maxDist, mergeClosest(nodes, snippets));
    //                 if (maxDist > mergeRelationsThreshold) {
    //                     ok = false;
    //                     break;
    //                 }
    //             }
    //             if (ok) {
    //                 e->myCurrentNodes = snippets.front();
    //                 edges[e->id] = e;
    //                 double frontBackDist = 0;
    //                 if (e->myCurrentNodes.front() != e->myCurrentNodes.back()) {
    //                     // should be filled
    //                     const Position posFront = convertNodePosition(nodes[e->myCurrentNodes.front()]);
    //                     const Position posBack = convertNodePosition(nodes[e->myCurrentNodes.back()]);
    //                     frontBackDist = posFront.distanceTo2D(posBack);
    //                     if (frontBackDist < mergeRelationsThreshold) {
    //                         e->myCurrentNodes.push_back(e->myCurrentNodes.front());
    //                         frontBackDist = 0;
    //                     }
    //                 }
    //                 std::string frontBackMsg = "";
    //                 if (frontBackDist > 0) {
    //                     frontBackMsg = TLF(", (front-to-back dist: %)", frontBackDist);
    //                 }
    //                 WRITE_MESSAGEF(TL("Assembled polygon from relation '%' (name:%)%"), toString(rel->id), e->name, frontBackMsg);
    //             } else {
    //                 WRITE_WARNINGF(TL("Could not import polygon from relation '%' (name:% reason: found gap of %m)."), rel->id, rel->name, maxDist)
    //                 delete e;
    //                 // export ways by themselves
    //                 for (long long int wayID : rel->myWays) {
    //                     PCOSMEdge* part = edges[wayID];
    //                     part->standalone = true;
    //                 }
    //             }
    //         }
    //     }
    //
    //     // instantiate polygons
    //     for (EdgeMap::iterator i = edges.begin(); i != edges.end(); ++i) {
    //         PCOSMEdge* e = (*i).second;
    //         if (e->myAttributes.size() == 0) {
    //             // cannot be relevant as a polygon
    //             continue;
    //         }
    //         if (!e->standalone && mergeRelationsThreshold >= 0) {
    //             // part of a relation
    //             continue;
    //         }
    //         if (e->myCurrentNodes.size() == 0) {
    //             WRITE_WARNINGF(TL("Polygon '%' has no shape."), toString(e->id));
    //             continue;
    //         }
    //         // compute shape
    //         PositionVector vec;
    //         for (std::vector<long long int>::iterator j = e->myCurrentNodes.begin(); j != e->myCurrentNodes.end(); ++j) {
    //             PCOSMNode* n = nodes.find(*j)->second;
    //             Position pos(n->lon, n->lat);
    //             if (!GeoConvHelper::getProcessing().x2cartesian(pos)) {
    //                 WRITE_WARNINGF(TL("Unable to project coordinates for polygon '%'."), e->id);
    //             }
    //             vec.push_back_noDoublePos(pos);
    //         }
    //         const bool ignorePruning = OptionsCont::getOptions().isInStringVector("prune.keep-list", toString(e->id));
    //         // add as many polygons as keys match defined types
    //         int index = 0;
    //         std::string unknownPolyType = "";
    //         bool isInner = mergeRelationsThreshold >= 0 && innerEdges.count(e->id) != 0 && tm.has("inner");
    //         for (std::map<std::string, std::string>::iterator it = e->myAttributes.begin(); it != e->myAttributes.end(); ++it) {
    //             const std::string& key = it->first;
    //             const std::string& value = it->second;
    //             const std::string fullType = key + "." + value;
    //             if (tm.has(key + "." + value)) {
    //                 auto def = tm.get(isInner ? "inner" : fullType);
    //                 index = addPolygon(e, vec, def, fullType, index, useName, toFill, ignorePruning, withAttributes);
    //             } else if (tm.has(key)) {
    //                 auto def = tm.get(isInner ? "inner" : key);
    //                 index = addPolygon(e, vec, def, fullType, index, useName, toFill, ignorePruning, withAttributes);
    //             } else if (MyKeysToInclude.count(key) > 0) {
    //                 unknownPolyType = fullType;
    //             }
    //         }
    //         const PCTypeMap::TypeDef& def = tm.getDefault();
    //         if (index == 0 && !def.discard && unknownPolyType != "") {
    //             addPolygon(e, vec, def, unknownPolyType, index, useName, toFill, ignorePruning, withAttributes);
    //         }
    //     }
    //
    //
    //     // instantiate pois
    //     for (std::map<long long int, PCOSMNode*>::iterator i = nodes.begin(); i != nodes.end(); ++i) {
    //         PCOSMNode* n = (*i).second;
    //         if (n->myAttributes.size() == 0) {
    //             // cannot be relevant as a poi
    //             continue;
    //         }
    //         Position pos(n->lon, n->lat);
    //         if (!GeoConvHelper::getProcessing().x2cartesian(pos)) {
    //             WRITE_WARNINGF(TL("Unable to project coordinates for POI '%'."), n->id);
    //         }
    //         const bool ignorePruning = OptionsCont::getOptions().isInStringVector("prune.keep-list", toString(n->id));
    //         // add as many POIs as keys match defined types
    //         int index = 0;
    //         std::string unKnownPOIType = "";
    //         for (std::map<std::string, std::string>::iterator it = n->myAttributes.begin(); it != n->myAttributes.end(); ++it) {
    //             const std::string& key = it->first;
    //             const std::string& value = it->second;
    //             const std::string fullType = key + "." + value;
    //             if (tm.has(key + "." + value)) {
    //                 index = addPOI(n, pos, tm.get(fullType), fullType, index, useName, toFill, ignorePruning, withAttributes);
    //             } else if (tm.has(key)) {
    //                 index = addPOI(n, pos, tm.get(key), fullType, index, useName, toFill, ignorePruning, withAttributes);
    //             } else if (MyKeysToInclude.count(key) > 0) {
    //                 unKnownPOIType = fullType;
    //             }
    //         }
    //         const PCTypeMap::TypeDef& def = tm.getDefault();
    //         if (index == 0 && !def.discard && unKnownPOIType != "") {
    //             addPOI(n, pos, def, unKnownPOIType, index, useName, toFill, ignorePruning, withAttributes);
    //         }
    //     }
    //     // delete nodes
    //     for (std::map<long long int, PCOSMNode*>::const_iterator i = nodes.begin(); i != nodes.end(); ++i) {
    //         delete (*i).second;
    //     }
    //     // delete edges
    //     for (EdgeMap::iterator i = edges.begin(); i != edges.end(); ++i) {
    //         delete (*i).second;
    //     }
    //     // delete relations
    //     for (Relations::iterator i = relations.begin(); i != relations.end(); ++i) {
    //         delete (*i);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn load_if_set(oc: &mut OptionsCont, toFill: &mut PCPolyContainer, tm: &mut PCTypeMap) {
        if !or_panic(oc.is_set("osm-files", true)) {
            return;
        }
        // parse file(s)
        let files: Vec<String> = or_panic(oc.get_string_vector("osm-files")).clone();
        // load nodes, first
        let mut nodes: BTreeMap<i64, Ptr<PCLoaderOSM_PCOSMNode>> = BTreeMap::new();
        let withAttributes = or_panic(oc.get_bool("all-attributes"));
        MY_FULL_TYPE.store(or_panic(oc.get_bool("osm.keep-full-type")), Ordering::Relaxed);
        let m: Ptr<MsgHandler> = if get_bool("ignore-errors") {
            with_instance(MsgType::Warning, |h| Ptr::from_mut(h))
        } else {
            with_instance(MsgType::Error, |h| Ptr::from_mut(h))
        };
        let mut nodesHandler = PCLoaderOSM_NodesHandler::new(&mut nodes, withAttributes, m.m());
        for file in files.iter() {
            // nodes
            if !is_readable(file) {
                write_errorf!("Could not open osm-file '%'.", file);
                return;
            }
            let before = current_millis();
            progress_begin_message(&format!("Parsing nodes from osm-file '{}'", file));
            if !or_panic(run_parser(&mut nodesHandler, file, false, false, false, true)) {
                for n in nodes.values().copied() {
                    free_box(n);
                }
                panic!("");
            }
            progress_time_message(current_millis() - before);
        }

        // load relations to see which additional ways may be relevant
        let mut relations: PCLoaderOSM_Relations = Vec::new();
        let mut additionalWays: PCLoaderOSM_RelationsMap = BTreeMap::new();
        let mut innerEdges: BTreeSet<i64> = BTreeSet::new();
        let mut relationsHandler = PCLoaderOSM_RelationsHandler::new(&mut additionalWays, &mut relations, &mut innerEdges, withAttributes, m.m());
        for file in files.iter() {
            // edges
            let before = current_millis();
            progress_begin_message(&format!("Parsing relations from osm-file '{}'", file));
            let _ = run_parser(&mut relationsHandler, file, false, false, false, true);
            progress_time_message(current_millis() - before);
        }

        // load ways
        let mut edges: PCLoaderOSM_EdgeMap = BTreeMap::new();
        let mut edgesHandler = PCLoaderOSM_EdgesHandler::new(&nodes, &mut edges, &additionalWays, withAttributes, m.m());
        for file in files.iter() {
            // edges
            let before = current_millis();
            progress_begin_message(&format!("Parsing edges from osm-file '{}'", file));
            let _ = run_parser(&mut edgesHandler, file, false, false, false, true);
            progress_time_message(current_millis() - before);
        }

        // build all
        let useName = or_panic(oc.get_bool("osm.use-name"));
        let mergeRelationsThreshold = get_float("osm.merge-relations");
        // create polygons from relations
        if mergeRelationsThreshold >= 0.0 {
            for rel in relations.iter().copied().collect::<Vec<_>>() {
                if !rel.keep || rel.myWays.is_empty() {
                    continue;
                }
                // filter unknown and empty ways
                let mut numNodes: i32 = 0;
                let mut kept: Vec<i64> = Vec::new();
                for w in rel.myWays.iter().copied() {
                    if !edges.contains_key(&w) || edges[&w].myCurrentNodes.is_empty() {
                        continue;
                    } else if innerEdges.contains(&w) {
                        // @note: it would be a good idea to merge inner shapes but
                        // it's difficult since there may be more than one
                        // independent inner shape
                        edges[&w].m().standalone = true;
                    } else {
                        numNodes += edges[&w].myCurrentNodes.len() as i32;
                        kept.push(w);
                    }
                }
                rel.m().myWays = kept;
                if numNodes == 0 {
                    write_warningf!("Could not import polygon from relation '%' (missing ways)", rel.id);
                    continue;
                }
                let e = leak_box(PCLoaderOSM_PCOSMEdge {
                    id: rel.id,
                    name: rel.name.clone(),
                    myIsClosed: false,
                    myCurrentNodes: Vec::new(),
                    myAttributes: rel.myAttributes.clone(),
                    standalone: true,
                });

                let mut snippets: Vec<Vec<i64>> = Vec::new();
                for wayID in rel.myWays.iter().copied() {
                    let edge = edges[&wayID];
                    snippets.push(edge.myCurrentNodes.clone());
                }
                let mut maxDist: f64 = 0.0;
                let mut ok = true;
                while snippets.len() > 1 {
                    let d = PCLoaderOSM::merge_closest(&nodes, &mut snippets);
                    maxDist = if maxDist > d { maxDist } else { d };
                    if maxDist > mergeRelationsThreshold {
                        ok = false;
                        break;
                    }
                }
                if ok {
                    e.m().myCurrentNodes = snippets[0].clone();
                    let eid = e.id;
                    edges.insert(eid, e);
                    let mut frontBackDist: f64 = 0.0;
                    let first = e.myCurrentNodes[0];
                    let last = *e.myCurrentNodes.last().unwrap();
                    if first != last {
                        // should be filled
                        let posFront = PCLoaderOSM::convert_node_position(nodes[&first]);
                        let posBack = PCLoaderOSM::convert_node_position(nodes[&last]);
                        frontBackDist = posFront.distance_to2d(&posBack);
                        if frontBackDist < mergeRelationsThreshold {
                            e.m().myCurrentNodes.push(first);
                            frontBackDist = 0.0;
                        }
                    }
                    let frontBackMsg = if frontBackDist > 0.0 { utils::tlf!(", (front-to-back dist: %)", frontBackDist) } else { String::new() };
                    write_messagef!("Assembled polygon from relation '%' (name:%)%", rel.id.to_string(), e.name, frontBackMsg);
                } else {
                    write_warningf!("Could not import polygon from relation '%' (name:% reason: found gap of %m).", rel.id, rel.name, maxDist);
                    free_box(e);
                    // export ways by themselves
                    for wayID in rel.myWays.iter().copied() {
                        let part = edges[&wayID];
                        part.m().standalone = true;
                    }
                }
            }
        }

        // instantiate polygons
        for e in edges.values().copied().collect::<Vec<_>>() {
            if e.myAttributes.is_empty() {
                // cannot be relevant as a polygon
                continue;
            }
            if !e.standalone && mergeRelationsThreshold >= 0.0 {
                // part of a relation
                continue;
            }
            if e.myCurrentNodes.is_empty() {
                write_warningf!("Polygon '%' has no shape.", e.id.to_string());
                continue;
            }
            // compute shape
            let mut vec = PositionVector::default();
            for j in e.myCurrentNodes.iter().copied() {
                let n = nodes[&j];
                let mut pos = Position::new(n.lon, n.lat);
                if !project(&mut pos) {
                    write_warningf!("Unable to project coordinates for polygon '%'.", e.id.to_string());
                }
                vec.push_back_no_double_pos(&pos);
            }
            let ignorePruning = or_panic(oc.is_in_string_vector("prune.keep-list", &e.id.to_string()));

            // add as many polygons as keys match defined types
            let mut index: i32 = 0;
            let mut unknownPolyType = String::new();
            let isInner = mergeRelationsThreshold >= 0.0 && innerEdges.contains(&e.id) && tm.has("inner");
            for (key, value) in e.myAttributes.iter() {
                let fullType = format!("{}.{}", key, value);
                if tm.has(&fullType) {
                    let def = tm.get(if isInner { "inner" } else { fullType.as_str() });
                    index = PCLoaderOSM::add_polygon(e, &vec, def, &fullType, index, useName, toFill, ignorePruning, withAttributes);
                } else if tm.has(key) {
                    let def = tm.get(if isInner { "inner" } else { key.as_str() });
                    index = PCLoaderOSM::add_polygon(e, &vec, def, &fullType, index, useName, toFill, ignorePruning, withAttributes);
                } else if MY_KEYS_TO_INCLUDE.contains(key) {
                    unknownPolyType = fullType;
                }
            }
            let def = tm.get_default();
            if index == 0 && !def.discard && unknownPolyType != "" {
                PCLoaderOSM::add_polygon(e, &vec, def, &unknownPolyType, index, useName, toFill, ignorePruning, withAttributes);
            }
        }

        // instantiate pois
        for n in nodes.values().copied().collect::<Vec<_>>() {
            if n.myAttributes.is_empty() {
                // cannot be relevant as a poi
                continue;
            }
            let mut pos = Position::new(n.lon, n.lat);
            if !project(&mut pos) {
                write_warningf!("Unable to project coordinates for POI '%'.", n.id.to_string());
            }
            let ignorePruning = or_panic(oc.is_in_string_vector("prune.keep-list", &n.id.to_string()));

            // add as many POIs as keys match defined types
            let mut index: i32 = 0;
            let mut unKnownPOIType = String::new();
            for (key, value) in n.myAttributes.iter() {
                let fullType = format!("{}.{}", key, value);
                if tm.has(&fullType) {
                    let def = tm.get(&fullType);
                    index = PCLoaderOSM::add_poi(n, &pos, def, &fullType, index, useName, toFill, ignorePruning, withAttributes);
                } else if tm.has(key) {
                    let def = tm.get(key);
                    index = PCLoaderOSM::add_poi(n, &pos, def, &fullType, index, useName, toFill, ignorePruning, withAttributes);
                } else if MY_KEYS_TO_INCLUDE.contains(key) {
                    unKnownPOIType = fullType;
                }
            }
            let def = tm.get_default();
            if index == 0 && !def.discard && unKnownPOIType != "" {
                PCLoaderOSM::add_poi(n, &pos, def, &unKnownPOIType, index, useName, toFill, ignorePruning, withAttributes);
            }
        }

        // delete nodes
        for n in nodes.values().copied() {
            free_box(n);
        }
        // delete edges
        for e in edges.values().copied() {
            free_box(e);
        }
        // delete relations
        for r in relations.iter().copied() {
            free_box(r);
        }
    }

    /// C++: `int PCLoaderOSM::addPolygon(const PCOSMEdge *, const PositionVector &, const PCTypeMap::TypeDef &, const std::string &, int, bool, PCPolyContainer &, bool, bool)` [static, protected]
    /// try add the polygon and return the next index on success
    // int
    // PCLoaderOSM::addPolygon(const PCOSMEdge* edge, const PositionVector& vec, const PCTypeMap::TypeDef& def, const std::string& fullType, int index, bool useName, PCPolyContainer& toFill, bool ignorePruning, bool withAttributes) {
    //     if (def.discard) {
    //         return index;
    //     } else {
    //         const bool closedShape = vec.front() == vec.back();
    //         const std::string idSuffix = (index == 0 ? "" : "#" + toString(index));
    //         const std::string id = def.prefix + (useName && edge->name != "" ? edge->name : toString(edge->id)) + idSuffix;
    //         bool fill = def.allowFill == PCTypeMap::Filltype::FORCE || (def.allowFill == PCTypeMap::Filltype::FILL && closedShape);
    //         SUMOPolygon* poly = new SUMOPolygon(
    //             StringUtils::escapeXML(id),
    //             StringUtils::escapeXML(myFullType ? fullType : def.id),
    //             def.color, vec, false, fill, 1, def.layer,
    //             SUMOPolygon::DEFAULT_ANGLE,
    //             SUMOPolygon::DEFAULT_IMG_FILE,
    //             SUMOPolygon::DEFAULT_NAME,
    //             parseHeight(edge->myAttributes, edge->id));
    //         if (withAttributes) {
    //             poly->updateParameters(edge->myAttributes);
    //         }
    //         if (!toFill.add(poly, ignorePruning)) {
    //             return index;
    //         } else {
    //             return index + 1;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_polygon(edge: Ptr<PCLoaderOSM_PCOSMEdge>, vec: &PositionVector, def: &PCTypeMap_TypeDef, fullType: &str, index: i32, useName: bool, toFill: &mut PCPolyContainer, ignorePruning: bool, withAttributes: bool) -> i32 {
        if def.discard {
            return index;
        }
        let closedShape = vec.front() == vec.back();
        let idSuffix = if index == 0 { String::new() } else { format!("#{}", index) };
        let baseName = if useName && edge.name != "" { edge.name.clone() } else { edge.id.to_string() };
        let id = format!("{}{}{}", def.prefix, baseName, idSuffix);
        let fill = matches!(def.allowFill, PCTypeMap_Filltype::FORCE) || (matches!(def.allowFill, PCTypeMap_Filltype::FILL) && closedShape);
        let typeName = if MY_FULL_TYPE.load(Ordering::Relaxed) { fullType } else { def.id.as_str() };
        let empty: BTreeMap<String, String> = BTreeMap::new();
        let poly = leak_box(SUMOPolygon::new(
            &escape_xml(&id, false),
            &escape_xml(typeName, false),
            &def.color,
            vec,
            false,
            fill,
            1.0,
            def.layer,
            DEFAULT_ANGLE,
            DEFAULT_IMG_FILE,
            DEFAULT_NAME,
            PCLoaderOSM::parse_height(&edge.myAttributes, edge.id),
            &empty,
        ));
        if withAttributes {
            poly.m().base2.update_parameters(&edge.myAttributes);
        }
        if !toFill.add(poly, ignorePruning) {
            return index;
        } else {
            return index + 1;
        }
    }

    /// C++: `int PCLoaderOSM::addPOI(const PCOSMNode *, const Position &, const PCTypeMap::TypeDef &, const std::string &, int, bool, PCPolyContainer &, bool, bool)` [static, protected]
    /// try add the POI and return the next index on success
    // int
    // PCLoaderOSM::addPOI(const PCOSMNode* node, const Position& pos, const PCTypeMap::TypeDef& def, const std::string& fullType,
    //                     int index, bool useName, PCPolyContainer& toFill, bool ignorePruning, bool withAttributes) {
    //     if (def.discard) {
    //         return index;
    //     } else {
    //         const std::string idSuffix = (index == 0 ? "" : "#" + toString(index));
    //         const std::string id = def.prefix + (useName && node->name != "" ? node->name : toString(node->id)) + idSuffix;
    //         PointOfInterest* poi = new PointOfInterest(
    //             StringUtils::escapeXML(id),
    //             StringUtils::escapeXML(myFullType ? fullType : def.id),
    //             def.color, pos, false, "", 0, false, 0, def.icon, def.layer);
    //         if (withAttributes) {
    //             poi->updateParameters(node->myAttributes);
    //         }
    //         if (!toFill.add(poi, ignorePruning)) {
    //             return index;
    //         } else {
    //             return index + 1;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_poi(node: Ptr<PCLoaderOSM_PCOSMNode>, pos: &Position, def: &PCTypeMap_TypeDef, fullType: &str, index: i32, useName: bool, toFill: &mut PCPolyContainer, ignorePruning: bool, withAttributes: bool) -> i32 {
        if def.discard {
            return index;
        }
        let idSuffix = if index == 0 { String::new() } else { format!("#{}", index) };
        let baseName = if useName && node.name != "" { node.name.clone() } else { node.id.to_string() };
        let id = format!("{}{}{}", def.prefix, baseName, idSuffix);
        let typeName = if MY_FULL_TYPE.load(Ordering::Relaxed) { fullType } else { def.id.as_str() };
        let empty: BTreeMap<String, String> = BTreeMap::new();
        let poi = leak_box(PointOfInterest::new(
            &escape_xml(&id, false),
            &escape_xml(typeName, false),
            &def.color,
            pos,
            false,
            "",
            0.0,
            false,
            0.0,
            &def.icon,
            def.layer,
            DEFAULT_ANGLE,
            DEFAULT_IMG_FILE,
            DEFAULT_IMG_WIDTH,
            DEFAULT_IMG_HEIGHT,
            DEFAULT_NAME,
            &empty,
        ));
        if withAttributes {
            poi.m().base3.update_parameters(&node.myAttributes);
        }
        if !toFill.add_2(poi, ignorePruning) {
            return index;
        } else {
            return index + 1;
        }
    }

    /// C++: `std::set<std::string> PCLoaderOSM::initMyKeysToInclude()` [static, private]
    // std::set<std::string> PCLoaderOSM::initMyKeysToInclude() {
    //     std::set<std::string> result;
    //     result.insert("highway");
    //     result.insert("railway");
    //     result.insert("railway:position");
    //     result.insert("railway:position:exact");
    //     result.insert("waterway");
    //     result.insert("aeroway");
    //     result.insert("aerialway");
    //     result.insert("power");
    //     result.insert("man_made");
    //     result.insert("building");
    //     result.insert("leisure");
    //     result.insert("amenity");
    //     result.insert("shop");
    //     result.insert("tourism");
    //     result.insert("historic");
    //     result.insert("landuse");
    //     result.insert("natural");
    //     result.insert("military");
    //     result.insert("boundary");
    //     result.insert("admin_level");
    //     result.insert("sport");
    //     result.insert("polygon");
    //     result.insert("place");
    //     result.insert("population");
    //     result.insert("barrier");
    //     result.insert("openGeoDB:population");
    //     result.insert("openGeoDB:name");
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn init_my_keys_to_include() -> BTreeSet<String> {
        let mut result: BTreeSet<String> = BTreeSet::new();
        for key in [
            "highway",
            "railway",
            "railway:position",
            "railway:position:exact",
            "waterway",
            "aeroway",
            "aerialway",
            "power",
            "man_made",
            "building",
            "leisure",
            "amenity",
            "shop",
            "tourism",
            "historic",
            "landuse",
            "natural",
            "military",
            "boundary",
            "admin_level",
            "sport",
            "polygon",
            "place",
            "population",
            "barrier",
            "openGeoDB:population",
            "openGeoDB:name",
        ] {
            result.insert(key.to_string());
        }
        result
    }

    /// C++: `Position PCLoaderOSM::convertNodePosition(PCOSMNode *)` [static, private]
    /// retrieve cartesian coordinate for given node
    // Position
    // PCLoaderOSM::convertNodePosition(PCOSMNode* n) {
    //     Position pos(n->lon, n->lat);
    //     GeoConvHelper::getProcessing().x2cartesian(pos);
    //     return pos;
    // }
    #[allow(unused_variables)]
    pub fn convert_node_position(n: Ptr<PCLoaderOSM_PCOSMNode>) -> Position {
        let mut pos = Position::new(n.lon, n.lat);
        project(&mut pos);
        pos
    }

    /// C++: `double PCLoaderOSM::mergeClosest(const std::map<long long, PCOSMNode *> &, std::vector<std::vector<long long>> &)` [static, private]
    // double
    // PCLoaderOSM::mergeClosest(const std::map<long long int, PCOSMNode*>& nodes, std::vector<std::vector<long long int> >& snippets) {
    //     double best = std::numeric_limits<double>::max();
    //     int best_i = 0;
    //     int best_j = 1;
    //     bool iFW = true;
    //     bool jFW = true;
    //
    //     for (int i = 0; i < (int)snippets.size(); i++) {
    //         for (int j = i + 1; j < (int)snippets.size(); j++) {
    //             Position front1(convertNodePosition(nodes.find(snippets[i].front())->second));
    //             Position back1(convertNodePosition(nodes.find(snippets[i].back())->second));
    //             Position front2(convertNodePosition(nodes.find(snippets[j].front())->second));
    //             Position back2(convertNodePosition(nodes.find(snippets[j].back())->second));
    //             double dist1 = front1.distanceTo2D(front2);
    //             double dist2 = front1.distanceTo2D(back2);
    //             double dist3 = back1.distanceTo2D(front2);
    //             double dist4 = back1.distanceTo2D(back2);
    //             if (dist1 < best) {
    //                 best = dist1;
    //                 best_i = i;
    //                 best_j = j;
    //                 iFW = false;
    //                 jFW = true;
    //             }
    //             if (dist2 < best) {
    //                 best = dist2;
    //                 best_i = i;
    //                 best_j = j;
    //                 iFW = false;
    //                 jFW = false;
    //             }
    //             if (dist3 < best) {
    //                 best = dist3;
    //                 best_i = i;
    //                 best_j = j;
    //                 iFW = true;
    //                 jFW = true;
    //             }
    //             if (dist4 < best) {
    //                 best = dist4;
    //                 best_i = i;
    //                 best_j = j;
    //                 iFW = true;
    //                 jFW = false;
    //             }
    //         }
    //     }
    //     std::vector<long long int> merged;
    //     if (iFW) {
    //         merged.insert(merged.end(), snippets[best_i].begin(), snippets[best_i].end());
    //     } else {
    //         merged.insert(merged.end(), snippets[best_i].rbegin(), snippets[best_i].rend());
    //     }
    //     if (jFW) {
    //         merged.insert(merged.end(), snippets[best_j].begin(), snippets[best_j].end());
    //     } else {
    //         merged.insert(merged.end(), snippets[best_j].rbegin(), snippets[best_j].rend());
    //     }
    //     snippets.erase(snippets.begin() + best_j);
    //     snippets.erase(snippets.begin() + best_i);
    //     snippets.push_back(merged);
    //     return best;
    // }
    #[allow(unused_variables)]
    pub fn merge_closest(nodes: &BTreeMap<i64, Ptr<PCLoaderOSM_PCOSMNode>>, snippets: &mut Vec<Vec<i64>>) -> f64 {
        let mut best: f64 = f64::MAX;
        let mut best_i: usize = 0;
        let mut best_j: usize = 1;
        let mut iFW = true;
        let mut jFW = true;

        for i in 0..snippets.len() {
            for j in (i + 1)..snippets.len() {
                let front1 = Self::convert_node_position(nodes[snippets[i].first().unwrap()]);
                let back1 = Self::convert_node_position(nodes[snippets[i].last().unwrap()]);
                let front2 = Self::convert_node_position(nodes[snippets[j].first().unwrap()]);
                let back2 = Self::convert_node_position(nodes[snippets[j].last().unwrap()]);
                let dist1 = front1.distance_to2d(&front2);
                let dist2 = front1.distance_to2d(&back2);
                let dist3 = back1.distance_to2d(&front2);
                let dist4 = back1.distance_to2d(&back2);
                if dist1 < best {
                    best = dist1;
                    best_i = i;
                    best_j = j;
                    iFW = false;
                    jFW = true;
                }
                if dist2 < best {
                    best = dist2;
                    best_i = i;
                    best_j = j;
                    iFW = false;
                    jFW = false;
                }
                if dist3 < best {
                    best = dist3;
                    best_i = i;
                    best_j = j;
                    iFW = true;
                    jFW = true;
                }
                if dist4 < best {
                    best = dist4;
                    best_i = i;
                    best_j = j;
                    iFW = true;
                    jFW = false;
                }
            }
        }
        let mut merged: Vec<i64> = Vec::new();
        if iFW {
            merged.extend(snippets[best_i].iter().copied());
        } else {
            merged.extend(snippets[best_i].iter().rev().copied());
        }
        if jFW {
            merged.extend(snippets[best_j].iter().copied());
        } else {
            merged.extend(snippets[best_j].iter().rev().copied());
        }
        snippets.remove(best_j);
        snippets.remove(best_i);
        snippets.push(merged);
        best
    }

    /// C++: `double PCLoaderOSM::parseHeight(const std::map<std::string, std::string> &, long long)` [static, private]
    // double
    // PCLoaderOSM::parseHeight(const std::map<std::string, std::string>& attrs, long long int id) {
    //     auto itH = attrs.find("height");
    //     if (itH != attrs.end()) {
    //         try {
    //             return StringUtils::parseDist(itH->second);
    //         } catch (...) {
    //             WRITE_WARNINGF(TL("Value of key '%' is not numeric ('%') in way '%'."), "height", itH->second, id);
    //         }
    //     }
    //     return SUMOPolygon::DEFAULT_HEIGHT;
    // }
    #[allow(unused_variables)]
    pub fn parse_height(attrs: &BTreeMap<String, String>, id: i64) -> f64 {
        if let Some(itH) = attrs.get("height") {
            match parse_dist(itH) {
                Ok(v) => return v,
                Err(_) => {
                    write_warningf!("Value of key '%' is not numeric ('%') in way '%'.", "height", itH.as_str(), id);
                }
            }
        }
        DEFAULT_HEIGHT
    }

}

/// An internal definition of a loaded edge
/// C++ class `PCLoaderOSM::PCOSMEdge` (polyconvert/PCLoaderOSM.h)
pub struct PCLoaderOSM_PCOSMEdge {
    /// The edge's id
    pub id: i64,
    /// The edge's name (if any)
    pub name: String,
    /// Information whether this area is closed
    pub myIsClosed: bool,
    /// The list of nodes this edge is made of
    pub myCurrentNodes: Vec<i64>,
    /// Additional attributes
    pub myAttributes: BTreeMap<String, String>,
    pub standalone: bool,
}

impl PCLoaderOSM_PCOSMEdge {
}

/// An internal representation of an OSM-node
/// C++ class `PCLoaderOSM::PCOSMNode` (polyconvert/PCLoaderOSM.h)
pub struct PCLoaderOSM_PCOSMNode {
    /// The node's id
    pub id: i64,
    /// The longitude the node is located at
    pub lon: f64,
    /// The latitude the node is located at
    pub lat: f64,
    /// The nodes name (if any)
    pub name: String,
    /// Additional attributes
    pub myAttributes: BTreeMap<String, String>,
}

impl PCLoaderOSM_PCOSMNode {
}

/// An internal definition of a loaded relation
/// C++ class `PCLoaderOSM::PCOSMRelation` (polyconvert/PCLoaderOSM.h)
pub struct PCLoaderOSM_PCOSMRelation {
    /// The relation's id
    pub id: i64,
    /// The relation's name (if any)
    pub name: String,
    /// The list of ways this relation is made of
    pub myWays: Vec<i64>,
    /// Additional attributes
    pub myAttributes: BTreeMap<String, String>,
    /// whether this relation is a valid polygon
    pub keep: bool,
}

impl PCLoaderOSM_PCOSMRelation {
}

/// A class which extracts relevant way-ids from relations in a parsed OSM-file
/// C++ class `PCLoaderOSM::RelationsHandler` (polyconvert/PCLoaderOSM.h)
#[repr(C)]
pub struct PCLoaderOSM_RelationsHandler {
    /// C++ base class `SUMOSAXHandler`
    pub base: GenericSAXHandler,
    /// additional ways which are reference by relations
    pub myAdditionalWays: Ptr<PCLoaderOSM_RelationsMap>,
    /// the loaded relations
    pub myRelations: Ptr<PCLoaderOSM_Relations>,
    /// the loaded edges
    pub myInnerEdges: Ptr<BTreeSet<i64>>,
    /// Whether all attributes shall be stored
    pub myWithAttributes: bool,
    /// The handler to report errors to (will be the WarningsHandler if --ignore-errors was set)
    pub myErrorHandler: Ptr<MsgHandler>,
    /// The currently parsed relation
    pub myCurrentRelation: Ptr<PCLoaderOSM_PCOSMRelation>,
    /// the ways within the current relation
    pub myCurrentWays: Vec<i64>,
    /// Current path in order to know to what occuring values belong
    pub myParentElements: Vec<i64>,
    /// whether the last edge (way) should be kept because it had a key from the inclusion list
    pub myKeep: bool,
}

impl std::ops::Deref for PCLoaderOSM_RelationsHandler {
    type Target = GenericSAXHandler;
    fn deref(&self) -> &GenericSAXHandler {
        &self.base
    }
}
impl std::ops::DerefMut for PCLoaderOSM_RelationsHandler {
    fn deref_mut(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
}

impl PCLoaderOSM_RelationsHandler {
    /// C++: `void RelationsHandler::RelationsHandler(RelationsMap &, Relations &, std::set<long long> &, bool, MsgHandler &)`
    /// Constructor
    // PCLoaderOSM::RelationsHandler::RelationsHandler(RelationsMap& additionalWays,
    //         Relations& relations,
    //         std::set<long long int>& innerEdges,
    //         bool withAttributes,
    //         MsgHandler& errorHandler) :
    //     SUMOSAXHandler("osm - file"),
    //     myAdditionalWays(additionalWays),
    //     myRelations(relations),
    //     myInnerEdges(innerEdges),
    //     myWithAttributes(withAttributes),
    //     myErrorHandler(errorHandler),
    //     myCurrentRelation(nullptr) {
    // }
    #[allow(unused_variables)]
    pub fn new(additionalWays: &mut PCLoaderOSM_RelationsMap, relations: &mut PCLoaderOSM_Relations, innerEdges: &mut BTreeSet<i64>, withAttributes: bool, errorHandler: &mut MsgHandler) -> Self {
        PCLoaderOSM_RelationsHandler {
            base: utils::xml::sumo_sax_handler::new("osm - file", ""),
            myAdditionalWays: Ptr::from_mut(additionalWays),
            myRelations: Ptr::from_mut(relations),
            myInnerEdges: Ptr::from_mut(innerEdges),
            myWithAttributes: withAttributes,
            myErrorHandler: Ptr::from_mut(errorHandler),
            myCurrentRelation: Ptr::null(),
            myCurrentWays: Vec::new(),
            myParentElements: Vec::new(),
            myKeep: false,
        }
    }

    /// C++: `void RelationsHandler::~RelationsHandler() noexcept` [virtual]
    /// Destructor
    // PCLoaderOSM::RelationsHandler::~RelationsHandler() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `void RelationsHandler::myStartElement(int, const SUMOSAXAttributes &)` [virtual, protected]
    /// Called on the opening of a tag;
    // void
    // PCLoaderOSM::RelationsHandler::myStartElement(int element, const SUMOSAXAttributes& attrs) {
    //     myParentElements.push_back(element);
    //     // parse "relation" elements
    //     if (element == SUMO_TAG_RELATION) {
    //         myCurrentWays.clear();
    //         bool ok = true;
    //         const std::string& action = attrs.getOpt<std::string>(SUMO_ATTR_ACTION, nullptr, ok);
    //         if (action == "delete" || !ok) {
    //             myCurrentRelation = nullptr;
    //         } else {
    //             myCurrentRelation = new PCOSMRelation();
    //             myCurrentRelation->keep = false;
    //             myCurrentRelation->id = attrs.get<long long int>(SUMO_ATTR_ID, nullptr, ok);
    //             myRelations.push_back(myCurrentRelation);
    //         }
    //         return;
    //     } else if (myCurrentRelation == nullptr) {
    //         return;
    //     }
    //     // parse member elements
    //     if (element == SUMO_TAG_MEMBER) {
    //         bool ok = true;
    //         std::string role = attrs.hasAttribute("role") ? attrs.getStringSecure("role", "") : "";
    //         long long int ref = attrs.get<long long int>(SUMO_ATTR_REF, nullptr, ok);
    //         if (role == "outer" || role == "inner") {
    //             std::string memberType = attrs.get<std::string>(SUMO_ATTR_TYPE, nullptr, ok);
    //             if (memberType == "way") {
    //                 myCurrentWays.push_back(ref);
    //                 if (role == "inner") {
    //                     myInnerEdges.insert(ref);
    //                 }
    //             }
    //         }
    //         return;
    //     }
    //     // parse values
    //     if (element == SUMO_TAG_TAG && myParentElements.size() > 2 && myParentElements[myParentElements.size() - 2] == SUMO_TAG_RELATION
    //             && myCurrentRelation != nullptr) {
    //         bool ok = true;
    //         std::string key = attrs.getOpt<std::string>(SUMO_ATTR_K, toString(myCurrentRelation).c_str(), ok, "", false);
    //         std::string value = attrs.getOpt<std::string>(SUMO_ATTR_V, toString(myCurrentRelation).c_str(), ok, "", false);
    //         if (key == "") {
    //             myErrorHandler.inform("Empty key in a a tag while parsing way '" + toString(myCurrentRelation) + "' occurred.");
    //             ok = false;
    //         }
    //         if (!ok) {
    //             return;
    //         }
    //         if (key == "name") {
    //             myCurrentRelation->name = value;
    //         } else if (MyKeysToInclude.count(key) > 0) {
    //             myCurrentRelation->keep = true;
    //             for (std::vector<long long int>::iterator it = myCurrentWays.begin(); it != myCurrentWays.end(); ++it) {
    //                 myAdditionalWays[*it] = myCurrentRelation;
    //             }
    //         }
    //         myCurrentRelation->myAttributes[key] = value;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) {
        self.myParentElements.push(element as i64);
        // parse "relation" elements
        if element == SUMO_TAG_RELATION {
            self.myCurrentWays.clear();
            let mut ok = true;
            let action: String = attrs.get_opt(SUMO_ATTR_ACTION, "", &mut ok, String::new(), true);
            if action == "delete" || !ok {
                self.myCurrentRelation = Ptr::null();
            } else {
                let id: i64 = attrs.get(SUMO_ATTR_ID, "", &mut ok, true);
                self.myCurrentRelation = leak_box(PCLoaderOSM_PCOSMRelation { id, name: String::new(), myWays: Vec::new(), myAttributes: BTreeMap::new(), keep: false });
                self.myRelations.m().push(self.myCurrentRelation);
            }
            return;
        } else if self.myCurrentRelation.is_null() {
            return;
        }
        // parse member elements
        if element == SUMO_TAG_MEMBER {
            let mut ok = true;
            let role: String = if attrs.has_attribute_name("role") { attrs.get_string_secure_name("role", "") } else { String::new() };
            let refid: i64 = attrs.get(SUMO_ATTR_REF, "", &mut ok, true);
            if role == "outer" || role == "inner" {
                let memberType: String = attrs.get(SUMO_ATTR_TYPE, "", &mut ok, true);
                if memberType == "way" {
                    self.myCurrentWays.push(refid);
                    if role == "inner" {
                        self.myInnerEdges.m().insert(refid);
                    }
                }
            }
            return;
        }
        // parse values
        if element == SUMO_TAG_TAG
            && self.myParentElements.len() > 2
            && self.myParentElements[self.myParentElements.len() - 2] == SUMO_TAG_RELATION as i64
            && !self.myCurrentRelation.is_null()
        {
            let mut ok = true;
            // C++ toString() of the relation pointer: the address is used as objectid
            let relID = format!("{:p}", self.myCurrentRelation.as_raw());
            let key: String = attrs.get_opt(SUMO_ATTR_K, &relID, &mut ok, String::new(), false);
            let value: String = attrs.get_opt(SUMO_ATTR_V, &relID, &mut ok, String::new(), false);
            if key == "" {
                self.myErrorHandler.m().inform(&format!("Empty key in a a tag while parsing way '{}' occurred.", relID), true);
                ok = false;
            }
            if !ok {
                return;
            }
            if key == "name" {
                self.myCurrentRelation.m().name = value.clone();
            } else if MY_KEYS_TO_INCLUDE.contains(&key) {
                self.myCurrentRelation.m().keep = true;
                for it in self.myCurrentWays.iter() {
                    self.myAdditionalWays.m().insert(*it, self.myCurrentRelation);
                }
            }
            self.myCurrentRelation.m().myAttributes.insert(key, value);
        }
    }

    /// C++: `void RelationsHandler::myEndElement(int)` [virtual, protected]
    /// Called when a closing tag occurs
    // void
    // PCLoaderOSM::RelationsHandler::myEndElement(int element) {
    //     myParentElements.pop_back();
    //     if (element == SUMO_TAG_RELATION) {
    //         myCurrentRelation->myWays = myCurrentWays;
    //         myCurrentRelation = nullptr;
    //         myCurrentWays.clear();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_end_element(&mut self, element: i32) {
        self.myParentElements.pop();
        if element == SUMO_TAG_RELATION {
            self.myCurrentRelation.m().myWays = self.myCurrentWays.clone();
            self.myCurrentRelation = Ptr::null();
            self.myCurrentWays.clear();
        }
    }

    /// C++: `void RelationsHandler::RelationsHandler(const RelationsHandler &)` [private]
    /// Invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &PCLoaderOSM_RelationsHandler) -> Self {
        panic!("PCLoaderOSM::RelationsHandler copy constructor is private and undefined in C++ (invalidated)")
    }

    /// C++: `RelationsHandler & RelationsHandler::operator=(const RelationsHandler &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &PCLoaderOSM_RelationsHandler) -> &mut PCLoaderOSM_RelationsHandler {
        panic!("PCLoaderOSM::RelationsHandler assignment operator is private and undefined in C++ (invalidated)")
    }

}

impl SaxHandler for PCLoaderOSM_RelationsHandler {
    fn handler(&mut self) -> &mut utils::xml::generic_sax_handler::GenericSAXHandler {
        &mut self.base
    }
    fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) -> Result<(), ProcessError> {
        PCLoaderOSM_RelationsHandler::my_start_element(self, element, attrs);
        Ok(())
    }
    fn my_end_element(&mut self, element: i32) -> Result<(), ProcessError> {
        PCLoaderOSM_RelationsHandler::my_end_element(self, element);
        Ok(())
    }
}

