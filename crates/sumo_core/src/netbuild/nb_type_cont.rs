//! Port of netbuild/NBTypeCont.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::msg_handler::write_warning;
use utils::common::sumo_vehicle_class::get_vehicle_class_names;
use utils::iodevices::output_device::OutputDevice;
use utils::xml::sumo_xml_definitions::{
    LaneSpreadFunctions, SUMO_ATTR_ALLOW, SUMO_ATTR_BIKELANEWIDTH, SUMO_ATTR_DISALLOW, SUMO_ATTR_DISCARD, SUMO_ATTR_FRICTION, SUMO_ATTR_INDEX, SUMO_ATTR_NUMLANES,
    SUMO_ATTR_ONEWAY, SUMO_ATTR_PRIORITY, SUMO_ATTR_SIDEWALKWIDTH, SUMO_ATTR_SPEED, SUMO_ATTR_SPREADTYPE, SUMO_ATTR_VCLASS, SUMO_ATTR_WIDTH, SUMO_TAG_LANETYPE,
    SUMO_TAG_RESTRICTION, SUMO_TAG_TYPE,
};

// NBEdge::UNSPECIFIED_* are static members of NBEdge (NBEdge.cpp) that the Rust NBEdge does not expose yet; same values as NBEdge.cpp
const UNSPECIFIED_SPEED: f64 = -1.;
const UNSPECIFIED_FRICTION: f64 = 1.;
const UNSPECIFIED_WIDTH: f64 = -1.;

/// C++ `delete p` for a Ptr that was allocated as a Box (same convention as ms_route.rs)
fn delete_ptr<T>(p: Ptr<T>) {
    drop(unsafe { Box::from_raw(p.as_raw()) });
}

/// C++ `newType->restrictions.insert(old->restrictions...)` and `attrs.insert(...)`:
/// std::map::insert keeps existing keys, so restrictions already on `to` win
fn merge_restrictions_and_attrs(to: Ptr<NBTypeCont_EdgeTypeDefinition>, from: Ptr<NBTypeCont_EdgeTypeDefinition>) {
    let toM = to.m();
    for (svc, speed) in &from.restrictions {
        toM.restrictions.entry(*svc).or_insert(*speed);
    }
    toM.attrs.extend(from.attrs.iter().copied());
}

/// C++ typedef `std::map<std::string, EdgeTypeDefinition *> NBTypeCont::TypesCont`
pub type NBTypeCont_TypesCont = BTreeMap<String, Ptr<NBTypeCont_EdgeTypeDefinition>>;

/// edgeType definition
/// C++ class `NBTypeCont::EdgeTypeDefinition` (netbuild/NBTypeCont.h)
pub struct NBTypeCont_EdgeTypeDefinition {
    /// The maximal velocity on an edge in m/s
    pub speed: f64,
    /// The default friction on an edge
    pub friction: f64,
    /// The priority of an edge
    pub priority: i32,
    /// List of vehicle edgeTypes that are allowed on this edge
    pub permissions: SVCPermissions,
    /// lane spread type
    pub spreadType: LaneSpreadFunction,
    /// Whether one-way traffic is mostly common for this edgeType (mostly unused)
    pub oneWay: bool,
    /// Whether edges of this edgeType shall be discarded
    pub discard: bool,
    /// The width of lanes of edges of this edgeType [m]
    pub width: f64,
    /// The resolution for interpreting custom (noisy) lane widths of this edgeType [m]
    pub widthResolution: f64,
    /// The maximum width for lanes of this edgeType [m]
    pub maxWidth: f64,
    /// The minimum width for lanes of this edgeType [m]
    pub minWidth: f64,
    pub sidewalkWidth: f64,
    pub bikeLaneWidth: f64,
    /// The vehicle class specific speed restrictions
    pub restrictions: BTreeMap<SUMOVehicleClass, f64>,
    /// The attributes which have been set
    pub attrs: BTreeSet<SumoXMLAttr>,
    /// vector with LaneTypeDefinitions
    pub laneTypeDefinitions: Vec<NBTypeCont_LaneTypeDefinition>,
}

impl NBTypeCont_EdgeTypeDefinition {
    /// C++: `void EdgeTypeDefinition::EdgeTypeDefinition()`
    /// Constructor
    // NBTypeCont::EdgeTypeDefinition::EdgeTypeDefinition() :
    //     speed(13.89), friction(NBEdge::UNSPECIFIED_FRICTION), priority(-1),
    //     permissions(SVC_UNSPECIFIED),
    //     spreadType(LaneSpreadFunction::RIGHT),
    //     oneWay(true), discard(false),
    //     width(NBEdge::UNSPECIFIED_WIDTH),
    //     widthResolution(0),
    //     maxWidth(0),
    //     minWidth(0),
    //     sidewalkWidth(NBEdge::UNSPECIFIED_WIDTH),
    //     bikeLaneWidth(NBEdge::UNSPECIFIED_WIDTH) {
    //     // set laneTypes
    //     laneTypeDefinitions.resize(1);
    // }
    #[allow(unused_variables)]
    pub fn new() -> Self {
        Self {
            speed: 13.89,
            friction: UNSPECIFIED_FRICTION,
            priority: -1,
            permissions: SVC_UNSPECIFIED,
            spreadType: LaneSpreadFunction::RIGHT,
            oneWay: true,
            discard: false,
            width: UNSPECIFIED_WIDTH,
            widthResolution: 0.,
            maxWidth: 0.,
            minWidth: 0.,
            sidewalkWidth: UNSPECIFIED_WIDTH,
            bikeLaneWidth: UNSPECIFIED_WIDTH,
            restrictions: BTreeMap::new(),
            attrs: BTreeSet::new(),
            // set laneTypes
            laneTypeDefinitions: vec![NBTypeCont_LaneTypeDefinition::new()],
        }
    }

    /// C++: `void EdgeTypeDefinition::EdgeTypeDefinition(const EdgeTypeDefinition *)`
    /// copy constructor
    // NBTypeCont::EdgeTypeDefinition::EdgeTypeDefinition(const EdgeTypeDefinition* edgeType) :
    //     speed(edgeType->speed),
    //     friction(edgeType->friction),
    //     priority(edgeType->priority),
    //     permissions(edgeType->permissions),
    //     spreadType(edgeType->spreadType),
    //     oneWay(edgeType->oneWay),
    //     discard(edgeType->discard),
    //     width(edgeType->width),
    //     widthResolution(edgeType->widthResolution),
    //     maxWidth(edgeType->maxWidth),
    //     minWidth(edgeType->minWidth),
    //     sidewalkWidth(edgeType->sidewalkWidth),
    //     bikeLaneWidth(edgeType->bikeLaneWidth),
    //     restrictions(edgeType->restrictions),
    //     attrs(edgeType->attrs),
    //     laneTypeDefinitions(edgeType->laneTypeDefinitions) {
    // }
    #[allow(unused_variables)]
    pub fn new_2(edgeType: Ptr<NBTypeCont_EdgeTypeDefinition>) -> Self {
        let e = edgeType;
        Self {
            speed: e.speed,
            friction: e.friction,
            priority: e.priority,
            permissions: e.permissions,
            spreadType: e.spreadType,
            oneWay: e.oneWay,
            discard: e.discard,
            width: e.width,
            widthResolution: e.widthResolution,
            maxWidth: e.maxWidth,
            minWidth: e.minWidth,
            sidewalkWidth: e.sidewalkWidth,
            bikeLaneWidth: e.bikeLaneWidth,
            restrictions: e.restrictions.clone(),
            attrs: e.attrs.clone(),
            laneTypeDefinitions: e.laneTypeDefinitions.clone(),
        }
    }

    /// C++: `void EdgeTypeDefinition::EdgeTypeDefinition(int, double, double, int, double, SVCPermissions, LaneSpreadFunction, bool, double, double, double, double, double)`
    /// parameter Constructor
    // NBTypeCont::EdgeTypeDefinition::EdgeTypeDefinition(int numLanes, double _speed, double _friction, int _priority,
    //         double _width, SVCPermissions _permissions, LaneSpreadFunction _spreadType, bool _oneWay, double _sideWalkWidth,
    //         double _bikeLaneWidth, double _widthResolution, double _maxWidth, double _minWidth) :
    //     speed(_speed), friction(_friction), priority(_priority), //TODO
    //     permissions(_permissions),
    //     spreadType(_spreadType),
    //     oneWay(_oneWay),
    //     discard(false),
    //     width(_width),
    //     widthResolution(_widthResolution),
    //     maxWidth(_maxWidth),
    //     minWidth(_minWidth),
    //     sidewalkWidth(_sideWalkWidth),
    //     bikeLaneWidth(_bikeLaneWidth) {
    //     // set laneTypes
    //     laneTypeDefinitions.resize(numLanes);
    // }
    #[allow(unused_variables)]
    pub fn new_3(numLanes: i32, speed: f64, friction: f64, priority: i32, width: f64, permissions: SVCPermissions, spreadType: LaneSpreadFunction, oneWay: bool, sideWalkWidth: f64, bikeLaneWidth: f64, widthResolution: f64, maxWidth: f64, minWidth: f64) -> Self {
        Self {
            speed,
            friction,
            priority,
            permissions,
            spreadType,
            oneWay,
            discard: false,
            width,
            widthResolution,
            maxWidth,
            minWidth,
            sidewalkWidth: sideWalkWidth,
            bikeLaneWidth,
            restrictions: BTreeMap::new(),
            attrs: BTreeSet::new(),
            // set laneTypes
            laneTypeDefinitions: (0..numLanes as usize).map(|_| NBTypeCont_LaneTypeDefinition::new()).collect(),
        }
    }

    /// C++: `bool EdgeTypeDefinition::needsLaneType() const` [const]
    /// whether any lane attributes deviate from the edge attributes
    // bool
    // NBTypeCont::EdgeTypeDefinition::needsLaneType() const {
    //     for (const LaneTypeDefinition& laneType : laneTypeDefinitions) {
    //         if (laneType.attrs.count(SUMO_ATTR_SPEED) > 0 && laneType.speed != NBEdge::UNSPECIFIED_SPEED && laneType.speed != speed) {
    //             return true;
    //         }
    //         if (laneType.attrs.count(SUMO_ATTR_FRICTION) > 0 && laneType.friction != NBEdge::UNSPECIFIED_FRICTION && laneType.friction != friction) {
    //             return true;
    //         }
    //         if ((laneType.attrs.count(SUMO_ATTR_DISALLOW) > 0 || laneType.attrs.count(SUMO_ATTR_ALLOW) > 0)
    //                 && laneType.permissions != permissions) {
    //             return true;
    //         }
    //         if (laneType.attrs.count(SUMO_ATTR_WIDTH) > 0 && laneType.width != width && laneType.width != NBEdge::UNSPECIFIED_WIDTH) {
    //             return true;
    //         }
    //         if (laneType.restrictions.size() > 0) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn needs_lane_type(&self) -> bool {
        for laneType in &self.laneTypeDefinitions {
            if laneType.attrs.contains(&SUMO_ATTR_SPEED) && laneType.speed != UNSPECIFIED_SPEED && laneType.speed != self.speed {
                return true;
            }
            if laneType.attrs.contains(&SUMO_ATTR_FRICTION) && laneType.friction != UNSPECIFIED_FRICTION && laneType.friction != self.friction {
                return true;
            }
            if (laneType.attrs.contains(&SUMO_ATTR_DISALLOW) || laneType.attrs.contains(&SUMO_ATTR_ALLOW)) && laneType.permissions != self.permissions {
                return true;
            }
            if laneType.attrs.contains(&SUMO_ATTR_WIDTH) && laneType.width != self.width && laneType.width != UNSPECIFIED_WIDTH {
                return true;
            }
            if !laneType.restrictions.is_empty() {
                return true;
            }
        }
        false
    }

}

/// laneType definition
/// C++ class `NBTypeCont::LaneTypeDefinition` (netbuild/NBTypeCont.h)
#[derive(Clone)]
pub struct NBTypeCont_LaneTypeDefinition {
    /// The maximal velocity on a lane in m/s
    pub speed: f64,
    /// The default friction on a lane
    pub friction: f64,
    /// List of vehicle edgeTypes that are allowed on this lane
    pub permissions: SVCPermissions,
    /// lane width [m]
    pub width: f64,
    /// The vehicle class specific speed restrictions
    pub restrictions: BTreeMap<SUMOVehicleClass, f64>,
    /// The attributes which have been set
    pub attrs: BTreeSet<SumoXMLAttr>,
}

impl NBTypeCont_LaneTypeDefinition {
    /// C++: `void LaneTypeDefinition::LaneTypeDefinition()`
    /// default Constructor
    // NBTypeCont::LaneTypeDefinition::LaneTypeDefinition() :
    //     speed(NBEdge::UNSPECIFIED_SPEED),
    //     friction(NBEdge::UNSPECIFIED_FRICTION),
    //     permissions(SVC_UNSPECIFIED),
    //     width(NBEdge::UNSPECIFIED_WIDTH) {
    // }
    #[allow(unused_variables)]
    pub fn new() -> Self {
        Self {
            speed: UNSPECIFIED_SPEED,
            friction: UNSPECIFIED_FRICTION,
            permissions: SVC_UNSPECIFIED,
            width: UNSPECIFIED_WIDTH,
            restrictions: BTreeMap::new(),
            attrs: BTreeSet::new(),
        }
    }

    /// C++: `void LaneTypeDefinition::LaneTypeDefinition(const EdgeTypeDefinition *)`
    /// constructor (using parameters of EdgeTypeDefinition)
    // NBTypeCont::LaneTypeDefinition::LaneTypeDefinition(const EdgeTypeDefinition* edgeTypeDefinition) :
    //     speed(edgeTypeDefinition->speed),
    //     friction(edgeTypeDefinition->friction),
    //     permissions(edgeTypeDefinition->permissions),
    //     width(edgeTypeDefinition->width) {
    // }
    #[allow(unused_variables)]
    pub fn new_2(edgeTypeDefinition: Ptr<NBTypeCont_EdgeTypeDefinition>) -> Self {
        Self {
            speed: edgeTypeDefinition.speed,
            friction: edgeTypeDefinition.friction,
            permissions: edgeTypeDefinition.permissions,
            width: edgeTypeDefinition.width,
            restrictions: BTreeMap::new(),
            attrs: BTreeSet::new(),
        }
    }

    /// C++: `void LaneTypeDefinition::LaneTypeDefinition(const double, const double, const double, SVCPermissions, const std::set<SumoXMLAttr> &)`
    /// parameter constructor
    // NBTypeCont::LaneTypeDefinition::LaneTypeDefinition(const double _speed, const double _friction, const double _width, SVCPermissions _permissions, const std::set<SumoXMLAttr>& _attrs) :
    //     speed(_speed),
    //     friction(_friction),
    //     permissions(_permissions),
    //     width(_width),
    //     attrs(_attrs) {
    // }
    #[allow(unused_variables)]
    pub fn new_3(speed: f64, friction: f64, width: f64, permissions: SVCPermissions, attrs: &BTreeSet<SumoXMLAttr>) -> Self {
        Self {
            speed,
            friction,
            permissions,
            width,
            restrictions: BTreeMap::new(),
            attrs: attrs.clone(),
        }
    }

    /// C++: `void LaneTypeDefinition::LaneTypeDefinition(const LaneTypeDefinition *)`
    /// copy constructor
    // NBTypeCont::LaneTypeDefinition::LaneTypeDefinition(const LaneTypeDefinition* laneTypeDefinition) :
    //     speed(laneTypeDefinition->speed),
    //     friction(laneTypeDefinition->friction),
    //     permissions(laneTypeDefinition->permissions),
    //     width(laneTypeDefinition->width),
    //     restrictions(laneTypeDefinition->restrictions),
    //     attrs(laneTypeDefinition->attrs) {
    // }
    #[allow(unused_variables)]
    pub fn new_4(laneTypeDefinition: Ptr<NBTypeCont_LaneTypeDefinition>) -> Self {
        let l = laneTypeDefinition;
        Self {
            speed: l.speed,
            friction: l.friction,
            permissions: l.permissions,
            width: l.width,
            restrictions: l.restrictions.clone(),
            attrs: l.attrs.clone(),
        }
    }

}

/// A storage for available edgeTypes of edges
/// C++ class `NBTypeCont` (netbuild/NBTypeCont.h)
pub struct NBTypeCont {
    /// The default edgeType
    pub myDefaultType: Ptr<NBTypeCont_EdgeTypeDefinition>,
    /// The container of edgeTypes
    pub myEdgeTypes: NBTypeCont_TypesCont,
}

impl NBTypeCont {
    /// C++: `void NBTypeCont::NBTypeCont()`
    /// Constructor
    // NBTypeCont::NBTypeCont() :
    //     myDefaultType(new EdgeTypeDefinition()) {}
    #[allow(unused_variables)]
    pub fn new() -> Self {
        Self {
            myDefaultType: Ptr::from_raw(Box::into_raw(Box::new(NBTypeCont_EdgeTypeDefinition::new()))),
            myEdgeTypes: BTreeMap::new(),
        }
    }

    /// C++: `void NBTypeCont::~NBTypeCont()`
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        self.clear_types();
        delete_ptr(self.myDefaultType);
    }

    /// C++: `void NBTypeCont::clearTypes()`
    /// clear types
    // void
    // NBTypeCont::clearTypes() {
    //     // remove edge types
    //     for (const auto& edgeType : myEdgeTypes) {
    //         delete edgeType.second;
    //     }
    //     // clear edge types
    //     myEdgeTypes.clear();
    // }
    #[allow(unused_variables)]
    pub fn clear_types(&mut self) {
        // remove edge types
        for edgeType in std::mem::take(&mut self.myEdgeTypes).into_values() {
            delete_ptr(edgeType);
        }
    }

    /// C++: `void NBTypeCont::setEdgeTypeDefaults(int, double, double, double, int, SVCPermissions, LaneSpreadFunction)`
    /// Sets the default values
    // void
    // NBTypeCont::setEdgeTypeDefaults(int defaultNumLanes,
    //                                 double defaultLaneWidth,
    //                                 double defaultSpeed,
    //                                 double defaultFriction,
    //                                 int defaultPriority,
    //                                 SVCPermissions defaultPermissions,
    //                                 LaneSpreadFunction defaultSpreadType) {
    //     myDefaultType->laneTypeDefinitions.clear();
    //     myDefaultType->laneTypeDefinitions.resize(defaultNumLanes);
    //     myDefaultType->width = defaultLaneWidth;
    //     myDefaultType->speed = defaultSpeed;
    //     myDefaultType->friction = defaultFriction;
    //     myDefaultType->priority = defaultPriority;
    //     myDefaultType->permissions = defaultPermissions;
    //     myDefaultType->spreadType = defaultSpreadType;
    // }
    #[allow(unused_variables)]
    pub fn set_edge_type_defaults(&mut self, defaultNumLanes: i32, defaultLaneWidth: f64, defaultSpeed: f64, defaultFriction: f64, defaultPriority: i32, defaultPermissions: SVCPermissions, defaultSpreadType: LaneSpreadFunction) {
        let d = self.myDefaultType.m();
        d.laneTypeDefinitions.clear();
        d.laneTypeDefinitions.extend((0..defaultNumLanes as usize).map(|_| NBTypeCont_LaneTypeDefinition::new()));
        d.width = defaultLaneWidth;
        d.speed = defaultSpeed;
        d.friction = defaultFriction;
        d.priority = defaultPriority;
        d.permissions = defaultPermissions;
        d.spreadType = defaultSpreadType;
    }

    /// C++: `void NBTypeCont::insertEdgeType(const std::string &, int, double, int, SVCPermissions, LaneSpreadFunction, double, bool, double, double, double, double, double)`
    /// Adds a edgeType into the list
    // void
    // NBTypeCont::insertEdgeType(const std::string& id, int numLanes, double maxSpeed, int prio,
    //                            SVCPermissions permissions, LaneSpreadFunction spreadType, double width,
    //                            bool oneWayIsDefault, double sidewalkWidth, double bikeLaneWidth,
    //                            double widthResolution, double maxWidth, double minWidth) {
    //     // Create edge type definition
    //     EdgeTypeDefinition* newType = new EdgeTypeDefinition(numLanes, maxSpeed, NBEdge::UNSPECIFIED_FRICTION, prio,
    //             width, permissions, spreadType, oneWayIsDefault, sidewalkWidth,
    //             bikeLaneWidth, widthResolution, maxWidth, minWidth);
    //     // check if edgeType already exist in types
    //     TypesCont::iterator old = myEdgeTypes.find(id);
    //     // if exists, then update restrictions and attributes
    //     if (old != myEdgeTypes.end()) {
    //         newType->restrictions.insert(old->second->restrictions.begin(), old->second->restrictions.end());
    //         newType->attrs.insert(old->second->attrs.begin(), old->second->attrs.end());
    //         delete old->second;
    //     }
    //     // insert it in types
    //     myEdgeTypes[id] = newType;
    // }
    #[allow(unused_variables)]
    pub fn insert_edge_type(&mut self, id: &str, numLanes: i32, maxSpeed: f64, prio: i32, permissions: SVCPermissions, spreadType: LaneSpreadFunction, width: f64, oneWayIsDefault: bool, sidewalkWidth: f64, bikeLaneWidth: f64, widthResolution: f64, maxWidth: f64, minWidth: f64) {
        // Create edge type definition
        let newType = Ptr::from_raw(Box::into_raw(Box::new(NBTypeCont_EdgeTypeDefinition::new_3(
            numLanes,
            maxSpeed,
            UNSPECIFIED_FRICTION,
            prio,
            width,
            permissions,
            spreadType,
            oneWayIsDefault,
            sidewalkWidth,
            bikeLaneWidth,
            widthResolution,
            maxWidth,
            minWidth,
        ))));
        // check if edgeType already exist in types
        // if exists, then update restrictions and attributes
        if let Some(old) = self.myEdgeTypes.get(id).copied() {
            merge_restrictions_and_attrs(newType, old);
            delete_ptr(old);
        }
        // insert it in types
        self.myEdgeTypes.insert(id.to_string(), newType);
    }

    /// C++: `void NBTypeCont::insertEdgeType(const std::string &, const EdgeTypeDefinition *)`
    /// Adds a edgeType into the list
    // void
    // NBTypeCont::insertEdgeType(const std::string& id, const EdgeTypeDefinition* edgeType) {
    //     // Create edge type definition
    //     EdgeTypeDefinition* newType = new EdgeTypeDefinition(edgeType);
    //     // check if edgeType already exist in types
    //     TypesCont::iterator old = myEdgeTypes.find(id);
    //     // if exists, then update restrictions and attributes
    //     if (old != myEdgeTypes.end()) {
    //         newType->restrictions.insert(old->second->restrictions.begin(), old->second->restrictions.end());
    //         newType->attrs.insert(old->second->attrs.begin(), old->second->attrs.end());
    //         delete old->second;
    //     }
    //     // insert it in types
    //     myEdgeTypes[id] = newType;
    // }
    #[allow(unused_variables)]
    pub fn insert_edge_type_2(&mut self, id: &str, edgeType: Ptr<NBTypeCont_EdgeTypeDefinition>) {
        // Create edge type definition
        let newType = Ptr::from_raw(Box::into_raw(Box::new(NBTypeCont_EdgeTypeDefinition::new_2(edgeType))));
        // check if edgeType already exist in types
        // if exists, then update restrictions and attributes
        if let Some(old) = self.myEdgeTypes.get(id).copied() {
            merge_restrictions_and_attrs(newType, old);
            delete_ptr(old);
        }
        // insert it in types
        self.myEdgeTypes.insert(id.to_string(), newType);
    }

    /// C++: `void NBTypeCont::insertLaneType(const std::string &, int, double, SVCPermissions, double, const std::set<SumoXMLAttr> &)`
    /// Adds a laneType into the list
    // void
    // NBTypeCont::insertLaneType(const std::string& edgeTypeID, int index, double maxSpeed, SVCPermissions permissions,
    //                            double width, const std::set<SumoXMLAttr>& attrs) {
    //     EdgeTypeDefinition* et = myEdgeTypes.at(edgeTypeID);
    //     while ((int)et->laneTypeDefinitions.size() <= index) {
    //         et->laneTypeDefinitions.push_back(et);
    //     }
    //     // add LaneTypeDefinition with the given attributes
    //     et->laneTypeDefinitions[index] = LaneTypeDefinition(maxSpeed, NBEdge::UNSPECIFIED_FRICTION, width, permissions, attrs);
    // }
    #[allow(unused_variables)]
    pub fn insert_lane_type(&mut self, edgeTypeID: &str, index: i32, maxSpeed: f64, permissions: SVCPermissions, width: f64, attrs: &BTreeSet<SumoXMLAttr>) {
        let et = *self.myEdgeTypes.get(edgeTypeID).expect("std::map::at: key not found");
        while (et.laneTypeDefinitions.len() as i32) <= index {
            et.m().laneTypeDefinitions.push(NBTypeCont_LaneTypeDefinition::new_2(et));
        }
        // add LaneTypeDefinition with the given attributes
        et.m().laneTypeDefinitions[index as usize] = NBTypeCont_LaneTypeDefinition::new_3(maxSpeed, UNSPECIFIED_FRICTION, width, permissions, attrs);
    }

    /// C++: `int NBTypeCont::size() const` [const]
    /// Returns the number of known edgeTypes
    // int
    // NBTypeCont::size() const {
    //     return (int)myEdgeTypes.size();
    // }
    #[allow(unused_variables)]
    pub fn size(&self) -> i32 {
        self.myEdgeTypes.len() as i32
    }

    /// C++: `void NBTypeCont::removeEdgeType(const std::string &)`
    /// Remove a edgeType from the list
    // void
    // NBTypeCont::removeEdgeType(const std::string& id) {
    //     // check if edgeType already exist in types
    //     const auto it = myEdgeTypes.find(id);
    //     // if exists, then remove it
    //     if (it != myEdgeTypes.end()) {
    //         // remove it from map
    //         delete it->second;
    //         myEdgeTypes.erase(it);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remove_edge_type(&mut self, id: &str) {
        if let Some(edgeType) = self.myEdgeTypes.remove(id) {
            delete_ptr(edgeType);
        }
    }

    /// C++: `void NBTypeCont::updateEdgeTypeID(const std::string &, const std::string &)`
    /// change edge type ID
    // void
    // NBTypeCont::updateEdgeTypeID(const std::string& oldId, const std::string& newId) {
    //     // check if edgeType already exist in types
    //     const auto oldIt = myEdgeTypes.find(oldId);
    //     const auto newIt = myEdgeTypes.find(newId);
    //     // if exists, then remove it
    //     if ((oldIt != myEdgeTypes.end()) && (newIt == myEdgeTypes.end())) {
    //         // obtain pointer
    //         auto edgeType = oldIt->second;
    //         // remove it from map
    //         myEdgeTypes.erase(oldIt);
    //         // add it again
    //         myEdgeTypes[newId] = edgeType;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn update_edge_type_id(&mut self, oldId: &str, newId: &str) {
        // if exists, then remove it
        if self.myEdgeTypes.contains_key(oldId) && !self.myEdgeTypes.contains_key(newId) {
            // remove it from map and add it again
            let edgeType = self.myEdgeTypes.remove(oldId).unwrap();
            self.myEdgeTypes.insert(newId.to_string(), edgeType);
        }
    }

    /// C++: `TypesCont::const_iterator NBTypeCont::begin() const` [const]
    /// return begin iterator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn begin(&self) -> std::collections::btree_map::Iter<'_, String, Ptr<NBTypeCont_EdgeTypeDefinition>> {
        self.myEdgeTypes.iter()
    }

    /// C++: `TypesCont::const_iterator NBTypeCont::end() const` [const]
    /// return end iterator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn end(&self) -> std::collections::btree_map::Iter<'_, String, Ptr<NBTypeCont_EdgeTypeDefinition>> {
        // C++ end() is the past-the-end position; a Rust iterator cannot name that position, so return an exhausted iterator
        let mut it = self.myEdgeTypes.iter();
        let _ = it.nth(self.myEdgeTypes.len());
        it
    }

    /// C++: `bool NBTypeCont::knows(const std::string &) const` [const]
    /// Returns whether the named edgeType is in the container
    // bool
    // NBTypeCont::knows(const std::string& type) const {
    //     return myEdgeTypes.find(type) != myEdgeTypes.end();
    // }
    #[allow(unused_variables)]
    pub fn knows(&self, edgeType: &str) -> bool {
        self.myEdgeTypes.contains_key(edgeType)
    }

    /// C++: `const EdgeTypeDefinition * NBTypeCont::getEdgeType(const std::string &) const` [const]
    /// Retrieve the name or the default edgeType
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_edge_type(&self, name: &str) -> Ptr<NBTypeCont_EdgeTypeDefinition> {
        // check if return edge types, or default edge types
        match self.myEdgeTypes.get(name) {
            None => self.myDefaultType,
            Some(t) => *t,
        }
    }

    /// C++: `bool NBTypeCont::markEdgeTypeAsToDiscard(const std::string &)`
    /// Marks a edgeType as to be discarded
    // bool
    // NBTypeCont::markEdgeTypeAsToDiscard(const std::string& id) {
    //     TypesCont::iterator i = myEdgeTypes.find(id);
    //     if (i == myEdgeTypes.end()) {
    //         return false;
    //     }
    //     i->second->discard = true;
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn mark_edge_type_as_to_discard(&mut self, id: &str) -> bool {
        match self.myEdgeTypes.get(id) {
            None => false,
            Some(t) => {
                t.m().discard = true;
                true
            }
        }
    }

    /// C++: `bool NBTypeCont::markEdgeTypeAsSet(const std::string &, const SumoXMLAttr)`
    /// Marks an attribute of a edgeType as set
    // bool
    // NBTypeCont::markEdgeTypeAsSet(const std::string& id, const SumoXMLAttr attr) {
    //     TypesCont::iterator i = myEdgeTypes.find(id);
    //     if (i == myEdgeTypes.end()) {
    //         return false;
    //     }
    //     i->second->attrs.insert(attr);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn mark_edge_type_as_set(&mut self, id: &str, attr: SumoXMLAttr) -> bool {
        match self.myEdgeTypes.get(id) {
            None => false,
            Some(t) => {
                t.m().attrs.insert(attr);
                true
            }
        }
    }

    /// C++: `bool NBTypeCont::addEdgeTypeRestriction(const std::string &, const SUMOVehicleClass, const double)`
    /// Adds a restriction to a edgeType
    // bool
    // NBTypeCont::addEdgeTypeRestriction(const std::string& id, const SUMOVehicleClass svc, const double speed) {
    //     TypesCont::iterator i = myEdgeTypes.find(id);
    //     if (i == myEdgeTypes.end()) {
    //         return false;
    //     }
    //     i->second->restrictions[svc] = speed;
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn add_edge_type_restriction(&mut self, id: &str, svc: SUMOVehicleClass, speed: f64) -> bool {
        match self.myEdgeTypes.get(id) {
            None => false,
            Some(t) => {
                t.m().restrictions.insert(svc, speed);
                true
            }
        }
    }

    /// C++: `bool NBTypeCont::copyEdgeTypeRestrictionsAndAttrs(const std::string &, const std::string &)`
    /// Copy restrictions to a edgeType
    // bool
    // NBTypeCont::copyEdgeTypeRestrictionsAndAttrs(const std::string& fromId, const std::string& toId) {
    //     TypesCont::iterator from = myEdgeTypes.find(fromId);
    //     TypesCont::iterator to = myEdgeTypes.find(toId);
    //     if (from == myEdgeTypes.end() || to == myEdgeTypes.end()) {
    //         return false;
    //     }
    //     to->second->restrictions.insert(from->second->restrictions.begin(), from->second->restrictions.end());
    //     to->second->attrs.insert(from->second->attrs.begin(), from->second->attrs.end());
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn copy_edge_type_restrictions_and_attrs(&mut self, fromId: &str, toId: &str) -> bool {
        let (Some(from), Some(to)) = (self.myEdgeTypes.get(fromId).copied(), self.myEdgeTypes.get(toId).copied()) else {
            return false;
        };
        merge_restrictions_and_attrs(to, from);
        true
    }

    /// C++: `bool NBTypeCont::markLaneTypeAsSet(const std::string &, int, const SumoXMLAttr)`
    /// Marks an attribute of last laneType as set
    // bool
    // NBTypeCont::markLaneTypeAsSet(const std::string& id, int index, const SumoXMLAttr attr) {
    //     TypesCont::iterator i = myEdgeTypes.find(id);
    //     if (i == myEdgeTypes.end()) {
    //         return false;
    //     }
    //     i->second->laneTypeDefinitions[index].attrs.insert(attr);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn mark_lane_type_as_set(&mut self, id: &str, index: i32, attr: SumoXMLAttr) -> bool {
        match self.myEdgeTypes.get(id) {
            None => false,
            Some(t) => {
                t.m().laneTypeDefinitions[index as usize].attrs.insert(attr);
                true
            }
        }
    }

    /// C++: `bool NBTypeCont::addLaneTypeRestriction(const std::string &, const SUMOVehicleClass, const double)`
    /// Adds a restriction to last laneType
    // bool
    // NBTypeCont::addLaneTypeRestriction(const std::string& id, const SUMOVehicleClass svc, const double speed) {
    //     TypesCont::iterator i = myEdgeTypes.find(id);
    //     if (i == myEdgeTypes.end()) {
    //         return false;
    //     }
    //     i->second->laneTypeDefinitions.back().restrictions[svc] = speed;
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn add_lane_type_restriction(&mut self, id: &str, svc: SUMOVehicleClass, speed: f64) -> bool {
        match self.myEdgeTypes.get(id) {
            None => false,
            Some(t) => {
                t.m().laneTypeDefinitions.last_mut().unwrap().restrictions.insert(svc, speed);
                true
            }
        }
    }

    /// C++: `void NBTypeCont::writeEdgeTypes(OutputDevice &, const std::set<std::string> &) const` [const]
    /// writes all EdgeTypes (and their lanes) as XML
    // void
    // NBTypeCont::writeEdgeTypes(OutputDevice& into,  const std::set<std::string>& typeIDs) const {
    //     // iterate over edge types
    //     for (const auto& edgeType : myEdgeTypes) {
    //         if (typeIDs.size() > 0 && typeIDs.count(edgeType.first) == 0) {
    //             continue;
    //         }
    //         auto write = [&](SumoXMLAttr attr, const auto & val) {
    //             into.writeOptionalAttr(attr, val, edgeType.second->attrs.count(attr) == 0);
    //         };
    //
    //         into.openTag(SUMO_TAG_TYPE);
    //         into.writeAttr(SUMO_ATTR_ID, edgeType.first);
    //         write(SUMO_ATTR_PRIORITY, edgeType.second->priority);
    //         into.writeOptionalAttr(SUMO_ATTR_NUMLANES, edgeType.second->laneTypeDefinitions.size(),
    //                                edgeType.second->attrs.count(SUMO_ATTR_NUMLANES) == 0 && edgeType.second->laneTypeDefinitions.size() <= 1);
    //         write(SUMO_ATTR_SPEED, edgeType.second->speed);
    //         write(SUMO_ATTR_FRICTION, edgeType.second->friction);
    //         // write permissions
    //         if ((edgeType.second->attrs.count(SUMO_ATTR_DISALLOW) > 0) || (edgeType.second->attrs.count(SUMO_ATTR_ALLOW) > 0)) {
    //             writePermissions(into, edgeType.second->permissions);
    //         }
    //         // write spreadType (unless default)
    //         if ((edgeType.second->attrs.count(SUMO_ATTR_SPREADTYPE) > 0) && edgeType.second->spreadType != LaneSpreadFunction::RIGHT) {
    //             into.writeAttr(SUMO_ATTR_SPREADTYPE, SUMOXMLDefinitions::LaneSpreadFunctions.getString(edgeType.second->spreadType));
    //         }
    //         write(SUMO_ATTR_ONEWAY, edgeType.second->oneWay);
    //         write(SUMO_ATTR_DISCARD, edgeType.second->discard);
    //         write(SUMO_ATTR_WIDTH, edgeType.second->width);
    //         write(SUMO_ATTR_SIDEWALKWIDTH, edgeType.second->sidewalkWidth);
    //         write(SUMO_ATTR_BIKELANEWIDTH, edgeType.second->bikeLaneWidth);
    //         for (const auto& restriction : edgeType.second->restrictions) {
    //             // open restriction tag
    //             into.openTag(SUMO_TAG_RESTRICTION);
    //             // write vclass
    //             into.writeAttr(SUMO_ATTR_VCLASS, getVehicleClassNames(restriction.first));
    //             // write speed
    //             into.writeAttr(SUMO_ATTR_SPEED, restriction.second);
    //             // close restriction tag
    //             into.closeTag();
    //         }
    //         // iterate over lanes
    //         if (edgeType.second->needsLaneType()) {
    //             int index = 0;
    //             for (const auto& laneType : edgeType.second->laneTypeDefinitions) {
    //                 // open lane type taG
    //                 into.openTag(SUMO_TAG_LANETYPE);
    //                 into.writeAttr(SUMO_ATTR_INDEX, index++);
    //                 // write speed
    //                 if (laneType.attrs.count(SUMO_ATTR_SPEED) > 0 && laneType.speed != NBEdge::UNSPECIFIED_SPEED
    //                         && laneType.speed != edgeType.second->speed) {
    //                     into.writeAttr(SUMO_ATTR_SPEED, laneType.speed);
    //                 }
    //                 // write friction
    //                 if (laneType.attrs.count(SUMO_ATTR_FRICTION) > 0 && laneType.friction != NBEdge::UNSPECIFIED_FRICTION
    //                         && laneType.friction != edgeType.second->friction) {
    //                     into.writeAttr(SUMO_ATTR_FRICTION, laneType.friction);
    //                 }
    //                 // write permissions
    //                 if (laneType.attrs.count(SUMO_ATTR_DISALLOW) > 0 || laneType.attrs.count(SUMO_ATTR_ALLOW) > 0) {
    //                     writePermissions(into, laneType.permissions);
    //                 }
    //                 // write width
    //                 if (laneType.attrs.count(SUMO_ATTR_WIDTH) > 0 && laneType.width != edgeType.second->width
    //                         && laneType.width != NBEdge::UNSPECIFIED_WIDTH) {
    //                     into.writeAttr(SUMO_ATTR_WIDTH, laneType.width);
    //                 }
    //                 // write restrictions
    //                 for (const auto& restriction : laneType.restrictions) {
    //                     // open restriction tag
    //                     into.openTag(SUMO_TAG_RESTRICTION);
    //                     // write vclass
    //                     into.writeAttr(SUMO_ATTR_VCLASS, getVehicleClassNames(restriction.first));
    //                     // write speed
    //                     into.writeAttr(SUMO_ATTR_SPEED, restriction.second);
    //                     // close restriction tag
    //                     into.closeTag();
    //                 }
    //                 // close lane type tag
    //                 into.closeTag();
    //             }
    //         }
    //         // close edge type tag
    //         into.closeTag();
    //     }
    //     //write endlype
    //     if (!myEdgeTypes.empty()) {
    //         into.lf();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_edge_types(&self, into: &mut OutputDevice, typeIDs: &BTreeSet<String>) {
        // iterate over edge types
        for (id, edgeType) in &self.myEdgeTypes {
            if !typeIDs.is_empty() && !typeIDs.contains(id) {
                continue;
            }
            let et = *edgeType;
            // C++ write(attr, val): writeOptionalAttr(attr, val, attrs.count(attr) == 0)
            into.open_tag_id(SUMO_TAG_TYPE);
            into.write_attr(SUMO_ATTR_ID, id.as_str());
            into.write_optional_attr(SUMO_ATTR_PRIORITY, &et.priority, !et.attrs.contains(&SUMO_ATTR_PRIORITY));
            let numLanes = et.laneTypeDefinitions.len();
            into.write_optional_attr(SUMO_ATTR_NUMLANES, &numLanes, !et.attrs.contains(&SUMO_ATTR_NUMLANES) && numLanes <= 1);
            into.write_optional_attr(SUMO_ATTR_SPEED, &et.speed, !et.attrs.contains(&SUMO_ATTR_SPEED));
            into.write_optional_attr(SUMO_ATTR_FRICTION, &et.friction, !et.attrs.contains(&SUMO_ATTR_FRICTION));
            // write permissions
            if et.attrs.contains(&SUMO_ATTR_DISALLOW) || et.attrs.contains(&SUMO_ATTR_ALLOW) {
                // shortcut: writePermissions (utils/common/SUMOVehicleClass.cpp) is not ported, so the allow/disallow attributes of edge types are not written yet; upgrade when writePermissions is ported
            }
            // write spreadType (unless default)
            if et.attrs.contains(&SUMO_ATTR_SPREADTYPE) && et.spreadType != LaneSpreadFunction::RIGHT {
                into.write_attr(SUMO_ATTR_SPREADTYPE, LaneSpreadFunctions.get_string(et.spreadType));
            }
            into.write_optional_attr(SUMO_ATTR_ONEWAY, &et.oneWay, !et.attrs.contains(&SUMO_ATTR_ONEWAY));
            into.write_optional_attr(SUMO_ATTR_DISCARD, &et.discard, !et.attrs.contains(&SUMO_ATTR_DISCARD));
            into.write_optional_attr(SUMO_ATTR_WIDTH, &et.width, !et.attrs.contains(&SUMO_ATTR_WIDTH));
            into.write_optional_attr(SUMO_ATTR_SIDEWALKWIDTH, &et.sidewalkWidth, !et.attrs.contains(&SUMO_ATTR_SIDEWALKWIDTH));
            into.write_optional_attr(SUMO_ATTR_BIKELANEWIDTH, &et.bikeLaneWidth, !et.attrs.contains(&SUMO_ATTR_BIKELANEWIDTH));
            for (svc, speed) in &et.restrictions {
                // open restriction tag
                into.open_tag_id(SUMO_TAG_RESTRICTION);
                // write vclass
                let vclass = get_vehicle_class_names(*svc, false);
                into.write_attr(SUMO_ATTR_VCLASS, vclass.as_str());
                // write speed
                into.write_attr(SUMO_ATTR_SPEED, speed);
                // close restriction tag
                into.close_tag("");
            }
            // iterate over lanes
            if et.needs_lane_type() {
                let mut index: i32 = 0;
                for laneType in &et.laneTypeDefinitions {
                    // open lane type tag
                    into.open_tag_id(SUMO_TAG_LANETYPE);
                    into.write_attr(SUMO_ATTR_INDEX, &index);
                    index += 1;
                    // write speed
                    if laneType.attrs.contains(&SUMO_ATTR_SPEED) && laneType.speed != UNSPECIFIED_SPEED && laneType.speed != et.speed {
                        into.write_attr(SUMO_ATTR_SPEED, &laneType.speed);
                    }
                    // write friction
                    if laneType.attrs.contains(&SUMO_ATTR_FRICTION) && laneType.friction != UNSPECIFIED_FRICTION && laneType.friction != et.friction {
                        into.write_attr(SUMO_ATTR_FRICTION, &laneType.friction);
                    }
                    // write permissions
                    if laneType.attrs.contains(&SUMO_ATTR_DISALLOW) || laneType.attrs.contains(&SUMO_ATTR_ALLOW) {
                        // shortcut: writePermissions (utils/common/SUMOVehicleClass.cpp) is not ported, so lane-level allow/disallow are not written yet; upgrade when writePermissions is ported
                    }
                    // write width
                    if laneType.attrs.contains(&SUMO_ATTR_WIDTH) && laneType.width != et.width && laneType.width != UNSPECIFIED_WIDTH {
                        into.write_attr(SUMO_ATTR_WIDTH, &laneType.width);
                    }
                    // write restrictions
                    for (svc, speed) in &laneType.restrictions {
                        into.open_tag_id(SUMO_TAG_RESTRICTION);
                        let vclass = get_vehicle_class_names(*svc, false);
                        into.write_attr(SUMO_ATTR_VCLASS, vclass.as_str());
                        into.write_attr(SUMO_ATTR_SPEED, speed);
                        into.close_tag("");
                    }
                    // close lane type tag
                    into.close_tag("");
                }
            }
            // close edge type tag
            into.close_tag("");
        }
        // write endtype
        if !self.myEdgeTypes.is_empty() {
            into.lf();
        }
    }

    /// C++: `int NBTypeCont::getEdgeTypeNumLanes(const std::string &) const` [const]
    /// Returns the number of lanes for the given edgeType
    // int
    // NBTypeCont::getEdgeTypeNumLanes(const std::string& type) const {
    //     return (int)getEdgeType(type)->laneTypeDefinitions.size();
    // }
    #[allow(unused_variables)]
    pub fn get_edge_type_num_lanes(&self, edgeType: &str) -> i32 {
        self.get_edge_type(edgeType).laneTypeDefinitions.len() as i32
    }

    /// C++: `double NBTypeCont::getEdgeTypeSpeed(const std::string &) const` [const]
    /// Returns the maximal velocity for the given edgeType [m/s]
    // double
    // NBTypeCont::getEdgeTypeSpeed(const std::string& type) const {
    //     return getEdgeType(type)->speed;
    // }
    #[allow(unused_variables)]
    pub fn get_edge_type_speed(&self, edgeType: &str) -> f64 {
        self.get_edge_type(edgeType).speed
    }

    /// C++: `double NBTypeCont::getEdgeTypeFriction(const std::string &) const` [const]
    /// Returns the default friction for the given edgeType [-]
    // double
    // NBTypeCont::getEdgeTypeFriction(const std::string& type) const {
    //     return getEdgeType(type)->friction;
    // }
    #[allow(unused_variables)]
    pub fn get_edge_type_friction(&self, edgeType: &str) -> f64 {
        self.get_edge_type(edgeType).friction
    }

    /// C++: `int NBTypeCont::getEdgeTypePriority(const std::string &) const` [const]
    /// Returns the priority for the given edgeType
    // int
    // NBTypeCont::getEdgeTypePriority(const std::string& type) const {
    //     return getEdgeType(type)->priority;
    // }
    #[allow(unused_variables)]
    pub fn get_edge_type_priority(&self, edgeType: &str) -> i32 {
        self.get_edge_type(edgeType).priority
    }

    /// C++: `bool NBTypeCont::getEdgeTypeIsOneWay(const std::string &) const` [const]
    /// Returns whether edges are one-way per default for the given edgeType
    // bool
    // NBTypeCont::getEdgeTypeIsOneWay(const std::string& type) const {
    //     return getEdgeType(type)->oneWay;
    // }
    #[allow(unused_variables)]
    pub fn get_edge_type_is_one_way(&self, edgeType: &str) -> bool {
        self.get_edge_type(edgeType).oneWay
    }

    /// C++: `bool NBTypeCont::getEdgeTypeShallBeDiscarded(const std::string &) const` [const]
    /// Returns the information whether edges of this edgeType shall be discarded.
    // bool
    // NBTypeCont::getEdgeTypeShallBeDiscarded(const std::string& type) const {
    //     return getEdgeType(type)->discard;
    // }
    #[allow(unused_variables)]
    pub fn get_edge_type_shall_be_discarded(&self, edgeType: &str) -> bool {
        self.get_edge_type(edgeType).discard
    }

    /// C++: `double NBTypeCont::getEdgeTypeWidthResolution(const std::string &) const` [const]
    /// Returns the resolution for interpreting edge/lane widths of the given edgeType
    // double
    // NBTypeCont::getEdgeTypeWidthResolution(const std::string& type) const {
    //     return getEdgeType(type)->widthResolution;
    // }
    #[allow(unused_variables)]
    pub fn get_edge_type_width_resolution(&self, edgeType: &str) -> f64 {
        self.get_edge_type(edgeType).widthResolution
    }

    /// C++: `double NBTypeCont::getEdgeTypeMaxWidth(const std::string &) const` [const]
    /// Returns the maximum edge/lane widths of the given edgeType
    // double
    // NBTypeCont::getEdgeTypeMaxWidth(const std::string& type) const {
    //     return getEdgeType(type)->maxWidth;
    // }
    #[allow(unused_variables)]
    pub fn get_edge_type_max_width(&self, edgeType: &str) -> f64 {
        self.get_edge_type(edgeType).maxWidth
    }

    /// C++: `double NBTypeCont::getEdgeTypeMinWidth(const std::string &) const` [const]
    /// Returns the minimum edge/lane widths of the given edgeType
    // double
    // NBTypeCont::getEdgeTypeMinWidth(const std::string& type) const {
    //     return getEdgeType(type)->minWidth;
    // }
    #[allow(unused_variables)]
    pub fn get_edge_type_min_width(&self, edgeType: &str) -> f64 {
        self.get_edge_type(edgeType).minWidth
    }

    /// C++: `bool NBTypeCont::wasSetEdgeTypeAttribute(const std::string &, const SumoXMLAttr) const` [const]
    /// Returns whether an attribute of a edgeType was set
    // bool
    // NBTypeCont::wasSetEdgeTypeAttribute(const std::string& type, const SumoXMLAttr attr) const {
    //     return getEdgeType(type)->attrs.count(attr) > 0;
    // }
    #[allow(unused_variables)]
    pub fn was_set_edge_type_attribute(&self, edgeType: &str, attr: SumoXMLAttr) -> bool {
        self.get_edge_type(edgeType).attrs.contains(&attr)
    }

    /// C++: `SVCPermissions NBTypeCont::getEdgeTypePermissions(const std::string &) const` [const]
    /// Returns allowed vehicle classes for the given edgeType
    // SVCPermissions
    // NBTypeCont::getEdgeTypePermissions(const std::string& type) const {
    //     return getEdgeType(type)->permissions;
    // }
    #[allow(unused_variables)]
    pub fn get_edge_type_permissions(&self, edgeType: &str) -> SVCPermissions {
        self.get_edge_type(edgeType).permissions
    }

    /// C++: `LaneSpreadFunction NBTypeCont::getEdgeTypeSpreadType(const std::string &) const` [const]
    /// Returns spreadType for the given edgeType
    // LaneSpreadFunction
    // NBTypeCont::getEdgeTypeSpreadType(const std::string& type) const {
    //     return getEdgeType(type)->spreadType;
    // }
    #[allow(unused_variables)]
    pub fn get_edge_type_spread_type(&self, edgeType: &str) -> LaneSpreadFunction {
        self.get_edge_type(edgeType).spreadType
    }

    /// C++: `double NBTypeCont::getEdgeTypeWidth(const std::string &) const` [const]
    /// Returns the lane width for the given edgeType [m]
    // double
    // NBTypeCont::getEdgeTypeWidth(const std::string& type) const {
    //     return getEdgeType(type)->width;
    // }
    #[allow(unused_variables)]
    pub fn get_edge_type_width(&self, edgeType: &str) -> f64 {
        self.get_edge_type(edgeType).width
    }

    /// C++: `double NBTypeCont::getEdgeTypeSidewalkWidth(const std::string &) const` [const]
    /// Returns the lane width for a sidewalk to be added [m]
    // double
    // NBTypeCont::getEdgeTypeSidewalkWidth(const std::string& type) const {
    //     return getEdgeType(type)->sidewalkWidth;
    // }
    #[allow(unused_variables)]
    pub fn get_edge_type_sidewalk_width(&self, edgeType: &str) -> f64 {
        self.get_edge_type(edgeType).sidewalkWidth
    }

    /// C++: `double NBTypeCont::getEdgeTypeBikeLaneWidth(const std::string &) const` [const]
    /// Returns the lane width for a bike lane to be added [m]
    // double
    // NBTypeCont::getEdgeTypeBikeLaneWidth(const std::string& type) const {
    //     return getEdgeType(type)->bikeLaneWidth;
    // }
    #[allow(unused_variables)]
    pub fn get_edge_type_bike_lane_width(&self, edgeType: &str) -> f64 {
        self.get_edge_type(edgeType).bikeLaneWidth
    }

    /// C++: `void NBTypeCont::NBTypeCont(const NBTypeCont &)` [private]
    /// invalid copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &NBTypeCont) -> Self {
        panic!("NBTypeCont(const NBTypeCont&) is deleted in C++")
    }

    /// C++: `NBTypeCont & NBTypeCont::operator=(const NBTypeCont &)` [private]
    /// invalid assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &NBTypeCont) -> &mut NBTypeCont {
        panic!("NBTypeCont::operator= is deleted in C++")
    }

}

