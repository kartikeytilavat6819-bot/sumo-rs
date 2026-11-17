//! Port of utils/shapes/ShapeContainer.{h,cpp}
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use super::shape::{DEFAULT_HEIGHT, DEFAULT_NAME};
use crate::utils::parametrised_wrapping_command::ParametrisedWrappingCommand;

/// C++ typedef `NamedObjectCont<PointOfInterest *> ShapeContainer::POIs`
pub type ShapeContainer_POIs = BTreeMap<String, Ptr<PointOfInterest>>; // NamedObjectCont is a std::map

/// C++ typedef `NamedObjectCont<SUMOPolygon *> ShapeContainer::Polygons`
pub type ShapeContainer_Polygons = BTreeMap<String, Ptr<SUMOPolygon>>; // NamedObjectCont is a std::map

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
    pub myPolygonUpdateCommands: BTreeMap<String, Ptr<ParametrisedWrappingCommand<ShapeContainer, Ptr<PolygonDynamics>>>>, // C++: std::map<const std::string, ParametrisedWrappingCommand<ShapeContainer, PolygonDynamics *> *>
    pub myListeners: Vec<Ptr<ShapeListener>>,
}

impl ShapeContainer {
    /// C++: `void ShapeContainer::ShapeContainer()`
    /// Constructor
    // ShapeContainer::ShapeContainer() {}
    #[allow(unused_variables)]
    pub fn new() -> Self {
        ShapeContainer { myPolygons: BTreeMap::new(), myPolygonDynamics: BTreeMap::new(), myHighlightPolygons: BTreeMap::new(), myHighlightedObjects: BTreeMap::new(), myTrackingPolygons: BTreeMap::new(), myPOIs: BTreeMap::new(), myPolygonUpdateCommands: BTreeMap::new(), myListeners: Vec::new() }
    }

    /// C++: `void ShapeContainer::~ShapeContainer()` [virtual]
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor: implicit, nothing to do
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
        self.add(new_ptr(SUMOPolygon::new(id, type_, color, shape, geo, fill, lineWidth, layer, angle, imgFile, name, DEFAULT_HEIGHT, &BTreeMap::new())), ignorePruning)
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
        let p: Ptr<SUMOPolygon> = match self.myPolygons.get(&polyID) {
            Some(p) => p.clone(),
            None => return Ptr::null(),
        };
        // remove eventually existent previously
        self.remove_polygon_dynamics(&polyID);

        // Add new dynamics
        let pd: Ptr<PolygonDynamics> = new_ptr(PolygonDynamics::new(simtime, p.clone(), trackedObject.clone(), timeSpan, alphaSpan, looped, rotate));
        self.myPolygonDynamics.insert(polyID.clone(), pd.clone());

        // Add tracking information
        if !trackedObject.is_null() {
            let trackedID = pd.get_tracked_object_id().to_string();
            self.myTrackingPolygons.entry(trackedID).or_default().insert(p);
        }
        pd
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
        let p: Ptr<SUMOPolygon> = match self.myPolygons.get(polyID) {
            Some(p) => p.clone(),
            None => return false,
        };
        if let Some(d) = self.myPolygonDynamics.get(polyID).cloned() {
            let trackedObjID = d.get_tracked_object_id().to_string();
            if trackedObjID != "" {
                // Remove tracking information
                let i = self.myTrackingPolygons.get_mut(&trackedObjID);
                debug_assert!(i.is_some());
                if let Some(set) = i {
                    debug_assert!(set.contains(&p));
                    set.remove(&p);
                }
                // Remove highlighting information
                self.clear_highlights(&trackedObjID, p.clone());
            }
            self.myPolygonDynamics.remove(polyID);
            // Clear existing polygon dynamics commands before adding new dynamics
            self.cleanup_polygon_dynamics(polyID);
            true
        } else {
            false
        }
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
        self.add_2(new_ptr(PointOfInterest::new(id, type_, color, pos, geo, lane, posOverLane, friendlyPos, posLat, icon, layer, angle, imgFile, width, height, DEFAULT_NAME, &BTreeMap::new())), ignorePruning)
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
        self.remove_polygon_dynamics(id);
        if let Some(p) = self.myPolygons.get(id).cloned() {
            for listener in self.myListeners.clone() {
                listener.m().polygon_changed(p.clone(), false, true);
            }
        }
        self.myPolygons.remove(id).is_some()
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
        if let Some(p) = self.myPOIs.get(id).cloned() {
            for listener in self.myListeners.clone() {
                listener.m().poi_changed(p.clone(), false, true);
            }
        }
        self.myPOIs.remove(id).is_some()
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
        if let Some(p) = self.myPOIs.get(id).cloned() {
            // static_cast<Position*>(p)->set(pos)
            p.m().base2 = pos.clone();
            for listener in self.myListeners.clone() {
                listener.m().poi_changed(p.clone(), false, false);
            }
        }
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
        if let Some(p) = self.myPolygons.get(id).cloned() {
            p.m().set_shape(shape);
            for listener in self.myListeners.clone() {
                listener.m().polygon_changed(p.clone(), false, false);
            }
        }
    }

    /// C++: `const Polygons & ShapeContainer::getPolygons() const` [const]
    /// Returns all polygons
    // inline const Polygons& getPolygons() const {
    //         return myPolygons;
    //     }
    #[allow(unused_variables)]
    pub fn get_polygons(&self) -> &ShapeContainer_Polygons {
        &self.myPolygons
    }

    /// C++: `const POIs & ShapeContainer::getPOIs() const` [const]
    /// Returns all pois
    // inline const POIs& getPOIs() const {
    //         return myPOIs;
    //     }
    #[allow(unused_variables)]
    pub fn get_po_is(&self) -> &ShapeContainer_POIs {
        &self.myPOIs
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
        let next = pd.m().update(t);
        if next == 0 {
            // Dynamics have expired => remove polygon
            let polyID = pd.get_polygon_id().to_string();
            // shortcut: C++ operator[] would insert null and crash here, guarded lookup used instead
            if let Some(cmd) = self.myPolygonUpdateCommands.get(&polyID) {
                cmd.m().deschedule();
            }
            // Don't aquire lock (in GUI case GUIShapeContainer::polygonDynamicsUpdate() does this)
            self.remove_polygon(&polyID, false);
        }
        next
    }

    /// C++: `void ShapeContainer::addPolygonUpdateCommand(std::string, ParametrisedWrappingCommand<ShapeContainer, PolygonDynamics *> *)` [virtual]
    /// Register update command (for descheduling at removal)
    // void
    // ShapeContainer::addPolygonUpdateCommand(std::string polyID, ParametrisedWrappingCommand<ShapeContainer, PolygonDynamics*>* cmd) {
    //     myPolygonUpdateCommands.insert(std::make_pair(polyID, cmd));
    // }
    #[allow(unused_variables)]
    pub fn add_polygon_update_command(&mut self, polyID: String, cmd: Ptr<ParametrisedWrappingCommand<ShapeContainer, Ptr<PolygonDynamics>>>) {
        self.myPolygonUpdateCommands.entry(polyID).or_insert(cmd);
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
        while let Some(first) = self.myTrackingPolygons.get(&objectID).and_then(|s| s.iter().next().cloned()) {
            self.remove_polygon(first.get_id(), true);
        }
        self.myTrackingPolygons.remove(&objectID);
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
        let mut toRemove = String::new();
        self.clear_highlight(objectID, type_, &mut toRemove);
        if toRemove != "" {
            self.remove_polygon(&toRemove, true);
        }
        self.myHighlightPolygons.entry(objectID.to_string()).or_default().entry(type_).or_insert_with(|| polygonID.to_string());
        self.myHighlightedObjects.entry(polygonID.to_string()).or_insert_with(|| objectID.to_string());
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
        for item in self.myPolygonUpdateCommands.values() {
            item.m().deschedule();
        }
        self.myPolygonUpdateCommands.clear();
    }

    /// C++: `void ShapeContainer::addShapeListener(ShapeListener *)`
    // void addShapeListener(ShapeListener* listener) {
    //         myListeners.push_back(listener);
    //     }
    #[allow(unused_variables)]
    pub fn add_shape_listener(&mut self, listener: Ptr<ShapeListener>) {
        self.myListeners.push(listener);
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
        let id = poly.get_id().to_string();
        if self.myPolygons.contains_key(&id) {
            // C++ deletes poly here; dropping the Ptr is the equivalent
            return false;
        }
        self.myPolygons.insert(id, poly.clone());
        for listener in self.myListeners.clone() {
            listener.m().polygon_changed(poly.clone(), true, false);
        }
        true
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
        let id = poi.get_id().to_string();
        if self.myPOIs.contains_key(&id) {
            // C++ deletes poi here; dropping the Ptr is the equivalent
            return false;
        }
        self.myPOIs.insert(id, poi.clone());
        for listener in self.myListeners.clone() {
            listener.m().poi_changed(poi.clone(), true, false);
        }
        true
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
        if let Some(cmd) = self.myPolygonUpdateCommands.remove(id) {
            cmd.m().deschedule();
        }
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
        let mut emptied = false;
        if let Some(inner) = self.myHighlightPolygons.get_mut(objectID) {
            if let Some(v) = inner.remove(&type_) {
                self.myHighlightedObjects.remove(&v);
                *toRemove = v;
                emptied = inner.is_empty();
            }
        }
        if emptied {
            self.myHighlightPolygons.remove(objectID);
        }
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
        let mut emptied = false;
        if let Some(inner) = self.myHighlightPolygons.get_mut(objectID) {
            let found = inner.iter().find(|(_, v)| v.as_str() == p.get_id()).map(|(k, _)| *k);
            if let Some(k) = found {
                inner.remove(&k);
            }
            emptied = inner.is_empty();
        }
        if emptied {
            self.myHighlightPolygons.remove(objectID);
        }
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
        // C++ virtual destructor: empty
    }

    /// C++: `void ShapeListener::polygonChanged(const SUMOPolygon *const, const bool, const bool)` [virtual]
    // virtual void polygonChanged(const SUMOPolygon* const poly, const bool added, const bool removed) {
    //         UNUSED_PARAMETER(poly);
    //         UNUSED_PARAMETER(added);
    //         UNUSED_PARAMETER(removed);
    //     }
    #[allow(unused_variables)]
    pub fn polygon_changed(&mut self, poly: Ptr<SUMOPolygon>, added: bool, removed: bool) {
        // C++ default: UNUSED_PARAMETER only, no-op
        let _ = (poly, added, removed);
    }

    /// C++: `void ShapeListener::poiChanged(const PointOfInterest *const, const bool, const bool)` [virtual]
    // virtual void poiChanged(const PointOfInterest* const poi, const bool added, const bool removed) {
    //         UNUSED_PARAMETER(poi);
    //         UNUSED_PARAMETER(added);
    //         UNUSED_PARAMETER(removed);
    //     }
    #[allow(unused_variables)]
    pub fn poi_changed(&mut self, poi: Ptr<PointOfInterest>, added: bool, removed: bool) {
        // C++ default: UNUSED_PARAMETER only, no-op
        let _ = (poi, added, removed);
    }

}


/// C++ `new T(...)` as a raw Ptr (owned by the shape container for its lifetime)
fn new_ptr<T>(t: T) -> Ptr<T> {
    Ptr::from_raw(Box::into_raw(Box::new(t)))
}
