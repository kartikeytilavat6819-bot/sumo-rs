//! Port of microsim/traffic_lights/MSRailSignalConstraint.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ enum `MSRailSignalConstraint::ConstraintType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum MSRailSignalConstraint_ConstraintType {
    PREDECESSOR = 0,
    INSERTION_PREDECESSOR = 1,
    FOE_INSERTION = 2,
    INSERTION_ORDER = 3,
    BIDI_PREDECESSOR = 4,
}

/// A base class for constraints
/// C++ class `MSRailSignalConstraint` (microsim/traffic_lights/MSRailSignalConstraint.h)
pub struct MSRailSignalConstraint {
    /// C++ base class `Parameterised`
    pub base: Parameterised,
    pub myType: MSRailSignalConstraint_ConstraintType,
}

impl std::ops::Deref for MSRailSignalConstraint {
    type Target = Parameterised;
    fn deref(&self) -> &Parameterised {
        &self.base
    }
}
impl std::ops::DerefMut for MSRailSignalConstraint {
    fn deref_mut(&mut self) -> &mut Parameterised {
        &mut self.base
    }
}

// C++ static members of MSRailSignalConstraint (port as module-level statics, see AGENTS.md):
//   static std::map<std::string, std::string> MSRailSignalConstraint::myTripIdLookup;

impl MSRailSignalConstraint {
    /// C++: `void MSRailSignalConstraint::MSRailSignalConstraint(ConstraintType)`
    /// Constructor
    // MSRailSignalConstraint(ConstraintType type) : myType(type) {}
    #[allow(unused_variables)]
    pub fn new(type_: MSRailSignalConstraint_ConstraintType) -> Self {
        todo!("port MSRailSignalConstraint::MSRailSignalConstraint")
    }

    /// C++: `void MSRailSignalConstraint::~MSRailSignalConstraint() noexcept` [virtual]
    /// Destructor
    // virtual ~MSRailSignalConstraint() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSRailSignalConstraint::~MSRailSignalConstraint")
    }

    /// C++: `bool MSRailSignalConstraint::cleared() const` [const, virtual, pure virtual]
    /// whether the constraint has been met
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn cleared(&self) -> bool {
        todo!("port MSRailSignalConstraint::cleared")
    }

    /// C++: `void MSRailSignalConstraint::setActive(bool)` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn set_active(&mut self, active: bool) {
        todo!("port MSRailSignalConstraint::setActive")
    }

    /// C++: `bool MSRailSignalConstraint::isActive() const` [const, virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn is_active(&self) -> bool {
        todo!("port MSRailSignalConstraint::isActive")
    }

    /// C++: `std::string MSRailSignalConstraint::getDescription() const` [const, virtual]
    // virtual std::string getDescription() const {
    //         return "RailSignalConstraint";
    //     }
    #[allow(unused_variables)]
    pub fn get_description(&self) -> String {
        todo!("port MSRailSignalConstraint::getDescription")
    }

    /// C++: `const SUMOVehicle * MSRailSignalConstraint::getFoe() const` [const, virtual]
    // virtual const SUMOVehicle* getFoe() const {
    //         return nullptr;
    //     }
    #[allow(unused_variables)]
    pub fn get_foe(&self) -> Ptr<SUMOVehicle> {
        todo!("port MSRailSignalConstraint::getFoe")
    }

    /// C++: `void MSRailSignalConstraint::write(OutputDevice &, const std::string &) const` [const, virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn write(&self, out: &mut OutputDevice, tripId: &str) {
        todo!("port MSRailSignalConstraint::write")
    }

    /// C++: `ConstraintType MSRailSignalConstraint::getType() const` [const]
    // ConstraintType getType() const {
    //         return myType;
    //     }
    #[allow(unused_variables)]
    pub fn get_type(&self) -> MSRailSignalConstraint_ConstraintType {
        todo!("port MSRailSignalConstraint::getType")
    }

    /// C++: `SumoXMLTag MSRailSignalConstraint::getTag() const` [const]
    // SumoXMLTag getTag() const {
    //         switch (myType) {
    //             case INSERTION_PREDECESSOR:
    //                 return SUMO_TAG_INSERTION_PREDECESSOR;
    //             case FOE_INSERTION:
    //                 return SUMO_TAG_FOE_INSERTION;
    //             case INSERTION_ORDER:
    //                 return SUMO_TAG_INSERTION_ORDER;
    //             case BIDI_PREDECESSOR:
    //                 return SUMO_TAG_BIDI_PREDECESSOR;
    //             default:
    //                 return SUMO_TAG_PREDECESSOR;
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn get_tag(&self) -> SumoXMLTag {
        todo!("port MSRailSignalConstraint::getTag")
    }

    /// C++: `ConstraintType MSRailSignalConstraint::getSwappedType() const` [const]
    // ConstraintType getSwappedType() const {
    //         switch (myType) {
    //             case INSERTION_PREDECESSOR:
    //                 return FOE_INSERTION;
    //             case FOE_INSERTION:
    //                 return INSERTION_PREDECESSOR;
    //             default:
    //                 return myType;
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn get_swapped_type(&self) -> MSRailSignalConstraint_ConstraintType {
        todo!("port MSRailSignalConstraint::getSwappedType")
    }

    /// C++: `bool MSRailSignalConstraint::isInsertionConstraint() const` [const]
    // bool isInsertionConstraint() const {
    //         return myType == INSERTION_PREDECESSOR || myType == INSERTION_ORDER;
    //     }
    #[allow(unused_variables)]
    pub fn is_insertion_constraint(&self) -> bool {
        todo!("port MSRailSignalConstraint::isInsertionConstraint")
    }

    /// C++: `void MSRailSignalConstraint::storeTripId(const std::string &, const std::string &)` [static]
    // void
    // MSRailSignalConstraint::storeTripId(const std::string& tripId, const std::string& vehID) {
    //     myTripIdLookup[tripId] = vehID;
    // }
    #[allow(unused_variables)]
    pub fn store_trip_id(tripId: &str, vehID: &str) {
        todo!("port MSRailSignalConstraint::storeTripId")
    }

    /// C++: `const std::string & MSRailSignalConstraint::lookupVehId(const std::string &)` [static]
    // const std::string&
    // MSRailSignalConstraint::lookupVehId(const std::string& tripId) {
    //     return myTripIdLookup[tripId];
    // }
    #[allow(unused_variables)]
    pub fn lookup_veh_id(tripId: &str) -> &'static str {
        todo!("port MSRailSignalConstraint::lookupVehId")
    }

    /// C++: `void MSRailSignalConstraint::cleanup()` [static]
    /// clean up state
    // void
    // MSRailSignalConstraint::cleanup() {
    //     MSRailSignalConstraint_Predecessor::cleanup();
    // }
    #[allow(unused_variables)]
    pub fn cleanup() {
        todo!("port MSRailSignalConstraint::cleanup")
    }

    /// C++: `void MSRailSignalConstraint::saveState(OutputDevice &)` [static]
    /// Saves the current constraint states into the given stream
    // void
    // MSRailSignalConstraint::saveState(OutputDevice& out) {
    //     if (OptionsCont::getOptions().getBool("save-state.constraints")) {
    //         for (MSRailSignal* s : MSRailSignalControl::getInstance().getSignals()) {
    //             if (s->getConstraints().size() > 0) {
    //                 out.openTag(SUMO_TAG_RAILSIGNAL_CONSTRAINTS);
    //                 out.writeAttr(SUMO_ATTR_ID, s->getID());
    //                 for (auto item : s->getConstraints()) {
    //                     for (MSRailSignalConstraint* c : item.second) {
    //                         c->write(out, item.first);
    //                     }
    //                 }
    //                 out.closeTag();
    //             }
    //         }
    //     }
    //     MSRailSignalConstraint_Predecessor::saveState(out);
    // }
    #[allow(unused_variables)]
    pub fn save_state(out: &mut OutputDevice) {
        todo!("port MSRailSignalConstraint::saveState")
    }

    /// C++: `void MSRailSignalConstraint::clearState()` [static]
    /// Clear all constraint states before quick-loading state
    // void
    // MSRailSignalConstraint::clearState() {
    //     MSRailSignalConstraint_Predecessor::clearState();
    //     myTripIdLookup.clear();
    // }
    #[allow(unused_variables)]
    pub fn clear_state() {
        todo!("port MSRailSignalConstraint::clearState")
    }

    /// C++: `void MSRailSignalConstraint::clearAll()` [static]
    /// Remove all constraints before quick-loading state
    // void
    // MSRailSignalConstraint::clearAll() {
    //     for (MSRailSignal* s : MSRailSignalControl::getInstance().getSignals()) {
    //         s->removeConstraints();
    //     }
    //     myTripIdLookup.clear();
    // }
    #[allow(unused_variables)]
    pub fn clear_all() {
        todo!("port MSRailSignalConstraint::clearAll")
    }

    /// C++: `const SUMOVehicle * MSRailSignalConstraint::getVeh(const std::string &, bool)` [static, protected]
    // const SUMOVehicle*
    // MSRailSignalConstraint::getVeh(const std::string& tripID, bool checkID) {
    //     MSVehicleControl& c = MSNet::getInstance()->getVehicleControl();
    //     const std::string& vehID = lookupVehId(tripID);
    //     if (vehID != "") {
    //         SUMOVehicle* veh = c.getVehicle(vehID);
    //         if (veh != nullptr) {
    //             return veh;
    //         }
    //     }
    //     for (MSVehicleControl::constVehIt i = c.loadedVehBegin(); i != c.loadedVehEnd(); ++i) {
    //         SUMOVehicle* veh = i->second;
    //         if (veh->getParameter().getParameter("tripId") == tripID || (checkID && veh->getID() == tripID)) {
    //             return veh;
    //         }
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_veh(tripID: &str, checkID: bool) -> Ptr<SUMOVehicle> {
        todo!("port MSRailSignalConstraint::getVeh")
    }

}

/// C++ class `MSRailSignalConstraint_Predecessor` (microsim/traffic_lights/MSRailSignalConstraint.h)
pub struct MSRailSignalConstraint_Predecessor {
    /// C++ base class `MSRailSignalConstraint`
    pub base: MSRailSignalConstraint,
    /// the tracker object for this constraint
    pub myTrackers: Vec<Ptr<MSRailSignalConstraint_Predecessor_PassedTracker>>,
    /// id of the predecessor that must already have passed
    pub myTripId: String,
    /// the number of passed vehicles within which tripId must have occurred
    pub myLimit: i32,
    /// Whether this constraint is currently active
    pub myAmActive: bool,
    /// store the foe signal (for TraCI access)
    pub myFoeSignal: Ptr<MSRailSignal>,
}

impl std::ops::Deref for MSRailSignalConstraint_Predecessor {
    type Target = MSRailSignalConstraint;
    fn deref(&self) -> &MSRailSignalConstraint {
        &self.base
    }
}
impl std::ops::DerefMut for MSRailSignalConstraint_Predecessor {
    fn deref_mut(&mut self) -> &mut MSRailSignalConstraint {
        &mut self.base
    }
}

// C++ static members of MSRailSignalConstraint_Predecessor (port as module-level statics, see AGENTS.md):
//   static std::map<const MSLane *, PassedTracker *, ComparatorNumericalIdLess> MSRailSignalConstraint_Predecessor::myTrackerLookup;

impl MSRailSignalConstraint_Predecessor {
    /// C++: `void MSRailSignalConstraint_Predecessor::MSRailSignalConstraint_Predecessor(ConstraintType, const MSRailSignal *, const std::string &, int, bool)`
    /// Constructor
    // MSRailSignalConstraint_Predecessor::MSRailSignalConstraint_Predecessor(ConstraintType type, const MSRailSignal* signal, const std::string& tripId, int limit, bool active) :
    //     MSRailSignalConstraint(type),
    //     myTripId(tripId),
    //     myLimit(limit),
    //     myAmActive(active),
    //     myFoeSignal(signal) {
    //     for (const auto& lv : signal->getLinks()) {
    //         for (const MSLink* link : lv) {
    //             MSLane* lane = link->getViaLaneOrLane();
    //             PassedTracker* pt = nullptr;
    //             if (myTrackerLookup.count(lane) == 0) {
    //                 pt = new PassedTracker(lane);
    //                 myTrackerLookup[lane] = pt;
    //             } else {
    //                 pt = myTrackerLookup[lane];
    //             }
    //             pt->raiseLimit(limit);
    //             myTrackers.push_back(pt);
    //         }
    //     }
    //
    // }
    #[allow(unused_variables)]
    pub fn new(type_: MSRailSignalConstraint_ConstraintType, signal: Ptr<MSRailSignal>, tripId: &str, limit: i32, active: bool) -> Self {
        todo!("port MSRailSignalConstraint_Predecessor::MSRailSignalConstraint_Predecessor")
    }

    /// C++: `void MSRailSignalConstraint_Predecessor::~MSRailSignalConstraint_Predecessor() noexcept` [virtual]
    /// Destructor
    // ~MSRailSignalConstraint_Predecessor() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSRailSignalConstraint_Predecessor::~MSRailSignalConstraint_Predecessor")
    }

    /// C++: `void MSRailSignalConstraint_Predecessor::write(OutputDevice &, const std::string &) const` [const, virtual]
    // void
    // MSRailSignalConstraint_Predecessor::write(OutputDevice& out, const std::string& tripId) const {
    //     out.openTag(getTag());
    //     out.writeAttr(SUMO_ATTR_TRIP_ID, tripId);
    //     out.writeAttr(SUMO_ATTR_TLID, myFoeSignal->getID());
    //     out.writeAttr(SUMO_ATTR_FOES, myTripId);
    //     if (myLimit > 1) {
    //         out.writeAttr(SUMO_ATTR_LIMIT, myLimit);
    //     }
    //     if (!myAmActive) {
    //         out.writeAttr(SUMO_ATTR_ACTIVE, myAmActive);
    //     }
    //     writeParams(out);
    //     out.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn write(&self, out: &mut OutputDevice, tripId: &str) {
        todo!("port MSRailSignalConstraint_Predecessor::write")
    }

    /// C++: `void MSRailSignalConstraint_Predecessor::cleanup()` [static]
    /// clean up state
    // void
    // MSRailSignalConstraint_Predecessor::cleanup() {
    //     for (auto item : myTrackerLookup) {
    //         delete item.second;
    //     }
    //     myTrackerLookup.clear();
    // }
    #[allow(unused_variables)]
    pub fn cleanup() {
        todo!("port MSRailSignalConstraint_Predecessor::cleanup")
    }

    /// C++: `void MSRailSignalConstraint_Predecessor::saveState(OutputDevice &)` [static]
    /// Saves the current constraint states into the given stream
    // void
    // MSRailSignalConstraint_Predecessor::saveState(OutputDevice& out) {
    //     for (auto item : myTrackerLookup) {
    //         item.second->saveState(out);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn save_state(out: &mut OutputDevice) {
        todo!("port MSRailSignalConstraint_Predecessor::saveState")
    }

    /// C++: `void MSRailSignalConstraint_Predecessor::loadState(const SUMOSAXAttributes &)` [static]
    /// loads the constraint state from the given attrs
    // void
    // MSRailSignalConstraint_Predecessor::loadState(const SUMOSAXAttributes& attrs) {
    //     bool ok;
    //     const std::string laneID = attrs.getString(SUMO_ATTR_LANE);
    //     const int index = attrs.get<int>(SUMO_ATTR_INDEX, nullptr, ok);
    //     const std::vector<std::string>& tripIDs = attrs.get<std::vector<std::string> >(SUMO_ATTR_STATE, nullptr, ok);
    //     MSLane* lane = MSLane::dictionary(laneID);
    //     if (lane == nullptr) {
    //         throw ProcessError(TLF("Unknown lane '%' in loaded state.", laneID));
    //     }
    //     if (myTrackerLookup.count(lane) == 0) {
    //         WRITE_WARNINGF(TL("Unknown tracker lane '%' in loaded state."), laneID);
    //         return;
    //     }
    //     PassedTracker* tracker = myTrackerLookup[lane];
    //     tracker->loadState(index, tripIDs);
    // }
    #[allow(unused_variables)]
    pub fn load_state(attrs: &SUMOSAXAttributes) {
        todo!("port MSRailSignalConstraint_Predecessor::loadState")
    }

    /// C++: `void MSRailSignalConstraint_Predecessor::clearState()` [static]
    /// Clear all constraint states before quick-loading state
    // void
    // MSRailSignalConstraint_Predecessor::clearState() {
    //     for (auto item : myTrackerLookup) {
    //         item.second->clearState();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn clear_state() {
        todo!("port MSRailSignalConstraint_Predecessor::clearState")
    }

    /// C++: `bool MSRailSignalConstraint_Predecessor::cleared() const` [const, virtual]
    // bool
    // MSRailSignalConstraint_Predecessor::cleared() const {
    //     if (!myAmActive) {
    //         return true;
    //     }
    //     for (PassedTracker* pt : myTrackers) {
    //         if (pt->hasPassed(myTripId, myLimit)) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn cleared(&self) -> bool {
        todo!("port MSRailSignalConstraint_Predecessor::cleared")
    }

    /// C++: `void MSRailSignalConstraint_Predecessor::setActive(bool)` [virtual]
    // void setActive(bool active) {
    //         myAmActive = active;
    //     }
    #[allow(unused_variables)]
    pub fn set_active(&mut self, active: bool) {
        todo!("port MSRailSignalConstraint_Predecessor::setActive")
    }

    /// C++: `bool MSRailSignalConstraint_Predecessor::isActive() const` [const, virtual]
    // bool isActive() const {
    //         return myAmActive;
    //     }
    #[allow(unused_variables)]
    pub fn is_active(&self) -> bool {
        todo!("port MSRailSignalConstraint_Predecessor::isActive")
    }

    /// C++: `std::string MSRailSignalConstraint_Predecessor::getDescription() const` [const, virtual]
    // std::string
    // MSRailSignalConstraint_Predecessor::getDescription() const {
    //     // try to retrieve vehicle id that belongs to myTripId
    //     // this may be slow so it should only be used for debugging
    //     const SUMOVehicle* veh = getVeh(myTripId);
    //     std::string vehID;
    //     if (veh != nullptr) {
    //         vehID = " (" + veh->getID() + ")";
    //     }
    //     std::vector<std::string> passedIDs;
    //     for (const std::string& passedTripID : myTrackers.front()->myPassed) {
    //         if (passedTripID == "") {
    //             continue;
    //         }
    //         const SUMOVehicle* passedVeh = getVeh(passedTripID);
    //         if (passedVeh != nullptr) {
    //             passedIDs.push_back(passedVeh->getID());
    //         }
    //     }
    //     std::string passedIDs2 = "";
    //     if (passedIDs.size() > 0) {
    //         passedIDs2 = " (" + toString(passedIDs) + ")";
    //     }
    //     std::string params = "";
    //     for (auto item : getParametersMap()) {
    //         params += ("\n  key=" + item.first + " value=" + item.second);
    //     }
    //     return (toString(getTag()) + "  " + myTripId + vehID + " at signal " + myTrackers.front()->getLane()->getEdge().getFromJunction()->getID()
    //             + " passed=" + StringUtils::prune(toString(myTrackers.front()->myPassed)) + passedIDs2 + params);
    // }
    #[allow(unused_variables)]
    pub fn get_description(&self) -> String {
        todo!("port MSRailSignalConstraint_Predecessor::getDescription")
    }

    /// C++: `const SUMOVehicle * MSRailSignalConstraint_Predecessor::getFoe() const` [const, virtual]
    // const SUMOVehicle*
    // MSRailSignalConstraint_Predecessor::getFoe() const {
    //     return getVeh(myTripId, true);
    // }
    #[allow(unused_variables)]
    pub fn get_foe(&self) -> Ptr<SUMOVehicle> {
        todo!("port MSRailSignalConstraint_Predecessor::getFoe")
    }

    /// C++: `MSRailSignalConstraint_Predecessor & MSRailSignalConstraint_Predecessor::operator=(const MSRailSignalConstraint_Predecessor &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &MSRailSignalConstraint_Predecessor) -> &mut MSRailSignalConstraint_Predecessor {
        todo!("port MSRailSignalConstraint_Predecessor::operator=")
    }

}

/// C++ class `MSRailSignalConstraint_Predecessor::PassedTracker` (microsim/traffic_lights/MSRailSignalConstraint.h)
pub struct MSRailSignalConstraint_Predecessor_PassedTracker {
    /// C++ base class `MSMoveReminder`
    pub base: MSMoveReminder,
    /// passed tripIds
    pub myPassed: Vec<String>,
    /// index of the last passed object
    pub myLastIndex: i32,
}

impl std::ops::Deref for MSRailSignalConstraint_Predecessor_PassedTracker {
    type Target = MSMoveReminder;
    fn deref(&self) -> &MSMoveReminder {
        &self.base
    }
}
impl std::ops::DerefMut for MSRailSignalConstraint_Predecessor_PassedTracker {
    fn deref_mut(&mut self) -> &mut MSMoveReminder {
        &mut self.base
    }
}

impl MSRailSignalConstraint_Predecessor_PassedTracker {
    /// C++: `void PassedTracker::PassedTracker(MSLane *)`
    // MSRailSignalConstraint_Predecessor::PassedTracker::PassedTracker(MSLane* lane) :
    //     MSMoveReminder("PassedTracker_" + lane->getID(), lane, true),
    //     myPassed(1, ""),
    //     myLastIndex(-1)
    // { }
    #[allow(unused_variables)]
    pub fn new(lane: Ptr<MSLane>) -> Self {
        todo!("port PassedTracker::PassedTracker")
    }

    /// C++: `bool PassedTracker::notifyEnter(SUMOTrafficObject &, MSMoveReminder::Notification, const MSLane *)` [virtual]
    /// tracks vehicles that passed this link (entered the next lane)
    // bool
    // MSRailSignalConstraint_Predecessor::PassedTracker::notifyEnter(SUMOTrafficObject& veh, MSMoveReminder::Notification /*reason*/, const MSLane* /*enteredLane*/) {
    //     myLastIndex = (myLastIndex + 1) % myPassed.size();
    //     myPassed[myLastIndex] = veh.getParameter().getParameter("tripId", veh.getID());
    // #ifdef DEBUG_PASSED
    //     if (myLane->getID() == DEBUG_LANE) {
    //         std::cout << SIMTIME << " hasPassed " << veh.getID() << " tripId=" << veh.getParameter().getParameter("tripId", veh.getID()) << " index=" << myLastIndex << "\n";
    //     }
    // #endif
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn notify_enter(&mut self, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        todo!("port PassedTracker::notifyEnter")
    }

    /// C++: `void PassedTracker::raiseLimit(int)`
    // void
    // MSRailSignalConstraint_Predecessor::PassedTracker::raiseLimit(int limit) {
    //     while (limit > (int)myPassed.size()) {
    //         myPassed.insert(myPassed.begin() + (myLastIndex + 1), "");
    //     }
    // #ifdef DEBUG_PASSED
    //     if (myLane->getID() == DEBUG_LANE) {
    //         std::cout << " raiseLimit=" << limit << "\n";
    //     }
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn raise_limit(&mut self, limit: i32) {
        todo!("port PassedTracker::raiseLimit")
    }

    /// C++: `bool PassedTracker::hasPassed(const std::string &, int) const` [const]
    // bool
    // MSRailSignalConstraint_Predecessor::PassedTracker::hasPassed(const std::string& tripId, int limit) const {
    //     if (myLastIndex < 0) {
    //         return false;
    //     }
    //     int i = myLastIndex;
    //     while (limit > 0) {
    //         if (myPassed[i] == tripId) {
    //             return true;
    //         }
    //         if (i == 0) {
    //             i = (int)myPassed.size() - 1;
    //         } else {
    //             i--;
    //         }
    //         limit--;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn has_passed(&self, tripId: &str, limit: i32) -> bool {
        todo!("port PassedTracker::hasPassed")
    }

    /// C++: `void PassedTracker::clearState()`
    /// Clear all passed states before quick-loading state
    // void
    // MSRailSignalConstraint_Predecessor::PassedTracker::clearState() {
    //     myPassed = std::vector<std::string>(myPassed.size());
    //     myLastIndex = 0;
    // }
    #[allow(unused_variables)]
    pub fn clear_state(&mut self) {
        todo!("port PassedTracker::clearState")
    }

    /// C++: `void PassedTracker::saveState(OutputDevice &)`
    /// Saves the current passed states into the given stream
    // void
    // MSRailSignalConstraint_Predecessor::PassedTracker::saveState(OutputDevice& out) {
    //     const std::string state = toString(myPassed.back() == ""
    //                                        ? std::vector<std::string>(myPassed.begin(), myPassed.begin() + (myLastIndex + 1))
    //                                        // wrapped around
    //                                        : myPassed);
    //     // no need to save state if no vehicles have passed this tracker
    //     if (state != "") {
    //         out.openTag(SUMO_TAG_RAILSIGNAL_CONSTRAINT_TRACKER);
    //         out.writeAttr(SUMO_ATTR_LANE, getLane()->getID());
    //         out.writeAttr(SUMO_ATTR_INDEX, myLastIndex);
    //         out.writeAttr(SUMO_ATTR_STATE, state);
    //         out.closeTag();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn save_state(&mut self, out: &mut OutputDevice) {
        todo!("port PassedTracker::saveState")
    }

    /// C++: `void PassedTracker::loadState(int, const std::vector<std::string> &)`
    /// loads the current passed states into the given stream
    // void
    // MSRailSignalConstraint_Predecessor::PassedTracker::loadState(int index, const std::vector<std::string>& tripIDs) {
    //     raiseLimit((int)tripIDs.size());
    //     for (int i = 0; i < (int)tripIDs.size(); i++) {
    //         myPassed[i] = tripIDs[i];
    //     }
    // #ifdef DEBUG_PASSED
    //     if (myLane->getID() == DEBUG_LANE) {
    //         std::cout << " loadState limit=" << tripIDs.size() << " index=" << index << "\n";
    //         for (int i = 0; i < (int)myPassed.size(); i++) {
    //             std::cout << " i=" << i << " passed=" << myPassed[i] << "\n";
    //         }
    //     }
    // #endif
    //     myLastIndex = index;
    // }
    #[allow(unused_variables)]
    pub fn load_state(&mut self, index: i32, tripIDs: &Vec<String>) {
        todo!("port PassedTracker::loadState")
    }

}

