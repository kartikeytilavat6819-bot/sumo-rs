//! Port of microsim/traffic_lights/MSDriveWay.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::pair<const SUMOVehicle *const, const MSLink::ApproachingVehicleInformation> MSDriveWay::Approaching`
pub type MSDriveWay_Approaching = (Ptr<SUMOVehicle>, MSLink_ApproachingVehicleInformation);

/// C++ typedef `std::set<const MSLane *, ComparatorNumericalIdLess> MSDriveWay::LaneSet`
pub type MSDriveWay_LaneSet = BTreeSet<Ptr<MSLane>>;

/// C++ typedef `std::map<const MSLane *, int, ComparatorNumericalIdLess> MSDriveWay::LaneVisitedMap`
pub type MSDriveWay_LaneVisitedMap = BTreeMap<Ptr<MSLane>, i32>;

/// C++ class `MSDriveWay` (microsim/traffic_lights/MSDriveWay.h)
#[repr(C)]
pub struct MSDriveWay {
    /// C++ base class `MSMoveReminder`
    pub base: MSMoveReminder,
    /// C++ base class `Named`
    pub base2: Named,
    /// global driveway index
    pub myNumericalID: i32,
    /// the link that enters this driveway or nullptr for a departure driveWay
    pub myOrigin: Ptr<MSLink>,
    /// whether the current signal is switched green for a train approaching this block
    pub myActive: Ptr<SUMOVehicle>,
    /// list of edges for matching against train routes
    pub myRoute: Vec<Ptr<MSEdge>>,
    /// number of edges in myRoute where overlap with other driveways is forbidden
    pub myCoreSize: i32,
    /// number of normal edges in myForward
    pub myForwardEdgeCount: i32,
    /// whether this driveway ends its forward section with a rail signal (and thus comprises a full block)
    pub myFoundSignal: bool,
    pub myFoundJump: bool,
    pub myTerminateRoute: bool,
    /// whether driveway building was aborted due to MAX_BLOCK_LENGTH
    pub myAbortedBuild: bool,
    /// whether driveway building was aborted when no further bidi edge was found
    pub myBidiEnded: bool,
    /// set to parent driveway if this is a subDriveway, nullptr otherwise
    pub myParent: Ptr<MSDriveWay>,
    pub myForward: Vec<Ptr<MSLane>>,
    pub myBidi: Vec<Ptr<MSLane>>,
    pub myBidiExtended: Vec<Ptr<MSLane>>,
    pub myBidiExtendedSorted: Vec<Ptr<MSLane>>,
    pub myFlank: Vec<Ptr<MSLane>>,
    /// the lanes that must be clear of trains before this signal can switch to green
    pub myConflictLanes: Vec<Ptr<MSLane>>,
    pub myConflictLinks: Vec<Ptr<MSLink>>,
    pub myTrains: BTreeSet<Ptr<SUMOVehicle>>, // C++: std::set<SUMOVehicle *, ComparatorNumericalIdLess>
    pub myVehicleEvents: Vec<MSDriveWay_VehicleEvent>,
    pub myFoes: Vec<Ptr<MSDriveWay>>,
    pub mySidings: BTreeMap<Ptr<MSDriveWay>, Vec<MSDriveWay_Siding>>, // C++: std::map<const MSDriveWay *, std::vector<Siding>, ComparatorIdLess>
    pub myDeadlocks: Vec<BTreeSet<Ptr<MSDriveWay>>>,
    pub mySubDriveWays: Vec<Ptr<MSDriveWay>>,
    /// track own occurences in myReversalDriveWays for cleanup in destructor
    pub myReversals: Vec<Ptr<MSEdge>>,
    /// the first vehicle using this driveway
    pub myFirstVehicle: String,
}

impl std::ops::Deref for MSDriveWay {
    type Target = MSMoveReminder;
    fn deref(&self) -> &MSMoveReminder {
        &self.base
    }
}
impl std::ops::DerefMut for MSDriveWay {
    fn deref_mut(&mut self) -> &mut MSMoveReminder {
        &mut self.base
    }
}

// C++ static members of MSDriveWay (port as module-level statics, see AGENTS.md):
//   static int MSDriveWay::myGlobalDriveWayIndex;
//   static bool MSDriveWay::myWriteVehicles;
//   static double MSDriveWay::myMovingBlockMaxDist;
//   static std::set<const MSEdge *> MSDriveWay::myBlockLengthWarnings;
//   static std::map<const MSLink *, std::vector<MSDriveWay *>> MSDriveWay::mySwitchDriveWays;
//   static std::map<const MSEdge *, std::vector<MSDriveWay *>> MSDriveWay::myReversalDriveWays;
//   static std::map<const MSEdge *, std::vector<MSDriveWay *>, ComparatorNumericalIdLess> MSDriveWay::myDepartureDriveways;
//   static std::map<const MSJunction *, int> MSDriveWay::myDepartDrivewayIndex;
//   static std::map<const MSEdge *, std::vector<MSDriveWay *>> MSDriveWay::myDepartureDrivewaysEnds;
//   static std::map<const MSEdge *, std::vector<MSDriveWay *>, ComparatorNumericalIdLess> MSDriveWay::myEndingDriveways;
//   static std::map<const MSEdge *, std::vector<std::pair<MSDriveWay *, MSDriveWay *>>, ComparatorNumericalIdLess> MSDriveWay::mySidingDriveways;
//   static std::vector<const MSEdge *> MSDriveWay::myCurrentSidingCheck;
//   static std::map<ConstMSEdgeVector, MSDriveWay *> MSDriveWay::myDriveWayRouteLookup;
//   static std::map<std::string, MSDriveWay *> MSDriveWay::myDriveWayLookup;
//   static const std::vector<MSDriveWay *> MSDriveWay::EMPTY_DRIVEWAY_VECTOR;

thread_local! {
    /// C++ `MSDriveWay::myGlobalDriveWayIndex`
    static MY_GLOBAL_DRIVE_WAY_INDEX: std::cell::Cell<i32> = const { std::cell::Cell::new(0) };
    /// C++ `MSDriveWay::myWriteVehicles`
    static MY_WRITE_VEHICLES: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    /// C++ `MSDriveWay::myMovingBlockMaxDist`
    static MY_MOVING_BLOCK_MAX_DIST: std::cell::Cell<f64> = const { std::cell::Cell::new(1e10) };
    /// C++ `MSDriveWay::myBlockLengthWarnings`
    static MY_BLOCK_LENGTH_WARNINGS: std::cell::RefCell<BTreeSet<Ptr<MSEdge>>> = std::cell::RefCell::new(BTreeSet::new());
    /// C++ `MSDriveWay::mySwitchDriveWays`
    static MY_SWITCH_DRIVE_WAYS: std::cell::RefCell<BTreeMap<Ptr<MSLink>, Vec<Ptr<MSDriveWay>>>> = std::cell::RefCell::new(BTreeMap::new());
    /// C++ `MSDriveWay::myReversalDriveWays`
    static MY_REVERSAL_DRIVE_WAYS: std::cell::RefCell<BTreeMap<Ptr<MSEdge>, Vec<Ptr<MSDriveWay>>>> = std::cell::RefCell::new(BTreeMap::new());
    /// C++ `MSDriveWay::myDepartureDriveways`
    static MY_DEPARTURE_DRIVEWAYS: std::cell::RefCell<BTreeMap<Ptr<MSEdge>, Vec<Ptr<MSDriveWay>>>> = std::cell::RefCell::new(BTreeMap::new());
    /// C++ `MSDriveWay::myDepartDrivewayIndex`
    static MY_DEPART_DRIVEWAY_INDEX: std::cell::RefCell<BTreeMap<Ptr<MSJunction>, i32>> = std::cell::RefCell::new(BTreeMap::new());
    /// C++ `MSDriveWay::myDepartureDrivewaysEnds`
    static MY_DEPARTURE_DRIVEWAYS_ENDS: std::cell::RefCell<BTreeMap<Ptr<MSEdge>, Vec<Ptr<MSDriveWay>>>> = std::cell::RefCell::new(BTreeMap::new());
    /// C++ `MSDriveWay::myEndingDriveways`
    static MY_ENDING_DRIVEWAYS: std::cell::RefCell<BTreeMap<Ptr<MSEdge>, Vec<Ptr<MSDriveWay>>>> = std::cell::RefCell::new(BTreeMap::new());
    /// C++ `MSDriveWay::mySidingDriveways`
    static MY_SIDING_DRIVEWAYS: std::cell::RefCell<BTreeMap<Ptr<MSEdge>, Vec<(Ptr<MSDriveWay>, Ptr<MSDriveWay>)>>> = std::cell::RefCell::new(BTreeMap::new());
    /// C++ `MSDriveWay::myCurrentSidingCheck`
    static MY_CURRENT_SIDING_CHECK: std::cell::RefCell<Vec<Ptr<MSEdge>>> = std::cell::RefCell::new(Vec::new());
    /// C++ `MSDriveWay::myDriveWayRouteLookup`
    static MY_DRIVE_WAY_ROUTE_LOOKUP: std::cell::RefCell<BTreeMap<Vec<Ptr<MSEdge>>, Ptr<MSDriveWay>>> = std::cell::RefCell::new(BTreeMap::new());
    /// C++ `MSDriveWay::myDriveWayLookup`
    static MY_DRIVE_WAY_LOOKUP: std::cell::RefCell<BTreeMap<String, Ptr<MSDriveWay>>> = std::cell::RefCell::new(BTreeMap::new());
}
/// C++ `MSDriveWay::EMPTY_DRIVEWAY_VECTOR`
// C++ MSDriveWay::EMPTY_DRIVEWAY_VECTOR is only ever returned empty; no Rust static needed (raw pointers are not Sync).

// determinism: C++ orders myDepartureDriveways/myEndingDriveways/mySidingDriveways by edge numerical id
// (ComparatorNumericalIdLess) while the BTreeMap here orders by pointer address; this only affects the order
// of output-only iterations (writeDepatureBlocks, saveState).
// determinism: C++ orders mySidings by driveway id (ComparatorIdLess) while the BTreeMap here orders by pointer
// address; this only affects the order of the "sidings" tags in the block output.

/// `toString(std::vector<const MSEdge*>)`: the edge ids joined by " ".
fn edge_ids(v: &[Ptr<MSEdge>]) -> String {
    v.iter().map(|e| e.get_id().to_string()).collect::<Vec<_>>().join(" ")
}

/// `toString(std::vector<const MSLane*>)`: the lane ids joined by " ".
fn lane_ids(v: &[Ptr<MSLane>]) -> String {
    v.iter().map(|l| l.get_id().to_string()).collect::<Vec<_>>().join(" ")
}

/// `joinNamedToStringSorting(std::set<const MSDriveWay*>, " ")`: the sorted driveway ids joined by " ".
fn drive_way_ids_sorted(v: &BTreeSet<Ptr<MSDriveWay>>) -> String {
    let mut ids: Vec<String> = v.iter().map(|d| d.base2.get_id().to_string()).collect();
    ids.sort();
    ids.join(" ")
}

/// `std::lower_bound(v.begin(), v.end(), key)` for the pointer-sorted vector v: the first element that is
/// not less than key.
fn lower_bound_lane(v: &[Ptr<MSLane>], key: Ptr<MSLane>) -> Option<&Ptr<MSLane>> {
    let mut lo = 0usize;
    let mut hi = v.len();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if v[mid] < key {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    if lo < v.len() { Some(&v[lo]) } else { None }
}

/// `MSMoveReminder::Notifications.getString(reason)`
fn notification_string(reason: MSMoveReminder_Notification) -> &'static str {
    match reason {
        MSMoveReminder_Notification::NOTIFICATION_DEPARTED => "departed",
        MSMoveReminder_Notification::NOTIFICATION_JUNCTION => "junction",
        MSMoveReminder_Notification::NOTIFICATION_SEGMENT => "segment",
        MSMoveReminder_Notification::NOTIFICATION_LANE_CHANGE => "laneChange",
        MSMoveReminder_Notification::NOTIFICATION_LOAD_STATE => "loadState",
        MSMoveReminder_Notification::NOTIFICATION_TELEPORT => "teleport",
        MSMoveReminder_Notification::NOTIFICATION_TELEPORT_CONTINUATION => "teleportContinuation",
        MSMoveReminder_Notification::NOTIFICATION_PARKING => "parking",
        MSMoveReminder_Notification::NOTIFICATION_REROUTE => "reroute",
        MSMoveReminder_Notification::NOTIFICATION_PARKING_REROUTE => "parkingReroute",
        MSMoveReminder_Notification::NOTIFICATION_ARRIVED => "arrived",
        MSMoveReminder_Notification::NOTIFICATION_TELEPORT_ARRIVED => "teleportArrived",
        MSMoveReminder_Notification::NOTIFICATION_VAPORIZED_CALIBRATOR => "vaporizedCalibrator",
        MSMoveReminder_Notification::NOTIFICATION_VAPORIZED_COLLISION => "vaporizedCollision",
        MSMoveReminder_Notification::NOTIFICATION_VAPORIZED_TRACI => "vaporizedTraCI",
        MSMoveReminder_Notification::NOTIFICATION_VAPORIZED_GUI => "vaporizedGUI",
        MSMoveReminder_Notification::NOTIFICATION_VAPORIZED_VAPORIZER => "vaporizer",
        MSMoveReminder_Notification::NOTIFICATION_VAPORIZED_BREAKDOWN => "vaporizedBreakdown",
        MSMoveReminder_Notification::NOTIFICATION_NONE => "none",
    }
}

/// `std::bitset<256>::test` for `MSLogicJunction::LinkBits`.
fn link_bits_test(bits: &MSLogicJunction_LinkBits, bit: i32) -> bool {
    bits.test(bit)
}

/// `dynamic_cast<const MSRailSignal*>(logic)`: null unless the logic is a rail signal.
fn as_rail_signal(logic: Ptr<MSTrafficLightLogic>) -> Ptr<MSRailSignal> {
    if logic.is_null() || logic.get_logic_type() != TrafficLightType::RAIL_SIGNAL {
        return Ptr::null();
    }
    logic.cast::<MSRailSignal>()
}

/// `myOrigin != nullptr ? dynamic_cast<const MSRailSignal*>(myOrigin->getTLLogic()) : nullptr`
fn origin_rail_signal(origin: Ptr<MSLink>) -> Ptr<MSRailSignal> {
    if origin.is_null() {
        return Ptr::null();
    }
    as_rail_signal(origin.get_tl_logic())
}

impl MSDriveWay {
    /// C++: `void MSDriveWay::MSDriveWay(const MSLink *, const std::string &, bool)`
    // MSDriveWay::MSDriveWay(const MSLink* origin, const std::string& id, bool temporary) :
    //     MSMoveReminder("DriveWay_" + (temporary ? "tmp" : id)),
    //     Named(id),
    //     myNumericalID(temporary ? -1 : myGlobalDriveWayIndex++),
    //     myOrigin(origin),
    //     myActive(nullptr),
    //     myCoreSize(0),
    //     myForwardEdgeCount(0),
    //     myFoundSignal(false),
    //     myFoundJump(false),
    //     myTerminateRoute(false),
    //     myAbortedBuild(false),
    //     myBidiEnded(false),
    //     myParent(nullptr)
    // {}
    #[allow(unused_variables)]
    pub fn new(origin: Ptr<MSLink>, id: &str, temporary: bool) -> Self {
        MSDriveWay {
            base: MSMoveReminder::new(
                &format!("DriveWay_{}", if temporary { "tmp" } else { id }),
                Ptr::null(),
                false,
            ),
            base2: Named::new(id),
            myNumericalID: if temporary {
                -1
            } else {
                MY_GLOBAL_DRIVE_WAY_INDEX.with(|i| {
                    let v = i.get();
                    i.set(v + 1);
                    v
                })
            },
            myOrigin: origin,
            myActive: Ptr::null(),
            myRoute: Vec::new(),
            myCoreSize: 0,
            myForwardEdgeCount: 0,
            myFoundSignal: false,
            myFoundJump: false,
            myTerminateRoute: false,
            myAbortedBuild: false,
            myBidiEnded: false,
            myParent: Ptr::null(),
            myForward: Vec::new(),
            myBidi: Vec::new(),
            myBidiExtended: Vec::new(),
            myBidiExtendedSorted: Vec::new(),
            myFlank: Vec::new(),
            myConflictLanes: Vec::new(),
            myConflictLinks: Vec::new(),
            myTrains: BTreeSet::new(),
            myVehicleEvents: Vec::new(),
            myFoes: Vec::new(),
            mySidings: BTreeMap::new(),
            myDeadlocks: Vec::new(),
            mySubDriveWays: Vec::new(),
            myReversals: Vec::new(),
            myFirstVehicle: String::new(),
        }
    }

    /// C++: `void MSDriveWay::~MSDriveWay() noexcept` [virtual]
    /// Destructor
    // MSDriveWay::~MSDriveWay() {
    //     for (const MSDriveWay* sub : mySubDriveWays) {
    //         delete sub;
    //     }
    //     mySubDriveWays.clear();
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ deletes the sub-driveways here; they are allocated with Box::leak (see build_sub_foe)
        // and are therefore only released when the process exits.
        self.mySubDriveWays.clear();
    }

    /// C++: `void MSDriveWay::cleanup()` [static]
    // void
    // MSDriveWay::cleanup() {
    //     myGlobalDriveWayIndex = 0;
    //     myBlockLengthWarnings.clear();
    //     myWriteVehicles = false;
    //
    //     for (auto item : myDepartureDriveways) {
    //         for (MSDriveWay* dw : item.second) {
    //             delete dw;
    //         }
    //     }
    //     MSDriveWay::mySwitchDriveWays.clear();
    //     MSDriveWay::myReversalDriveWays.clear();
    //     MSDriveWay::myDepartureDriveways.clear();
    //     MSDriveWay::myDepartDrivewayIndex.clear();
    //     MSDriveWay::myDepartureDrivewaysEnds.clear();
    //     MSDriveWay::myEndingDriveways.clear();
    //     MSDriveWay::mySidingDriveways.clear();
    // }
    #[allow(unused_variables)]
    pub fn cleanup() {
        MY_GLOBAL_DRIVE_WAY_INDEX.with(|i| i.set(0));
        MY_BLOCK_LENGTH_WARNINGS.with(|w| w.borrow_mut().clear());
        MY_WRITE_VEHICLES.with(|w| w.set(false));

        // shortcut: the departure driveways are allocated with Box::leak (see build_drive_way) so they
        // cannot be deleted here; the maps below are cleared just like in C++.
        MY_SWITCH_DRIVE_WAYS.with(|m| m.borrow_mut().clear());
        MY_REVERSAL_DRIVE_WAYS.with(|m| m.borrow_mut().clear());
        MY_DEPARTURE_DRIVEWAYS.with(|m| m.borrow_mut().clear());
        MY_DEPART_DRIVEWAY_INDEX.with(|m| m.borrow_mut().clear());
        MY_DEPARTURE_DRIVEWAYS_ENDS.with(|m| m.borrow_mut().clear());
        MY_ENDING_DRIVEWAYS.with(|m| m.borrow_mut().clear());
        MY_SIDING_DRIVEWAYS.with(|m| m.borrow_mut().clear());
    }

    /// C++: `void MSDriveWay::clearState()` [static]
    // void
    // MSDriveWay::clearState() {
    //     for (auto item : myEndingDriveways) {
    //         for (MSDriveWay* dw : item.second) {
    //             dw->myTrains.clear();
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn clear_state() {
        MY_ENDING_DRIVEWAYS.with(|m| {
            for item in m.borrow().values() {
                for dw in item {
                    dw.m().myTrains.clear();
                }
            }
        });
    }

    /// C++: `bool MSDriveWay::haveDriveWays()` [static]
    // static bool haveDriveWays() {
    //         return myGlobalDriveWayIndex > 0;
    //     }
    #[allow(unused_variables)]
    pub fn have_drive_ways() -> bool {
        MY_GLOBAL_DRIVE_WAY_INDEX.with(|i| i.get() > 0)
    }

    /// C++: `const MSDriveWay * MSDriveWay::retrieveDepartDriveWay(const MSEdge *, const std::string &)` [static]
    // const MSDriveWay*
    // MSDriveWay::retrieveDepartDriveWay(const MSEdge* edge, const std::string& id) {
    //     for (MSDriveWay* dw : myDepartureDriveways[edge]) {
    //         if (dw->getID() == id) {
    //             return dw;
    //         }
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn retrieve_depart_drive_way(edge: Ptr<MSEdge>, id: &str) -> Ptr<MSDriveWay> {
        let dws = MY_DEPARTURE_DRIVEWAYS.with(|m| m.borrow().get(&edge).cloned().unwrap_or_default());
        for dw in dws.iter() {
            if dw.base2.get_id() == id {
                return *dw;
            }
        }
        Ptr::null()
    }

    /// C++: `bool MSDriveWay::notifyEnter(SUMOTrafficObject &, Notification, const MSLane *)` [virtual]
    // bool
    // MSDriveWay::notifyEnter(SUMOTrafficObject& veh, Notification reason, const MSLane* enteredLane) {
    // #ifdef DEBUG_MOVEREMINDER
    //     std::cout << SIMTIME << " notifyEnter " << getDescription() << " veh=" << veh.getID() << " lane=" << (MSGlobals::gUseMesoSim ? veh.getCurrentEdge()->getID() : Named::getIDSecure(enteredLane)) << " reason=" << reason << "\n";
    // #endif
    //
    //     if (veh.isVehicle() && MSRailSignalControl::isUsingDriveWays(veh.getVClass())) {
    //         if ((enteredLane == myLane || (MSGlobals::gUseMesoSim && veh.getCurrentEdge() == &myLane->getEdge()))
    //                 && (reason == NOTIFICATION_DEPARTED || reason == NOTIFICATION_JUNCTION || reason == NOTIFICATION_PARKING)) {
    //             SUMOVehicle& sveh = dynamic_cast<SUMOVehicle&>(veh);
    //             MSRouteIterator firstIt = std::find(sveh.getCurrentRouteEdge(), sveh.getRoute().end(), myLane->getNextNormal());
    //             if (match(firstIt, sveh.getRoute().end())) {
    //                 if (myTrains.count(&sveh) == 0) {
    //                     enterDriveWay(sveh, reason);
    //                 }
    //                 return true;
    //             }
    //         } else if (reason == NOTIFICATION_REROUTE) {
    //             SUMOVehicle& sveh = dynamic_cast<SUMOVehicle&>(veh);
    //             assert(myTrains.count(&sveh) == 0);
    //             int movedPast = matchesPastRoute(sveh);
    //             // vehicle must still be one the drivway
    //             if (movedPast >= 0 && movedPast < myForwardEdgeCount) {
    //                 enterDriveWay(sveh, reason);
    //                 return true;
    //             }
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn notify_enter(&mut self, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        let sveh: Ptr<SUMOVehicle> = Ptr::from_mut(veh).cast::<SUMOVehicle>();
        let mveh: Ptr<MSVehicle> = sveh.cast::<MSVehicle>();
        if mveh.is_vehicle() && MSRailSignalControl::is_using_drive_ways(mveh.get_v_class()) {
            let my_lane = self.myLane;
            if (enteredLane == my_lane
                || (MSGlobals::use_meso_sim() && mveh.get_current_edge() == Ptr::new(my_lane.get_edge())))
                && (reason == MSMoveReminder_Notification::NOTIFICATION_DEPARTED
                    || reason == MSMoveReminder_Notification::NOTIFICATION_JUNCTION
                    || reason == MSMoveReminder_Notification::NOTIFICATION_PARKING) {
                let route_edges = mveh.get_route().get_edges();
                let route_len = route_edges.len();
                let cur = *mveh.get_current_route_edge();
                let next_normal = my_lane.get_next_normal();
                let first_it = route_edges
                    .iter()
                    .skip(cur)
                    .position(|e| *e == next_normal)
                    .map(|p| cur + p)
                    .unwrap_or(route_len);
                if self.match_(route_edges, first_it, route_len) {
                    if !self.myTrains.contains(&sveh) {
                        self.enter_drive_way(sveh.m(), reason);
                    }
                    return true;
                }
            } else if reason == MSMoveReminder_Notification::NOTIFICATION_REROUTE {
                assert!(!self.myTrains.contains(&sveh));
                let moved_past = self.matches_past_route(sveh.m());
                // vehicle must still be one the drivway
                if moved_past >= 0 && moved_past < self.myForwardEdgeCount {
                    self.enter_drive_way(sveh.m(), reason);
                    return true;
                }
            }
        }
        false
    }

    /// C++: `bool MSDriveWay::notifyLeave(SUMOTrafficObject &, double, Notification, const MSLane *)` [virtual]
    // bool
    // MSDriveWay::notifyLeave(SUMOTrafficObject& veh, double /*lastPos*/, Notification reason, const MSLane* enteredLane) {
    //     UNUSED_PARAMETER(enteredLane);
    // #ifdef DEBUG_MOVEREMINDER
    //     std::cout << SIMTIME << " notifyLeave " << getDescription() << " veh=" << veh.getID() << " lane=" << Named::getIDSecure(enteredLane) << " reason=" << toString(reason) << "\n";
    // #endif
    //     if (veh.isVehicle() && MSRailSignalControl::isUsingDriveWays(veh.getVClass())) {
    //         // leaving network with departure, teleport etc
    //         if (reason != MSMoveReminder::NOTIFICATION_JUNCTION && reason != MSMoveReminder::NOTIFICATION_SEGMENT) {
    //             myTrains.erase(&dynamic_cast<SUMOVehicle&>(veh));
    //             if (myWriteVehicles) {
    //                 myVehicleEvents.push_back(VehicleEvent(SIMSTEP, false, veh.getID(), reason));
    //             }
    //             return false;
    //         } else if (MSGlobals::gUseMesoSim && reason != MSMoveReminder::NOTIFICATION_SEGMENT) {
    //             // notifyLeave is called before moving the route iterator
    //             const MSLane* leftLane = (*(dynamic_cast<SUMOVehicle&>(veh).getCurrentRouteEdge()))->getLanes().front();
    //             return notifyLeaveBack(veh, reason, leftLane);
    //         } else {
    //             return true;
    //         }
    //     } else {
    //         return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn notify_leave(&mut self, veh: &mut SUMOTrafficObject, lastPos: f64, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        let sveh: Ptr<SUMOVehicle> = Ptr::from_mut(veh).cast::<SUMOVehicle>();
        let mveh: Ptr<MSVehicle> = sveh.cast::<MSVehicle>();
        if mveh.is_vehicle() && MSRailSignalControl::is_using_drive_ways(mveh.get_v_class()) {
            // leaving network with departure, teleport etc
            if reason != MSMoveReminder_Notification::NOTIFICATION_JUNCTION
                && reason != MSMoveReminder_Notification::NOTIFICATION_SEGMENT {
                self.myTrains.remove(&sveh);
                if MY_WRITE_VEHICLES.with(|w| w.get()) {
                    self.myVehicleEvents.push(MSDriveWay_VehicleEvent::new(
                        MSNet::get_instance().get_current_time_step(),
                        false,
                        &mveh.get_id().to_string(),
                        reason,
                    ));
                }
                return false;
            } else if MSGlobals::use_meso_sim() && reason != MSMoveReminder_Notification::NOTIFICATION_SEGMENT {
                // notifyLeave is called before moving the route iterator
                let left_lane = {
                    let route_edges = mveh.get_route().get_edges();
                    let cur = *mveh.get_current_route_edge();
                    route_edges[cur].get_lanes().first().copied().unwrap_or(Ptr::null())
                };
                return self.notify_leave_back(veh, reason, left_lane);
            } else {
                return true;
            }
        } else {
            return false;
        }
    }

    /// C++: `bool MSDriveWay::notifyLeaveBack(SUMOTrafficObject &, Notification, const MSLane *)` [virtual]
    // bool
    // MSDriveWay::notifyLeaveBack(SUMOTrafficObject& veh, Notification reason, const MSLane* leftLane) {
    // #ifdef DEBUG_MOVEREMINDER
    //     std::cout << SIMTIME << " notifyLeaveBack " << getDescription() << " veh=" << veh.getID() << " lane=" << Named::getIDSecure(leftLane) << " reason=" << toString(reason) << "\n";
    // #endif
    //     if (veh.isVehicle() && MSRailSignalControl::isUsingDriveWays(veh.getVClass())) {
    //         if (leftLane == myForward.back() && (veh.getBackLane() != leftLane->getBidiLane() || MSGlobals::gUseMesoSim)) {
    //             myTrains.erase(&dynamic_cast<SUMOVehicle&>(veh));
    //             if (myWriteVehicles) {
    //                 myVehicleEvents.push_back(VehicleEvent(SIMSTEP, false, veh.getID(), reason));
    //             }
    //             return false;
    //         } else {
    //             return true;
    //         }
    //     } else {
    //         return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn notify_leave_back(&mut self, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, leftLane: Ptr<MSLane>) -> bool {
        let sveh: Ptr<SUMOVehicle> = Ptr::from_mut(veh).cast::<SUMOVehicle>();
        let mveh: Ptr<MSVehicle> = sveh.cast::<MSVehicle>();
        if mveh.is_vehicle() && MSRailSignalControl::is_using_drive_ways(mveh.get_v_class()) {
            let my_forward_last = *self.myForward.last().unwrap();
            if leftLane == my_forward_last && (mveh.get_back_lane() != leftLane.get_bidi_lane() || MSGlobals::use_meso_sim()) {
                self.myTrains.remove(&sveh);
                if MY_WRITE_VEHICLES.with(|w| w.get()) {
                    self.myVehicleEvents.push(MSDriveWay_VehicleEvent::new(
                        MSNet::get_instance().get_current_time_step(),
                        false,
                        &mveh.get_id().to_string(),
                        reason,
                    ));
                }
                return false;
            } else {
                return true;
            }
        } else {
            return false;
        }
    }

    /// C++: `bool MSDriveWay::notifyReroute(SUMOTrafficObject &)` [virtual]
    // bool
    // MSDriveWay::notifyReroute(SUMOTrafficObject& veh) {
    // #ifdef DEBUG_MOVEREMINDER
    //     std::cout << SIMTIME << " notifyReroute " << getDescription() << " veh=" << veh.getID() << "\n";
    // #endif
    //     assert(veh.isVehicle());
    //     if (MSRailSignalControl::isUsingDriveWays(veh.getVClass())) {
    //         SUMOVehicle* sveh = dynamic_cast<SUMOVehicle*>(&veh);
    //         assert(myTrains.count(sveh) != 0);
    //         if (matchesPastRoute(*sveh) >= 0) {
    //             //std::cout << SIMTIME << " notifyReroute " << getDescription() << " veh=" << veh.getID() << " valid\n";
    //             return true;
    //         }
    //         // no match found, remove
    //         myTrains.erase(sveh);
    //         if (myWriteVehicles) {
    //             myVehicleEvents.push_back(VehicleEvent(SIMSTEP, false, veh.getID(), NOTIFICATION_REROUTE));
    //         }
    //         //std::cout << SIMTIME << " notifyReroute " << getDescription() << " veh=" << veh.getID() << " invalid\n";
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn notify_reroute(&mut self, veh: &mut SUMOTrafficObject) -> bool {
        let sveh: Ptr<SUMOVehicle> = Ptr::from_mut(veh).cast::<SUMOVehicle>();
        let mveh: Ptr<MSVehicle> = sveh.cast::<MSVehicle>();
        assert!(mveh.is_vehicle());
        if MSRailSignalControl::is_using_drive_ways(mveh.get_v_class()) {
            assert!(self.myTrains.contains(&sveh));
            if self.matches_past_route(sveh.m()) >= 0 {
                return true;
            }
            // no match found, remove
            self.myTrains.remove(&sveh);
            if MY_WRITE_VEHICLES.with(|w| w.get()) {
                self.myVehicleEvents.push(MSDriveWay_VehicleEvent::new(
                    MSNet::get_instance().get_current_time_step(),
                    false,
                    &mveh.get_id().to_string(),
                    MSMoveReminder_Notification::NOTIFICATION_REROUTE,
                ));
            }
        }
        false
    }

    /// C++: `bool MSDriveWay::flankConflict(const MSDriveWay &) const` [const]
    /// Whether there is a flank conflict with the given driveway
    // bool
    // MSDriveWay::flankConflict(const MSDriveWay& other) const {
    //     for (const MSLane* lane : myForward) {
    //         for (const MSLane* lane2 : other.myForward) {
    //             if (lane == lane2) {
    //                 return true;
    //             }
    //         }
    //         for (const MSLane* lane2 : other.myBidi) {
    //             if (lane == lane2) {
    //                 if (bidiBlockedBy(other)) {
    //                     // it's only a deadlock if both trains block symmetrically
    //                     return true;
    //                 }
    //             }
    //         }
    //         auto it = std::lower_bound(other.myBidiExtendedSorted.begin(), other.myBidiExtendedSorted.end(), lane);
    //         if (it != other.myBidiExtendedSorted.end() && *it == lane) {
    //             if (bidiBlockedBy(other)) {
    //                 // it's only a deadlock if both trains block symmetrically
    //                 return true;
    //             }
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn flank_conflict(&self, other: &MSDriveWay) -> bool {
        for &lane in self.myForward.iter() {
            for &lane2 in other.myForward.iter() {
                if lane == lane2 {
                    return true;
                }
            }
            for &lane2 in other.myBidi.iter() {
                if lane == lane2 {
                    if self.bidi_blocked_by(other) {
                        // it's only a deadlock if both trains block symmetrically
                        return true;
                    }
                }
            }
            if let Some(&it) = lower_bound_lane(&other.myBidiExtendedSorted, lane) {
                if it == lane {
                    if self.bidi_blocked_by(other) {
                        // it's only a deadlock if both trains block symmetrically
                        return true;
                    }
                }
            }
        }
        false
    }

    /// C++: `bool MSDriveWay::crossingConflict(const MSDriveWay &) const` [const]
    /// Whether there is a crossing conflict with the given driveway
    // bool
    // MSDriveWay::crossingConflict(const MSDriveWay& other) const {
    //     for (const MSLane* lane : myForward) {
    //         for (const MSLane* lane2 : other.myForward) {
    //             if (lane->isNormal() && lane2->isNormal() && lane->getEdge().getToJunction() == lane2->getEdge().getToJunction()) {
    //                 if (lane->getToJunction()->getType() == SumoXMLNodeType::RAIL_SIGNAL
    //                         && lane == myForward.back()
    //                         && &lane->getEdge() != myRoute.back()) {
    //                     // switch/crossing is also a rail_signal (direct control) at the end of the forward section.
    //                     // (and the route continues)
    //                     // This means the conflict is already guarded
    //                     return false;
    //                 }
    //                 return true;
    //             }
    //         }
    //     }
    //     if (myOrigin != nullptr && other.myOrigin != nullptr
    //             && myOrigin->getJunction() == other.myOrigin->getJunction()
    //             //&& myForward.front()->isInternal() && other.myForward.front()->isInternal()
    //             && myOrigin->getJunction()->getLogic() != nullptr
    //             && myOrigin->getJunction()->getLogic()->getFoesFor(myOrigin->getIndex()).test(other.myOrigin->getIndex())) {
    //         // switch/crossing is also a rail_signal (direct control)
    //         if (!(myForward.front()->isInternal() && other.myForward.front()->isInternal())) {
    //             return false;
    //         }
    //         return true;
    //     }
    //     if (other.myOrigin != nullptr && other.myForward.front()->isInternal()) {
    //         for (int i = 0; i < (int)myForward.size() - 1; i++) {
    //             const MSLane* lane = myForward[i];
    //             if (lane->getToJunction() == other.myOrigin->getJunction()) {
    //                 const MSLane* next = myForward[i + 1];
    //                 const MSLink* link = lane->getLinkTo(next);
    //                 if (link && link->getTLLogic() == nullptr) {
    //                     // switch/crossing is also a rail_signal (direct control) but own link is uncontrolled
    //                     if (lane->getToJunction()->getLogic() != nullptr
    //                             && lane->getToJunction()->getLogic()->getFoesFor(link->getIndex()).test(other.myOrigin->getIndex())) {
    //                         // and links are in conflict
    //                         return true;
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn crossing_conflict(&self, other: &MSDriveWay) -> bool {
        for &lane in self.myForward.iter() {
            for &lane2 in other.myForward.iter() {
                if lane.is_normal() && lane2.is_normal() {
                    let lane_edge = lane.get_edge();
                    let lane2_edge = lane2.get_edge();
                    if lane_edge.get_to_junction() == lane2_edge.get_to_junction() {
                        if lane.get_to_junction().get_type() == SumoXMLNodeType::RAIL_SIGNAL
                            && lane == *self.myForward.last().unwrap()
                            && Ptr::new(lane.get_edge()) != *self.myRoute.last().unwrap()
                        {
                            // switch/crossing is also a rail_signal (direct control) at the end of the forward section.
                            // (and the route continues)
                            // This means the conflict is already guarded
                            return false;
                        }
                        return true;
                    }
                }
            }
        }
        if !self.myOrigin.is_null() && !other.myOrigin.is_null()
            && self.myOrigin.get_junction() == other.myOrigin.get_junction()
            //&& myForward.front()->isInternal() && other.myForward.front()->isInternal()
            && !self.myOrigin.get_junction().get_logic().is_null()
        {
            let logic = self.myOrigin.get_junction().get_logic();
            let foes = logic.get_foes_for(self.myOrigin.get_index());
            if link_bits_test(foes, other.myOrigin.get_index()) {
                // switch/crossing is also a rail_signal (direct control)
                if !(self.myForward.first().unwrap().is_internal() && other.myForward.first().unwrap().is_internal()) {
                    return false;
                }
                return true;
            }
        }
        if !other.myOrigin.is_null() && other.myForward.first().unwrap().is_internal() {
            for i in 0..(self.myForward.len() as i32 - 1) {
                let lane = self.myForward[i as usize];
                if lane.get_to_junction() == other.myOrigin.get_junction() {
                    let next = self.myForward[i as usize + 1];
                    let link = lane.get_link_to(next);
                    if !link.is_null() && link.get_tl_logic().is_null() {
                        // switch/crossing is also a rail_signal (direct control) but own link is uncontrolled
                        let logic = lane.get_to_junction().get_logic();
                        if !logic.is_null() && link_bits_test(logic.get_foes_for(link.get_index()), other.myOrigin.get_index()) {
                            // and links are in conflict
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    /// C++: `bool MSDriveWay::bidiBlockedBy(const MSDriveWay &) const` [const]
    /// Whether there is a bidi conflict with the given driveway
    // bool
    // MSDriveWay::bidiBlockedBy(const MSDriveWay& other) const {
    //     for (const MSLane* lane : myBidi) {
    //         for (const MSLane* lane2 : other.myForward) {
    //             if (lane == lane2) {
    //                 return true;
    //             }
    //         }
    //     }
    //
    //     for (const MSLane* lane2 : other.myForward) {
    //         auto it = std::lower_bound(myBidiExtendedSorted.begin(), myBidiExtendedSorted.end(), lane2);
    //         if (it != myBidiExtendedSorted.end() && *it == lane2) {
    //             if (overlap(other)) {
    //                 return true;
    //             }
    //         }
    //     }
    //
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn bidi_blocked_by(&self, other: &MSDriveWay) -> bool {
        for &lane in self.myBidi.iter() {
            for &lane2 in other.myForward.iter() {
                if lane == lane2 {
                    return true;
                }
            }
        }

        for &lane2 in other.myForward.iter() {
            if let Some(&it) = lower_bound_lane(&self.myBidiExtendedSorted, lane2) {
                if it == lane2 && self.overlap(other) {
                    return true;
                }
            }
        }

        false
    }

    /// C++: `bool MSDriveWay::bidiBlockedByEnd(const MSDriveWay &) const` [const]
    /// Whether there is a bidi conflict with the end of the given driveway
    // bool
    // MSDriveWay::bidiBlockedByEnd(const MSDriveWay& other) const {
    //     const MSLane* end = other.myForward.back();
    //     for (const MSLane* lane : myBidi) {
    //         if (lane == end) {
    //             return true;
    //         }
    //     }
    //     auto it = std::lower_bound(myBidiExtendedSorted.begin(), myBidiExtendedSorted.end(), end);
    //     if (it != myBidiExtendedSorted.end() && *it == end) {
    //         if (overlap(other)) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn bidi_blocked_by_end(&self, other: &MSDriveWay) -> bool {
        let end = *other.myForward.last().unwrap();
        for &lane in self.myBidi.iter() {
            if lane == end {
                return true;
            }
        }
        if let Some(&it) = lower_bound_lane(&self.myBidiExtendedSorted, end) {
            if it == end && self.overlap(other) {
                return true;
            }
        }
        false
    }

    /// C++: `bool MSDriveWay::forwardRouteConflict(std::set<const MSEdge *>, const MSDriveWay &, bool)`
    /// Whether the route of other passes into the forward section of this driveway
    // bool
    // MSDriveWay::forwardRouteConflict(std::set<const MSEdge*> forward, const MSDriveWay& other, bool secondCheck) {
    //     int i = 0;
    //     for (const MSEdge* edge2 : other.myRoute) {
    //         if (i == other.myCoreSize) {
    //             return false;
    //         }
    //         i++;
    //         if (edge2 == myForward.front()->getNextNormal() && !secondCheck) {
    //             // foe should not pass from behind through our own forward section
    //             return false;
    //         }
    //         if (forward.count(edge2->getBidiEdge()) != 0) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn forward_route_conflict(&mut self, forward: BTreeSet<Ptr<MSEdge>>, other: &MSDriveWay, secondCheck: bool) -> bool {
        let mut i = 0;
        for &edge2 in other.myRoute.iter() {
            if i == other.myCoreSize {
                return false;
            }
            i += 1;
            if edge2 == self.myForward.first().unwrap().get_next_normal() && !secondCheck {
                // foe should not pass from behind through our own forward section
                return false;
            }
            if forward.contains(&edge2.get_bidi_edge()) {
                return true;
            }
        }
        false
    }

    /// C++: `bool MSDriveWay::conflictLaneOccupied(bool, const SUMOVehicle *) const` [const]
    /// whether any of myConflictLanes is occupied (vehicles that are the target of a join must be ignored)
    // bool
    // MSDriveWay::conflictLaneOccupied(bool store, const SUMOVehicle* ego) const {
    //     for (const MSLane* lane : myConflictLanes) {
    //         if (!lane->isEmpty()) {
    //             std::string joinVehicle = "";
    //             if (ego != nullptr && !MSGlobals::gUseMesoSim) {
    //                 const SUMOVehicleParameter::Stop* stop = ego->getNextStopParameter();
    //                 if (stop != nullptr) {
    //                     joinVehicle = stop->join;
    //                 }
    //             }
    // #ifdef DEBUG_SIGNALSTATE
    //             if (gDebugFlag4) {
    //                 std::cout << SIMTIME << " conflictLane " << lane->getID() << " occupied ego=" << Named::getIDSecure(ego) << " vehNumber=" << lane->getVehicleNumber() << "\n";
    //                 if (joinVehicle != "") {
    //                     std::cout << "  joinVehicle=" << joinVehicle << " occupant=" << toString(lane->getVehiclesSecure()) << "\n";
    //                     lane->releaseVehicles();
    //                 }
    //             }
    // #endif
    //             if (lane->getVehicleNumberWithPartials() == 1) {
    //                 MSVehicle* foe = lane->getLastAnyVehicle();
    //                 if (joinVehicle != "") {
    //                     if (foe->getID() == joinVehicle && foe->isStopped()) {
    // #ifdef DEBUG_SIGNALSTATE
    //                         if (gDebugFlag4) {
    //                             std::cout << "    ignore join-target '" << joinVehicle << "\n";
    //                         }
    // #endif
    //                         continue;
    //                     }
    //                 }
    //                 if (ego != nullptr) {
    //                     if (foe == ego && std::find(myForward.begin(), myForward.end(), lane) == myForward.end()) {
    // #ifdef DEBUG_SIGNALSTATE
    //                         if (gDebugFlag4) {
    //                             std::cout << "    ignore ego as oncoming '" << ego->getID() << "\n";
    //                         }
    // #endif
    //                         continue;
    //                     }
    //                     if (foe->isStopped() && foe->getNextStopParameter()->join == ego->getID()) {
    // #ifdef DEBUG_SIGNALSTATE
    //                         if (gDebugFlag4) {
    //                             std::cout << "    ignore " << foe->getID() << " for which ego is join-target\n";
    //                         }
    // #endif
    //                         continue;
    //                     }
    //                 }
    //             }
    //             if (MSRailSignal::storeVehicles() && store) {
    //                 MSRailSignal::blockingVehicles().push_back(lane->getLastAnyVehicle());
    //             }
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn conflict_lane_occupied(&self, store: bool, ego: Ptr<SUMOVehicle>) -> bool {
        for &lane in self.myConflictLanes.iter() {
            if !lane.is_empty() {
                let mut join_vehicle = String::new();
                if !ego.is_null() && !MSGlobals::use_meso_sim() {
                    let stop = ego.cast::<MSVehicle>().get_next_stop_parameter();
                    if !stop.is_null() {
                        join_vehicle = stop.join.clone();
                    }
                }
                if lane.get_vehicle_number_with_partials() == 1 {
                    let foe = lane.get_last_any_vehicle();
                    if join_vehicle != "" {
                        if foe.get_id() == join_vehicle && foe.cast::<MSVehicle>().is_stopped() {
                            continue;
                        }
                    }
                    if !ego.is_null() {
                        if (foe.as_raw() as *mut ()) == (ego.as_raw() as *mut ()) && !self.myForward.contains(&lane) {
                            continue;
                        }
                        let foe_stop = foe.cast::<MSVehicle>().get_next_stop_parameter();
                        let foe_join = if foe_stop.is_null() { String::new() } else { foe_stop.join.clone() };
                        if foe.cast::<MSVehicle>().is_stopped() && foe_join == ego.get_id() {
                            continue;
                        }
                    }
                }
                if MSRailSignal::store_vehicles() && store {
                    MSRailSignal::blocking_vehicles().push(lane.get_last_any_vehicle().cast::<SUMOVehicle>());
                }
                return true;
            }
        }
        false
    }

    /// C++: `bool MSDriveWay::foeDriveWayOccupied(bool, const SUMOVehicle *, MSEdgeVector &) const` [const]
    /// whether any of myFoes is occupied (vehicles that are the target of a join must be ignored)
    // bool
    // MSDriveWay::foeDriveWayOccupied(bool store, const SUMOVehicle* ego, MSEdgeVector& occupied) const {
    //     for (const MSDriveWay* foeDW : myFoes) {
    //         if (!foeDW->myTrains.empty()) {
    // #ifdef DEBUG_SIGNALSTATE
    //             if (gDebugFlag4 || DEBUG_COND_DW2 || DEBUG_HELPER(ego)) {
    //                 std::cout << SIMTIME << " " << getID() << " foeDriveWay " << foeDW->getID() << " occupied ego=" << Named::getIDSecure(ego) << " foeVeh=" << toString(foeDW->myTrains) << "\n";
    //             }
    // #endif
    //             if (foeDW->myTrains.size() == 1) {
    //                 SUMOVehicle* foe = *foeDW->myTrains.begin();
    //                 if (foe == ego) {
    // #ifdef DEBUG_SIGNALSTATE
    //                     if (gDebugFlag4 || DEBUG_HELPER(ego)) {
    //                         std::cout << "    ignore ego as foe '" << Named::getIDSecure(ego) << "\n";
    //                     }
    // #endif
    //                     continue;
    //                 }
    //                 if (hasJoin(ego, foe)) {
    //                     continue;
    //                 }
    //             }
    //             std::pair<bool, const MSDriveWay*> useSiding = canUseSiding(ego, foeDW);
    // #ifdef DEBUG_SIGNALSTATE
    //             if (gDebugFlag4 || DEBUG_COND_DW2 || DEBUG_HELPER(ego)) {
    //                 auto it = mySidings.find(foeDW);
    //                 int numSidings = 0;
    //                 if (it != mySidings.end()) {
    //                     numSidings = it->second.size();
    //                 }
    //                 std::cout << " ego=" << Named::getIDSecure(ego) << " useSiding=" << useSiding.first << " sidingFoe=" << Named::getIDSecure(useSiding.second) << " numSidings=" << numSidings << "\n";
    //             }
    // #endif
    //             if (useSiding.first) {
    //                 continue;
    //             } else {
    //                 if (MSRailSignal::storeVehicles() && store) {
    //                     for (SUMOVehicle* foe : foeDW->myTrains) {
    //                         MSRailSignal::blockingVehicles().push_back(foe);
    //                     }
    //                     MSRailSignal::blockingDriveWays().push_back(foeDW);
    //                 }
    //                 for (const SUMOVehicle* foe : foeDW->myTrains) {
    //                     occupied.push_back(const_cast<MSEdge*>(foe->getEdge()));
    //                     MSEdge* bidi = const_cast<MSEdge*>(foe->getEdge()->getBidiEdge());
    //                     if (bidi != nullptr) {
    //                         occupied.push_back(bidi);
    //                     }
    //                     // @todo: if foe occupies more than one edge we should add all of them to the occupied vector
    //                 }
    //                 if (ego != nullptr && MSGlobals::gTimeToTeleportRSDeadlock > 0
    //                         && (ego->getWaitingTime() > ego->getVehicleType().getCarFollowModel().getStartupDelay() || !ego->isOnRoad())) {
    //                     // if there is an occupied siding, it becomes part of the waitRelation
    //                     SUMOVehicle* foe = *(useSiding.second == nullptr ? foeDW : useSiding.second)->myTrains.begin();
    //                     const MSRailSignal* rs = myOrigin != nullptr ? dynamic_cast<const MSRailSignal*>(myOrigin->getTLLogic()) : nullptr;
    //                     MSRailSignalControl::getInstance().addWaitRelation(ego, rs, foe);
    //                 }
    //                 return true;
    //             }
    //         } else if (foeDW != this && isDepartDriveway() && !foeDW->isDepartDriveway()) {
    //             if (foeDW->myOrigin->getApproaching().size() > 0) {
    //                 Approaching foeA = foeDW->myOrigin->getClosest();
    //                 const SUMOVehicle* foe = foeA.first;
    //                 if (foeA.second.dist < foe->getBrakeGap(true)) {
    //                     MSRouteIterator firstIt = std::find(foe->getCurrentRouteEdge(), foe->getRoute().end(), foeDW->myRoute.front());
    //                     if (firstIt != foe->getRoute().end()) {
    //                         if (foeDW->match(firstIt, foe->getRoute().end())) {
    //                             bool useSiding = canUseSiding(ego, foeDW).first;
    // #ifdef DEBUG_SIGNALSTATE
    //                             if (gDebugFlag4 || DEBUG_COND_DW2 || DEBUG_HELPER(ego)) {
    //                                 std::cout << SIMTIME << " " << getID() << " blocked by " << foeDW->getID() << " (approached by " << foe->getID() << ") useSiding=" << useSiding << "\n";
    //                             }
    // #endif
    //                             if (useSiding) {
    //                                 //std::cout << SIMTIME << " " << getID() << " ego=" << ego->getID() << " foeDW=" << foeDW->getID() << " myFoes=" << toString(myFoes) << "\n";
    //                                 continue;
    //                             } else {
    //                                 return true;
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     for (const std::set<const MSDriveWay*>& dlFoes : myDeadlocks) {
    //         bool allOccupied = true;
    //         for (const MSDriveWay* dlFoe : dlFoes) {
    //             if (dlFoe->myTrains.empty()) {
    //                 allOccupied = false;
    //                 //std::cout << SIMTIME << " " << getID() << " ego=" << Named::getIDSecure(ego) << "  deadlockCheck clear " << dlFoe->getID() << "\n";
    //                 break;
    //             }
    //         }
    //         if (allOccupied) {
    // #ifdef DEBUG_SIGNALSTATE
    //             if (gDebugFlag4 || DEBUG_COND_DW2 || DEBUG_HELPER(ego)) {
    //                 std::cout << SIMTIME << " " << getID() << " ego=" << Named::getIDSecure(ego) << " deadlockCheck " << joinNamedToString(dlFoes, " ") << " myFoes=" << toString(myFoes) << "\n";
    //             }
    // #endif
    //             for (const MSDriveWay* dlFoe : dlFoes) {
    //                 MSRailSignal::blockingDriveWays().push_back(dlFoe);
    //             }
    //             // for addWaitRelation we need to find the foe that results in the
    //             // longest chain (otherwise the deadlock relation is smaller than the original)
    //             int start = myForwardEdgeCount;
    //             bool foundFoe = false;
    //             while (start < (int)myRoute.size() && !foundFoe) {
    //                 const MSEdge* e = myRoute[start];
    //                 if (e->getFromJunction()->getType() == SumoXMLNodeType::RAIL_SIGNAL) {
    //                     const MSEdge* p = myRoute[start - 1];
    //                     const MSLink* link = p->getLanes().front()->getLinkTo(e->getLanes().front());
    //                     if (link != nullptr && link->isTLSControlled()) {
    //                         MSRailSignal* rs = const_cast<MSRailSignal*>(dynamic_cast<const MSRailSignal*>(link->getTLLogic()));
    //                         const MSDriveWay& cand = rs->getExistingDriveWay(link, myRoute.begin() + start, myRoute.end());
    //                         for (const MSDriveWay* dlFoe : dlFoes) {
    //                             if (std::find(cand.myFoes.begin(), cand.myFoes.end(), dlFoe) != cand.myFoes.end()) {
    //                                 foundFoe = true;
    //                                 const MSRailSignal* rs2 = myOrigin != nullptr ? dynamic_cast<const MSRailSignal*>(myOrigin->getTLLogic()) : nullptr;
    //                                 MSRailSignalControl::getInstance().addWaitRelation(ego, rs2, *dlFoe->myTrains.begin(), nullptr);
    //                             }
    //                         }
    //                     }
    //                 }
    //                 start += 1;
    //             }
    //             if (!foundFoe) {
    //                 for (const MSDriveWay* dlFoe : dlFoes) {
    //                     const MSRailSignal* rs2 = myOrigin != nullptr ? dynamic_cast<const MSRailSignal*>(myOrigin->getTLLogic()) : nullptr;
    //                     MSRailSignalControl::getInstance().addWaitRelation(ego, rs2, *dlFoe->myTrains.begin(), nullptr);
    //                     break;
    //                 }
    //             }
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn foe_drive_way_occupied(&self, store: bool, ego: Ptr<SUMOVehicle>, occupied: &mut MSEdgeVector) -> bool {
        let this_ptr = Ptr::new(self);
        for &foe_dw in self.myFoes.iter() {
            if !foe_dw.myTrains.is_empty() {
                if foe_dw.myTrains.len() == 1 {
                    let foe = *foe_dw.myTrains.iter().next().unwrap();
                    if foe == ego {
                        continue;
                    }
                    if Self::has_join(ego, foe) {
                        continue;
                    }
                }
                let use_siding = self.can_use_siding(ego, foe_dw, Ptr::null());
                if use_siding.0 {
                    continue;
                } else {
                    if MSRailSignal::store_vehicles() && store {
                        for &foe in foe_dw.myTrains.iter() {
                            MSRailSignal::blocking_vehicles().push(foe);
                        }
                        MSRailSignal::blocking_drive_ways().push(foe_dw);
                    }
                    for &foe in foe_dw.myTrains.iter() {
                        let mfoe = foe.cast::<MSVehicle>();
                        occupied.push(mfoe.get_edge());
                        let bidi = mfoe.get_edge().get_bidi_edge();
                        if !bidi.is_null() {
                            occupied.push(bidi);
                        }
                        // @todo: if foe occupies more than one edge we should add all of them to the occupied vector
                    }
                    if !ego.is_null() && MSGlobals::time_to_teleport_rs_deadlock() > 0
                        && (ego.cast::<MSVehicle>().get_waiting_time(false)
                            > ego.cast::<MSVehicle>().get_vehicle_type().get_car_follow_model().myStartupDelay
                            || !ego.cast::<MSVehicle>().is_on_road())
                    {
                        // if there is an occupied siding, it becomes part of the waitRelation
                        let src = if use_siding.1.is_null() { foe_dw } else { use_siding.1 };
                        let foe = *src.myTrains.iter().next().unwrap();
                        let rs = origin_rail_signal(self.myOrigin);
                        MSRailSignalControl::get_instance().add_wait_relation(ego, rs, foe, Ptr::null());
                    }
                    return true;
                }
            } else if foe_dw != this_ptr && self.is_depart_driveway() && !foe_dw.is_depart_driveway() {
                if !foe_dw.myOrigin.get_approaching_2().is_empty() {
                    let foe_a = foe_dw.myOrigin.get_closest();
                    let foe = foe_a.0;
                    let mfoe = foe.cast::<MSVehicle>();
                    if foe_a.1.dist < mfoe.get_brake_gap(true) {
                        let route_edges = mfoe.get_route().get_edges();
                        let route_len = route_edges.len();
                        let cur = *mfoe.get_current_route_edge();
                        let front = *foe_dw.myRoute.first().unwrap();
                        let first_it = route_edges
                            .iter()
                            .skip(cur)
                            .position(|e| *e == front)
                            .map(|p| cur + p)
                            .unwrap_or(route_len);
                        if first_it != route_len && foe_dw.match_(route_edges, first_it, route_len) {
                            let use_siding = self.can_use_siding(ego, foe_dw, Ptr::null()).0;
                            if use_siding {
                                continue;
                            } else {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        for dl_foes in self.myDeadlocks.iter() {
            let mut all_occupied = true;
            for &dl_foe in dl_foes.iter() {
                if dl_foe.myTrains.is_empty() {
                    all_occupied = false;
                    break;
                }
            }
            if all_occupied {
                for &dl_foe in dl_foes.iter() {
                    MSRailSignal::blocking_drive_ways().push(dl_foe);
                }
                // for addWaitRelation we need to find the foe that results in the
                // longest chain (otherwise the deadlock relation is smaller than the original)
                let mut start = self.myForwardEdgeCount;
                let mut found_foe = false;
                while start < self.myRoute.len() as i32 && !found_foe {
                    let e = self.myRoute[start as usize];
                    if e.get_from_junction().get_type() == SumoXMLNodeType::RAIL_SIGNAL {
                        let p = self.myRoute[start as usize - 1];
                        let link = p.get_lanes().first().unwrap().get_link_to(*e.get_lanes().first().unwrap());
                        if !link.is_null() && link.is_tls_controlled() {
                            let rs = as_rail_signal(link.get_tl_logic());
                            if !rs.is_null() {
                                let cand: Ptr<MSDriveWay> =
                                    Ptr::new(rs.m().get_existing_drive_way(link, start as usize, self.myRoute.len()));
                                for &dl_foe in dl_foes.iter() {
                                    if cand.myFoes.contains(&dl_foe) {
                                        found_foe = true;
                                        let rs2 = origin_rail_signal(self.myOrigin);
                                        let reason = *dl_foe.myTrains.iter().next().unwrap();
                                        MSRailSignalControl::get_instance()
                                            .add_wait_relation(ego, rs2, reason, Ptr::null());
                                    }
                                }
                            }
                        }
                    }
                    start += 1;
                }
                if !found_foe {
                    for &dl_foe in dl_foes.iter() {
                        let rs2 = origin_rail_signal(self.myOrigin);
                        let reason = *dl_foe.myTrains.iter().next().unwrap();
                        MSRailSignalControl::get_instance().add_wait_relation(ego, rs2, reason, Ptr::null());
                        break;
                    }
                }
                return true;
            }
        }
        false
    }

    /// C++: `bool MSDriveWay::sidingDeadlock(const SUMOVehicle *) const` [const]
    /// whether insertion on this driveway would cause deadlock in an approached siding
    // bool
    // MSDriveWay::sidingDeadlock(const SUMOVehicle* veh) const {
    //     const MSEdge* end = &myForward.back()->getEdge();
    //     const auto it = mySidingDriveways.find(end);
    //     if (it != mySidingDriveways.end()) {
    //         for (const auto& item : it->second) {
    //             if (!item.first->myTrains.empty() && !item.second->myTrains.empty()
    //                     && bidiBlockedBy(*item.second)) {
    //                 // we found two driveways that are in conflict and
    //                 // could make use of this siding. Inserting a vehicle here could create deadlock.
    // #ifdef DEBUG_SIGNALSTATE
    //                 if (gDebugFlag4 || DEBUG_COND_DW2) {
    //                     std::cout << SIMTIME << " sidingDeadlock insertionDw=" << getID() << " approachingDW=" << item.first->getID() << " foeDW=" << item.second->getID() << "\n";
    //                 }
    // #endif
    //                 myCurrentSidingCheck.push_back(end);
    //                 if (veh->getLength() > end->getLength() && !MSGlobals::gUseMesoSim) {
    //                     double backDist =  veh->getLength() - getForwardDistance((int)myForward.size() - 1);
    //                     const MSLane* lane = myForward.front();
    //                     const MSLane* prev = lane->getLogicalPredecessorLane();
    //                     while (backDist > 0 && prev != nullptr) {
    //                         backDist -= lane->getLength();
    //                         const MSLink* link = prev->getLinkTo(lane);
    //                         if (prev->isNormal() && link->getTLLogic() != nullptr
    //                                 && link->getTLLogic()->getLogicType() == TrafficLightType::RAIL_SIGNAL) {
    //                             // inserted train back reaches onto this edge
    //                             myCurrentSidingCheck.push_back(&prev->getEdge());
    //                         }
    //                         lane = prev;
    //                         prev = lane->getLogicalPredecessorLane();
    //                     }
    //                 }
    //                 const bool requiredSiding = !item.first->canUseSiding(*item.first->myTrains.begin(), item.second).first;
    //                 myCurrentSidingCheck.clear();
    //                 if (requiredSiding) {
    //                     return true;
    //                 }
    //             }
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn siding_deadlock(&self, veh: Ptr<SUMOVehicle>) -> bool {
        let end = Ptr::new(self.myForward.last().unwrap().get_edge());
        let items = MY_SIDING_DRIVEWAYS.with(|m| m.borrow().get(&end).cloned());
        if let Some(items) = items {
            for (dw, foe) in items.iter() {
                if !dw.myTrains.is_empty() && !foe.myTrains.is_empty() && self.bidi_blocked_by(&*foe) {
                    // we found two driveways that are in conflict and
                    // could make use of this siding. Inserting a vehicle here could create deadlock.
                    let mveh = veh.cast::<MSVehicle>();
                    MY_CURRENT_SIDING_CHECK.with(|c| c.borrow_mut().push(end));
                    if mveh.get_length() > end.get_length() && !MSGlobals::use_meso_sim() {
                        let mut back_dist = mveh.get_length() - self.get_forward_distance(self.myForward.len() as i32 - 1);
                        let mut lane = *self.myForward.first().unwrap();
                        let mut prev = lane.get_logical_predecessor_lane();
                        while back_dist > 0. && !prev.is_null() {
                            back_dist -= lane.get_length();
                            let link = prev.get_link_to(lane);
                            if prev.is_normal()
                                && !link.get_tl_logic().is_null()
                                && link.get_tl_logic().get_logic_type() == TrafficLightType::RAIL_SIGNAL
                            {
                                // inserted train back reaches onto this edge
                                MY_CURRENT_SIDING_CHECK.with(|c| c.borrow_mut().push(Ptr::new(prev.get_edge())));
                            }
                            lane = prev;
                            prev = lane.get_logical_predecessor_lane();
                        }
                    }
                    let required_siding = !dw
                        .can_use_siding(*dw.myTrains.iter().next().unwrap(), *foe, Ptr::null())
                        .0;
                    MY_CURRENT_SIDING_CHECK.with(|c| c.borrow_mut().clear());
                    if required_siding {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// C++: `bool MSDriveWay::foeDriveWayApproached() const` [const]
    /// whether any of my Foes is being approached
    // bool
    // MSDriveWay::foeDriveWayApproached() const {
    //     for (const MSDriveWay* foeDW : myFoes) {
    //         if (foeDW->myOrigin != nullptr && foeDW->myOrigin->getApproaching().size() > 0) {
    // #ifdef DEBUG_SIGNALSTATE
    //             if (gDebugFlag4) {
    //                 std::cout << SIMTIME << " foeLink=" << foeDW->myOrigin->getDescription() << " approachedBy=" << foeDW->myOrigin->getApproaching().begin()->first->getID() << "\n";
    //             }
    // #endif
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn foe_drive_way_approached(&self) -> bool {
        for &foe_dw in self.myFoes.iter() {
            if !foe_dw.myOrigin.is_null() && !foe_dw.myOrigin.get_approaching_2().is_empty() {
                return true;
            }
        }
        false
    }

    /// C++: `bool MSDriveWay::reserve(const Approaching &, MSEdgeVector &)`
    /// attempt reserve this driveway for the given vehicle
    // bool
    // MSDriveWay::reserve(const Approaching& closest, MSEdgeVector& occupied) {
    //     if (foeDriveWayOccupied(true, closest.first, occupied)) {
    //         return false;
    //     }
    //     for (MSLink* foeLink : myConflictLinks) {
    //         if (hasLinkConflict(closest, foeLink)) {
    // #ifdef DEBUG_SIGNALSTATE
    //             if (gDebugFlag4 || DEBUG_HELPER(closest.first)) {
    //                 std::cout << getID() << " linkConflict with " << getTLLinkID(foeLink) << "\n";
    //             }
    // #endif
    //             return false;
    //         }
    //     }
    //     myActive = closest.first;
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn reserve(&mut self, closest: &MSRailSignal_Approaching, occupied: &mut MSEdgeVector) -> bool {
        if self.foe_drive_way_occupied(true, closest.0, occupied) {
            return false;
        }
        for &foe_link in self.myConflictLinks.iter() {
            if self.has_link_conflict(closest, foe_link) {
                return false;
            }
        }
        self.myActive = closest.0;
        true
    }

    /// C++: `void MSDriveWay::writeBlocks(OutputDevice &) const` [const]
    /// Write block items for this driveway
    // void
    // MSDriveWay::writeBlocks(OutputDevice& od) const {
    //     od.openTag(isSubDriveWay() ? SUMO_TAG_SUBDRIVEWAY : SUMO_TAG_DRIVEWAY);
    //     od.writeAttr(SUMO_ATTR_ID, myID);
    //     od.writeAttr(SUMO_ATTR_VEHICLE, myFirstVehicle);
    //     od.writeAttr(SUMO_ATTR_EDGES, toString(myRoute));
    //     if (myCoreSize != (int)myRoute.size()) {
    //         od.writeAttr("core", myCoreSize);
    //     }
    //     od.openTag("forward");
    //     od.writeAttr(SUMO_ATTR_LANES, toString(myForward));
    //     od.closeTag();
    //     if (!isSubDriveWay()) {
    //         od.openTag("bidi");
    //         od.writeAttr(SUMO_ATTR_LANES, toString(myBidi));
    //         if (myBidiExtended.size() > 0) {
    //             od.lf();
    //             od << "                   ";
    //             od.writeAttr("deadlockCheck", toString(myBidiExtended));
    //         }
    //         od.closeTag();
    //         od.openTag("flank");
    //         od.writeAttr(SUMO_ATTR_LANES, toString(myFlank));
    //         od.closeTag();
    //
    //         od.openTag("conflictLinks");
    //
    //         std::vector<std::string> signals;
    //         for (MSLink* link : myConflictLinks) {
    //             signals.push_back(getTLLinkID(link));
    //         }
    //         od.writeAttr("signals", joinToStringSorting(signals, " "));
    //         od.closeTag();
    //
    //         std::vector<std::string> foes;
    //         for (MSDriveWay* dw : myFoes) {
    //             foes.push_back(dw->myID);
    //         }
    //         if (foes.size() > 0) {
    //             od.openTag("foes");
    //             od.writeAttr("driveWays", joinToStringSorting(foes, " "));
    //             od.closeTag();
    //         }
    //         for (auto item : mySidings) {
    //             od.openTag("sidings");
    //             od.writeAttr("foe", item.first->getID());
    //             for (auto siding : item.second) {
    //                 od.openTag("siding");
    //                 od.writeAttr("start", myRoute[siding.start]->getID());
    //                 od.writeAttr("end", myRoute[siding.end]->getID());
    //                 od.writeAttr("length", siding.length);
    //                 if (siding.intermediateEnds.size() > 0) {
    //                     std::vector<std::string> endEdges;
    //                     for (int i : siding.intermediateEnds) {
    //                         endEdges.push_back(myRoute[i]->getID());
    //                     }
    //                     od.writeAttr("intermediateEnds", endEdges);
    //                 }
    //                 od.closeTag();
    //             }
    //             od.closeTag();
    //         }
    //         for (auto item : myDeadlocks) {
    //             od.openTag("deadlock");
    //             od.writeAttr("foes", joinNamedToStringSorting(item, " "));
    //             od.closeTag();
    //         }
    //     }
    //     od.closeTag(); // driveWay
    //
    //     for (const MSDriveWay* sub : mySubDriveWays) {
    //         sub->writeBlocks(od);
    //     }
    // #ifdef DRIVEWAY_SANITY_CHECK
    //     std::set<MSDriveWay*> uFoes(myFoes.begin(), myFoes.end());
    //     if (uFoes.size() != myFoes.size()) {
    //         WRITE_WARNINGF("Duplicate foes in driveway '%'", getID());
    //
    //     }
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn write_blocks(&self, od: &mut OutputDevice) {
        if self.is_sub_drive_way() {
            od.open_tag_id(SUMO_TAG_SUBDRIVEWAY);
        } else {
            od.open_tag_id(SUMO_TAG_DRIVEWAY);
        }
        od.write_attr(SUMO_ATTR_ID, &self.base2.get_id().to_string());
        od.write_attr(SUMO_ATTR_VEHICLE, &self.myFirstVehicle);
        od.write_attr(SUMO_ATTR_EDGES, &edge_ids(&self.myRoute));
        if self.myCoreSize != self.myRoute.len() as i32 {
            od.write_attr("core", &self.myCoreSize);
        }
        od.open_tag("forward");
        od.write_attr(SUMO_ATTR_LANES, &lane_ids(&self.myForward));
        od.close_tag("");
        if !self.is_sub_drive_way() {
            od.open_tag("bidi");
            od.write_attr(SUMO_ATTR_LANES, &lane_ids(&self.myBidi));
            if !self.myBidiExtended.is_empty() {
                od.lf();
                od.write_padding("                   ");
                od.write_attr("deadlockCheck", &lane_ids(&self.myBidiExtended));
            }
            od.close_tag("");
            od.open_tag("flank");
            od.write_attr(SUMO_ATTR_LANES, &lane_ids(&self.myFlank));
            od.close_tag("");

            od.open_tag("conflictLinks");

            let mut signals: Vec<String> = Vec::new();
            for &link in self.myConflictLinks.iter() {
                signals.push(Self::get_tl_link_id(link));
            }
            od.write_attr("signals", &utils::common::to_string::join_to_string_sorting(&signals, " "));
            od.close_tag("");

            let mut foes: Vec<String> = Vec::new();
            for &dw in self.myFoes.iter() {
                foes.push(dw.base2.get_id().to_string());
            }
            if !foes.is_empty() {
                od.open_tag("foes");
                od.write_attr("driveWays", &utils::common::to_string::join_to_string_sorting(&foes, " "));
                od.close_tag("");
            }
            for (foe, sidings) in self.mySidings.iter() {
                od.open_tag("sidings");
                od.write_attr("foe", &foe.base2.get_id().to_string());
                for siding in sidings.iter() {
                    od.open_tag("siding");
                    od.write_attr("start", &self.myRoute[siding.start as usize].get_id().to_string());
                    od.write_attr("end", &self.myRoute[siding.end as usize].get_id().to_string());
                    od.write_attr("length", &siding.length);
                    if !siding.intermediateEnds.is_empty() {
                        let mut end_edges: Vec<String> = Vec::new();
                        for &i in siding.intermediateEnds.iter() {
                            end_edges.push(self.myRoute[i as usize].get_id().to_string());
                        }
                        od.write_attr("intermediateEnds", &end_edges);
                    }
                    od.close_tag("");
                }
                od.close_tag("");
            }
            for item in self.myDeadlocks.iter() {
                od.open_tag("deadlock");
                od.write_attr("foes", &drive_way_ids_sorted(item));
                od.close_tag("");
            }
        }
        od.close_tag(""); // driveWay

        for &sub in self.mySubDriveWays.iter() {
            sub.write_blocks(od);
        }
        // #ifdef DRIVEWAY_SANITY_CHECK
        let u_foes: BTreeSet<Ptr<MSDriveWay>> = self.myFoes.iter().copied().collect();
        if u_foes.len() != self.myFoes.len() {
            utils::write_warningf!("Duplicate foes in driveway '%'", self.base2.get_id().to_string());
        }
        // #endif
    }

    /// C++: `void MSDriveWay::writeBlockVehicles(OutputDevice &) const` [const]
    // void
    // MSDriveWay::writeBlockVehicles(OutputDevice& od) const {
    //     od.openTag(isSubDriveWay() ? "subDriveWay" : "driveWay");
    //     od.writeAttr(SUMO_ATTR_ID, myID);
    //     for (const VehicleEvent& ve : myVehicleEvents) {
    //         od.openTag(ve.isEntry ? "entry" : "exit");
    //         od.writeAttr(SUMO_ATTR_ID, ve.id);
    //         od.writeAttr(SUMO_ATTR_TIME, time2string(ve.time));
    //         od.writeAttr("reason", Notifications.getString(ve.reason));
    //         od.closeTag(); // event
    //     }
    //     od.closeTag(); // driveWay
    //
    //     for (const MSDriveWay* sub : mySubDriveWays) {
    //         sub->writeBlockVehicles(od);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_block_vehicles(&self, od: &mut OutputDevice) {
        if self.is_sub_drive_way() {
            od.open_tag("subDriveWay");
        } else {
            od.open_tag("driveWay");
        }
        od.write_attr(SUMO_ATTR_ID, &self.base2.get_id().to_string());
        for ve in self.myVehicleEvents.iter() {
            if ve.isEntry {
                od.open_tag("entry");
            } else {
                od.open_tag("exit");
            }
            od.write_attr(SUMO_ATTR_ID, &ve.id);
            od.write_attr(SUMO_ATTR_TIME, &time2string(ve.time));
            od.write_attr("reason", &notification_string(ve.reason));
            od.close_tag(""); // event
        }
        od.close_tag(""); // driveWay

        for &sub in self.mySubDriveWays.iter() {
            sub.write_block_vehicles(od);
        }
    }

    /// C++: `const std::vector<const MSEdge *> & MSDriveWay::getRoute() const` [const]
    // const std::vector<const MSEdge*>& getRoute() const {
    //         return myRoute;
    //     }
    #[allow(unused_variables)]
    pub fn get_route(&self) -> &Vec<Ptr<MSEdge>> {
        &self.myRoute
    }

    /// C++: `const std::vector<const MSLane *> & MSDriveWay::getFlank() const` [const]
    // const std::vector<const MSLane*>& getFlank() const {
    //         return myFlank;
    //     }
    #[allow(unused_variables)]
    pub fn get_flank(&self) -> &Vec<Ptr<MSLane>> {
        &self.myFlank
    }

    /// C++: `const std::vector<const MSLane *> & MSDriveWay::getBidi() const` [const]
    // const std::vector<const MSLane*>& getBidi() const {
    //         return myBidi;
    //     }
    #[allow(unused_variables)]
    pub fn get_bidi(&self) -> &Vec<Ptr<MSLane>> {
        &self.myBidi
    }

    /// C++: `const std::vector<MSLink *> & MSDriveWay::getConflictLinks() const` [const]
    // const std::vector<MSLink*>& getConflictLinks() const {
    //         return myConflictLinks;
    //     }
    #[allow(unused_variables)]
    pub fn get_conflict_links(&self) -> &Vec<Ptr<MSLink>> {
        &self.myConflictLinks
    }

    /// C++: `int MSDriveWay::getNumericalID() const` [const]
    // int getNumericalID() const {
    //         return myNumericalID;
    //     }
    #[allow(unused_variables)]
    pub fn get_numerical_id(&self) -> i32 {
        self.myNumericalID
    }

    /// C++: `const std::vector<VehicleEvent> & MSDriveWay::getEvents() const` [const]
    // const std::vector<VehicleEvent>& getEvents() const {
    //         return myVehicleEvents;
    //     }
    #[allow(unused_variables)]
    pub fn get_events(&self) -> &Vec<MSDriveWay_VehicleEvent> {
        &self.myVehicleEvents
    }

    /// C++: `void MSDriveWay::setEvents(const std::vector<VehicleEvent> &)`
    // void setEvents(const std::vector<VehicleEvent>& events) {
    //         myVehicleEvents = events;
    //     }
    #[allow(unused_variables)]
    pub fn set_events(&mut self, events: &Vec<MSDriveWay_VehicleEvent>) {
        self.myVehicleEvents = events.to_vec();
    }

    /// C++: `void MSDriveWay::setVehicle(const std::string &)`
    // void setVehicle(const std::string& vehID) {
    //         myFirstVehicle = vehID;
    //     }
    #[allow(unused_variables)]
    pub fn set_vehicle(&mut self, vehID: &str) {
        self.myFirstVehicle = vehID.to_string();
    }

    /// C++: `const std::string & MSDriveWay::getFirstVehicle() const` [const]
    // const std::string& getFirstVehicle() const {
    //         return myFirstVehicle;
    //     }
    #[allow(unused_variables)]
    pub fn get_first_vehicle(&self) -> &str {
        &self.myFirstVehicle
    }

    /// C++: `const std::vector<MSDriveWay *> & MSDriveWay::getFoes() const` [const]
    // const std::vector<MSDriveWay*>& getFoes() const {
    //         return myFoes;
    //     }
    #[allow(unused_variables)]
    pub fn get_foes(&self) -> &Vec<Ptr<MSDriveWay>> {
        &self.myFoes
    }

    /// C++: `const std::vector<const MSLane *> & MSDriveWay::getForward() const` [const]
    // const std::vector<const MSLane*>& getForward() const {
    //         return myForward;
    //     }
    #[allow(unused_variables)]
    pub fn get_forward(&self) -> &Vec<Ptr<MSLane>> {
        &self.myForward
    }

    /// C++: `bool MSDriveWay::match(MSRouteIterator, MSRouteIterator) const` [const]
    /// whether the give route matches this driveway
    // bool
    // MSDriveWay::match(MSRouteIterator firstIt, MSRouteIterator endIt) const {
    //     // @todo optimize: it is sufficient to check for specific edges (after each switch)
    //     auto itRoute = firstIt;
    //     auto itDwRoute = myRoute.begin();
    //     bool match = true;
    //     while (itRoute != endIt && itDwRoute != myRoute.end()) {
    //         if (*itRoute != *itDwRoute) {
    //             match = false;
    // #ifdef DEBUG_MATCH
    //             std::cout << "  check dw=" << getID() << " match failed at vehEdge=" << (*itRoute)->getID() << " dwEdge=" << (*itDwRoute)->getID() << "\n";
    // #endif
    //             break;
    //         }
    //         itRoute++;
    //         itDwRoute++;
    //     }
    //     // if the vehicle arrives before the end of this driveway,
    //     // we'd rather build a new driveway to avoid superfluous restrictions
    //     if (match && itDwRoute == myRoute.end()
    //             && (itRoute == endIt || myAbortedBuild || myBidiEnded || myFoundJump || isSubDriveWay())) {
    //         //std::cout << "  using dw=" << "\n";
    //         if (itRoute != endIt) {
    //             // check whether the current route requires an extended driveway
    //             const MSEdge* next = *itRoute;
    //             const MSEdge* prev = myRoute.back();
    //             if (myFoundJump && prev->getBidiEdge() != next && prev->getBidiEdge() != nullptr
    //                     && prev->isConnectedTo(*next, (SUMOVehicleClass)(SVC_RAIL_CLASSES & prev->getPermissions()))) {
    // #ifdef DEBUG_MATCH
    //                 std::cout << "  check dw=" << getID() << " prev=" << prev->getID() << " next=" << next->getID() << "\n";
    // #endif
    //                 return false;
    //             }
    //             if (!myFoundJump && prev->getBidiEdge() == next && prev == &myForward.back()->getEdge()) {
    //                 assert(isSubDriveWay() || myBidiEnded);
    //                 // must not leave driveway via reversal
    // #ifdef DEBUG_MATCH
    //                 std::cout << getID() << " back=" << myForward.back()->getID() << " noMatch route " << toString(ConstMSEdgeVector(firstIt, endIt)) << "\n";
    // #endif
    //                 return false;
    //             }
    //             if (myForward.back()->isInternal() && myForward.back()->getNextNormal() != (*itRoute)) {
    //                 // driveway is part of a direct-control conflict and continues elsewhere
    // #ifdef DEBUG_MATCH
    //                 std::cout << getID() << " back=" << myForward.back()->getID() << " noMatch route " << toString(ConstMSEdgeVector(firstIt, itRoute)) << " (direct control)\n";
    // #endif
    //                 return false;
    //             }
    //         }
    //         return !isSubDriveWay() || myParent->match(firstIt, endIt);
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn match_(&self, route: &ConstMSEdgeVector, firstIt: MSRouteIterator, endIt: MSRouteIterator) -> bool {
        // @todo optimize: it is sufficient to check for specific edges (after each switch)
        let mut it_route = firstIt;
        let mut it_dw_route = 0usize;
        let mut matched = true;
        while it_route != endIt && it_dw_route != self.myRoute.len() {
            if route[it_route] != self.myRoute[it_dw_route] {
                matched = false;
                break;
            }
            it_route += 1;
            it_dw_route += 1;
        }
        // if the vehicle arrives before the end of this driveway,
        // we'd rather build a new driveway to avoid superfluous restrictions
        if matched
            && it_dw_route == self.myRoute.len()
            && (it_route == endIt || self.myAbortedBuild || self.myBidiEnded || self.myFoundJump || self.is_sub_drive_way())
        {
            if it_route != endIt {
                // check whether the current route requires an extended driveway
                let next = route[it_route];
                let prev = *self.myRoute.last().unwrap();
                if self.myFoundJump
                    && prev.get_bidi_edge() != next
                    && !prev.get_bidi_edge().is_null()
                    && prev.is_connected_to(&*next, SVC_RAIL_CLASSES & prev.get_permissions(), false)
                {
                    return false;
                }
                if !self.myFoundJump
                    && prev.get_bidi_edge() == next
                    && prev == Ptr::new(self.myForward.last().unwrap().get_edge())
                {
                    assert!(self.is_sub_drive_way() || self.myBidiEnded);
                    // must not leave driveway via reversal
                    return false;
                }
                if self.myForward.last().unwrap().is_internal() && self.myForward.last().unwrap().get_next_normal() != next {
                    // driveway is part of a direct-control conflict and continues elsewhere
                    return false;
                }
            }
            return !self.is_sub_drive_way() || self.myParent.match_(route, firstIt, endIt);
        }
        false
    }

    /// C++: `void MSDriveWay::addDWDeadlock(const std::vector<const MSDriveWay *> &)`
    // void
    // MSDriveWay::addDWDeadlock(const std::vector<const MSDriveWay*>& deadlockFoes) {
    //     std::set<const MSDriveWay*> filtered;
    //     // we do not need to keep any of our own foes. If any of them were occupied then
    //     // foeDriveWayOccupied would return True without considering myDeadlocks
    //     for (const MSDriveWay* foe : deadlockFoes) {
    //         if (std::find(myFoes.begin(), myFoes.end(), foe) == myFoes.end()) {
    //             filtered.insert(foe);
    //         }
    //     }
    //     if (std::find(myDeadlocks.begin(), myDeadlocks.end(), filtered) == myDeadlocks.end()) {
    //         myDeadlocks.push_back(filtered);
    //         //std::cout << getID() << " deadlockFoes=" << toString(filtered) << "\n";
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_dw_deadlock(&mut self, deadlockFoes: &Vec<Ptr<MSDriveWay>>) {
        let mut filtered: BTreeSet<Ptr<MSDriveWay>> = BTreeSet::new();
        // we do not need to keep any of our own foes. If any of them were occupied then
        // foeDriveWayOccupied would return True without considering myDeadlocks
        for &foe in deadlockFoes.iter() {
            if !self.myFoes.contains(&foe) {
                filtered.insert(foe);
            }
        }
        if !self.myDeadlocks.contains(&filtered) {
            self.myDeadlocks.push(filtered);
        }
    }

    /// C++: `bool MSDriveWay::isDepartDriveway() const` [const]
    // bool isDepartDriveway() const {
    //         return myOrigin == nullptr;
    //     }
    #[allow(unused_variables)]
    pub fn is_depart_driveway(&self) -> bool {
        self.myOrigin.is_null()
    }

    /// C++: `const MSLink * MSDriveWay::getOrigin() const` [const]
    // const MSLink* getOrigin() const {
    //         return myOrigin;
    //     }
    #[allow(unused_variables)]
    pub fn get_origin(&self) -> Ptr<MSLink> {
        self.myOrigin
    }

    /// C++: `bool MSDriveWay::hasTrain(SUMOVehicle *) const` [const]
    /// whether the given train is on this driveway
    // bool
    // MSDriveWay::hasTrain(SUMOVehicle* veh) const {
    //     return myTrains.count(veh) != 0;
    // }
    #[allow(unused_variables)]
    pub fn has_train(&self, veh: Ptr<SUMOVehicle>) -> bool {
        self.myTrains.contains(&veh)
    }

    /// C++: `const std::vector<MSDriveWay *> & MSDriveWay::getSubDriveWays() const` [const]
    // const std::vector<MSDriveWay*>& getSubDriveWays() const {
    //         return mySubDriveWays;
    //     }
    #[allow(unused_variables)]
    pub fn get_sub_drive_ways(&self) -> &Vec<Ptr<MSDriveWay>> {
        &self.mySubDriveWays
    }

    /// C++: `bool MSDriveWay::foundSignal() const` [const]
    // bool foundSignal() const {
    //         return myFoundSignal;
    //     }
    #[allow(unused_variables)]
    pub fn found_signal(&self) -> bool {
        self.myFoundSignal
    }

    /// C++: `bool MSDriveWay::isSubDriveWay() const` [const]
    // bool isSubDriveWay() const {
    //         return myParent != nullptr;
    //     }
    #[allow(unused_variables)]
    pub fn is_sub_drive_way(&self) -> bool {
        !self.myParent.is_null()
    }

    /// C++: `const MSDriveWay * MSDriveWay::getParent() const` [const]
    // const MSDriveWay* getParent() const {
    //         return myParent;
    //     }
    #[allow(unused_variables)]
    pub fn get_parent(&self) -> Ptr<MSDriveWay> {
        self.myParent
    }

    /// C++: `double MSDriveWay::getParentRemainingLength() const` [const]
    /// return for how much longer the parent driveway continues after the end of this subdriveway
    // double
    // MSDriveWay::getParentRemainingLength() const {
    //     double result = 0;
    //     if (myParent != nullptr) {
    //         auto it = std::find(myParent->myForward.begin(), myParent->myForward.end(), myForward.back());
    //         assert(it != myParent->myForward.end());
    //         for (; it != myParent->myForward.end(); it++) {
    //             result += (*it)->getLength();
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_parent_remaining_length(&self) -> f64 {
        let mut result = 0.0;
        if !self.myParent.is_null() {
            let last = *self.myForward.last().unwrap();
            let start = self.myParent.myForward.iter().position(|l| *l == last);
            assert!(start.is_some());
            if let Some(start) = start {
                for &lane in self.myParent.myForward[start..].iter() {
                    result += lane.get_length();
                }
            }
        }
        result
    }

    /// C++: `void MSDriveWay::init()` [static]
    // void
    // MSDriveWay::init() {
    //     myWriteVehicles = OptionsCont::getOptions().isSet("railsignal-vehicle-output");
    //     myMovingBlockMaxDist = OptionsCont::getOptions().getFloat("railsignal.moving-block.max-dist");
    // }
    #[allow(unused_variables)]
    pub fn init() {
        let write_vehicles = utils::options::options_cont::is_set("railsignal-vehicle-output");
        MY_WRITE_VEHICLES.with(|w| w.set(write_vehicles));
        let max_dist = utils::options::options_cont::get_float("railsignal.moving-block.max-dist");
        MY_MOVING_BLOCK_MAX_DIST.with(|d| d.set(max_dist));
    }

    /// C++: `bool MSDriveWay::hasRS(const MSEdge *, const MSEdge *)` [static]
    // bool
    // MSDriveWay::hasRS(const MSEdge* cur, const MSEdge* next) {
    //     if (cur->getToJunction()->getType() == SumoXMLNodeType::RAIL_SIGNAL) {
    //         // check if there is a controlled link between cur and next
    //         for (auto lane : cur->getLanes()) {
    //             for (const MSLink* link : lane->getLinkCont()) {
    //                 if (&link->getLane()->getEdge() == next && link->getTLLogic() != nullptr) {
    //                     return true;
    //                 }
    //             }
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn has_rs(cur: Ptr<MSEdge>, next: Ptr<MSEdge>) -> bool {
        if cur.get_to_junction().get_type() == SumoXMLNodeType::RAIL_SIGNAL {
            // check if there is a controlled link between cur and next
            for &lane in cur.get_lanes().iter() {
                for &link in lane.get_link_cont().iter() {
                    if Ptr::new(link.get_lane().get_edge()) == next && !link.get_tl_logic().is_null() {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// C++: `bool MSDriveWay::mustYield(const Approaching &, const Approaching &)` [static]
    /// Whether veh must yield to the foe train
    // bool
    // MSDriveWay::mustYield(const Approaching& veh, const Approaching& foe) {
    //     if (foe.second.arrivalSpeedBraking == veh.second.arrivalSpeedBraking) {
    //         if (foe.second.arrivalTime == veh.second.arrivalTime) {
    //             if (foe.first->getSpeed() == veh.first->getSpeed()) {
    //                 if (foe.second.dist == veh.second.dist) {
    //                     if (foe.first->getWaitingTime() == veh.first->getWaitingTime()) {
    //                         return foe.first->getNumericalID() < veh.first->getNumericalID();
    //                     } else {
    //                         return foe.first->getWaitingTime() > veh.first->getWaitingTime();
    //                     }
    //                 } else {
    //                     return foe.second.dist < veh.second.dist;
    //                 }
    //             } else {
    //                 return foe.first->getSpeed() > veh.first->getSpeed();
    //             }
    //         } else {
    //             return foe.second.arrivalTime < veh.second.arrivalTime;
    //         }
    //     } else {
    //         return foe.second.arrivalSpeedBraking > veh.second.arrivalSpeedBraking;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn must_yield(veh: &MSRailSignal_Approaching, foe: &MSRailSignal_Approaching) -> bool {
        let mveh = veh.0.cast::<MSVehicle>();
        let mfoe = foe.0.cast::<MSVehicle>();
        if foe.1.arrivalSpeedBraking == veh.1.arrivalSpeedBraking {
            if foe.1.arrivalTime == veh.1.arrivalTime {
                if mfoe.get_speed() == mveh.get_speed() {
                    if foe.1.dist == veh.1.dist {
                        if mfoe.get_waiting_time(false) == mveh.get_waiting_time(false) {
                            return mfoe.get_numerical_id() < mveh.get_numerical_id();
                        } else {
                            return mfoe.get_waiting_time(false) > mveh.get_waiting_time(false);
                        }
                    } else {
                        return foe.1.dist < veh.1.dist;
                    }
                } else {
                    return mfoe.get_speed() > mveh.get_speed();
                }
            } else {
                return foe.1.arrivalTime < veh.1.arrivalTime;
            }
        } else {
            return foe.1.arrivalSpeedBraking > veh.1.arrivalSpeedBraking;
        }
    }

    /// C++: `MSDriveWay * MSDriveWay::buildDriveWay(const std::string &, const MSLink *, MSRouteIterator, MSRouteIterator)` [static]
    /// construct a new driveway by searching along the given route until all block structures are found
    // MSDriveWay*
    // MSDriveWay::buildDriveWay(const std::string& id, const MSLink* link, MSRouteIterator first, MSRouteIterator end) {
    //     // collect lanes and links that are relevant for setting this signal for the current driveWay
    //     // For each driveway we collect
    //     //   - conflictLanes (signal must be red if any conflict lane is occupied)
    //     //   - conflictLinks (signal must be red if any conflict link is approached by a vehicle
    //     //      - that cannot break in time (arrivalSpeedBraking > 0)
    //     //      - approached by a vehicle with higher switching priority (see #3941)
    //     // These objects are construct in steps:
    //     //
    //     // forwardBlock
    //     // - search forward recursive from outgoing lane until controlled railSignal link found
    //     //   -> add all found lanes to conflictLanes
    //     //
    //     // bidiBlock (if any forwardBlock edge has bidi edge)
    //     // - search bidi backward recursive until first switch
    //     //   - from switch search backward recursive all other incoming until controlled rail signal link
    //     //     -> add final links to conflictLinks
    //     //
    //     // flanks
    //     // - search backward recursive from flanking switches
    //     //   until controlled railSignal link or protecting switch is found
    //     //   -> add all found lanes to conflictLanes
    //     //   -> add final links to conflictLinks
    //     MSDriveWay* dw = new MSDriveWay(link, id);
    //     LaneVisitedMap visited;
    //     std::vector<const MSLane*> before;
    //     MSLane* fromBidi = nullptr;
    //     if (link != nullptr) {
    //         appendMapIndex(visited, link->getLaneBefore());
    //         fromBidi = link->getLaneBefore()->getBidiLane();
    //     }
    //     std::set<MSLink*, MSLink::ComparatorNumericalLaneIdLess> flankSwitches; // list of switches that threaten the driveway and for which protection must be found
    //
    //     if (fromBidi != nullptr) {
    //         before.push_back(fromBidi);
    //     }
    // #ifdef DEBUG_BUILD_DRIVEWAY
    //     gDebugFlag4 = DEBUG_COND_DW(dw);
    // #endif
    //     dw->buildRoute(link, first, end, visited, flankSwitches);
    //     dw->myBidiExtendedSorted.assign(dw->myBidiExtended.begin(), dw->myBidiExtended.end());
    //     std::sort(dw->myBidiExtendedSorted.begin(), dw->myBidiExtendedSorted.end());
    //     dw->myCoreSize = (int)dw->myRoute.size();
    //
    //     MSRailSignal* rs = link ? const_cast<MSRailSignal*>(static_cast<const MSRailSignal*>(link->getTLLogic())) : nullptr;
    //     const bool movingBlock = (rs && rs->isMovingBlock()) || (!rs &&
    //                              (OptionsCont::getOptions().getBool("railsignal-moving-block")
    //                               || MSRailSignalControl::isMovingBlock((*first)->getPermissions())));
    //
    //     dw->checkFlanks(link, dw->myForward, visited, true, movingBlock, flankSwitches);
    //     dw->checkFlanks(link, dw->myBidi, visited, false, movingBlock, flankSwitches);
    //     dw->checkFlanks(link, before, visited, true, movingBlock, flankSwitches);
    //     for (MSLink* fsLink : flankSwitches) {
    // #ifdef DEBUG_ADD_FOES
    //         if (DEBUG_COND_DW(dw)) {
    //             std::cout << " fsLink=" << fsLink->getDescription() << "\n";
    //         }
    // #endif
    //         dw->findFlankProtection(fsLink, fsLink, dw->myFlank);
    //     }
    //     std::set<MSLink*, MSLink::ComparatorNumericalLaneIdLess> flankSwitchesBidiExtended;
    //     dw->checkFlanks(link, dw->myBidiExtended, visited, false, movingBlock, flankSwitchesBidiExtended);
    //     for (MSLink* const flink : flankSwitchesBidiExtended) {
    // #ifdef DEBUG_ADD_FOES
    //         if (DEBUG_COND_DW(dw)) {
    //             std::cout << " fsLinkExtended=" << flink->getDescription() << "\n";
    //         }
    // #endif
    //         dw->findFlankProtection(flink, flink, dw->myBidiExtended);
    //     }
    // #ifdef DEBUG_BUILD_DRIVEWAY
    //     if (DEBUG_COND_DW(dw)) {
    //         std::cout << SIMTIME << " buildDriveWay " << dw->myID << " link=" << (link == nullptr ? "NULL" : link->getDescription())
    //                   << "\n    route=" << toString(dw->myRoute)
    //                   << "\n    forward=" << toString(dw->myForward)
    //                   << "\n    bidi=" << toString(dw->myBidi)
    //                   << "\n    bidiEx=" << toString(dw->myBidiExtended)
    //                   << "\n    flank=" << toString(dw->myFlank)
    //                   << "\n    flankSwitch=" << MSRailSignal::describeLinks(std::vector<MSLink*>(flankSwitches.begin(), flankSwitches.end()))
    //                   << "\n    coreSize=" << dw->myCoreSize
    //                   << "\n";
    //     }
    // #endif
    //     if (!rs || !rs->isMovingBlock()) {
    //         dw->myConflictLanes.insert(dw->myConflictLanes.end(), dw->myForward.begin(), dw->myForward.end());
    //     }
    //     dw->myConflictLanes.insert(dw->myConflictLanes.end(), dw->myBidi.begin(), dw->myBidi.end());
    //     dw->myConflictLanes.insert(dw->myConflictLanes.end(), dw->myFlank.begin(), dw->myFlank.end());
    //     dw->addBidiFoes(rs, false);
    //     dw->addBidiFoes(rs, true);
    //     if (!movingBlock) {
    //         // add driveways that start on the same signal / lane
    //         dw->addParallelFoes(link, *first);
    //     }
    //     // add driveways that reverse along this driveways route
    //     dw->addReversalFoes(movingBlock);
    //     // make foes unique and symmetrical
    //     std::set<MSDriveWay*, ComparatorNumericalIdLess> uniqueFoes(dw->myFoes.begin(), dw->myFoes.end());
    //     dw->myFoes.clear();
    //     // check for self-intersecting forward-section in movingBlock mode
    //     if (movingBlock && uniqueFoes.count(dw) == 0) {
    //         std::map<const MSJunction*, std::vector<const MSLink*> > forwardJunctions;
    //         int iLast = (int)dw->myForward.size() - 1;
    //         bool selfIntersect = false;
    //         for (int i = 0; i < iLast && !selfIntersect; i++) {
    //             const MSLane* fw = dw->myForward[i];
    //             if (fw->isNormal()) {
    //                 const MSJunction* fwTo = fw->getEdge().getToJunction();
    //                 const MSLink* fwLink = fw->getLinkTo(dw->myForward[i + 1]);
    //                 if (fwLink != nullptr) {
    //                     for (const MSLink* link2 : forwardJunctions[fwTo]) {
    //                         const std::vector<MSLink*>& foeLinks = fwLink->getFoeLinks();
    //                         const std::vector<MSLink*>& foeLinks2 = link2->getFoeLinks();
    //                         if (std::find(foeLinks.begin(), foeLinks.end(), link2) != foeLinks.end()
    //                                 || std::find(foeLinks2.begin(), foeLinks2.end(), fwLink) != foeLinks2.end()
    //                                 || fwLink->getLane()->getBidiLane() == link2->getLaneBefore()
    //                                 || link2->getLane()->getBidiLane() == fwLink->getLaneBefore()) {
    //                             dw->myFoes.push_back(dw);
    //                             selfIntersect = true;
    // #ifdef DEBUG_ADD_FOES
    //                             if (DEBUG_COND_DW(dw)) {
    //                                 std::cout << " self-intersecting movingBlock for dw=" << dw->getID() << " at junction " << fwTo->getID() << " fwLink=" << fwLink->getDescription() << " link2=" << link2->getDescription() << "\n";
    //                             }
    // #endif
    //                             break;
    //                         }
    //                     }
    //                     forwardJunctions[fwTo].push_back(fwLink);
    //                 }
    //             }
    //         }
    //     }
    //     std::set<MSLink*, MSLink::ComparatorNumericalLaneIdLess> uniqueCLink(dw->myConflictLinks.begin(), dw->myConflictLinks.end());
    //     const MSEdge* lastEdge = &dw->myForward.back()->getEdge();
    //     for (MSDriveWay* foe : uniqueFoes) {
    //         const MSEdge* foeLastEdge = &foe->myForward.back()->getEdge();
    //         const bool sameLast = foeLastEdge == lastEdge;
    //         if (sameLast && !movingBlock) {
    //             dw->myFoes.push_back(foe);
    //             if (foe != dw) {
    //                 foe->myFoes.push_back(dw);
    //             }
    //         } else {
    //             if (foe->bidiBlockedByEnd(*dw)) {
    // #ifdef DEBUG_ADD_FOES
    //                 if (DEBUG_COND_DW(dw)) {
    //                     std::cout << " setting " << dw->getID() << " as foe of " << foe->getID() << "\n";
    //                 }
    // #endif
    //                 foe->myFoes.push_back(dw);
    //                 foe->addSidings(dw);
    //             } else {
    //                 dw->buildSubFoe(foe, movingBlock);
    //             }
    //             if (foe != dw) { // check for movingBlock
    //                 if (dw->bidiBlockedByEnd(*foe)) {
    // #ifdef DEBUG_ADD_FOES
    //                     if (DEBUG_COND_DW(dw)) {
    //                         std::cout << " addFoeCheckSiding " << foe->getID() << "\n";
    //                     }
    // #endif
    //                     dw->myFoes.push_back(foe);
    //                     dw->addSidings(foe);
    //                 } else  {
    //                     foe->buildSubFoe(dw, movingBlock);
    //                 }
    //             }
    //         }
    //         if (link) {
    //             foe->addConflictLink(link);
    //         }
    //         // ignore links that have the same start junction
    //         if (foe->myRoute.front()->getFromJunction() != dw->myRoute.front()->getFromJunction()) {
    //             for (auto ili : foe->myForward.front()->getIncomingLanes()) {
    //                 if (ili.viaLink->getTLLogic() != nullptr) {
    //                     // ignore links that originate on myBidi
    //                     const MSLane* origin = ili.viaLink->getLaneBefore();
    //                     if (std::find(dw->myBidi.begin(), dw->myBidi.end(), origin) == dw->myBidi.end()) {
    //                         uniqueCLink.insert(ili.viaLink);
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     dw->myConflictLinks.clear();
    //     dw->myConflictLinks.insert(dw->myConflictLinks.begin(), uniqueCLink.begin(), uniqueCLink.end());
    //     myEndingDriveways[lastEdge].push_back(dw);
    //     if (!movingBlock) {
    //         // every driveway is it's own foe (also all driveways that depart in the same block)
    //         for (MSDriveWay* sameEnd : myEndingDriveways[lastEdge]) {
    //             if (uniqueFoes.count(sameEnd) == 0) {
    //                 dw->myFoes.push_back(sameEnd);
    //                 if (sameEnd != dw) {
    //                     sameEnd->myFoes.push_back(dw);
    //                 }
    //             }
    //         }
    //     }
    // #ifdef DEBUG_BUILD_DRIVEWAY
    //     if (DEBUG_COND_DW(dw)) {
    //         std::cout << dw->myID << " mb=" << movingBlock << " finalFoes " << toString(dw->myFoes) << "\n";
    //     }
    //     gDebugFlag4 = false;
    // #endif
    //     return dw;
    // }
    #[allow(unused_variables)]
    pub fn build_drive_way(id: &str, link: Ptr<MSLink>, route: &ConstMSEdgeVector, first: MSRouteIterator, end: MSRouteIterator) -> Ptr<MSDriveWay> {
        // collect lanes and links that are relevant for setting this signal for the current driveWay
        // (see the C++ comment block above for the full description)
        // shortcut: the driveway is allocated with Box::leak because the C++ code hands the pointer to the
        // rail signal (or the departure map) without a matching delete; upgrade when ownership is modelled.
        let dw = Ptr::from_mut(Box::leak(Box::new(MSDriveWay::new(link, id, false))));
        let mut visited: MSRailSignal_LaneVisitedMap = BTreeMap::new();
        let mut before: Vec<Ptr<MSLane>> = Vec::new();
        let mut from_bidi = Ptr::null();
        if !link.is_null() {
            Self::append_map_index(&mut visited, link.get_lane_before());
            from_bidi = link.get_lane_before().get_bidi_lane();
        }
        // list of switches that threaten the driveway and for which protection must be found
        let mut flank_switches: BTreeSet<Ptr<MSLink>> = BTreeSet::new();

        if !from_bidi.is_null() {
            before.push(from_bidi);
        }
        let mut dwm = dw.m();
        dwm.build_route(route, link, first, end, &mut visited, &mut flank_switches);
        let bidi_ext = dwm.myBidiExtended.clone();
        dwm.myBidiExtendedSorted = bidi_ext;
        dwm.myBidiExtendedSorted.sort();
        dwm.myCoreSize = dwm.myRoute.len() as i32;

        let rs = if link.is_null() { Ptr::null() } else { as_rail_signal(link.get_tl_logic()) };
        let moving_block = (!rs.is_null() && rs.is_moving_block())
            || (rs.is_null()
                && (utils::options::options_cont::get_bool("railsignal-moving-block")
                    || MSRailSignalControl::is_moving_block(route[first].get_permissions())));

        dwm.check_flanks(link, &dwm.myForward, &visited, true, moving_block, &mut flank_switches);
        dwm.check_flanks(link, &dwm.myBidi, &visited, false, moving_block, &mut flank_switches);
        dwm.check_flanks(link, &before, &visited, true, moving_block, &mut flank_switches);
        for &fs_link in flank_switches.iter() {
            let flank_copy = dwm.myFlank.clone();
            dwm.find_flank_protection(fs_link, fs_link, &flank_copy);
        }
        let mut flank_switches_bidi_extended: BTreeSet<Ptr<MSLink>> = BTreeSet::new();
        dwm.check_flanks(link, &dwm.myBidiExtended, &visited, false, moving_block, &mut flank_switches_bidi_extended);
        for &flink in flank_switches_bidi_extended.iter() {
            let ext_copy = dwm.myBidiExtended.clone();
            dwm.find_flank_protection(flink, flink, &ext_copy);
        }
        if rs.is_null() || !rs.is_moving_block() {
            dwm.myConflictLanes.extend(dwm.myForward.iter().copied());
        }
        dwm.myConflictLanes.extend(dwm.myBidi.iter().copied());
        dwm.myConflictLanes.extend(dwm.myFlank.iter().copied());
        dwm.add_bidi_foes(rs, false);
        dwm.add_bidi_foes(rs, true);
        if !moving_block {
            // add driveways that start on the same signal / lane
            dwm.add_parallel_foes(link, route[first]);
        }
        // add driveways that reverse along this driveways route
        dwm.add_reversal_foes(moving_block);
        // make foes unique and symmetrical
        // determinism: C++ orders uniqueFoes by driveway numerical id (ComparatorNumericalIdLess) while the
        // BTreeSet here orders by pointer address.
        let unique_foes: BTreeSet<Ptr<MSDriveWay>> = dwm.myFoes.iter().copied().collect();
        dwm.myFoes.clear();
        // check for self-intersecting forward-section in movingBlock mode
        if moving_block && !unique_foes.contains(&dw) {
            let mut forward_junctions: BTreeMap<Ptr<MSJunction>, Vec<Ptr<MSLink>>> = BTreeMap::new();
            let i_last = dwm.myForward.len() as i32 - 1;
            let mut self_intersect = false;
            for i in 0..i_last {
                if self_intersect {
                    break;
                }
                let fw = dwm.myForward[i as usize];
                if fw.is_normal() {
                    let fw_to = fw.get_edge().get_to_junction();
                    let fw_link = fw.get_link_to(dwm.myForward[i as usize + 1]);
                    if !fw_link.is_null() {
                        let links2 = forward_junctions.get(&fw_to).cloned().unwrap_or_default();
                        for &link2 in links2.iter() {
                            let foe_links = fw_link.get_foe_links();
                            let foe_links2 = link2.get_foe_links();
                            if foe_links.contains(&link2)
                                || foe_links2.contains(&fw_link)
                                || fw_link.get_lane().get_bidi_lane() == link2.get_lane_before()
                                || link2.get_lane().get_bidi_lane() == fw_link.get_lane_before()
                            {
                                dwm.myFoes.push(dw);
                                self_intersect = true;
                                break;
                            }
                        }
                        forward_junctions.entry(fw_to).or_default().push(fw_link);
                    }
                }
            }
        }
        let mut unique_c_link: BTreeSet<Ptr<MSLink>> = dwm.myConflictLinks.iter().copied().collect();
        let last_edge = Ptr::new(dwm.myForward.last().unwrap().get_edge());
        for &foe in unique_foes.iter() {
            let foe_last_edge = Ptr::new(foe.myForward.last().unwrap().get_edge());
            let same_last = foe_last_edge == last_edge;
            if same_last && !moving_block {
                dwm.myFoes.push(foe);
                if foe != dw {
                    foe.m().myFoes.push(dw);
                }
            } else {
                if foe.bidi_blocked_by_end(&*dwm) {
                    foe.m().myFoes.push(dw);
                    foe.m().add_sidings(dw, false);
                } else {
                    dwm.build_sub_foe(foe, moving_block);
                }
                if foe != dw {
                    // check for movingBlock
                    if dwm.bidi_blocked_by_end(&*foe) {
                        dwm.myFoes.push(foe);
                        dwm.add_sidings(foe, false);
                    } else {
                        foe.m().build_sub_foe(dw, moving_block);
                    }
                }
            }
            if !link.is_null() {
                foe.m().add_conflict_link(link);
            }
            // ignore links that have the same start junction
            if foe.myRoute.first().unwrap().get_from_junction() != dwm.myRoute.first().unwrap().get_from_junction() {
                for ili in foe.myForward.first().unwrap().get_incoming_lanes().iter() {
                    if !ili.viaLink.get_tl_logic().is_null() {
                        // ignore links that originate on myBidi
                        let origin = ili.viaLink.get_lane_before();
                        if !dwm.myBidi.contains(&origin) {
                            unique_c_link.insert(ili.viaLink);
                        }
                    }
                }
            }
        }
        dwm.myConflictLinks.clear();
        dwm.myConflictLinks.extend(unique_c_link.iter().copied());
        MY_ENDING_DRIVEWAYS.with(|m| m.borrow_mut().entry(last_edge).or_default().push(dw));
        if !moving_block {
            // every driveway is it's own foe (also all driveways that depart in the same block)
            let same_ends = MY_ENDING_DRIVEWAYS.with(|m| m.borrow().get(&last_edge).cloned().unwrap_or_default());
            for same_end in same_ends.iter() {
                if !unique_foes.contains(same_end) {
                    dwm.myFoes.push(*same_end);
                    if *same_end != dw {
                        same_end.m().myFoes.push(dw);
                    }
                }
            }
        }
        dw
    }

    /// C++: `std::string MSDriveWay::getClickableTLLinkID(const MSLink *)` [static]
    /// return logicID_linkIndex in a way that allows clicking in sumo-gui
    // std::string
    // MSDriveWay::getClickableTLLinkID(const MSLink* link) {
    //     return "junction '" +  link->getTLLogic()->getID() + "', link " + toString(link->getTLIndex());
    // }
    #[allow(unused_variables)]
    pub fn get_clickable_tl_link_id(link: Ptr<MSLink>) -> String {
        format!(
            "junction '{}', link {}",
            link.get_tl_logic().get_id(),
            link.get_tl_index()
        )
    }

    /// C++: `const MSDriveWay * MSDriveWay::getDepartureDriveway(const SUMOVehicle *, bool)` [static]
    // const MSDriveWay*
    // MSDriveWay::getDepartureDriveway(const SUMOVehicle* veh, bool init) {
    //     const MSEdge* edge = init ? veh->getRoute().getEdges()[veh->getDepartEdge()] : veh->getEdge();
    //     if (edge->getFromJunction()->getType() == SumoXMLNodeType::RAIL_SIGNAL) {
    //         for (const MSLane* lane : edge->getLanes()) {
    //             for (auto ili : lane->getIncomingLanes()) {
    //                 const MSLink* entry = ili.viaLink->getCorrespondingEntryLink();
    //                 const MSRailSignal* rs = dynamic_cast<const MSRailSignal*>(entry->getTLLogic());
    //                 if (rs != nullptr) {
    //                     const MSDriveWay* dw = &const_cast<MSRailSignal*>(rs)->retrieveDriveWayForVeh(entry->getTLIndex(), veh);
    //                     if (&dw->myForward.front()->getEdge() == edge) {
    //                         return dw;
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     for (MSDriveWay* dw : myDepartureDriveways[edge]) {
    //         auto matchStart = init ? veh->getRoute().begin() + veh->getDepartEdge() : veh->getCurrentRouteEdge();
    //         if (dw->match(matchStart, veh->getRoute().end())) {
    //             return dw;
    //         }
    //     }
    //     const std::string id = edge->getFromJunction()->getID() + ".d" + toString(myDepartDrivewayIndex[edge->getFromJunction()]++);
    //     MSDriveWay* dw = buildDriveWay(id, nullptr, veh->getCurrentRouteEdge(), veh->getRoute().end());
    //     myDepartureDriveways[edge].push_back(dw);
    //     myDepartureDrivewaysEnds[&dw->myForward.back()->getEdge()].push_back(dw);
    //     dw->setVehicle(veh->getID());
    //     return dw;
    // }
    #[allow(unused_variables)]
    pub fn get_departure_driveway(veh: Ptr<SUMOVehicle>, init: bool) -> Ptr<MSDriveWay> {
        let mveh = veh.cast::<MSVehicle>();
        let route_edges = mveh.get_route().get_edges();
        let edge = if init {
            route_edges[mveh.get_depart_edge() as usize]
        } else {
            mveh.get_edge()
        };
        if edge.get_from_junction().get_type() == SumoXMLNodeType::RAIL_SIGNAL {
            for &lane in edge.get_lanes().iter() {
                for ili in lane.get_incoming_lanes().iter() {
                    let entry = ili.viaLink.get_corresponding_entry_link();
                    let rs = as_rail_signal(entry.get_tl_logic());
                    if !rs.is_null() {
                        let dw: Ptr<MSDriveWay> =
                            Ptr::new(rs.m().retrieve_drive_way_for_veh(entry.get_tl_index(), veh));
                        if Ptr::new(dw.myForward.first().unwrap().get_edge()) == edge {
                            return dw;
                        }
                    }
                }
            }
        }
        let departure_driveways = MY_DEPARTURE_DRIVEWAYS.with(|m| m.borrow().get(&edge).cloned().unwrap_or_default());
        for dw in departure_driveways.iter() {
            let match_start = if init {
                mveh.get_depart_edge() as usize
            } else {
                *mveh.get_current_route_edge()
            };
            if dw.match_(route_edges, match_start, route_edges.len()) {
                return *dw;
            }
        }
        let junction = edge.get_from_junction();
        let index = MY_DEPART_DRIVEWAY_INDEX.with(|m| {
            let mut map = m.borrow_mut();
            let e = map.entry(junction).or_insert(0);
            let v = *e;
            *e += 1;
            v
        });
        let id = format!("{}.d{}", junction.get_id(), index);
        let dw = Self::build_drive_way(&id, Ptr::null(), route_edges, *mveh.get_current_route_edge(), route_edges.len());
        MY_DEPARTURE_DRIVEWAYS.with(|m| m.borrow_mut().entry(edge).or_default().push(dw));
        let end_edge = Ptr::new(dw.myForward.last().unwrap().get_edge());
        MY_DEPARTURE_DRIVEWAYS_ENDS.with(|m| m.borrow_mut().entry(end_edge).or_default().push(dw));
        dw.m().set_vehicle(&mveh.get_id());
        dw
    }

    /// C++: `const std::vector<MSDriveWay *> & MSDriveWay::getEndingDriveWays(const MSEdge *)` [static]
    // const std::vector<MSDriveWay*>&
    // MSDriveWay::getEndingDriveWays(const MSEdge* edge) {
    //     auto it = myEndingDriveways.find(edge);
    //     if (it == myEndingDriveways.end()) {
    //         return EMPTY_DRIVEWAY_VECTOR;
    //     } else {
    //         return it->second;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_ending_drive_ways(edge: Ptr<MSEdge>) -> Vec<Ptr<MSDriveWay>> {
        // the C++ version returns a reference into the static map; the map is thread-local here, so the
        // callers get a copy of the vector instead.
        MY_ENDING_DRIVEWAYS.with(|m| m.borrow().get(&edge).cloned().unwrap_or_default())
    }

    /// C++: `void MSDriveWay::writeDepatureBlocks(OutputDevice &, bool)` [static]
    // void
    // MSDriveWay::writeDepatureBlocks(OutputDevice& od, bool writeVehicles) {
    //     for (auto item  : myDepartureDriveways) {
    //         const MSEdge* edge = item.first;
    //         if (item.second.size() > 0) {
    //             od.openTag("departJunction");
    //             od.writeAttr(SUMO_ATTR_ID, edge->getFromJunction()->getID());
    //             for (const MSDriveWay* dw : item.second) {
    //                 if (writeVehicles) {
    //                     dw->writeBlockVehicles(od);
    //                 } else {
    //                     dw->writeBlocks(od);
    //                 }
    //             }
    //             od.closeTag(); // departJunction
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_depature_blocks(od: &mut OutputDevice, writeVehicles: bool) {
        let items: Vec<(Ptr<MSEdge>, Vec<Ptr<MSDriveWay>>)> =
            MY_DEPARTURE_DRIVEWAYS.with(|m| m.borrow().iter().map(|(k, v)| (*k, v.clone())).collect());
        for (edge, driveways) in items {
            if !driveways.is_empty() {
                od.open_tag("departJunction");
                od.write_attr(SUMO_ATTR_ID, &edge.get_from_junction().get_id().to_string());
                for &dw in driveways.iter() {
                    if writeVehicles {
                        dw.write_block_vehicles(od);
                    } else {
                        dw.write_blocks(od);
                    }
                }
                od.close_tag(""); // departJunction
            }
        }
    }

    /// C++: `void MSDriveWay::saveState(OutputDevice &)` [static]
    /// Save driveway occupancy into the given stream
    // void
    // MSDriveWay::saveState(OutputDevice& out) {
    //     // all driveways are in myEndingDriveways which makes it convenient
    //     for (auto item : myEndingDriveways) {
    //         for (MSDriveWay* dw : item.second) {
    //             dw->_saveState(out);
    //             for (MSDriveWay* sub : dw->mySubDriveWays) {
    //                 sub->_saveState(out);
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn save_state(out: &mut OutputDevice) {
        // all driveways are in myEndingDriveways which makes it convenient
        let items: Vec<Vec<Ptr<MSDriveWay>>> =
            MY_ENDING_DRIVEWAYS.with(|m| m.borrow().values().cloned().collect());
        for driveways in items {
            for &dw in driveways.iter() {
                dw.save_state_2(out);
                for &sub in dw.mySubDriveWays.iter() {
                    sub.save_state_2(out);
                }
            }
        }
    }

    /// C++: `void MSDriveWay::loadState(const SUMOSAXAttributes &, int)` [static]
    // void
    // MSDriveWay::loadState(const SUMOSAXAttributes& attrs, int tag) {
    //     if ((int)myDriveWayRouteLookup.size() < myGlobalDriveWayIndex) {
    //         for (auto item : myEndingDriveways) {
    //             for (MSDriveWay* dw : item.second) {
    //                 myDriveWayRouteLookup[dw->myRoute] = dw;
    //             }
    //         }
    //     }
    //     MSVehicleControl& c = MSNet::getInstance()->getVehicleControl();
    //     bool ok;
    //     const std::string id  = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     const std::string edges = attrs.get<std::string>(SUMO_ATTR_EDGES, id.c_str(), ok);
    //     ConstMSEdgeVector route;
    //     if (attrs.hasAttribute(SUMO_ATTR_EDGES)) {
    //         MSEdge::parseEdgesList(edges, route, id);
    //     }
    //     // missing driveways and subdriveways can be ignored. They may have been created
    //     // for vehicles that are not relevant at state loading time
    //     MSDriveWay* dw = nullptr;
    //     if (tag == SUMO_TAG_DRIVEWAY) {
    //         auto it = myDriveWayRouteLookup.find(route);
    //         if (it == myDriveWayRouteLookup.end()) {
    //             //WRITE_WARNING(TLF("Unknown driveWay '%' with route '%'", id, edges));
    //             return;
    //         }
    //         dw = it->second;
    //         myDriveWayLookup[id] = dw;
    //     } else {
    //         std::string parentID = id.substr(0, id.rfind('.'));
    //         auto it = myDriveWayLookup.find(parentID);
    //         if (it == myDriveWayLookup.end()) {
    //             //WRITE_WARNING(TLF("Unknown parent driveway '%' for subDriveWay '%'", parentID, id));
    //             return;
    //         }
    //         MSDriveWay* parent = it->second;
    //         for (MSDriveWay* sub : parent->mySubDriveWays) {
    //             if (sub->myRoute == route) {
    //                 dw = sub;
    //                 break;
    //             }
    //         }
    //         if (dw == nullptr) {
    //             // missing subdriveways can be ignored. They may have been created
    //             // as foes for driveways that are not relevant at state loading time
    //             return;
    //         }
    //     }
    //     const std::string vehicles = attrs.getOpt<std::string>(SUMO_ATTR_VEHICLES, id.c_str(), ok, "");
    //     for (const std::string& vehID : StringTokenizer(vehicles).getVector()) {
    //         MSBaseVehicle* veh = dynamic_cast<MSBaseVehicle*>(c.getVehicle(vehID));
    //         if (veh == nullptr) {
    //             throw ProcessError(TLF("Unknown vehicle '%' in driveway '%'", vehID, id));
    //         }
    //         if (!dw->hasTrain(veh)) {
    //             dw->myTrains.insert(veh);
    //             veh->addReminder(dw);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn load_state(attrs: &SUMOSAXAttributes, tag: i32) {
        let lookup_size = MY_DRIVE_WAY_ROUTE_LOOKUP.with(|m| m.borrow().len()) as i32;
        if lookup_size < MY_GLOBAL_DRIVE_WAY_INDEX.with(|i| i.get()) {
            let ending: Vec<Vec<Ptr<MSDriveWay>>> =
                MY_ENDING_DRIVEWAYS.with(|m| m.borrow().values().cloned().collect());
            MY_DRIVE_WAY_ROUTE_LOOKUP.with(|m| {
                let mut lookup = m.borrow_mut();
                for driveways in ending {
                    for dw in driveways {
                        lookup.insert(dw.myRoute.clone(), dw);
                    }
                }
            });
        }
        let net = MSNet::get_instance();
        let c = net.m().get_vehicle_control();
        let mut ok = false;
        let id: String = attrs.get(SUMO_ATTR_ID, "", &mut ok, true);
        let edges: String = attrs.get(SUMO_ATTR_EDGES, &id, &mut ok, true);
        let mut route: ConstMSEdgeVector = Vec::new();
        if attrs.has_attribute(SUMO_ATTR_EDGES) {
            MSEdge::parse_edges_list(&edges, &mut route, &id);
        }
        // missing driveways and subdriveways can be ignored. They may have been created
        // for vehicles that are not relevant at state loading time
        let mut dw = Ptr::null();
        if tag == SUMO_TAG_DRIVEWAY {
            let found = MY_DRIVE_WAY_ROUTE_LOOKUP.with(|m| m.borrow().get(&route).copied());
            match found {
                Some(d) => {
                    dw = d;
                    MY_DRIVE_WAY_LOOKUP.with(|m| m.borrow_mut().insert(id.clone(), dw));
                }
                None => return,
            }
        } else {
            let parent_id = &id[..id.rfind('.').unwrap_or(id.len())];
            let parent = MY_DRIVE_WAY_LOOKUP.with(|m| m.borrow().get(parent_id).copied());
            let parent = match parent {
                Some(p) => p,
                None => return,
            };
            for &sub in parent.mySubDriveWays.iter() {
                if sub.myRoute == route {
                    dw = sub;
                    break;
                }
            }
            if dw.is_null() {
                // missing subdriveways can be ignored. They may have been created
                // as foes for driveways that are not relevant at state loading time
                return;
            }
        }
        let vehicles: String = attrs.get_opt(SUMO_ATTR_VEHICLES, &id, &mut ok, String::new(), true);
        let mut tokenizer = StringTokenizer::new(&vehicles);
        for veh_id in tokenizer.get_vector() {
            let veh = c.get_vehicle(&veh_id);
            if veh.is_null() {
                panic!(
                    "{}",
                    ProcessError::new(utils::tlf!("Unknown vehicle '%' in driveway '%'", veh_id, id))
                );
            }
            if !dw.has_train(veh) {
                dw.m().myTrains.insert(veh);
                veh.cast::<MSBaseVehicle>().m().add_reminder(dw.cast::<MSMoveReminder>(), 0.);
            }
        }
    }

    /// C++: `bool MSDriveWay::hasLinkConflict(const Approaching &, const MSLink *) const` [const, protected]
    /// Whether the approaching vehicle is prevent from driving by another vehicle approaching the given link
    // bool
    // MSDriveWay::hasLinkConflict(const Approaching& veh, const MSLink* foeLink) const {
    // #ifdef DEBUG_SIGNALSTATE_PRIORITY
    //     if (gDebugFlag4) {
    //         std::cout << "   checkLinkConflict foeLink=" << getTLLinkID(foeLink) << " ego=" << Named::getIDSecure(veh.first) << "\n";
    //     }
    // #endif
    //     if (foeLink->getApproaching().size() > 0) {
    //         Approaching foe = foeLink->getClosest();
    // #ifdef DEBUG_SIGNALSTATE_PRIORITY
    //         if (gDebugFlag4) {
    //             std::cout << "     approaching foe=" << foe.first->getID() << "\n";
    //         }
    // #endif
    //         if (foe.first == veh.first) {
    //             return false;
    //         }
    //         const MSTrafficLightLogic* foeTLL = foeLink->getTLLogic();
    //         assert(foeTLL != nullptr);
    //         const MSRailSignal* constFoeRS = dynamic_cast<const MSRailSignal*>(foeTLL);
    //         MSRailSignal* foeRS = const_cast<MSRailSignal*>(constFoeRS);
    //         if (foeRS != nullptr) {
    //             const MSDriveWay& foeDriveWay = foeRS->retrieveDriveWayForVeh(foeLink->getTLIndex(), foe.first);
    //             MSEdgeVector occupied;
    //             if (foeDriveWay.foeDriveWayOccupied(false, foe.first, occupied) ||
    //                     !foeRS->constraintsAllow(foe.first) ||
    //                     !overlap(foeDriveWay) ||
    //                     getFoeOrSubFoe(&foeDriveWay) == nullptr ||
    //                     canUseSiding(veh.first, getFoeOrSubFoe(&foeDriveWay)).first) {
    // #ifdef DEBUG_SIGNALSTATE_PRIORITY
    //                 if (gDebugFlag4 || veh.first->isSelected()) {
    //                     if (foeDriveWay.foeDriveWayOccupied(false, foe.first, occupied)) {
    //                         std::cout << "     foe blocked\n";
    //                     } else if (!foeRS->constraintsAllow(foe.first)) {
    //                         std::cout << "     foe constrained\n";
    //                     } else if (!overlap(foeDriveWay)) {
    //                         std::cout << "     no overlap with foeDW=" << foeDriveWay.getID() << "\n";
    //                     } else if (getFoeOrSubFoe(&foeDriveWay) == nullptr) {
    //                         std::cout << "     foeDW=" << foeDriveWay.getID() << " is not a foe to " << getID() << "\n";
    //                     } else if (canUseSiding(veh.first, &foeDriveWay).first) {
    //                         std::cout << "     use siding\n";
    //                     }
    //                 }
    // #endif
    //                 return false;
    //             }
    // #ifdef DEBUG_SIGNALSTATE_PRIORITY
    //             if (gDebugFlag4) {
    //                 std::cout
    //                         << "  aSB=" << veh.second.arrivalSpeedBraking << " foeASB=" << foe.second.arrivalSpeedBraking
    //                         << "  aT=" << veh.second.arrivalTime << " foeAT=" << foe.second.arrivalTime
    //                         << "  aS=" << veh.first->getSpeed() << " foeS=" << foe.first->getSpeed()
    //                         << "  aD=" << veh.second.dist << " foeD=" << foe.second.dist
    //                         << "  aW=" << veh.first->getWaitingTime() << " foeW=" << foe.first->getWaitingTime()
    //                         << "  aN=" << veh.first->getNumericalID() << " foeN=" << foe.first->getNumericalID()
    //                         << "\n";
    //             }
    // #endif
    //             const bool yield = mustYield(veh, foe);
    //             if (MSRailSignal::storeVehicles()) {
    //                 MSRailSignal::rivalVehicles().push_back(foe.first);
    //                 if (yield) {
    //                     MSRailSignal::priorityVehicles().push_back(foe.first);
    //                 }
    //             }
    //             return yield;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn has_link_conflict(&self, closest: &MSRailSignal_Approaching, foeLink: Ptr<MSLink>) -> bool {
        if !foeLink.get_approaching_2().is_empty() {
            let foe = foeLink.get_closest();
            if foe.0 == closest.0 {
                return false;
            }
            let foe_tll = foeLink.get_tl_logic();
            assert!(!foe_tll.is_null());
            let foe_rs = as_rail_signal(foe_tll);
            if !foe_rs.is_null() {
                let foe_drive_way: Ptr<MSDriveWay> =
                    Ptr::new(foe_rs.m().retrieve_drive_way_for_veh(foeLink.get_tl_index(), foe.0));
                let mut occupied: MSEdgeVector = Vec::new();
                let foe_or_sub_foe = self.get_foe_or_sub_foe(foe_drive_way);
                if foe_drive_way.foe_drive_way_occupied(false, foe.0, &mut occupied)
                    || !foe_rs.constraints_allow(foe.0, false)
                    || !self.overlap(&*foe_drive_way)
                    || foe_or_sub_foe.is_null()
                    || self.can_use_siding(closest.0, foe_or_sub_foe, Ptr::null()).0
                {
                    return false;
                }
                let yield_ = Self::must_yield(closest, &foe);
                if MSRailSignal::store_vehicles() {
                    MSRailSignal::rival_vehicles().push(foe.0);
                    if yield_ {
                        MSRailSignal::priority_vehicles().push(foe.0);
                    }
                }
                return yield_;
            }
        }
        false
    }

    /// C++: `bool MSDriveWay::overlap(const MSDriveWay &) const` [const, protected]
    /// Whether this driveway (route) overlaps with the given one
    // bool
    // MSDriveWay::overlap(const MSDriveWay& other) const {
    //     for (int i = 0; i < myCoreSize; i++) {
    //         for (int j = 0; j < other.myCoreSize; j++) {
    //             const MSEdge* edge = myRoute[i];
    //             const MSEdge* edge2 = other.myRoute[j];
    //             if (edge->getToJunction() == edge2->getToJunction()
    //                     || edge->getToJunction() == edge2->getFromJunction()
    //                     || edge->getFromJunction() == edge2->getFromJunction()) {
    //                 // XXX might be rail_crossing with parallel tracks
    //                 return true;
    //             }
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn overlap(&self, other: &MSDriveWay) -> bool {
        for i in 0..self.myCoreSize {
            for j in 0..other.myCoreSize {
                let edge = self.myRoute[i as usize];
                let edge2 = other.myRoute[j as usize];
                if edge.get_to_junction() == edge2.get_to_junction()
                    || edge.get_to_junction() == edge2.get_from_junction()
                    || edge.get_from_junction() == edge2.get_from_junction()
                {
                    // XXX might be rail_crossing with parallel tracks
                    return true;
                }
            }
        }
        false
    }

    /// C++: `void MSDriveWay::buildRoute(const MSLink *, MSRouteIterator, MSRouteIterator, LaneVisitedMap &, std::set<MSLink *, MSLink::ComparatorNumericalLaneIdLess> &)` [protected]
    // void
    // MSDriveWay::buildRoute(const MSLink* origin,
    //                        MSRouteIterator next, MSRouteIterator end,
    //                        LaneVisitedMap& visited,
    //                        std::set<MSLink*, MSLink::ComparatorNumericalLaneIdLess>& flankSwitches) {
    //     double length = 0;
    //     bool seekForwardSignal = true;
    //     bool seekBidiSwitch = true;
    //     bool foundUnsafeSwitch = false;
    //     MSLane* toLane = origin ? origin->getViaLaneOrLane() : (*next)->getLanes()[0];
    //     const std::string warnID = origin ? "rail signal " + getClickableTLLinkID(origin) : "insertion lane '" + toLane->getID() + "'";
    // #ifdef DEBUG_DRIVEWAY_BUILDROUTE
    //     if (gDebugFlag4) std::cout << "buildRoute origin=" << warnID << " vehRoute=" << toString(ConstMSEdgeVector(next, end))
    //                                    << " visited=" << formatVisitedMap(visited) << "\n";
    // #endif
    //     while (true) {
    //         if (length > MSGlobals::gMaxRailSignalBlockLength) {
    //             // typical block length in germany on main lines is 3-5km on branch lines up to 7km
    //             // special branches that are used by one train exclusively could also be up to 20km in length
    //             // minimum block size in germany is 37.5m (LZB)
    //             // larger countries (USA, Russia) might see blocks beyond 20km)
    //             if (seekForwardSignal && myBlockLengthWarnings.count(myRoute.front()) == 0) {
    //                 WRITE_WARNINGF("Block after % exceeds maximum length (stopped searching after edge '%' (length=%m).",
    //                                warnID, toLane->getEdge().getID(), length);
    //                 myBlockLengthWarnings.insert(myRoute.front());
    //             }
    //             myAbortedBuild = true;
    //             // length exceeded
    // #ifdef DEBUG_DRIVEWAY_BUILDROUTE
    //             if (gDebugFlag4) {
    //                 std::cout << " abort: length=" << length << "\n";
    //             }
    // #endif
    //             return;
    //         }
    // #ifdef DEBUG_DRIVEWAY_BUILDROUTE
    //         if (gDebugFlag4) {
    //             std::cout << "   toLane=" << toLane->getID() << " visited=" << formatVisitedMap(visited) << "\n";
    //         }
    // #endif
    //         const MSEdge* current = &toLane->getEdge();
    //         if (current->isNormal()) {
    //             myRoute.push_back(current);
    //             if (next != end) {
    //                 next++;
    //             }
    //         }
    //         appendMapIndex(visited, toLane);
    //         length += toLane->getLength();
    //         MSLane* bidi = toLane->getBidiLane();
    //         if (seekForwardSignal) {
    //             if (!foundUnsafeSwitch) {
    //                 myForward.push_back(toLane);
    //                 if (toLane->isNormal()) {
    //                     myForwardEdgeCount++;
    //                 }
    //                 if (myForward.size() == 1) {
    //                     myLane = toLane;
    //                     if (MSGlobals::gUseMesoSim) {
    //                         MESegment* s = MSGlobals::gMesoNet->getSegmentForEdge(myLane->getEdge());
    //                         s->addDetector(this, myLane->getIndex());
    //                     } else {
    //                         toLane->addMoveReminder(this, false);
    //                     }
    //                 }
    //             }
    //         } else if (bidi == nullptr) {
    //             if (toLane->isInternal() && toLane->getIncomingLanes().front().viaLink->isTurnaround()) {
    // #ifdef DEBUG_DRIVEWAY_BUILDROUTE
    //                 if (gDebugFlag4) {
    //                     std::cout << "      continue bidiSearch beyond turnaround\n";
    //                 }
    // #endif
    //             } else {
    //                 seekBidiSwitch = false;
    // #ifdef DEBUG_DRIVEWAY_BUILDROUTE
    //                 if (gDebugFlag4) {
    //                     std::cout << "      noBidi, abort search for bidiSwitch\n";
    //                 }
    // #endif
    //             }
    //         }
    //         if (bidi != nullptr) {
    //             if (!seekForwardSignal && !foundUnsafeSwitch && bidi->isNormal()) {
    //                 // look for switch that could protect from oncoming vehicles
    //                 for (const MSLink* const link : bidi->getLinkCont()) {
    //                     if (link->getDirection() == LinkDirection::TURN) {
    //                         continue;
    //                     }
    //                     if (!myBidi.empty() && link->getViaLaneOrLane() != myBidi.back()) {
    //                         myCoreSize = (int)myRoute.size() - 1;
    //                         MSLink* used = const_cast<MSLink*>(bidi->getLinkTo(myBidi.back()));
    // #ifdef DEBUG_DRIVEWAY_BUILDROUTE
    //                         if (gDebugFlag4) {
    //                             std::cout << "      found unsafe switch " << link->getDescription() << " (used=" << (used == nullptr ? "NULL" : used->getDescription()) << ")\n";
    //                         }
    // #endif
    //                         // trains along our route beyond this switch might create deadlock
    //                         foundUnsafeSwitch = true;
    //                         // the switch itself must still be guarded to ensure safety
    //                         if (used != nullptr) {
    //                             // possibly nullptr if there was an intermediate section of unidirectional edges
    //                             flankSwitches.insert(used);
    //                         }
    //                         break;
    //                     }
    //                 }
    //             }
    //             if (foundUnsafeSwitch) {
    //                 myBidiExtended.push_back(bidi);
    //             } else {
    //                 myBidi.push_back(bidi);
    //             }
    //         }
    //         const std::vector<MSLink*>& links = toLane->getLinkCont();
    //         toLane = nullptr;
    //         for (const MSLink* const link : links) {
    //             if ((next != end && &link->getLane()->getEdge() == *next)) {
    //                 toLane = link->getViaLaneOrLane();
    //                 if (link->getTLLogic() != nullptr && link->getTLIndex() >= 0 && link->getTLLogic()->getLogicType() == TrafficLightType::RAIL_SIGNAL) {
    //                     if (link == origin) {
    //                         if (seekForwardSignal) {
    //                             WRITE_WARNINGF(TL("Found circular block after % (% edges, length %)"), warnID, toString(myRoute.size()), toString(length));
    //                         }
    //                         //std::cout << getClickableTLLinkID(origin) << " circularBlock2=" << toString(myRoute) << "\n";
    //                         myAbortedBuild = true;
    // #ifdef DEBUG_DRIVEWAY_BUILDROUTE
    //                         if (gDebugFlag4) {
    //                             std::cout << " abort: found circle\n";
    //                         }
    // #endif
    //                         return;
    //                     }
    //                     seekForwardSignal = false;
    //                     myFoundSignal = true;
    //                     seekBidiSwitch = bidi != nullptr;
    // #ifdef DEBUG_DRIVEWAY_BUILDROUTE
    //                     if (gDebugFlag4) {
    //                         std::cout << "      found forwardSignal " << link->getTLLogic()->getID() << " seekBidiSwitch=" << seekBidiSwitch << "\n";
    //                     }
    // #endif
    //                 }
    //                 //if (links.size() > 1 && !foundUnsafeSwitch) {
    //                 if (isSwitch(link)) {
    //                     // switch on driveway
    //                     //std::cout << "mySwitchDriveWays " << getID() << " link=" << link->getDescription() << "\n";
    //                     mySwitchDriveWays[link].push_back(this);
    //                 }
    //                 if (link->getLane()->getBidiLane() != nullptr && &link->getLane()->getEdge() == current->getBidiEdge()) {
    //                     // reversal on driveway
    //                     myReversalDriveWays[current].push_back(this);
    //                     myReversals.push_back(current);
    //                 }
    //                 break;
    //             }
    //         }
    //         if (toLane == nullptr) {
    //             if (next != end) {
    //                 // no connection found, jump to next route edge
    //                 toLane = (*next)->getLanes()[0];
    // #ifdef DEBUG_DRIVEWAY_BUILDROUTE
    //                 if (gDebugFlag4) {
    //                     std::cout << "      abort: turn-around or jump\n";
    //                 }
    // #endif
    //                 myFoundJump = true;
    //                 return;
    //             } else {
    // #ifdef DEBUG_DRIVEWAY_BUILDROUTE
    //                 if (gDebugFlag4) {
    //                     std::cout << "      abort: no next lane available\n";
    //                 }
    // #endif
    //                 myTerminateRoute = true;
    //                 return;
    //             }
    //         }
    //     }
    //     myBidiEnded = !seekBidiSwitch;
    // #ifdef DEBUG_DRIVEWAY_BUILDROUTE
    //     if (gDebugFlag4) {
    //         std::cout << " normalEnd myBidiEnded=" << myBidiEnded << "\n";
    //     }
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn build_route(&mut self, route: &ConstMSEdgeVector, origin: Ptr<MSLink>, next: MSRouteIterator, end: MSRouteIterator, visited: &mut MSRailSignal_LaneVisitedMap, flankSwitches: &mut BTreeSet<Ptr<MSLink>>) {
        let mut length = 0.0;
        let mut seek_forward_signal = true;
        let mut seek_bidi_switch = true;
        let mut found_unsafe_switch = false;
        let mut next = next;
        let mut to_lane: Ptr<MSLane> = if !origin.is_null() {
            origin.get_via_lane_or_lane()
        } else {
            route[next].get_lanes().first().copied().unwrap_or(Ptr::null())
        };
        let warn_id = if !origin.is_null() {
            format!("rail signal {}", Self::get_clickable_tl_link_id(origin))
        } else {
            format!("insertion lane '{}'", to_lane.get_id())
        };
        loop {
            if length > MSGlobals::max_rail_signal_block_length() {
                // typical block length in germany on main lines is 3-5km on branch lines up to 7km
                // special branches that are used by one train exclusively could also be up to 20km in length
                // minimum block size in germany is 37.5m (LZB)
                // larger countries (USA, Russia) might see blocks beyond 20km)
                let block_start = self.myRoute.first().copied();
                let already_warned = match block_start {
                    Some(e) => MY_BLOCK_LENGTH_WARNINGS.with(|w| w.borrow().contains(&e)),
                    None => false,
                };
                if seek_forward_signal && !already_warned {
                    utils::write_warningf!(
                        "Block after % exceeds maximum length (stopped searching after edge '%' (length=%m).",
                        warn_id.clone(),
                        to_lane.get_edge().get_id().to_string(),
                        length
                    );
                    if let Some(e) = block_start {
                        MY_BLOCK_LENGTH_WARNINGS.with(|w| w.borrow_mut().insert(e));
                    }
                }
                self.myAbortedBuild = true;
                // length exceeded
                return;
            }
            let current = Ptr::new(to_lane.get_edge());
            if current.is_normal() {
                self.myRoute.push(current);
                if next != end {
                    next += 1;
                }
            }
            Self::append_map_index(visited, to_lane);
            length += to_lane.get_length();
            let bidi = to_lane.get_bidi_lane();
            if seek_forward_signal {
                if !found_unsafe_switch {
                    self.myForward.push(to_lane);
                    if to_lane.is_normal() {
                        self.myForwardEdgeCount += 1;
                    }
                    if self.myForward.len() == 1 {
                        self.base.myLane = to_lane;
                        let this_ptr = Ptr::from_mut(self).cast::<MSMoveReminder>();
                        if MSGlobals::use_meso_sim() {
                            let edge = to_lane.get_edge();
                            let s = crate::microsim::ms_globals::meso_net().m().get_segment_for_edge(&edge, 0.);
                            s.m().add_detector(this_ptr, self.base.myLane.get_index());
                        } else {
                            to_lane.m().add_move_reminder(this_ptr, false);
                        }
                    }
                }
            } else if bidi.is_null() {
                if to_lane.is_internal() && to_lane.get_incoming_lanes().first().unwrap().viaLink.is_turnaround() {
                    // continue bidiSearch beyond turnaround
                } else {
                    seek_bidi_switch = false;
                    // noBidi, abort search for bidiSwitch
                }
            }
            if !bidi.is_null() {
                if !seek_forward_signal && !found_unsafe_switch && bidi.is_normal() {
                    // look for switch that could protect from oncoming vehicles
                    for &link in bidi.get_link_cont().iter() {
                        if link.get_direction() == LinkDirection::TURN {
                            continue;
                        }
                        if !self.myBidi.is_empty() && link.get_via_lane_or_lane() != *self.myBidi.last().unwrap() {
                            self.myCoreSize = self.myRoute.len() as i32 - 1;
                            let used = bidi.get_link_to(*self.myBidi.last().unwrap());
                            // trains along our route beyond this switch might create deadlock
                            found_unsafe_switch = true;
                            // the switch itself must still be guarded to ensure safety
                            if !used.is_null() {
                                // possibly nullptr if there was an intermediate section of unidirectional edges
                                flankSwitches.insert(used);
                            }
                            break;
                        }
                    }
                }
                if found_unsafe_switch {
                    self.myBidiExtended.push(bidi);
                } else {
                    self.myBidi.push(bidi);
                }
            }
            let links: Vec<Ptr<MSLink>> = to_lane.get_link_cont().clone();
            let mut next_lane = Ptr::null();
            for &link in links.iter() {
                if next != end && Ptr::new(link.get_lane().get_edge()) == route[next] {
                    next_lane = link.get_via_lane_or_lane();
                    if !link.get_tl_logic().is_null()
                        && link.get_tl_index() >= 0
                        && link.get_tl_logic().get_logic_type() == TrafficLightType::RAIL_SIGNAL
                    {
                        if link == origin {
                            if seek_forward_signal {
                                utils::write_warningf!(
                                    "Found circular block after % (% edges, length %)",
                                    warn_id.clone(),
                                    utils::common::to_string::to_string(&self.myRoute.len()),
                                    utils::common::to_string::to_string(&length)
                                );
                            }
                            self.myAbortedBuild = true;
                            return;
                        }
                        seek_forward_signal = false;
                        self.myFoundSignal = true;
                        seek_bidi_switch = !bidi.is_null();
                    }
                    if Self::is_switch(link) {
                        // switch on driveway
                        let this_ptr = Ptr::from_mut(self);
                        MY_SWITCH_DRIVE_WAYS.with(|m| m.borrow_mut().entry(link).or_default().push(this_ptr));
                    }
                    if !link.get_lane().get_bidi_lane().is_null() && Ptr::new(link.get_lane().get_edge()) == current.get_bidi_edge() {
                        // reversal on driveway
                        let this_ptr = Ptr::from_mut(self);
                        MY_REVERSAL_DRIVE_WAYS.with(|m| m.borrow_mut().entry(current).or_default().push(this_ptr));
                        self.myReversals.push(current);
                    }
                    break;
                }
            }
            if next_lane.is_null() {
                if next != end {
                    // no connection found, jump to next route edge
                    to_lane = route[next].get_lanes().first().copied().unwrap_or(Ptr::null());
                    self.myFoundJump = true;
                    return;
                } else {
                    self.myTerminateRoute = true;
                    return;
                }
            }
            to_lane = next_lane;
        }
    }

    /// C++: `void MSDriveWay::checkFlanks(const MSLink *, const std::vector<const MSLane *> &, const LaneVisitedMap &, bool, bool, std::set<MSLink *, MSLink::ComparatorNumericalLaneIdLess> &) const` [const, protected]
    // void
    // MSDriveWay::checkFlanks(const MSLink* originLink, const std::vector<const MSLane*>& lanes, const LaneVisitedMap& visited,
    //                         bool allFoes, bool movingBlock, std::set<MSLink*, MSLink::ComparatorNumericalLaneIdLess>& flankSwitches) const {
    // #ifdef DEBUG_CHECK_FLANKS
    //     if (gDebugFlag4) {
    //         std::cout << " checkFlanks lanes=" << toString(lanes) << " allFoes=" << allFoes << "\n";
    //     }
    // #endif
    //     const MSLink* reverseOriginLink = originLink != nullptr && originLink->getLane()->getBidiLane() != nullptr && originLink->getLaneBefore()->getBidiLane() != nullptr
    //                                       ? originLink->getLane()->getBidiLane()->getLinkTo(originLink->getLaneBefore()->getBidiLane())
    //                                       : nullptr;
    //     //std::cout << "   originLink=" << originLink->getDescription() << "\n";
    //     if (reverseOriginLink != nullptr) {
    //         reverseOriginLink = reverseOriginLink->getCorrespondingExitLink();
    //         //std::cout << "   reverseOriginLink=" << reverseOriginLink->getDescription() << "\n";
    //     }
    //     for (int i = 0; i < (int)lanes.size(); i++) {
    //         const MSLane* lane = lanes[i];
    //         const MSLane* prev = i > 0 ? lanes[i - 1] : nullptr;
    //         const MSLane* next = i + 1 < (int)lanes.size() ? lanes[i + 1] : nullptr;
    //         if (lane->isInternal()) {
    //             continue;
    //         }
    //         for (auto ili : lane->getIncomingLanes()) {
    //             if (ili.viaLink == originLink
    //                     || ili.viaLink == reverseOriginLink
    //                     || ili.viaLink->getDirection() == LinkDirection::TURN
    //                     || ili.viaLink->getDirection() == LinkDirection::TURN_LEFTHAND
    //                     || (originLink == nullptr && i == 0 && movingBlock)) {
    //                 continue;
    //             }
    //             if (ili.lane != prev && ili.lane != next) {
    // #ifdef DEBUG_CHECK_FLANKS
    //                 if (gDebugFlag4) {
    //                     std::cout << " add flankSwitch junction=" << ili.viaLink->getJunction()->getID() << " index=" << ili.viaLink->getIndex() << " iLane=" << ili.lane->getID() << " prev=" << Named::getIDSecure(prev) <<  " targetLane=" << lane->getID() << " next=" << Named::getIDSecure(next) << "\n";
    //                 }
    // #endif
    //                 flankSwitches.insert(ili.viaLink);
    //             } else if (allFoes) {
    //                 // link is part of the driveway, find foes that cross the driveway without entering
    //                 checkCrossingFlanks(ili.viaLink, visited, flankSwitches);
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn check_flanks(&self, originLink: Ptr<MSLink>, lanes: &Vec<Ptr<MSLane>>, visited: &MSRailSignal_LaneVisitedMap, allFoes: bool, movingBlock: bool, flankSwitches: &mut BTreeSet<Ptr<MSLink>>) {
        let reverse_origin_link = if !originLink.is_null()
            && !originLink.get_lane().get_bidi_lane().is_null()
            && !originLink.get_lane_before().get_bidi_lane().is_null()
        {
            originLink
                .get_lane()
                .get_bidi_lane()
                .get_link_to(originLink.get_lane_before().get_bidi_lane())
        } else {
            Ptr::null()
        };
        let reverse_origin_link = if !reverse_origin_link.is_null() {
            reverse_origin_link.get_corresponding_exit_link()
        } else {
            Ptr::null()
        };
        for i in 0..lanes.len() {
            let lane = lanes[i];
            let prev = if i > 0 { Some(lanes[i - 1]) } else { None };
            let next = if i + 1 < lanes.len() { Some(lanes[i + 1]) } else { None };
            if lane.is_internal() {
                continue;
            }
            for ili in lane.get_incoming_lanes().iter() {
                if ili.viaLink == originLink
                    || ili.viaLink == reverse_origin_link
                    || ili.viaLink.get_direction() == LinkDirection::TURN
                    || ili.viaLink.get_direction() == LinkDirection::TURN_LEFTHAND
                    || (originLink.is_null() && i == 0 && movingBlock)
                {
                    continue;
                }
                if Some(ili.lane) != prev && Some(ili.lane) != next {
                    flankSwitches.insert(ili.viaLink);
                } else if allFoes {
                    // link is part of the driveway, find foes that cross the driveway without entering
                    self.check_crossing_flanks(ili.viaLink, visited, flankSwitches);
                }
            }
        }
    }

    /// C++: `void MSDriveWay::checkCrossingFlanks(MSLink *, const LaneVisitedMap &, std::set<MSLink *, MSLink::ComparatorNumericalLaneIdLess> &) const` [const, protected]
    // void
    // MSDriveWay::checkCrossingFlanks(MSLink* dwLink, const LaneVisitedMap& visited, std::set<MSLink*, MSLink::ComparatorNumericalLaneIdLess>& flankSwitches) const {
    // #ifdef DEBUG_CHECK_FLANKS
    //     if (gDebugFlag4) {
    //         std::cout << "  checkCrossingFlanks  dwLink=" << dwLink->getDescription() << " visited=" << formatVisitedMap(visited) << "\n";
    //     }
    // #endif
    //     const MSJunction* junction = dwLink->getJunction();
    //     if (junction == nullptr) {
    //         return; // unregulated junction;
    //     }
    //     const MSJunctionLogic* logic = junction->getLogic();
    //     if (logic == nullptr) {
    //         return; // unregulated junction;
    //     }
    //     for (const MSEdge* in : junction->getIncoming()) {
    //         if (in->isInternal()) {
    //             continue;
    //         }
    //         for (MSLane* inLane : in->getLanes()) {
    //             const MSLane* inBidi = inLane->getBidiLane();
    //             if (isRailwayOrShared(inLane->getPermissions()) && visited.count(inLane) == 0 && (inBidi == nullptr || visited.count(inBidi) == 0)) {
    //                 for (MSLink* link : inLane->getLinkCont()) {
    //                     if (link->getIndex() >= 0 && logic->getFoesFor(dwLink->getIndex()).test(link->getIndex())
    //                             && visited.count(link->getLane()) == 0) {
    // #ifdef DEBUG_CHECK_FLANKS
    //                         if (gDebugFlag4) {
    //                             std::cout << " add crossing flankSwitch junction=" << junction->getID() << " index=" << link->getIndex() << "\n";
    //                         }
    // #endif
    //                         if (link->getViaLane() == nullptr) {
    //                             flankSwitches.insert(link);
    //                         } else {
    //                             flankSwitches.insert(link->getViaLane()->getLinkCont().front());
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn check_crossing_flanks(&self, dwLink: Ptr<MSLink>, visited: &MSRailSignal_LaneVisitedMap, flankSwitches: &mut BTreeSet<Ptr<MSLink>>) {
        let junction = dwLink.get_junction();
        if junction.is_null() {
            return; // unregulated junction;
        }
        let logic = junction.get_logic();
        if logic.is_null() {
            return; // unregulated junction;
        }
        for &in_edge in junction.get_incoming().iter() {
            if in_edge.is_internal() {
                continue;
            }
            for &in_lane in in_edge.get_lanes().iter() {
                let in_bidi = in_lane.get_bidi_lane();
                if is_railway_or_shared(in_lane.get_permissions())
                    && !visited.contains_key(&in_lane)
                    && (in_bidi.is_null() || !visited.contains_key(&in_bidi))
                {
                    for &link in in_lane.get_link_cont().iter() {
                        if link.get_index() >= 0
                            && link_bits_test(logic.get_foes_for(dwLink.get_index()), link.get_index())
                            && !visited.contains_key(&link.get_lane())
                        {
                            if link.get_via_lane().is_null() {
                                flankSwitches.insert(link);
                            } else {
                                flankSwitches.insert(*link.get_via_lane().get_link_cont().first().unwrap());
                            }
                        }
                    }
                }
            }
        }
    }

    /// C++: `void MSDriveWay::findFlankProtection(MSLink *, MSLink *, const std::vector<const MSLane *> &)` [protected]
    // void
    // MSDriveWay::findFlankProtection(MSLink* link, MSLink* origLink, const std::vector<const MSLane*>& flank) {
    // #ifdef DEBUG_CHECK_FLANKS
    //     if (gDebugFlag4) {
    //         std::cout << "  findFlankProtection link=" << link->getDescription() << " origLink=" << origLink->getDescription() << "\n";
    //     }
    // #endif
    //     if (link->getCorrespondingEntryLink()->getTLLogic() != nullptr && link->getJunction()->getType() == SumoXMLNodeType::RAIL_SIGNAL) {
    //         MSLink* entry = const_cast<MSLink*>(link->getCorrespondingEntryLink());
    //         // guarded by signal
    // #ifdef DEBUG_CHECK_FLANKS
    //         if (gDebugFlag4) {
    //             std::cout << "   flank guarded by " << entry->getTLLogic()->getID() << "\n";
    //         }
    // #endif
    //         // @note, technically it's enough to collect links from foe driveways
    //         // but this also adds "unused" conflict links which may aid comprehension
    //         myConflictLinks.push_back(entry);
    //         addFoes(entry);
    //     } else {
    //         const MSLane* lane = link->getLaneBefore();
    //         std::vector<MSLink*> predLinks;
    //         for (auto ili : lane->getIncomingLanes()) {
    //             if (!ili.viaLink->isTurnaround()) {
    //                 predLinks.push_back(ili.viaLink);
    //             }
    //         }
    //         if (predLinks.size() > 1) {
    //             // this is a switch
    // #ifdef DEBUG_ADD_FOES
    //             if (gDebugFlag4) {
    //                 std::cout << "    predecessors of " << link->getDescription() << " isSwitch\n";
    //             }
    // #endif
    //             addSwitchFoes(link);
    //             for (MSLink* pred : predLinks) {
    //                 addSwitchFoes(pred);
    //             }
    //         } else if (predLinks.size() == 1) {
    //             if (isSwitch(link)) {
    //                 addSwitchFoes(link);
    //             } else {
    //                 // continue upstream via single predecessor
    //                 findFlankProtection(predLinks.front(), origLink, flank);
    //             }
    //         }
    //         // check for insertions
    //         if (myDepartureDriveways.count(&lane->getEdge()) != 0) {
    //             for (MSDriveWay* foe : myDepartureDriveways[&lane->getEdge()]) {
    //                 if (flankConflict(*foe) || crossingConflict(*foe)) {
    // #ifdef DEBUG_ADD_FOES
    //                     if (gDebugFlag4) {
    //                         std::cout << "  foe " << foe->getID() << " departs on flank=" << lane->getID() << "\n";
    //                     }
    // #endif
    //                     myFoes.push_back(foe);
    //                 } else {
    // #ifdef DEBUG_ADD_FOES
    //                     if (gDebugFlag4) {
    //                         std::cout << "  cand foe " << foe->getID() << " departs on flank=" << lane->getID() << " rejected\n";
    //                     }
    // #endif
    //                 }
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn find_flank_protection(&mut self, link: Ptr<MSLink>, origLink: Ptr<MSLink>, flank: &Vec<Ptr<MSLane>>) {
        if !link.get_corresponding_entry_link().get_tl_logic().is_null()
            && link.get_junction().get_type() == SumoXMLNodeType::RAIL_SIGNAL
        {
            let entry = link.get_corresponding_entry_link();
            // guarded by signal
            // @note, technically it's enough to collect links from foe driveways
            // but this also adds "unused" conflict links which may aid comprehension
            self.myConflictLinks.push(entry);
            self.add_foes(entry);
        } else {
            let lane = link.get_lane_before();
            let mut pred_links: Vec<Ptr<MSLink>> = Vec::new();
            for ili in lane.get_incoming_lanes().iter() {
                if !ili.viaLink.is_turnaround() {
                    pred_links.push(ili.viaLink);
                }
            }
            if pred_links.len() > 1 {
                // this is a switch
                self.add_switch_foes(link);
                for &pred in pred_links.iter() {
                    self.add_switch_foes(pred);
                }
            } else if pred_links.len() == 1 {
                if Self::is_switch(link) {
                    self.add_switch_foes(link);
                } else {
                    // continue upstream via single predecessor
                    self.find_flank_protection(pred_links[0], origLink, flank);
                }
            }
            // check for insertions
            let edge = Ptr::new(lane.get_edge());
            let departures = MY_DEPARTURE_DRIVEWAYS.with(|m| m.borrow().get(&edge).cloned().unwrap_or_default());
            for foe in departures {
                if self.flank_conflict(&*foe) || self.crossing_conflict(&*foe) {
                    self.myFoes.push(foe);
                }
            }
        }
    }

    /// C++: `void MSDriveWay::addFoes(const MSLink *)` [protected]
    /// add all driveWays that start at the given link as foes
    // void
    // MSDriveWay::addFoes(const MSLink* link) {
    // #ifdef DEBUG_ADD_FOES
    //     if (gDebugFlag4) {
    //         std::cout << "driveway " << myID << " addFoes for link " << link->getDescription() << "\n";
    //     }
    // #endif
    //     const MSRailSignal* rs = dynamic_cast<const MSRailSignal*>(link->getTLLogic());
    //     if (rs != nullptr) {
    //         for (MSDriveWay* foe : rs->retrieveDriveWays(link->getTLIndex())) {
    // #ifdef DEBUG_ADD_FOES
    //             if (gDebugFlag4) {
    //                 std::cout << "  cand foe=" << foe->myID << " fc1=" << flankConflict(*foe) << " fc2=" << foe->flankConflict(*this) << " cc1=" << crossingConflict(*foe) << " cc2=" <<  foe->crossingConflict(*this) << "\n";
    //             }
    // #endif
    //             if (foe != this && (flankConflict(*foe) || foe->flankConflict(*this) || crossingConflict(*foe) || foe->crossingConflict(*this))) {
    // #ifdef DEBUG_ADD_FOES
    //                 if (gDebugFlag4) {
    //                     std::cout << "   foe=" << foe->myID << "\n";
    //                 }
    // #endif
    //                 myFoes.push_back(foe);
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_foes(&mut self, link: Ptr<MSLink>) {
        let rs = as_rail_signal(link.get_tl_logic());
        if !rs.is_null() {
            let this_ptr = Ptr::new(&*self);
            for foe in rs.retrieve_drive_ways(link.get_tl_index()) {
                if foe != this_ptr
                    && (self.flank_conflict(&*foe)
                        || foe.flank_conflict(&*self)
                        || self.crossing_conflict(&*foe)
                        || foe.crossing_conflict(&*self))
                {
                    self.myFoes.push(foe);
                }
            }
        }
    }

    /// C++: `void MSDriveWay::addSidings(MSDriveWay *, bool)` [protected]
    /// add sidings for the given foe
    // void
    // MSDriveWay::addSidings(MSDriveWay* foe, bool addToFoe) {
    //     const ConstMSEdgeVector& foeRoute = foe->isSubDriveWay() ? foe->myParent->myRoute : foe->myRoute;
    //     const MSEdge* foeEndBidi = foe->myForward.back()->getEdge().getBidiEdge();
    //     int foeForwardNormals = 0;
    //     for (auto lane : foe->myForward) {
    //         if (lane->isNormal()) {
    //             foeForwardNormals++;
    //         }
    //     }
    //     if (foeForwardNormals == (int)foeRoute.size()) {
    // #ifdef DEBUG_BUILD_SIDINGS
    //         if (gDebugFlag4) {
    //             std::cout << "checkSiding " << getID() << " foe=" << foe->getID() << " foeForwardNormals=" << foeForwardNormals << " frSize=" << foeRoute.size() <<  " aborted\n";
    //         }
    // #endif
    //         return;
    //     }
    //     auto foeSearchBeg = foeRoute.begin() + foeForwardNormals;
    //     auto foeSearchEnd = foeRoute.end();
    //     if (foeEndBidi == nullptr) {
    //         throw ProcessError("checkSiding " + getID() + " foe=" + foe->getID() + " noBidi\n");
    //     }
    //     // if foe is a subDriveway, the forward section may end on an internal edge which would not be found on myRoute
    //     foeEndBidi = foeEndBidi->getNormalSuccessor();
    //     std::set<const MSEdge*> foeForwardEdges(foeRoute.begin(), foeRoute.begin() + foeForwardNormals);
    //     int forwardNormals = 0;
    //     for (auto lane : myForward) {
    //         if (lane->isNormal()) {
    //             forwardNormals++;
    //             if (foeForwardEdges.count(&lane->getEdge()) != 0) {
    // #ifdef DEBUG_BUILD_SIDINGS
    //                 if (gDebugFlag4) {
    //                     std::cout << "checkSiding " << getID() << " foe=" << foe->getID() << " forwardEdge=" << lane->getEdge().getID() << " on foeForward (sidings unsafe)\n";
    //                 }
    // #endif
    //                 return;
    //             }
    //         }
    //     }
    //     int i;
    //     std::vector<int> start;
    //     std::vector<double> length;
    //     std::vector<std::vector<int> > intermediateRS;
    //     for (i = 0; i < (int)myRoute.size(); i++) {
    //         if (myRoute[i] == foeEndBidi) {
    //             break;
    //         }
    //     }
    //     if (i == (int)myRoute.size()) {
    //         throw ProcessError("checkSiding " + getID() + " foe=" + foe->getID() + " foeEndBidi=" + foeEndBidi->getID() + " not on route\n");
    //     }
    //     const MSEdge* next = myRoute[i];
    // #ifdef DEBUG_BUILD_SIDINGS
    //     if (gDebugFlag4) {
    //         std::cout << "checkSiding " << getID() << " foe=" << foe->getID() << " i=" << i << " next=" << next->getID() << " foeForwardNormals=" << foeForwardNormals << " frSize=" << foeRoute.size() << " foeSearchBeg=" << (*foeSearchBeg)->getID() << " addToFoe=" << addToFoe << "\n";
    //     }
    // #endif
    //     i--;
    //     // look backward along our route starting at the final edge of the foe dw
    //     for (; i >= 0; i--) {
    //         const MSEdge* cur = myRoute[i];
    //         const bool curHasRS = hasRS(cur, next);
    //         if (curHasRS) {
    //             if (std::find(foeSearchBeg, foeSearchEnd, cur->getBidiEdge()) == foeSearchEnd) {
    //                 // we found a rail signal in a safe spot (not on the foe route)
    //                 start.push_back(i);
    //                 length.push_back(0);
    //                 intermediateRS.push_back({});
    //             }
    //         }
    //         if (!start.empty()) {
    //             // move further backwards along the route to find the spot where it merges again with the foe route
    //             auto itFind = std::find(foeSearchBeg, foeSearchEnd, cur->getBidiEdge());
    //             if (itFind != foeSearchEnd) {
    // #ifdef DEBUG_BUILD_SIDINGS
    //                 if (gDebugFlag4) {
    //                     std::cout << "endSiding " << getID() << " foe=" << foe->getID() << " i=" << i << " curBidi=" << Named::getIDSecure(cur->getBidiEdge()) << " length=" << toString(length) << "\n";
    //                 }
    // #endif
    //                 const int firstIndex = i + 1;
    //                 if (addToFoe) {
    //                     auto& foeSidings = foe->mySidings[this];
    //                     // indices must be mapped onto foe route;
    //                     const MSEdge* first = myRoute[firstIndex];
    //                     auto itFirst = std::find(foeRoute.begin(), foeRoute.end(), first);
    //                     if (itFirst != foeRoute.end()) {
    //                         for (int j = 0; j < (int)length.size(); j++) {
    //                             const MSEdge* last = myRoute[start[j]];
    //                             auto itLast = std::find(itFirst, foeRoute.end(), last);
    //                             if (itLast != foeRoute.end()) {
    //                                 // @todo are intermediateRS relevant here?
    //                                 foeSidings.insert(foeSidings.begin(), Siding((int)(itFirst - foeRoute.begin()), (int)(itLast - foeRoute.begin()), length[j], {}));
    // #ifdef DEBUG_BUILD_SIDINGS
    //                                 if (gDebugFlag4) {
    //                                     std::cout << "   found endSiding start=" << foeSidings.front().start << " end=" << foeSidings.front().end << " length=" << foeSidings.front().length << "\n";
    //                                 }
    // #endif
    //                                 if (foeSidings.size() > 2) {
    //                                     foeSidings.pop_back();  // only need the first 2 sidings
    //                                 }
    //                             }
    //                         }
    //                     }
    //                 } else {
    //                     // pick up further rail signals between the start of the siding and the start of the current driveway
    //                     std::vector<int> furtherRS;
    //                     if (curHasRS) {
    //                         furtherRS.push_back(i);
    //                     }
    //                     const MSEdge* next2 = cur;
    //                     for (int i2 = i - 1; i2 >= forwardNormals; i2--) {
    //                         const MSEdge* cur2 = myRoute[i2];
    //                         if (hasRS(cur2, next2)) {
    //                             furtherRS.push_back(i2);
    //                         }
    //                         next2 = cur2;
    //                     }
    //                     auto& foeSidings = mySidings[foe];
    //                     for (int j = 0; j < (int)length.size(); j++) {
    //                         intermediateRS[j].insert(intermediateRS[j].end(), furtherRS.begin(), furtherRS.end());
    //                         foeSidings.insert(foeSidings.begin(), Siding(firstIndex, start[j], length[j], intermediateRS[j]));
    // #ifdef DEBUG_BUILD_SIDINGS
    //                         if (gDebugFlag4) {
    //                             std::cout << "   found endSiding start=" << foeSidings.front().start << " end=" << foeSidings.front().end << " length=" << foeSidings.front().length << "\n";
    //                         }
    // #endif
    //                         if (foeSidings.size() > 2) {
    //                             foeSidings.pop_back();  // only need the first 2 sidings
    //                         }
    //                     }
    //                 }
    //                 start.clear();
    //                 length.clear();
    //                 intermediateRS.clear();
    //                 foeSearchBeg = itFind;
    //             } else {
    //                 for (int j = 0; j < (int)length.size(); j++) {
    //                     length[j] += cur->getLength();
    //                     if (curHasRS && i != start[j]) {
    //                         intermediateRS[j].push_back(i);
    //                     }
    //                 }
    //             }
    //         }
    //         next = cur;
    //     }
    //     // register in mySidingDriveways
    //     MSDriveWay* dw = addToFoe ? foe : this;
    //     MSDriveWay* dwFoe = addToFoe ? this : foe;
    //     auto it = dw->mySidings.find(dwFoe);
    //     if (it != dw->mySidings.end()) {
    //         for (const auto& siding : it->second) {
    //             mySidingDriveways[dw->myRoute[siding.end]].push_back(std::make_pair(dw, dwFoe));
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_sidings(&mut self, foe: Ptr<MSDriveWay>, addToFoe: bool) {
        let self_ptr = Ptr::from_mut(self);
        let foe_route: Vec<Ptr<MSEdge>> = if foe.is_sub_drive_way() {
            foe.myParent.myRoute.clone()
        } else {
            foe.myRoute.clone()
        };
        let mut foe_end_bidi = foe.myForward.last().unwrap().get_edge().get_bidi_edge();
        let mut foe_forward_normals = 0;
        for &lane in foe.myForward.iter() {
            if lane.is_normal() {
                foe_forward_normals += 1;
            }
        }
        if foe_forward_normals == foe_route.len() as i32 {
            return;
        }
        let mut foe_search_beg = foe_forward_normals as usize;
        let foe_search_end = foe_route.len();
        if foe_end_bidi.is_null() {
            panic!(
                "{}",
                ProcessError::new(format!(
                    "checkSiding {} foe={} noBidi\n",
                    self.base2.get_id(),
                    foe.base2.get_id()
                ))
            );
        }
        // if foe is a subDriveway, the forward section may end on an internal edge which would not be found on myRoute
        foe_end_bidi = foe_end_bidi.get_normal_successor();
        let foe_forward_edges: BTreeSet<Ptr<MSEdge>> =
            foe_route[..foe_forward_normals as usize].iter().copied().collect();
        let mut forward_normals = 0;
        for &lane in self.myForward.iter() {
            if lane.is_normal() {
                forward_normals += 1;
                if foe_forward_edges.contains(&Ptr::new(lane.get_edge())) {
                    return;
                }
            }
        }
        let mut i = 0;
        let mut start: Vec<i32> = Vec::new();
        let mut length: Vec<f64> = Vec::new();
        let mut intermediate_rs: Vec<Vec<i32>> = Vec::new();
        while i < self.myRoute.len() as i32 {
            if self.myRoute[i as usize] == foe_end_bidi {
                break;
            }
            i += 1;
        }
        if i == self.myRoute.len() as i32 {
            panic!(
                "{}",
                ProcessError::new(format!(
                    "checkSiding {} foe={} foeEndBidi={} not on route\n",
                    self.base2.get_id(),
                    foe.base2.get_id(),
                    foe_end_bidi.get_id()
                ))
            );
        }
        let mut next = self.myRoute[i as usize];
        i -= 1;
        // look backward along our route starting at the final edge of the foe dw
        while i >= 0 {
            let cur = self.myRoute[i as usize];
            let cur_has_rs = Self::has_rs(cur, next);
            if cur_has_rs {
                let found = if foe_search_beg < foe_search_end {
                    foe_route[foe_search_beg..].iter().position(|e| *e == cur.get_bidi_edge())
                } else {
                    None
                };
                if found.is_none() {
                    // we found a rail signal in a safe spot (not on the foe route)
                    start.push(i);
                    length.push(0.0);
                    intermediate_rs.push(Vec::new());
                }
            }
            if !start.is_empty() {
                // move further backwards along the route to find the spot where it merges again with the foe route
                let it_find = if foe_search_beg < foe_search_end {
                    foe_route[foe_search_beg..].iter().position(|e| *e == cur.get_bidi_edge())
                } else {
                    None
                };
                if let Some(rel) = it_find {
                    let it_find = foe_search_beg + rel;
                    let first_index = i + 1;
                    if addToFoe {
                        // indices must be mapped onto foe route;
                        let first = self.myRoute[first_index as usize];
                        let it_first = foe_route.iter().position(|e| *e == first);
                        if let Some(it_first) = it_first {
                            for j in 0..length.len() {
                                let last = self.myRoute[start[j] as usize];
                                let it_last = foe_route[it_first..].iter().position(|e| *e == last);
                                if let Some(it_last) = it_last {
                                    let it_last = it_first + it_last;
                                    // @todo are intermediateRS relevant here?
                                    let foe_sidings = foe.m().mySidings.entry(self_ptr).or_default();
                                    foe_sidings.insert(
                                        0,
                                        MSDriveWay_Siding::new(
                                            it_first as i32,
                                            it_last as i32,
                                            length[j],
                                            Vec::new(),
                                        ),
                                    );
                                    if foe_sidings.len() > 2 {
                                        foe_sidings.pop(); // only need the first 2 sidings
                                    }
                                }
                            }
                        }
                    } else {
                        // pick up further rail signals between the start of the siding and the start of the current driveway
                        let mut further_rs: Vec<i32> = Vec::new();
                        if cur_has_rs {
                            further_rs.push(i);
                        }
                        let mut next2 = cur;
                        let mut i2 = i - 1;
                        while i2 >= forward_normals {
                            let cur2 = self.myRoute[i2 as usize];
                            if Self::has_rs(cur2, next2) {
                                further_rs.push(i2);
                            }
                            next2 = cur2;
                            i2 -= 1;
                        }
                        let my_sidings = self.mySidings.entry(foe).or_default();
                        for j in 0..length.len() {
                            intermediate_rs[j].extend(further_rs.iter().copied());
                            my_sidings.insert(
                                0,
                                MSDriveWay_Siding::new(first_index, start[j], length[j], intermediate_rs[j].clone()),
                            );
                            if my_sidings.len() > 2 {
                                my_sidings.pop(); // only need the first 2 sidings
                            }
                        }
                    }
                    start.clear();
                    length.clear();
                    intermediate_rs.clear();
                    foe_search_beg = it_find;
                } else {
                    for j in 0..length.len() {
                        length[j] += cur.get_length();
                        if cur_has_rs && i != start[j] {
                            intermediate_rs[j].push(i);
                        }
                    }
                }
            }
            next = cur;
            i -= 1;
        }
        // register in mySidingDriveways
        let dw = if addToFoe { foe } else { self_ptr };
        let dw_foe = if addToFoe { self_ptr } else { foe };
        let sidings = dw.mySidings.get(&dw_foe).cloned();
        if let Some(sidings) = sidings {
            for siding in sidings.iter() {
                let end_edge = dw.myRoute[siding.end as usize];
                MY_SIDING_DRIVEWAYS.with(|m| m.borrow_mut().entry(end_edge).or_default().push((dw, dw_foe)));
            }
        }
    }

    /// C++: `void MSDriveWay::addBidiFoes(const MSRailSignal *, bool)` [protected]
    /// derive foe driveways based on myBidi or myBidiExtended
    // void
    // MSDriveWay::addBidiFoes(const MSRailSignal* ownSignal, bool extended) {
    // #ifdef DEBUG_ADD_FOES
    //     if (gDebugFlag4) {
    //         std::cout << "driveway " << myID << " addBidiFoes extended=" << extended << "\n";
    //     }
    // #endif
    //     const std::vector<const MSLane*>& bidiLanes = extended ? myBidiExtended : myBidi;
    //     for (const MSLane* bidi : bidiLanes) {
    //         for (auto ili : bidi->getIncomingLanes()) {
    //             const MSRailSignal* rs = dynamic_cast<const MSRailSignal*>(ili.viaLink->getTLLogic());
    //             if (rs != nullptr && rs != ownSignal &&
    //                     std::find(bidiLanes.begin(), bidiLanes.end(), ili.lane) != bidiLanes.end()) {
    //                 addFoes(ili.viaLink);
    //             }
    //         }
    //         const MSEdge* bidiEdge = &bidi->getEdge();
    //         if (myDepartureDriveways.count(bidiEdge) != 0) {
    //             for (MSDriveWay* foe : myDepartureDriveways[bidiEdge]) {
    //                 if (flankConflict(*foe)) {
    // #ifdef DEBUG_ADD_FOES
    //                     if (gDebugFlag4) {
    //                         std::cout << "  foe " << foe->getID() << " departs on bidi=" << bidiEdge->getID() << "\n";
    //                     }
    // #endif
    //                     myFoes.push_back(foe);
    //                 } else {
    // #ifdef DEBUG_ADD_FOES
    //                     if (gDebugFlag4) {
    //                         std::cout << "  cand foe " << foe->getID() << " departs on bidi=" << bidiEdge->getID() << " rejected\n";
    //                     }
    // #endif
    //                 }
    //             }
    //         }
    //         if (myDepartureDrivewaysEnds.count(bidiEdge) != 0) {
    //             for (MSDriveWay* foe : myDepartureDrivewaysEnds[bidiEdge]) {
    //                 if (flankConflict(*foe)) {
    // #ifdef DEBUG_ADD_FOES
    //                     if (gDebugFlag4) {
    //                         std::cout << "  foe " << foe->getID() << " ends on bidi=" << bidiEdge->getID() << "\n";
    //                     }
    // #endif
    //                     myFoes.push_back(foe);
    //                 } else {
    // #ifdef DEBUG_ADD_FOES
    //                     if (gDebugFlag4) {
    //                         std::cout << "  cand foe " << foe->getID() << " ends on bidi=" << bidiEdge->getID() << " rejected\n";
    //                     }
    // #endif
    //                 }
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_bidi_foes(&mut self, ownSignal: Ptr<MSRailSignal>, extended: bool) {
        let bidi_lanes: Vec<Ptr<MSLane>> = if extended {
            self.myBidiExtended.clone()
        } else {
            self.myBidi.clone()
        };
        for &bidi in bidi_lanes.iter() {
            for ili in bidi.get_incoming_lanes().iter() {
                let rs = as_rail_signal(ili.viaLink.get_tl_logic());
                if !rs.is_null()
                    && rs != ownSignal
                    && bidi_lanes.contains(&ili.lane)
                {
                    self.add_foes(ili.viaLink);
                }
            }
            let bidi_edge = Ptr::new(bidi.get_edge());
            let departures = MY_DEPARTURE_DRIVEWAYS.with(|m| m.borrow().get(&bidi_edge).cloned().unwrap_or_default());
            for foe in departures {
                if self.flank_conflict(&*foe) {
                    self.myFoes.push(foe);
                }
            }
            let departure_ends = MY_DEPARTURE_DRIVEWAYS_ENDS.with(|m| m.borrow().get(&bidi_edge).cloned().unwrap_or_default());
            for foe in departure_ends {
                if self.flank_conflict(&*foe) {
                    self.myFoes.push(foe);
                }
            }
        }
    }

    /// C++: `void MSDriveWay::addParallelFoes(const MSLink *, const MSEdge *)` [protected]
    /// derive foe driveways that start at the same signal
    // void
    // MSDriveWay::addParallelFoes(const MSLink* link, const MSEdge* first) {
    // #ifdef DEBUG_ADD_FOES
    //     if (gDebugFlag4) {
    //         std::cout << "driveway " << myID << " addParallelFoes\n";
    //     }
    // #endif
    //     if (link) {
    //         addFoes(link);
    //     } else {
    //         auto it = myDepartureDriveways.find(first);
    //         if (it != myDepartureDriveways.end()) {
    //             for (MSDriveWay* foe : it->second) {
    // #ifdef DEBUG_ADD_FOES
    //                 if (gDebugFlag4) {
    //                     std::cout << "  foe " << foe->getID() << " departs on first=" << first->getID() << "\n";
    //                 }
    // #endif
    //                 myFoes.push_back(foe);
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_parallel_foes(&mut self, link: Ptr<MSLink>, first: Ptr<MSEdge>) {
        if !link.is_null() {
            self.add_foes(link);
        } else {
            let foes = MY_DEPARTURE_DRIVEWAYS.with(|m| m.borrow().get(&first).cloned().unwrap_or_default());
            for foe in foes {
                self.myFoes.push(foe);
            }
        }
    }

    /// C++: `void MSDriveWay::addReversalFoes(bool)` [protected]
    /// derive foe driveways that enter the bidi section by reversing
    // void
    // MSDriveWay::addReversalFoes(bool movingBlock) {
    // #ifdef DEBUG_ADD_FOES
    //     if (gDebugFlag4) {
    //         std::cout << "driveway " << myID << " addReversalFoes\n";
    //     }
    // #endif
    //     std::set<const MSEdge*> forward;
    //     for (const MSLane* lane : myForward) {
    //         if (lane->isNormal()) {
    //             forward.insert(&lane->getEdge());
    //         }
    //     }
    //     int i = 0;
    //     for (const MSEdge* e : myRoute) {
    //         if (forward.count(e) != 0 && !movingBlock) {
    //             // reversals in our own forward can be ignored because each driveway
    //             // is automatically a foe of itself by default
    //             continue;
    //         }
    //         if (i == myCoreSize) {
    //             break;
    //         }
    //         i++;
    //         auto it = myReversalDriveWays.find(e);
    //         if (it != myReversalDriveWays.end()) {
    //             for (MSDriveWay* foe : it->second) {
    //                 // check whether the foe reverses into our own forward section
    //                 // (it might reverse again or disappear via arrival)
    // #ifdef DEBUG_ADD_FOES
    //                 //std::cout << "  candidate foe " << foe->getID() << " reverses on edge=" << e->getID() << " forward=" << joinNamedToString(forward, " ") << " foeRoute=" << toString(foe->myRoute) << "\n";
    // #endif
    //                 if (forwardRouteConflict(forward, *foe)) {
    //                     std::set<const MSEdge*> foeForward;
    //                     for (const MSLane* lane : foe->myForward) {
    //                         if (lane->isNormal()) {
    //                             foeForward.insert(&lane->getEdge());
    //                             if (lane->getBidiLane() != nullptr) {
    //                                 foeForward.insert(lane->getEdge().getBidiEdge());
    //                             }
    //                         }
    //                     }
    // #ifdef DEBUG_ADD_FOES
    //                     if (gDebugFlag4) {
    //                         std::cout << "  reversal cand=" << foe->getID() << " foeForward " << toString(foeForward) << "\n";
    //                     }
    // #endif
    //                     if (foe->forwardRouteConflict(foeForward, *this, true)) {
    // #ifdef DEBUG_ADD_FOES
    //                         if (gDebugFlag4) {
    //                             std::cout << "  foe " << foe->getID() << " reverses on edge=" << e->getID() << "\n";
    //                         }
    // #endif
    //                         myFoes.push_back(foe);
    //                     }
    //                 } else if (movingBlock && foe == this) {
    // #ifdef DEBUG_ADD_FOES
    //                     if (gDebugFlag4) {
    //                         std::cout << "  dw " << getID() << " reverses on forward edge=" << e->getID() << " (movingBlock)\n";
    //                     }
    // #endif
    //                     myFoes.push_back(foe);
    //                 }
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_reversal_foes(&mut self, movingBlock: bool) {
        let mut forward: BTreeSet<Ptr<MSEdge>> = BTreeSet::new();
        for &lane in self.myForward.iter() {
            if lane.is_normal() {
                forward.insert(Ptr::new(lane.get_edge()));
            }
        }
        let this_ptr = Ptr::new(&*self);
        let route = self.myRoute.clone();
        let mut i = 0;
        for &e in route.iter() {
            if forward.contains(&e) && !movingBlock {
                // reversals in our own forward can be ignored because each driveway
                // is automatically a foe of itself by default
                continue;
            }
            if i == self.myCoreSize {
                break;
            }
            i += 1;
            let foes = MY_REVERSAL_DRIVE_WAYS.with(|m| m.borrow().get(&e).cloned().unwrap_or_default());
            for foe in foes {
                // check whether the foe reverses into our own forward section
                // (it might reverse again or disappear via arrival)
                if self.forward_route_conflict(forward.clone(), &*foe, false) {
                    let mut foe_forward: BTreeSet<Ptr<MSEdge>> = BTreeSet::new();
                    for &lane in foe.myForward.iter() {
                        if lane.is_normal() {
                            foe_forward.insert(Ptr::new(lane.get_edge()));
                            if !lane.get_bidi_lane().is_null() {
                                foe_forward.insert(lane.get_edge().get_bidi_edge());
                            }
                        }
                    }
                    if foe.m().forward_route_conflict(foe_forward, &*self, true) {
                        self.myFoes.push(foe);
                    }
                } else if movingBlock && foe == this_ptr {
                    self.myFoes.push(foe);
                }
            }
        }
    }

    /// C++: `bool MSDriveWay::buildSubFoe(MSDriveWay *, bool)` [protected]
    // bool
    // MSDriveWay::buildSubFoe(MSDriveWay* foe, bool movingBlock) {
    //     // Subdriveways (Teilfahrstraße) model the resolution of a driving conflict
    //     // before a vehicle has left the driveway. This is possible when the driveway diverges from the foe
    //     // driveway at an earlier point (switch or crossing).
    //     //
    //     // We already know that the last edge of this driveway doesn't impact the foe (unless the driveway ends within the block).
    //     // Remove further edges from the end of the driveway (myForward) until the point of conflict is found.
    //     //
    //     // For movingBlock the logic is changed:
    //     // We remove the conflict-free part as before but then keep removing the conflict part until another non-conconflit part is found
    //     if (myForward.size() < foe->myForward.size() &&
    //             myForward == std::vector<const MSLane*>(foe->myForward.begin(), foe->myForward.begin() + myForward.size())) {
    // #ifdef DEBUG_BUILD_SUBDRIVEWAY
    //         if (gDebugFlag4) {
    //             std::cout << SIMTIME << " buildSubFoe dw=" << getID() << " is subpart of foe=" << foe->getID() << "\n";
    //         }
    // #endif
    //         foe->myFoes.push_back(this);
    //         return true;
    //     }
    //     int subLast = (int)myForward.size() - 2;
    //     if (movingBlock && myForward.back() == foe->myForward.back()) {
    //         subLast++;
    //     }
    // #ifdef DEBUG_BUILD_SUBDRIVEWAY
    //     if (subLast < 0) {
    //         if (gDebugFlag4) {
    //             std::cout << "  " << getID() << " cannot build subDriveWay for foe " << foe->getID() << " because myForward has only a single lane\n";
    //         }
    //     }
    // #endif
    //     bool foundConflict = false;
    //     bool flankC = false;
    //     bool zipperC = false;
    //     bool crossC = false;
    //     while (subLast >= 0) {
    //         const MSLane* lane = myForward[subLast];
    //         const MSLink* tmpOrigin = subLast > 0 ? myForward[subLast - 1]->getLinkTo(lane) : myOrigin;
    //         MSDriveWay tmp(tmpOrigin, "tmp", true);
    //         tmp.myForward.push_back(lane);
    //         tmp.myBidi = myBidi;
    //         tmp.myBidiExtended = myBidiExtended;
    //         tmp.myBidiExtendedSorted = myBidiExtendedSorted;
    //         tmp.myRoute.push_back(lane->getNextNormal());
    //         tmp.myCoreSize = 1;
    //         flankC = tmp.flankConflict(*foe);
    //         const bool bidiConflict = std::find(foe->myBidi.begin(), foe->myBidi.end(), lane) != foe->myBidi.end();
    //         crossC = tmp.crossingConflict(*foe);
    // #ifdef DEBUG_BUILD_SUBDRIVEWAY
    //         if (gDebugFlag4) {
    //             std::cout << "  subLast=" << subLast << " lane=" << lane->getID() << " fc=" << flankC << " cc=" << crossC << " bc=" << bidiConflict << "\n";
    //         }
    // #endif
    //         if (flankC || crossC || bidiConflict) {
    //             foundConflict = true;
    //             if (!movingBlock || bidiConflict) {
    //                 break;
    //             }
    //             if (((flankC && lane->getFromJunction()->getType() == SumoXMLNodeType::ZIPPER)
    //                     || (!flankC && lane->getToJunction()->getType() == SumoXMLNodeType::ZIPPER))
    //                     && (isDepartDriveway()
    //                         || getForwardDistance(flankC ? subLast - 1 : subLast) > myMovingBlockMaxDist)) {
    //                 zipperC = true;
    //                 foundConflict = false;
    // #ifdef DEBUG_BUILD_SUBDRIVEWAY
    //                 if (gDebugFlag4) {
    //                     std::cout << "     ignored movingBlock zipperConflict\n";
    //                 }
    // #endif
    //                 if (!flankC && crossC) {
    // #ifdef DEBUG_BUILD_SUBDRIVEWAY
    //                     if (gDebugFlag4) {
    //                         std::cout << SIMTIME << " buildSubFoe dw=" << getID() << " foe=" << foe->getID() << " movingBlock-save\n";
    //                     }
    // #endif
    //                     return false;
    //                 }
    //             }
    //             if (!flankC && crossC) {
    //                 break;
    //             }
    //         } else if (foundConflict) {
    //             break;
    //         }
    //         subLast--;
    //     }
    // #ifdef DEBUG_BUILD_SUBDRIVEWAY
    //     if (gDebugFlag4) {
    //         std::cout << "  subLastFinal=" << subLast << " movingBlock=" << movingBlock << " zipperC=" << zipperC << "\n";
    //     }
    // #endif
    //     if (bidiBlockedByEnd(*foe) && bidiBlockedBy(*this) && foe->forwardEndOnRoute(this)) {
    //         //std::set<const MSEdge*> firstEdge;
    //         //firstEdge.insert(foe->myRoute.front());
    //
    //         ConstMSEdgeVector forward(myRoute.begin(), myRoute.begin() + myForwardEdgeCount);
    //         ConstMSEdgeVector foeAfterForward(foe->myRoute.begin() + foe->myForwardEdgeCount,
    //                                           foe->myRoute.begin() + MIN2(foe->myRoute.size(), foe->myForwardEdgeCount + forward.size()));
    //         // @todo the check for forwardRouteConflict correctly reduces waiting in
    //         // test rail/reversal/consecutive_before_reversal but creates deadlock in
    //         // test rail/reversal/reversal_onRoute_beyond_core3b
    //         if (forward == foeAfterForward /*&& forwardRouteConflict(firstEdge, *this, true)*/) {
    //             foe->myFoes.push_back(this);
    //             // foe will get the sidings
    //             addSidings(foe, true);
    // #ifdef DEBUG_BUILD_SUBDRIVEWAY
    //             if (gDebugFlag4) {
    //                 std::cout << SIMTIME << " buildSubFoe dw=" << getID() << " foe=" << foe->getID() << " special case 1\n";
    //             }
    // #endif
    //             return true;
    //         }
    //     }
    //     if (subLast < 0) {
    //         if (movingBlock && zipperC) {
    // #ifdef DEBUG_BUILD_SUBDRIVEWAY
    //             if (gDebugFlag4) {
    //                 std::cout << SIMTIME << " buildSubFoe dw=" << getID() << " foe=" << foe->getID() << " movingBlock-save\n";
    //             }
    // #endif
    //             return false;
    //         } else if (&myForward.back()->getEdge() == myRoute.back() && foe->forwardEndOnRoute(this)) {
    //             // driveway ends in the middle of the block and only the final edge overlaps with the foe driveWay
    //             foe->myFoes.push_back(this);
    // #ifdef DEBUG_BUILD_SUBDRIVEWAY
    //             if (gDebugFlag4) {
    //                 std::cout << SIMTIME << " buildSubFoe dw=" << getID() << " foe=" << foe->getID() << " foe endsOnForward\n";
    //             }
    // #endif
    //         } else if (foe->myTerminateRoute) {
    //             if (bidiBlockedByEnd(*foe) && bidiBlockedBy(*this) && foe->forwardEndOnRoute(this)) {
    //                 foe->myFoes.push_back(this);
    //                 // foe will get the sidings
    //                 addSidings(foe, true);
    //             }
    // #ifdef DEBUG_BUILD_SUBDRIVEWAY
    //             if (gDebugFlag4) {
    //                 std::cout << SIMTIME << " buildSubFoe dw=" << getID() << " foe=" << foe->getID() << " terminates\n";
    //             }
    // #endif
    //         } else if (myTerminateRoute && myBidi.size() <= myForward.size()) {
    //             foe->myFoes.push_back(this);
    // #ifdef DEBUG_BUILD_SUBDRIVEWAY
    //             if (gDebugFlag4) {
    //                 std::cout << SIMTIME << " buildSubFoe dw=" << getID() << " terminates, foe=" << foe->getID() << "\n";
    //             }
    // #endif
    //             return true;
    //         } else if (foe->myReversals.size() % 2 == 1) {
    // #ifdef DEBUG_BUILD_SUBDRIVEWAY
    //             if (gDebugFlag4) {
    //                 std::cout << SIMTIME << " buildSubFoe dw=" << getID() << " foe=" << foe->getID() << " has " << foe->myReversals.size() << " reversals\n";
    //             }
    // #endif
    //         } else {
    // #ifdef DEBUG_BUILD_SUBDRIVEWAY
    //             if (gDebugFlag4) {
    //                 std::cout << SIMTIME << " buildSubFoe dw=" << getID() << " foe=" << foe->getID() << " failed\n";
    //             }
    // #endif
    // #ifdef SUBDRIVEWAY_WARN_NOCONFLICT
    //             WRITE_WARNINGF("No point of conflict found between driveway '%' and driveway '%' when creating sub-driveway", getID(), foe->getID());
    // #endif
    //         }
    //         return false;
    //     }
    //     int subSize = subLast + 1;
    //     for (MSDriveWay* cand : mySubDriveWays) {
    //         if ((int)cand->myForward.size() == subSize) {
    //             // can re-use existing sub-driveway
    //             foe->myFoes.push_back(cand);
    //             if (foe->bidiBlockedByEnd(*cand)) {
    //                 foe->addSidings(cand);
    //             }
    //             cand->myFoes.push_back(foe);
    // #ifdef DEBUG_BUILD_SUBDRIVEWAY
    //             if (gDebugFlag4) {
    //                 std::cout << SIMTIME << " buildSubFoe dw=" << getID() << " foe=" << foe->getID() << " useExisting=" << cand->getID() << "\n";
    //             }
    // #endif
    //             return true;
    //         }
    //     }
    //     std::vector<const MSLane*> forward(myForward.begin(), myForward.begin() + subSize);
    //     std::vector<const MSEdge*> route;
    //     for (const MSLane* lane : forward) {
    //         if (lane->isNormal()) {
    //             route.push_back(&lane->getEdge());
    //         }
    //     }
    //     if (route.empty()) {
    //         if (subSize == 1 && crossC
    //                 && forward.front()->getFromJunction() == foe->myForward.front()->getFromJunction()
    //                 && forward.front()->getFromJunction()->getType() == SumoXMLNodeType::RAIL_SIGNAL) {
    //             assert(myForward.front()->isInternal());
    //             // sub-driveway ends after a single internal lane but since the route cannot be empty we add the next edge
    //             route.push_back(foe->myForward.front()->getEdge().getNormalSuccessor());
    //         }  else {
    // #ifdef DEBUG_BUILD_SUBDRIVEWAY
    //             if (gDebugFlag4) {
    //                 std::cout << SIMTIME << " abort subFoe dw=" << getID() << " foe=" << foe->getID() << " empty subRoute\n";
    //             }
    // #endif
    //             return false;
    //         }
    //     }
    //     if (myRoute.size() > route.size()) {
    //         // route continues. make sure the subDriveway does not end with a reversal
    //         const MSEdge* lastNormal = route.back();
    //         const MSEdge* nextNormal = myRoute[route.size()];
    //         if (lastNormal->getBidiEdge() == nextNormal) {
    // #ifdef DEBUG_BUILD_SUBDRIVEWAY
    //             if (gDebugFlag4) std::cout << SIMTIME << " abort subFoe dw=" << getID() << " foe=" << foe->getID()
    //                                            << " lastNormal=" << lastNormal->getID() << " nextNormal=" << nextNormal->getID() << " endWithReversal\n";
    // #endif
    //             return false;
    //         }
    //     }
    //     MSDriveWay* sub = new MSDriveWay(myOrigin, getID() + "." + toString(mySubDriveWays.size()));
    //     sub->myLane = myLane;
    //     sub->myParent = this;
    //     sub->myForward = forward;
    //     sub->myRoute = route;
    //     sub->myCoreSize = (int)sub->myRoute.size();
    //     myLane->addMoveReminder(sub, false);
    //
    //     // copy trains that are currently on this driveway (and associated entry events)
    //     for (SUMOVehicle* veh : myTrains) {
    //         auto itOnSub = std::find(sub->myRoute.begin(), sub->myRoute.end(), veh->getEdge());
    //         if (itOnSub != sub->myRoute.end()) {
    //             sub->myTrains.insert(veh);
    //             // non-zero is enough to avoid superfluous activation via activateReminders (and removal)
    //             const double pos = sub->myRoute.front()->getLength();
    //             dynamic_cast<MSBaseVehicle*>(veh)->addReminder(sub, pos);
    //             for (const VehicleEvent& ve : myVehicleEvents) {
    //                 if (ve.id == veh->getID()) {
    //                     sub->myVehicleEvents.push_back(ve);
    //                 }
    //             }
    //         }
    //     }
    //
    //     foe->myFoes.push_back(sub);
    //     if (foe->bidiBlockedByEnd(*sub)) {
    //         foe->addSidings(sub);
    //     }
    //     sub->myFoes.push_back(foe);
    //     mySubDriveWays.push_back(sub);
    // #ifdef DEBUG_BUILD_SUBDRIVEWAY
    //     if (gDebugFlag4) {
    //         std::cout << SIMTIME << " buildSubFoe dw=" << getID() << " foe=" << foe->getID() << " sub=" << sub->getID() << " route=" << toString(sub->myRoute) << "\n";
    //     }
    // #endif
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn build_sub_foe(&mut self, foe: Ptr<MSDriveWay>, movingBlock: bool) -> bool {
        // Subdriveways (Teilfahrstraße) model the resolution of a driving conflict
        // before a vehicle has left the driveway. This is possible when the driveway diverges from the foe
        // driveway at an earlier point (switch or crossing).
        //
        // We already know that the last edge of this driveway doesn't impact the foe (unless the driveway ends within the block).
        // Remove further edges from the end of the driveway (myForward) until the point of conflict is found.
        //
        // For movingBlock the logic is changed:
        // We remove the conflict-free part as before but then keep removing the conflict part until another non-conconflit part is found
        let self_ptr = Ptr::from_mut(self);
        if self.myForward.len() < foe.myForward.len()
            && self.myForward[..] == foe.myForward[..self.myForward.len()]
        {
            foe.m().myFoes.push(self_ptr);
            return true;
        }
        let mut sub_last = self.myForward.len() as i32 - 2;
        if movingBlock && *self.myForward.last().unwrap() == *foe.myForward.last().unwrap() {
            sub_last += 1;
        }
        let mut found_conflict = false;
        let mut flank_c = false;
        let mut zipper_c = false;
        let mut cross_c = false;
        while sub_last >= 0 {
            let lane = self.myForward[sub_last as usize];
            let tmp_origin = if sub_last > 0 {
                self.myForward[sub_last as usize - 1].get_link_to(lane)
            } else {
                self.myOrigin
            };
            let mut tmp = MSDriveWay::new(tmp_origin, "tmp", true);
            tmp.myForward.push(lane);
            tmp.myBidi = self.myBidi.clone();
            tmp.myBidiExtended = self.myBidiExtended.clone();
            tmp.myBidiExtendedSorted = self.myBidiExtendedSorted.clone();
            tmp.myRoute.push(lane.get_next_normal());
            tmp.myCoreSize = 1;
            flank_c = tmp.flank_conflict(&*foe);
            let bidi_conflict = foe.myBidi.contains(&lane);
            cross_c = tmp.crossing_conflict(&*foe);
            if flank_c || cross_c || bidi_conflict {
                found_conflict = true;
                if !movingBlock || bidi_conflict {
                    break;
                }
                if ((flank_c && lane.get_from_junction().get_type() == SumoXMLNodeType::ZIPPER)
                    || (!flank_c && lane.get_to_junction().get_type() == SumoXMLNodeType::ZIPPER))
                    && (self.is_depart_driveway()
                        || self.get_forward_distance(if flank_c { sub_last - 1 } else { sub_last })
                            > MY_MOVING_BLOCK_MAX_DIST.with(|d| d.get()))
                {
                    zipper_c = true;
                    found_conflict = false;
                    if !flank_c && cross_c {
                        return false;
                    }
                }
                if !flank_c && cross_c {
                    break;
                }
            } else if found_conflict {
                break;
            }
            sub_last -= 1;
        }
        if self.bidi_blocked_by_end(&*foe) && self.bidi_blocked_by(&*self) && foe.forward_end_on_route(self_ptr) {
            let forward_end = std::cmp::min(self.myForwardEdgeCount as usize, self.myRoute.len());
            let forward: ConstMSEdgeVector = self.myRoute[..forward_end].to_vec();
            let foe_beg = std::cmp::min(foe.myForwardEdgeCount as usize, foe.myRoute.len());
            let foe_end = std::cmp::min(foe.myRoute.len(), foe.myForwardEdgeCount as usize + forward.len());
            let foe_after_forward: ConstMSEdgeVector = foe.myRoute[foe_beg..foe_end].to_vec();
            // @todo the check for forwardRouteConflict correctly reduces waiting in
            // test rail/reversal/consecutive_before_reversal but creates deadlock in
            // test rail/reversal/reversal_onRoute_beyond_core3b
            if forward == foe_after_forward {
                foe.m().myFoes.push(self_ptr);
                // foe will get the sidings
                self.add_sidings(foe, true);
                return true;
            }
        }
        if sub_last < 0 {
            if movingBlock && zipper_c {
                return false;
            } else if Ptr::new(self.myForward.last().unwrap().get_edge()) == *self.myRoute.last().unwrap()
                && foe.forward_end_on_route(self_ptr)
            {
                // driveway ends in the middle of the block and only the final edge overlaps with the foe driveWay
                foe.m().myFoes.push(self_ptr);
            } else if foe.myTerminateRoute {
                if self.bidi_blocked_by_end(&*foe) && self.bidi_blocked_by(&*self) && foe.forward_end_on_route(self_ptr) {
                    foe.m().myFoes.push(self_ptr);
                    // foe will get the sidings
                    self.add_sidings(foe, true);
                }
            } else if self.myTerminateRoute && self.myBidi.len() <= self.myForward.len() {
                foe.m().myFoes.push(self_ptr);
                return true;
            } else if foe.myReversals.len() % 2 == 1 {
                // foe has an odd number of reversals
            } else {
                // no point of conflict found
                // #ifdef SUBDRIVEWAY_WARN_NOCONFLICT
                // WRITE_WARNINGF("No point of conflict found between driveway '%' and driveway '%' when creating sub-driveway", getID(), foe->getID());
                // #endif
            }
            return false;
        }
        let sub_size = sub_last + 1;
        for &cand in self.mySubDriveWays.iter() {
            if cand.myForward.len() as i32 == sub_size {
                // can re-use existing sub-driveway
                foe.m().myFoes.push(cand);
                if foe.bidi_blocked_by_end(&*cand) {
                    foe.m().add_sidings(cand, false);
                }
                cand.m().myFoes.push(foe);
                return true;
            }
        }
        let forward: Vec<Ptr<MSLane>> = self.myForward[..sub_size as usize].to_vec();
        let mut route: Vec<Ptr<MSEdge>> = Vec::new();
        for &lane in forward.iter() {
            if lane.is_normal() {
                route.push(Ptr::new(lane.get_edge()));
            }
        }
        if route.is_empty() {
            if sub_size == 1
                && cross_c
                && forward.first().unwrap().get_from_junction() == foe.myForward.first().unwrap().get_from_junction()
                && forward.first().unwrap().get_from_junction().get_type() == SumoXMLNodeType::RAIL_SIGNAL
            {
                assert!(self.myForward.first().unwrap().is_internal());
                // sub-driveway ends after a single internal lane but since the route cannot be empty we add the next edge
                route.push(foe.myForward.first().unwrap().get_edge().get_normal_successor());
            } else {
                return false;
            }
        }
        if self.myRoute.len() > route.len() {
            // route continues. make sure the subDriveway does not end with a reversal
            let last_normal = *route.last().unwrap();
            let next_normal = self.myRoute[route.len()];
            if last_normal.get_bidi_edge() == next_normal {
                return false;
            }
        }
        // shortcut: the sub-driveway is allocated with Box::leak (see build_drive_way)
        let sub = Ptr::from_mut(Box::leak(Box::new(MSDriveWay::new(
            self.myOrigin,
            &format!("{}.{}", self.base2.get_id(), self.mySubDriveWays.len()),
            false,
        ))));
        sub.m().base.myLane = self.myLane;
        sub.m().myParent = self_ptr;
        sub.m().myForward = forward;
        sub.m().myRoute = route.clone();
        sub.m().myCoreSize = route.len() as i32;
        self.myLane.m().add_move_reminder(sub.cast::<MSMoveReminder>(), false);

        // copy trains that are currently on this driveway (and associated entry events)
        for &veh in self.myTrains.iter() {
            let mveh = veh.cast::<MSVehicle>();
            if route.contains(&mveh.get_edge()) {
                sub.m().myTrains.insert(veh);
                // non-zero is enough to avoid superfluous activation via activateReminders (and removal)
                let pos = sub.myRoute.first().unwrap().get_length();
                veh.cast::<MSBaseVehicle>()
                    .m()
                    .add_reminder(sub.cast::<MSMoveReminder>(), pos);
                for ve in self.myVehicleEvents.iter() {
                    if ve.id == mveh.get_id() {
                        sub.m().myVehicleEvents.push((*ve).clone());
                    }
                }
            }
        }

        foe.m().myFoes.push(sub);
        if foe.bidi_blocked_by_end(&*sub) {
            foe.m().add_sidings(sub, false);
        }
        sub.m().myFoes.push(foe);
        self.mySubDriveWays.push(sub);
        true
    }

    /// C++: `void MSDriveWay::addConflictLink(const MSLink *)` [protected]
    /// add symmetical conflict link for foes when building a new driveway
    // void
    // MSDriveWay::addConflictLink(const MSLink* link) {
    //     if (link->getTLLogic() != nullptr) {
    //         // ignore links that originate on myBidi
    //         // and also links from the same junction as my own link
    //         const MSLane* origin = link->getLaneBefore();
    //         if (std::find(myBidi.begin(), myBidi.end(), origin) == myBidi.end()) {
    //             if (link->getJunction() != myRoute.front()->getFromJunction()) {
    //                 if (std::find(myConflictLinks.begin(), myConflictLinks.end(), link) == myConflictLinks.end()) {
    //                     myConflictLinks.push_back(const_cast<MSLink*>(link));
    //                 }
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_conflict_link(&mut self, link: Ptr<MSLink>) {
        if !link.get_tl_logic().is_null() {
            // ignore links that originate on myBidi
            // and also links from the same junction as my own link
            let origin = link.get_lane_before();
            if !self.myBidi.contains(&origin) {
                if link.get_junction() != self.myRoute.first().unwrap().get_from_junction()
                    && !self.myConflictLinks.contains(&link)
                {
                    self.myConflictLinks.push(link);
                }
            }
        }
    }

    /// C++: `std::pair<bool, const MSDriveWay *> MSDriveWay::canUseSiding(const SUMOVehicle *, const MSDriveWay *, const MSEdge *) const` [const, protected]
    /// return whether a siding can be used. If a siding exist but is occupied, also return the occupied driveway in the siding
    // std::pair<bool, const MSDriveWay*>
    // MSDriveWay::canUseSiding(const SUMOVehicle* ego, const MSDriveWay* foe, const MSEdge* recurseSidingEnd) const {
    //     auto it = mySidings.find(foe);
    //     if (it != mySidings.end()) {
    //         for (auto siding : it->second) {
    //             // assume siding is usuable when computing state for unapproached signal (ego == nullptr)
    //             if (ego == nullptr || siding.length >= ego->getLength()) {
    //                 // if the siding is already "reserved" by another vehicle we cannot use it here
    //                 const MSEdge* sidingEnd = myRoute[siding.end];
    //                 bool checkNext = false;
    //                 for (MSDriveWay* sidingApproach : myEndingDriveways[sidingEnd]) {
    //                     if (!sidingApproach->myTrains.empty() && *sidingApproach->myTrains.begin() == ego && sidingEnd == recurseSidingEnd) {
    //                         // check next siding if if exists
    //                         checkNext = true;
    // #ifdef DEBUG_SIGNALSTATE
    //                         if (gDebugFlag4 || DEBUG_COND_DW2 || DEBUG_HELPER(ego)) {
    //                             std::cout << "   checkNext\n";
    //                         }
    // #endif
    //                         continue;
    //                     }
    //                     if ((!sidingApproach->myTrains.empty() && *sidingApproach->myTrains.begin() != ego)
    //                             || std::find(myCurrentSidingCheck.begin(), myCurrentSidingCheck.end(), sidingEnd) != myCurrentSidingCheck.end()) {
    //                         // possibly the foe vehicle can use the other part of the siding
    //                         if (recurseSidingEnd == nullptr) {
    //                             const SUMOVehicle* foeVeh = nullptr;
    //                             if (!foe->myTrains.empty()) {
    //                                 foeVeh = *foe->myTrains.begin();
    //                             } else if (foe->myOrigin != nullptr && foe->myOrigin->getApproaching().size() > 0) {
    //                                 foeVeh = foe->myOrigin->getClosest().first;
    //                             }
    //                             if (foeVeh == nullptr) {
    //                                 WRITE_WARNINGF("Invalid call to canUseSiding dw=% foe=% ego=% time=%", getID(), foe->getID(), Named::getIDSecure(ego), time2string(SIMSTEP));
    //                                 continue;
    //                             }
    //                             const MSDriveWay* foe2 = foe->isSubDriveWay() ? foe->myParent : foe;
    //                             const MSDriveWay* this2 = foe2->getFoeOrSubFoe(this);
    // #ifdef DEBUG_SIGNALSTATE
    //                             if (gDebugFlag4 || DEBUG_COND_DW2 || DEBUG_HELPER(ego)) {
    //                                 std::cout << "   foe2=" << foe2->getID() << " this2=" << this2->getID() << "\n";
    //                             }
    // #endif
    //                             if (this2 != nullptr && foe2->canUseSiding(foeVeh, this2, sidingEnd).first) {
    //                                 continue;
    //                             }
    //                         }
    //                         // @todo: in principle it might still be possible to continue if vehicle that approaches the siding can safely leave the situation (#18416)
    // #ifdef DEBUG_SIGNALSTATE
    //                         if (gDebugFlag4 || DEBUG_COND_DW2 || DEBUG_HELPER(ego)) {
    //                             std::cout << SIMTIME << " " << getID() << " ego=" << Named::getIDSecure(ego) << " foe=" << foe->getID()
    //                                       << " foeVeh=" << toString(foe->myTrains)
    //                                       << " sidingEnd=" << sidingEnd->getID() << " sidingApproach=" << sidingApproach->getID() << " approaching=" << toString(sidingApproach->myTrains) << "\n";
    //                         }
    // #endif
    //                         return std::make_pair(false, sidingApproach);
    //                     }
    //                 }
    //                 // vehicles approaching intermediate driveways could also make the siding unusable but would not show up as sidingApproaches
    //                 for (int i : siding.intermediateEnds) {
    //                     const MSEdge* intermediateEnd = myRoute[i];
    //                     for (MSDriveWay* intermediateApproach : myEndingDriveways[intermediateEnd]) {
    //                         if (!intermediateApproach->myTrains.empty()
    //                                 && (*intermediateApproach->myTrains.begin() != ego || intermediateEnd == recurseSidingEnd)) {
    //                             SUMOVehicle* onApproach = *intermediateApproach->myTrains.begin();
    //                             if (std::find(onApproach->getCurrentRouteEdge(), onApproach->getRoute().end(), sidingEnd) == onApproach->getRoute().end()) {
    //                                 // intermediate vehicle does not make use of the siding
    //                                 continue;
    //                             }
    //                             // possibly the foe vehicle can use the other part of the siding
    //                             if (recurseSidingEnd == nullptr) {
    //                                 const SUMOVehicle* foeVeh = nullptr;
    //                                 if (!foe->myTrains.empty()) {
    //                                     foeVeh = *foe->myTrains.begin();
    //                                 } else if (foe->myOrigin != nullptr && foe->myOrigin->getApproaching().size() > 0) {
    //                                     foeVeh = foe->myOrigin->getClosest().first;
    //                                 }
    //                                 if (foeVeh == nullptr) {
    //                                     WRITE_WARNINGF("Invalid call to canUseSiding dw=% foe=% ego=% time=%", getID(), foe->getID(), Named::getIDSecure(ego), time2string(SIMSTEP));
    //                                     continue;
    //                                 }
    //                                 const MSDriveWay* foe2 = foe->isSubDriveWay() ? foe->myParent : foe;
    //                                 const MSDriveWay* this2 = foe2->getFoeOrSubFoe(this);
    //                                 if (this2 != nullptr && foe2->canUseSiding(foeVeh, this2, intermediateEnd).first) {
    //                                     continue;
    //                                 }
    //                             }
    //                             // @todo: in principle it might still be possible to continue if vehicle that approaches the siding can safely leave the situation
    // #ifdef DEBUG_SIGNALSTATE
    //                             if (gDebugFlag4 || DEBUG_COND_DW2 || DEBUG_HELPER(ego)) {
    //                                 std::cout << SIMTIME << " " << getID() << " ego=" << Named::getIDSecure(ego) << " foe=" << foe->getID()
    //                                           << " foeVeh=" << toString(foe->myTrains)
    //                                           << " sidingEnd=" << sidingEnd->getID() << " intermediateApproach=" << intermediateApproach->getID() << " approaching=" << toString(intermediateApproach->myTrains) << "\n";
    //                             }
    // #endif
    //                             return std::make_pair(false, intermediateApproach);
    //                         }
    //                     }
    //                 }
    //                 if (checkNext) {
    //                     continue;
    //                 }
    // #ifdef DEBUG_SIGNALSTATE
    //                 if (gDebugFlag4 || DEBUG_COND_DW2 || DEBUG_HELPER(ego)) {
    //                     std::cout << SIMTIME << " " << getID() << " ego=" << Named::getIDSecure(ego) << " foe=" << foe->getID()
    //                               << " foeVeh=" << toString(foe->myTrains)
    //                               << " sidingEnd=" << sidingEnd->getID() << " usable\n";
    //                 }
    // #endif
    //                 return std::make_pair(true, nullptr);
    //             }
    //         }
    //     }
    // #ifdef DEBUG_SIGNALSTATE
    //     if (gDebugFlag4 || DEBUG_COND_DW2 || DEBUG_HELPER(ego)) {
    //         std::cout << SIMTIME << " " << getID() << " ego=" << Named::getIDSecure(ego) << " foe=" << foe->getID() << " noSidings\n";
    //     }
    // #endif
    //     return std::make_pair(false, nullptr);
    // }
    #[allow(unused_variables)]
    pub fn can_use_siding(&self, ego: Ptr<SUMOVehicle>, foe: Ptr<MSDriveWay>, recurseSidingEnd: Ptr<MSEdge>) -> (bool, Ptr<MSDriveWay>) {
        let self_ptr = Ptr::new(self);
        let sidings = self.mySidings.get(&foe).cloned();
        if let Some(sidings) = sidings {
            for siding in sidings.iter() {
                // assume siding is usuable when computing state for unapproached signal (ego == nullptr)
                let long_enough = ego.is_null() || siding.length >= ego.cast::<MSVehicle>().get_length();
                if long_enough {
                    // if the siding is already "reserved" by another vehicle we cannot use it here
                    let siding_end = self.myRoute[siding.end as usize];
                    let mut check_next = false;
                    let siding_approaches = MY_ENDING_DRIVEWAYS.with(|m| m.borrow().get(&siding_end).cloned().unwrap_or_default());
                    for siding_approach in siding_approaches.iter() {
                        if !siding_approach.myTrains.is_empty()
                            && *siding_approach.myTrains.iter().next().unwrap() == ego
                            && siding_end == recurseSidingEnd
                        {
                            // check next siding if if exists
                            check_next = true;
                            continue;
                        }
                        let occupied_by_other = !siding_approach.myTrains.is_empty()
                            && *siding_approach.myTrains.iter().next().unwrap() != ego;
                        let currently_checked = MY_CURRENT_SIDING_CHECK.with(|c| c.borrow().contains(&siding_end));
                        if occupied_by_other || currently_checked {
                            // possibly the foe vehicle can use the other part of the siding
                            if recurseSidingEnd.is_null() {
                                let mut foe_veh = Ptr::null();
                                if !foe.myTrains.is_empty() {
                                    foe_veh = *foe.myTrains.iter().next().unwrap();
                                } else if !foe.myOrigin.is_null() && !foe.myOrigin.get_approaching_2().is_empty() {
                                    foe_veh = foe.myOrigin.get_closest().0;
                                }
                                if foe_veh.is_null() {
                                    utils::write_warningf!(
                                        "Invalid call to canUseSiding dw=% foe=% ego=% time=%",
                                        self.base2.get_id().to_string(),
                                        foe.base2.get_id().to_string(),
                                        if ego.is_null() { "NULL".to_string() } else { ego.get_id().to_string() },
                                        time2string(MSNet::get_instance().get_current_time_step())
                                    );
                                    continue;
                                }
                                let foe2 = if foe.is_sub_drive_way() { foe.myParent } else { foe };
                                let this2 = foe2.get_foe_or_sub_foe(self_ptr);
                                if !this2.is_null() && foe2.can_use_siding(foe_veh, this2, siding_end).0 {
                                    continue;
                                }
                            }
                            // @todo: in principle it might still be possible to continue if vehicle that approaches the siding can safely leave the situation (#18416)
                            return (false, *siding_approach);
                        }
                    }
                    // vehicles approaching intermediate driveways could also make the siding unusable but would not show up as sidingApproaches
                    for &i in siding.intermediateEnds.iter() {
                        let intermediate_end = self.myRoute[i as usize];
                        let intermediate_approaches =
                            MY_ENDING_DRIVEWAYS.with(|m| m.borrow().get(&intermediate_end).cloned().unwrap_or_default());
                        for intermediate_approach in intermediate_approaches.iter() {
                            if !intermediate_approach.myTrains.is_empty()
                                && (*intermediate_approach.myTrains.iter().next().unwrap() != ego
                                    || intermediate_end == recurseSidingEnd)
                            {
                                let on_approach = *intermediate_approach.myTrains.iter().next().unwrap();
                                let m_on_approach = on_approach.cast::<MSVehicle>();
                                let route_edges = m_on_approach.get_route().get_edges();
                                let cur = std::cmp::min(*m_on_approach.get_current_route_edge(), route_edges.len());
                                let uses_siding = route_edges[cur..].contains(&siding_end);
                                if !uses_siding {
                                    // intermediate vehicle does not make use of the siding
                                    continue;
                                }
                                // possibly the foe vehicle can use the other part of the siding
                                if recurseSidingEnd.is_null() {
                                    let mut foe_veh = Ptr::null();
                                    if !foe.myTrains.is_empty() {
                                        foe_veh = *foe.myTrains.iter().next().unwrap();
                                    } else if !foe.myOrigin.is_null() && !foe.myOrigin.get_approaching_2().is_empty() {
                                        foe_veh = foe.myOrigin.get_closest().0;
                                    }
                                    if foe_veh.is_null() {
                                        utils::write_warningf!(
                                            "Invalid call to canUseSiding dw=% foe=% ego=% time=%",
                                            self.base2.get_id().to_string(),
                                            foe.base2.get_id().to_string(),
                                            if ego.is_null() { "NULL".to_string() } else { ego.get_id().to_string() },
                                            time2string(MSNet::get_instance().get_current_time_step())
                                        );
                                        continue;
                                    }
                                    let foe2 = if foe.is_sub_drive_way() { foe.myParent } else { foe };
                                    let this2 = foe2.get_foe_or_sub_foe(self_ptr);
                                    if !this2.is_null() && foe2.can_use_siding(foe_veh, this2, intermediate_end).0 {
                                        continue;
                                    }
                                }
                                // @todo: in principle it might still be possible to continue if vehicle that approaches the siding can safely leave the situation
                                return (false, *intermediate_approach);
                            }
                        }
                    }
                    if check_next {
                        continue;
                    }
                    return (true, Ptr::null());
                }
            }
        }
        (false, Ptr::null())
    }

    /// C++: `const MSDriveWay * MSDriveWay::getFoeOrSubFoe(const MSDriveWay *) const` [const, protected]
    /// return dw if it is a foe or a subDrivway of dw if it is a foe
    // const MSDriveWay*
    // MSDriveWay::getFoeOrSubFoe(const MSDriveWay* dw) const {
    //     if (std::find(myFoes.begin(), myFoes.end(), dw) != myFoes.end()) {
    //         return dw;
    //     }
    //     for (const MSDriveWay* sub : dw->mySubDriveWays) {
    //         const MSDriveWay* foe = getFoeOrSubFoe(sub);
    //         if (foe != nullptr) {
    //             return foe;
    //         }
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_foe_or_sub_foe(&self, dw: Ptr<MSDriveWay>) -> Ptr<MSDriveWay> {
        if self.myFoes.contains(&dw) {
            return dw;
        }
        for &sub in dw.mySubDriveWays.iter() {
            let foe = self.get_foe_or_sub_foe(sub);
            if !foe.is_null() {
                return foe;
            }
        }
        Ptr::null()
    }

    /// C++: `bool MSDriveWay::forwardEndOnRoute(const MSDriveWay *) const` [const, protected]
    // bool
    // MSDriveWay::forwardEndOnRoute(const MSDriveWay* foe) const {
    //     const MSEdge* foeForwardEnd = &foe->myForward.back()->getNormalPredecessorLane()->getEdge();
    //     return std::find(myRoute.begin(), myRoute.end(), foeForwardEnd) != myRoute.end();
    // }
    #[allow(unused_variables)]
    pub fn forward_end_on_route(&self, foe: Ptr<MSDriveWay>) -> bool {
        let foe_forward_end = Ptr::new(foe.myForward.last().unwrap().get_normal_predecessor_lane().get_edge());
        self.myRoute.contains(&foe_forward_end)
    }

    /// C++: `void MSDriveWay::addSwitchFoes(MSLink *)` [protected]
    // void
    // MSDriveWay::addSwitchFoes(MSLink* link) {
    //     auto it = mySwitchDriveWays.find(link);
    //     if (it != mySwitchDriveWays.end()) {
    // #ifdef DEBUG_ADD_FOES
    //         if (gDebugFlag4) {
    //             std::cout << "   driveway " << myID << " addSwitchFoes for link " << link->getDescription() << "\n";
    //         }
    // #endif
    //         for (MSDriveWay* foe : it->second) {
    //             if (foe != this && (flankConflict(*foe) || foe->flankConflict(*this) || crossingConflict(*foe) || foe->crossingConflict(*this))) {
    // #ifdef DEBUG_ADD_FOES
    //                 if (gDebugFlag4) std::cout << "   foe=" << foe->myID
    //                                                << " fc1=" << flankConflict(*foe) << " fc2=" << foe->flankConflict(*this)
    //                                                << " cc1=" << crossingConflict(*foe) << " cc2=" << foe->crossingConflict(*this) << "\n";
    // #endif
    //                 myFoes.push_back(foe);
    //             } else {
    // #ifdef DEBUG_ADD_FOES
    //                 if (gDebugFlag4) {
    //                     std::cout << "   cand=" << foe->myID << "\n";
    //                 }
    // #endif
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_switch_foes(&mut self, link: Ptr<MSLink>) {
        let foes = MY_SWITCH_DRIVE_WAYS.with(|m| m.borrow().get(&link).cloned().unwrap_or_default());
        let this_ptr = Ptr::new(&*self);
        for foe in foes {
            if foe != this_ptr
                && (self.flank_conflict(&*foe)
                    || foe.flank_conflict(&*self)
                    || self.crossing_conflict(&*foe)
                    || foe.crossing_conflict(&*self))
            {
                self.myFoes.push(foe);
            }
        }
    }

    /// C++: `bool MSDriveWay::haveSubTrains() const` [const, protected]
    // bool
    // MSDriveWay::haveSubTrains() const {
    //     for (MSDriveWay* sub : mySubDriveWays) {
    //         if (!sub->myTrains.empty()) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn have_sub_trains(&self) -> bool {
        for &sub in self.mySubDriveWays.iter() {
            if !sub.myTrains.is_empty() {
                return true;
            }
        }
        false
    }

    /// C++: `double MSDriveWay::getForwardDistance(int) const` [const, protected]
    /// compute distance along the forward section up to lastIndex
    // double
    // MSDriveWay::getForwardDistance(int lastIndex) const {
    //     assert(lastIndex < (int)myForward.size());
    //     double result = 0;
    //     for (int i = 0; i <= lastIndex; i++) {
    //         result += myForward[i]->getLength();
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_forward_distance(&self, lastIndex: i32) -> f64 {
        assert!(lastIndex < self.myForward.len() as i32);
        let mut result = 0.0;
        for i in 0..=lastIndex {
            result += self.myForward[i as usize].get_length();
        }
        result
    }

    /// C++: `int MSDriveWay::matchesPastRoute(SUMOVehicle &) const` [const, protected]
    // int
    // MSDriveWay::matchesPastRoute(SUMOVehicle& sveh) const {
    //     // look backwards along the route to find the driveway lane
    //     const ConstMSEdgeVector& routeEdges = sveh.getRoute().getEdges();
    //     for (int i = sveh.getRoutePosition(); i >= 0; i--) {
    //         if (routeEdges[i] == myLane->getNextNormal()) {
    //             MSRouteIterator firstIt = routeEdges.begin() + i;
    //             if (match(firstIt, sveh.getRoute().end())) {
    //                 // driveway is still valid after rerouting
    //                 //std::cout << SIMTIME << " notifyReroute " << getDescription() << " veh=" << veh.getID() << " valid\n";
    //                 return sveh.getRoutePosition() - i;
    //             }
    //             break;
    //         }
    //     }
    //     return -1;
    // }
    #[allow(unused_variables)]
    pub fn matches_past_route(&self, sveh: &mut SUMOVehicle) -> i32 {
        // look backwards along the route to find the driveway lane
        let mveh = Ptr::from_mut(sveh).cast::<MSVehicle>();
        let route_edges = mveh.get_route().get_edges();
        let position = mveh.get_route_position();
        let mut i = position;
        while i >= 0 {
            if route_edges[i as usize] == self.myLane.get_next_normal() {
                if self.match_(route_edges, i as usize, route_edges.len()) {
                    // driveway is still valid after rerouting
                    return position - i;
                }
                break;
            }
            i -= 1;
        }
        -1
    }

    /// C++: `void MSDriveWay::enterDriveWay(SUMOVehicle &, Notification)` [protected]
    /// helper method for notifyEnter
    // void
    // MSDriveWay::enterDriveWay(SUMOVehicle& sveh, Notification reason) {
    //     myTrains.insert(&sveh);
    //     if (myOrigin != nullptr) {
    //         MSRailSignalControl::getInstance().notifyApproach(myOrigin);
    //     }
    //     for (const MSDriveWay* foe : myFoes) {
    //         if (foe->myOrigin != nullptr) {
    //             MSRailSignalControl::getInstance().notifyApproach(foe->myOrigin);
    //         }
    //     }
    //     if (myWriteVehicles) {
    //         myVehicleEvents.push_back(VehicleEvent(SIMSTEP, true, sveh.getID(), reason));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn enter_drive_way(&mut self, sveh: &mut SUMOVehicle, reason: MSMoveReminder_Notification) {
        let sveh_ptr = Ptr::from_mut(sveh);
        self.myTrains.insert(sveh_ptr);
        if !self.myOrigin.is_null() {
            MSRailSignalControl::get_instance().notify_approach(self.myOrigin);
        }
        for &foe in self.myFoes.iter() {
            if !foe.myOrigin.is_null() {
                MSRailSignalControl::get_instance().notify_approach(foe.myOrigin);
            }
        }
        if MY_WRITE_VEHICLES.with(|w| w.get()) {
            self.myVehicleEvents.push(MSDriveWay_VehicleEvent::new(
                MSNet::get_instance().get_current_time_step(),
                true,
                &sveh_ptr.get_id().to_string(),
                reason,
            ));
        }
    }

    /// C++: `bool MSDriveWay::hasJoin(const SUMOVehicle *, const SUMOVehicle *)` [static, protected]
    // bool
    // MSDriveWay::hasJoin(const SUMOVehicle* ego, const SUMOVehicle* foe) {
    //     if (ego != nullptr && !MSGlobals::gUseMesoSim) {
    //         std::string joinVehicle = "";
    //         const SUMOVehicleParameter::Stop* stop = ego->getNextStopParameter();
    //         if (stop != nullptr) {
    //             joinVehicle = stop->join;
    //         }
    //         if (joinVehicle == "" && !ego->hasDeparted() && ego->getStops().size() > 1) {
    //             // check one more stop
    //             auto it = ego->getStops().begin();
    //             std::advance(it, 1);
    //             joinVehicle = it->pars.join;
    //         }
    //         if (joinVehicle != "") {
    // #ifdef DEBUG_SIGNALSTATE
    //             if (gDebugFlag4 || DEBUG_COND_DW(ego) || DEBUG_COND_DW(foe)) {
    //                 std::cout << "  joinVehicle=" << joinVehicle << "\n";
    //             }
    // #endif
    //             if (foe->getID() == joinVehicle && foe->isStopped()) {
    // #ifdef DEBUG_SIGNALSTATE
    //                 if (gDebugFlag4 || DEBUG_COND_DW(ego) || DEBUG_COND_DW(foe)) {
    //                     std::cout << "    ignore join-target '" << joinVehicle << "\n";
    //                 }
    // #endif
    //                 return true;
    //             }
    //         }
    //
    //         if (foe->isStopped() && foe->getNextStopParameter()->join == ego->getID()) {
    // #ifdef DEBUG_SIGNALSTATE
    //             if (gDebugFlag4 || DEBUG_COND_DW(ego) || DEBUG_COND_DW(foe)) {
    //                 std::cout << "    ignore " << foe->getID() << " for which ego is join-target\n";
    //             }
    // #endif
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn has_join(ego: Ptr<SUMOVehicle>, foe: Ptr<SUMOVehicle>) -> bool {
        if !ego.is_null() && !MSGlobals::use_meso_sim() {
            let mego = ego.cast::<MSVehicle>();
            let mfoe = foe.cast::<MSVehicle>();
            let mut join_vehicle = String::new();
            let stop = mego.get_next_stop_parameter();
            if !stop.is_null() {
                join_vehicle = stop.join.clone();
            }
            if join_vehicle == "" && !mego.has_departed() && mego.get_stops().len() > 1 {
                // check one more stop
                join_vehicle = mego.get_stops()[1].pars.join.clone();
            }
            if join_vehicle != "" {
                if mfoe.get_id() == join_vehicle && mfoe.is_stopped() {
                    return true;
                }
            }
            let foe_stop = mfoe.get_next_stop_parameter();
            let foe_join = if foe_stop.is_null() { String::new() } else { foe_stop.join.clone() };
            if mfoe.is_stopped() && foe_join == ego.get_id() {
                return true;
            }
        }
        false
    }

    /// C++: `bool MSDriveWay::isSwitch(const MSLink *)` [static, protected]
    // bool
    // MSDriveWay::isSwitch(const MSLink* link) {
    //     for (const MSLink* other : link->getLaneBefore()->getNormalPredecessorLane()->getLinkCont()) {
    //         if (other->getLane() != link->getLane() && !other->isTurnaround()) {
    //             return true;
    //         }
    //     }
    //     for (auto ili : link->getLane()->getIncomingLanes()) {
    //         if (ili.viaLink != link && !ili.viaLink->isTurnaround()) {
    //             return true;
    //         }
    //     }
    //     const MSLane* bidi = link->getLane()->getBidiLane();
    //     if (bidi != nullptr) {
    //         for (const MSLink* other : bidi->getLinkCont()) {
    //             if (other->getLane() != link->getLaneBefore()->getNormalPredecessorLane()->getBidiLane() && !other->isTurnaround()) {
    //                 return true;
    //             }
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn is_switch(link: Ptr<MSLink>) -> bool {
        let lane_before = link.get_lane_before();
        let normal_predecessor = lane_before.get_normal_predecessor_lane();
        for &other in normal_predecessor.get_link_cont().iter() {
            if other.get_lane() != link.get_lane() && !other.is_turnaround() {
                return true;
            }
        }
        let lane = link.get_lane();
        for ili in lane.get_incoming_lanes().iter() {
            if ili.viaLink != link && !ili.viaLink.is_turnaround() {
                return true;
            }
        }
        let bidi = lane.get_bidi_lane();
        if !bidi.is_null() {
            for &other in bidi.get_link_cont().iter() {
                if other.get_lane() != normal_predecessor.get_bidi_lane() && !other.is_turnaround() {
                    return true;
                }
            }
        }
        false
    }

    /// C++: `void MSDriveWay::_saveState(OutputDevice &) const` [const, protected]
    // void
    // MSDriveWay::_saveState(OutputDevice& out) const {
    //     if (!myTrains.empty() || haveSubTrains()) {
    //         out.openTag(isSubDriveWay() ? SUMO_TAG_SUBDRIVEWAY : SUMO_TAG_DRIVEWAY);
    //         out.writeAttr(SUMO_ATTR_ID, getID());
    //         out.writeAttr(SUMO_ATTR_EDGES, toString(myRoute));
    //         if (!myTrains.empty()) {
    //             std::vector<std::string> trainIDs;
    //             for (SUMOVehicle* veh : myTrains) {
    //                 trainIDs.push_back(veh->getID());
    //             }
    //             out.writeAttr(SUMO_ATTR_VEHICLES, toString(trainIDs));
    //         }
    //         out.closeTag();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn save_state_2(&self, out: &mut OutputDevice) {
        if !self.myTrains.is_empty() || self.have_sub_trains() {
            if self.is_sub_drive_way() {
                out.open_tag_id(SUMO_TAG_SUBDRIVEWAY);
            } else {
                out.open_tag_id(SUMO_TAG_DRIVEWAY);
            }
            out.write_attr(SUMO_ATTR_ID, &self.base2.get_id().to_string());
            out.write_attr(SUMO_ATTR_EDGES, &edge_ids(&self.myRoute));
            if !self.myTrains.is_empty() {
                let train_ids: Vec<String> = self.myTrains.iter().map(|v| v.get_id().to_string()).collect();
                out.write_attr(SUMO_ATTR_VEHICLES, &utils::common::to_string::join_to_string(&train_ids, " "));
            }
            out.close_tag("");
        }
    }

    /// C++: `std::string MSDriveWay::getTLLinkID(const MSLink *)` [static, protected]
    /// return logicID_linkIndex
    // std::string
    // MSDriveWay::getTLLinkID(const MSLink* link) {
    //     return link->getTLLogic()->getID() + "_" + toString(link->getTLIndex());
    // }
    #[allow(unused_variables)]
    pub fn get_tl_link_id(link: Ptr<MSLink>) -> String {
        format!("{}_{}", link.get_tl_logic().get_id(), link.get_tl_index())
    }

    /// C++: `std::string MSDriveWay::getJunctionLinkID(const MSLink *)` [static, protected]
    /// return junctionID_junctionLinkIndex
    // std::string
    // MSDriveWay::getJunctionLinkID(const MSLink* link) {
    //     return link->getJunction()->getID() + "_" + toString(link->getIndex());
    // }
    #[allow(unused_variables)]
    pub fn get_junction_link_id(link: Ptr<MSLink>) -> String {
        format!("{}_{}", link.get_junction().get_id(), link.get_index())
    }

    /// C++: `std::string MSDriveWay::formatVisitedMap(const LaneVisitedMap &)` [static, protected]
    /// print link descriptions
    // std::string
    // MSDriveWay::formatVisitedMap(const LaneVisitedMap& visited) {
    //     UNUSED_PARAMETER(visited);
    //     /*
    //     std::vector<const MSLane*> lanes(visited.size(), nullptr);
    //     for (auto item : visited) {
    //         lanes[item.second] = item.first;
    //     }
    //     for (auto it = lanes.begin(); it != lanes.end();) {
    //         if (*it == nullptr) {
    //             it = lanes.erase(it);
    //         } else {
    //             it++;
    //         }
    //     }
    //     return toString(lanes);
    //     */
    //     return "dummy";
    // }
    #[allow(unused_variables)]
    pub fn format_visited_map(visited: &MSRailSignal_LaneVisitedMap) -> String {
        // UNUSED_PARAMETER(visited);
        /*
        std::vector<const MSLane*> lanes(visited.size(), nullptr);
        for (auto item : visited) {
            lanes[item.second] = item.first;
        }
        for (auto it = lanes.begin(); it != lanes.end();) {
            if (*it == nullptr) {
                it = lanes.erase(it);
            } else {
                it++;
            }
        }
        return toString(lanes);
        */
        "dummy".to_string()
    }

    /// C++: `void MSDriveWay::appendMapIndex(LaneVisitedMap &, const MSLane *)` [static, protected]
    /// append to map by map index and avoid undefined behavior
    // void
    // MSDriveWay::appendMapIndex(LaneVisitedMap& map, const MSLane* lane) {
    //     // avoid undefined behavior from evaluation order
    //     const int tmp = (int)map.size();
    //     map[lane] = tmp;
    // }
    #[allow(unused_variables)]
    pub fn append_map_index(map: &mut MSRailSignal_LaneVisitedMap, lane: Ptr<MSLane>) {
        // avoid undefined behavior from evaluation order
        let tmp = map.len() as i32;
        map.insert(lane, tmp);
    }

}

/// C++ class `MSDriveWay::Siding` (microsim/traffic_lights/MSDriveWay.h)
#[derive(Clone)]
pub struct MSDriveWay_Siding {
    pub start: i32,
    pub end: i32,
    pub length: f64,
    pub intermediateEnds: Vec<i32>,
}

impl MSDriveWay_Siding {
    /// C++: `void Siding::Siding(int, int, double, std::vector<int>)`
    // Siding(int s, int e, double l, std::vector<int> iEnds) : start(s), end(e), length(l), intermediateEnds(iEnds) {}
    #[allow(unused_variables)]
    pub fn new(s: i32, e: i32, l: f64, iEnds: Vec<i32>) -> Self {
        MSDriveWay_Siding { start: s, end: e, length: l, intermediateEnds: iEnds }
    }

}

/// C++ class `MSDriveWay::VehicleEvent` (microsim/traffic_lights/MSDriveWay.h)
#[derive(Clone)]
pub struct MSDriveWay_VehicleEvent {
    pub time: SUMOTime,
    pub isEntry: bool,
    pub id: String,
    pub reason: MSMoveReminder_Notification,
}

impl MSDriveWay_VehicleEvent {
    /// C++: `void VehicleEvent::VehicleEvent(SUMOTime, bool, const std::string &, Notification)`
    // VehicleEvent(SUMOTime _time, bool _isEntry, const std::string& _id, Notification _reason):
    //             time(_time), isEntry(_isEntry), id(_id), reason(_reason) {}
    #[allow(unused_variables)]
    pub fn new(_time: SUMOTime, _isEntry: bool, _id: &str, _reason: MSMoveReminder_Notification) -> Self {
        MSDriveWay_VehicleEvent {
            time: _time,
            isEntry: _isEntry,
            id: _id.to_string(),
            reason: _reason,
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    /// A lane pointer that is only used as a distinct map key and is never dereferenced.
    fn fake_lane(addr: usize) -> Ptr<MSLane> {
        Ptr::from_raw(addr as *mut MSLane)
    }

    #[test]
    fn constructor_assigns_numerical_ids() {
        // C++: myNumericalID(temporary ? -1 : myGlobalDriveWayIndex++)
        let tmp = MSDriveWay::new(Ptr::null(), "tmp", true);
        assert_eq!(tmp.get_numerical_id(), -1);
        assert_eq!(tmp.base2.get_id(), "tmp");
        let a = MSDriveWay::new(Ptr::null(), "a", false);
        let b = MSDriveWay::new(Ptr::null(), "b", false);
        assert!(b.get_numerical_id() > a.get_numerical_id());
        // a driveway without an origin link is a departure driveway
        assert!(a.is_depart_driveway());
        assert!(!a.is_sub_drive_way());
        assert!(a.get_origin().is_null());
        assert!(a.get_parent().is_null());
    }

    #[test]
    fn append_map_index_counts_existing_entries() {
        // C++: const int tmp = (int)map.size(); map[lane] = tmp;
        let mut map: MSRailSignal_LaneVisitedMap = BTreeMap::new();
        let l1 = fake_lane(0x1000);
        let l2 = fake_lane(0x2000);
        MSDriveWay::append_map_index(&mut map, l1);
        assert_eq!(map.get(&l1), Some(&0));
        MSDriveWay::append_map_index(&mut map, l2);
        assert_eq!(map.get(&l2), Some(&1));
        assert_eq!(map.len(), 2);
        // re-visiting a lane keeps its original index
        MSDriveWay::append_map_index(&mut map, l1);
        assert_eq!(map.get(&l1), Some(&2));
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn notification_names_match_the_cpp_bijection() {
        assert_eq!(notification_string(MSMoveReminder_Notification::NOTIFICATION_DEPARTED), "departed");
        assert_eq!(notification_string(MSMoveReminder_Notification::NOTIFICATION_JUNCTION), "junction");
        assert_eq!(notification_string(MSMoveReminder_Notification::NOTIFICATION_REROUTE), "reroute");
        assert_eq!(notification_string(MSMoveReminder_Notification::NOTIFICATION_VAPORIZED_VAPORIZER), "vaporizer");
        assert_eq!(notification_string(MSMoveReminder_Notification::NOTIFICATION_NONE), "none");
    }

    #[test]
    fn siding_and_vehicle_event_constructors() {
        let s = MSDriveWay_Siding::new(1, 2, 3.5, vec![4]);
        assert_eq!(s.start, 1);
        assert_eq!(s.end, 2);
        assert_eq!(s.length, 3.5);
        assert_eq!(s.intermediateEnds, vec![4]);
        let e = MSDriveWay_VehicleEvent::new(7, true, "veh", MSMoveReminder_Notification::NOTIFICATION_JUNCTION);
        assert_eq!(e.time, 7);
        assert!(e.isEntry);
        assert_eq!(e.id, "veh");
        assert_eq!(e.reason, MSMoveReminder_Notification::NOTIFICATION_JUNCTION);
    }

    #[test]
    fn format_visited_map_returns_the_cpp_dummy() {
        assert_eq!(MSDriveWay::format_visited_map(&BTreeMap::new()), "dummy");
    }
}

