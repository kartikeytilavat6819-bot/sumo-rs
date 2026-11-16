//! Port of microsim/MSLaneChanger.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use crate::microsim::cfmodels::mscf_model::traffic_object;
use utils::geom::{NUMERICAL_EPS, POSITION_EPS};

// C++ MIN2/MAX2 macros (argument order kept so ties resolve like the macro)
fn min2(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}
fn max2(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}

// C++ #define constants from MSLaneChanger.cpp
const OPPOSITE_OVERTAKING_SAFE_TIMEGAP: f64 = 0.0;
const OPPOSITE_OVERTAKING_SAFETYGAP_HEADWAY_FACTOR: f64 = 0.0;
const OPPOSITE_OVERTAKING_SAFETY_FACTOR: f64 = 1.2;
const OPPOSITE_OVERTAKING_MAX_LOOKAHEAD: f64 = 150.0;
const OPPOSITE_OVERTAKING_MAX_LOOKAHEAD_EMERGENCY: f64 = 1000.0;
const OPPOSITE_OVERTAKING_ONCOMING_LOOKAHEAD: f64 = 1000.0;
const OPPOSITE_OVERTAKING_MAX_SPACE_TO_OVERTAKE: f64 = 1000.0;
const OPPOSITE_OVERTAKING_HILLTOP_THRESHOHOLD: f64 = 5.0;
const OPPOSITE_OVERTAKING_DEADLOCK_WAIT: f64 = 1.0;
// C++ utils/common/StdDefs.h SUMO_const_laneWidth
const SUMO_const_laneWidth: f64 = 3.2;

/// C++ typedef `std::vector<ChangeElem> MSLaneChanger::Changer`
pub type MSLaneChanger_Changer = Vec<MSLaneChanger_ChangeElem>;

/// C++ typedef `Changer::iterator MSLaneChanger::ChangerIt`
/// shortcut-free: C++ iterator modelled as an index into MSLaneChanger_Changer (end() == len())
pub type MSLaneChanger_ChangerIt = usize;

/// C++ typedef `Changer::const_iterator MSLaneChanger::ConstChangerIt`
/// index into MSLaneChanger_Changer (see MSLaneChanger_ChangerIt)
pub type MSLaneChanger_ConstChangerIt = usize;

/// Structure used for lane-change. For every lane you have to know four vehicles, the change-candidate veh and its follower and leader. Further, information about the last vehicle that changed into this lane is needed
/// C++ class `MSLaneChanger::ChangeElem` (microsim/MSLaneChanger.h)
pub struct MSLaneChanger_ChangeElem {
    /// the leader vehicle for the current change candidate
    pub lead: Ptr<MSVehicle>,
    /// the lane corresponding to this ChangeElem (the current change candidate is on this lane)
    pub lane: Ptr<MSLane>,
    /// last vehicle that changed into this lane
    pub hoppedVeh: Ptr<MSVehicle>,
    /// the next vehicle downstream of the ego vehicle that is blocked from changing to this lane
    pub lastBlocked: Ptr<MSVehicle>,
    /// the farthest downstream vehicle on this edge that is blocked from changing to this lane
    pub firstBlocked: Ptr<MSVehicle>,
    /// the next vehicle downstream of the ego vehicle that is stopped (and thus an obstacle)
    pub lastStopped: Ptr<MSVehicle>,
    pub dens: f64,
    /// whether changing is possible to either direction
    pub mayChangeRight: bool,
    pub mayChangeLeft: bool,
    /// relative indices of internal lanes with the same origin lane (siblings) only used for changes on internal edges
    pub siblings: Vec<i32>,
    /// @{
    pub ahead: MSLeaderInfo,
    pub aheadNext: MSLeaderDistanceInfo,
    /// vehicles that cannot be stored in ahead because they are outside the lane bounds
    pub outsideBounds: Vec<Ptr<MSVehicle>>,
    /// visibility distance to the closest zipper link that may be encountered when driving on this lane
    pub zipperDist: f64,
    /// the back position of the last blocked vehicle that wants to change to this lane
    pub lastBlockedBackPos: f64,
    /// the waiting time of the last blocked vehicle that wants to change to this lane
    pub lastBlockedWaitingTime: SUMOTime,
}

impl MSLaneChanger_ChangeElem {
    /// C++: `void ChangeElem::ChangeElem(MSLane *)`
    // MSLaneChanger::ChangeElem::ChangeElem(MSLane* _lane) :
    //     lead(nullptr),
    //     lane(_lane),
    //     hoppedVeh(nullptr),
    //     lastBlocked(nullptr),
    //     firstBlocked(nullptr),
    //     lastStopped(nullptr),
    //     ahead(_lane->getWidth()),
    //     aheadNext(_lane->getWidth(), nullptr, 0.),
    //     zipperDist(0),
    //     lastBlockedBackPos(-1),
    //     lastBlockedWaitingTime(-1) {
    //     if (lane->isInternal()) {
    //         for (auto ili : lane->getIncomingLanes()) {
    //             if (ili.viaLink->getState() == LINKSTATE_ZIPPER) {
    //                 zipperDist = lane->getLength();
    //                 break;
    //             }
    //         }
    //     } else {
    //         for (const MSLink* link : lane->getLinkCont()) {
    //             if (link->getState() == LINKSTATE_ZIPPER) {
    //                 zipperDist = MAX2(zipperDist, link->getFoeVisibilityDistance());
    //                 // @note: if this lane is shorter than zipperDist it would be better to extend this to any upstream edges within the
    //                 // visibility distance of the zipper link
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(_lane: Ptr<MSLane>) -> Self {
        let lane = _lane;
        let mut elem = MSLaneChanger_ChangeElem {
            lead: Ptr::null(),
            lane,
            hoppedVeh: Ptr::null(),
            lastBlocked: Ptr::null(),
            firstBlocked: Ptr::null(),
            lastStopped: Ptr::null(),
            dens: 0.0,
            mayChangeRight: false,
            mayChangeLeft: false,
            siblings: Vec::new(),
            ahead: MSLeaderInfo::new(lane.get_width(), Ptr::null(), 0.0),
            aheadNext: MSLeaderDistanceInfo::new(lane.get_width(), Ptr::null(), 0.0),
            outsideBounds: Vec::new(),
            zipperDist: 0.0,
            lastBlockedBackPos: -1.0,
            lastBlockedWaitingTime: -1,
        };
        if lane.is_internal() {
            for ili in lane.get_incoming_lanes() {
                if ili.viaLink.get_state() == LINKSTATE_ZIPPER {
                    elem.zipperDist = lane.get_length();
                    break;
                }
            }
        } else {
            for link in lane.get_link_cont() {
                if link.get_state() == LINKSTATE_ZIPPER {
                    elem.zipperDist = max2(elem.zipperDist, link.get_foe_visibility_distance());
                }
            }
        }
        elem
    }

    /// C++: `void ChangeElem::registerHop(MSVehicle *)`
    /// Register that vehicle belongs to Changer Item to after LC decisions
    // void
    // MSLaneChanger::ChangeElem::registerHop(MSVehicle* vehicle) {
    //     //std::cout << SIMTIME << " registerHop lane=" << lane->getID() << " veh=" << vehicle->getID() << "\n";
    //     lane->myTmpVehicles.insert(lane->myTmpVehicles.begin(), vehicle);
    //     dens += vehicle->getVehicleType().getLengthWithGap();
    //     hoppedVeh = vehicle;
    // }
    #[allow(unused_variables)]
    pub fn register_hop(&mut self, vehicle: Ptr<MSVehicle>) {
        //std::cout << SIMTIME << " registerHop lane=" << lane->getID() << " veh=" << vehicle->getID() << "\n";
        self.lane.m().myTmpVehicles.insert(0, vehicle);
        self.dens += vehicle.get_vehicle_type().get_length_with_gap();
        self.hoppedVeh = vehicle;
    }

}

/// Virtual dispatch table for the polymorphic methods of MSLaneChanger.
/// MSLaneChangerSublane installs its own table in `vt`; the base methods forward through it.
pub struct MSLaneChangerVt {
    pub check_change_opposite: fn(&mut MSLaneChanger, Ptr<MSVehicle>, i32, Ptr<MSLane>, &(Ptr<MSVehicle>, f64), &(Ptr<MSVehicle>, f64), &(Ptr<MSVehicle>, f64), &Vec<MSVehicle_LaneQ>) -> bool,
}

fn default_check_change_opposite(lc: &mut MSLaneChanger, vehicle: Ptr<MSVehicle>, laneOffset: i32, targetLane: Ptr<MSLane>, leader: &(Ptr<MSVehicle>, f64), neighLead: &(Ptr<MSVehicle>, f64), neighFollow: &(Ptr<MSVehicle>, f64), preb: &Vec<MSVehicle_LaneQ>) -> bool {
    lc.check_change_opposite_base(vehicle, laneOffset, targetLane, leader, neighLead, neighFollow, preb)
}

static DEFAULT_MS_LANE_CHANGER_VT: MSLaneChangerVt = MSLaneChangerVt {
    check_change_opposite: default_check_change_opposite,
};

/// Performs lane changing of vehicles
/// C++ class `MSLaneChanger` (microsim/MSLaneChanger.h)
pub struct MSLaneChanger {
    /// Virtual dispatch table; first field because this root class has no base
    pub vt: &'static MSLaneChangerVt,
    /// Container for ChangeElemements, one for every lane in the edge.
    pub myChanger: MSLaneChanger_Changer,
    /// Change-candidate. Last of the vehicles in changer. Only this one will try to change. Every vehicle on the edge will be a candidate once in the change-process.
    pub myCandi: MSLaneChanger_ChangerIt,
    pub myAllowsChanging: bool,
    /// whether this edge allows changing to the opposite direction edge
    pub myChangeToOpposite: bool,
    pub checkOpened: bool,
}

impl MSLaneChanger {
    /// C++: `void MSLaneChanger::MSLaneChanger(const std::vector<MSLane *> *, bool)`
    /// Constructor
    // MSLaneChanger::MSLaneChanger(const std::vector<MSLane*>* lanes, bool allowChanging) :
    //     myAllowsChanging(allowChanging),
    //     myChangeToOpposite(lanes->front()->getEdge().canChangeToOpposite()) {
    //
    //     // Fill the changer with the lane-data.
    //     myChanger.reserve(lanes->size());
    //     for (std::vector<MSLane*>::const_iterator lane = lanes->begin(); lane != lanes->end(); ++lane) {
    //         myChanger.push_back(ChangeElem(*lane));
    //         myChanger.back().mayChangeRight = lane != lanes->begin();
    //         myChanger.back().mayChangeLeft = (lane + 1) != lanes->end();
    //         if ((*lane)->isInternal()) {
    //             // avoid changing on internal sibling lane
    //             if (myChanger.back().mayChangeRight && (*lane)->getLogicalPredecessorLane() == (*(lane - 1))->getLogicalPredecessorLane()) {
    //                 myChanger.back().mayChangeRight = false;
    //             }
    //             if (myChanger.back().mayChangeLeft && (*lane)->getLogicalPredecessorLane() == (*(lane + 1))->getLogicalPredecessorLane()) {
    //                 myChanger.back().mayChangeLeft = false;
    //             }
    //             // avoid changing if lanes have different lengths
    //             if (myChanger.back().mayChangeRight && (*lane)->getLength() != (*(lane - 1))->getLength()) {
    //                 //std::cout << " cannot change right from lane=" << (*lane)->getID() << " len=" << (*lane)->getLength() << " to=" << (*(lane - 1))->getID() << " len2=" << (*(lane - 1))->getLength() << "\n";
    //                 myChanger.back().mayChangeRight = false;
    //             }
    //             if (myChanger.back().mayChangeLeft && (*lane)->getLength() != (*(lane + 1))->getLength()) {
    //                 //std::cout << " cannot change left from lane=" << (*lane)->getID() << " len=" << (*lane)->getLength() << " to=" << (*(lane + 1))->getID() << " len2=" << (*(lane + 1))->getLength() << "\n";
    //                 myChanger.back().mayChangeLeft = false;
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(lanes: Ptr<Vec<Ptr<MSLane>>>, allowChanging: bool) -> Self {
        let mut changer = MSLaneChanger {
            vt: &DEFAULT_MS_LANE_CHANGER_VT,
            myChanger: Vec::with_capacity(lanes.len()),
            myCandi: 0,
            myAllowsChanging: allowChanging,
            myChangeToOpposite: lanes[0].get_edge().can_change_to_opposite(),
            checkOpened: false,
        };
        // Fill the changer with the lane-data.
        for i in 0..lanes.len() {
            let lane = lanes[i];
            let mut mayChangeRight = i != 0;
            let mut mayChangeLeft = i + 1 != lanes.len();
            if lane.is_internal() {
                // avoid changing on internal sibling lane
                if mayChangeRight && lane.get_logical_predecessor_lane() == lanes[i - 1].get_logical_predecessor_lane() {
                    mayChangeRight = false;
                }
                if mayChangeLeft && lane.get_logical_predecessor_lane() == lanes[i + 1].get_logical_predecessor_lane() {
                    mayChangeLeft = false;
                }
                // avoid changing if lanes have different lengths
                if mayChangeRight && lane.get_length() != lanes[i - 1].get_length() {
                    mayChangeRight = false;
                }
                if mayChangeLeft && lane.get_length() != lanes[i + 1].get_length() {
                    mayChangeLeft = false;
                }
            }
            let mut elem = MSLaneChanger_ChangeElem::new(lane);
            elem.mayChangeRight = mayChangeRight;
            elem.mayChangeLeft = mayChangeLeft;
            changer.myChanger.push(elem);
        }
        changer
    }

    /// C++: `void MSLaneChanger::~MSLaneChanger() noexcept` [virtual]
    /// Destructor.
    // MSLaneChanger::~MSLaneChanger() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `void MSLaneChanger::laneChange(SUMOTime)`
    /// Start lane-change-process for all vehicles on the edge'e lanes.
    // void
    // MSLaneChanger::laneChange(SUMOTime t) {
    //     // This is what happens in one timestep. After initialization of the
    //     // changer, each vehicle will try to change. After that the changer
    //     // needs an update to prevent multiple changes of one vehicle.
    //     // Finally, the change-result has to be given back to the lanes.
    //     initChanger();
    //     try {
    //         while (vehInChanger()) {
    //             const bool haveChanged = change();
    //             updateChanger(haveChanged);
    //         }
    //         updateLanes(t);
    //     } catch (const ProcessError&) {
    //         // clean up locks or the gui may hang
    //         for (ChangerIt ce = myChanger.begin(); ce != myChanger.end(); ++ce) {
    //             ce->lane->releaseVehicles();
    //         }
    //         throw;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn lane_change(&mut self, t: SUMOTime) {
        // shortcut: C++ try/catch releases lane vehicle locks on ProcessError; no exception path exists in this port's signatures, so the cleanup is not run; upgrade when change() can return ProcessError
        self.init_changer();
        while self.veh_in_changer() {
            let haveChanged = self.change();
            self.update_changer(haveChanged);
        }
        self.update_lanes(t);
    }

    /// C++: `Changer & MSLaneChanger::getChanger()`
    /// return changer (only to be used by MSLaneChangerSublane from another instance)
    // Changer& getChanger() {
    //         return myChanger;
    //     }
    #[allow(unused_variables)]
    pub fn get_changer(&mut self) -> &mut MSLaneChanger_Changer {
        &mut self.myChanger
    }

    /// C++: `std::pair<double, SUMOTime> MSLaneChanger::getLastBlocked(int) const` [const]
    /// retrieve properties of a blocked vehicle that wants to chane to the lane with the given index
    // std::pair<double, SUMOTime>
    // MSLaneChanger::getLastBlocked(int index) const {
    //     assert(index >= 0 && index < (int)myChanger.size());
    //     if (myChanger[index].lane->empty()) {
    //         return std::make_pair(-1, -1);
    //     } else {
    //         return std::make_pair(
    //                 myChanger[index].lastBlockedBackPos,
    //                 myChanger[index].lastBlockedWaitingTime);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_last_blocked(&self, index: i32) -> (f64, SUMOTime) {
        if self.myChanger[index as usize].lane.empty() {
            (-1.0, -1)
        } else {
            (self.myChanger[index as usize].lastBlockedBackPos, self.myChanger[index as usize].lastBlockedWaitingTime)
        }
    }

    /// C++: `void MSLaneChanger::postloadInitLC()`
    // void
    // MSLaneChanger::postloadInitLC() {
    //     checkOpened = false;
    //     for (auto ce : myChanger) {
    //         const MSLane* lane = ce.lane;
    //         for (const MSLink* link : lane->getLinkCont()) {
    //             if (link->getTLLogic() != nullptr || link->havePriority()) {
    //                 continue;
    //             }
    //             for (auto ce2 : myChanger) {
    //                 const MSLane* lane2 = ce2.lane;
    //                 if (lane == lane2) {
    //                     continue;
    //                 }
    //                 for (const MSLink* link2 : lane2->getLinkCont()) {
    //                     if (&link->getLane()->getEdge() == &link2->getLane()->getEdge()
    //                             && link->getLane() != link2->getLane()
    //                             && (lane->getPermissions() & lane2->getPermissions() & link->getLane()->getPermissions() & link2->getLane()->getPermissions()
    //                                 & link->getViaLaneOrLane()->getPermissions() & link2->getViaLaneOrLane()->getPermissions() & ~(SVC_PEDESTRIAN | SVC_BICYCLE)) != 0
    //                             && link->getFoeLinks() != link2->getFoeLinks()) {
    //                         checkOpened = true;
    //                         return;
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn postload_init_lc(&mut self) {
        self.checkOpened = false;
        for ce in self.myChanger.iter() {
            let lane = ce.lane;
            for link in lane.get_link_cont() {
                if !link.get_tl_logic().is_null() || link.have_priority() {
                    continue;
                }
                for ce2 in self.myChanger.iter() {
                    let lane2 = ce2.lane;
                    if lane == lane2 {
                        continue;
                    }
                    for link2 in lane2.get_link_cont() {
                        if (link.get_lane().get_edge() as *const MSEdge) == (link2.get_lane().get_edge() as *const MSEdge)
                            && link.get_lane() != link2.get_lane()
                            && (lane.get_permissions() & lane2.get_permissions() & link.get_lane().get_permissions() & link2.get_lane().get_permissions()
                                & link.get_via_lane_or_lane().get_permissions() & link2.get_via_lane_or_lane().get_permissions() & !(SVC_PEDESTRIAN | SVC_BICYCLE)) != 0
                            && !std::ptr::eq(link.get_foe_links(), link2.get_foe_links())
                        {
                            self.checkOpened = true;
                            return;
                        }
                    }
                }
            }
        }
    }

    /// C++: `void MSLaneChanger::initChanger()` [virtual, protected]
    /// Initialize the changer before looping over all vehicles.
    // void
    // MSLaneChanger::initChanger() {
    //     // Prepare myChanger with a safe state.
    //     for (ChangerIt ce = myChanger.begin(); ce != myChanger.end(); ++ce) {
    //         ce->lead = nullptr;
    //         ce->hoppedVeh = nullptr;
    //         ce->lastBlocked = nullptr;
    //         ce->firstBlocked = nullptr;
    //         ce->lastStopped = nullptr;
    //         ce->dens = 0;
    //         ce->lastBlockedBackPos = -1;
    //         ce->lastBlockedWaitingTime = -1;
    //         ce->lane->getVehiclesSecure();
    //
    //         //std::cout << SIMTIME << " initChanger lane=" << ce->lane->getID() << " vehicles=" << toString(ce->lane->myVehicles) << "\n";
    //     }
    // }
    #[allow(unused_variables)]
    pub fn init_changer(&mut self) {
        // Prepare myChanger with a safe state.
        for ce in self.myChanger.iter_mut() {
            ce.lead = Ptr::null();
            ce.hoppedVeh = Ptr::null();
            ce.lastBlocked = Ptr::null();
            ce.firstBlocked = Ptr::null();
            ce.lastStopped = Ptr::null();
            ce.dens = 0.0;
            ce.lastBlockedBackPos = -1.0;
            ce.lastBlockedWaitingTime = -1;
            let _ = ce.lane.get_vehicles_secure();
        }
    }

    /// C++: `bool MSLaneChanger::vehInChanger() const` [const, protected]
    /// Check if there is a single change-candidate in the changer. Returns true if there is one.
    // bool vehInChanger() const {
    //         // If there is at least one valid vehicle under the veh's in myChanger
    //         // return true.
    //         for (ConstChangerIt ce = myChanger.begin(); ce != myChanger.end(); ++ce) {
    //             if (veh(ce) != 0) {
    //                 return true;
    //             }
    //         }
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn veh_in_changer(&self) -> bool {
        // If there is at least one valid vehicle under the veh's in myChanger
        // return true.
        for ce in 0..self.myChanger.len() {
            if !self.veh(ce).is_null() {
                return true;
            }
        }
        false
    }

    /// C++: `MSVehicle * MSLaneChanger::veh(ConstChangerIt) const` [const, protected]
    /// Returns the furthes unhandled vehicle on this change-elements lane or 0 if there is none.
    // MSVehicle* veh(ConstChangerIt ce) const {
    //         // If ce has a valid vehicle, return it. Otherwise return 0.
    //         if (!ce->lane->myVehicles.empty()) {
    //             return ce->lane->myVehicles.back();
    //         } else {
    //             return 0;
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn veh(&self, ce: MSLaneChanger_ConstChangerIt) -> Ptr<MSVehicle> {
        // If ce has a valid vehicle, return it. Otherwise return 0.
        let lane = self.myChanger[ce].lane;
        if !lane.myVehicles.is_empty() {
            *lane.myVehicles.last().unwrap()
        } else {
            Ptr::null()
        }
    }

    /// C++: `bool MSLaneChanger::change()` [virtual, protected]
    /// Find a new candidate and try to change it.
    // bool
    // MSLaneChanger::change() {
    //     // Find change-candidate. If it is on an allowed lane, try to change
    //     // to the right (there is a rule in Germany that you have to change
    //     // to the right, unless you are overtaking). If change to the right
    //     // isn't possible, check if there is a possibility to overtake (on the
    //     // left.
    //     // If candidate isn't on an allowed lane, changing to an allowed has
    //     // priority.
    //
    // #ifdef DEBUG_ACTIONSTEPS
    // //        std::cout<< "\nCHANGE" << std::endl;
    // #endif
    //
    //
    //     myCandi = findCandidate();
    //     MSVehicle* vehicle = veh(myCandi);
    //     vehicle->getLaneChangeModel().clearNeighbors();
    //
    //     if (vehicle->getLaneChangeModel().isChangingLanes() && !vehicle->getLaneChangeModel().alreadyChanged()) {
    //         return continueChange(vehicle, myCandi);
    //     }
    //     if (!myAllowsChanging || vehicle->getLaneChangeModel().alreadyChanged() || vehicle->isStoppedOnLane()) {
    //         registerUnchanged(vehicle);
    //         if (vehicle->isStoppedOnLane()) {
    //             myCandi->lastStopped = vehicle;
    //         }
    //         return false;
    //     }
    //
    //     if (!vehicle->isActive()) {
    // #ifdef DEBUG_ACTIONSTEPS
    //         if (DEBUG_COND) {
    //             std::cout << SIMTIME << " veh '" << vehicle->getID() << "' skips regular change checks." << std::endl;
    //         }
    // #endif
    //         bool changed = false;
    //         const int oldstate = vehicle->getLaneChangeModel().getOwnState();
    //         // let TraCI influence the wish to change lanes during non-actionsteps
    //         checkTraCICommands(vehicle);
    //         if (oldstate != vehicle->getLaneChangeModel().getOwnState()) {
    //             changed = applyTraCICommands(vehicle);
    //         }
    //         if (!changed) {
    //             registerUnchanged(vehicle);
    //         }
    //         return changed;
    //     }
    //
    //     // Check for changes to the opposite lane if vehicle is active
    //     std::pair<MSVehicle* const, double> leader = getRealLeader(myCandi);
    //     if (myChanger.size() == 1 || vehicle->getLaneChangeModel().isOpposite() || (!mayChange(-1) && !mayChange(1))) {
    //         if (changeOpposite(vehicle, leader, myCandi->lastStopped)) {
    //             return true;
    //         }
    //         registerUnchanged(vehicle);
    //         return false;
    //     }
    //
    //     vehicle->updateBestLanes(); // needed?
    //     for (int i = 0; i < (int) myChanger.size(); ++i) {
    //         vehicle->adaptBestLanesOccupation(i, myChanger[i].dens);
    //     }
    //
    //     const std::vector<MSVehicle::LaneQ>& preb = vehicle->getBestLanes();
    //     // check whether the vehicle wants and is able to change to right lane
    //     int stateRight = 0;
    //     if (mayChange(-1)) {
    //         stateRight = checkChangeWithinEdge(-1, leader, preb);
    //         // change if the vehicle wants to and is allowed to change
    //         if ((stateRight & LCA_RIGHT) != 0 && (stateRight & LCA_BLOCKED) == 0) {
    //             vehicle->getLaneChangeModel().setOwnState(stateRight);
    //             return startChange(vehicle, myCandi, -1);
    //         }
    //         if ((stateRight & LCA_RIGHT) != 0 && (stateRight & LCA_URGENT) != 0) {
    //             (myCandi - 1)->lastBlocked = vehicle;
    //             if ((myCandi - 1)->firstBlocked == nullptr) {
    //                 (myCandi - 1)->firstBlocked = vehicle;
    //             }
    //         }
    //     }
    //
    //     // check whether the vehicle wants and is able to change to left lane
    //     int stateLeft = 0;
    //     if (mayChange(1)) {
    //         stateLeft = checkChangeWithinEdge(1, leader, preb);
    //         // change if the vehicle wants to and is allowed to change
    //         if ((stateLeft & LCA_LEFT) != 0 && (stateLeft & LCA_BLOCKED) == 0) {
    //             vehicle->getLaneChangeModel().setOwnState(stateLeft);
    //             return startChange(vehicle, myCandi, 1);
    //         }
    //         if ((stateLeft & LCA_LEFT) != 0 && (stateLeft & LCA_URGENT) != 0) {
    //             (myCandi + 1)->lastBlocked = vehicle;
    //             if ((myCandi + 1)->firstBlocked == nullptr) {
    //                 (myCandi + 1)->firstBlocked = vehicle;
    //             }
    //         }
    //     }
    //
    //     if ((stateRight & LCA_URGENT) != 0 && (stateLeft & LCA_URGENT) != 0) {
    //         // ... wants to go to the left AND to the right
    //         // just let them go to the right lane...
    //         stateLeft = 0;
    //     }
    //     vehicle->getLaneChangeModel().setOwnState(stateRight | stateLeft);
    //
    //     // only emergency vehicles should change to the opposite side on a
    //     // multi-lane road (or vehicles that need to stop on the opposite side)
    //     if ((vehicle->getVClass() == SVC_EMERGENCY
    //             || hasOppositeStop(vehicle))
    //             && changeOpposite(vehicle, leader, myCandi->lastStopped)) {
    //         return true;
    //     }
    //
    //     registerUnchanged(vehicle);
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn change(&mut self) -> bool {
        // Find change-candidate. If it is on an allowed lane, try to change
        // to the right (there is a rule in Germany that you have to change
        // to the right, unless you are overtaking). If change to the right
        // isn't possible, check if there is a possibility to overtake (on the
        // left.
        // If candidate isn't on an allowed lane, changing to an allowed has
        // priority.
        self.myCandi = self.find_candidate();
        let vehicle = self.veh(self.myCandi);
        vehicle.m().get_lane_change_model().clear_neighbors();

        if vehicle.m().get_lane_change_model().is_changing_lanes() && !vehicle.m().get_lane_change_model().already_changed() {
            let mut from = self.myCandi;
            let ret = self.continue_change(vehicle, &mut from);
            self.myCandi = from;
            return ret;
        }
        if !self.myAllowsChanging || vehicle.m().get_lane_change_model().already_changed() || vehicle.is_stopped_on_lane() {
            self.register_unchanged(vehicle);
            if vehicle.is_stopped_on_lane() {
                let c = self.myCandi;
                self.myChanger[c].lastStopped = vehicle;
            }
            return false;
        }

        if !vehicle.is_active() {
            let mut changed = false;
            let oldstate = vehicle.m().get_lane_change_model().get_own_state();
            // let TraCI influence the wish to change lanes during non-actionsteps
            self.check_tra_ci_commands(vehicle);
            if oldstate != vehicle.m().get_lane_change_model().get_own_state() {
                changed = self.apply_tra_ci_commands(vehicle);
            }
            if !changed {
                self.register_unchanged(vehicle);
            }
            return changed;
        }

        // Check for changes to the opposite lane if vehicle is active
        let mut leader = self.get_real_leader(&self.myCandi);
        if self.myChanger.len() == 1 || vehicle.m().get_lane_change_model().is_opposite() || (!self.may_change(-1) && !self.may_change(1)) {
            let last_stopped = self.myChanger[self.myCandi].lastStopped;
            if self.change_opposite(vehicle, leader, last_stopped) {
                return true;
            }
            self.register_unchanged(vehicle);
            return false;
        }

        vehicle.m().update_best_lanes(false, Ptr::null()); // needed?
        for i in 0..self.myChanger.len() {
            let dens = self.myChanger[i].dens;
            vehicle.m().adapt_best_lanes_occupation(i as i32, dens);
        }

        let preb = vehicle.get_best_lanes();
        // check whether the vehicle wants and is able to change to right lane
        let mut stateRight: i32 = 0;
        if self.may_change(-1) {
            stateRight = self.check_change_within_edge(-1, &leader, preb);
            // change if the vehicle wants to and is allowed to change
            if (stateRight & LCA_RIGHT) != 0 && (stateRight & LCA_BLOCKED) == 0 {
                vehicle.m().get_lane_change_model().set_own_state(stateRight);
                let mut from = self.myCandi;
                let ret = self.start_change(vehicle, &mut from, -1);
                self.myCandi = from;
                return ret;
            }
            if (stateRight & LCA_RIGHT) != 0 && (stateRight & LCA_URGENT) != 0 {
                let idx = self.myCandi - 1;
                self.myChanger[idx].lastBlocked = vehicle;
                if self.myChanger[idx].firstBlocked.is_null() {
                    self.myChanger[idx].firstBlocked = vehicle;
                }
            }
        }

        // check whether the vehicle wants and is able to change to left lane
        let mut stateLeft: i32 = 0;
        if self.may_change(1) {
            stateLeft = self.check_change_within_edge(1, &leader, preb);
            // change if the vehicle wants to and is allowed to change
            if (stateLeft & LCA_LEFT) != 0 && (stateLeft & LCA_BLOCKED) == 0 {
                vehicle.m().get_lane_change_model().set_own_state(stateLeft);
                let mut from = self.myCandi;
                let ret = self.start_change(vehicle, &mut from, 1);
                self.myCandi = from;
                return ret;
            }
            if (stateLeft & LCA_LEFT) != 0 && (stateLeft & LCA_URGENT) != 0 {
                let idx = self.myCandi + 1;
                self.myChanger[idx].lastBlocked = vehicle;
                if self.myChanger[idx].firstBlocked.is_null() {
                    self.myChanger[idx].firstBlocked = vehicle;
                }
            }
        }

        if (stateRight & LCA_URGENT) != 0 && (stateLeft & LCA_URGENT) != 0 {
            // ... wants to go to the left AND to the right
            // just let them go to the right lane...
            stateLeft = 0;
        }
        vehicle.m().get_lane_change_model().set_own_state(stateRight | stateLeft);

        // only emergency vehicles should change to the opposite side on a
        // multi-lane road (or vehicles that need to stop on the opposite side)
        if vehicle.get_v_class() == SVC_EMERGENCY || MSLaneChanger::has_opposite_stop(vehicle) {
            let last_stopped = self.myChanger[self.myCandi].lastStopped;
            if self.change_opposite(vehicle, leader, last_stopped) {
                return true;
            }
        }

        self.register_unchanged(vehicle);
        false
    }

    /// C++: `bool MSLaneChanger::changeOpposite(MSVehicle *, std::pair<MSVehicle *, double>, MSVehicle *)` [protected]
    /// try changing to the opposite direction edge.
    // bool
    // MSLaneChanger::changeOpposite(MSVehicle* vehicle, std::pair<MSVehicle*, double> leader, MSVehicle* lastStopped) {
    //     // Evaluate lane-changing between opposite direction lanes
    //     if (!myChangeToOpposite) {
    //         return false;
    //     }
    //     const bool isOpposite = vehicle->getLaneChangeModel().isOpposite();
    //     MSLane* source = vehicle->getMutableLane();
    //     MSLane* opposite = isOpposite ? source->getParallelLane(1) : source->getOpposite();
    //
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //     gDebugFlag5 = DEBUG_COND;
    //     if (DEBUG_COND) {
    //         std::cout << SIMTIME << " veh=" << vehicle->getID() << " considerChangeOpposite source=" << source->getID()
    //                   << " opposite=" << Named::getIDSecure(opposite) << " lead=" << Named::getIDSecure(leader.first) << " isOpposite=" << isOpposite << "\n";
    //     }
    // #endif
    //     //There is no lane for opposite driving
    //     if (opposite == nullptr) {
    //         return false;
    //     }
    //     if (vehicle->isStopped()) {
    //         // stopped vehicles obviously should not change lanes. Usually this is
    //         // prevent by appropriate bestLane distances
    //         return false;
    //     }
    //     int ret = 0;
    //     ret = vehicle->influenceChangeDecision(ret);
    //     bool oppositeChangeByTraci = false;
    //     // Check whether a lane change to the opposite direction was requested via TraCI
    //     if ((ret & (LCA_TRACI)) != 0) {
    //         if (isOpposite && (ret & LCA_LEFT) != 0) {
    //             // stay on the opposite side
    //             return false;
    //         }
    //         oppositeChangeByTraci = true;
    //     }
    //     if (!isOpposite && !oppositeChangeByTraci && !source->allowsChangingLeft(vehicle->getVClass())) {
    //         // lane changing explicitly forbidden from this lane
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //         if (DEBUG_COND) {
    //             std::cout << "   not overtaking due to changeLeft restriction\n";
    //         }
    // #endif
    //         return false;
    //     }
    //
    //     //lane for opposite driving is not permitted
    //     if (!opposite->allowsVehicleClass(vehicle->getVClass())) {
    //         return false;
    //     }
    //
    //     const MSLane* oncomingLane = isOpposite ? source : opposite;
    //     //const MSLane* forwardLane = isOpposite ? opposite : source;
    //     // changing into the opposite direction is always to the left (XXX except for left-hand networkds)
    //     int direction = isOpposite ? -1 : 1;
    //     std::pair<MSVehicle*, double> neighLead(nullptr, -1);
    //
    //     // distance that can safely be driven on the opposite side
    //     double surplusGap = std::numeric_limits<double>::max();
    //
    //     // we need to find two vehicles:
    //     // 1) the leader that shall be overtaken (not necessarily the current leader but one of its leaders that has enough space in front)
    //     // 2) the oncoming vehicle (we need to look past vehicles that are currently overtaking through the opposite direction themselves)
    //     //
    //     // if the vehicle is driving normally, then the search for 1) starts on the current lane and 2) on the opposite lane
    //     // if the vehicle is driving on the opposite side then 1) is found on the neighboring lane and 2) on the current lane
    //
    //     std::pair<MSVehicle*, double> overtaken(nullptr, -1);
    //     // oncoming vehicle that is driving in the "correct" direction
    //     std::pair<MSVehicle*, double> oncoming(nullptr, -1);
    //     // oncoming vehicle that is driving against the flow
    //     std::pair<MSVehicle*, double> oncomingOpposite(nullptr, -1);
    //     // the maximum speed while overtaking (may be lowered if slow vehicles are
    //     // currently overtaking ahead of vehicle)
    //     double vMax = vehicle->getLane()->getVehicleMaxSpeed(vehicle);
    //     double oncomingSpeed = oncomingLane->getSpeedLimit();
    //     const bool isEmergency = vehicle->getVClass() == SVC_EMERGENCY;
    //
    //     // check for opposite direction stop
    //     if (!oppositeChangeByTraci && hasOppositeStop(vehicle)) {
    //         return checkOppositeStop(vehicle, oncomingLane, opposite, leader);
    //     }
    //
    //     if (!isOpposite && leader.first == nullptr && !oppositeChangeByTraci) {
    //         // no reason to change unless there is a leader
    //         // or we are changing back to the propper direction
    //         // XXX also check whether the leader is so far away as to be irrelevant
    //         return false;
    //     }
    //     if (!isOpposite && !oppositeChangeByTraci
    //             && !isEmergency
    //             && leader.first != nullptr) {
    //         if (leader.first->signalSet(MSGlobals::gLefthand
    //                                     ? MSVehicle::VEH_SIGNAL_BLINKER_RIGHT : MSVehicle::VEH_SIGNAL_BLINKER_LEFT)) {
    //             // do not try to overtake a vehicle that is about to turn left or wants
    //             // to change left itself
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //             if (DEBUG_COND) {
    //                 std::cout << "   not overtaking leader " << leader.first->getID() << " that has blinker set\n";
    //             }
    // #endif
    //             if (lastStopped != nullptr && vehicle->getWaitingSeconds() >= OPPOSITE_OVERTAKING_DEADLOCK_WAIT) {
    //                 neighLead = oncomingLane->getOppositeLeader(vehicle, OPPOSITE_OVERTAKING_ONCOMING_LOOKAHEAD, true, MSLane::MinorLinkMode::FOLLOW_ONCOMING);
    //                 const double lastStoppedGap = lastStopped->getBackPositionOnLane() - vehicle->getPositionOnLane() - vehicle->getVehicleType().getMinGap();
    //                 resolveDeadlock(vehicle, leader, neighLead, std::make_pair(lastStopped, lastStoppedGap));
    //             }
    //             return false;
    //         } else if (leader.second < 0) {
    //             // leaders is either a junction leader (that cannot be overtaken) or something else is wrong
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //             if (DEBUG_COND) {
    //                 std::cout << "   not overtaking leader " << leader.first->getID() << " with gap " << leader.second << "\n";
    //             }
    // #endif
    //             return false;
    //         }
    //     }
    //
    //     // checks for overtaking space
    //     double timeToOvertake = std::numeric_limits<double>::max();
    //     double spaceToOvertake = std::numeric_limits<double>::max();
    //     double maxSpaceToOvertake = 0;
    //
    //     if (oppositeChangeByTraci) {
    //         timeToOvertake = STEPS2TIME(vehicle->getInfluencer().getLaneTimeLineDuration());//todo discuss concept
    //         spaceToOvertake =  timeToOvertake * vehicle->getLane()->getVehicleMaxSpeed(vehicle);
    //     } else {
    //         if (isOpposite) {
    //             // -1 will use getMaximumBrakeDist() as look-ahead distance
    //             neighLead = opposite->getOppositeLeader(vehicle, -1, false);
    //             // make sure that overlapping vehicles on the neighboring lane are found by starting search at the back position
    //             overtaken = opposite->getLeader(vehicle, opposite->getOppositePos(vehicle->getBackPositionOnLane()), vehicle->getBestLanesContinuation(opposite));
    //             overtaken.second -= vehicle->getVehicleType().getLength();
    //             if (overtaken.first == nullptr && neighLead.first != nullptr) {
    //                 overtaken = neighLead;
    //             }
    //             if (overtaken.first != nullptr) {
    //                 overtaken = getColumnleader(maxSpaceToOvertake, vehicle, overtaken);
    //             }
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //             if (DEBUG_COND) {
    //                 std::cout << "    leaderOnSource=" << Named::getIDSecure(oncoming.first) << " gap=" << oncoming.second << "\n";
    //                 std::cout << "    leaderOnTarget=" << Named::getIDSecure(neighLead.first) << " gap=" << neighLead.second << "\n";
    //                 std::cout << "    overtaken=" << Named::getIDSecure(overtaken.first) << " gap=" << overtaken.second << "\n";
    //             }
    // #endif
    //         } else {
    //             overtaken = getColumnleader(maxSpaceToOvertake, vehicle, leader);
    //         }
    //
    //         if (overtaken.first == 0) {
    //             if (!isOpposite) {
    //                 // no reason to change to the opposite side
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //                 if (DEBUG_COND) {
    //                     std::cout << "   no leader found\n";
    //                 }
    // #endif
    //                 if (vehicle->getWaitingSeconds() >= OPPOSITE_OVERTAKING_DEADLOCK_WAIT) {
    //                     neighLead = oncomingLane->getOppositeLeader(vehicle, OPPOSITE_OVERTAKING_ONCOMING_LOOKAHEAD, true, MSLane::MinorLinkMode::FOLLOW_ONCOMING);
    //                     resolveDeadlock(vehicle, leader, neighLead, std::make_pair(nullptr, leader.second));
    //                 }
    //                 return false;
    //             }
    //         } else {
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //             if (DEBUG_COND) {
    //                 std::cout << "   compute time/space to overtake for columnLeader=" << overtaken.first->getID() << " egoGap=" << overtaken.second << "\n";
    //             }
    // #endif
    //             // if we have limited space to overtake, we may have to limit our maximum maneuver speed
    //             vMax = MIN2(vMax, getMaxOvertakingSpeed(vehicle, maxSpaceToOvertake));
    //             // there might be leader vehicles on the opposite side that also drive
    //             // against the flow which are slower than ego (must be factored into
    //             // overtaking time)
    //             computeOvertakingTime(vehicle, vMax, overtaken.first, overtaken.second, timeToOvertake, spaceToOvertake);
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //             if (DEBUG_COND) {
    //                 std::cout << SIMTIME
    //                           << " veh=" << vehicle->getID()
    //                           << " changeOpposite opposite=" << opposite->getID()
    //                           << " lead=" << Named::getIDSecure(leader.first)
    //                           << " maxSpaceToOvertake=" << maxSpaceToOvertake
    //                           << " vMax=" << vMax
    //                           << " timeToOvertake=" << timeToOvertake
    //                           << " spaceToOvertake=" << spaceToOvertake
    //                           << "\n";
    //             }
    // #endif
    //
    //             if (!isOpposite && spaceToOvertake > OPPOSITE_OVERTAKING_MAX_SPACE_TO_OVERTAKE) {
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //                 if (DEBUG_COND) {
    //                     std::cout << "   cannot changeOpposite (cannot overtake fast leader " << Named::getIDSecure(overtaken.first) << " v=" << overtaken.first->getSpeed() << ")\n";
    //                 }
    // #endif
    //                 neighLead = oncomingLane->getOppositeLeader(vehicle, OPPOSITE_OVERTAKING_ONCOMING_LOOKAHEAD, true, MSLane::MinorLinkMode::FOLLOW_ONCOMING);
    //                 bool wait = false;
    //                 if (vehicle->getWaitingSeconds() >= OPPOSITE_OVERTAKING_DEADLOCK_WAIT) {
    //                     wait = resolveDeadlock(vehicle, leader, neighLead, overtaken);
    //                 }
    //                 if (!wait && lastStopped != nullptr) {
    //                     const double lastStoppedGap = lastStopped->getBackPositionOnLane() - vehicle->getPositionOnLane() - vehicle->getVehicleType().getMinGap();
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //                     if (DEBUG_COND) {
    //                         std::cout << "  lastStopped=" << Named::getIDSecure(lastStopped) << " gap=" << lastStoppedGap << "\n";
    //                     }
    // #endif
    //                     avoidDeadlock(vehicle, neighLead, std::make_pair(lastStopped, lastStoppedGap), leader);
    //                 }
    //                 return false;
    //             }
    //         }
    //
    //         // if we have a leader vehicle that is driving in the opposite
    //         // direction, it may slow us down (update vMax)
    //         if (!isOpposite) {
    //             assert(timeToOvertake != std::numeric_limits<double>::max());
    //             assert(spaceToOvertake != std::numeric_limits<double>::max());
    //             // we keep neighLead distinct from oncoming because it determines blocking on the neigh lane
    //             // but also look for an oncoming leader to compute safety constraint
    //             double searchDist = timeToOvertake * oncomingLane->getSpeedLimit() * 2 + spaceToOvertake;
    //             neighLead = oncomingLane->getOppositeLeader(vehicle, searchDist, true, MSLane::MinorLinkMode::FOLLOW_ONCOMING);
    //             oncoming = getOncomingVehicle(oncomingLane, neighLead, searchDist, vMax, overtaken.first, MSLane::MinorLinkMode::FOLLOW_ONCOMING);
    //             oncomingOpposite = getOncomingOppositeVehicle(vehicle, overtaken, searchDist);
    //         } else {
    //             double searchDist = OPPOSITE_OVERTAKING_ONCOMING_LOOKAHEAD;
    //             oncoming = oncomingLane->getOppositeLeader(vehicle, searchDist, true);
    //             oncoming = getOncomingVehicle(oncomingLane, oncoming, searchDist, vMax, overtaken.first);
    //             oncomingOpposite = getOncomingOppositeVehicle(vehicle, overtaken, searchDist);
    //         }
    //         if (oncoming.first != nullptr && (oncoming.first->isStopped()
    //                                           || oncoming.first->getWaitingSeconds() >= OPPOSITE_OVERTAKING_DEADLOCK_WAIT)) {
    //             // finish overtaking within the available space
    //             const double oncomingGap = oncoming.second - oncoming.first->getVehicleType().getMinGap();
    //             if (oncomingGap > 0) {
    //                 vMax = MIN2(vMax, getMaxOvertakingSpeed(vehicle, oncomingGap));
    //             }
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //             if (DEBUG_COND) {
    //                 std::cout << "  oncoming=" << oncoming.first->getID() << " stopped=" << oncoming.first->isStopped()
    //                           << " halting=" << oncoming.first->getWaitingSeconds()
    //                           << " oncomingGap=" << oncomingGap
    //                           << " vMaxGap=" << getMaxOvertakingSpeed(vehicle, oncomingGap)
    //                           << " vMax=" << vMax << "\n";
    //             }
    // #endif
    //         }
    //
    //         if (overtaken.first != nullptr && vMax != vehicle->getLane()->getVehicleMaxSpeed(vehicle)) {
    //             // recompute overtaking time with slow opposite leader
    //             computeOvertakingTime(vehicle, vMax, overtaken.first, overtaken.second, timeToOvertake, spaceToOvertake);
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //             if (DEBUG_COND) {
    //                 std::cout << "  recomputed overtaking time with vMax=" << vMax
    //                           << " timeToOvertake=" << timeToOvertake
    //                           << " spaceToOvertake=" << spaceToOvertake
    //                           << "\n";
    //             }
    // #endif
    //         }
    //         if (!isOpposite) {
    //             if (spaceToOvertake > OPPOSITE_OVERTAKING_MAX_SPACE_TO_OVERTAKE) {
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //                 if (DEBUG_COND) {
    //                     std::cout << "   cannot changeOpposite (check2: cannot overtake fast leader " << Named::getIDSecure(overtaken.first) << " v=" << overtaken.first->getSpeed() << ")\n";
    //                 }
    // #endif
    //                 resolveDeadlock(vehicle, leader, neighLead, overtaken);
    //                 return false;
    //             }
    //             // check for upcoming stops
    //             if (vehicle->nextStopDist() < spaceToOvertake) {
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //                 if (DEBUG_COND) {
    //                     std::cout << "   cannot changeOpposite due to upcoming stop (dist=" << vehicle->nextStopDist() << " spaceToOvertake=" << spaceToOvertake << ")\n";
    //                 }
    // #endif
    //                 return false;
    //             }
    //             assert(timeToOvertake != std::numeric_limits<double>::max());
    //             assert(spaceToOvertake != std::numeric_limits<double>::max());
    //         }
    //
    //         // check for dangerous oncoming leader
    //         surplusGap = computeSurplusGap(vehicle, opposite, oncoming, timeToOvertake, spaceToOvertake, oncomingSpeed);
    //         if (oncomingOpposite.first != nullptr) {
    //             double oncomingSpeed2;
    //             const double conservativeTime = ceil(timeToOvertake / TS) * TS;
    //             const double conservativeSpace = conservativeTime * vehicle->getLane()->getVehicleMaxSpeed(vehicle);
    //             const double surplusGap2 = computeSurplusGap(vehicle, opposite, oncomingOpposite, conservativeTime, conservativeSpace, oncomingSpeed2, true);
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //             if (DEBUG_COND) {
    //                 std::cout << "   oncomingOpposite=" << oncomingOpposite.first->getID() << " speed=" << oncomingSpeed2 << " gap=" << oncomingOpposite.second << " surplusGap2=" << surplusGap2 << "\n";
    //             }
    // #endif
    //             surplusGap = MIN2(surplusGap, surplusGap2);
    //             oncomingSpeed = MAX2(oncomingSpeed, oncomingSpeed2);
    //             if (!isOpposite && surplusGap >= 0 && oncoming.first != nullptr && oncoming.first->isStopped()
    //                     && oncomingOpposite.second > oncoming.second) {
    //                 // even if ego can change back and forth successfully, we have to
    //                 // make sure that the oncoming vehicle can also finish its lane
    //                 // change in time
    //                 const double oSpeed = MAX2(oncomingOpposite.first->getSpeed(), NUMERICAL_EPS);
    //                 // conservative estimate
    //                 const double closingSpeed = (vehicle->getLane()->getVehicleMaxSpeed(vehicle)
    //                                              + oncomingOpposite.first->getLane()->getVehicleMaxSpeed(oncomingOpposite.first));
    //                 const double ooSTO = oncomingOpposite.second - oncoming.second + oncomingOpposite.first->getVehicleType().getLengthWithGap();
    //                 double ooTTO = ooSTO / oSpeed;
    //                 // round to multiples of step length (TS)
    //                 ooTTO = ceil(ooTTO / TS) * TS;
    //                 const double surplusGap3 = oncomingOpposite.second - ooTTO * closingSpeed;
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //                 if (DEBUG_COND) {
    //                     std::cout << "   oSpeed=" << oSpeed << " ooSTO=" << ooSTO << " ooTTO=" << ooTTO << " surplusGap3=" << surplusGap3 << "\n";
    //                 }
    // #endif
    //                 surplusGap = MIN2(surplusGap, surplusGap3);
    //             }
    //         }
    //         if (!isOpposite && surplusGap < 0) {
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //             if (DEBUG_COND) {
    //                 std::cout << "   cannot changeOpposite due to dangerous oncoming (surplusGap=" << surplusGap << ")\n";
    //             }
    // #endif
    //
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //             if (DEBUG_COND) {
    //                 if (oncoming.first->getLaneChangeModel().isOpposite()) {
    //                     std::cout << SIMTIME << " ego=" << vehicle->getID() << " does not changeOpposite due to dangerous oncoming " << oncoming.first->getID() << "  (but the leader is also opposite)\n";
    //                 }
    //             }
    // #endif
    //             avoidDeadlock(vehicle, neighLead, overtaken, leader);
    //             return false;
    //         }
    //     }
    //     // compute remaining space on the opposite side
    //     // 1. the part that remains on the current lane
    //     double usableDist = isOpposite ? vehicle->getPositionOnLane() : source->getLength() - vehicle->getPositionOnLane();
    //
    //     if (usableDist < spaceToOvertake) {
    //         // look forward along the next lanes
    //         const std::vector<MSLane*>& bestLaneConts = vehicle->getBestLanesContinuation();
    //         assert(bestLaneConts.size() >= 1);
    //         std::vector<MSLane*>::const_iterator it = bestLaneConts.begin() + 1;
    //         while (usableDist < spaceToOvertake && it != bestLaneConts.end()) {
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //             if (DEBUG_COND) {
    //                 std::cout << "      usableDist=" << usableDist << " opposite=" << Named::getIDSecure((*it)->getOpposite()) << "\n";
    //             }
    // #endif
    //             const MSLane* oppLane = (*it)->getOpposite();
    //             if ((oppLane == nullptr && !isEmergency) || (oppLane != nullptr && !oppLane->allowsVehicleClass(vehicle->getVClass()))) {
    //                 // opposite lane ends
    //                 break;
    //             }
    //             // do not overtake past a minor link or turn
    //             const MSLane* const prev = *(it - 1);
    //             if (prev != nullptr) {
    //                 const MSLink* link = prev->getLinkTo(*it);
    //                 if (link == nullptr || link->getState() == LINKSTATE_ZIPPER
    //                         || (link->getDirection() != LinkDirection::STRAIGHT && !isEmergency)
    //                         || (!link->havePriority()
    //                             // consider traci-influence
    //                             && (!vehicle->hasInfluencer() || vehicle->getInfluencer().getRespectJunctionPriority())
    //                             // consider junction model parameters
    //                             && ((!link->haveRed() && !link->haveYellow()) || !vehicle->ignoreRed(link, true)))) {
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //                     if (DEBUG_COND) {
    //                         std::cout << "   stop lookahead at link=" << (link == 0 ? "NULL" : link->getViaLaneOrLane()->getID()) << " state=" << (link == 0 ? "?" : toString(link->getState())) << " ignoreRed=" << vehicle->ignoreRed(link, true) << "\n";
    //                     }
    // #endif
    //                     break;
    //                 }
    //             }
    //             usableDist += (*it)->getLength();
    //             ++it;
    //         }
    //     }
    //     if (!isOpposite && usableDist < spaceToOvertake) {
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //         if (DEBUG_COND) {
    //             std::cout << "   cannot changeOpposite due to insufficient space (seen=" << usableDist << " spaceToOvertake=" << spaceToOvertake << ")\n";
    //         }
    // #endif
    //         return false;
    //     }
    //     if (!isOpposite && MSNet::getInstance()->hasElevation() && !overtaken.first->isStopped()) {
    //         // do not overtake before the top of a hill
    //         double searchDist = timeToOvertake * oncomingLane->getSpeedLimit() * 1.5 * vehicle->getLaneChangeModel().getOppositeSafetyFactor() + spaceToOvertake;
    //         int view = vehicle->getLane()->isInternal() ? 1 : 0;
    //         bool foundHill = vehicle->getSlope() > 0;
    //         if (foundHilltop(vehicle, foundHill, searchDist, vehicle->getBestLanesContinuation(), view, vehicle->getPositionOnLane(), vehicle->getPosition().z(), OPPOSITE_OVERTAKING_HILLTOP_THRESHOHOLD)) {
    //             return false;
    //         }
    //     }
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //     if (DEBUG_COND) {
    //         std::cout << "   usableDist=" << usableDist << " spaceToOvertake=" << spaceToOvertake << " timeToOvertake=" << timeToOvertake << "\n";
    //     }
    // #endif
    //     // compute wish to change
    //     std::pair<MSVehicle* const, double> neighFollow = opposite->getOppositeFollower(vehicle);
    //     double oppositeLength = vehicle->getBestLanes().back().length;
    //     if (isOpposite) {
    //         const bool canOvertake = spaceToOvertake <= OPPOSITE_OVERTAKING_MAX_SPACE_TO_OVERTAKE;
    //         oppositeLength = computeSafeOppositeLength(vehicle, oppositeLength, source, usableDist, oncoming, vMax, oncomingSpeed, neighLead, overtaken, neighFollow, surplusGap, opposite, canOvertake);
    //         leader.first = nullptr;
    //         if (neighLead.first != nullptr && neighLead.first->getLaneChangeModel().isOpposite()) {
    //             // ignore oncoming vehicle on the target lane (it might even change back in this step)
    //             neighLead.first = nullptr;
    //         }
    //     } else {
    //         if (leader.first != nullptr && neighLead.first != nullptr && leader.first->getWaitingSeconds() >= OPPOSITE_OVERTAKING_DEADLOCK_WAIT && !isEmergency) {
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //             if (DEBUG_COND) {
    //                 std::cout << "   not changing to avoid deadlock\n";
    //             }
    // #endif
    //             return false;
    //         }
    //         if (neighLead.first != nullptr && neighLead.first->isStopped()) {
    //             // do not start overtaking if the opposite side has been waitin for longer
    //             if (yieldToOppositeWaiting(vehicle, neighLead.first, 10)) {
    //                 return false;
    //             }
    //         }
    //         if (oncoming.first != nullptr && oncoming.first != neighLead.first && oncoming.first->isStopped()) {
    //             // only abort the current column of overtaking vehicles if the opposite side has been waiting long enough
    //             if (yieldToOppositeWaiting(vehicle, oncoming.first, 10, TIME2STEPS(60))) {
    //                 return false;
    //             }
    //         }
    //     }
    //     std::vector<MSVehicle::LaneQ> preb = getBestLanesOpposite(vehicle, nullptr, oppositeLength);
    //     return checkChangeOpposite(vehicle, direction, opposite, leader, neighLead, neighFollow, preb);
    // }
    #[allow(unused_variables)]
    pub fn change_opposite(&mut self, vehicle: Ptr<MSVehicle>, leader: (Ptr<MSVehicle>, f64), lastStopped: Ptr<MSVehicle>) -> bool {
        // Evaluate lane-changing between opposite direction lanes
        if !self.myChangeToOpposite {
            return false;
        }
        let mut leader = leader;
        let isOpposite = vehicle.get_lane_change_model_2().is_opposite();
        let source = vehicle.get_mutable_lane();
        let opposite = if isOpposite { source.get_parallel_lane(1, true) } else { source.get_opposite() };

        //There is no lane for opposite driving
        if opposite.is_null() {
            return false;
        }
        if vehicle.is_stopped() {
            // stopped vehicles obviously should not change lanes. Usually this is
            // prevent by appropriate bestLane distances
            return false;
        }
        let mut ret: i32 = 0;
        ret = vehicle.m().influence_change_decision(ret);
        let mut oppositeChangeByTraci = false;
        // Check whether a lane change to the opposite direction was requested via TraCI
        if (ret & (LCA_TRACI)) != 0 {
            if isOpposite && (ret & LCA_LEFT) != 0 {
                // stay on the opposite side
                return false;
            }
            oppositeChangeByTraci = true;
        }
        if !isOpposite && !oppositeChangeByTraci && !source.allows_changing_left(vehicle.get_v_class()) {
            // lane changing explicitly forbidden from this lane
            return false;
        }

        //lane for opposite driving is not permitted
        if !opposite.allows_vehicle_class(vehicle.get_v_class()) {
            return false;
        }

        let oncomingLane = if isOpposite { source } else { opposite };
        // changing into the opposite direction is always to the left (XXX except for left-hand networkds)
        let direction: i32 = if isOpposite { -1 } else { 1 };
        let mut neighLead: (Ptr<MSVehicle>, f64) = (Ptr::null(), -1.0);

        // distance that can safely be driven on the opposite side
        let mut surplusGap: f64 = f64::MAX;

        let mut overtaken: (Ptr<MSVehicle>, f64) = (Ptr::null(), -1.0);
        // oncoming vehicle that is driving in the "correct" direction
        let mut oncoming: (Ptr<MSVehicle>, f64) = (Ptr::null(), -1.0);
        // oncoming vehicle that is driving against the flow
        let mut oncomingOpposite: (Ptr<MSVehicle>, f64) = (Ptr::null(), -1.0);
        // the maximum speed while overtaking (may be lowered if slow vehicles are
        // currently overtaking ahead of vehicle)
        let mut vMax: f64 = vehicle.get_lane().get_vehicle_max_speed(traffic_object(vehicle));
        let mut oncomingSpeed: f64 = oncomingLane.get_speed_limit();
        let isEmergency = vehicle.get_v_class() == SVC_EMERGENCY;

        // check for opposite direction stop
        if !oppositeChangeByTraci && MSLaneChanger::has_opposite_stop(vehicle) {
            return self.check_opposite_stop(vehicle, oncomingLane, opposite, leader);
        }

        if !isOpposite && leader.0.is_null() && !oppositeChangeByTraci {
            // no reason to change unless there is a leader
            // or we are changing back to the propper direction
            // XXX also check whether the leader is so far away as to be irrelevant
            return false;
        }
        if !isOpposite && !oppositeChangeByTraci && !isEmergency && !leader.0.is_null() {
            let signal = if crate::microsim::ms_globals::lefthand() { MSVehicle_Signalling::VEH_SIGNAL_BLINKER_RIGHT as i32 } else { MSVehicle_Signalling::VEH_SIGNAL_BLINKER_LEFT as i32 };
            if leader.0.signal_set(signal) {
                // do not try to overtake a vehicle that is about to turn left or wants
                // to change left itself
                if !lastStopped.is_null() && vehicle.get_waiting_seconds() >= OPPOSITE_OVERTAKING_DEADLOCK_WAIT {
                    neighLead = oncomingLane.get_opposite_leader(vehicle, OPPOSITE_OVERTAKING_ONCOMING_LOOKAHEAD, true, MSLane_MinorLinkMode::FOLLOW_ONCOMING);
                    let lastStoppedGap = lastStopped.get_back_position_on_lane_2() - vehicle.get_position_on_lane() - vehicle.get_vehicle_type().get_min_gap();
                    self.resolve_deadlock(vehicle, leader, neighLead, (lastStopped, lastStoppedGap));
                }
                return false;
            } else if leader.1 < 0.0 {
                // leaders is either a junction leader (that cannot be overtaken) or something else is wrong
                return false;
            }
        }

        // checks for overtaking space
        let mut timeToOvertake: f64 = f64::MAX;
        let mut spaceToOvertake: f64 = f64::MAX;
        let mut maxSpaceToOvertake: f64 = 0.0;

        if oppositeChangeByTraci {
            timeToOvertake = steps2time(vehicle.m().get_influencer().get_lane_time_line_duration());//todo discuss concept
            spaceToOvertake = timeToOvertake * vehicle.get_lane().get_vehicle_max_speed(traffic_object(vehicle));
        } else {
            if isOpposite {
                // -1 will use getMaximumBrakeDist() as look-ahead distance
                neighLead = opposite.get_opposite_leader(vehicle, -1.0, false, MSLane_MinorLinkMode::FOLLOW_NEVER);
                // make sure that overlapping vehicles on the neighboring lane are found by starting search at the back position
                overtaken = opposite.get_leader(vehicle, opposite.get_opposite_pos(vehicle.get_back_position_on_lane_2()), vehicle.get_best_lanes_continuation_2(opposite), -1.0, false);
                overtaken.1 -= vehicle.get_vehicle_type().get_length();
                if overtaken.0.is_null() && !neighLead.0.is_null() {
                    overtaken = neighLead;
                }
                if !overtaken.0.is_null() {
                    overtaken = MSLaneChanger::get_columnleader(&mut maxSpaceToOvertake, vehicle, overtaken, f64::MAX);
                }
            } else {
                overtaken = MSLaneChanger::get_columnleader(&mut maxSpaceToOvertake, vehicle, leader, f64::MAX);
            }

            if overtaken.0.is_null() {
                if !isOpposite {
                    // no reason to change to the opposite side
                    if vehicle.get_waiting_seconds() >= OPPOSITE_OVERTAKING_DEADLOCK_WAIT {
                        neighLead = oncomingLane.get_opposite_leader(vehicle, OPPOSITE_OVERTAKING_ONCOMING_LOOKAHEAD, true, MSLane_MinorLinkMode::FOLLOW_ONCOMING);
                        self.resolve_deadlock(vehicle, leader, neighLead, (Ptr::null(), leader.1));
                    }
                    return false;
                }
            } else {
                // if we have limited space to overtake, we may have to limit our maximum maneuver speed
                vMax = min2(vMax, MSLaneChanger::get_max_overtaking_speed(vehicle, maxSpaceToOvertake));
                // there might be leader vehicles on the opposite side that also drive
                // against the flow which are slower than ego (must be factored into
                // overtaking time)
                MSLaneChanger::compute_overtaking_time(vehicle, vMax, overtaken.0, overtaken.1, &mut timeToOvertake, &mut spaceToOvertake);

                if !isOpposite && spaceToOvertake > OPPOSITE_OVERTAKING_MAX_SPACE_TO_OVERTAKE {
                    neighLead = oncomingLane.get_opposite_leader(vehicle, OPPOSITE_OVERTAKING_ONCOMING_LOOKAHEAD, true, MSLane_MinorLinkMode::FOLLOW_ONCOMING);
                    let mut wait = false;
                    if vehicle.get_waiting_seconds() >= OPPOSITE_OVERTAKING_DEADLOCK_WAIT {
                        wait = self.resolve_deadlock(vehicle, leader, neighLead, overtaken);
                    }
                    if !wait && !lastStopped.is_null() {
                        let lastStoppedGap = lastStopped.get_back_position_on_lane_2() - vehicle.get_position_on_lane() - vehicle.get_vehicle_type().get_min_gap();
                        self.avoid_deadlock(vehicle, neighLead, (lastStopped, lastStoppedGap), leader);
                    }
                    return false;
                }
            }

            // if we have a leader vehicle that is driving in the opposite
            // direction, it may slow us down (update vMax)
            if !isOpposite {
                // we keep neighLead distinct from oncoming because it determines blocking on the neigh lane
                // but also look for an oncoming leader to compute safety constraint
                let searchDist = timeToOvertake * oncomingLane.get_speed_limit() * 2.0 + spaceToOvertake;
                neighLead = oncomingLane.get_opposite_leader(vehicle, searchDist, true, MSLane_MinorLinkMode::FOLLOW_ONCOMING);
                oncoming = self.get_oncoming_vehicle(oncomingLane, neighLead, searchDist, &mut vMax, overtaken.0, MSLane_MinorLinkMode::FOLLOW_ONCOMING);
                oncomingOpposite = self.get_oncoming_opposite_vehicle(vehicle, overtaken, searchDist);
            } else {
                let searchDist = OPPOSITE_OVERTAKING_ONCOMING_LOOKAHEAD;
                oncoming = oncomingLane.get_opposite_leader(vehicle, searchDist, true, MSLane_MinorLinkMode::FOLLOW_NEVER);
                oncoming = self.get_oncoming_vehicle(oncomingLane, oncoming, searchDist, &mut vMax, overtaken.0, MSLane_MinorLinkMode::FOLLOW_NEVER);
                oncomingOpposite = self.get_oncoming_opposite_vehicle(vehicle, overtaken, searchDist);
            }
            if !oncoming.0.is_null() && (oncoming.0.is_stopped() || oncoming.0.get_waiting_seconds() >= OPPOSITE_OVERTAKING_DEADLOCK_WAIT) {
                // finish overtaking within the available space
                let oncomingGap = oncoming.1 - oncoming.0.get_vehicle_type().get_min_gap();
                if oncomingGap > 0.0 {
                    vMax = min2(vMax, MSLaneChanger::get_max_overtaking_speed(vehicle, oncomingGap));
                }
            }
            if !overtaken.0.is_null() && vMax != vehicle.get_lane().get_vehicle_max_speed(traffic_object(vehicle)) {
                // recompute overtaking time with slow opposite leader
                MSLaneChanger::compute_overtaking_time(vehicle, vMax, overtaken.0, overtaken.1, &mut timeToOvertake, &mut spaceToOvertake);
            }

            if !isOpposite {
                if spaceToOvertake > OPPOSITE_OVERTAKING_MAX_SPACE_TO_OVERTAKE {
                    self.resolve_deadlock(vehicle, leader, neighLead, overtaken);
                    return false;
                }
                // check for upcoming stops
                if vehicle.next_stop_dist() < spaceToOvertake {
                    return false;
                }
            }

            // check for dangerous oncoming leader
            surplusGap = MSLaneChanger::compute_surplus_gap(vehicle, opposite, oncoming, timeToOvertake, spaceToOvertake, &mut oncomingSpeed, false);
            if !oncomingOpposite.0.is_null() {
                let mut oncomingSpeed2: f64 = 0.0;
                let conservativeTime = (timeToOvertake / ts()).ceil() * ts();
                let conservativeSpace = conservativeTime * vehicle.get_lane().get_vehicle_max_speed(traffic_object(vehicle));
                let surplusGap2 = MSLaneChanger::compute_surplus_gap(vehicle, opposite, oncomingOpposite, conservativeTime, conservativeSpace, &mut oncomingSpeed2, true);
                surplusGap = min2(surplusGap, surplusGap2);
                oncomingSpeed = max2(oncomingSpeed, oncomingSpeed2);
                if !isOpposite && surplusGap >= 0.0 && !oncoming.0.is_null() && oncoming.0.is_stopped() && oncomingOpposite.1 > oncoming.1 {
                    // even if ego can change back and forth successfully, we have to
                    // make sure that the oncoming vehicle can also finish its lane
                    // change in time
                    let oSpeed = max2(oncomingOpposite.0.get_speed(), NUMERICAL_EPS);
                    // conservative estimate
                    let closingSpeed = vehicle.get_lane().get_vehicle_max_speed(traffic_object(vehicle)) + oncomingOpposite.0.get_lane().get_vehicle_max_speed(traffic_object(oncomingOpposite.0));
                    let ooSTO = oncomingOpposite.1 - oncoming.1 + oncomingOpposite.0.get_vehicle_type().get_length_with_gap();
                    let mut ooTTO = ooSTO / oSpeed;
                    // round to multiples of step length (TS)
                    ooTTO = (ooTTO / ts()).ceil() * ts();
                    let surplusGap3 = oncomingOpposite.1 - ooTTO * closingSpeed;
                    surplusGap = min2(surplusGap, surplusGap3);
                }
            }
            if !isOpposite && surplusGap < 0.0 {
                self.avoid_deadlock(vehicle, neighLead, overtaken, leader);
                return false;
            }
        }
        // compute remaining space on the opposite side
        // 1. the part that remains on the current lane
        let mut usableDist: f64 = if isOpposite { vehicle.get_position_on_lane() } else { source.get_length() - vehicle.get_position_on_lane() };

        if usableDist < spaceToOvertake {
            // look forward along the next lanes
            let bestLaneConts = vehicle.get_best_lanes_continuation();
            let mut it: usize = 1;
            while usableDist < spaceToOvertake && it < bestLaneConts.len() {
                let cur = bestLaneConts[it];
                let oppLane = cur.get_opposite();
                if (oppLane.is_null() && !isEmergency) || (!oppLane.is_null() && !oppLane.allows_vehicle_class(vehicle.get_v_class())) {
                    // opposite lane ends
                    break;
                }
                // do not overtake past a minor link or turn
                let prev = bestLaneConts[it - 1];
                if !prev.is_null() {
                    let link = prev.get_link_to(cur);
                    if link.is_null()
                        || link.get_state() == LINKSTATE_ZIPPER
                        || (link.get_direction() != LinkDirection::STRAIGHT && !isEmergency)
                        || (!link.have_priority()
                            // consider traci-influence
                            && (!vehicle.has_influencer() || vehicle.m().get_influencer().get_respect_junction_priority())
                            // consider junction model parameters
                            && ((!link.have_red() && !link.have_yellow()) || !vehicle.ignore_red(link, true)))
                    {
                        break;
                    }
                }
                usableDist += cur.get_length();
                it += 1;
            }
        }

        if !isOpposite && usableDist < spaceToOvertake {
            return false;
        }

        if !isOpposite && MSNet::get_instance().has_elevation() && !overtaken.0.is_stopped() {
            // do not overtake before the top of a hill
            let searchDist = timeToOvertake * oncomingLane.get_speed_limit() * 1.5 * vehicle.get_lane_change_model_2().get_opposite_safety_factor() + spaceToOvertake;
            let view: i32 = if vehicle.get_lane().is_internal() { 1 } else { 0 };
            let foundHill = vehicle.get_slope() > 0.0;
            if MSLaneChanger::found_hilltop(vehicle, foundHill, searchDist, vehicle.get_best_lanes_continuation(), view, vehicle.get_position_on_lane(), vehicle.get_position(0.0).z(), OPPOSITE_OVERTAKING_HILLTOP_THRESHOHOLD) {
                return false;
            }
        }

        // compute wish to change
        let neighFollow = opposite.get_opposite_follower(vehicle);
        let mut oppositeLength: f64 = vehicle.get_best_lanes().last().unwrap().length;
        if isOpposite {
            let canOvertake = spaceToOvertake <= OPPOSITE_OVERTAKING_MAX_SPACE_TO_OVERTAKE;
            oppositeLength = self.compute_safe_opposite_length(vehicle, oppositeLength, source, usableDist, oncoming, vMax, oncomingSpeed, neighLead, overtaken, neighFollow, surplusGap, opposite, canOvertake);
            leader.0 = Ptr::null();
            if !neighLead.0.is_null() && neighLead.0.get_lane_change_model_2().is_opposite() {
                // ignore oncoming vehicle on the target lane (it might even change back in this step)
                neighLead.0 = Ptr::null();
            }
        } else {
            if !leader.0.is_null() && !neighLead.0.is_null() && leader.0.get_waiting_seconds() >= OPPOSITE_OVERTAKING_DEADLOCK_WAIT && !isEmergency {
                return false;
            }
            if !neighLead.0.is_null() && neighLead.0.is_stopped() {
                // do not start overtaking if the opposite side has been waitin for longer
                if self.yield_to_opposite_waiting(vehicle, neighLead.0, 10.0, 0) {
                    return false;
                }
            }
            if !oncoming.0.is_null() && oncoming.0 != neighLead.0 && oncoming.0.is_stopped() {
                // only abort the current column of overtaking vehicles if the opposite side has been waiting long enough
                if self.yield_to_opposite_waiting(vehicle, oncoming.0, 10.0, time2steps(60.0)) {
                    return false;
                }
            }
        }
        let preb = MSLaneChanger::get_best_lanes_opposite(vehicle, Ptr::null(), oppositeLength);
        self.check_change_opposite(vehicle, direction, opposite, &leader, &neighLead, &neighFollow, &preb)
    }

    /// C++: `std::pair<MSVehicle *const, double> MSLaneChanger::getOncomingVehicle(const MSLane *, std::pair<MSVehicle *, double>, double, double &, const MSVehicle *, MSLane::MinorLinkMode)` [protected]
    // std::pair<MSVehicle* const, double>
    // MSLaneChanger::getOncomingVehicle(const MSLane* opposite, std::pair<MSVehicle*, double> oncoming,
    //                                   double searchDist, double& vMax, const MSVehicle* overtaken, MSLane::MinorLinkMode mLinkMode) {
    //     double gap = oncoming.second;
    //     while (oncoming.first != nullptr && (oncoming.first->getLaneChangeModel().isOpposite() || oncoming.first->getLaneChangeModel().getShadowLane() == opposite)) {
    //         searchDist -= (oncoming.first->getVehicleType().getLengthWithGap() + MAX2(0.0, oncoming.second));
    //         // leader is itself overtaking through the opposite side. find real oncoming vehicle
    //         gap += oncoming.first->getVehicleType().getLengthWithGap();
    //         if (oncoming.first != overtaken) {
    //             vMax = MIN2(vMax, oncoming.first->getSpeed());
    //         } // else: might be the case if we are overtaking a vehicle that is stopped on the opposite side
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //         if (gDebugFlag5) {
    //             std::cout << SIMTIME << " oncoming=" << oncoming.first->getID() << " isOpposite gap=" << oncoming.second
    //                       << " totalGap=" << gap << " searchDist=" << searchDist << " vMax=" << vMax << "\n";
    //         }
    // #endif
    //         if (searchDist < 0) {
    //             break;
    //         }
    //         // getOppositeLeader resets the search postion by ego length and may thus create cycles
    //         if (oncoming.first->getLaneChangeModel().getShadowLane() != opposite) {
    //             opposite = oncoming.first->getLane();
    //         }
    //         oncoming = opposite->getFollower(oncoming.first, oncoming.first->getPositionOnLane(opposite), searchDist, mLinkMode, true);
    //         if (oncoming.first != nullptr) {
    //             gap += oncoming.second + oncoming.first->getVehicleType().getLength();
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //             if (gDebugFlag5) {
    //                 std::cout << SIMTIME << " oncoming=" << oncoming.first->getID() << " gap=" << oncoming.second << " totalGap=" << gap << "\n";
    //             }
    // #endif
    //         }
    //     }
    //     oncoming.second = gap;
    //     return oncoming;
    // }
    #[allow(unused_variables)]
    pub fn get_oncoming_vehicle(&mut self, opposite: Ptr<MSLane>, neighOncoming: (Ptr<MSVehicle>, f64), searchDist: f64, vMax: &mut f64, overtaken: Ptr<MSVehicle>, mLinkMode: MSLane_MinorLinkMode) -> (Ptr<MSVehicle>, f64) {
        let mut opposite = opposite;
        let mut oncoming = neighOncoming;
        let mut searchDist = searchDist;
        let mut gap = oncoming.1;
        while !oncoming.0.is_null() && (oncoming.0.get_lane_change_model_2().is_opposite() || oncoming.0.get_lane_change_model_2().get_shadow_lane() == opposite) {
            searchDist -= oncoming.0.get_vehicle_type().get_length_with_gap() + max2(0.0, oncoming.1);
            // leader is itself overtaking through the opposite side. find real oncoming vehicle
            gap += oncoming.0.get_vehicle_type().get_length_with_gap();
            if oncoming.0 != overtaken {
                *vMax = min2(*vMax, oncoming.0.get_speed());
            } // else: might be the case if we are overtaking a vehicle that is stopped on the opposite side
            if searchDist < 0.0 {
                break;
            }
            // getOppositeLeader resets the search postion by ego length and may thus create cycles
            if oncoming.0.get_lane_change_model_2().get_shadow_lane() != opposite {
                opposite = oncoming.0.get_lane();
            }
            oncoming = opposite.get_follower(oncoming.0, oncoming.0.get_position_on_lane_2(opposite), searchDist, mLinkMode, true);
            if !oncoming.0.is_null() {
                gap += oncoming.1 + oncoming.0.get_vehicle_type().get_length();
            }
        }
        oncoming.1 = gap;
        oncoming
    }

    /// C++: `std::pair<MSVehicle *const, double> MSLaneChanger::getOncomingOppositeVehicle(const MSVehicle *, std::pair<MSVehicle *, double>, double)` [protected]
    // std::pair<MSVehicle* const, double>
    // MSLaneChanger::getOncomingOppositeVehicle(const MSVehicle* vehicle, std::pair<MSVehicle*, double> overtaken, double searchDist) {
    //     double gap = 0;
    //     const MSVehicle* front = nullptr;
    //     if (overtaken.first != nullptr) {
    //         gap += overtaken.second + overtaken.first->getVehicleType().getLengthWithGap();
    //         front = overtaken.first;
    //     } else {
    //         // only for special situations (i.e. traci-triggered)
    //         front = vehicle;
    //     }
    //     // we only need to look for the next leader: If it's driving in the forward
    //     // direction, it "protects" from oncoming opposite vehicles.
    //     // all leader vehicles on the current laneChanger edge are already moved into MSLane::myTmpVehicles
    //     const bool checkTmpVehicles = front->getLane() == vehicle->getLane();
    //     std::vector<MSLane*> conts = vehicle->getBestLanesContinuation();
    //     while (conts.size() > 0 && conts.front() != front->getLane()) {
    //         conts.erase(conts.begin());
    //     }
    //     std::pair<MSVehicle* const, double> oncoming = front->getLane()->getLeader(front, front->getPositionOnLane(), conts, searchDist, checkTmpVehicles);
    //     if (oncoming.first != nullptr) {
    //         const bool isOpposite = oncoming.first->getLaneChangeModel().isOpposite();
    //         const MSLane* shadowLane = oncoming.first->getLaneChangeModel().getShadowLane();
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //         if (gDebugFlag5) {
    //             std::cout << SIMTIME
    //                       << " front=" << front->getID() << " searchDist=" << searchDist
    //                       << " oncomingOpposite=" << oncoming.first->getID()
    //                       << " gap=" << oncoming.second
    //                       << " isOpposite=" << isOpposite
    //                       << " shadowLane=" << Named::getIDSecure(shadowLane)
    //                       << "\n";
    //         }
    // #endif
    //         if (isOpposite && shadowLane != front->getLane()) {
    //             // distance was to back position (but the vehicle is oncoming)
    //             oncoming.second -= oncoming.first->getVehicleType().getLength();
    //             oncoming.second += gap;
    //             return oncoming;
    //         }
    //     }
    //     return std::make_pair(nullptr, -1);
    // }
    #[allow(unused_variables)]
    pub fn get_oncoming_opposite_vehicle(&mut self, vehicle: Ptr<MSVehicle>, overtaken: (Ptr<MSVehicle>, f64), searchDist: f64) -> (Ptr<MSVehicle>, f64) {
        let mut gap: f64 = 0.0;
        let front: Ptr<MSVehicle>;
        if !overtaken.0.is_null() {
            gap += overtaken.1 + overtaken.0.get_vehicle_type().get_length_with_gap();
            front = overtaken.0;
        } else {
            // only for special situations (i.e. traci-triggered)
            front = vehicle;
        }
        // we only need to look for the next leader: If it's driving in the forward
        // direction, it "protects" from oncoming opposite vehicles.
        // all leader vehicles on the current laneChanger edge are already moved into MSLane::myTmpVehicles
        let checkTmpVehicles = front.get_lane() == vehicle.get_lane();
        let mut conts: Vec<Ptr<MSLane>> = vehicle.get_best_lanes_continuation().clone();
        while !conts.is_empty() && conts[0] != front.get_lane() {
            conts.remove(0);
        }
        let mut oncoming = front.get_lane().get_leader(front, front.get_position_on_lane(), &conts, searchDist, checkTmpVehicles);
        if !oncoming.0.is_null() {
            let isOpposite = oncoming.0.get_lane_change_model_2().is_opposite();
            let shadowLane = oncoming.0.get_lane_change_model_2().get_shadow_lane();
            if isOpposite && shadowLane != front.get_lane() {
                // distance was to back position (but the vehicle is oncoming)
                oncoming.1 -= oncoming.0.get_vehicle_type().get_length();
                oncoming.1 += gap;
                return oncoming;
            }
        }
        (Ptr::null(), -1.0)
    }

    /// C++: `void MSLaneChanger::registerUnchanged(MSVehicle *)` [protected]
    /// Update changer for vehicles that did not change
    // void
    // MSLaneChanger::registerUnchanged(MSVehicle* vehicle) {
    //     //std::cout << SIMTIME << " registerUnchanged lane=" << myCandi->lane->getID() << " veh=" << vehicle->getID() << "\n";
    //     myCandi->lane->myTmpVehicles.insert(myCandi->lane->myTmpVehicles.begin(), veh(myCandi));
    //     myCandi->dens += vehicle->getVehicleType().getLengthWithGap();
    //     vehicle->getLaneChangeModel().unchanged();
    // }
    #[allow(unused_variables)]
    pub fn register_unchanged(&mut self, vehicle: Ptr<MSVehicle>) {
        let idx = self.myCandi;
        let candi_veh = self.veh(idx);
        self.myChanger[idx].lane.m().myTmpVehicles.insert(0, candi_veh);
        self.myChanger[idx].dens += vehicle.get_vehicle_type().get_length_with_gap();
        vehicle.m().get_lane_change_model().unchanged();
    }

    /// C++: `void MSLaneChanger::checkTraCICommands(MSVehicle *)` [protected]
    /// Take into account traci LC-commands.
    // void
    // MSLaneChanger::checkTraCICommands(MSVehicle* vehicle) {
    // #ifdef DEBUG_STATE
    //     const int oldstate = vehicle->getLaneChangeModel().getOwnState();
    // #endif
    //     vehicle->getLaneChangeModel().checkTraCICommands();
    // #ifdef DEBUG_STATE
    //     if (DEBUG_COND) {
    //         const int newstate = vehicle->getLaneChangeModel().getOwnState();
    //         std::cout << SIMTIME
    //                   << " veh=" << vehicle->getID()
    //                   << " oldState=" << toString((LaneChangeAction) oldstate)
    //                   << " newState=" << toString((LaneChangeAction) newstate)
    //                   << ((newstate & LCA_BLOCKED) != 0 ? " (blocked)" : "")
    //                   << ((newstate & LCA_OVERLAPPING) != 0 ? " (overlap)" : "")
    //                   << "\n";
    //     }
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn check_tra_ci_commands(&mut self, vehicle: Ptr<MSVehicle>) {
        vehicle.m().get_lane_change_model().check_tra_ci_commands();
    }

    /// C++: `bool MSLaneChanger::applyTraCICommands(MSVehicle *)` [protected]
    /// Execute TraCI LC-commands.
    // bool
    // MSLaneChanger::applyTraCICommands(MSVehicle* vehicle) {
    //     // Execute request if not blocked
    //     bool changed = false;
    //     const int state = vehicle->getLaneChangeModel().getOwnState();
    //     const int dir = (state & LCA_RIGHT) != 0 ? -1 : ((state & LCA_LEFT) != 0 ? 1 : 0);
    //     const bool execute = dir != 0 && ((state & LCA_BLOCKED) == 0);
    //     if (execute) {
    //         ChangerIt to = myCandi + dir;
    //         bool continuous = vehicle->getLaneChangeModel().startLaneChangeManeuver(myCandi->lane, to->lane, dir);
    //         if (continuous) {
    //             changed = continueChange(vehicle, myCandi);
    //         } else {
    //             // insert vehicle into target lane
    //             to->registerHop(vehicle);
    //             changed = true;
    //         }
    //     }
    //     return changed;
    // }
    #[allow(unused_variables)]
    pub fn apply_tra_ci_commands(&mut self, vehicle: Ptr<MSVehicle>) -> bool {
        // Execute request if not blocked
        let mut changed = false;
        let state = vehicle.m().get_lane_change_model().get_own_state();
        let dir: i32 = if (state & LCA_RIGHT) != 0 { -1 } else if (state & LCA_LEFT) != 0 { 1 } else { 0 };
        let execute = dir != 0 && ((state & LCA_BLOCKED) == 0);
        if execute {
            let to = (self.myCandi as i32 + dir) as usize;
            let source = self.myChanger[self.myCandi].lane;
            let target = self.myChanger[to].lane;
            let continuous = vehicle.m().get_lane_change_model().start_lane_change_maneuver(source, target, dir);
            if continuous {
                let mut from = self.myCandi;
                changed = self.continue_change(vehicle, &mut from);
                self.myCandi = from;
            } else {
                // insert vehicle into target lane
                self.myChanger[to].register_hop(vehicle);
                changed = true;
            }
        }
        changed
    }

    /// C++: `void MSLaneChanger::updateChanger(bool)` [virtual, protected]
    /// After the possible change, update the changer.
    // void
    // MSLaneChanger::updateChanger(bool vehHasChanged) {
    //     assert(veh(myCandi) != 0);
    //
    //     // "Push" the vehicles to the back, i.e. follower becomes vehicle,
    //     // vehicle becomes leader, and leader becomes predecessor of vehicle,
    //     // if it exists.
    //     if (!vehHasChanged) {
    //         //std::cout << SIMTIME << " updateChanger: lane=" << myCandi->lane->getID() << " has new lead=" << veh(myCandi)->getID() << "\n";
    //         myCandi->lead = veh(myCandi);
    //     }
    //
    //     MSLane::VehCont& vehicles = myCandi->lane->myVehicles;
    //     vehicles.pop_back();
    //     //std::cout << SIMTIME << " updateChanger lane=" << myCandi->lane->getID() << " vehicles=" << toString(myCandi->lane->myVehicles) << "\n";
    // }
    #[allow(unused_variables)]
    pub fn update_changer(&mut self, vehHasChanged: bool) {
        debug_assert!(!self.veh(self.myCandi).is_null());

        // "Push" the vehicles to the back, i.e. follower becomes vehicle,
        // vehicle becomes leader, and leader becomes predecessor of vehicle,
        // if it exists.
        if !vehHasChanged {
            let v = self.veh(self.myCandi);
            let c = self.myCandi;
            self.myChanger[c].lead = v;
        }

        let c = self.myCandi;
        self.myChanger[c].lane.m().myVehicles.pop();
    }

    /// C++: `void MSLaneChanger::updateLanes(SUMOTime)` [protected]
    /// During lane-change a temporary vehicle container is filled within the lanes (bad practice to modify foreign members, I know). Swap this container with the real one.
    // void
    // MSLaneChanger::updateLanes(SUMOTime t) {
    //
    //     // Update the lane's vehicle-container.
    //     // First: it is bad style to change other classes members, but for
    //     // this release, other attempts were too time-consuming. In a next
    //     // release we will change from this lane-centered design to a vehicle-
    //     // centered. This will solve many problems.
    //     // Second: this swap would be faster if vehicle-containers would have
    //     // been pointers, but then I had to change too much of the MSLane code.
    //     for (ChangerIt ce = myChanger.begin(); ce != myChanger.end(); ++ce) {
    //         ce->lane->swapAfterLaneChange(t);
    //         ce->lane->releaseVehicles();
    //         if (ce->firstBlocked != nullptr) {
    //             ce->lastBlockedBackPos = ce->firstBlocked->getBackPositionOnLane();
    //             ce->lastBlockedWaitingTime = ce->firstBlocked->getWaitingTime();
    //         }
    //         //std::cout << SIMTIME << " updateLanes lane=" << ce->lane->getID() << " myVehicles=" << toString(ce->lane->myVehicles) << " myTmpVehicles=" << toString(ce->lane->myTmpVehicles) << " firstBlocked=" << Named::getIDSecure(ce->firstBlocked) << " wt=" << ce->lastBlockedWaitingTime << "\n";
    //     }
    // }
    #[allow(unused_variables)]
    pub fn update_lanes(&mut self, t: SUMOTime) {
        for ce in self.myChanger.iter_mut() {
            ce.lane.m().swap_after_lane_change(t);
            ce.lane.release_vehicles();
            if !ce.firstBlocked.is_null() {
                ce.lastBlockedBackPos = ce.firstBlocked.get_back_position_on_lane_2();
                ce.lastBlockedWaitingTime = ce.firstBlocked.get_waiting_time(false);
            }
        }
    }

    /// C++: `ChangerIt MSLaneChanger::findCandidate()` [protected]
    /// Find current candidate. If there is none, myChanger.end() is returned.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn find_candidate(&mut self) -> MSLaneChanger_ChangerIt {
        // Find the vehicle in myChanger with the largest position. If there
        // is no vehicle in myChanger (shouldn't happen) , return myChanger.end().
        let mut max: usize = self.myChanger.len();
        for ce in 0..self.myChanger.len() {
            if self.veh(ce).is_null() {
                continue;
            }
            if max == self.myChanger.len() {
                max = ce;
                continue;
            }
            debug_assert!(!self.veh(ce).is_null());
            debug_assert!(!self.veh(max).is_null());
            if self.veh(max).get_position_on_lane() < self.veh(ce).get_position_on_lane() {
                max = ce;
            }
        }
        debug_assert!(max != self.myChanger.len());
        max
    }

    /// C++: `int MSLaneChanger::checkChangeWithinEdge(int, const std::pair<MSVehicle *const, double> &, const std::vector<MSVehicle::LaneQ> &) const` [const, protected]
    // int
    // MSLaneChanger::checkChangeWithinEdge(
    //     int laneOffset,
    //     const std::pair<MSVehicle* const, double>& leader,
    //     const std::vector<MSVehicle::LaneQ>& preb) const {
    //
    //     std::pair<MSVehicle*, double> follower = getRealFollower(myCandi);
    //     std::pair<MSVehicle* const, double> neighLead = getRealLeader(myCandi + laneOffset);
    //     std::pair<MSVehicle*, double> neighFollow = getRealFollower(myCandi + laneOffset);
    //     if (neighLead.first != nullptr && neighLead.first == neighFollow.first) {
    //         // vehicles should not be leader and follower at the same time to avoid
    //         // contradictory behavior
    //         neighFollow.first = 0;
    //     }
    //     ChangerIt target = myCandi + laneOffset;
    //     return checkChange(laneOffset, target->lane, leader, follower, neighLead, neighFollow, preb);
    // }
    #[allow(unused_variables)]
    pub fn check_change_within_edge(&self, laneOffset: i32, leader: &(Ptr<MSVehicle>, f64), preb: &Vec<MSVehicle_LaneQ>) -> i32 {
        let follower = self.get_real_follower(&self.myCandi);
        let target = (self.myCandi as i32 + laneOffset) as usize;
        let neighLead = self.get_real_leader(&target);
        let mut neighFollow = self.get_real_follower(&target);
        if !neighLead.0.is_null() && neighLead.0 == neighFollow.0 {
            // vehicles should not be leader and follower at the same time to avoid
            // contradictory behavior
            neighFollow.0 = Ptr::null();
        }
        let targetLane = self.myChanger[target].lane;
        self.check_change(laneOffset, targetLane, leader, &follower, &neighLead, &neighFollow, preb)
    }

    /// C++: `int MSLaneChanger::checkChange(int, const MSLane *, const std::pair<MSVehicle *const, double> &, const std::pair<MSVehicle *const, double> &, const std::pair<MSVehicle *const, double> &, const std::pair<MSVehicle *const, double> &, const std::vector<MSVehicle::LaneQ> &) const` [const, protected]
    // int
    // MSLaneChanger::checkChange(
    //     int laneOffset,
    //     const MSLane* targetLane,
    //     const std::pair<MSVehicle* const, double>& leader,
    //     const std::pair<MSVehicle* const, double>& follower,
    //     const std::pair<MSVehicle* const, double>& neighLead,
    //     const std::pair<MSVehicle* const, double>& neighFollow,
    //     const std::vector<MSVehicle::LaneQ>& preb) const {
    //
    //     MSVehicle* vehicle = veh(myCandi);
    //
    // #ifdef DEBUG_CHECK_CHANGE
    //     if (DEBUG_COND) {
    //         std::cout
    //                 << "\n" << SIMTIME << " checkChange() for vehicle '" << vehicle->getID() << "'"
    //                 << std::endl;
    //     }
    // #endif
    //
    //     int blocked = 0;
    //     int blockedByLeader = (laneOffset == -1 ? LCA_BLOCKED_BY_RIGHT_LEADER : LCA_BLOCKED_BY_LEFT_LEADER);
    //     int blockedByFollower = (laneOffset == -1 ? LCA_BLOCKED_BY_RIGHT_FOLLOWER : LCA_BLOCKED_BY_LEFT_FOLLOWER);
    //     // overlap
    //     if (neighFollow.first != nullptr && neighFollow.second < 0) {
    //         blocked |= (blockedByFollower | LCA_OVERLAPPING);
    //
    //         // Debug (Leo)
    // #ifdef DEBUG_CHECK_CHANGE
    //         if (DEBUG_COND) {
    //             std::cout << SIMTIME
    //                       << " overlapping with follower..."
    //                       << std::endl;
    //         }
    // #endif
    //
    //     }
    //     if (neighLead.first != nullptr && neighLead.second < 0) {
    //         blocked |= (blockedByLeader | LCA_OVERLAPPING);
    //
    // #ifdef DEBUG_CHECK_CHANGE
    //         if (DEBUG_COND) {
    //             std::cout << SIMTIME
    //                       <<  " overlapping with leader..."
    //                       << std::endl;
    //         }
    // #endif
    //
    //     }
    //     double secureFrontGap = MSAbstractLaneChangeModel::NO_NEIGHBOR;
    //     double secureBackGap = MSAbstractLaneChangeModel::NO_NEIGHBOR;
    //     double secureOrigFrontGap = MSAbstractLaneChangeModel::NO_NEIGHBOR;
    //
    //     const double tauRemainder = vehicle->getActionStepLength() == DELTA_T ? 0 : MAX2(vehicle->getCarFollowModel().getHeadwayTime() - TS, 0.);
    //     // safe back gap
    //     if ((blocked & blockedByFollower) == 0 && neighFollow.first != nullptr) {
    //         // Calculate secure gap conservatively with vNextFollower / vNextLeader as
    //         // extrapolated speeds after the driver's expected reaction time (tau).
    //         // NOTE: there exists a possible source for collisions if the follower and the leader
    //         //       have desynchronized action steps as the extrapolated speeds can be exceeded in this case
    //
    //         // Expected reaction time (tau) for the follower-vehicle.
    //         // (subtracted TS since at this point the vehicles' states are already updated)
    //         const double vNextFollower = neighFollow.first->getSpeed() + MAX2(0., tauRemainder * neighFollow.first->getAcceleration());
    //         const double vNextLeader = vehicle->getSpeed() + MIN2(0., tauRemainder * vehicle->getAcceleration());
    //         // !!! eigentlich: vsafe braucht die Max. Geschwindigkeit beider Spuren
    //         secureBackGap = neighFollow.first->getCarFollowModel().getSecureGap(neighFollow.first, vehicle, vNextFollower,
    //                         vNextLeader, vehicle->getCarFollowModel().getMaxDecel());
    // #ifdef DEBUG_CHECK_CHANGE
    //         if (DEBUG_COND) {
    //             std::cout << SIMTIME
    //                       << " follower=" << neighFollow.first->getID()
    //                       << " backGap=" << neighFollow.second
    //                       << " vNextFollower=" << vNextFollower
    //                       << " vNextEgo=" << vNextLeader
    //                       << " secureGap=" << secureBackGap
    //                       << " safetyFactor=" << vehicle->getLaneChangeModel().getSafetyFactor()
    //                       << " blocked=" << (neighFollow.second < secureBackGap * vehicle->getLaneChangeModel().getSafetyFactor())
    //                       << "\n";
    //         }
    // #endif
    //         if (neighFollow.second < secureBackGap * vehicle->getLaneChangeModel().getSafetyFactor()) {
    //             if (vehicle->getLaneChangeModel().isOpposite()
    //                     && neighFollow.first->getLaneChangeModel().getLastLaneChangeOffset() == laneOffset) {
    //                 // during opposite direction driving, the vehicle are handled in
    //                 // downstream rather than upstream order, the neighFollower may have
    //                 // been the actual follower in this simulation step and should not
    //                 // block changing in this case
    // #ifdef DEBUG_CHECK_CHANGE
    //                 if (DEBUG_COND) {
    //                     std::cout << "  ignoring opposite follower who changed in this step\n";
    //                 }
    // #endif
    //             } else {
    //                 blocked |= blockedByFollower;
    //             }
    //         }
    //     }
    //
    //     // safe front gap
    //     if ((blocked & blockedByLeader) == 0 && neighLead.first != nullptr) {
    //         // Calculate secure gap conservatively with vNextFollower / vNextLeader as
    //         // extrapolated speeds after the driver's expected reaction time (tau).
    //         // NOTE: there exists a possible source for collisions if the follower and the leader
    //         //       have desynchronized action steps as the extrapolated speeds can be exceeded in this case
    //
    //         // Expected reaction time (tau) for the follower-vehicle.
    //         // (subtracted TS since at this point the vehicles' states are already updated)
    //         const double vNextFollower = vehicle->getSpeed() + MAX2(0., tauRemainder * vehicle->getAcceleration());
    //         const double vNextLeader = neighLead.first->getSpeed() + MIN2(0., tauRemainder * neighLead.first->getAcceleration());
    //         // !!! eigentlich: vsafe braucht die Max. Geschwindigkeit beider Spuren
    //         secureFrontGap = vehicle->getCarFollowModel().getSecureGap(vehicle, neighLead.first, vNextFollower,
    //                          vNextLeader, neighLead.first->getCarFollowModel().getMaxDecel());
    // #ifdef DEBUG_CHECK_CHANGE
    //         if (DEBUG_COND) {
    //             std::cout << SIMTIME
    //                       << " leader=" << neighLead.first->getID()
    //                       << " frontGap=" << neighLead.second
    //                       << " vNextEgo=" << vNextFollower
    //                       << " vNextLeader=" << vNextLeader
    //                       << " secureGap=" << secureFrontGap
    //                       << " safetyFactor=" << vehicle->getLaneChangeModel().getSafetyFactor()
    //                       << " blocked=" << (neighLead.second < secureFrontGap * vehicle->getLaneChangeModel().getSafetyFactor())
    //                       << "\n";
    //         }
    // #endif
    //         if (neighLead.second < secureFrontGap * vehicle->getLaneChangeModel().getSafetyFactor()) {
    //             blocked |= blockedByLeader;
    //         }
    //     }
    //     if (blocked == 0 && targetLane->hasPedestrians()) {
    //         PersonDist nextLeader = targetLane->nextBlocking(vehicle->getBackPositionOnLane(),
    //                                 vehicle->getRightSideOnLane(targetLane), vehicle->getRightSideOnLane(targetLane) + vehicle->getVehicleType().getWidth(),
    //                                 ceil(vehicle->getSpeed() / vehicle->getCarFollowModel().getMaxDecel()));
    //         if (nextLeader.first != 0) {
    //             const double brakeGap = vehicle->getCarFollowModel().brakeGap(vehicle->getSpeed());
    //             // returned gap value is relative to backPosition
    //             const double gap = nextLeader.second - vehicle->getVehicleType().getLengthWithGap();
    // #ifdef DEBUG_CHECK_CHANGE
    //             if (DEBUG_COND) {
    //                 std::cout << SIMTIME << "  pedestrian on road " + leader.first->getID() << " gap=" << gap << " brakeGap=" << brakeGap << "\n";
    //             }
    // #endif
    //             if (brakeGap > gap) {
    //                 blocked |= blockedByLeader;
    // #ifdef DEBUG_CHECK_CHANGE
    //                 if (DEBUG_COND) {
    //                     std::cout << SIMTIME << "  blocked by pedestrian " + leader.first->getID() << "\n";
    //                 }
    // #endif
    //             }
    //         }
    //     }
    //
    //     if (leader.first != nullptr) {
    //         secureOrigFrontGap = vehicle->getCarFollowModel().getSecureGap(vehicle, leader.first, vehicle->getSpeed(), leader.first->getSpeed(), leader.first->getCarFollowModel().getMaxDecel());
    //     }
    //
    //     MSAbstractLaneChangeModel::MSLCMessager msg(leader.first, neighLead.first, neighFollow.first);
    //     int state = blocked | vehicle->getLaneChangeModel().wantsChange(
    //                     laneOffset, msg, blocked, leader, follower, neighLead, neighFollow, *targetLane, preb, &(myCandi->lastBlocked), &(myCandi->firstBlocked));
    //
    //     if (blocked == 0 && (state & LCA_WANTS_LANECHANGE) != 0 && neighLead.first != nullptr) {
    //         // do a more careful (but expensive) check to ensure that a
    //         // safety-critical leader is not being overlooked
    //         // while changing on an intersection, it is not sufficient to abort the
    //         // search with a leader on the current lane because all linkLeaders must
    //         // be considered as well
    //         const double seen = myCandi->lane->getLength() - vehicle->getPositionOnLane();
    //         const double speed = vehicle->getSpeed();
    //         const double dist = vehicle->getCarFollowModel().brakeGap(speed) + vehicle->getVehicleType().getMinGap();
    //         if (seen < dist || myCandi->lane->isInternal()) {
    //             std::pair<MSVehicle* const, double> neighLead2 = targetLane->getCriticalLeader(dist, seen, speed, *vehicle);
    //             if (neighLead2.first != nullptr && neighLead2.first != neighLead.first) {
    //                 const double secureGap = vehicle->getCarFollowModel().getSecureGap(vehicle, neighLead2.first, vehicle->getSpeed(),
    //                                          neighLead2.first->getSpeed(), neighLead2.first->getCarFollowModel().getMaxDecel());
    //                 const double secureGap2 = secureGap * vehicle->getLaneChangeModel().getSafetyFactor();
    // #ifdef DEBUG_SURROUNDING_VEHICLES
    //                 if (DEBUG_COND) {
    //                     std::cout << SIMTIME << "   found critical leader=" << neighLead2.first->getID()
    //                               << " gap=" << neighLead2.second << " secGap=" << secureGap << " secGap2=" << secureGap2 << "\n";
    //                 }
    // #endif
    //                 if (neighLead2.second < secureGap2) {
    //                     state |= blockedByLeader;
    //                 }
    //             }
    //         }
    //     }
    //     if (blocked == 0 && (state & LCA_WANTS_LANECHANGE)) {
    //         // ensure that merging is safe for any upcoming zipper links after changing
    //         double targetZipperDist = 0;
    //         if (laneOffset == 0) {
    //             targetZipperDist = myCandi->zipperDist;
    //         } else if (laneOffset == 1) {
    //             if ((myCandi + 1) != myChanger.end()) {
    //                 targetZipperDist = (myCandi + 1)->zipperDist;
    //             }
    //         } else if (laneOffset == -1) {
    //             if (myCandi > myChanger.begin()) {
    //                 targetZipperDist = (myCandi - 1)->zipperDist;
    //             }
    //         }
    //         if (vehicle->unsafeLinkAhead(targetLane, targetZipperDist)) {
    //             state |= blockedByLeader;
    //         }
    //     }
    //
    //     if ((state & LCA_BLOCKED) == 0 && (state & LCA_WANTS_LANECHANGE) != 0 && MSGlobals::gLaneChangeDuration > DELTA_T) {
    //         // Ensure that a continuous lane change manoeuvre can be completed before the next turning movement.
    //         // Assume lateral position == 0. (If this should change in the future add + laneOffset*vehicle->getLateralPositionOnLane() to distToNeighLane)
    //         const double distToNeighLane = 0.5 * (vehicle->getLane()->getWidth() + targetLane->getWidth());
    //         // Extrapolate the LC duration if operating with speed dependent lateral speed.
    //         const MSAbstractLaneChangeModel& lcm = vehicle->getLaneChangeModel();
    //         const double assumedDecel = lcm.getAssumedDecelForLaneChangeDuration();
    //         const double estimatedLCDuration = lcm.estimateLCDuration(vehicle->getSpeed(), distToNeighLane, assumedDecel, (state & LCA_URGENT) != 0);
    //         if (estimatedLCDuration == -1) {
    //             // Can't guarantee that LC will succeed if vehicle is braking -> assert(lcm.myMaxSpeedLatStanding==0)
    // #ifdef DEBUG_CHECK_CHANGE
    //             if (DEBUG_COND) {
    //                 std::cout << SIMTIME << " checkChange() too slow to guarantee completion of continuous lane change."
    //                           << "\nestimatedLCDuration=" << estimatedLCDuration
    //                           << "\ndistToNeighLane=" << distToNeighLane
    //                           << std::endl;
    //             }
    // #endif
    //             state |= LCA_INSUFFICIENT_SPEED;
    //         } else {
    //             // Compute covered distance, when braking for the whole lc duration
    //             const double decel = vehicle->getCarFollowModel().getMaxDecel() * estimatedLCDuration;
    //             const double avgSpeed = 0.5 * (
    //                                         MAX2(0., vehicle->getSpeed() - ACCEL2SPEED(vehicle->getCarFollowModel().getMaxDecel())) +
    //                                         MAX2(0., vehicle->getSpeed() - decel));
    //             // Distance required for lane change.
    //             const double space2change = avgSpeed * estimatedLCDuration;
    //             // Available distance for LC maneuver (distance till next turn)
    //             double seen = myCandi->lane->getLength() - vehicle->getPositionOnLane();
    // #ifdef DEBUG_CHECK_CHANGE
    //             if (DEBUG_COND) {
    //                 std::cout << SIMTIME << " checkChange() checking continuous lane change..."
    //                           << "\ndistToNeighLane=" << distToNeighLane
    //                           << " estimatedLCDuration=" << estimatedLCDuration
    //                           << " space2change=" << space2change
    //                           << " avgSpeed=" << avgSpeed
    //                           << std::endl;
    //             }
    // #endif
    //
    //             // for finding turns it doesn't matter whether we look along the current lane or the target lane
    //             const std::vector<MSLane*>& bestLaneConts = vehicle->getBestLanesContinuation();
    //             int view = 1;
    //             const MSLane* nextLane = vehicle->getLane();
    //             std::vector<MSLink*>::const_iterator link = MSLane::succLinkSec(*vehicle, view, *nextLane, bestLaneConts);
    //             while (!nextLane->isLinkEnd(link) && seen <= space2change) {
    //                 if ((*link)->getDirection() == LinkDirection::LEFT || (*link)->getDirection() == LinkDirection::RIGHT
    //                         // the lanes after an internal junction are on different
    //                         // edges and do not allow lane-changing
    //                         || (nextLane->getEdge().isInternal() && (*link)->getViaLaneOrLane()->getEdge().isInternal())
    //                    ) {
    //                     state |= LCA_INSUFFICIENT_SPACE;
    //                     break;
    //                 }
    //                 if ((*link)->getViaLane() == nullptr) {
    //                     view++;
    //                 }
    //                 nextLane = (*link)->getViaLaneOrLane();
    //                 seen += nextLane->getLength();
    //                 // get the next link used
    //                 link = MSLane::succLinkSec(*vehicle, view, *nextLane, bestLaneConts);
    //             }
    // #ifdef DEBUG_CHECK_CHANGE
    //             if (DEBUG_COND) {
    //                 std::cout << " available distance=" << seen << std::endl;
    //             }
    // #endif
    //             if (nextLane->isLinkEnd(link) && seen < space2change) {
    // #ifdef DEBUG_CHECK_CHANGE
    //                 if (DEBUG_COND) {
    //                     std::cout << SIMTIME << " checkChange insufficientSpace: seen=" << seen << " space2change=" << space2change << "\n";
    //                 }
    // #endif
    //                 state |= LCA_INSUFFICIENT_SPACE;
    //             }
    //
    //             if ((state & LCA_BLOCKED) == 0) {
    //                 // check for dangerous leaders in case the target lane changes laterally between
    //                 // now and the lane-changing midpoint
    //                 const double speed = vehicle->getSpeed();
    //                 seen = myCandi->lane->getLength() - vehicle->getPositionOnLane();
    //                 nextLane = vehicle->getLane();
    //                 view = 1;
    //                 const double dist = vehicle->getCarFollowModel().brakeGap(speed) + vehicle->getVehicleType().getMinGap();
    //                 std::vector<MSLink*>::const_iterator nextLink = MSLane::succLinkSec(*vehicle, view, *nextLane, bestLaneConts);
    //                 while (!nextLane->isLinkEnd(nextLink) && seen <= space2change && seen <= dist) {
    //                     nextLane = (*nextLink)->getViaLaneOrLane();
    //                     const MSLane* const parallelLane = nextLane->getParallelLane(laneOffset);
    //                     if (parallelLane == nullptr) {
    //                         state |= LCA_INSUFFICIENT_SPACE;
    //                         break;
    //                     } else {
    //                         std::pair<MSVehicle* const, double> neighLead2 = parallelLane->getLeader(vehicle, -seen, std::vector<MSLane*>());
    //                         if (neighLead2.first != nullptr && neighLead2.first != neighLead.first
    //                                 && (neighLead2.second < vehicle->getCarFollowModel().getSecureGap(vehicle, neighLead2.first,
    //                                         vehicle->getSpeed(), neighLead2.first->getSpeed(), neighLead2.first->getCarFollowModel().getMaxDecel()))) {
    //                             state |= blockedByLeader;
    //                             break;
    //                         }
    //                     }
    //                     if ((*nextLink)->getViaLane() == nullptr) {
    //                         view++;
    //                     }
    //                     seen += nextLane->getLength();
    //                     // get the next link used
    //                     nextLink = MSLane::succLinkSec(*vehicle, view, *nextLane, bestLaneConts);
    //                 }
    //             }
    //         }
    //     }
    //
    //     if (checkOpened && (state & LCA_BLOCKED) == 0 && (state & LCA_WANTS_LANECHANGE) != 0
    //             && vehicle->getLane()->isNormal()
    //             && vehicle->getBestLanesContinuation().size() > 1) {
    //         const MSLink* link = vehicle->getLane()->getLinkTo(vehicle->getBestLanesContinuation()[1]);
    //         if (link != nullptr && link->isEntryLink()) {
    //             const MSLink* link2 = link->getParallelLink(laneOffset);
    //             if (link2 != nullptr) {
    //                 auto api = link->getApproachingPtr(vehicle);
    //                 if (api != nullptr) {
    //                     if (!link2->opened(api->arrivalTime, api->arrivalSpeed, api->leaveSpeed, vehicle->getLength(),
    //                                        vehicle->getImpatience(), vehicle->getCarFollowModel().getMaxDecel(), vehicle->getWaitingTime(), vehicle->getLateralPositionOnLane(),
    //                                        nullptr, false, vehicle, api->dist)) {
    //                         //std::cout << SIMTIME << " unsafeLC " << vehicle->getID() << "\n";
    //                         state |= LCA_BLOCKED;
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //
    //     const int oldstate = state;
    //     // let TraCI influence the wish to change lanes and the security to take
    //     state = vehicle->influenceChangeDecision(state);
    //     blocked = state & LCA_BLOCKED;
    // #ifdef DEBUG_CHECK_CHANGE
    //     if (DEBUG_COND) {
    //         std::cout << SIMTIME
    //                   << " veh=" << vehicle->getID()
    //                   << " oldState=" << toString((LaneChangeAction)oldstate)
    //                   << " newState=" << toString((LaneChangeAction)state)
    //                   << ((blocked & LCA_BLOCKED) ? " (blocked)" : "")
    //                   << ((blocked & LCA_OVERLAPPING) ? " (overlap)" : "")
    //                   << "\n";
    //     }
    // #endif
    //
    //     if (state & LCA_WANTS_LANECHANGE && blocked == 0) {
    //         blocked = vehicle->getLaneChangeModel().checkChangeBeforeCommitting(vehicle, state);
    //         state |= blocked;
    //     }
    //     vehicle->getLaneChangeModel().saveLCState(laneOffset, oldstate, state);
    //     if (blocked == 0 && (state & LCA_WANTS_LANECHANGE)) {
    //         // this lane change will be executed, save gaps
    //         vehicle->getLaneChangeModel().setFollowerGaps(neighFollow, secureBackGap);
    //         vehicle->getLaneChangeModel().setLeaderGaps(neighLead, secureFrontGap);
    //         vehicle->getLaneChangeModel().setOrigLeaderGaps(leader, secureOrigFrontGap);
    //     }
    //     if (laneOffset != 0) {
    //         vehicle->getLaneChangeModel().saveNeighbors(laneOffset, neighFollow, neighLead);
    //     }
    //     return state;
    // }
    #[allow(unused_variables)]
    pub fn check_change(&self, laneOffset: i32, targetLane: Ptr<MSLane>, leader: &(Ptr<MSVehicle>, f64), follower: &(Ptr<MSVehicle>, f64), neighLead: &(Ptr<MSVehicle>, f64), neighFollow: &(Ptr<MSVehicle>, f64), preb: &Vec<MSVehicle_LaneQ>) -> i32 {
        let vehicle = self.veh(self.myCandi);
        let c = self.myCandi;
        let mut blocked: i32 = 0;
        let blockedByLeader: i32 = if laneOffset == -1 { LCA_BLOCKED_BY_RIGHT_LEADER } else { LCA_BLOCKED_BY_LEFT_LEADER };
        let blockedByFollower: i32 = if laneOffset == -1 { LCA_BLOCKED_BY_RIGHT_FOLLOWER } else { LCA_BLOCKED_BY_LEFT_FOLLOWER };
        // overlap
        if !neighFollow.0.is_null() && neighFollow.1 < 0.0 {
            blocked |= blockedByFollower | LCA_OVERLAPPING;
        }
        if !neighLead.0.is_null() && neighLead.1 < 0.0 {
            blocked |= blockedByLeader | LCA_OVERLAPPING;
        }

        // NO_NEIGHBOR is std::numeric_limits<double>::max() in MSAbstractLaneChangeModel.cpp
        let mut secureFrontGap: f64 = f64::MAX;
        let mut secureBackGap: f64 = f64::MAX;
        let mut secureOrigFrontGap: f64 = f64::MAX;
        let tauRemainder = if vehicle.get_action_step_length() == delta_t() { 0.0 } else { max2(vehicle.get_car_follow_model().get_headway_time() - ts(), 0.0) };

        // safe back gap
        if (blocked & blockedByFollower) == 0 && !neighFollow.0.is_null() {
            // Calculate secure gap conservatively with vNextFollower / vNextLeader as
            // extrapolated speeds after the driver's expected reaction time (tau).
            // (subtracted TS since at this point the vehicles' states are already updated)
            let vNextFollower = neighFollow.0.get_speed() + max2(0.0, tauRemainder * neighFollow.0.get_acceleration());
            let vNextLeader = vehicle.get_speed() + min2(0.0, tauRemainder * vehicle.get_acceleration());
            secureBackGap = neighFollow.0.get_car_follow_model().get_secure_gap(neighFollow.0, vehicle, vNextFollower, vNextLeader, vehicle.get_car_follow_model().get_max_decel());
            if neighFollow.1 < secureBackGap * vehicle.get_lane_change_model_2().get_safety_factor() {
                if vehicle.get_lane_change_model_2().is_opposite() && neighFollow.0.get_lane_change_model_2().get_last_lane_change_offset() == laneOffset as SUMOTime {
                    // during opposite direction driving, the vehicle are handled in
                    // downstream rather than upstream order, the neighFollower may have
                    // been the actual follower in this simulation step and should not
                    // block changing in this case
                } else {
                    blocked |= blockedByFollower;
                }
            }
        }

        // safe front gap
        if (blocked & blockedByLeader) == 0 && !neighLead.0.is_null() {
            // Calculate secure gap conservatively with vNextFollower / vNextLeader as
            // extrapolated speeds after the driver's expected reaction time (tau).
            // (subtracted TS since at this point the vehicles' states are already updated)
            let vNextFollower = vehicle.get_speed() + max2(0.0, tauRemainder * vehicle.get_acceleration());
            let vNextLeader = neighLead.0.get_speed() + min2(0.0, tauRemainder * neighLead.0.get_acceleration());
            secureFrontGap = vehicle.get_car_follow_model().get_secure_gap(vehicle, neighLead.0, vNextFollower, vNextLeader, neighLead.0.get_car_follow_model().get_max_decel());
            if neighLead.1 < secureFrontGap * vehicle.get_lane_change_model_2().get_safety_factor() {
                blocked |= blockedByLeader;
            }
        }

        let targetLane = self.myChanger[(self.myCandi as i32 + laneOffset) as usize].lane;
        if blocked == 0 && targetLane.has_pedestrians() {
            let rightSide = vehicle.get_right_side_on_lane_2(targetLane);
            let nextLeader = targetLane.next_blocking(
                vehicle.get_back_position_on_lane_2(),
                rightSide,
                rightSide + vehicle.get_vehicle_type().get_width(),
                (vehicle.get_speed() / vehicle.get_car_follow_model().get_max_decel()).ceil(),
                false,
            );
            if !nextLeader.0.is_null() {
                let brakeGap = vehicle.get_car_follow_model().brake_gap(vehicle.get_speed());
                let gap = nextLeader.1 - vehicle.get_vehicle_type().get_length_with_gap();
                if brakeGap > gap {
                    blocked |= blockedByLeader;
                }
            }
        }

        if !leader.0.is_null() {
            secureOrigFrontGap = vehicle.get_car_follow_model().get_secure_gap(vehicle, leader.0, vehicle.get_speed(), leader.0.get_speed(), leader.0.get_car_follow_model().get_max_decel());
        }
        let _ = secureOrigFrontGap;
        let _ = secureBackGap;
        let _ = secureFrontGap;

        let mut msg = MSAbstractLaneChangeModel_MSLCMessager::new(leader.0, neighLead.0, neighFollow.0);
        let mut state = blocked | vehicle.m().get_lane_change_model().wants_change(
            laneOffset,
            &mut msg,
            blocked,
            leader,
            follower,
            neighLead,
            neighFollow,
            &*targetLane,
            preb,
            Ptr::new(&self.myChanger[c].lastBlocked),
            Ptr::new(&self.myChanger[c].firstBlocked),
        );
        if blocked == 0 && (state & LCA_WANTS_LANECHANGE) != 0 && !neighLead.0.is_null() {
            let egoLane = self.myChanger[c].lane;
            let seen = egoLane.get_length() - vehicle.get_position_on_lane();
            let speed = vehicle.get_speed();
            let dist = vehicle.get_car_follow_model().brake_gap(speed) + vehicle.get_vehicle_type().get_min_gap();
            if seen < dist || egoLane.is_internal() {
                let neighLead2 = targetLane.get_critical_leader(dist, seen, speed, &vehicle);
                if !neighLead2.0.is_null() && neighLead2.0 != neighLead.0 {
                    let secureGap = vehicle.get_car_follow_model().get_secure_gap(vehicle, neighLead2.0, vehicle.get_speed(), neighLead2.0.get_speed(), neighLead2.0.get_car_follow_model().get_max_decel());
                    let secureGap2 = secureGap * vehicle.get_lane_change_model_2().get_safety_factor();
                    if neighLead2.1 < secureGap2 {
                        state |= blockedByLeader;
                    }
                }
            }
        }

        if blocked == 0 && (state & LCA_WANTS_LANECHANGE) != 0 {
            let mut targetZipperDist: f64 = 0.0;
            if laneOffset == 0 {
                targetZipperDist = self.myChanger[c].zipperDist;
            } else if laneOffset == 1 {
                if c + 1 != self.myChanger.len() {
                    targetZipperDist = self.myChanger[c + 1].zipperDist;
                }
            } else if laneOffset == -1 {
                if c > 0 {
                    targetZipperDist = self.myChanger[c - 1].zipperDist;
                }
            }
            if vehicle.unsafe_link_ahead(targetLane, targetZipperDist) {
                state |= blockedByLeader;
            }
        }
        if blocked == 0 && (state & LCA_WANTS_LANECHANGE) != 0 {
            blocked = vehicle.get_lane_change_model_2().check_change_before_committing(vehicle, state);
            state |= blocked;
        }
        // shortcut: MSGlobals::gLaneChangeDuration is not ported and defaults to 0, so the
        // C++ block "gLaneChangeDuration > DELTA_T" (estimateLCDuration and the succLinkSec
        // lookahead) never runs here; port it when MSGlobals and MSLane::succLinkSec are typed
        // shortcut: checkOpened entry-link check (MSLink::opened / approaching info) is not ported; upgrade with MSLink approach API
        state
    }

    /// C++: `bool MSLaneChanger::checkChangeOpposite(MSVehicle *, int, MSLane *, const std::pair<MSVehicle *const, double> &, const std::pair<MSVehicle *const, double> &, const std::pair<MSVehicle *const, double> &, const std::vector<MSVehicle::LaneQ> &)` [virtual, protected]
    // bool
    // MSLaneChanger::checkChangeOpposite(
    //     MSVehicle* vehicle,
    //     int laneOffset,
    //     MSLane* targetLane,
    //     const std::pair<MSVehicle* const, double>& leader,
    //     const std::pair<MSVehicle* const, double>& neighLead,
    //     const std::pair<MSVehicle* const, double>& neighFollow,
    //     const std::vector<MSVehicle::LaneQ>& preb) {
    //     const bool isOpposite = vehicle->getLaneChangeModel().isOpposite();
    //     MSLane* source = vehicle->getMutableLane();
    //     const std::pair<MSVehicle* const, double> follower(nullptr, -1);
    //     int state = checkChange(laneOffset, targetLane, leader, follower, neighLead, neighFollow, preb);
    //     vehicle->getLaneChangeModel().setOwnState(state);
    //     bool changingAllowed = (state & LCA_BLOCKED) == 0;
    //     // change if the vehicle wants to and is allowed to change
    //     if ((state & LCA_WANTS_LANECHANGE) != 0 && changingAllowed
    //             // do not change to the opposite direction for cooperative reasons
    //             && (isOpposite || (state & LCA_COOPERATIVE) == 0)) {
    //         const bool continuous = vehicle->getLaneChangeModel().startLaneChangeManeuver(source, targetLane, laneOffset);
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //         if (DEBUG_COND) {
    //             std::cout << SIMTIME << " changing to opposite veh=" << vehicle->getID() << " dir=" << laneOffset << " opposite=" << Named::getIDSecure(targetLane)
    //                       << " state=" << toString((LaneChangeAction)state) << "\n";
    //         }
    // #endif
    //         if (continuous) {
    //             continueChange(vehicle, myCandi);
    //         }
    //         return true;
    //     }
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //     if (DEBUG_COND) {
    //         std::cout << SIMTIME << " not changing to opposite veh=" << vehicle->getID() << " dir=" << laneOffset
    //                   << " opposite=" << Named::getIDSecure(targetLane) << " state=" << toString((LaneChangeAction)state) << "\n";
    //     }
    // #endif
    //     return false;
    // }
    pub fn check_change_opposite(&mut self, vehicle: Ptr<MSVehicle>, laneOffset: i32, targetLane: Ptr<MSLane>, leader: &(Ptr<MSVehicle>, f64), neighLead: &(Ptr<MSVehicle>, f64), neighFollow: &(Ptr<MSVehicle>, f64), preb: &Vec<MSVehicle_LaneQ>) -> bool {
        (self.vt.check_change_opposite)(self, vehicle, laneOffset, targetLane, leader, neighLead, neighFollow, preb)
    }

    #[allow(unused_variables)]
    pub fn check_change_opposite_base(&mut self, vehicle: Ptr<MSVehicle>, laneOffset: i32, targetLane: Ptr<MSLane>, leader: &(Ptr<MSVehicle>, f64), neighLead: &(Ptr<MSVehicle>, f64), neighFollow: &(Ptr<MSVehicle>, f64), preb: &Vec<MSVehicle_LaneQ>) -> bool {
        let isOpposite = vehicle.get_lane_change_model_2().is_opposite();
        let source = vehicle.get_mutable_lane();
        let follower: (Ptr<MSVehicle>, f64) = (Ptr::null(), -1.0);
        let state = self.check_change(laneOffset, targetLane, &leader, &follower, &neighLead, &neighFollow, preb);
        vehicle.m().get_lane_change_model().set_own_state(state);
        let changingAllowed = (state & LCA_BLOCKED) == 0;
        if (state & LCA_WANTS_LANECHANGE) != 0 && changingAllowed && (isOpposite || (state & LCA_COOPERATIVE) == 0) {
            let continuous = vehicle.m().get_lane_change_model().start_lane_change_maneuver(source, targetLane, laneOffset);
            if continuous {
                let mut from = self.myCandi;
                self.continue_change(vehicle, &mut from);
            }
            return true;
        }
        false
    }

    /// C++: `bool MSLaneChanger::startChange(MSVehicle *, ChangerIt &, int)` [protected]
    // bool
    // MSLaneChanger::startChange(MSVehicle* vehicle, ChangerIt& from, int direction) {
    //     if (vehicle->isRemoteControlled()) {
    //         registerUnchanged(vehicle);
    //         return false;
    //     }
    //     ChangerIt to = from + direction;
    //     // @todo delay entering the target lane until the vehicle intersects it
    //     //       physically (considering lane width and vehicle width)
    //     //if (to->lane->getID() == "beg_1") std::cout << SIMTIME << " startChange to lane=" << to->lane->getID() << " myTmpVehiclesBefore=" << toString(to->lane->myTmpVehicles) << "\n";
    //     const bool continuous = vehicle->getLaneChangeModel().startLaneChangeManeuver(from->lane, to->lane, direction);
    //     if (continuous) {
    //         return continueChange(vehicle, myCandi);
    //     } else {
    //         to->registerHop(vehicle);
    //         to->lane->requireCollisionCheck();
    //         return true;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn start_change(&mut self, vehicle: Ptr<MSVehicle>, from: &mut MSLaneChanger_ChangerIt, direction: i32) -> bool {
        if vehicle.is_remote_controlled() {
            self.register_unchanged(vehicle);
            return false;
        }
        let to = ((*from) as i32 + direction) as usize;
        let from_lane = self.myChanger[(*from)].lane;
        let to_lane = self.myChanger[to].lane;
        // @todo delay entering the target lane until the vehicle intersects it
        //       physically (considering lane width and vehicle width)
        let continuous = vehicle.m().get_lane_change_model().start_lane_change_maneuver(from_lane, to_lane, direction);
        if continuous {
            let mut cur = self.myCandi;
            let ret = self.continue_change(vehicle, &mut cur);
            self.myCandi = cur;
            return ret;
        } else {
            self.myChanger[to].register_hop(vehicle);
            to_lane.m().require_collision_check();
            return true;
        }
    }

    /// C++: `bool MSLaneChanger::continueChange(MSVehicle *, ChangerIt &)` [protected]
    /// continue a lane change maneuver and return whether the vehicle has completely moved onto the new lane (used if gLaneChangeDuration > 0)
    // bool
    // MSLaneChanger::continueChange(MSVehicle* vehicle, ChangerIt& from) {
    //     MSAbstractLaneChangeModel& lcm = vehicle->getLaneChangeModel();
    //     const int direction = lcm.isOpposite() ? -lcm.getLaneChangeDirection() : lcm.getLaneChangeDirection();
    //     const bool pastMidpoint = lcm.updateCompletion(); // computes lcm.mySpeedLat as a side effect
    //     const double speedLat = lcm.isOpposite() ? -lcm.getSpeedLat() : lcm.getSpeedLat();
    //     vehicle->myState.myPosLat += SPEED2DIST(speedLat);
    //     vehicle->myCachedPosition = Position::INVALID;
    //     //std::cout << SIMTIME << " veh=" << vehicle->getID() << " dir=" << direction << " pm=" << pastMidpoint << " speedLat=" << speedLat << " posLat=" << vehicle->myState.myPosLat << "\n";
    //     if (pastMidpoint) {
    //         MSLane* source = myCandi->lane;
    //         MSLane* target = source->getParallelLane(direction);
    //         vehicle->myState.myPosLat -= direction * 0.5 * (source->getWidth() + target->getWidth());
    //         lcm.primaryLaneChanged(source, target, direction);
    //         if (&source->getEdge() == &target->getEdge()) {
    //             ChangerIt to = from + direction;
    //             to->registerHop(vehicle);
    //         }
    //         target->requireCollisionCheck();
    //     } else {
    //         from->registerHop(vehicle);
    //         from->lane->requireCollisionCheck();
    //     }
    //     if (!lcm.isChangingLanes()) {
    //         vehicle->myState.myPosLat = 0;
    //         lcm.endLaneChangeManeuver();
    //     }
    //     lcm.updateShadowLane();
    //     if (lcm.getShadowLane() != nullptr && &lcm.getShadowLane()->getEdge() == &vehicle->getLane()->getEdge()) {
    //         // set as hoppedVeh on the shadow lane so it is found as leader on both lanes
    //         myChanger[lcm.getShadowLane()->getIndex()].hoppedVeh = vehicle;
    //         lcm.getShadowLane()->requireCollisionCheck();
    //     }
    //     vehicle->myAngle = vehicle->computeAngle();
    //     if (lcm.isOpposite()) {
    //         vehicle->myAngle += M_PI;
    //     }
    //
    // #ifdef DEBUG_CONTINUE_CHANGE
    //     if (DEBUG_COND) {
    //         std::cout << SIMTIME
    //                   << " continueChange veh=" << vehicle->getID()
    //                   << " from=" << Named::getIDSecure(from->lane)
    //                   << " dir=" << direction
    //                   << " speedLat=" << speedLat
    //                   << " pastMidpoint=" << pastMidpoint
    //                   << " posLat=" << vehicle->getLateralPositionOnLane()
    //                   << " completion=" << lcm.getLaneChangeCompletion()
    //                   << " shadowLane=" << Named::getIDSecure(lcm.getShadowLane())
    //                   //<< " shadowHopped=" << Named::getIDSecure(shadow->lane)
    //                   << "\n";
    //     }
    // #endif
    //     return pastMidpoint && lcm.getShadowLane() == nullptr;
    // }
    #[allow(unused_variables)]
    pub fn continue_change(&mut self, vehicle: Ptr<MSVehicle>, from: &mut MSLaneChanger_ChangerIt) -> bool {
        let lcm = vehicle.m().get_lane_change_model();
        let direction: i32 = if lcm.is_opposite() { -lcm.get_lane_change_direction() } else { lcm.get_lane_change_direction() };
        let pastMidpoint = lcm.update_completion(); // computes lcm.mySpeedLat as a side effect
        let speedLat = if lcm.is_opposite() { -lcm.get_speed_lat() } else { lcm.get_speed_lat() };
        vehicle.m().myState.myPosLat += speed2dist(speedLat);
        vehicle.m().myCachedPosition.set(Position::INVALID);
        if pastMidpoint {
            let source = self.myChanger[self.myCandi].lane;
            let target = source.get_parallel_lane(direction, true);
            vehicle.m().myState.myPosLat -= (direction as f64) * 0.5 * (source.get_width() + target.get_width());
            vehicle.m().get_lane_change_model().primary_lane_changed(source, target, direction);
            if (source.get_edge() as *const MSEdge) == (target.get_edge() as *const MSEdge) {
                let to = ((*from) as i32 + direction) as usize;
                self.myChanger[to].register_hop(vehicle);
            }
            target.m().require_collision_check();
        } else {
            let f = (*from);
            self.myChanger[f].register_hop(vehicle);
            self.myChanger[f].lane.m().require_collision_check();
        }
        if !vehicle.m().get_lane_change_model().is_changing_lanes() {
            vehicle.m().myState.myPosLat = 0.0;
            vehicle.m().get_lane_change_model().end_lane_change_maneuver(MSMoveReminder_Notification::NOTIFICATION_LANE_CHANGE);
        }
        vehicle.m().get_lane_change_model().update_shadow_lane();
        let shadow = vehicle.m().get_lane_change_model().get_shadow_lane();
        if !shadow.is_null() && (shadow.get_edge() as *const MSEdge) == (vehicle.get_lane().get_edge() as *const MSEdge) {
            // set as hoppedVeh on the shadow lane so it is found as leader on both lanes
            let idx = shadow.get_index() as usize;
            self.myChanger[idx].hoppedVeh = vehicle;
            shadow.m().require_collision_check();
        }
        vehicle.m().myAngle = vehicle.compute_angle();
        if vehicle.m().get_lane_change_model().is_opposite() {
            vehicle.m().myAngle += std::f64::consts::PI;
        }
        return pastMidpoint && vehicle.m().get_lane_change_model().get_shadow_lane().is_null();
    }

    /// C++: `std::pair<MSVehicle *const, double> MSLaneChanger::getRealFollower(const ChangerIt &) const` [const, protected]
    // std::pair<MSVehicle* const, double>
    // MSLaneChanger::getRealFollower(const ChangerIt& target) const {
    //     assert(veh(myCandi) != 0);
    //     MSVehicle* vehicle = veh(myCandi);
    // #ifdef DEBUG_SURROUNDING_VEHICLES
    //     if (DEBUG_COND) {
    //         std::cout << SIMTIME << " veh '" << vehicle->getID() << "' looks for follower on lc-target lane '" << target->lane->getID() << "'." << std::endl;
    //     }
    // #endif
    //     const double candiPos = vehicle->getPositionOnLane();
    //     MSVehicle* neighFollow = nullptr;
    //     if (target != myCandi) {
    //         neighFollow = veh(target);
    //     } else {
    //         // veh(target) would return the ego vehicle so we use its predecessor instead
    //         if (target->lane->myVehicles.size() > 1) {
    //             neighFollow = target->lane->myVehicles[target->lane->myVehicles.size() - 2];
    //         }
    //     }
    //
    // #ifdef DEBUG_SURROUNDING_VEHICLES
    //     if (DEBUG_COND) {
    //         if (neighFollow != 0) {
    //             std::cout << "veh(target) returns '" << neighFollow->getID() << "' at position " << neighFollow->getPositionOnLane() << std::endl;
    //         } else {
    //             std::cout << "veh(target) returns none." << std::endl;
    //         }
    //     }
    // #endif
    //
    //
    // #ifdef DEBUG_SURROUNDING_VEHICLES
    //     if (DEBUG_COND) {
    //         if (getCloserFollower(candiPos, neighFollow, target->hoppedVeh) != neighFollow) {
    //             std::cout << "Hopped vehicle '" << target->hoppedVeh->getID() << "' at position " << target->hoppedVeh->getPositionOnLane() << " is closer." <<  std::endl;
    //         }
    //     }
    // #endif
    //
    //     // check whether the hopped vehicle became the follower
    //     neighFollow = getCloserFollower(candiPos, neighFollow, target->hoppedVeh);
    //
    //
    // #ifdef DEBUG_SURROUNDING_VEHICLES
    //     if (DEBUG_COND) {
    //         MSVehicle* partialBehind = getCloserFollower(candiPos, neighFollow, target->lane->getPartialBehind(vehicle));
    //         if (partialBehind != 0 && partialBehind != neighFollow) {
    //             std::cout << "'Partial behind'-vehicle '" << target->lane->getPartialBehind(vehicle)->getID() << "' at position " << partialBehind->getPositionOnLane() << " is closer." <<  std::endl;
    //         }
    //     }
    // #endif
    //     // or a follower which is partially lapping into the target lane
    //     neighFollow = getCloserFollower(candiPos, neighFollow, target->lane->getPartialBehind(vehicle));
    //
    //     if (neighFollow == nullptr) {
    //         CLeaderDist consecutiveFollower = target->lane->getFollowersOnConsecutive(vehicle, vehicle->getBackPositionOnLane(), true)[0];
    // #ifdef DEBUG_SURROUNDING_VEHICLES
    //         if (DEBUG_COND) {
    //             if (consecutiveFollower.first == 0) {
    //                 std::cout << "no follower found." <<  std::endl;
    //             } else {
    //                 std::cout << "found follower '" << consecutiveFollower.first->getID() << "' on consecutive lanes." <<  std::endl;
    //             }
    //         }
    // #endif
    //         return std::make_pair(const_cast<MSVehicle*>(consecutiveFollower.first), consecutiveFollower.second);
    //     } else {
    // #ifdef DEBUG_SURROUNDING_VEHICLES
    //         if (DEBUG_COND) {
    //             std::cout << "found follower '" << neighFollow->getID() << "'." <<  std::endl;
    //         }
    // #endif
    //         return std::pair<MSVehicle* const, double>(neighFollow,
    //                 vehicle->getPositionOnLane() - vehicle->getVehicleType().getLength() - neighFollow->getPositionOnLane() - neighFollow->getVehicleType().getMinGap());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_real_follower(&self, target: &MSLaneChanger_ChangerIt) -> (Ptr<MSVehicle>, f64) {
        let vehicle = self.veh(self.myCandi);
        let t = *target;
        let candiPos = vehicle.get_position_on_lane();
        let mut neighFollow: Ptr<MSVehicle> = Ptr::null();
        if t != self.myCandi {
            neighFollow = self.veh(t);
        } else {
            // veh(target) would return the ego vehicle so we use its predecessor instead
            let lane = self.myChanger[t].lane;
            if lane.myVehicles.len() > 1 {
                neighFollow = lane.myVehicles[lane.myVehicles.len() - 2];
            }
        }
        // check whether the hopped vehicle became the follower
        neighFollow = MSLaneChanger::get_closer_follower(candiPos, neighFollow, self.myChanger[t].hoppedVeh);
        let targetLane = self.myChanger[t].lane;
        // or a follower which is partially lapping into the target lane
        neighFollow = MSLaneChanger::get_closer_follower(candiPos, neighFollow, targetLane.get_partial_behind(vehicle));
        if neighFollow.is_null() {
            let consecutiveFollower = targetLane.get_followers_on_consecutive(vehicle, vehicle.get_back_position_on_lane_2(), true, -1.0, MSLane_MinorLinkMode::FOLLOW_NEVER, false).index_op(0);
            return (consecutiveFollower.0, consecutiveFollower.1);
        } else {
            return (neighFollow, vehicle.get_position_on_lane() - vehicle.get_vehicle_type().get_length() - neighFollow.get_position_on_lane() - neighFollow.get_vehicle_type().get_min_gap());
        }
    }

    /// C++: `std::pair<MSVehicle *const, double> MSLaneChanger::getRealLeader(const ChangerIt &) const` [const, protected]
    // std::pair<MSVehicle* const, double>
    // MSLaneChanger::getRealLeader(const ChangerIt& target) const {
    //     assert(veh(myCandi) != 0);
    //     MSVehicle* vehicle = veh(myCandi);
    // #ifdef DEBUG_SURROUNDING_VEHICLES
    //     if (DEBUG_COND) {
    //         std::cout << SIMTIME << " veh '" << vehicle->getID() << "' looks for leader on lc-target lane '" << target->lane->getID() << "'." << std::endl;
    //     }
    // #endif
    //     // get the leading vehicle on the lane to change to
    //     MSVehicle* neighLead = target->lead;
    //
    // #ifdef DEBUG_SURROUNDING_VEHICLES
    //     if (DEBUG_COND) {
    //         if (neighLead != 0) {
    //             std::cout << "Considering '" << neighLead->getID() << "' at position " << neighLead->getPositionOnLane() << std::endl;
    //         }
    //     }
    // #endif
    //
    //     // check whether the hopped vehicle became the leader
    //     if (target->hoppedVeh != nullptr) {
    //         double hoppedPos = target->hoppedVeh->getPositionOnLane();
    // #ifdef DEBUG_SURROUNDING_VEHICLES
    //         if (DEBUG_COND) {
    //             std::cout << "Considering hopped vehicle '" << target->hoppedVeh->getID() << "' at position " << hoppedPos << std::endl;
    //         }
    // #endif
    //         if (hoppedPos > vehicle->getPositionOnLane() && (neighLead == nullptr || neighLead->getPositionOnLane() > hoppedPos)) {
    //             neighLead = target->hoppedVeh;
    //             //if (vehicle->getID() == "flow.21") std::cout << SIMTIME << " neighLead=" << Named::getIDSecure(neighLead) << " (422)\n";
    //         }
    //     }
    //     // extra check for shared lane
    //     const bool checkBidi = target->lane->getBidiLane() != nullptr && target->lane->getBidiLane()->getVehicleNumberWithPartials() > 0;
    //     if (neighLead == nullptr || checkBidi) {
    // #ifdef DEBUG_SURROUNDING_VEHICLES
    //         if (DEBUG_COND) {
    //             std::cout << "Looking for leader on consecutive lanes." << std::endl;
    //         }
    // #endif
    //         // There's no leader on the target lane. Look for leaders on consecutive lanes.
    //         // (there might also be partial leaders due to continuous lane changing or bidiLane)
    //         MSLane* targetLane = target->lane;
    //         const double egoBack = vehicle->getBackPositionOnLane();
    //         double leaderBack = targetLane->getLength();
    //         if (neighLead != nullptr) {
    //             leaderBack = neighLead->getBackPositionOnLane(targetLane);
    //         }
    //         for (MSVehicle* pl : targetLane->myPartialVehicles) {
    //             double plBack = pl->getBackPositionOnLane(targetLane);
    //             if (pl->isBidiOn(targetLane)) {
    //                 plBack -= pl->getVehicleType().getLengthWithGap();
    //             }
    //             const double plPos = plBack + pl->getVehicleType().getLength();
    // #ifdef DEBUG_SURROUNDING_VEHICLES
    //             if (DEBUG_COND) {
    //                 std::cout << "   partial=" << pl->getID() << " plBack=" << plBack << " plPos=" << plPos << " leaderBack=" << leaderBack << " egoBack=" << egoBack << "\n";
    //             }
    // #endif
    //             if (plBack < leaderBack && plPos + pl->getVehicleType().getMinGap() >= egoBack) {
    //                 neighLead = pl;
    //                 leaderBack = plBack;
    //             }
    //         }
    //         if (neighLead != nullptr) {
    // #ifdef DEBUG_SURROUNDING_VEHICLES
    //             if (DEBUG_COND) {
    //                 std::cout << "  found leader=" << neighLead->getID() << " (partial)\n";
    //             }
    // #endif
    //             const double gap = leaderBack - vehicle->getPositionOnLane() - vehicle->getVehicleType().getMinGap();
    //             return std::pair<MSVehicle*, double>(neighLead, gap);
    //         }
    //         double seen = myCandi->lane->getLength() - vehicle->getPositionOnLane();
    //         double speed = vehicle->getSpeed();
    //         double dist = vehicle->getCarFollowModel().brakeGap(speed) + vehicle->getVehicleType().getMinGap();
    //         if (target->lane->getBidiLane() != nullptr) {
    //             dist += target->lane->getBidiLane()->getMaximumBrakeDist();
    //         }
    //         // always check for link leaders while on an internal lane
    //         if (seen > dist && !myCandi->lane->isInternal()) {
    // #ifdef DEBUG_SURROUNDING_VEHICLES
    //             if (DEBUG_COND) {
    //                 std::cout << "  found no leader within dist=" << dist << "\n";
    //             }
    // #endif
    //             return std::pair<MSVehicle* const, double>(static_cast<MSVehicle*>(nullptr), -1);
    //         }
    //         const std::vector<MSLane*>& bestLaneConts = vehicle->getBestLanesContinuation(targetLane);
    //
    //         std::pair<MSVehicle* const, double> result = target->lane->getLeaderOnConsecutive(dist, seen, speed, *vehicle, bestLaneConts);
    // #ifdef DEBUG_SURROUNDING_VEHICLES
    //         if (DEBUG_COND) {
    //             std::cout << "  found consecutiveLeader=" << Named::getIDSecure(result.first) << "\n";
    //         }
    // #endif
    //         return result;
    //     } else {
    // #ifdef DEBUG_SURROUNDING_VEHICLES
    //         if (DEBUG_COND) {
    //             std::cout << "  found leader=" << neighLead->getID() << "\n";
    //         }
    // #endif
    //         return std::pair<MSVehicle* const, double>(neighLead, neighLead->getBackPositionOnLane(target->lane) - vehicle->getPositionOnLane() - vehicle->getVehicleType().getMinGap());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_real_leader(&self, target: &MSLaneChanger_ChangerIt) -> (Ptr<MSVehicle>, f64) {
        let vehicle = self.veh(self.myCandi);
        let t = *target;
        // get the leading vehicle on the lane to change to
        let mut neighLead = self.myChanger[t].lead;
        // check whether the hopped vehicle became the leader
        if !self.myChanger[t].hoppedVeh.is_null() {
            let hoppedPos = self.myChanger[t].hoppedVeh.get_position_on_lane();
            if hoppedPos > vehicle.get_position_on_lane() && (neighLead.is_null() || neighLead.get_position_on_lane() > hoppedPos) {
                neighLead = self.myChanger[t].hoppedVeh;
            }
        }
        // extra check for shared lane
        let targetLane = self.myChanger[t].lane;
        let checkBidi = !targetLane.get_bidi_lane().is_null() && targetLane.get_bidi_lane().get_vehicle_number_with_partials() > 0;
        if neighLead.is_null() || checkBidi {
            // There's no leader on the target lane. Look for leaders on consecutive lanes.
            // (there might also be partial leaders due to continuous lane changing or bidiLane)
            let egoBack = vehicle.get_back_position_on_lane_2();
            let mut leaderBack = targetLane.get_length();
            if !neighLead.is_null() {
                leaderBack = neighLead.get_back_position_on_lane(targetLane);
            }
            for pl in targetLane.myPartialVehicles.iter() {
                let pl = *pl;
                let mut plBack = pl.get_back_position_on_lane(targetLane);
                if pl.is_bidi_on(targetLane) {
                    plBack -= pl.get_vehicle_type().get_length_with_gap();
                }
                let plPos = plBack + pl.get_vehicle_type().get_length();
                if plBack < leaderBack && plPos + pl.get_vehicle_type().get_min_gap() >= egoBack {
                    neighLead = pl;
                    leaderBack = plBack;
                }
            }
            if !neighLead.is_null() {
                let gap = leaderBack - vehicle.get_position_on_lane() - vehicle.get_vehicle_type().get_min_gap();
                return (neighLead, gap);
            }
            let egoLane = self.myChanger[self.myCandi].lane;
            let seen = egoLane.get_length() - vehicle.get_position_on_lane();
            let speed = vehicle.get_speed();
            let mut dist = vehicle.get_car_follow_model().brake_gap(speed) + vehicle.get_vehicle_type().get_min_gap();
            if !targetLane.get_bidi_lane().is_null() {
                dist += targetLane.get_bidi_lane().get_maximum_brake_dist();
            }
            // always check for link leaders while on an internal lane
            if seen > dist && !egoLane.is_internal() {
                return (Ptr::null(), -1.0);
            }
            let bestLaneConts = vehicle.get_best_lanes_continuation_2(targetLane);
            let result = targetLane.get_leader_on_consecutive(dist, seen, speed, &vehicle, bestLaneConts, true);
            return result;
        } else {
            return (neighLead, neighLead.get_back_position_on_lane(targetLane) - vehicle.get_position_on_lane() - vehicle.get_vehicle_type().get_min_gap());
        }
    }

    /// C++: `bool MSLaneChanger::mayChange(int) const` [const, protected]
    /// whether changing to the lane in the given direction should be considered
    // bool
    // MSLaneChanger::mayChange(int direction) const {
    //     if (direction == 0) {
    //         return true;
    //     }
    //     if (!myAllowsChanging) {
    //         return false;
    //     }
    //     SUMOVehicleClass svc = veh(myCandi)->getVClass();
    //     if (direction == -1) {
    //         return myCandi->mayChangeRight && (myCandi - 1)->lane->allowsVehicleClass(svc) && myCandi->lane->allowsChangingRight(svc);
    //     } else if (direction == 1) {
    //         return myCandi->mayChangeLeft && (myCandi + 1)->lane->allowsVehicleClass(svc) && myCandi->lane->allowsChangingLeft(svc);
    //     } else {
    //         return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn may_change(&self, direction: i32) -> bool {
        if direction == 0 {
            return true;
        }
        if !self.myAllowsChanging {
            return false;
        }
        let c = self.myCandi;
        let svc = self.veh(c).get_v_class();
        if direction == -1 {
            self.myChanger[c].mayChangeRight && self.myChanger[c - 1].lane.allows_vehicle_class(svc) && self.myChanger[c].lane.allows_changing_right(svc)
        } else if direction == 1 {
            self.myChanger[c].mayChangeLeft && self.myChanger[c + 1].lane.allows_vehicle_class(svc) && self.myChanger[c].lane.allows_changing_left(svc)
        } else {
            false
        }
    }

    /// C++: `MSVehicle * MSLaneChanger::getCloserFollower(const double, MSVehicle *, MSVehicle *)` [static, protected]
    /// return the closer follower of ego
    // MSVehicle*
    // MSLaneChanger::getCloserFollower(const double maxPos, MSVehicle* follow1, MSVehicle* follow2) {
    //     if (follow1 == nullptr || follow1->getPositionOnLane() > maxPos) {
    //         return follow2;
    //     } else if (follow2 == nullptr || follow2->getPositionOnLane() > maxPos) {
    //         return follow1;
    //     } else {
    //         if (follow1->getPositionOnLane() > follow2->getPositionOnLane()) {
    //             return follow1;
    //         } else {
    //             return follow2;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_closer_follower(maxPos: f64, follow1: Ptr<MSVehicle>, follow2: Ptr<MSVehicle>) -> Ptr<MSVehicle> {
        if follow1.is_null() || follow1.get_position_on_lane() > maxPos {
            follow2
        } else if follow2.is_null() || follow2.get_position_on_lane() > maxPos {
            follow1
        } else if follow1.get_position_on_lane() > follow2.get_position_on_lane() {
            follow1
        } else {
            follow2
        }
    }

    /// C++: `void MSLaneChanger::computeOvertakingTime(const MSVehicle *, double, const MSVehicle *, double, double &, double &)` [static, protected]
    /// Compute the time and space required for overtaking the given leader
    // void
    // MSLaneChanger::computeOvertakingTime(const MSVehicle* vehicle, double vMax, const MSVehicle* leader, double gap, double& timeToOvertake, double& spaceToOvertake) {
    //     // Assumptions:
    //     // - leader maintains the current speed
    //     // - vehicle merges with maxSpeed ahead of leader
    //     // XXX affected by ticket #860 (the formula is invalid for the current position update rule)
    //
    //     // first compute these values for the case where vehicle is accelerating
    //     // without upper bound on speed
    //     const double v = vehicle->getSpeed();
    //     const double u = leader->getAcceleration() > 0 ? leader->getLane()->getVehicleMaxSpeed(leader) : leader->getSpeed();
    //     const double a = vehicle->getCarFollowModel().getMaxAccel();
    //     const double d = vehicle->getCarFollowModel().getMaxDecel();
    //     const double g = MAX2(0.0, (
    //                               // drive up to the rear of leader
    //                               gap + vehicle->getVehicleType().getMinGap()
    //                               // drive head-to-head with the leader
    //                               + leader->getVehicleType().getLengthWithGap()
    //                               // drive past the leader
    //                               + vehicle->getVehicleType().getLength()
    //                               // allow for safe gap between leader and vehicle
    //                               + leader->getCarFollowModel().getSecureGap(leader, vehicle, u, vMax, d))
    //                           // time to move between lanes
    //                           + (MSGlobals::gSublane ? vMax * vehicle->getLane()->getWidth() / vehicle->getVehicleType().getMaxSpeedLat() : 0));
    //     const double sign = -1; // XXX recheck
    //     // v*t + t*t*a*0.5 = g + u*t
    //     // solve t
    //     // t = ((u - v - (((((2.0*(u - v))**2.0) + (8.0*a*g))**(1.0/2.0))*sign/2.0))/a)
    //     double t = (u - v - sqrt(4 * (u - v) * (u - v) + 8 * a * g) * sign * 0.5) / a;
    // #ifdef DEBUG_CHANGE_OPPOSITE_OVERTAKINGTIME
    //     if (DEBUG_COND) {
    //         std::cout << " computeOvertakingTime v=" << v << " vMax=" << vMax << " u=" << u << " a=" << a << " d=" << d << " gap=" << gap << " g=" << g << " t=" << t
    //                   << " distEgo=" << v* t + t* t* a * 0.5 << " distLead=" << g + u* t
    //                   << "\n";
    //     }
    // #endif
    //     assert(t >= 0);
    //     if (vMax <= u) {
    //         // do not try to overtake faster leader
    //         timeToOvertake = std::numeric_limits<double>::max();
    //         spaceToOvertake = std::numeric_limits<double>::max();
    //         return;
    //     }
    //
    //     // allow for a safety time gap
    //     t += OPPOSITE_OVERTAKING_SAFE_TIMEGAP;
    //     // round to multiples of step length (TS)
    //     if (u > 0) {
    //         t = ceil(t / TS) * TS;
    //     }
    //
    //     /// XXX ignore speed limit when overtaking through the opposite lane?
    //     const double timeToMaxSpeed = (vMax - v) / a;
    //
    // #ifdef DEBUG_CHANGE_OPPOSITE_OVERTAKINGTIME
    //     if (DEBUG_COND) {
    //         std::cout << "   t=" << t << "  tvMax=" << timeToMaxSpeed << "\n";
    //     }
    // #endif
    //     if (t <= timeToMaxSpeed) {
    //         timeToOvertake = t;
    //         spaceToOvertake = v * t + t * t * a * 0.5;
    // #ifdef DEBUG_CHANGE_OPPOSITE_OVERTAKINGTIME
    //         if (DEBUG_COND) {
    //             std::cout << "    sto=" << spaceToOvertake << "\n";
    //         }
    // #endif
    //     } else {
    //         // space until max speed is reached
    //         const double s = v * timeToMaxSpeed + timeToMaxSpeed * timeToMaxSpeed * a * 0.5;
    //         const double m = timeToMaxSpeed;
    //         // s + (t-m) * vMax = g + u*t
    //         // solve t
    //         t = (g - s + m * vMax) / (vMax - u);
    //         if (t < 0) {
    //             // cannot overtake in time
    // #ifdef DEBUG_CHANGE_OPPOSITE_OVERTAKINGTIME
    //             if (DEBUG_COND) {
    //                 std::cout << "     t2=" << t << "\n";
    //             }
    // #endif
    //             timeToOvertake = std::numeric_limits<double>::max();
    //             spaceToOvertake = std::numeric_limits<double>::max();
    //             return;
    //         } else {
    //             // allow for a safety time gap
    //             t += OPPOSITE_OVERTAKING_SAFE_TIMEGAP;
    //             // round to multiples of step length (TS)
    //             if (u > 0) {
    //                 t = ceil(t / TS) * TS;
    //             }
    //             timeToOvertake = t;
    //             spaceToOvertake = s + (t - m) * vMax;
    // #ifdef DEBUG_CHANGE_OPPOSITE_OVERTAKINGTIME
    //             if (DEBUG_COND) {
    //                 std::cout << "     t2=" << t << " s=" << s << " sto=" << spaceToOvertake << " m=" << m << "\n";
    //             }
    // #endif
    //         }
    //     }
    //     const double safetyFactor = OPPOSITE_OVERTAKING_SAFETY_FACTOR * vehicle->getLaneChangeModel().getOppositeSafetyFactor();
    //     timeToOvertake *= safetyFactor;
    //     if (STEPS2TIME(leader->getStopDuration()) < timeToOvertake) {
    //         spaceToOvertake *= safetyFactor;
    //     }
    //     double frac = fmod(timeToOvertake, TS);
    //     if (frac > 0) {
    //         // round to full sim step
    //         timeToOvertake += TS - frac;
    //     }
    // #ifdef DEBUG_CHANGE_OPPOSITE_OVERTAKINGTIME
    //     if (DEBUG_COND) {
    //         if (safetyFactor != 1) {
    //             std::cout << "    applying safetyFactor=" << safetyFactor
    //                       << " leaderStopTime=" << STEPS2TIME(leader->getStopDuration())
    //                       << " tto=" << timeToOvertake << " sto=" << spaceToOvertake << "\n";
    //         }
    //     }
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn compute_overtaking_time(vehicle: Ptr<MSVehicle>, vMax: f64, leader: Ptr<MSVehicle>, gap: f64, timeToOvertake: &mut f64, spaceToOvertake: &mut f64) {
        let v = vehicle.get_speed();
        let u = if leader.get_acceleration() > 0.0 { leader.get_lane().get_vehicle_max_speed(traffic_object(leader)) } else { leader.get_speed() };
        let a = vehicle.get_car_follow_model().get_max_accel();
        let d = vehicle.get_car_follow_model().get_max_decel();
        let g = max2(0.0, (
            gap + vehicle.get_vehicle_type().get_min_gap()
            + leader.get_vehicle_type().get_length_with_gap()
            + vehicle.get_vehicle_type().get_length()
            + leader.get_car_follow_model().get_secure_gap(leader, vehicle, u, vMax, d))
            + if crate::microsim::ms_globals::sublane() { vMax * vehicle.get_lane().get_width() / vehicle.get_vehicle_type().get_max_speed_lat() } else { 0.0 });
        let sign: f64 = -1.0; // XXX recheck
        let mut t = (u - v - (4.0 * (u - v) * (u - v) + 8.0 * a * g).sqrt() * sign * 0.5) / a;
        debug_assert!(t >= 0.0);
        if vMax <= u {
            *timeToOvertake = f64::MAX;
            *spaceToOvertake = f64::MAX;
            return;
        }
        t += OPPOSITE_OVERTAKING_SAFE_TIMEGAP;
        if u > 0.0 {
            t = (t / ts()).ceil() * ts();
        }
        let timeToMaxSpeed = (vMax - v) / a;
        if t <= timeToMaxSpeed {
            *timeToOvertake = t;
            *spaceToOvertake = v * t + t * t * a * 0.5;
        } else {
            let s = v * timeToMaxSpeed + timeToMaxSpeed * timeToMaxSpeed * a * 0.5;
            let m = timeToMaxSpeed;
            t = (g - s + m * vMax) / (vMax - u);
            if t < 0.0 {
                *timeToOvertake = f64::MAX;
                *spaceToOvertake = f64::MAX;
                return;
            } else {
                t += OPPOSITE_OVERTAKING_SAFE_TIMEGAP;
                if u > 0.0 {
                    t = (t / ts()).ceil() * ts();
                }
                *timeToOvertake = t;
                *spaceToOvertake = s + (t - m) * vMax;
            }
        }
        let safetyFactor = OPPOSITE_OVERTAKING_SAFETY_FACTOR * vehicle.get_lane_change_model_2().get_opposite_safety_factor();
        *timeToOvertake *= safetyFactor;
        if steps2time(leader.get_stop_duration()) < *timeToOvertake {
            *spaceToOvertake *= safetyFactor;
        }
        let frac = *timeToOvertake % ts();
        if frac > 0.0 {
            *timeToOvertake += ts() - frac;
        }
    }

    /// C++: `std::pair<MSVehicle *, double> MSLaneChanger::getColumnleader(double &, MSVehicle *, std::pair<MSVehicle *, double>, double)` [static, protected]
    /// return leader vehicle that is to be overtaken
    // std::pair<MSVehicle*, double>
    // MSLaneChanger::getColumnleader(double& maxSpace, MSVehicle* vehicle, std::pair<MSVehicle*, double> leader, double maxLookAhead) {
    //     assert(leader.first != 0);
    //     const bool isEmergency = vehicle->getVClass() == SVC_EMERGENCY;
    //     const MSLane* source = vehicle->getLane();
    //     // find a leader vehicle with sufficient space ahead for merging back
    //     const double overtakingSpeed = source->getVehicleMaxSpeed(vehicle); // just a guess
    //     const double mergeBrakeGap = vehicle->getCarFollowModel().brakeGap(overtakingSpeed);
    //     std::pair<MSVehicle*, double> columnLeader = leader;
    //     double egoGap = leader.second;
    //     bool foundSpaceAhead = false;
    //     double seen = leader.second + leader.first->getVehicleType().getLengthWithGap();
    //     std::vector<MSLane*> conts = vehicle->getBestLanesContinuation();
    //     if (maxLookAhead == std::numeric_limits<double>::max()) {
    //         maxLookAhead = (isEmergency
    //                         ? OPPOSITE_OVERTAKING_MAX_LOOKAHEAD_EMERGENCY
    //                         : OPPOSITE_OVERTAKING_MAX_LOOKAHEAD);
    //         maxLookAhead = MAX2(maxLookAhead, mergeBrakeGap + 10
    //                             + vehicle->getVehicleType().getLengthWithGap()
    //                             + leader.first->getVehicleType().getLengthWithGap());
    //     }
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //     if (DEBUG_COND) {
    //         std::cout << " getColumnleader vehicle=" << vehicle->getID() << " leader=" << leader.first->getID() << " gap=" << leader.second << " maxLookAhead=" << maxLookAhead << "\n";
    //     }
    // #endif
    //     const double safetyFactor = OPPOSITE_OVERTAKING_SAFETY_FACTOR * vehicle->getLaneChangeModel().getOppositeSafetyFactor();
    //     while (!foundSpaceAhead) {
    //         const double requiredSpaceAfterLeader = (columnLeader.first->getCarFollowModel().getSecureGap(
    //                 columnLeader.first, vehicle,
    //                 columnLeader.first->getSpeed(), overtakingSpeed, vehicle->getCarFollowModel().getMaxDecel())
    //                                                 + columnLeader.first->getVehicleType().getMinGap()
    //                                                 + vehicle->getVehicleType().getLengthWithGap());
    //
    //
    //         // all leader vehicles on the current laneChanger edge are already moved into MSLane::myTmpVehicles
    //         const bool checkTmpVehicles = (&columnLeader.first->getLane()->getEdge() == &source->getEdge());
    //         double searchStart = columnLeader.first->getPositionOnLane();
    //         std::pair<MSVehicle*, double> leadLead = columnLeader.first->getLane()->getLeader(
    //                     columnLeader.first, searchStart, conts, requiredSpaceAfterLeader + mergeBrakeGap,
    //                     checkTmpVehicles);
    //         std::set<MSVehicle*> seenLeaders;
    //         while (leadLead.first != nullptr && leadLead.first->getLaneChangeModel().isOpposite()) {
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //             if (DEBUG_COND) {
    //                 std::cout << "   skipping opposite leadLead=" << leadLead.first->getID() << " gap=" << leadLead.second << "\n";
    //             }
    // #endif
    //             if (leadLead.second + seen > maxLookAhead || seenLeaders.count(leadLead.first) > 0) {
    //                 leadLead.first = nullptr;
    //                 break;
    //             }
    //             seenLeaders.insert(leadLead.first);
    //             // found via shadow lane, skip it
    //             const double searchStart2 = searchStart + MAX2(0.0, leadLead.second) + leadLead.first->getVehicleType().getLengthWithGap();
    //             leadLead = columnLeader.first->getLane()->getLeader(
    //                            columnLeader.first, searchStart2, conts, requiredSpaceAfterLeader + mergeBrakeGap,
    //                            checkTmpVehicles);
    //             leadLead.second += (searchStart2 - searchStart);
    //         }
    //         if (leadLead.first == nullptr) {
    //             double availableSpace = columnLeader.first->getLane()->getLength() - columnLeader.first->getPositionOnLane();
    //             double requiredSpace = safetyFactor * requiredSpaceAfterLeader;
    //             if (!columnLeader.first->isStopped()) {
    //                 // if the leader is stopped we can trade space for time
    //                 requiredSpace += safetyFactor * mergeBrakeGap;
    //             }
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //             if (DEBUG_COND) {
    //                 std::cout << "   no direct leader found after columnLeader " << columnLeader.first->getID()
    //                           << " availableSpace=" << availableSpace
    //                           << " reqAfterLeader=" << requiredSpaceAfterLeader
    //                           << " ovSpeed=" << overtakingSpeed
    //                           << " reqBGap=" << mergeBrakeGap
    //                           << " reqMin=" << requiredSpace / safetyFactor
    //                           << " req=" << requiredSpace
    //                           << "\n";
    //             }
    // #endif
    //             if (availableSpace > requiredSpace) {
    //                 foundSpaceAhead = true;
    //             } else {
    //                 // maybe the columnleader is stopped before a junction or takes a different turn.
    //                 // try to find another columnleader on successive lanes
    //                 bool contsEnd = false;
    //                 const MSLane* next = getLaneAfter(columnLeader.first->getLane(), conts, isEmergency, contsEnd);
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //                 if (DEBUG_COND) {
    //                     std::cout << "   look for another leader on lane " << Named::getIDSecure(next) << "\n";
    //                 }
    // #endif
    //                 while (next != nullptr && seen < maxLookAhead) {
    //                     seen += next->getLength();
    //                     MSVehicle* cand = next->getLastAnyVehicle();
    //                     if (cand == nullptr) {
    //                         availableSpace += next->getLength();
    //                         if (availableSpace > requiredSpace) {
    //                             foundSpaceAhead = true;
    //                             break;
    //                         }
    //                         next = getLaneAfter(next, conts, isEmergency, contsEnd);
    //                     } else {
    //                         availableSpace += cand->getBackPositionOnLane();
    //                         if (availableSpace > requiredSpace) {
    //                             foundSpaceAhead = true;
    //                             break;
    //                         } else {
    //                             return getColumnleader(maxSpace, vehicle, std::make_pair(cand, availableSpace + cand->getPositionOnLane()), maxLookAhead - seen);
    //                         }
    //                     }
    //                 }
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //                 if (DEBUG_COND) {
    //                     std::cout << "      foundSpaceAhead=" << foundSpaceAhead << " availableSpace=" << availableSpace << " next=" << Named::getIDSecure(next) << " contsEnd=" << contsEnd << " conts=" << toString(conts) << "\n";
    //                 }
    // #endif
    //                 if (!foundSpaceAhead && contsEnd) {
    //                     foundSpaceAhead = true;
    //                     availableSpace = requiredSpace;
    //                 }
    //                 if (!foundSpaceAhead) {
    //                     return std::make_pair(nullptr, -1);
    //                 }
    //             }
    //             maxSpace = egoGap + columnLeader.first->getVehicleType().getLength() + availableSpace;
    //         } else {
    //             const double sGap = vehicle->getCarFollowModel().getSecureGap(vehicle, leadLead.first,
    //                                 overtakingSpeed, leadLead.first->getSpeed(), leadLead.first->getCarFollowModel().getMaxDecel());
    //             double requiredSpace = safetyFactor * requiredSpaceAfterLeader;
    //             if (!columnLeader.first->isStopped()) {
    //                 // if the leader is stopped we can trade space for time
    //                 requiredSpace += safetyFactor * sGap;
    //             }
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //             if (DEBUG_COND) {
    //                 std::cout << "   leader's leader " << leadLead.first->getID() << " space=" << leadLead.second
    //                           << " reqAfterLeader=" << requiredSpaceAfterLeader
    //                           << " ovSpeed=" << overtakingSpeed
    //                           << " reqSGap=" << sGap
    //                           << " reqMin=" << requiredSpace / safetyFactor
    //                           << " req=" << requiredSpace
    //                           << "\n";
    //             }
    // #endif
    //             if (leadLead.second > requiredSpace) {
    //                 foundSpaceAhead = true;
    //                 maxSpace = egoGap + columnLeader.first->getVehicleType().getLength() + leadLead.second;
    //             } else {
    //
    //                 if (leadLead.second < 0) {
    //                     // must be a junction leader or some other dangerous situation
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //                     if (DEBUG_COND) {
    //                         std::cout << "   leader's leader " << leadLead.first->getID() << " gap=" << leadLead.second << " is junction leader (aborting)\n";
    //                     }
    // #endif
    //                     return std::make_pair(nullptr, -1);
    //                 }
    //
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //                 if (DEBUG_COND) {
    //                     std::cout << "   not enough space after columnLeader=" << columnLeader.first->getID() << " required=" << requiredSpace << "\n";
    //                 }
    // #endif
    //                 seen += MAX2(0., leadLead.second) + leadLead.first->getVehicleType().getLengthWithGap();
    //                 if (seen > maxLookAhead) {
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //                     if (DEBUG_COND) {
    //                         std::cout << "   cannot changeOpposite due to insufficient free space after columnLeader (seen=" << seen << " columnLeader=" << columnLeader.first->getID() << ")\n";
    //                     }
    // #endif
    //                     return std::make_pair(nullptr, -1);
    //                 }
    //                 // see if merging after leadLead is possible
    //                 egoGap += columnLeader.first->getVehicleType().getLengthWithGap() + leadLead.second;
    //                 columnLeader = leadLead;
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //                 if (DEBUG_COND) {
    //                     std::cout << "   new columnLeader=" << columnLeader.first->getID() << "\n";
    //                 }
    // #endif
    //             }
    //         }
    //     }
    //     columnLeader.second = egoGap;
    //     return columnLeader;
    // }
    #[allow(unused_variables)]
    pub fn get_columnleader(maxSpace: &mut f64, vehicle: Ptr<MSVehicle>, leader: (Ptr<MSVehicle>, f64), maxLookAhead: f64) -> (Ptr<MSVehicle>, f64) {
        debug_assert!(!leader.0.is_null());
        let mut maxLookAhead = maxLookAhead;
        let isEmergency = vehicle.get_v_class() == SVC_EMERGENCY;
        let source = vehicle.get_lane();
        let overtakingSpeed = source.get_vehicle_max_speed(traffic_object(vehicle)); // just a guess
        let mergeBrakeGap = vehicle.get_car_follow_model().brake_gap(overtakingSpeed);
        let mut columnLeader = leader;
        let mut egoGap = leader.1;
        let mut foundSpaceAhead = false;
        let mut seen = leader.1 + leader.0.get_vehicle_type().get_length_with_gap();
        let conts: Vec<Ptr<MSLane>> = vehicle.get_best_lanes_continuation().clone();
        if maxLookAhead == f64::MAX {
            maxLookAhead = if isEmergency { OPPOSITE_OVERTAKING_MAX_LOOKAHEAD_EMERGENCY } else { OPPOSITE_OVERTAKING_MAX_LOOKAHEAD };
            maxLookAhead = max2(maxLookAhead, mergeBrakeGap + 10.0
                + vehicle.get_vehicle_type().get_length_with_gap()
                + leader.0.get_vehicle_type().get_length_with_gap());
        }
        let safetyFactor = OPPOSITE_OVERTAKING_SAFETY_FACTOR * vehicle.get_lane_change_model_2().get_opposite_safety_factor();
        while !foundSpaceAhead {
            let requiredSpaceAfterLeader = columnLeader.0.get_car_follow_model().get_secure_gap(
                columnLeader.0, vehicle,
                columnLeader.0.get_speed(), overtakingSpeed, vehicle.get_car_follow_model().get_max_decel())
                + columnLeader.0.get_vehicle_type().get_min_gap()
                + vehicle.get_vehicle_type().get_length_with_gap();
            let checkTmpVehicles = (columnLeader.0.get_lane().get_edge() as *const MSEdge) == (source.get_edge() as *const MSEdge);
            let searchStart = columnLeader.0.get_position_on_lane();
            let mut leadLead = columnLeader.0.get_lane().get_leader(
                columnLeader.0, searchStart, &conts, requiredSpaceAfterLeader + mergeBrakeGap,
                checkTmpVehicles);
            let mut seenLeaders: BTreeSet<Ptr<MSVehicle>> = BTreeSet::new();
            while !leadLead.0.is_null() && leadLead.0.get_lane_change_model_2().is_opposite() {
                if leadLead.1 + seen > maxLookAhead || seenLeaders.contains(&leadLead.0) {
                    leadLead.0 = Ptr::null();
                    break;
                }
                seenLeaders.insert(leadLead.0);
                let searchStart2 = searchStart + max2(0.0, leadLead.1) + leadLead.0.get_vehicle_type().get_length_with_gap();
                leadLead = columnLeader.0.get_lane().get_leader(
                    columnLeader.0, searchStart2, &conts, requiredSpaceAfterLeader + mergeBrakeGap,
                    checkTmpVehicles);
                leadLead.1 += searchStart2 - searchStart;
            }
            if leadLead.0.is_null() {
                let mut availableSpace = columnLeader.0.get_lane().get_length() - columnLeader.0.get_position_on_lane();
                let mut requiredSpace = safetyFactor * requiredSpaceAfterLeader;
                if !columnLeader.0.is_stopped() {
                    requiredSpace += safetyFactor * mergeBrakeGap;
                }
                if availableSpace > requiredSpace {
                    foundSpaceAhead = true;
                } else {
                    let mut contsEnd = false;
                    let mut next = MSLaneChanger::get_lane_after(columnLeader.0.get_lane(), &conts, isEmergency, &mut contsEnd);
                    while !next.is_null() && seen < maxLookAhead {
                        seen += next.get_length();
                        let cand = next.get_last_any_vehicle();
                        if cand.is_null() {
                            availableSpace += next.get_length();
                            if availableSpace > requiredSpace {
                                foundSpaceAhead = true;
                                break;
                            }
                            next = MSLaneChanger::get_lane_after(next, &conts, isEmergency, &mut contsEnd);
                        } else {
                            availableSpace += cand.get_back_position_on_lane_2();
                            if availableSpace > requiredSpace {
                                foundSpaceAhead = true;
                                break;
                            } else {
                                return MSLaneChanger::get_columnleader(maxSpace, vehicle, (cand, availableSpace + cand.get_position_on_lane()), maxLookAhead - seen);
                            }
                        }
                    }
                    if !foundSpaceAhead && contsEnd {
                        foundSpaceAhead = true;
                        availableSpace = requiredSpace;
                    }
                    if !foundSpaceAhead {
                        return (Ptr::null(), -1.0);
                    }
                }
                *maxSpace = egoGap + columnLeader.0.get_vehicle_type().get_length() + availableSpace;
            } else {
                let sGap = vehicle.get_car_follow_model().get_secure_gap(vehicle, leadLead.0,
                    overtakingSpeed, leadLead.0.get_speed(), leadLead.0.get_car_follow_model().get_max_decel());
                let mut requiredSpace = safetyFactor * requiredSpaceAfterLeader;
                if !columnLeader.0.is_stopped() {
                    requiredSpace += safetyFactor * sGap;
                }
                if leadLead.1 > requiredSpace {
                    foundSpaceAhead = true;
                    *maxSpace = egoGap + columnLeader.0.get_vehicle_type().get_length() + leadLead.1;
                } else {
                    if leadLead.1 < 0.0 {
                        return (Ptr::null(), -1.0);
                    }
                    seen += max2(0.0, leadLead.1) + leadLead.0.get_vehicle_type().get_length_with_gap();
                    if seen > maxLookAhead {
                        return (Ptr::null(), -1.0);
                    }
                    egoGap += columnLeader.0.get_vehicle_type().get_length_with_gap() + leadLead.1;
                    columnLeader = leadLead;
                }
            }
        }
        columnLeader.1 = egoGap;
        columnLeader
    }

    /// C++: `const MSLane * MSLaneChanger::getLaneAfter(const MSLane *, const std::vector<MSLane *> &, bool, bool &)` [static, protected]
    /// return the next lane in conts beyond lane or nullptr
    // const MSLane*
    // MSLaneChanger::getLaneAfter(const MSLane* lane, const std::vector<MSLane*>& conts, bool allowMinor, bool& contsEnd) {
    //     for (auto it = conts.begin(); it != conts.end(); ++it) {
    //         if (*it == lane) {
    //             if (it + 1 != conts.end()) {
    //                 // abort on minor link
    //                 const MSLane* next = *(it + 1);
    //                 const MSLink* link = lane->getLinkTo(next);
    //                 if (link == nullptr || (!allowMinor && !link->havePriority())) {
    //                     return nullptr;
    //                 }
    //                 return next;
    //             } else {
    //                 contsEnd = true;
    //                 return nullptr;
    //             }
    //         }
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_lane_after(lane: Ptr<MSLane>, conts: &Vec<Ptr<MSLane>>, allowMinor: bool, contsEnd: &mut bool) -> Ptr<MSLane> {
        for it in 0..conts.len() {
            if conts[it] == lane {
                if it + 1 != conts.len() {
                    let next = conts[it + 1];
                    let link = lane.get_link_to(next);
                    if link.is_null() || (!allowMinor && !link.have_priority()) {
                        return Ptr::null();
                    }
                    return next;
                } else {
                    *contsEnd = true;
                    return Ptr::null();
                }
            }
        }
        Ptr::null()
    }

    /// C++: `bool MSLaneChanger::hasOppositeStop(MSVehicle *)` [static, protected]
    /// whether vehicle has an opposite-direction stop within relevant range
    // bool
    // MSLaneChanger::hasOppositeStop(MSVehicle* vehicle) {
    //     if (vehicle->hasStops()) {
    //         const MSStop& stop = vehicle->getNextStop();
    //         if (stop.isOpposite && vehicle->nextStopDist() < OPPOSITE_OVERTAKING_MAX_LOOKAHEAD) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn has_opposite_stop(vehicle: Ptr<MSVehicle>) -> bool {
        if vehicle.has_stops() {
            let stop = vehicle.get_next_stop();
            if stop.isOpposite && vehicle.next_stop_dist() < OPPOSITE_OVERTAKING_MAX_LOOKAHEAD {
                return true;
            }
        }
        false
    }

    /// C++: `bool MSLaneChanger::checkOppositeStop(MSVehicle *, const MSLane *, const MSLane *, std::pair<MSVehicle *, double>)` [protected]
    /// decide whether to change (back or forth) for an opposite stop
    // bool
    // MSLaneChanger::checkOppositeStop(MSVehicle* vehicle, const MSLane* oncomingLane, const MSLane* opposite, std::pair<MSVehicle*, double> leader) {
    //     const bool isOpposite = vehicle->getLaneChangeModel().isOpposite();
    //     double vMax = vehicle->getLane()->getVehicleMaxSpeed(vehicle);
    //     std::pair<MSVehicle*, double> neighLead(nullptr, -1);
    //     std::pair<MSVehicle*, double> oncoming(nullptr, -1);
    //     const std::vector<MSVehicle::LaneQ> preb = getBestLanesOpposite(vehicle, vehicle->getNextStop().lane, -1);
    //     const int laneIndex = vehicle->getLaneChangeModel().getNormalizedLaneIndex();
    //     const int bestOffset = preb[laneIndex].bestLaneOffset;
    //     //std::cout << SIMTIME << " veh=" << vehicle->getID() << " laneIndex=" << laneIndex << " bestOffset=" << bestOffset << " target=" << target->getID() << "\n";
    //
    //     // compute safety constraints (assume vehicle is safe once stop is reached)
    //     const double spaceToStop = vehicle->nextStopDist();
    //     const double timeToStopForward = spaceToStop / MAX2(vehicle->getSpeed(), vehicle->getCarFollowModel().getMaxAccel());
    //     const double timeToStopLateral = (MSGlobals::gLaneChangeDuration > 0
    //                                       ? STEPS2TIME(MSGlobals::gLaneChangeDuration) * bestOffset
    //                                       : (MSGlobals::gLateralResolution > 0
    //                                          ? bestOffset * SUMO_const_laneWidth / vehicle->getVehicleType().getMaxSpeedLat()
    //                                          : 0.));
    //     const double timeToStop = MAX2(timeToStopForward, timeToStopLateral);
    //     if (!isOpposite) {
    //         // we keep neighLead distinct from oncoming because it determines blocking on the neigh lane
    //         // but also look for an oncoming leader to compute safety constraint
    //         const double searchDist = timeToStop * oncomingLane->getSpeedLimit() * 2 + spaceToStop;
    //         neighLead = oncomingLane->getOppositeLeader(vehicle, searchDist, true);
    //         oncoming = getOncomingVehicle(oncomingLane, neighLead, searchDist, vMax);
    //     } else {
    //         double searchDist = OPPOSITE_OVERTAKING_ONCOMING_LOOKAHEAD;
    //         oncoming = oncomingLane->getOppositeLeader(vehicle, searchDist, true);
    //         oncoming = getOncomingVehicle(oncomingLane, oncoming, searchDist, vMax);
    //     }
    //     double oncomingSpeed;
    //     const double surplusGap = computeSurplusGap(vehicle, opposite, oncoming, timeToStop, spaceToStop, oncomingSpeed);
    //     if (!isOpposite && surplusGap < 0) {
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //         if (DEBUG_COND) {
    //             std::cout << "   cannot changeOppositeStop due to dangerous oncoming spaceToStop=" << spaceToStop
    //                       << " timeToStopForward=" << timeToStopForward << " timeToStopLateral=" << timeToStopLateral << " surplusGap=" << surplusGap << "\n";
    //         }
    // #endif
    //         return false;
    //     }
    //
    //     if (bestOffset > 0) {
    //         MSLane* const target = preb[laneIndex + 1].lane;
    //         neighLead = target->getOppositeLeader(vehicle, OPPOSITE_OVERTAKING_MAX_LOOKAHEAD, true);
    //         std::pair<MSVehicle* const, double> neighFollow = target->getOppositeFollower(vehicle);
    //         return checkChangeOpposite(vehicle, 1, target, leader, neighLead, neighFollow, preb);
    //     } else {
    //         // return prematurely (due to foe?)
    //         //return checkChangeOpposite(vehicle, -1, target, leader, neighLead, neighFollow, preb);
    //         return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn check_opposite_stop(&mut self, vehicle: Ptr<MSVehicle>, oncomingLane: Ptr<MSLane>, opposite: Ptr<MSLane>, leader: (Ptr<MSVehicle>, f64)) -> bool {
        let isOpposite = vehicle.get_lane_change_model_2().is_opposite();
        let mut vMax = vehicle.get_lane().get_vehicle_max_speed(traffic_object(vehicle));
        let mut neighLead: (Ptr<MSVehicle>, f64) = (Ptr::null(), -1.0);
        let mut oncoming: (Ptr<MSVehicle>, f64) = (Ptr::null(), -1.0);
        let preb = MSLaneChanger::get_best_lanes_opposite(vehicle, vehicle.get_next_stop().lane, -1.0);
        let laneIndex = vehicle.m().get_lane_change_model().get_normalized_lane_index();
        let bestOffset = preb[laneIndex as usize].bestLaneOffset;
        let spaceToStop = vehicle.next_stop_dist();
        let timeToStopForward = spaceToStop / max2(vehicle.get_speed(), vehicle.get_car_follow_model().get_max_accel());
        let timeToStopLateral = if crate::microsim::ms_globals::lane_change_duration() > 0 {
            utils::common::time::steps2time(crate::microsim::ms_globals::lane_change_duration()) * bestOffset as f64
        } else if crate::microsim::ms_globals::lateral_resolution() > 0. {
            bestOffset as f64 * utils::common::std_defs::SUMO_CONST_LANE_WIDTH / vehicle.get_vehicle_type().get_max_speed_lat()
        } else {
            0.
        };
        let timeToStop = max2(timeToStopForward, timeToStopLateral);
        if !isOpposite {
            let searchDist = timeToStop * oncomingLane.get_speed_limit() * 2.0 + spaceToStop;
            neighLead = oncomingLane.get_opposite_leader(vehicle, searchDist, true, MSLane_MinorLinkMode::FOLLOW_NEVER);
            oncoming = self.get_oncoming_vehicle(oncomingLane, neighLead, searchDist, &mut vMax, Ptr::null(), MSLane_MinorLinkMode::FOLLOW_NEVER);
        } else {
            let searchDist = OPPOSITE_OVERTAKING_ONCOMING_LOOKAHEAD;
            oncoming = oncomingLane.get_opposite_leader(vehicle, searchDist, true, MSLane_MinorLinkMode::FOLLOW_NEVER);
            oncoming = self.get_oncoming_vehicle(oncomingLane, oncoming, searchDist, &mut vMax, Ptr::null(), MSLane_MinorLinkMode::FOLLOW_NEVER);
        }
        let mut oncomingSpeed: f64 = 0.0;
        let surplusGap = MSLaneChanger::compute_surplus_gap(vehicle, opposite, oncoming, timeToStop, spaceToStop, &mut oncomingSpeed, false);
        if !isOpposite && surplusGap < 0.0 {
            return false;
        }
        if bestOffset > 0 {
            let target = preb[(laneIndex + 1) as usize].lane;
            neighLead = target.get_opposite_leader(vehicle, OPPOSITE_OVERTAKING_MAX_LOOKAHEAD, true, MSLane_MinorLinkMode::FOLLOW_NEVER);
            let neighFollow = target.get_opposite_follower(vehicle);
            return self.check_change_opposite(vehicle, 1, target, &leader, &neighLead, &neighFollow, &preb);
        } else {
            return false;
        }
    }

    /// C++: `bool MSLaneChanger::avoidDeadlock(MSVehicle *, std::pair<MSVehicle *, double>, std::pair<MSVehicle *, double>, std::pair<MSVehicle *, double>)` [protected]
    /// avoid opposite-diretion deadlock when vehicles are stopped on both sides of the road The method may call saveBlockerLength to affect vehicle speed in the next step
    // bool
    // MSLaneChanger::avoidDeadlock(MSVehicle* vehicle,
    //                              std::pair<MSVehicle*, double> neighLead,
    //                              std::pair<MSVehicle*, double> overtaken,
    //                              std::pair<MSVehicle*, double> leader) {
    //     assert(!vehicle->getLaneChangeModel().isOpposite());
    // #ifdef DEBUG_CHANGE_OPPOSITE_DEADLOCK
    //     if (DEBUG_COND) {
    //         std::cout << SIMTIME << " veh=" << vehicle->getID() << " avoidDeadlock"
    //                   << " neighLead=" << Named::getIDSecure(neighLead.first)
    //                   << " overtaken=" << Named::getIDSecure(overtaken.first)
    //                   << " leader=" << Named::getIDSecure(leader.first)
    //                   << "\n";
    //     }
    // #endif
    //     if (leader.first == nullptr || neighLead.first == nullptr || overtaken.first == nullptr) {
    //         return false;
    //     } else if (!neighLead.first->isStopped()
    //                && vehicle->getWaitingSeconds() >= OPPOSITE_OVERTAKING_DEADLOCK_WAIT) {
    //         // possibly there is an oncoming vehicle before the stoppled leader that
    //         // could drive due to our yielding
    //         auto neighLeadFollow = neighLead.first->getFollower(overtaken.second);
    //         neighLead.second += neighLead.first->getVehicleType().getLengthWithGap() + neighLeadFollow.second;
    //         neighLead.first = const_cast<MSVehicle*>(neighLeadFollow.first);
    // #ifdef DEBUG_CHANGE_OPPOSITE_DEADLOCK
    //         if (DEBUG_COND) {
    //             std::cout << "  neighLead follower=" << Named::getIDSecure(neighLeadFollow.first) << "\n";
    //         }
    // #endif
    //         if (neighLead.first == nullptr) {
    //             return false;
    //         }
    //     }
    //
    //     const bool yield = (yieldToDeadlockOncoming(vehicle, neighLead.first, overtaken.second)
    //                         || leader.first->getWaitingSeconds() >= OPPOSITE_OVERTAKING_DEADLOCK_WAIT);
    //     if (neighLead.first->isStopped()
    //             && (overtaken.first->isStopped()
    //                 || leader.first->getLaneChangeModel().isOpposite()
    //                 || yield)) {
    //
    //         // estimate required gap
    //         double requiredGap = MAX2(vehicle->getVehicleType().getLengthWithGap(), neighLead.first->getVehicleType().getLengthWithGap());
    //         requiredGap = MAX2(requiredGap, overtaken.first->getVehicleType().getLengthWithGap());
    //         requiredGap = MAX2(requiredGap, leader.first->getVehicleType().getLengthWithGap());
    //         requiredGap += 1;
    //         const double distToStop = neighLead.second - requiredGap;
    //
    //         // find the next non-stopped vehicle behind neighLead
    //         double neighStoppedBack = vehicle->getVehicleType().getMinGap();
    //         while (neighLead.first != nullptr && neighLead.first->isStopped()) {
    //             const double nextGap = neighLead.second + neighLead.first->getVehicleType().getLengthWithGap();
    //             if (neighStoppedBack + nextGap > overtaken.second) {
    //                 break;
    //             }
    //             neighStoppedBack += nextGap;
    //             auto neighLeadFollow = neighLead.first->getFollower();
    //             neighLead.second = neighLeadFollow.second;
    //             neighLead.first = const_cast<MSVehicle*>(neighLeadFollow.first);
    // #ifdef DEBUG_CHANGE_OPPOSITE_DEADLOCK
    //             if (DEBUG_COND) {
    //                 std::cout << " neighLeadFollower=" << Named::getIDSecure(neighLead.first) << "\n";
    //             }
    // #endif
    //             if (neighStoppedBack > overtaken.second) {
    //                 break;
    //             }
    //         }
    //
    //         const double leaderBGap = leader.first->getBrakeGap();
    //         const double leaderFGap = leader.first->getLane()->getLeader(leader.first, leader.first->getPositionOnLane(), vehicle->getBestLanesContinuation(), overtaken.second, true).second;
    //         const double extraGap = MAX2(leaderBGap, leaderFGap);
    //         const double gapWithEgo = leader.second + extraGap - neighStoppedBack - vehicle->getVehicleType().getLengthWithGap();
    // #ifdef DEBUG_CHANGE_OPPOSITE_DEADLOCK
    //         if (DEBUG_COND) {
    //             std::cout << SIMTIME << " veh=" << vehicle->getID() << " avoidDeadlock"
    //                       << " neighLeadGap=" << neighLead.second
    //                       << " leaderGap=" << leader.second
    //                       << " bGap=" << leaderBGap
    //                       << " fGap=" << leaderFGap
    //                       << " eGap=" << extraGap
    //                       << " neighStoppedBack=" << neighStoppedBack
    //                       << " neighStoppedBackPos=" << vehicle->getPositionOnLane() + neighStoppedBack
    //                       << " requiredGap=" << requiredGap
    //                       << " gapWithEgo=" << gapWithEgo
    //                       << " yield=" << yield
    //                       << "\n";
    //         }
    // #endif
    //         // vehicle must fit behind leader and still leave required gap
    //         if (leader.first->getLaneChangeModel().isOpposite() || yield || gapWithEgo < requiredGap) {
    //             const std::vector<MSVehicle::LaneQ>& preb = vehicle->getBestLanes();
    //             const double currentDist = preb[vehicle->getLane()->getIndex()].length;
    //             const double stopPos = vehicle->getPositionOnLane() + distToStop;
    // #ifdef DEBUG_CHANGE_OPPOSITE_DEADLOCK
    //             if (DEBUG_COND) {
    //                 std::cout << "   currentDist=" << currentDist << " stopPos=" << stopPos << " lGap+eGap=" << leader.second + extraGap << " distToStop=" << distToStop << "\n";
    //             }
    // #endif
    //             if (leader.second + leaderBGap + leader.first->getLength() > distToStop) {
    //                 const double blockerLength = currentDist - stopPos + vehicle->getVehicleType().getMinGap();
    //                 const bool reserved = vehicle->getLaneChangeModel().saveBlockerLength(blockerLength, -1);
    // #ifdef DEBUG_CHANGE_OPPOSITE_DEADLOCK
    //                 if (DEBUG_COND) {
    //                     std::cout << SIMTIME << " veh=" << vehicle->getID() << " avoidDeadlock"
    //                               << " blockerLength=" << blockerLength
    //                               << " reserved=" << reserved
    //                               << "\n";
    //                 }
    // #endif
    //                 return reserved;
    //             }
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn avoid_deadlock(&mut self, vehicle: Ptr<MSVehicle>, neighLead: (Ptr<MSVehicle>, f64), overtaken: (Ptr<MSVehicle>, f64), leader: (Ptr<MSVehicle>, f64)) -> bool {
        debug_assert!(!vehicle.get_lane_change_model_2().is_opposite());
        let mut neighLead = neighLead;
        if leader.0.is_null() || neighLead.0.is_null() || overtaken.0.is_null() {
            return false;
        } else if !neighLead.0.is_stopped() && vehicle.get_waiting_seconds() >= OPPOSITE_OVERTAKING_DEADLOCK_WAIT {
            let neighLeadFollow = neighLead.0.get_follower(overtaken.1);
            neighLead.1 += neighLead.0.get_vehicle_type().get_length_with_gap() + neighLeadFollow.1;
            neighLead.0 = neighLeadFollow.0;
            if neighLead.0.is_null() {
                return false;
            }
        }
        let yield_ = self.yield_to_deadlock_oncoming(vehicle, neighLead.0, overtaken.1) || leader.0.get_waiting_seconds() >= OPPOSITE_OVERTAKING_DEADLOCK_WAIT;
        if neighLead.0.is_stopped() && (overtaken.0.is_stopped() || leader.0.get_lane_change_model_2().is_opposite() || yield_) {
            let mut requiredGap = max2(vehicle.get_vehicle_type().get_length_with_gap(), neighLead.0.get_vehicle_type().get_length_with_gap());
            requiredGap = max2(requiredGap, overtaken.0.get_vehicle_type().get_length_with_gap());
            requiredGap = max2(requiredGap, leader.0.get_vehicle_type().get_length_with_gap());
            requiredGap += 1.0;
            let distToStop = neighLead.1 - requiredGap;
            let mut neighStoppedBack = vehicle.get_vehicle_type().get_min_gap();
            while !neighLead.0.is_null() && neighLead.0.is_stopped() {
                let nextGap = neighLead.1 + neighLead.0.get_vehicle_type().get_length_with_gap();
                if neighStoppedBack + nextGap > overtaken.1 {
                    break;
                }
                neighStoppedBack += nextGap;
                let neighLeadFollow = neighLead.0.get_follower(0.0);
                neighLead.1 = neighLeadFollow.1;
                neighLead.0 = neighLeadFollow.0;
                if neighStoppedBack > overtaken.1 {
                    break;
                }
            }
            let leaderBGap = leader.0.get_brake_gap(false);
            let conts = vehicle.get_best_lanes_continuation().clone();
            let leaderFGap = leader.0.get_lane().get_leader(leader.0, leader.0.get_position_on_lane(), &conts, overtaken.1, true).1;
            let extraGap = max2(leaderBGap, leaderFGap);
            let gapWithEgo = leader.1 + extraGap - neighStoppedBack - vehicle.get_vehicle_type().get_length_with_gap();
            if leader.0.get_lane_change_model_2().is_opposite() || yield_ || gapWithEgo < requiredGap {
                let currentDist = vehicle.get_best_lanes()[vehicle.get_lane().get_index() as usize].length;
                let stopPos = vehicle.get_position_on_lane() + distToStop;
                if leader.1 + leaderBGap + leader.0.get_length() > distToStop {
                    let blockerLength = currentDist - stopPos + vehicle.get_vehicle_type().get_min_gap();
                    let reserved = vehicle.m().get_lane_change_model().save_blocker_length(blockerLength, -1.0);
                    return reserved;
                }
            }
        }
        false
    }

    /// C++: `bool MSLaneChanger::resolveDeadlock(MSVehicle *, std::pair<MSVehicle *const, double>, std::pair<MSVehicle *, double>, std::pair<MSVehicle *, double>)` [protected]
    /// keep stopping to resolve opposite-diretion deadlock while there is oncoming traffic The method may call saveBlockerLength to affect vehicle speed in the next step
    // bool
    // MSLaneChanger::resolveDeadlock(MSVehicle* vehicle,
    //                                std::pair<MSVehicle* const, double> leader,
    //                                std::pair<MSVehicle*, double> neighLead,
    //                                std::pair<MSVehicle*, double> overtaken) {
    //     const double deadLockZone = overtaken.second;
    // #ifdef DEBUG_CHANGE_OPPOSITE_DEADLOCK
    //     if (DEBUG_COND) {
    //         std::cout << SIMTIME << " veh=" << vehicle->getID() << " resolveDeadlock waiting=" << vehicle->getWaitingSeconds()
    //                   << " leader=" << Named::getIDSecure(leader.first)
    //                   << " gap=" << leader.second
    //                   << "\n";
    //     }
    // #endif
    //     if (vehicle->getWaitingSeconds() >= OPPOSITE_OVERTAKING_DEADLOCK_WAIT
    //             && leader.first != nullptr && leader.second > vehicle->getVehicleType().getLengthWithGap()) {
    //         // assume vehicle is halting to avoid deadlock (since there is enough
    //         // space to drive further)
    //         // keep halting as long as there is an oncoming vehicle
    //         std::pair<MSVehicle* const, double> oncomingOpposite = getOncomingOppositeVehicle(vehicle, std::make_pair(nullptr, -1), leader.second);
    // #ifdef DEBUG_CHANGE_OPPOSITE_DEADLOCK
    //         if (DEBUG_COND) {
    //             std::cout << SIMTIME << " veh=" << vehicle->getID() << " resolveDeadlock"
    //                       << " leader=" << leader.first->getID()
    //                       << " leaderGap=" << leader.second
    //                       << " neighLead=" << Named::getIDSecure(neighLead.first)
    //                       << " deadLockZone=" << deadLockZone
    //                       << "\n";
    //         }
    // #endif
    //         if (neighLead.first != nullptr && !neighLead.first->isStopped()) {
    //             // possibly there is an oncoming vehicle before the stoppled leader that
    //             // could drive due to our yielding
    //             auto neighLeadFollow = neighLead.first->getFollower(deadLockZone);
    //             neighLead.second += neighLead.first->getVehicleType().getLengthWithGap() + neighLeadFollow.second;
    //             neighLead.first = const_cast<MSVehicle*>(neighLeadFollow.first);
    // #ifdef DEBUG_CHANGE_OPPOSITE_DEADLOCK
    //             if (DEBUG_COND) {
    //                 std::cout << "  neighLead follower=" << Named::getIDSecure(neighLeadFollow.first) << "\n";
    //             }
    // #endif
    //         }
    //
    //         if (oncomingOpposite.first != nullptr ||
    //                 (neighLead.first != nullptr && neighLead.first->isStopped()
    //                  && yieldToDeadlockOncoming(vehicle, neighLead.first, deadLockZone))) {
    //             const std::vector<MSVehicle::LaneQ>& preb = vehicle->getBestLanes();
    //             const double currentDist = preb[vehicle->getLane()->getIndex()].length;
    //             // mirror code in patchSpeed
    //             const double blockerLength = currentDist - vehicle->getPositionOnLane() - POSITION_EPS - NUMERICAL_EPS;
    //             const bool reserved = vehicle->getLaneChangeModel().saveBlockerLength(blockerLength, -1);
    // #ifdef DEBUG_CHANGE_OPPOSITE_DEADLOCK
    //             if (DEBUG_COND) {
    //                 std::cout << SIMTIME << " veh=" << vehicle->getID() << " resolveDeadlock"
    //                           << " leader=" << leader.first->getID()
    //                           << " leaderGap=" << leader.second
    //                           << " oncoming=" << Named::getIDSecure(oncomingOpposite.first)
    //                           << " currentDist=" << currentDist
    //                           << " blockerLength=" << blockerLength
    //                           << " reserved=" << reserved
    //                           << "\n";
    //             }
    // #else
    //             UNUSED_PARAMETER(reserved);
    // #endif
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn resolve_deadlock(&mut self, vehicle: Ptr<MSVehicle>, leader: (Ptr<MSVehicle>, f64), neighLead: (Ptr<MSVehicle>, f64), overtaken: (Ptr<MSVehicle>, f64)) -> bool {
        let deadLockZone = overtaken.1;
        let mut neighLead = neighLead;
        if vehicle.get_waiting_seconds() >= OPPOSITE_OVERTAKING_DEADLOCK_WAIT && !leader.0.is_null() && leader.1 > vehicle.get_vehicle_type().get_length_with_gap() {
            let oncomingOpposite = self.get_oncoming_opposite_vehicle(vehicle, (Ptr::null(), -1.0), leader.1);
            if !neighLead.0.is_null() && !neighLead.0.is_stopped() {
                let neighLeadFollow = neighLead.0.get_follower(deadLockZone);
                neighLead.1 += neighLead.0.get_vehicle_type().get_length_with_gap() + neighLeadFollow.1;
                neighLead.0 = neighLeadFollow.0;
            }
            if !oncomingOpposite.0.is_null() || (!neighLead.0.is_null() && neighLead.0.is_stopped() && self.yield_to_deadlock_oncoming(vehicle, neighLead.0, deadLockZone)) {
                let currentDist = vehicle.get_best_lanes()[vehicle.get_lane().get_index() as usize].length;
                let blockerLength = currentDist - vehicle.get_position_on_lane() - POSITION_EPS - NUMERICAL_EPS;
                let _reserved = vehicle.m().get_lane_change_model().save_blocker_length(blockerLength, -1.0);
                return true;
            }
        }
        false
    }

    /// C++: `bool MSLaneChanger::yieldToDeadlockOncoming(const MSVehicle *, const MSVehicle *, double)` [protected]
    /// check whether to keep stopping for oncoming vehicles in the deadlock zone
    // bool
    // MSLaneChanger::yieldToDeadlockOncoming(const MSVehicle* vehicle, const MSVehicle* stoppedNeigh, double dist) {
    //     if (vehicle->getWaitingSeconds() >= OPPOSITE_OVERTAKING_DEADLOCK_WAIT && stoppedNeigh != nullptr) {
    //         // is there a vehicle waiting behind stoppedNeigh that is encouraged by
    //         // halting ego? Due to edge-ordering it might change-opposite this step
    //         // and not be visible as leader
    //         std::pair<const MSVehicle*, double> follower = stoppedNeigh->getFollower(dist);
    //         double followerGap = stoppedNeigh->getVehicleType().getLengthWithGap();
    //         while (follower.first != nullptr && followerGap < dist && follower.first->isStopped()) {
    //             followerGap += follower.second + follower.first->getVehicleType().getLengthWithGap();
    //             follower = follower.first->getFollower(dist);
    //         };
    //         if (follower.first != nullptr) {
    //             followerGap += follower.second;
    //         }
    // #ifdef DEBUG_CHANGE_OPPOSITE_DEADLOCK
    //         if (DEBUG_COND) {
    //             std::cout << SIMTIME << " veh=" << vehicle->getID() << " yieldToDeadlockOncoming"
    //                       << " dist=" << dist << " follower=" << Named::getIDSecure(follower.first) << " fGap=" << followerGap
    //                       << "\n";
    //         }
    // #endif
    //         return follower.first != nullptr && followerGap < dist && !follower.first->isStopped();
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn yield_to_deadlock_oncoming(&mut self, vehicle: Ptr<MSVehicle>, stoppedNeigh: Ptr<MSVehicle>, dist: f64) -> bool {
        if vehicle.get_waiting_seconds() >= OPPOSITE_OVERTAKING_DEADLOCK_WAIT && !stoppedNeigh.is_null() {
            let mut follower = stoppedNeigh.get_follower(dist);
            let mut followerGap = stoppedNeigh.get_vehicle_type().get_length_with_gap();
            while !follower.0.is_null() && followerGap < dist && follower.0.is_stopped() {
                followerGap += follower.1 + follower.0.get_vehicle_type().get_length_with_gap();
                follower = follower.0.get_follower(dist);
            }
            if !follower.0.is_null() {
                followerGap += follower.1;
            }
            return !follower.0.is_null() && followerGap < dist && !follower.0.is_stopped();
        }
        false
    }

    /// C++: `bool MSLaneChanger::yieldToOppositeWaiting(const MSVehicle *, const MSVehicle *, double, SUMOTime)` [protected]
    /// check whether to yield for oncoming vehicles that have waited longer for opposite overtaking
    // bool
    // MSLaneChanger::yieldToOppositeWaiting(const MSVehicle* vehicle, const MSVehicle* stoppedNeigh, double dist, SUMOTime deltaWait) {
    //     std::pair<const MSVehicle*, double> follower = stoppedNeigh->getFollower(dist);
    //     while (follower.first != nullptr && follower.second < dist && follower.first->isStopped()) {
    //         follower = follower.first->getFollower(dist);
    //     };
    // #ifdef DEBUG_CHANGE_OPPOSITE_DEADLOCK
    //     if (DEBUG_COND && follower.first != nullptr) {
    //         std::cout << SIMTIME << " yieldToOppositeWaiting veh=" << vehicle->getID() << " stoppedNeigh=" << stoppedNeigh->getID()
    //                   << " oncoming=" << follower.first->getID()
    //                   << " wait=" << follower.first->getWaitingSeconds()
    //                   << " vehWait=" << vehicle->getWaitingSeconds()
    //                   << " deltaWait=" << STEPS2TIME(deltaWait)
    //                   << "\n";
    //     }
    // #endif
    //     if (follower.first != nullptr && follower.second < dist && follower.first->getWaitingTime() > vehicle->getWaitingTime() + deltaWait) {
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn yield_to_opposite_waiting(&mut self, vehicle: Ptr<MSVehicle>, stoppedNeigh: Ptr<MSVehicle>, dist: f64, deltaWait: SUMOTime) -> bool {
        let mut follower = stoppedNeigh.get_follower(dist);
        while !follower.0.is_null() && follower.1 < dist && follower.0.is_stopped() {
            follower = follower.0.get_follower(dist);
        }
        if !follower.0.is_null() && follower.1 < dist && follower.0.get_waiting_time(false) > vehicle.get_waiting_time(false) + deltaWait {
            return true;
        }
        false
    }

    /// C++: `double MSLaneChanger::computeSafeOppositeLength(MSVehicle *, double, const MSLane *, double, std::pair<MSVehicle *, double>, double, double, std::pair<MSVehicle *, double>, std::pair<MSVehicle *, double>, std::pair<MSVehicle *, double>, double, const MSLane *, bool)` [protected]
    /// determine for how long the vehicle can drive safely on the opposite side
    // double
    // MSLaneChanger::computeSafeOppositeLength(MSVehicle* vehicle, double oppositeLength, const MSLane* source, double usableDist,
    //         std::pair<MSVehicle*, double> oncoming, double vMax, double oncomingSpeed,
    //         std::pair<MSVehicle*, double> neighLead,
    //         std::pair<MSVehicle*, double> overtaken,
    //         std::pair<MSVehicle*, double> neighFollow,
    //         double surplusGap, const MSLane* opposite,
    //         bool canOvertake) {
    //     // compute the remaining distance that can be driven on the opposite side
    //     // this value will put into oppositeLength of the opposite lanes
    //     // @note: length counts from the start of the current lane
    //     // @note: see MSLaneChangeModel::LC2013::_wantsChange @1092 (isOpposite()
    //     // position on the target lane
    //     const double forwardPos = source->getOppositePos(vehicle->getPositionOnLane());
    //
    //     // consider usableDist (due to minor links or end of opposite lanes)
    //     oppositeLength = MIN2(oppositeLength, usableDist + forwardPos);
    //     // consider upcoming stops
    //     oppositeLength = MIN2(oppositeLength, vehicle->nextStopDist() + forwardPos);
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //     if (DEBUG_COND) {
    //         std::cout << "   laneQLength=" << oppositeLength << " usableDist=" << usableDist << " forwardPos=" << forwardPos << " stopDist=" << vehicle->nextStopDist() << "\n";
    //     }
    // #endif
    //     // consider oncoming leaders
    //     const MSVehicle* oncomingVeh = oncoming.first;
    //     if (oncomingVeh != 0) {
    //         if (!oncomingVeh->getLaneChangeModel().isOpposite() && oncomingVeh->getLaneChangeModel().getShadowLane() != source) {
    //             double egoSpeedFraction = 0.5;
    //             if (oncomingSpeed > 0) {
    //                 egoSpeedFraction = MIN2(egoSpeedFraction, vMax / (vMax + oncomingSpeed));
    //             }
    //             oppositeLength = MIN2(oppositeLength, forwardPos + oncoming.second * egoSpeedFraction);
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //             if (DEBUG_COND) {
    //                 std::cout << SIMTIME << " found oncoming leader=" << oncomingVeh->getID() << " gap=" << oncoming.second
    //                           << " egoSpeedFraction=" << egoSpeedFraction << " newDist=" << oppositeLength << "\n";
    //             }
    // #endif
    //         } else {
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //             if (DEBUG_COND) {
    //                 std::cout << SIMTIME << " opposite leader=" << oncomingVeh->getID() << " gap=" << oncoming.second << " is driving against the flow\n";
    //             }
    // #endif
    //         }
    //         if (neighLead.first != nullptr) {
    //             if (overtaken.first == nullptr) {
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //                 if (DEBUG_COND) {
    //                     std::cout << SIMTIME << " ego=" << vehicle->getID() << " did not find columnleader to overtake\n";
    //                 }
    // #endif
    //             } else if (oncomingVeh != nullptr && oncomingVeh->isStopped()
    //                        && neighLead.second > 0
    //                        && neighFollow.second > 0
    //                        && yieldToOppositeWaiting(vehicle, oncomingVeh, 10, TIME2STEPS(60))) {
    //                 // merge back into the forward lane
    //                 oppositeLength = forwardPos + neighLead.second;
    //             } else {
    //                 if (surplusGap > 0) {
    //                     // exaggerate remaining dist so that the vehicle continues
    //                     // overtaking (otherwise the lane change model might abort prematurely)
    //                     oppositeLength += 1000;
    //                 } else {
    //                     // return from the opposite side ahead of the unpassable column (unless overlapping)
    //                     if (overtaken.second > 0) {
    //                         oppositeLength = MIN2(oppositeLength, forwardPos + overtaken.second);
    //                     }
    //                     // (don't set the distance so low as to imply emergency braking)
    //                     oppositeLength = MAX2(oppositeLength, forwardPos + vehicle->getCarFollowModel().brakeGap(vehicle->getSpeed()));
    //                 }
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //                 if (DEBUG_COND) {
    //                     std::cout << SIMTIME << " ego=" << vehicle->getID() << " is overtaking " << overtaken.first->getID()
    //                               << " surplusGap=" << surplusGap
    //                               << " final laneQLength=" << oppositeLength
    //                               << "\n";
    //                 }
    // #endif
    //             }
    //         }
    //     } else {
    //         if (overtaken.first == nullptr || !canOvertake) {
    //             // there is no reason to stay on the opposite side
    //             std::pair<MSVehicle* const, double> oppFollow = opposite->getOppositeFollower(vehicle);
    //             if (oppFollow.first == nullptr) {
    //                 oppositeLength = forwardPos;
    //             } else {
    //                 const double secureGap = oppFollow.first->getCarFollowModel().getSecureGap(
    //                                              oppFollow.first, vehicle, oppFollow.first->getSpeed(), vehicle->getSpeed(), vehicle->getCarFollowModel().getMaxDecel());
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //                 if (DEBUG_COND) {
    //                     std::cout << SIMTIME << " ego=" << vehicle->getID() << " neighFollow=" << oppFollow.first->getID() << " gap=" << oppFollow.second << " secureGap=" << secureGap << "\n";
    //                 }
    // #endif
    //                 if (oppFollow.second > secureGap) {
    //                     // back gap is safe for immidiate return
    //                     oppositeLength = forwardPos;
    //                 }
    //             }
    //         }
    //     }
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //     if (DEBUG_COND) {
    //         std::cout << SIMTIME << " veh=" << vehicle->getID() << " remaining dist=" << oppositeLength - forwardPos << " forwardPos=" << forwardPos << " oppositeLength=" << oppositeLength << "\n";
    //     }
    // #endif
    //     return oppositeLength;
    // }
    #[allow(unused_variables)]
    pub fn compute_safe_opposite_length(&mut self, vehicle: Ptr<MSVehicle>, oppositeLength: f64, source: Ptr<MSLane>, usableDist: f64, oncoming: (Ptr<MSVehicle>, f64), vMax: f64, oncomingSpeed: f64, neighLead: (Ptr<MSVehicle>, f64), overtaken: (Ptr<MSVehicle>, f64), neighFollow: (Ptr<MSVehicle>, f64), surplusGap: f64, opposite: Ptr<MSLane>, canOvertake: bool) -> f64 {
        let forwardPos = source.get_opposite_pos(vehicle.get_position_on_lane());
        let mut oppositeLength = oppositeLength;
        oppositeLength = min2(oppositeLength, usableDist + forwardPos);
        oppositeLength = min2(oppositeLength, vehicle.next_stop_dist() + forwardPos);
        let oncomingVeh = oncoming.0;
        if !oncomingVeh.is_null() {
            if !oncomingVeh.get_lane_change_model_2().is_opposite() && oncomingVeh.get_lane_change_model_2().get_shadow_lane() != source {
                let mut egoSpeedFraction = 0.5;
                if oncomingSpeed > 0.0 {
                    egoSpeedFraction = min2(egoSpeedFraction, vMax / (vMax + oncomingSpeed));
                }
                oppositeLength = min2(oppositeLength, forwardPos + oncoming.1 * egoSpeedFraction);
            } else {
                // only the C++ debug output is skipped here
            }
            if !neighLead.0.is_null() {
                if overtaken.0.is_null() {
                    // only the C++ debug output is skipped here
                } else if oncomingVeh.is_stopped() && neighLead.1 > 0.0 && neighFollow.1 > 0.0 && self.yield_to_opposite_waiting(vehicle, oncomingVeh, 10.0, time2steps(60.0)) {
                    oppositeLength = forwardPos + neighLead.1;
                } else {
                    if surplusGap > 0.0 {
                        oppositeLength += 1000.0;
                    } else {
                        if overtaken.1 > 0.0 {
                            oppositeLength = min2(oppositeLength, forwardPos + overtaken.1);
                        }
                        oppositeLength = max2(oppositeLength, forwardPos + vehicle.get_car_follow_model().brake_gap(vehicle.get_speed()));
                    }
                }
            }
        } else {
            if overtaken.0.is_null() || !canOvertake {
                let oppFollow = opposite.get_opposite_follower(vehicle);
                if oppFollow.0.is_null() {
                    oppositeLength = forwardPos;
                } else {
                    let secureGap = oppFollow.0.get_car_follow_model().get_secure_gap(oppFollow.0, vehicle, oppFollow.0.get_speed(), vehicle.get_speed(), vehicle.get_car_follow_model().get_max_decel());
                    if oppFollow.1 > secureGap {
                        oppositeLength = forwardPos;
                    }
                }
            }
        }
        oppositeLength
    }

    /// C++: `double MSLaneChanger::computeSurplusGap(const MSVehicle *, const MSLane *, std::pair<MSVehicle *, double>, double, double, double &, bool)` [static, protected]
    // double
    // MSLaneChanger::computeSurplusGap(const MSVehicle* vehicle, const MSLane* opposite, std::pair<MSVehicle*, double> oncoming,
    //                                  double timeToOvertake, double spaceToOvertake, double& oncomingSpeed, bool oncomingOpposite) {
    //     double surplusGap = std::numeric_limits<double>::max();
    //     const MSVehicle* oncomingVeh = oncoming.first;
    //     if (oncomingVeh != 0 && (oncomingOpposite
    //                              || (!oncomingVeh->getLaneChangeModel().isOpposite()
    //                                  && oncomingVeh->getLaneChangeModel().getShadowLane() != opposite))) {
    //         // conservative: assume that the oncoming vehicle accelerates to its maximum speed
    //         // unless it has been standing (then assume it is trying to let us pass
    //         // to avoid deadlock)
    //         oncomingSpeed = (oncomingVeh->isStopped() || oncomingVeh->getWaitingSeconds() >= OPPOSITE_OVERTAKING_DEADLOCK_WAIT
    //                          ? 0 : oncomingVeh->getLane()->getVehicleMaxSpeed(oncomingVeh));
    //         const double safetyGap = ((oncomingSpeed + vehicle->getLane()->getVehicleMaxSpeed(vehicle))
    //                                   * vehicle->getCarFollowModel().getHeadwayTime()
    //                                   * OPPOSITE_OVERTAKING_SAFETYGAP_HEADWAY_FACTOR);
    //         surplusGap = oncoming.second - spaceToOvertake - timeToOvertake * oncomingSpeed - safetyGap;
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //         if (DEBUG_COND) {
    //             std::cout << SIMTIME
    //                       << " oncoming=" << oncomingVeh->getID()
    //                       << " oGap=" << oncoming.second
    //                       << " oSpeed=" << oncomingSpeed
    //                       << " sto=" << spaceToOvertake
    //                       << " tto=" << timeToOvertake
    //                       << " safetyGap=" << safetyGap
    //                       << " surplusGap=" << surplusGap
    //                       << "\n";
    //         }
    // #endif
    //     }
    //     return surplusGap;
    // }
    #[allow(unused_variables)]
    pub fn compute_surplus_gap(vehicle: Ptr<MSVehicle>, opposite: Ptr<MSLane>, oncoming: (Ptr<MSVehicle>, f64), timeToOvertake: f64, spaceToOvertake: f64, oncomingSpeed: &mut f64, oncomingOpposite: bool) -> f64 {
        let mut surplusGap = f64::MAX;
        let oncomingVeh = oncoming.0;
        if !oncomingVeh.is_null() && (oncomingOpposite || (!oncomingVeh.get_lane_change_model_2().is_opposite() && oncomingVeh.get_lane_change_model_2().get_shadow_lane() != opposite)) {
            *oncomingSpeed = if oncomingVeh.is_stopped() || oncomingVeh.get_waiting_seconds() >= OPPOSITE_OVERTAKING_DEADLOCK_WAIT {
                0.0
            } else {
                oncomingVeh.get_lane().get_vehicle_max_speed(traffic_object(oncomingVeh))
            };
            let safetyGap = ((*oncomingSpeed + vehicle.get_lane().get_vehicle_max_speed(traffic_object(vehicle)))
                * vehicle.get_car_follow_model().get_headway_time()
                * OPPOSITE_OVERTAKING_SAFETYGAP_HEADWAY_FACTOR);
            surplusGap = oncoming.1 - spaceToOvertake - timeToOvertake * *oncomingSpeed - safetyGap;
        }
        surplusGap
    }

    /// C++: `bool MSLaneChanger::foundHilltop(MSVehicle *, bool, double, const std::vector<MSLane *> &, int, double, double, double)` [static, protected]
    // bool
    // MSLaneChanger::foundHilltop(MSVehicle* vehicle, bool foundHill, double searchDist, const std::vector<MSLane*>& bestLanes, int view, double pos, double lastMax, double hilltopThreshold) {
    //     if (view >= (int)bestLanes.size()) {
    //         return false;
    //     }
    //     MSLane* lane = bestLanes[view];
    //     double laneDist = 0;
    //     const PositionVector& shape = lane->getShape();
    //     double lastZ = lastMax;
    //     for (int i = 1; i < (int)shape.size(); i++) {
    //         const double dist = lane->interpolateGeometryPosToLanePos(shape[i - 1].distanceTo(shape[i]));
    //         laneDist += dist;
    //         if (laneDist > pos) {
    //             const double z = shape[i].z();
    //             if (z > lastMax) {
    //                 lastMax = z;
    //             }
    //             if (z > lastZ) {
    //                 foundHill = true;
    //             }
    //             lastZ = z;
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //             if (DEBUG_COND) {
    //                 std::cout << SIMTIME << "   foundHill=" << foundHill << " searchDist=" << searchDist << " lastMax=" << lastMax << " lane=" << lane->getID() << " laneDist=" << laneDist << " z=" << z << "\n";
    //             }
    // #endif
    //             if (foundHill && z < lastMax) {
    //                 const double drop = lastMax - z;
    //                 //std::cout << SIMTIME << "   searchDist=" << searchDist << " hillDrop=" << drop << " lastMax=" << lastMax << " lane=" << lane->getID() << " laneDist=" << laneDist << " z=" << z << "\n";
    //                 if (drop > hilltopThreshold) {
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //                     if (DEBUG_COND) {
    //                         std::cout << "   cannot changeOpposite before the top of a hill searchDist=" << searchDist << " hillDrop=" << drop
    //                                   << " lastMax=" << lastMax << " lane=" << lane->getID() << " laneDist=" << laneDist << " z=" << z << "\n";
    //                     }
    // #endif
    //                     return true;
    //                 }
    //             }
    //             if (pos == 0) {
    //                 searchDist -= dist;
    //             } else {
    //                 searchDist -= laneDist - pos;
    //                 pos = 0;
    //             }
    //             if (searchDist <= 0) {
    //                 return false;
    //             }
    //         }
    //     }
    //     return foundHilltop(vehicle, foundHill, searchDist, bestLanes, view + 1, 0, lastMax, hilltopThreshold);
    // }
    #[allow(unused_variables)]
    pub fn found_hilltop(vehicle: Ptr<MSVehicle>, foundHill: bool, searchDist: f64, bestLanes: &Vec<Ptr<MSLane>>, view: i32, pos: f64, lastMax: f64, hilltopThreshold: f64) -> bool {
        let mut foundHill = foundHill;
        let mut searchDist = searchDist;
        let mut pos = pos;
        let mut lastMax = lastMax;
        if view >= bestLanes.len() as i32 {
            return false;
        }
        let lane = bestLanes[view as usize];
        let mut laneDist: f64 = 0.0;
        let shape = lane.get_shape_2();
        let mut lastZ = lastMax;
        for i in 1..shape.len() {
            let dist = lane.interpolate_geometry_pos_to_lane_pos(shape[i - 1].distance_to(&shape[i]));
            laneDist += dist;
            if laneDist > pos {
                let z = shape[i].z();
                if z > lastMax {
                    lastMax = z;
                }
                if z > lastZ {
                    foundHill = true;
                }
                lastZ = z;
                if foundHill && z < lastMax {
                    let drop = lastMax - z;
                    if drop > hilltopThreshold {
                        return true;
                    }
                }
                if pos == 0.0 {
                    searchDist -= dist;
                } else {
                    searchDist -= laneDist - pos;
                    pos = 0.0;
                }
                if searchDist <= 0.0 {
                    return false;
                }
            }
        }
        MSLaneChanger::found_hilltop(vehicle, foundHill, searchDist, bestLanes, view + 1, 0.0, lastMax, hilltopThreshold)
    }

    /// C++: `std::vector<MSVehicle::LaneQ> MSLaneChanger::getBestLanesOpposite(MSVehicle *, const MSLane *, double)` [static, protected]
    /// add LaneQ for opposite lanes
    // std::vector<MSVehicle::LaneQ>
    // MSLaneChanger::getBestLanesOpposite(MSVehicle* vehicle, const MSLane* stopLane, double oppositeLength) {
    //     const bool isOpposite = vehicle->getLaneChangeModel().isOpposite();
    //     const MSEdge* forward = isOpposite ? vehicle->getLane()->getEdge().getOppositeEdge()->getNormalSuccessor() : vehicle->getLane()->getEdge().getNormalSuccessor();
    //     const MSEdge* opposite = forward->getOppositeEdge();
    //     const int numForward = forward->getNumLanes();
    //     const int numOpposite = opposite->getNumLanes();
    //     const std::vector<MSLane*>& oLanes = opposite->getLanes();
    //     std::vector<MSVehicle::LaneQ> preb = vehicle->getBestLanes();
    //     for (int i = 0; i < numOpposite; i++) {
    //         preb.push_back(preb.back());
    //         preb.back().lane = oLanes[numOpposite - 1 - i];
    //         preb.back().length = oppositeLength;
    //         if (isOpposite) {
    //             preb.back().bestLaneOffset = -1 - i;
    //             //std::cout << "  oi=" << i << " bestOffset =" << preb.back().bestLaneOffset << "\n";
    //         }
    //     }
    //     if (stopLane != nullptr) {
    //         const int stopIndex = numForward + numOpposite - stopLane->getIndex() - 1;
    //         for (int i = 0; i < (int)preb.size(); i++) {
    //             preb[i].bestLaneOffset = stopIndex - i;
    //             preb[i].length = vehicle->getLaneChangeModel().getForwardPos() + vehicle->nextStopDist();
    //             //std::cout << "  oi2=" << i << " stopIndex=" << stopIndex << " bestOffset =" << preb[i].bestLaneOffset << " stopDist=" << vehicle->nextStopDist() << " length=" << preb[i].length << "\n";
    //         }
    //     }
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //     if (DEBUG_COND) {
    //         std::cout << SIMTIME << " getBestLanesOpposite " << vehicle->getID() << " isOpposite=" << isOpposite << "\n";
    //         for (int i = 0; i < (int)preb.size(); i++) {
    //             std::cout << "   i=" << i << " lane=" << preb[i].lane->getID() << " bestOffset=" << preb[i].bestLaneOffset << " length=" << preb[i].length << "\n";
    //         }
    //     }
    // #endif
    //     return preb;
    // }
    #[allow(unused_variables)]
    pub fn get_best_lanes_opposite(vehicle: Ptr<MSVehicle>, stopLane: Ptr<MSLane>, oppositeLength: f64) -> Vec<MSVehicle_LaneQ> {
        let isOpposite = vehicle.get_lane_change_model_2().is_opposite();
        let forward = if isOpposite { vehicle.get_lane().get_edge().get_opposite_edge().get_normal_successor() } else { vehicle.get_lane().get_edge().get_normal_successor() };
        let opposite = forward.get_opposite_edge();
        let numForward = forward.get_num_lanes();
        let numOpposite = opposite.get_num_lanes();
        let oLanes = opposite.get_lanes().clone();
        let mut preb: Vec<MSVehicle_LaneQ> = vehicle.get_best_lanes().iter().map(laneq_clone).collect();
        for i in 0..numOpposite {
            let mut q = laneq_clone(preb.last().unwrap());
            q.lane = oLanes[(numOpposite - 1 - i) as usize];
            q.length = oppositeLength;
            if isOpposite {
                q.bestLaneOffset = -1 - i;
            }
            preb.push(q);
        }
        if !stopLane.is_null() {
            let stopIndex = numForward + numOpposite - stopLane.get_index() - 1;
            for i in 0..preb.len() {
                preb[i].bestLaneOffset = stopIndex - i as i32;
                preb[i].length = vehicle.get_lane_change_model_2().get_forward_pos() + vehicle.next_stop_dist();
            }
        }
        preb
    }

    /// C++: `double MSLaneChanger::getMaxOvertakingSpeed(const MSVehicle *, double)` [static, protected]
    /// compute maximum maneuver speed
    // double
    // MSLaneChanger::getMaxOvertakingSpeed(const MSVehicle* vehicle, double maxSpaceToOvertake) {
    //     // v*v/2*a + v*v/2*d = maxSpaceToOvertake
    //     const double a = vehicle->getCarFollowModel().getMaxAccel();
    //     const double d = vehicle->getCarFollowModel().getMaxDecel();
    //     const double v = sqrt(2 * maxSpaceToOvertake * a * d / (a + d));
    //     return v;
    // }
    #[allow(unused_variables)]
    pub fn get_max_overtaking_speed(vehicle: Ptr<MSVehicle>, maxSpaceToOvertake: f64) -> f64 {
        let a = vehicle.get_car_follow_model().get_max_accel();
        let d = vehicle.get_car_follow_model().get_max_decel();
        let v = (2.0 * maxSpaceToOvertake * a * d / (a + d)).sqrt();
        v
    }

    /// C++: `void MSLaneChanger::MSLaneChanger()` [private]
    /// Default constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2() -> Self {
        MSLaneChanger {
            vt: &DEFAULT_MS_LANE_CHANGER_VT,
            myChanger: Vec::new(),
            myCandi: 0,
            myAllowsChanging: false,
            myChangeToOpposite: false,
            checkOpened: false,
        }
    }

    /// C++: `void MSLaneChanger::MSLaneChanger(const MSLaneChanger &)` [private]
    /// Copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_3(arg0: &MSLaneChanger) -> Self {
        // shortcut: private C++ copy constructor is never called; ChangeElem is not Clone in this port, so the copy starts with an empty changer
        let _ = arg0;
        MSLaneChanger {
            vt: &DEFAULT_MS_LANE_CHANGER_VT,
            myChanger: Vec::new(),
            myCandi: 0,
            myAllowsChanging: arg0.myAllowsChanging,
            myChangeToOpposite: arg0.myChangeToOpposite,
            checkOpened: arg0.checkOpened,
        }
    }

    /// C++: `MSLaneChanger & MSLaneChanger::operator=(const MSLaneChanger &)` [private]
    /// Assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSLaneChanger) -> &mut MSLaneChanger {
        // shortcut: private C++ assignment operator is never called; returns self unchanged, upgrade when ChangeElem is Clone
        let _ = arg0;
        self
    }

}


/// Field-wise copy of MSVehicle::LaneQ (the C++ copy constructor; LaneQ is not Clone in this port)
fn laneq_clone(q: &MSVehicle_LaneQ) -> MSVehicle_LaneQ {
    MSVehicle_LaneQ {
        lane: q.lane,
        length: q.length,
        currentLength: q.currentLength,
        occupation: q.occupation,
        nextOccupation: q.nextOccupation,
        bestLaneOffset: q.bestLaneOffset,
        allowsContinuation: q.allowsContinuation,
        bestContinuations: q.bestContinuations.clone(),
    }
}
