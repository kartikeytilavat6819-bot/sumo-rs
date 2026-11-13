//! Port of microsim/traffic_lights/MSRailSignalConstraint.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use std::cell::RefCell;

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
#[repr(C)]
pub struct MSRailSignalConstraint {
    /// C++ base class `Parameterised`
    pub base: Parameterised,
    pub myType: MSRailSignalConstraint_ConstraintType,
    /// C++ resolves the virtual methods of this class by dynamic dispatch; the Rust port
    /// emulates that by casting to the only concrete subclass (MSRailSignalConstraint_Predecessor)
    pub myIsPredecessor: bool,
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

thread_local! {
    // static std::map<std::string, std::string> MSRailSignalConstraint::myTripIdLookup
    // the values are leaked so that lookupVehId can hand out a reference like the C++ version
    static MY_TRIP_ID_LOOKUP: RefCell<BTreeMap<String, &'static str>> = RefCell::new(BTreeMap::new());
}

impl MSRailSignalConstraint {
    /// C++: `void MSRailSignalConstraint::MSRailSignalConstraint(ConstraintType)`
    /// Constructor
    // MSRailSignalConstraint(ConstraintType type) : myType(type) {}
    #[allow(unused_variables)]
    pub fn new(type_: MSRailSignalConstraint_ConstraintType) -> Self {
        MSRailSignalConstraint {
            base: Parameterised::new(),
            myType: type_,
            myIsPredecessor: false,
        }
    }

    /// C++: `void MSRailSignalConstraint::~MSRailSignalConstraint() noexcept` [virtual]
    /// Destructor
    // virtual ~MSRailSignalConstraint() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `bool MSRailSignalConstraint::cleared() const` [const, virtual, pure virtual]
    /// whether the constraint has been met
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn cleared(&self) -> bool {
        // C++ dispatches virtually; the port emulates that by casting to the only concrete subclass
        if self.myIsPredecessor {
            return Ptr::new(self).cast::<MSRailSignalConstraint_Predecessor>().cleared();
        }
        panic!("pure virtual MSRailSignalConstraint::cleared")
    }

    /// C++: `void MSRailSignalConstraint::setActive(bool)` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn set_active(&mut self, active: bool) {
        // C++ dispatches virtually; the port emulates that by casting to the only concrete subclass
        if self.myIsPredecessor {
            return Ptr::new(&*self).cast::<MSRailSignalConstraint_Predecessor>().m().set_active(active);
        }
        panic!("pure virtual MSRailSignalConstraint::setActive")
    }

    /// C++: `bool MSRailSignalConstraint::isActive() const` [const, virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn is_active(&self) -> bool {
        // C++ dispatches virtually; the port emulates that by casting to the only concrete subclass
        if self.myIsPredecessor {
            return Ptr::new(self).cast::<MSRailSignalConstraint_Predecessor>().is_active();
        }
        panic!("pure virtual MSRailSignalConstraint::isActive")
    }

    /// C++: `std::string MSRailSignalConstraint::getDescription() const` [const, virtual]
    // virtual std::string getDescription() const {
    //         return "RailSignalConstraint";
    //     }
    #[allow(unused_variables)]
    pub fn get_description(&self) -> String {
        // C++ dispatches virtually; the port emulates that by casting to the only concrete subclass
        if self.myIsPredecessor {
            return Ptr::new(self).cast::<MSRailSignalConstraint_Predecessor>().get_description();
        }
        "RailSignalConstraint".to_string()
    }

    /// C++: `const SUMOVehicle * MSRailSignalConstraint::getFoe() const` [const, virtual]
    // virtual const SUMOVehicle* getFoe() const {
    //         return nullptr;
    //     }
    #[allow(unused_variables)]
    pub fn get_foe(&self) -> Ptr<SUMOVehicle> {
        // C++ dispatches virtually; the port emulates that by casting to the only concrete subclass
        if self.myIsPredecessor {
            return Ptr::new(self).cast::<MSRailSignalConstraint_Predecessor>().get_foe();
        }
        Ptr::null()
    }

    /// C++: `void MSRailSignalConstraint::write(OutputDevice &, const std::string &) const` [const, virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn write(&self, out: &mut OutputDevice, tripId: &str) {
        // C++ dispatches virtually; the port emulates that by casting to the only concrete subclass
        if self.myIsPredecessor {
            return Ptr::new(self).cast::<MSRailSignalConstraint_Predecessor>().write(out, tripId);
        }
        panic!("pure virtual MSRailSignalConstraint::write")
    }

    /// C++: `ConstraintType MSRailSignalConstraint::getType() const` [const]
    // ConstraintType getType() const {
    //         return myType;
    //     }
    #[allow(unused_variables)]
    pub fn get_type(&self) -> MSRailSignalConstraint_ConstraintType {
        self.myType
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
        match self.myType {
            MSRailSignalConstraint_ConstraintType::INSERTION_PREDECESSOR => SUMO_TAG_INSERTION_PREDECESSOR,
            MSRailSignalConstraint_ConstraintType::FOE_INSERTION => SUMO_TAG_FOE_INSERTION,
            MSRailSignalConstraint_ConstraintType::INSERTION_ORDER => SUMO_TAG_INSERTION_ORDER,
            MSRailSignalConstraint_ConstraintType::BIDI_PREDECESSOR => SUMO_TAG_BIDI_PREDECESSOR,
            _ => SUMO_TAG_PREDECESSOR,
        }
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
        match self.myType {
            MSRailSignalConstraint_ConstraintType::INSERTION_PREDECESSOR => MSRailSignalConstraint_ConstraintType::FOE_INSERTION,
            MSRailSignalConstraint_ConstraintType::FOE_INSERTION => MSRailSignalConstraint_ConstraintType::INSERTION_PREDECESSOR,
            _ => self.myType,
        }
    }

    /// C++: `bool MSRailSignalConstraint::isInsertionConstraint() const` [const]
    // bool isInsertionConstraint() const {
    //         return myType == INSERTION_PREDECESSOR || myType == INSERTION_ORDER;
    //     }
    #[allow(unused_variables)]
    pub fn is_insertion_constraint(&self) -> bool {
        self.myType == MSRailSignalConstraint_ConstraintType::INSERTION_PREDECESSOR
            || self.myType == MSRailSignalConstraint_ConstraintType::INSERTION_ORDER
    }

    /// C++: `void MSRailSignalConstraint::storeTripId(const std::string &, const std::string &)` [static]
    // void
    // MSRailSignalConstraint::storeTripId(const std::string& tripId, const std::string& vehID) {
    //     myTripIdLookup[tripId] = vehID;
    // }
    #[allow(unused_variables)]
    pub fn store_trip_id(tripId: &str, vehID: &str) {
        // the value is leaked so that lookup_veh_id can return a reference like the C++ version
        let value: &'static str = Box::leak(vehID.to_string().into_boxed_str());
        MY_TRIP_ID_LOOKUP.with(|m| {
            m.borrow_mut().insert(tripId.to_string(), value);
        });
    }

    /// C++: `const std::string & MSRailSignalConstraint::lookupVehId(const std::string &)` [static]
    // const std::string&
    // MSRailSignalConstraint::lookupVehId(const std::string& tripId) {
    //     return myTripIdLookup[tripId];
    // }
    #[allow(unused_variables)]
    pub fn lookup_veh_id(tripId: &str) -> &'static str {
        // C++ uses operator[] which inserts an empty entry for unknown trip ids
        MY_TRIP_ID_LOOKUP.with(|m| {
            let mut map = m.borrow_mut();
            *map.entry(tripId.to_string()).or_insert("")
        })
    }

    /// C++: `void MSRailSignalConstraint::cleanup()` [static]
    /// clean up state
    // void
    // MSRailSignalConstraint::cleanup() {
    //     MSRailSignalConstraint_Predecessor::cleanup();
    // }
    #[allow(unused_variables)]
    pub fn cleanup() {
        MSRailSignalConstraint_Predecessor::cleanup();
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
        if utils::options::options_cont::get_bool("save-state.constraints") {
            for s in MSRailSignalControl::get_instance().get_signals() {
                if s.get_constraints().len() > 0 {
                    out.open_tag_id(SUMO_TAG_RAILSIGNAL_CONSTRAINTS);
                    out.write_attr(SUMO_ATTR_ID, s.get_id());
                    for (trip_id, constraints) in s.get_constraints() {
                        for c in constraints {
                            c.get().unwrap().write(out, trip_id);
                        }
                    }
                    out.close_tag("");
                }
            }
        }
        MSRailSignalConstraint_Predecessor::save_state(out);
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
        MSRailSignalConstraint_Predecessor::clear_state();
        MY_TRIP_ID_LOOKUP.with(|m| m.borrow_mut().clear());
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
        let signals: Vec<Ptr<MSRailSignal>> = MSRailSignalControl::get_instance().get_signals().clone();
        for s in signals {
            s.m().remove_constraints();
        }
        MY_TRIP_ID_LOOKUP.with(|m| m.borrow_mut().clear());
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
        let veh_id = Self::lookup_veh_id(tripID);
        if veh_id != "" {
            let veh = {
                let net = MSNet::get_instance();
                let control = net.m().get_vehicle_control();
                control.get_vehicle(veh_id)
            };
            if !veh.is_null() {
                return veh;
            }
        }
        let found = {
            let net = MSNet::get_instance();
            let control = net.m().get_vehicle_control();
            let mut result = Ptr::null();
            for veh in control.myVehicleDict.values() {
                let base = veh.cast::<MSBaseVehicle>();
                let trip_id = base.get_parameter().get_parameter("tripId", "");
                if trip_id == tripID || (checkID && veh.get_id() == tripID) {
                    result = *veh;
                    break;
                }
            }
            result
        };
        if !found.is_null() {
            return found;
        }
        Ptr::null()
    }

}

/// C++ class `MSRailSignalConstraint_Predecessor` (microsim/traffic_lights/MSRailSignalConstraint.h)
#[repr(C)]
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

thread_local! {
    // static std::map<const MSLane *, PassedTracker *, ComparatorNumericalIdLess> MSRailSignalConstraint_Predecessor::myTrackerLookup
    // determinism: C++ orders the trackers by lane numerical id, the Rust map orders by address
    static MY_TRACKER_LOOKUP: RefCell<BTreeMap<Ptr<MSLane>, Ptr<MSRailSignalConstraint_Predecessor_PassedTracker>>> =
        RefCell::new(BTreeMap::new());
}

/// creates a heap allocated tracker and registers it with its lane (as the C++ MSMoveReminder constructor does)
fn new_passed_tracker(lane: Ptr<MSLane>) -> Ptr<MSRailSignalConstraint_Predecessor_PassedTracker> {
    let mut boxed = Box::new(MSRailSignalConstraint_Predecessor_PassedTracker::new(lane));
    let ptr = Ptr::from_box(&boxed);
    lane.m().add_move_reminder(ptr.cast::<MSMoveReminder>(), true);
    Ptr::from_raw(Box::into_raw(boxed))
}

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
        let mut result = MSRailSignalConstraint_Predecessor {
            base: MSRailSignalConstraint::new(type_),
            myTrackers: Vec::new(),
            myTripId: tripId.to_string(),
            myLimit: limit,
            myAmActive: active,
            myFoeSignal: signal,
        };
        result.base.myIsPredecessor = true;
        for lv in signal.get_links() {
            for link in lv {
                let lane = link.get_via_lane_or_lane();
                let pt = match MY_TRACKER_LOOKUP.with(|m| m.borrow().get(&lane).copied()) {
                    Some(pt) => pt,
                    None => {
                        let pt = new_passed_tracker(lane);
                        MY_TRACKER_LOOKUP.with(|m| m.borrow_mut().insert(lane, pt));
                        pt
                    }
                };
                pt.m().raise_limit(limit);
                result.myTrackers.push(pt);
            }
        }
        result
    }

    /// C++: `void MSRailSignalConstraint_Predecessor::~MSRailSignalConstraint_Predecessor() noexcept` [virtual]
    /// Destructor
    // ~MSRailSignalConstraint_Predecessor() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
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
        out.open_tag_id(self.get_tag());
        out.write_attr(SUMO_ATTR_TRIP_ID, &tripId);
        out.write_attr(SUMO_ATTR_TLID, self.myFoeSignal.get_id());
        out.write_attr(SUMO_ATTR_FOES, &self.myTripId);
        if self.myLimit > 1 {
            out.write_attr(SUMO_ATTR_LIMIT, &self.myLimit);
        }
        if !self.myAmActive {
            out.write_attr(SUMO_ATTR_ACTIVE, &self.myAmActive);
        }
        self.write_params(out);
        out.close_tag("");
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
        let trackers: Vec<Ptr<MSRailSignalConstraint_Predecessor_PassedTracker>> =
            MY_TRACKER_LOOKUP.with(|m| m.borrow().values().copied().collect());
        for tracker in trackers {
            unsafe { drop(Box::from_raw(tracker.as_raw())) };
        }
        MY_TRACKER_LOOKUP.with(|m| m.borrow_mut().clear());
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
        let trackers: Vec<Ptr<MSRailSignalConstraint_Predecessor_PassedTracker>> =
            MY_TRACKER_LOOKUP.with(|m| m.borrow().values().copied().collect());
        for tracker in trackers {
            tracker.m().save_state(out);
        }
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
        let mut ok = false;
        let lane_id = attrs.get_string(SUMO_ATTR_LANE);
        let index = attrs.get::<i32>(SUMO_ATTR_INDEX, "", &mut ok, true);
        let trip_ids = attrs.get::<Vec<String>>(SUMO_ATTR_STATE, "", &mut ok, true);
        let lane = MSLane::dictionary_2(&lane_id);
        if lane.is_null() {
            panic!("{}", ProcessError::new(utils::tlf!("Unknown lane '%' in loaded state.", lane_id)));
        }
        let tracker = MY_TRACKER_LOOKUP.with(|m| m.borrow().get(&lane).copied());
        match tracker {
            None => {
                utils::write_warningf!("Unknown tracker lane '%' in loaded state.", lane_id);
            }
            Some(tracker) => tracker.m().load_state(index, &trip_ids),
        }
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
        let trackers: Vec<Ptr<MSRailSignalConstraint_Predecessor_PassedTracker>> =
            MY_TRACKER_LOOKUP.with(|m| m.borrow().values().copied().collect());
        for tracker in trackers {
            tracker.m().clear_state();
        }
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
        if !self.myAmActive {
            return true;
        }
        for pt in &self.myTrackers {
            if pt.has_passed(&self.myTripId, self.myLimit) {
                return true;
            }
        }
        false
    }

    /// C++: `void MSRailSignalConstraint_Predecessor::setActive(bool)` [virtual]
    // void setActive(bool active) {
    //         myAmActive = active;
    //     }
    #[allow(unused_variables)]
    pub fn set_active(&mut self, active: bool) {
        self.myAmActive = active;
    }

    /// C++: `bool MSRailSignalConstraint_Predecessor::isActive() const` [const, virtual]
    // bool isActive() const {
    //         return myAmActive;
    //     }
    #[allow(unused_variables)]
    pub fn is_active(&self) -> bool {
        self.myAmActive
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
        // try to retrieve vehicle id that belongs to myTripId
        // this may be slow so it should only be used for debugging
        let veh = MSRailSignalConstraint::get_veh(&self.myTripId, false);
        let mut veh_id = String::new();
        if !veh.is_null() {
            veh_id = " (".to_string() + veh.get_id() + ")";
        }
        let mut passed_ids: Vec<String> = Vec::new();
        for passed_trip_id in &self.myTrackers[0].myPassed {
            if passed_trip_id == "" {
                continue;
            }
            let passed_veh = MSRailSignalConstraint::get_veh(passed_trip_id, false);
            if !passed_veh.is_null() {
                passed_ids.push(passed_veh.get_id().to_string());
            }
        }
        let mut passed_ids2 = String::new();
        if passed_ids.len() > 0 {
            passed_ids2 = " (".to_string() + &passed_ids.join(" ") + ")";
        }
        let mut params = String::new();
        for item in self.get_parameters_map() {
            params += &("\n  key=".to_string() + &item.0 + " value=" + &item.1);
        }
        let junction = self.myTrackers[0].get_lane().myEdge.get_from_junction().get_id().to_string();
        let tag = utils::common::to_string::to_string(&self.get_tag());
        format!(
            "{}{}{} at signal {} passed={}{}{}",
            tag,
            "  ",
            self.myTripId.clone() + &veh_id,
            junction,
            utils::common::string_utils::prune(&self.myTrackers[0].myPassed.join(" ")),
            passed_ids2,
            params
        )
    }

    /// C++: `const SUMOVehicle * MSRailSignalConstraint_Predecessor::getFoe() const` [const, virtual]
    // const SUMOVehicle*
    // MSRailSignalConstraint_Predecessor::getFoe() const {
    //     return getVeh(myTripId, true);
    // }
    #[allow(unused_variables)]
    pub fn get_foe(&self) -> Ptr<SUMOVehicle> {
        MSRailSignalConstraint::get_veh(&self.myTripId, true)
    }

    /// C++: `MSRailSignalConstraint_Predecessor & MSRailSignalConstraint_Predecessor::operator=(const MSRailSignalConstraint_Predecessor &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &MSRailSignalConstraint_Predecessor) -> &mut MSRailSignalConstraint_Predecessor {
        panic!("MSRailSignalConstraint_Predecessor::operator= is deleted in C++")
    }

}

/// C++ class `MSRailSignalConstraint_Predecessor::PassedTracker` (microsim/traffic_lights/MSRailSignalConstraint.h)
#[repr(C)]
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
        MSRailSignalConstraint_Predecessor_PassedTracker {
            base: MSMoveReminder::new(&format!("PassedTracker_{}", lane.get_id()), lane, true),
            myPassed: vec![String::new()],
            myLastIndex: -1,
        }
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
        self.myLastIndex = (self.myLastIndex + 1) % self.myPassed.len() as i32;
        // the traffic object is a vehicle, so its parameter is the vehicle parameter
        let base = Ptr::from_mut(veh).cast::<MSBaseVehicle>();
        let trip_id = base.get_parameter().get_parameter("tripId", base.get_id());
        self.myPassed[self.myLastIndex as usize] = trip_id;
        true
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
        while limit > self.myPassed.len() as i32 {
            self.myPassed.insert(self.myLastIndex as usize + 1, String::new());
        }
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
        if self.myLastIndex < 0 {
            return false;
        }
        let mut i = self.myLastIndex;
        let mut limit = limit;
        while limit > 0 {
            if self.myPassed[i as usize] == tripId {
                return true;
            }
            if i == 0 {
                i = self.myPassed.len() as i32 - 1;
            } else {
                i -= 1;
            }
            limit -= 1;
        }
        false
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
        self.myPassed = vec![String::new(); self.myPassed.len()];
        self.myLastIndex = 0;
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
        // wrapped around
        let wrapped = self.myPassed.last().map_or(false, |s| s.is_empty());
        let state = if wrapped {
            self.myPassed[..(self.myLastIndex + 1) as usize].join(" ")
        } else {
            self.myPassed.join(" ")
        };
        // no need to save state if no vehicles have passed this tracker
        if state != "" {
            out.open_tag_id(SUMO_TAG_RAILSIGNAL_CONSTRAINT_TRACKER);
            out.write_attr(SUMO_ATTR_LANE, self.get_lane().get_id());
            out.write_attr(SUMO_ATTR_INDEX, &self.myLastIndex);
            out.write_attr(SUMO_ATTR_STATE, &state);
            out.close_tag("");
        }
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
        self.raise_limit(tripIDs.len() as i32);
        for i in 0..tripIDs.len() {
            self.myPassed[i] = tripIDs[i].clone();
        }
        self.myLastIndex = index;
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    /// the tag / swapped type / insertion flag mapping of the constraint types
    #[test]
    fn constraint_type_mapping() {
        let c = MSRailSignalConstraint::new(MSRailSignalConstraint_ConstraintType::PREDECESSOR);
        assert_eq!(c.get_tag(), SUMO_TAG_PREDECESSOR);
        assert_eq!(c.get_swapped_type(), MSRailSignalConstraint_ConstraintType::PREDECESSOR);
        assert!(!c.is_insertion_constraint());

        let c = MSRailSignalConstraint::new(MSRailSignalConstraint_ConstraintType::INSERTION_PREDECESSOR);
        assert_eq!(c.get_tag(), SUMO_TAG_INSERTION_PREDECESSOR);
        assert_eq!(c.get_swapped_type(), MSRailSignalConstraint_ConstraintType::FOE_INSERTION);
        assert!(c.is_insertion_constraint());

        let c = MSRailSignalConstraint::new(MSRailSignalConstraint_ConstraintType::FOE_INSERTION);
        assert_eq!(c.get_tag(), SUMO_TAG_FOE_INSERTION);
        assert_eq!(c.get_swapped_type(), MSRailSignalConstraint_ConstraintType::INSERTION_PREDECESSOR);
        assert!(!c.is_insertion_constraint());

        let c = MSRailSignalConstraint::new(MSRailSignalConstraint_ConstraintType::INSERTION_ORDER);
        assert_eq!(c.get_tag(), SUMO_TAG_INSERTION_ORDER);
        assert_eq!(c.get_swapped_type(), MSRailSignalConstraint_ConstraintType::INSERTION_ORDER);
        assert!(c.is_insertion_constraint());

        let c = MSRailSignalConstraint::new(MSRailSignalConstraint_ConstraintType::BIDI_PREDECESSOR);
        assert_eq!(c.get_tag(), SUMO_TAG_BIDI_PREDECESSOR);
        assert_eq!(c.get_swapped_type(), MSRailSignalConstraint_ConstraintType::BIDI_PREDECESSOR);
        assert!(!c.is_insertion_constraint());
    }

    /// the trip id lookup table (C++ uses operator[], so unknown ids are inserted as empty)
    #[test]
    fn trip_id_lookup() {
        MSRailSignalConstraint::store_trip_id("trip0", "veh0");
        assert_eq!(MSRailSignalConstraint::lookup_veh_id("trip0"), "veh0");
        assert_eq!(MSRailSignalConstraint::lookup_veh_id("unknown"), "");
    }
}

