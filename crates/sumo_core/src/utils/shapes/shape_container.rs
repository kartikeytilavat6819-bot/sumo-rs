//! Port of utils/shapes/ShapeContainer.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `NamedObjectCont<PointOfInterest *> ShapeContainer::POIs`
pub type ShapeContainer_POIs = Unmapped /* NamedObjectCont<PointOfInterest *> */;

/// C++ typedef `NamedObjectCont<SUMOPolygon *> ShapeContainer::Polygons`
pub type ShapeContainer_Polygons = Unmapped /* NamedObjectCont<SUMOPolygon *> */;

/// Storage for geometrical objects
/// C++ class `ShapeContainer` (utils/shapes/ShapeContainer.h)
pub struct ShapeContainer {
    /// stored Polygons
    pub myPolygons: ShapeContainer_Polygons,
    /// stored PolygonDynamics
    pub myPolygonDynamics: BTreeMap<String, Ptr<PolygonDynamics>>,
    /// maps objects to a map of highlight types to highlighting polygons
    pub myHighlightPolygons: BTreeMap<String, BTreeMap<i32, String>>, // C++: std::map<std::string, std::map<int, std::string>>
    /// inverse map to myHighlightPolygons saves the highlighted object for each polygon
    pub myHighlightedObjects: BTreeMap<String, String>,
    /// Information about tracked objects
    pub myTrackingPolygons: BTreeMap<String, BTreeSet<Ptr<SUMOPolygon>>>, // C++: std::map<const std::string, std::set<const SUMOPolygon *>>
    /// stored POIs
    pub myPOIs: ShapeContainer_POIs,
    /// Command pointers for scheduled polygon update. Maps PolyID->Command
    pub myPolygonUpdateCommands: BTreeMap<String, Ptr<Unmapped /* ParametrisedWrappingCommand<ShapeContainer, PolygonDynamics *> */>>, // C++: std::map<const std::string, ParametrisedWrappingCommand<ShapeContainer, PolygonDynamics *> *>
    pub myListeners: Vec<Ptr<ShapeListener>>,
}

impl ShapeContainer {
    /// C++: `void ShapeContainer::ShapeContainer()`
    /// Constructor
    // ShapeContainer::ShapeContainer() {}
    #[allow(unused_variables)]
    pub fn new() -> Self {
        todo!("port ShapeContainer::ShapeContainer")
    }

    /// C++: `void ShapeContainer::~ShapeContainer()` [virtual]
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port ShapeContainer::~ShapeContainer")
    }

    /// C++: `bool ShapeContainer::addPolygon(const std::string &, const std::string &, const RGBColor &, double, double, const std::string &, const PositionVector &, bool, bool, double, bool, const std::string &)` [virtual]
    /// Builds a polygon using the given values and adds it to the container
    // bool
    // ShapeContainer::addPolygon(const std::string& id, const std::string& type, const RGBColor& color,
    //                            double layer, double angle, const std::string& imgFile, const PositionVector& shape,
    //                            bool geo, bool fill, double lineWidth, bool ignorePruning, const std::string& name) {
    //     return add(new SUMOPolygon(id, type, color, shape, geo, fill, lineWidth, layer, angle, imgFile, name), ignorePruning);
    // }
    #[allow(unused_variables)]
    pub fn add_polygon(&mut self, id: &str, type_: &str, color: &RGBColor, layer: f64, angle: f64, imgFile: &str, shape: &PositionVector, geo: bool, fill: bool, lineWidth: f64, ignorePruning: bool, name: &str) -> bool {
        todo!("port ShapeContainer::addPolygon")
    }

    /// C++: `PolygonDynamics * ShapeContainer::addPolygonDynamics(double, std::string, SUMOTrafficObject *, const std::vector<double> &, const std::vector<double> &, bool, bool)` [virtual]
    /// Adds dynamics (animation / tracking) to the given polygon
    // PolygonDynamics*
    // ShapeContainer::addPolygonDynamics(double simtime,
    //                                    std::string polyID,
    //                                    SUMOTrafficObject* trackedObject,
    //                                    const std::vector<double>& timeSpan,
    //                                    const std::vector<double>& alphaSpan,
    //                                    bool looped,
    //                                    bool rotate) {
    //
    // #ifdef DEBUG_DYNAMIC_SHAPES
    //     std::cout << simtime << " ShapeContainer::addPolygonDynamics() called for polygon '" << polyID << "'" << std::endl;
    // #endif
    //
    //     SUMOPolygon* p = myPolygons.get(polyID);
    //     if (p == nullptr) {
    // #ifdef DEBUG_DYNAMIC_SHAPES
    //         std::cout << "   polygon '" << polyID << "' doesn't exist!" << std::endl;
    // #endif
    //         return nullptr;
    //     }
    //     // remove eventually existent previously
    //     removePolygonDynamics(polyID);
    //
    //     // Add new dynamics
    //     PolygonDynamics* pd = new PolygonDynamics(simtime, p, trackedObject, timeSpan, alphaSpan, looped, rotate);
    //     myPolygonDynamics.insert(std::make_pair(polyID, pd));
    //
    //     // Add tracking information
    //     if (trackedObject != nullptr) {
    //         auto i = myTrackingPolygons.find(pd->getTrackedObjectID());
    //         if (i == myTrackingPolygons.end()) {
    //             myTrackingPolygons.insert(std::make_pair(pd->getTrackedObjectID(), std::set<const SUMOPolygon*>({p})));
    //         } else {
    //             i->second.insert(p);
    //         }
    //     }
    //     return pd;
    // }
    #[allow(unused_variables)]
    pub fn add_polygon_dynamics(&mut self, simtime: f64, polyID: String, trackedObject: Ptr<SUMOTrafficObject>, timeSpan: &Vec<f64>, alphaSpan: &Vec<f64>, looped: bool, rotate: bool) -> Ptr<PolygonDynamics> {
        todo!("port ShapeContainer::addPolygonDynamics")
    }

    /// C++: `bool ShapeContainer::removePolygonDynamics(const std::string &)` [virtual]
    /// Remove dynamics (animation / tracking) for the given polygon
    // bool
    // ShapeContainer::removePolygonDynamics(const std::string& polyID) {
    //     SUMOPolygon* p = myPolygons.get(polyID);
    //     if (p == nullptr) {
    //         return false;
    //     }
    //     auto d = myPolygonDynamics.find(polyID);
    //     if (d != myPolygonDynamics.end()) {
    // #ifdef DEBUG_DYNAMIC_SHAPES
    //         std::cout << "   Removing dynamics of polygon '" << polyID << "'" << std::endl;
    // #endif
    //         const std::string& trackedObjID = d->second->getTrackedObjectID();
    //         if (trackedObjID != "") {
    //             // Remove tracking information
    //             auto i = myTrackingPolygons.find(trackedObjID);
    //             assert(i != myTrackingPolygons.end());
    //             assert(i->second.find(p) != i->second.end());
    //             i->second.erase(p);
    //             // Remove highlighting information
    //             clearHighlights(trackedObjID, p);
    //         }
    //         delete d->second;
    //         myPolygonDynamics.erase(d);
    //         // Clear existing polygon dynamics commands before adding new dynamics
    //         cleanupPolygonDynamics(polyID);
    //         return true;
    //     } else {
    //         return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remove_polygon_dynamics(&mut self, polyID: &str) -> bool {
        todo!("port ShapeContainer::removePolygonDynamics")
    }

    /// C++: `bool ShapeContainer::addPOI(const std::string &, const std::string &, const RGBColor &, const Position &, bool, const std::string &, double, bool, double, const std::string &, double, double, const std::string &, double, double, bool)` [virtual]
    /// Builds a POI using the given values and adds it to the container
    // bool
    // ShapeContainer::addPOI(const std::string& id, const std::string& type, const RGBColor& color, const Position& pos, bool geo,
    //                        const std::string& lane, double posOverLane, bool friendlyPos, double posLat, const std::string& icon, double layer,
    //                        double angle, const std::string& imgFile, double width, double height, bool ignorePruning) {
    //     return add(new PointOfInterest(id, type, color, pos, geo, lane, posOverLane, friendlyPos, posLat, icon, layer, angle, imgFile, width, height), ignorePruning);
    // }
    #[allow(unused_variables)]
    pub fn add_poi(&mut self, id: &str, type_: &str, color: &RGBColor, pos: &Position, geo: bool, lane: &str, posOverLane: f64, friendlyPos: bool, posLat: f64, icon: &str, layer: f64, angle: f64, imgFile: &str, width: f64, height: f64, ignorePruning: bool) -> bool {
        todo!("port ShapeContainer::addPOI")
    }

    /// C++: `bool ShapeContainer::removePolygon(const std::string &, bool)` [virtual]
    /// Removes a polygon from the container
    // bool
    // ShapeContainer::removePolygon(const std::string& id, bool /* useLock */) {
    // #ifdef DEBUG_DYNAMIC_SHAPES
    //     std::cout << "ShapeContainer: Removing Polygon '" << id << "'" << std::endl;
    // #endif
    //     removePolygonDynamics(id);
    //     SUMOPolygon* p = myPolygons.get(id);
    //     if (p != nullptr) {
    //         for (ShapeListener* listener : myListeners) {
    //             listener->polygonChanged(p, false, true);
    //         }
    //     }
    //     return myPolygons.remove(id);
    // }
    #[allow(unused_variables)]
    pub fn remove_polygon(&mut self, id: &str, useLock: bool) -> bool {
        todo!("port ShapeContainer::removePolygon")
    }

    /// C++: `bool ShapeContainer::removePOI(const std::string &)` [virtual]
    /// Removes a PoI from the container
    // bool
    // ShapeContainer::removePOI(const std::string& id) {
    //     PointOfInterest* p = myPOIs.get(id);
    //     if (p != nullptr) {
    //         for (ShapeListener* listener : myListeners) {
    //             listener->poiChanged(p, false, true);
    //         }
    //     }
    //     return myPOIs.remove(id);
    // }
    #[allow(unused_variables)]
    pub fn remove_poi(&mut self, id: &str) -> bool {
        todo!("port ShapeContainer::removePOI")
    }

    /// C++: `void ShapeContainer::movePOI(const std::string &, const Position &)` [virtual]
    /// Assigns a new position to the named PoI
    // void
    // ShapeContainer::movePOI(const std::string& id, const Position& pos) {
    //     PointOfInterest* p = myPOIs.get(id);
    //     if (p != nullptr) {
    //         static_cast<Position*>(p)->set(pos);
    //         for (ShapeListener* listener : myListeners) {
    //             listener->poiChanged(p, false, false);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn move_poi(&mut self, id: &str, pos: &Position) {
        todo!("port ShapeContainer::movePOI")
    }

    /// C++: `void ShapeContainer::reshapePolygon(const std::string &, const PositionVector &)` [virtual]
    /// Assigns a shape to the named polygon
    // void
    // ShapeContainer::reshapePolygon(const std::string& id, const PositionVector& shape) {
    //     SUMOPolygon* p = myPolygons.get(id);
    //     if (p != nullptr) {
    //         p->setShape(shape);
    //         for (ShapeListener* listener : myListeners) {
    //             listener->polygonChanged(p, false, false);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn reshape_polygon(&mut self, id: &str, shape: &PositionVector) {
        todo!("port ShapeContainer::reshapePolygon")
    }

    /// C++: `const Polygons & ShapeContainer::getPolygons() const` [const]
    /// Returns all polygons
    // inline const Polygons& getPolygons() const {
    //         return myPolygons;
    //     }
    #[allow(unused_variables)]
    pub fn get_polygons(&self) -> &ShapeContainer_Polygons {
        todo!("port ShapeContainer::getPolygons")
    }

    /// C++: `const POIs & ShapeContainer::getPOIs() const` [const]
    /// Returns all pois
    // inline const POIs& getPOIs() const {
    //         return myPOIs;
    //     }
    #[allow(unused_variables)]
    pub fn get_po_is(&self) -> &ShapeContainer_POIs {
        todo!("port ShapeContainer::getPOIs")
    }

    /// C++: `SUMOTime ShapeContainer::polygonDynamicsUpdate(SUMOTime, PolygonDynamics *)` [virtual]
    /// Regular update event for updating polygon dynamics
    // SUMOTime
    // ShapeContainer::polygonDynamicsUpdate(SUMOTime t, PolygonDynamics* pd) {
    //     SUMOTime next = pd->update(t);
    //     if (next == 0) {
    //         // Dynamics have expired => remove polygon
    //         myPolygonUpdateCommands[pd->getPolygonID()]->deschedule();
    //         // Don't aquire lock (in GUI case GUIShapeContainer::polygonDynamicsUpdate() does this)
    //         removePolygon(pd->getPolygonID(), false);
    //     }
    //     return next;
    // }
    #[allow(unused_variables)]
    pub fn polygon_dynamics_update(&mut self, t: SUMOTime, pd: Ptr<PolygonDynamics>) -> SUMOTime {
        todo!("port ShapeContainer::polygonDynamicsUpdate")
    }

    /// C++: `void ShapeContainer::addPolygonUpdateCommand(std::string, ParametrisedWrappingCommand<ShapeContainer, PolygonDynamics *> *)` [virtual]
    /// Register update command (for descheduling at removal)
    // void
    // ShapeContainer::addPolygonUpdateCommand(std::string polyID, ParametrisedWrappingCommand<ShapeContainer, PolygonDynamics*>* cmd) {
    //     myPolygonUpdateCommands.insert(std::make_pair(polyID, cmd));
    // }
    #[allow(unused_variables)]
    pub fn add_polygon_update_command(&mut self, polyID: String, cmd: Ptr<Unmapped /* ParametrisedWrappingCommand<ShapeContainer, PolygonDynamics *> */>) {
        todo!("port ShapeContainer::addPolygonUpdateCommand")
    }

    /// C++: `void ShapeContainer::removeTrackers(std::string)` [virtual]
    /// Remove all tracking polygons for the given object
    // void
    // ShapeContainer::removeTrackers(std::string objectID) {
    //     auto i = myTrackingPolygons.find(objectID);
    //     if (i != myTrackingPolygons.end()) {
    // #ifdef DEBUG_DYNAMIC_SHAPES
    //         std::cout << " Removing tracking polygons for object '" << objectID << "'" << std::endl;
    // #endif
    //         while (!i->second.empty()) {
    //             removePolygon((*i->second.begin())->getID());
    //         }
    //         myTrackingPolygons.erase(i);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remove_trackers(&mut self, objectID: String) {
        todo!("port ShapeContainer::removeTrackers")
    }

    /// C++: `void ShapeContainer::registerHighlight(const std::string &, const int, const std::string &)` [virtual]
    /// register highlight of the specified type if the given id
    // void
    // ShapeContainer::registerHighlight(const std::string& objectID, const int type, const std::string& polygonID) {
    //     std::string toRemove = "";
    //     clearHighlight(objectID, type, toRemove);
    //     if (toRemove != "") {
    //         removePolygon(toRemove);
    //     }
    //     auto i = myHighlightPolygons.find(objectID);
    //     if (i == myHighlightPolygons.end()) {
    //         myHighlightPolygons.insert(std::make_pair(objectID, std::map<int, std::string>({std::make_pair(type, polygonID)})));
    //     } else {
    //         i->second.insert(std::make_pair(type, polygonID));
    //     }
    //     myHighlightedObjects.insert(std::make_pair(polygonID, objectID));
    // }
    #[allow(unused_variables)]
    pub fn register_highlight(&mut self, objectID: &str, type_: i32, polygonID: &str) {
        todo!("port ShapeContainer::registerHighlight")
    }

    /// C++: `void ShapeContainer::clearState()`
    /// Remove all dynamics before quick-loading state
    // void
    // ShapeContainer::clearState() {
    //     for (auto& item : myPolygonUpdateCommands) {
    //         item.second->deschedule();
    //     }
    //     myPolygonUpdateCommands.clear();
    // }
    #[allow(unused_variables)]
    pub fn clear_state(&mut self) {
        todo!("port ShapeContainer::clearState")
    }

    /// C++: `void ShapeContainer::addShapeListener(ShapeListener *)`
    // void addShapeListener(ShapeListener* listener) {
    //         myListeners.push_back(listener);
    //     }
    #[allow(unused_variables)]
    pub fn add_shape_listener(&mut self, listener: Ptr<ShapeListener>) {
        todo!("port ShapeContainer::addShapeListener")
    }

    /// C++: `bool ShapeContainer::add(SUMOPolygon *, bool)` [virtual, protected]
    /// add polygon
    // bool
    // ShapeContainer::add(SUMOPolygon* poly, bool /* ignorePruning */) {
    //     if (!myPolygons.add(poly->getID(), poly)) {
    //         delete poly;
    //         return false;
    //     }
    //     for (ShapeListener* listener : myListeners) {
    //         listener->polygonChanged(poly, true, false);
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn add(&mut self, poly: Ptr<SUMOPolygon>, ignorePruning: bool) -> bool {
        todo!("port ShapeContainer::add")
    }

    /// C++: `bool ShapeContainer::add(PointOfInterest *, bool)` [virtual, protected]
    /// add poi
    // bool
    // ShapeContainer::add(PointOfInterest* poi, bool /* ignorePruning */) {
    //     if (!myPOIs.add(poi->getID(), poi)) {
    //         delete poi;
    //         return false;
    //     }
    //     for (ShapeListener* listener : myListeners) {
    //         listener->poiChanged(poi, true, false);
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn add_2(&mut self, poi: Ptr<PointOfInterest>, ignorePruning: bool) -> bool {
        todo!("port ShapeContainer::add")
    }

    /// C++: `void ShapeContainer::cleanupPolygonDynamics(const std::string &)` [virtual, protected]
    /// Unschedules the removal and update commands of the given polygon.
    // void
    // ShapeContainer::cleanupPolygonDynamics(const std::string& id) {
    //     auto j = myPolygonUpdateCommands.find(id);
    //     if (j != myPolygonUpdateCommands.end()) {
    //         j->second->deschedule();
    //         myPolygonUpdateCommands.erase(j);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn cleanup_polygon_dynamics(&mut self, id: &str) {
        todo!("port ShapeContainer::cleanupPolygonDynamics")
    }

    /// C++: `void ShapeContainer::clearHighlight(const std::string &, const int, std::string &)` [virtual, protected]
    /// Remove any previously added highlight polygon of the specified type
    // void
    // ShapeContainer::clearHighlight(const std::string& objectID, const int type, std::string& toRemove) {
    //     auto i = myHighlightPolygons.find(objectID);
    //     if (i != myHighlightPolygons.end()) {
    //         auto j = i->second.find(type);
    //         if (j != i->second.end()) {
    //             toRemove = j->second;
    //             myHighlightedObjects.erase(toRemove);
    //             i->second.erase(j);
    //             if (i->second.empty()) {
    //                 myHighlightPolygons.erase(i);
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn clear_highlight(&mut self, objectID: &str, type_: i32, toRemove: &mut String) {
        todo!("port ShapeContainer::clearHighlight")
    }

    /// C++: `void ShapeContainer::clearHighlights(const std::string &, SUMOPolygon *)` [virtual, protected]
    /// Clears all highlight information from the maps when the object leaves the net (Highlight polygons and dynamics are removed via removeTrackers())
    // void
    // ShapeContainer::clearHighlights(const std::string& objectID, SUMOPolygon* p) {
    //     auto i = myHighlightPolygons.find(objectID);
    //     if (i != myHighlightPolygons.end()) {
    //         auto j = i->second.begin();
    //         while (j != i->second.end()) {
    //             if (j->second == p->getID()) {
    //                 i->second.erase(j);
    //                 break;
    //             } else {
    //                 ++j;
    //             }
    //         }
    //         if (i->second.empty()) {
    //             myHighlightPolygons.erase(i);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn clear_highlights(&mut self, objectID: &str, p: Ptr<SUMOPolygon>) {
        todo!("port ShapeContainer::clearHighlights")
    }

}

/// Interface for objects which want to be notified about shape updates
/// C++ class `ShapeListener` (utils/shapes/ShapeContainer.h)
pub struct ShapeListener {
}

impl ShapeListener {
    /// C++: `void ShapeListener::~ShapeListener() noexcept` [virtual]
    // virtual ~ShapeListener() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port ShapeListener::~ShapeListener")
    }

    /// C++: `void ShapeListener::polygonChanged(const SUMOPolygon *const, const bool, const bool)` [virtual]
    // virtual void polygonChanged(const SUMOPolygon* const poly, const bool added, const bool removed) {
    //         UNUSED_PARAMETER(poly);
    //         UNUSED_PARAMETER(added);
    //         UNUSED_PARAMETER(removed);
    //     }
    #[allow(unused_variables)]
    pub fn polygon_changed(&mut self, poly: Ptr<SUMOPolygon>, added: bool, removed: bool) {
        todo!("port ShapeListener::polygonChanged")
    }

    /// C++: `void ShapeListener::poiChanged(const PointOfInterest *const, const bool, const bool)` [virtual]
    // virtual void poiChanged(const PointOfInterest* const poi, const bool added, const bool removed) {
    //         UNUSED_PARAMETER(poi);
    //         UNUSED_PARAMETER(added);
    //         UNUSED_PARAMETER(removed);
    //     }
    #[allow(unused_variables)]
    pub fn poi_changed(&mut self, poi: Ptr<PointOfInterest>, added: bool, removed: bool) {
        todo!("port ShapeListener::poiChanged")
    }

}

