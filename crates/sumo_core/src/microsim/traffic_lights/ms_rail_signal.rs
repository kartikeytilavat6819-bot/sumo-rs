//! Port of microsim/traffic_lights/MSRailSignal.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use std::cell::{Cell, RefCell};

/// C++ typedef `std::pair<const SUMOVehicle *const, const MSLink::ApproachingVehicleInformation> MSRailSignal::Approaching`
pub type MSRailSignal_Approaching = (Ptr<SUMOVehicle>, MSLink_ApproachingVehicleInformation);

/// C++ typedef `std::map<const MSLane *, int, ComparatorNumericalIdLess> MSRailSignal::LaneVisitedMap`
pub type MSRailSignal_LaneVisitedMap = BTreeMap<Ptr<MSLane>, i32>;

/// C++ class `MSRailSignal::LinkInfo` (microsim/traffic_lights/MSRailSignal.h)
pub struct MSRailSignal_LinkInfo {
    pub myLink: Ptr<MSLink>,
    /// all driveways immediately following this link
    pub myDriveways: Vec<Ptr<MSDriveWay>>,
    pub myLastRerouteTime: SUMOTime,
    pub myLastRerouteVehicle: Ptr<SUMOVehicle>,
    /// whether this link is used for railway operations
    pub myControlled: bool,
}

impl MSRailSignal_LinkInfo {
    /// C++: `void LinkInfo::LinkInfo(MSLink *)`
    /// constructor
    // MSRailSignal::LinkInfo::LinkInfo(MSLink* link):
    //     myLink(link) {
    //     reset();
    // }
    #[allow(unused_variables)]
    pub fn new(link: Ptr<MSLink>) -> Self {
        let mut result = MSRailSignal_LinkInfo {
            myLink: link,
            myDriveways: Vec::new(),
            myLastRerouteTime: 0,
            myLastRerouteVehicle: Ptr::null(),
            myControlled: false,
        };
        result.reset();
        result
    }

    /// C++: `void LinkInfo::~LinkInfo()`
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        for dw in &self.myDriveways {
            unsafe { drop(Box::from_raw(dw.as_raw())) };
        }
        self.myDriveways.clear();
    }

    /// C++: `std::string LinkInfo::getID() const` [const]
    /// return id for this railsignal-link
    // std::string
    // MSRailSignal::LinkInfo::getID() const {
    //     return myLink->getTLLogic()->getID() + "_" + toString(myLink->getTLIndex());
    // }
    #[allow(unused_variables)]
    pub fn get_id(&self) -> String {
        self.myLink.get_tl_logic().get_id().to_string()
            + "_"
            + &utils::common::to_string::to_string(&self.myLink.get_tl_index())
    }

    /// C++: `MSDriveWay & LinkInfo::getDriveWay(const SUMOVehicle *, int)`
    /// retrieve an existing Driveway or construct a new driveway based on the vehicles route
    // MSDriveWay&
    // MSRailSignal::LinkInfo::getDriveWay(const SUMOVehicle* veh, int searchStart) {
    //     MSEdge* first = &myLink->getLane()->getEdge();
    //     auto searchStartIt = searchStart < 0 ? veh->getCurrentRouteEdge() : veh->getRoute().begin() + searchStart;
    //     MSRouteIterator firstIt = std::find(searchStartIt, veh->getRoute().end(), first);
    //     if (firstIt == veh->getRoute().end()) {
    //         // possibly the vehicle has already gone past the first edge (i.e.
    //         // because first is short or the step-length is high)
    //         // lets look backward along the route
    //         // give some slack because the vehicle might have been braking from a higher speed and using ballistic integration
    //         double lookBack = SPEED2DIST(veh->getSpeed() + 10);
    //         int routeIndex = veh->getRoutePosition() - 1;
    //         while (lookBack > 0 && routeIndex > 0) {
    //             const MSEdge* prevEdge = veh->getRoute().getEdges()[routeIndex];
    //             if (prevEdge == first) {
    //                 firstIt = veh->getRoute().begin() + routeIndex;
    //                 break;
    //             }
    //             lookBack -= prevEdge->getLength();
    //             routeIndex--;
    //         }
    //     }
    //     MSRailSignal* rs = const_cast<MSRailSignal*>(dynamic_cast<const MSRailSignal*>(myLink->getTLLogic()));
    //     if (firstIt == veh->getRoute().end()) {
    //         WRITE_WARNING("Invalid approach information to rail signal " + MSDriveWay::getClickableTLLinkID(myLink) + " after rerouting for vehicle '" + veh->getID()
    //                       + "' first driveway edge '" + first->getID() + "' time=" + time2string(MSNet::getInstance()->getCurrentTimeStep()) + ".");
    //         if (myDriveways.empty()) {
    //             ConstMSEdgeVector dummyRoute;
    //             dummyRoute.push_back(&myLink->getLane()->getEdge());
    //             MSDriveWay* dw = MSDriveWay::buildDriveWay(rs->getNewDrivewayID(), myLink, dummyRoute.begin(), dummyRoute.end());
    //             myDriveways.push_back(dw);
    //         }
    //         return *myDriveways.front();
    //     }
    //     //std::cout << SIMTIME << " veh=" << veh->getID() << " rsl=" << getID() << " dws=" << myDriveways.size() << "\n";
    //     return getDriveWay(firstIt, veh->getRoute().end(), veh->getID());
    // }
    #[allow(unused_variables)]
    pub fn get_drive_way(&mut self, arg0: Ptr<SUMOVehicle>, searchStart: i32) -> &mut MSDriveWay {
        let veh = arg0.cast::<MSBaseVehicle>();
        let first = self.myLink.get_lane().myEdge;
        let edges = veh.get_route().get_edges();
        let end = edges.len();
        let search_start_it = if searchStart < 0 {
            *veh.get_current_route_edge()
        } else {
            searchStart as usize
        };
        let mut first_it = edges[search_start_it.min(end)..]
            .iter()
            .position(|e| *e == first)
            .map(|i| search_start_it.min(end) + i);
        if first_it.is_none() {
            // possibly the vehicle has already gone past the first edge (i.e.
            // because first is short or the step-length is high)
            // lets look backward along the route
            // give some slack because the vehicle might have been braking from a higher speed and using ballistic integration
            let mut look_back = utils::common::time::speed2dist(veh.cast::<MSVehicle>().get_speed() + 10.0);
            let mut route_index = veh.get_route_position() - 1;
            while look_back > 0.0 && route_index > 0 {
                let prev_edge = edges[route_index as usize];
                if prev_edge == first {
                    first_it = Some(route_index as usize);
                    break;
                }
                look_back -= prev_edge.get_length();
                route_index -= 1;
            }
        }
        let rs: Ptr<MSRailSignal> = self.myLink.get_tl_logic().cast::<MSRailSignal>();
        let first_it = match first_it {
            Some(i) => i,
            None => {
                utils::common::msg_handler::write_warning(&format!(
                    "Invalid approach information to rail signal {} after rerouting for vehicle '{}' first driveway edge '{}' time={}.",
                    MSDriveWay::get_clickable_tl_link_id(self.myLink),
                    arg0.get_id(),
                    first.get_id(),
                    utils::common::time::time2string(MSNet::get_instance().get_current_time_step())
                ));
                if self.myDriveways.is_empty() {
                    let dummy_route: ConstMSEdgeVector = vec![self.myLink.get_lane().myEdge];
                    let id = rs.m().get_new_driveway_id();
                    let dummy_end = dummy_route.len();
                    let dw = MSDriveWay::build_drive_way(&id, self.myLink, &dummy_route, 0, dummy_end);
                    self.myDriveways.push(dw);
                }
                return self.myDriveways[0].get_mut().unwrap();
            }
        };
        //std::cout << SIMTIME << " veh=" << veh->getID() << " rsl=" << getID() << " dws=" << myDriveways.size() << "\n";
        self.get_drive_way_2(arg0.cast::<MSVehicle>().get_route().get_edges(), first_it, end, arg0.get_id())
    }

    /// C++: `MSDriveWay & LinkInfo::getDriveWay(MSRouteIterator, MSRouteIterator, const std::string &)`
    // MSDriveWay&
    // MSRailSignal::LinkInfo::getDriveWay(MSRouteIterator firstIt, MSRouteIterator endIt, const std::string& info) {
    //     for (MSDriveWay* dw : myDriveways) {
    //         if (dw->match(firstIt, endIt)) {
    //             return *dw;
    //         }
    // #ifdef DEBUG_SELECT_DRIVEWAY
    //         std::cout << SIMTIME << " rs=" << getID() << " veh=" << info << " other dwSignal=" << dw->foundSignal() << " dwRoute=" << toString(dw->getRoute()) << "\n";
    // #else
    //         UNUSED_PARAMETER(info);
    // #endif
    //     }
    //     MSRailSignal* rs = const_cast<MSRailSignal*>(dynamic_cast<const MSRailSignal*>(myLink->getTLLogic()));
    //     MSDriveWay* dw = MSDriveWay::buildDriveWay(rs->getNewDrivewayID(), myLink, firstIt, endIt);
    //     dw->setVehicle(info);
    // #ifdef DEBUG_SELECT_DRIVEWAY
    //     std::cout << SIMTIME << " rs=" << getID() << " veh=" << info << " new dwSignal=" << dw->foundSignal() << " dwRoute=" << toString(dw->getRoute()) << "\n";
    // #endif
    //     myDriveways.push_back(dw);
    //     return *myDriveways.back();
    // }
    #[allow(unused_variables)]
    pub fn get_drive_way_2(&mut self, route: &ConstMSEdgeVector, firstIt: MSRouteIterator, endIt: MSRouteIterator, info: &str) -> &mut MSDriveWay {
        for i in 0..self.myDriveways.len() {
            let dw = self.myDriveways[i];
            if dw.match_(route, firstIt, endIt) {
                return dw.get_mut().unwrap();
            }
        }
        let rs: Ptr<MSRailSignal> = self.myLink.get_tl_logic().cast::<MSRailSignal>();
        let id = rs.m().get_new_driveway_id();
        let dw = MSDriveWay::build_drive_way(&id, self.myLink, route, firstIt, endIt);
        dw.m().set_vehicle(info);
        self.myDriveways.push(dw);
        self.myDriveways.last().unwrap().get_mut().unwrap()
    }

    /// C++: `void LinkInfo::reroute(SUMOVehicle *, const MSEdgeVector &)`
    /// try rerouting vehicle if reservation failed
    // void
    // MSRailSignal::LinkInfo::reroute(SUMOVehicle* veh, const MSEdgeVector& occupied) {
    //     MSDevice_Routing* rDev = static_cast<MSDevice_Routing*>(veh->getDevice(typeid(MSDevice_Routing)));
    //     const SUMOTime now = MSNet::getInstance()->getCurrentTimeStep();
    //     if (rDev != nullptr
    //             && rDev->mayRerouteRailSignal()
    //             && (myLastRerouteVehicle != veh
    //                 // reroute each vehicle only once if no periodic routing is allowed,
    //                 // otherwise with the specified period
    //                 || (rDev->getPeriod() > 0 && myLastRerouteTime + rDev->getPeriod() <= now))) {
    //         myLastRerouteVehicle = veh;
    //         myLastRerouteTime = now;
    //
    // #ifdef DEBUG_REROUTE
    //         ConstMSEdgeVector oldRoute = veh->getRoute().getEdges();
    //         if (DEBUG_COND_LINKINFO) {
    //             std::cout << SIMTIME << " reroute veh=" << veh->getID() << " rs=" << getID() << " occupied=" << toString(occupied) << "\n";
    //         }
    // #endif
    //         MSRoutingEngine::Prohibitions prohibited;
    //         for (MSEdge* e : occupied) {
    //             // indefinite occupation because vehicles might be in deadlock on their current routes
    //             prohibited[e].end = std::numeric_limits<double>::max();
    //         }
    //         MSRoutingEngine::reroute(*veh, now, "railSignal:" + getID(), false, true, prohibited);
    // #ifdef DEBUG_REROUTE
    //         // attention this works only if we are not parallel!
    //         if (DEBUG_COND_LINKINFO) {
    //             if (veh->getRoute().getEdges() != oldRoute) {
    //                 std::cout << "    rerouting successful\n";
    //             }
    //         }
    // #endif
    //     }
    // }
    #[allow(unused_variables)]
    pub fn reroute(&mut self, veh: Ptr<SUMOVehicle>, occupied: &MSEdgeVector) {
        let r_dev = veh.cast::<MSBaseVehicle>().get_device("rerouting");
        let now = MSNet::get_instance().get_current_time_step();
        if !r_dev.is_null() {
            let r_dev: Ptr<MSDevice_Routing> = r_dev.cast();
            if r_dev.may_reroute_rail_signal()
                && (self.myLastRerouteVehicle != veh
                    // reroute each vehicle only once if no periodic routing is allowed,
                    // otherwise with the specified period
                    || (r_dev.get_period() > 0 && self.myLastRerouteTime + r_dev.get_period() <= now))
            {
                self.myLastRerouteVehicle = veh;
                self.myLastRerouteTime = now;

                let mut prohibited: SUMOAbstractRouter_Prohibitions = BTreeMap::new();
                for e in occupied {
                    // indefinite occupation because vehicles might be in deadlock on their current routes
                    prohibited.insert(
                        e.cast::<()>(),
                        RouterProhibition {
                            begin: 0.,
                            end: f64::MAX,
                            permissions: SVC_IGNORING,
                        },
                    );
                }
                let info = "railSignal:".to_string() + &self.get_id();
                MSRoutingEngine::reroute(veh.m(), now, &info, false, true, &prohibited);
            }
        }
    }

    /// C++: `void LinkInfo::reset()`
    /// init LinkInfo
    // void
    // MSRailSignal::LinkInfo::reset() {
    //     myLastRerouteTime = -1;
    //     myLastRerouteVehicle = nullptr;
    //     myDriveways.clear();
    //     myControlled = (myLink->getViaLaneOrLane()->getPermissions() & myLink->getLane()->getPermissions() & SVC_RAIL_CLASSES) != 0;
    // }
    #[allow(unused_variables)]
    pub fn reset(&mut self) {
        self.myLastRerouteTime = -1;
        self.myLastRerouteVehicle = Ptr::null();
        self.myDriveways.clear();
        self.myControlled = (self.myLink.get_via_lane_or_lane().get_permissions()
            & self.myLink.get_lane().get_permissions()
            & SVC_RAIL_CLASSES)
            != 0;
    }

}

/// A signal for rails
/// C++ class `MSRailSignal` (microsim/traffic_lights/MSRailSignal.h)
#[repr(C)]
pub struct MSRailSignal {
    /// C++ base class `MSTrafficLightLogic`
    pub base: MSTrafficLightLogic,
    pub myNumericalID: i32,
    /// data storage for every link at this node (more than one when directly guarding a switch)
    pub myLinkInfos: Vec<MSRailSignal_LinkInfo>,
    /// The list of phases this logic uses
    pub myPhases: MSTrafficLightLogic_Phases,
    /// The current phase
    pub myCurrentPhase: MSPhaseDefinition,
    /// MSTrafficLightLogic requires that the phase index changes whenever signals change their state
    pub myPhaseIndex: i32,
    /// whether the signal is in moving block mode (only protects from oncoming and flanking trains)
    pub myMovingBlock: bool,
    /// map from tripId to constraint list
    pub myConstraints: BTreeMap<String, Vec<Ptr<MSRailSignalConstraint>>>,
    /// running number of driveways created for this signal
    pub myDriveWayIndex: i32,
}

impl std::ops::Deref for MSRailSignal {
    type Target = MSTrafficLightLogic;
    fn deref(&self) -> &MSTrafficLightLogic {
        &self.base
    }
}
impl std::ops::DerefMut for MSRailSignal {
    fn deref_mut(&mut self) -> &mut MSTrafficLightLogic {
        &mut self.base
    }
}

// C++ static members of MSRailSignal (port as module-level statics, see AGENTS.md):
//   static int MSRailSignal::myRSIndex;
//   static bool MSRailSignal::myStoreVehicles;
//   static VehicleVector MSRailSignal::myBlockingVehicles;
//   static VehicleVector MSRailSignal::myRivalVehicles;
//   static VehicleVector MSRailSignal::myPriorityVehicles;
//   static std::string MSRailSignal::myConstraintInfo;
//   static std::vector<const MSDriveWay *> MSRailSignal::myBlockingDriveWays;
//   static std::string MSRailSignal::myRequestedDriveWay;

thread_local! {
    // static int MSRailSignal::myRSIndex
    static MY_RS_INDEX: Cell<i32> = const { Cell::new(0) };
    // static bool MSRailSignal::myStoreVehicles
    static MY_STORE_VEHICLES: Cell<bool> = const { Cell::new(false) };
    // static VehicleVector MSRailSignal::myBlockingVehicles
    static MY_BLOCKING_VEHICLES: RefCell<Ptr<MSVehicleContainer_VehicleVector>> =
        RefCell::new(Ptr::from_raw(Box::into_raw(Box::new(Vec::new()))));
    // static VehicleVector MSRailSignal::myRivalVehicles
    static MY_RIVAL_VEHICLES: RefCell<Ptr<MSVehicleContainer_VehicleVector>> =
        RefCell::new(Ptr::from_raw(Box::into_raw(Box::new(Vec::new()))));
    // static VehicleVector MSRailSignal::myPriorityVehicles
    static MY_PRIORITY_VEHICLES: RefCell<Ptr<MSVehicleContainer_VehicleVector>> =
        RefCell::new(Ptr::from_raw(Box::into_raw(Box::new(Vec::new()))));
    // static std::string MSRailSignal::myConstraintInfo
    static MY_CONSTRAINT_INFO: RefCell<Ptr<String>> = RefCell::new(Ptr::from_raw(Box::into_raw(Box::new(String::new()))));
    // static std::vector<const MSDriveWay *> MSRailSignal::myBlockingDriveWays
    static MY_BLOCKING_DRIVE_WAYS: RefCell<Ptr<Vec<Ptr<MSDriveWay>>>> =
        RefCell::new(Ptr::from_raw(Box::into_raw(Box::new(Vec::new()))));
    // static std::string MSRailSignal::myRequestedDriveWay
    static MY_REQUESTED_DRIVE_WAY: RefCell<Ptr<String>> = RefCell::new(Ptr::from_raw(Box::into_raw(Box::new(String::new()))));
}

/// static bool MSRailSignal::myStoreVehicles
fn my_store_vehicles() -> bool {
    MY_STORE_VEHICLES.with(|c| c.get())
}

/// sets static bool MSRailSignal::myStoreVehicles
fn set_my_store_vehicles(value: bool) {
    MY_STORE_VEHICLES.with(|c| c.set(value));
}

/// static VehicleVector MSRailSignal::myBlockingVehicles
fn my_blocking_vehicles() -> &'static mut MSVehicleContainer_VehicleVector {
    unsafe { &mut *MY_BLOCKING_VEHICLES.with(|c| *c.borrow()).as_raw() }
}

/// static VehicleVector MSRailSignal::myRivalVehicles
fn my_rival_vehicles() -> &'static mut MSVehicleContainer_VehicleVector {
    unsafe { &mut *MY_RIVAL_VEHICLES.with(|c| *c.borrow()).as_raw() }
}

/// static VehicleVector MSRailSignal::myPriorityVehicles
fn my_priority_vehicles() -> &'static mut MSVehicleContainer_VehicleVector {
    unsafe { &mut *MY_PRIORITY_VEHICLES.with(|c| *c.borrow()).as_raw() }
}

/// static std::string MSRailSignal::myConstraintInfo
fn my_constraint_info() -> &'static mut String {
    unsafe { &mut *MY_CONSTRAINT_INFO.with(|c| *c.borrow()).as_raw() }
}

/// static std::vector<const MSDriveWay *> MSRailSignal::myBlockingDriveWays
fn my_blocking_drive_ways() -> &'static mut Vec<Ptr<MSDriveWay>> {
    unsafe { &mut *MY_BLOCKING_DRIVE_WAYS.with(|c| *c.borrow()).as_raw() }
}

/// static std::string MSRailSignal::myRequestedDriveWay
fn my_requested_drive_way() -> &'static mut String {
    unsafe { &mut *MY_REQUESTED_DRIVE_WAY.with(|c| *c.borrow()).as_raw() }
}

/// toString(VehicleVector): the vehicle ids joined by " "
fn vehicle_vector_to_string(vehicles: &MSVehicleContainer_VehicleVector) -> String {
    vehicles.iter().map(|v| v.get_id().to_string()).collect::<Vec<_>>().join(" ")
}

/// toString(std::vector<const MSDriveWay *>): the driveway ids joined by " "
fn drive_way_vector_to_string(driveways: &Vec<Ptr<MSDriveWay>>) -> String {
    driveways.iter().map(|d| d.base2.get_id().to_string()).collect::<Vec<_>>().join(" ")
}

fn rail_try_switch(l: &mut MSTrafficLightLogic) -> SUMOTime {
    Ptr::from_mut(l).cast::<MSRailSignal>().m().try_switch()
}
fn rail_get_phase_number(l: &MSTrafficLightLogic) -> i32 {
    Ptr::new(l).cast::<MSRailSignal>().get().unwrap().get_phase_number()
}
fn rail_get_phases(l: &MSTrafficLightLogic) -> &MSTrafficLightLogic_Phases {
    Ptr::new(l).cast::<MSRailSignal>().get().unwrap().get_phases()
}
fn rail_get_phase(l: &MSTrafficLightLogic, givenstep: i32) -> &MSPhaseDefinition {
    Ptr::new(l).cast::<MSRailSignal>().get().unwrap().get_phase(givenstep)
}
fn rail_get_current_phase_index(l: &MSTrafficLightLogic) -> i32 {
    Ptr::new(l).cast::<MSRailSignal>().get().unwrap().get_current_phase_index()
}
fn rail_get_current_phase_def(l: &MSTrafficLightLogic) -> &MSPhaseDefinition {
    Ptr::new(l).cast::<MSRailSignal>().get().unwrap().get_current_phase_def()
}
fn rail_reset_last_switch(l: &mut MSTrafficLightLogic, t: SUMOTime) {
    Ptr::from_mut(l).cast::<MSRailSignal>().m().reset_last_switch(t)
}
fn rail_get_phase_index_at_time(l: &MSTrafficLightLogic, simStep: SUMOTime) -> SUMOTime {
    Ptr::new(l).cast::<MSRailSignal>().get().unwrap().get_phase_index_at_time(simStep)
}
fn rail_get_offset_from_index(l: &MSTrafficLightLogic, index: i32) -> SUMOTime {
    Ptr::new(l).cast::<MSRailSignal>().get().unwrap().get_offset_from_index(index)
}
fn rail_get_index_from_offset(l: &MSTrafficLightLogic, offset: SUMOTime) -> i32 {
    Ptr::new(l).cast::<MSRailSignal>().get().unwrap().get_index_from_offset(offset)
}
fn rail_change_step_and_duration(l: &mut MSTrafficLightLogic, tlcontrol: &mut MSTLLogicControl, simStep: SUMOTime, step: i32, stepDuration: SUMOTime) {
    Ptr::from_mut(l).cast::<MSRailSignal>().m().change_step_and_duration(tlcontrol, simStep, step, stepDuration)
}
fn rail_add_link(l: &mut MSTrafficLightLogic, link: Ptr<MSLink>, lane: Ptr<MSLane>, pos: i32) {
    Ptr::from_mut(l).cast::<MSRailSignal>().m().add_link(link, lane, pos)
}
fn rail_init(l: &mut MSTrafficLightLogic, nb: &mut NLDetectorBuilder) {
    Ptr::from_mut(l).cast::<MSRailSignal>().m().init(nb)
}
fn rail_adapt_link_information_from(l: &mut MSTrafficLightLogic, logic: &MSTrafficLightLogic) {
    Ptr::from_mut(l).cast::<MSRailSignal>().m().adapt_link_information_from(logic)
}
fn rail_gets_major_green(l: &MSTrafficLightLogic, linkIndex: i32) -> bool {
    Ptr::new(l).cast::<MSRailSignal>().get().unwrap().gets_major_green(linkIndex)
}
fn rail_get_blocking_vehicles(l: &mut MSTrafficLightLogic, linkIndex: i32) -> MSVehicleContainer_VehicleVector {
    Ptr::from_mut(l).cast::<MSRailSignal>().m().get_blocking_vehicles(linkIndex)
}

fn rail_get_rival_vehicles(l: &mut MSTrafficLightLogic, linkIndex: i32) -> MSVehicleContainer_VehicleVector {
    Ptr::from_mut(l).cast::<MSRailSignal>().m().get_rival_vehicles(linkIndex)
}

fn rail_get_priority_vehicles(l: &mut MSTrafficLightLogic, linkIndex: i32) -> MSVehicleContainer_VehicleVector {
    Ptr::from_mut(l).cast::<MSRailSignal>().m().get_priority_vehicles(linkIndex)
}

fn rail_get_blocking_drive_ways(l: &mut MSTrafficLightLogic, linkIndex: i32) -> Vec<Ptr<MSDriveWay>> {
    Ptr::from_mut(l).cast::<MSRailSignal>().m().get_blocking_drive_ways(linkIndex)
}

fn rail_get_requested_drive_way(l: &mut MSTrafficLightLogic, linkIndex: i32) -> String {
    Ptr::from_mut(l).cast::<MSRailSignal>().m().get_requested_drive_way(linkIndex)
}

fn rail_ext_set_parameter(l: &mut MSTrafficLightLogic, key: &str, value: &str) {
    Ptr::from_mut(l).cast::<MSRailSignal>().m().set_parameter(key, value)
}

static MS_RAIL_SIGNAL_TRAFFIC_LIGHT_LOGIC_EXT_VT: super::ms_traffic_light_logic::MSTrafficLightLogicExtVt = super::ms_traffic_light_logic::MSTrafficLightLogicExtVt {
    get_rival_vehicles: rail_get_rival_vehicles,
    get_priority_vehicles: rail_get_priority_vehicles,
    get_blocking_drive_ways: rail_get_blocking_drive_ways,
    get_requested_drive_way: rail_get_requested_drive_way,
    get_detector_state: super::ms_traffic_light_logic::MSTrafficLightLogic::get_detector_state_base,
    get_tl_queue_length: super::ms_traffic_light_logic::MSTrafficLightLogic::get_tl_queue_length_base,
    load_extra_state: super::ms_traffic_light_logic::default_ext_load_extra_state,
    get_parameter: super::ms_traffic_light_logic::default_ext_get_parameter,
    set_parameter: rail_ext_set_parameter,
    activate_program: super::ms_traffic_light_logic::default_ext_activate_program,
    deactivate_program: super::ms_traffic_light_logic::default_ext_deactivate_program,
};

static MS_RAIL_SIGNAL_TRAFFIC_LIGHT_LOGIC_VT: MSTrafficLightLogicVt = MSTrafficLightLogicVt {
    try_switch: rail_try_switch,
    get_phase_number: rail_get_phase_number,
    get_phases: rail_get_phases,
    get_phase: rail_get_phase,
    get_current_phase_index: rail_get_current_phase_index,
    get_current_phase_def: rail_get_current_phase_def,
    reset_last_switch: rail_reset_last_switch,
    get_phase_index_at_time: rail_get_phase_index_at_time,
    get_offset_from_index: rail_get_offset_from_index,
    get_index_from_offset: rail_get_index_from_offset,
    change_step_and_duration: rail_change_step_and_duration,
    add_link: rail_add_link,
    init: rail_init,
    adapt_link_information_from: rail_adapt_link_information_from,
    gets_major_green: rail_gets_major_green,
    get_detector_states: MSTrafficLightLogic::get_detector_states_base,
    get_conditions: MSTrafficLightLogic::get_conditions_base,
    get_blocking_vehicles: rail_get_blocking_vehicles,
    save_state: MSTrafficLightLogic::save_state_base,
    load_state: MSTrafficLightLogic::load_state_base,
    map_time_in_cycle: MSTrafficLightLogic::map_time_in_cycle_base,
    decide_next_phase: super::ms_traffic_light_logic::default_decide_next_phase,
    can_release: super::ms_traffic_light_logic::default_can_release,
    compute_return_time: super::ms_traffic_light_logic::default_compute_return_time,
};

impl MSRailSignal {
    /// C++: `void MSRailSignal::MSRailSignal(MSTLLogicControl &, const std::string &, const std::string &, SUMOTime, const Parameterised::Map &)`
    /// Constructor
    // MSRailSignal::MSRailSignal(MSTLLogicControl& tlcontrol,
    //                            const std::string& id, const std::string& programID, SUMOTime delay,
    //                            const Parameterised::Map& parameters) :
    //     MSTrafficLightLogic(tlcontrol, id, programID, 0, TrafficLightType::RAIL_SIGNAL, delay, parameters),
    //     myNumericalID(myRSIndex++),
    //     myCurrentPhase(DELTA_T, std::string(SUMO_MAX_CONNECTIONS, 'X')), // dummy phase
    //     myPhaseIndex(0),
    //     myDriveWayIndex(0) {
    //     myDefaultCycleTime = DELTA_T;
    //     myMovingBlock = OptionsCont::getOptions().getBool("railsignal-moving-block");
    //     mySwitchCommand->deschedule(this);
    // }
    #[allow(unused_variables)]
    pub fn new(tlcontrol: &mut MSTLLogicControl, id: &str, programID: &str, delay: SUMOTime, parameters: &ParameterisedMap) -> Self {
        let mut base = MSTrafficLightLogic::new(tlcontrol, id, programID, 0, TrafficLightType::RAIL_SIGNAL, delay, parameters);
        base.vt = &MS_RAIL_SIGNAL_TRAFFIC_LIGHT_LOGIC_VT;
        base.ext_vt = &MS_RAIL_SIGNAL_TRAFFIC_LIGHT_LOGIC_EXT_VT;
        let numerical_id = MY_RS_INDEX.with(|c| {
            let value = c.get();
            c.set(value + 1);
            value
        });
        let mut result = MSRailSignal {
            base,
            myNumericalID: numerical_id,
            myLinkInfos: Vec::new(),
            myPhases: Vec::new(),
            // dummy phase
            myCurrentPhase: MSPhaseDefinition::new(
                utils::common::time::delta_t(),
                &"X".repeat(utils::common::std_defs::SUMO_MAX_CONNECTIONS),
                "",
            ),
            myPhaseIndex: 0,
            myMovingBlock: false,
            myConstraints: BTreeMap::new(),
            myDriveWayIndex: 0,
        };
        result.base.myDefaultCycleTime = utils::common::time::delta_t();
        result.myMovingBlock = utils::options::options_cont::get_bool("railsignal-moving-block");
        // shortcut: the C++ constructor deschedules the switch command created by the base class;
        // the Rust base class does not create one (the caller must call schedule_switch_command),
        // and a descheduled command behaves exactly like a missing one (getNextSwitchTime == -1)
        result
    }

    /// C++: `void MSRailSignal::init(NLDetectorBuilder &)` [virtual]
    /// Initialises the rail signal with information about adjacent rail signals
    // void
    // MSRailSignal::init(NLDetectorBuilder&) {
    //     if (myLanes.size() == 0) {
    //         WRITE_WARNINGF(TL("Rail signal at junction '%' does not control any links"), getID());
    //     }
    //     SVCPermissions outgoingPermissions = 0;
    //     for (LinkVector& links : myLinks) { //for every link index
    //         if (links.size() != 1) {
    //             throw ProcessError("At railSignal '" + getID() + "' found " + toString(links.size())
    //                                + " links controlled by index " + toString(links[0]->getTLIndex()));
    //         }
    //         myLinkInfos.push_back(LinkInfo(links[0]));
    //         outgoingPermissions |= links[0]->getPermissions();
    //     }
    //     updateCurrentPhase();
    //     setTrafficLightSignals(MSNet::getInstance()->getCurrentTimeStep());
    //     myNumLinks = (int)myLinks.size();
    //     MSRailSignalControl::getInstance().addSignal(this);
    //     myMovingBlock |= MSRailSignalControl::isMovingBlock(outgoingPermissions);
    // }
    #[allow(unused_variables)]
    pub fn init(&mut self, nb: &mut NLDetectorBuilder) {
        if self.myLanes.is_empty() {
            utils::write_warningf!("Rail signal at junction '%' does not control any links", self.get_id());
        }
        let mut outgoing_permissions: SVCPermissions = 0;
        for links in self.myLinks.clone() {
            //for every link index
            if links.len() != 1 {
                panic!(
                    "{}",
                    ProcessError::new(format!(
                        "At railSignal '{}' found {} links controlled by index {}",
                        self.get_id(),
                        links.len(),
                        utils::common::to_string::to_string(&links[0].get_tl_index())
                    ))
                );
            }
            self.myLinkInfos.push(MSRailSignal_LinkInfo::new(links[0]));
            outgoing_permissions |= links[0].get_permissions();
        }
        self.update_current_phase();
        let now = MSNet::get_instance().get_current_time_step();
        self.set_traffic_light_signals(now);
        self.base.myNumLinks = self.myLinks.len() as i32;
        MSRailSignalControl::get_instance().add_signal(Ptr::new(&*self));
        self.myMovingBlock |= MSRailSignalControl::is_moving_block(outgoing_permissions);
    }

    /// C++: `void MSRailSignal::~MSRailSignal() noexcept` [virtual]
    /// Destructor
    // MSRailSignal::~MSRailSignal() {
    //     removeConstraints();
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        self.remove_constraints();
    }

    /// C++: `int MSRailSignal::getNumericalID() const` [const]
    // inline int getNumericalID() const {
    //         return myNumericalID;
    //     }
    #[allow(unused_variables)]
    pub fn get_numerical_id(&self) -> i32 {
        self.myNumericalID
    }

    /// C++: `void MSRailSignal::setParameter(const std::string &, const std::string &)` [virtual]
    /// Sets a parameter and updates internal constants
    // void
    // MSRailSignal::setParameter(const std::string& key, const std::string& value) {
    //     // some pre-defined parameters can be updated at runtime
    //     if (key == "moving-block") {
    //         bool movingBlock = StringUtils::toBool(value);
    //         if (movingBlock != myMovingBlock) {
    //             // recompute driveways
    //             myMovingBlock = movingBlock;
    //             for (LinkInfo& li : myLinkInfos) {
    //                 li.reset();
    //             }
    //             updateCurrentPhase();
    //             setTrafficLightSignals(MSNet::getInstance()->getCurrentTimeStep());
    //         }
    //     }
    //     Parameterised::setParameter(key, value);
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(&mut self, key: &str, value: &str) {
        // some pre-defined parameters can be updated at runtime
        if key == "moving-block" {
            let moving_block = utils::common::string_utils::to_bool(value).unwrap();
            if moving_block != self.myMovingBlock {
                // recompute driveways
                self.myMovingBlock = moving_block;
                for li in &mut self.myLinkInfos {
                    li.reset();
                }
                self.update_current_phase();
                let now = MSNet::get_instance().get_current_time_step();
                self.set_traffic_light_signals(now);
            }
        }
        self.base.base2.set_parameter(key, value);
    }

    /// C++: `void MSRailSignal::addLink(MSLink *, MSLane *, int)` [virtual]
    /// Adds a link on building
    // void
    // MSRailSignal::addLink(MSLink* link, MSLane* lane, int pos) {
    //     if (pos >= 0) {
    //         MSTrafficLightLogic::addLink(link, lane, pos);
    //     } // ignore uncontrolled link
    // }
    #[allow(unused_variables)]
    pub fn add_link(&mut self, link: Ptr<MSLink>, lane: Ptr<MSLane>, pos: i32) {
        if pos >= 0 {
            self.base.add_link_base(link, lane, pos);
        } // ignore uncontrolled link
    }

    /// C++: `void MSRailSignal::adaptLinkInformationFrom(const MSTrafficLightLogic &)` [virtual]
    /// Applies information about controlled links and lanes from the given logic
    // void
    // MSRailSignal::adaptLinkInformationFrom(const MSTrafficLightLogic& logic) {
    //     MSTrafficLightLogic::adaptLinkInformationFrom(logic);
    //     updateCurrentPhase();
    // }
    #[allow(unused_variables)]
    pub fn adapt_link_information_from(&mut self, logic: &MSTrafficLightLogic) {
        self.base.adapt_link_information_from_base(logic);
        self.update_current_phase();
    }

    /// C++: `bool MSRailSignal::updateCurrentPhase()`
    /// updates the current phase of the signal and return whether it should be kept active
    // bool
    // MSRailSignal::updateCurrentPhase() {
    // #ifdef DEBUG_SIGNALSTATE
    //     gDebugFlag4 = DEBUG_COND;
    // #endif
    //     bool keepActive = false;
    //     // green by default so vehicles can be inserted at the borders of the network
    //     std::string state(myLinks.size(), 'G');
    //     for (LinkInfo& li : myLinkInfos) {
    //         if (li.myLink->getApproaching().size() > 0 && li.myControlled) {
    //             keepActive = true;
    //             Approaching closest = li.myLink->getClosest();
    //             MSDriveWay& driveway = li.getDriveWay(closest.first);
    //             //std::cout << SIMTIME << " signal=" << getTLLinkID(li.myLink) << " veh=" << closest.first->getID() << " dw:\n";
    //             //driveway.writeBlocks(*OutputDevice_COUT::getDevice());
    //             const bool mustWait = !constraintsAllow(closest.first, true);
    //             MSEdgeVector occupied;
    //             if (mustWait || !driveway.reserve(closest, occupied)) {
    //                 state[li.myLink->getTLIndex()] = 'r';
    //                 if (occupied.size() > 0) {
    //                     li.reroute(const_cast<SUMOVehicle*>(closest.first), occupied);
    //                 }
    // #ifdef DEBUG_SIGNALSTATE
    //                 if (gDebugFlag4) {
    //                     std::cout << SIMTIME << " rsl=" << li.getID() << " veh=" << closest.first->getID() << " notReserved\n";
    //                 }
    // #endif
    //             } else {
    //                 state[li.myLink->getTLIndex()] = 'G';
    // #ifdef DEBUG_SIGNALSTATE
    //                 if (gDebugFlag4) {
    //                     std::cout << SIMTIME << " rsl=" << li.getID() << " veh=" << closest.first->getID() << " reserved\n";
    //                 }
    // #endif
    //             }
    //         } else if (li.myControlled) {
    //             if (li.myDriveways.empty()) {
    // #ifdef DEBUG_SIGNALSTATE
    //                 if (gDebugFlag4) {
    //                     std::cout << SIMTIME << " rsl=" << li.getID() << " red for unitialized signal (no driveways yet)\n";
    //                 }
    // #endif
    //                 state[li.myLink->getTLIndex()] = 'r';
    //             } else {
    //                 const MSDriveWay& driveway = *li.myDriveways.front();
    //                 MSEdgeVector occupied;
    //                 if (driveway.foeDriveWayOccupied(true, nullptr, occupied) || driveway.foeDriveWayApproached()) {
    //                     keepActive = true;
    // #ifdef DEBUG_SIGNALSTATE
    //                     if (gDebugFlag4) {
    //                         std::cout << SIMTIME << " rsl=" << li.getID() << " red for default driveway " << driveway.getID() << "\n";
    //                     }
    // #endif
    //                     state[li.myLink->getTLIndex()] = 'r';
    //                 } else {
    // #ifdef DEBUG_SIGNALSTATE
    //                     if (gDebugFlag4) {
    //                         std::cout << SIMTIME << " rsl=" << li.getID() << " green for default driveway " << driveway.getID() << "\n";
    //                     }
    // #endif
    //                 }
    //             }
    //         } else {
    //             state[li.myLink->getTLIndex()] = 'O';
    //         }
    //     }
    //     if (myCurrentPhase.getState() != state) {
    //         myCurrentPhase.setState(state);
    //         myPhaseIndex = 1 - myPhaseIndex;
    //         // set link priorities
    //         setTrafficLightSignals(SIMSTEP);
    //         // execute switch actions (3D-gui)
    //         //const MSTLLogicControl::TLSLogicVariants& vars = myTLControl.get(myTLLogic->getID());
    //         //vars.executeOnSwitchActions();
    //     }
    // #ifdef DEBUG_SIGNALSTATE
    //     gDebugFlag4 = false;
    // #endif
    //     return keepActive;
    // }
    #[allow(unused_variables)]
    pub fn update_current_phase(&mut self) -> bool {
        let mut keep_active = false;
        // green by default so vehicles can be inserted at the borders of the network
        let mut state: Vec<char> = vec!['G'; self.myLinks.len()];
        for i in 0..self.myLinkInfos.len() {
            let (has_approaching, controlled, tl_index) = {
                let li = &self.myLinkInfos[i];
                (li.myLink.get_approaching_2().len() > 0, li.myControlled, li.myLink.get_tl_index())
            };
            if has_approaching && controlled {
                keep_active = true;
                let closest = self.myLinkInfos[i].myLink.get_closest();
                //std::cout << SIMTIME << " signal=" << getTLLinkID(li.myLink) << " veh=" << closest.first->getID() << " dw:\n";
                //driveway.writeBlocks(*OutputDevice_COUT::getDevice());
                let must_wait = !self.constraints_allow(closest.0, true);
                let mut occupied: MSEdgeVector = Vec::new();
                let reserved = {
                    let li = &mut self.myLinkInfos[i];
                    li.get_drive_way(closest.0, -1).reserve(&closest, &mut occupied)
                };
                if must_wait || !reserved {
                    state[tl_index as usize] = 'r';
                    if occupied.len() > 0 {
                        self.myLinkInfos[i].reroute(closest.0, &occupied);
                    }
                } else {
                    state[tl_index as usize] = 'G';
                }
            } else if controlled {
                if self.myLinkInfos[i].myDriveways.is_empty() {
                    state[tl_index as usize] = 'r';
                } else {
                    let mut occupied: MSEdgeVector = Vec::new();
                    let driveway = self.myLinkInfos[i].myDriveways[0];
                    if driveway.foe_drive_way_occupied(true, Ptr::null(), &mut occupied)
                        || driveway.foe_drive_way_approached()
                    {
                        keep_active = true;
                        state[tl_index as usize] = 'r';
                    }
                }
            } else {
                state[tl_index as usize] = 'O';
            }
        }
        let state: String = state.into_iter().collect();
        if self.myCurrentPhase.get_state() != state {
            self.myCurrentPhase.set_state(&state);
            self.myPhaseIndex = 1 - self.myPhaseIndex;
            // set link priorities
            let now = MSNet::get_instance().get_current_time_step();
            self.set_traffic_light_signals(now);
            // execute switch actions (3D-gui)
            //const MSTLLogicControl::TLSLogicVariants& vars = myTLControl.get(myTLLogic->getID());
            //vars.executeOnSwitchActions();
        }
        keep_active
    }

    /// C++: `SUMOTime MSRailSignal::trySwitch()` [virtual]
    /// Switches to the next phase
    // SUMOTime
    // MSRailSignal::trySwitch() {
    //     // deschedule regular traffic light event,
    //     // updateCurrentPhase is instead called from MSRailSignalControl::updateSignals
    //     return SUMOTime_MAX;
    // }
    #[allow(unused_variables)]
    pub fn try_switch(&mut self) -> SUMOTime {
        // deschedule regular traffic light event,
        // updateCurrentPhase is instead called from MSRailSignalControl::updateSignals
        utils::common::time::SUMOTIME_MAX
    }

    /// C++: `int MSRailSignal::getPhaseNumber() const` [const, virtual]
    /// Returns the number of phases
    // int
    // MSRailSignal::getPhaseNumber() const {
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn get_phase_number(&self) -> i32 {
        0
    }

    /// C++: `const Phases & MSRailSignal::getPhases() const` [const, virtual]
    /// Returns the phases of this tls program
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_phases(&self) -> &MSTrafficLightLogic_Phases {
        &self.myPhases
    }

    /// C++: `const MSPhaseDefinition & MSRailSignal::getPhase(int) const` [const, virtual]
    /// Returns the definition of the phase from the given position within the plan
    // const MSPhaseDefinition&
    // MSRailSignal::getPhase(int) const {
    //     return myCurrentPhase;
    // }
    #[allow(unused_variables)]
    pub fn get_phase(&self, givenstep: i32) -> &MSPhaseDefinition {
        &self.myCurrentPhase
    }

    /// C++: `bool MSRailSignal::getsMajorGreen(int) const` [const, virtual]
    /// whether the given link index ever turns 'G'
    // bool getsMajorGreen(int /*linkIndex*/) const override {
    //         return true;
    //     }
    #[allow(unused_variables)]
    pub fn gets_major_green(&self, arg0: i32) -> bool {
        true
    }

    /// C++: `int MSRailSignal::getCurrentPhaseIndex() const` [const, virtual]
    /// Returns the current index within the program
    // int
    // MSRailSignal::getCurrentPhaseIndex() const {
    //     return myPhaseIndex;
    // }
    #[allow(unused_variables)]
    pub fn get_current_phase_index(&self) -> i32 {
        self.myPhaseIndex
    }

    /// C++: `const MSPhaseDefinition & MSRailSignal::getCurrentPhaseDef() const` [const, virtual]
    /// Returns the definition of the current phase
    // const MSPhaseDefinition&
    // MSRailSignal::getCurrentPhaseDef() const {
    //     return myCurrentPhase;
    // }
    #[allow(unused_variables)]
    pub fn get_current_phase_def(&self) -> &MSPhaseDefinition {
        &self.myCurrentPhase
    }

    /// C++: `void MSRailSignal::resetLastSwitch(SUMOTime)` [virtual]
    /// @}
    // void
    // MSRailSignal::resetLastSwitch(SUMOTime t) {
    //     myCurrentPhase.myLastSwitch = t;
    // }
    #[allow(unused_variables)]
    pub fn reset_last_switch(&mut self, t: SUMOTime) {
        self.myCurrentPhase.myLastSwitch = t;
    }

    /// C++: `SUMOTime MSRailSignal::getPhaseIndexAtTime(SUMOTime) const` [const, virtual]
    /// Returns the index of the logic at the given simulation step
    // SUMOTime
    // MSRailSignal::getPhaseIndexAtTime(SUMOTime) const {
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn get_phase_index_at_time(&self, simStep: SUMOTime) -> SUMOTime {
        0
    }

    /// C++: `SUMOTime MSRailSignal::getOffsetFromIndex(int) const` [const, virtual]
    /// Returns the position (start of a phase during a cycle) from of a given step
    // SUMOTime
    // MSRailSignal::getOffsetFromIndex(int) const {
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn get_offset_from_index(&self, index: i32) -> SUMOTime {
        0
    }

    /// C++: `int MSRailSignal::getIndexFromOffset(SUMOTime) const` [const, virtual]
    /// Returns the step (the phasenumber) of a given position of the cycle
    // int
    // MSRailSignal::getIndexFromOffset(SUMOTime) const {
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn get_index_from_offset(&self, offset: SUMOTime) -> i32 {
        0
    }

    /// C++: `void MSRailSignal::changeStepAndDuration(MSTLLogicControl &, SUMOTime, int, SUMOTime)` [virtual]
    /// Changes the current phase and her duration
    // void changeStepAndDuration(MSTLLogicControl& tlcontrol, SUMOTime simStep, int step, SUMOTime stepDuration) override {
    //         UNUSED_PARAMETER(tlcontrol);
    //         UNUSED_PARAMETER(simStep);
    //         UNUSED_PARAMETER(step);
    //         UNUSED_PARAMETER(stepDuration);
    //     }
    #[allow(unused_variables)]
    pub fn change_step_and_duration(&mut self, tlcontrol: &mut MSTLLogicControl, simStep: SUMOTime, step: i32, stepDuration: SUMOTime) {
        // UNUSED_PARAMETER(tlcontrol); UNUSED_PARAMETER(simStep);
        // UNUSED_PARAMETER(step); UNUSED_PARAMETER(stepDuration);
    }

    /// C++: `VehicleVector MSRailSignal::getBlockingVehicles(int)` [virtual]
    /// return vehicles that block the intersection/rail signal for vehicles that wish to pass the given linkIndex
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_blocking_vehicles(&mut self, linkIndex: i32) -> MSVehicleContainer_VehicleVector {
        self.store_tra_ci_vehicles(linkIndex);
        my_blocking_vehicles().clone()
    }

    /// C++: `std::string MSRailSignal::getBlockingVehicleIDs() const` [const]
    // std::string
    // MSRailSignal::getBlockingVehicleIDs() const {
    //     MSRailSignal* rs = const_cast<MSRailSignal*>(this);
    //     if (myLinkInfos.size() == 1) {
    //         return toString(rs->getBlockingVehicles(0));
    //     } else {
    //         std::string result;
    //         for (int i = 0; i < (int)myLinkInfos.size(); i++) {
    //             result += toString(i) + ": " + toString(rs->getBlockingVehicles(i)) + ";";
    //         }
    //         return result;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_blocking_vehicle_i_ds(&mut self) -> String {
        // the C++ code const_casts this to call the non-const getters
        let rs = Ptr::from_mut(self);
        if self.myLinkInfos.len() == 1 {
            vehicle_vector_to_string(&rs.m().get_blocking_vehicles(0))
        } else {
            let mut result = String::new();
            for i in 0..self.myLinkInfos.len() {
                result += &format!("{}: {};", i, vehicle_vector_to_string(&rs.m().get_blocking_vehicles(i as i32)));
            }
            result
        }
    }

    /// C++: `VehicleVector MSRailSignal::getBlockingVehicles(const MSDriveWay *)` [static]
    /// return vehicles that block the given driveWay
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_blocking_vehicles_2(dw: Ptr<MSDriveWay>) -> MSVehicleContainer_VehicleVector {
        MSRailSignal::store_tra_ci_vehicles_2(dw);
        my_blocking_vehicles().clone()
    }

    /// C++: `VehicleVector MSRailSignal::getRivalVehicles(int)` [virtual]
    /// return vehicles that approach the intersection/rail signal and are in conflict with vehicles that wish to pass the given linkIndex
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_rival_vehicles(&mut self, linkIndex: i32) -> MSVehicleContainer_VehicleVector {
        self.store_tra_ci_vehicles(linkIndex);
        my_rival_vehicles().clone()
    }

    /// C++: `std::string MSRailSignal::getRivalVehicleIDs() const` [const]
    // std::string
    // MSRailSignal::getRivalVehicleIDs() const {
    //     MSRailSignal* rs = const_cast<MSRailSignal*>(this);
    //     if (myLinkInfos.size() == 1) {
    //         return toString(rs->getRivalVehicles(0));
    //     } else {
    //         std::string result;
    //         for (int i = 0; i < (int)myLinkInfos.size(); i++) {
    //             result += toString(i) + ": " + toString(rs->getRivalVehicles(i)) + ";";
    //         }
    //         return result;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_rival_vehicle_i_ds(&mut self) -> String {
        // the C++ code const_casts this to call the non-const getters
        let rs = Ptr::from_mut(self);
        if self.myLinkInfos.len() == 1 {
            vehicle_vector_to_string(&rs.m().get_rival_vehicles(0))
        } else {
            let mut result = String::new();
            for i in 0..self.myLinkInfos.len() {
                result += &format!("{}: {};", i, vehicle_vector_to_string(&rs.m().get_rival_vehicles(i as i32)));
            }
            result
        }
    }

    /// C++: `VehicleVector MSRailSignal::getPriorityVehicles(int)` [virtual]
    /// return vehicles that approach the intersection/rail signal and have priority over vehicles that wish to pass the given linkIndex
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_priority_vehicles(&mut self, linkIndex: i32) -> MSVehicleContainer_VehicleVector {
        self.store_tra_ci_vehicles(linkIndex);
        my_priority_vehicles().clone()
    }

    /// C++: `std::string MSRailSignal::getPriorityVehicleIDs() const` [const]
    // std::string
    // MSRailSignal::getPriorityVehicleIDs() const {
    //     MSRailSignal* rs = const_cast<MSRailSignal*>(this);
    //     if (myLinkInfos.size() == 1) {
    //         return toString(rs->getPriorityVehicles(0));
    //     } else {
    //         std::string result;
    //         for (int i = 0; i < (int)myLinkInfos.size(); i++) {
    //             result += toString(i) + ": " + toString(rs->getPriorityVehicles(i)) + ";";
    //         }
    //         return result;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_priority_vehicle_i_ds(&mut self) -> String {
        // the C++ code const_casts this to call the non-const getters
        let rs = Ptr::from_mut(self);
        if self.myLinkInfos.len() == 1 {
            vehicle_vector_to_string(&rs.m().get_priority_vehicles(0))
        } else {
            let mut result = String::new();
            for i in 0..self.myLinkInfos.len() {
                result += &format!("{}: {};", i, vehicle_vector_to_string(&rs.m().get_priority_vehicles(i as i32)));
            }
            result
        }
    }

    /// C++: `std::string MSRailSignal::getConstraintInfo(int)`
    /// return information regarding active rail signal constraints for the closest approaching vehicle
    // std::string
    // MSRailSignal::getConstraintInfo(int linkIndex) {
    //     storeTraCIVehicles(linkIndex);
    //     return myConstraintInfo;
    // }
    #[allow(unused_variables)]
    pub fn get_constraint_info(&mut self, linkIndex: i32) -> String {
        self.store_tra_ci_vehicles(linkIndex);
        my_constraint_info().clone()
    }

    /// C++: `std::string MSRailSignal::getConstraintInfo() const` [const]
    // std::string
    // MSRailSignal::getConstraintInfo() const {
    //     MSRailSignal* rs = const_cast<MSRailSignal*>(this);
    //     if (myLinkInfos.size() == 1) {
    //         return rs->getConstraintInfo(0);
    //     } else {
    //         std::string result;
    //         for (int i = 0; i < (int)myLinkInfos.size(); i++) {
    //             result += toString(i) + ": " + rs->getConstraintInfo(i);
    //         }
    //         return result;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_constraint_info_2(&mut self) -> String {
        // the C++ code const_casts this to call the non-const getters
        let rs = Ptr::from_mut(self);
        if self.myLinkInfos.len() == 1 {
            rs.m().get_constraint_info(0)
        } else {
            let mut result = String::new();
            for i in 0..self.myLinkInfos.len() {
                result += &format!("{}: {}", i, rs.m().get_constraint_info(i as i32));
            }
            result
        }
    }

    /// C++: `std::vector<const MSDriveWay *> MSRailSignal::getBlockingDriveWays(int)` [virtual]
    // std::vector<const MSDriveWay*>
    // MSRailSignal::getBlockingDriveWays(int linkIndex) {
    //     storeTraCIVehicles(linkIndex);
    //     return myBlockingDriveWays;
    // }
    #[allow(unused_variables)]
    pub fn get_blocking_drive_ways(&mut self, linkIndex: i32) -> Vec<Ptr<MSDriveWay>> {
        self.store_tra_ci_vehicles(linkIndex);
        my_blocking_drive_ways().clone()
    }

    /// C++: `std::string MSRailSignal::getBlockingDriveWayIDs() const` [const]
    // std::string
    // MSRailSignal::getBlockingDriveWayIDs() const {
    //     MSRailSignal* rs = const_cast<MSRailSignal*>(this);
    //     if (myLinkInfos.size() == 1) {
    //         return toString(rs->getBlockingDriveWays(0));
    //     } else {
    //         std::string result;
    //         for (int i = 0; i < (int)myLinkInfos.size(); i++) {
    //             result += toString(i) + ": " + toString(rs->getBlockingDriveWays(i)) + ";";
    //         }
    //         return result;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_blocking_drive_way_i_ds(&mut self) -> String {
        // the C++ code const_casts this to call the non-const getters
        let rs = Ptr::from_mut(self);
        if self.myLinkInfos.len() == 1 {
            drive_way_vector_to_string(&rs.m().get_blocking_drive_ways(0))
        } else {
            let mut result = String::new();
            for i in 0..self.myLinkInfos.len() {
                result += &format!("{}: {};", i, drive_way_vector_to_string(&rs.m().get_blocking_drive_ways(i as i32)));
            }
            result
        }
    }

    /// C++: `std::vector<const MSDriveWay *> MSRailSignal::getBlockingDriveWays(const MSDriveWay *)` [static]
    /// return driveways that block the given driveWay
    // std::vector<const MSDriveWay*>
    // MSRailSignal::getBlockingDriveWays(const MSDriveWay* dw) {
    //     storeTraCIVehicles(dw);
    //     return myBlockingDriveWays;
    // }
    #[allow(unused_variables)]
    pub fn get_blocking_drive_ways_2(dw: Ptr<MSDriveWay>) -> Vec<Ptr<MSDriveWay>> {
        MSRailSignal::store_tra_ci_vehicles_2(dw);
        my_blocking_drive_ways().clone()
    }

    /// C++: `std::string MSRailSignal::getRequestedDriveWay(int)` [virtual]
    // std::string
    // MSRailSignal::getRequestedDriveWay(int linkIndex) {
    //     storeTraCIVehicles(linkIndex);
    //     return myRequestedDriveWay;
    // }
    #[allow(unused_variables)]
    pub fn get_requested_drive_way(&mut self, linkIndex: i32) -> String {
        self.store_tra_ci_vehicles(linkIndex);
        my_requested_drive_way().clone()
    }

    /// C++: `std::string MSRailSignal::getRequestedDriveWay() const` [const]
    // std::string
    // MSRailSignal::getRequestedDriveWay() const {
    //     MSRailSignal* rs = const_cast<MSRailSignal*>(this);
    //     if (myLinkInfos.size() == 1) {
    //         return toString(rs->getRequestedDriveWay(0));
    //     } else {
    //         std::string result;
    //         for (int i = 0; i < (int)myLinkInfos.size(); i++) {
    //             result += toString(i) + ": " + toString(rs->getRequestedDriveWay(i)) + ";";
    //         }
    //         return result;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_requested_drive_way_2(&mut self) -> String {
        // the C++ code const_casts this to call the non-const getters
        let rs = Ptr::from_mut(self);
        if self.myLinkInfos.len() == 1 {
            rs.m().get_requested_drive_way(0)
        } else {
            let mut result = String::new();
            for i in 0..self.myLinkInfos.len() {
                result += &format!("{}: {};", i, rs.m().get_requested_drive_way(i as i32));
            }
            result
        }
    }

    /// C++: `void MSRailSignal::writeBlocks(OutputDevice &, bool) const` [const]
    /// write rail signal block output for all links and driveways
    // void
    // MSRailSignal::writeBlocks(OutputDevice& od, bool writeVehicles) const {
    //     od.openTag("railSignal");
    //     od.writeAttr(SUMO_ATTR_ID, getID());
    //     for (const LinkInfo& li : myLinkInfos) {
    //         MSLink* link = li.myLink;
    //         od.openTag("link");
    //         od.writeAttr(SUMO_ATTR_TLLINKINDEX, link->getTLIndex());
    //         od.writeAttr(SUMO_ATTR_FROM, link->getLaneBefore()->getID());
    //         od.writeAttr(SUMO_ATTR_TO, link->getViaLaneOrLane()->getID());
    //         for (const MSDriveWay* dw : li.myDriveways) {
    //             if (writeVehicles) {
    //                 dw->writeBlockVehicles(od);
    //             } else {
    //                 dw->writeBlocks(od);
    //             }
    //         }
    //         od.closeTag(); // link
    //     }
    //     od.closeTag(); // railSignal
    // }
    #[allow(unused_variables)]
    pub fn write_blocks(&self, od: &mut OutputDevice, writeVehicles: bool) {
        od.open_tag("railSignal");
        od.write_attr(SUMO_ATTR_ID, self.get_id());
        for li in &self.myLinkInfos {
            let link = li.myLink;
            od.open_tag("link");
            od.write_attr(SUMO_ATTR_TLLINKINDEX, &link.get_tl_index());
            od.write_attr(SUMO_ATTR_FROM, link.get_lane_before().get_id());
            od.write_attr(SUMO_ATTR_TO, link.get_via_lane_or_lane().get_id());
            for dw in &li.myDriveways {
                if writeVehicles {
                    dw.write_block_vehicles(od);
                } else {
                    dw.write_blocks(od);
                }
            }
            od.close_tag(""); // link
        }
        od.close_tag(""); // railSignal
    }

    /// C++: `void MSRailSignal::addConstraint(const std::string &, MSRailSignalConstraint *)`
    /// register constraint for signal switching
    // void
    // MSRailSignal::addConstraint(const std::string& tripId, MSRailSignalConstraint* constraint) {
    //     myConstraints[tripId].push_back(constraint);
    // }
    #[allow(unused_variables)]
    pub fn add_constraint(&mut self, tripId: &str, constraint: Ptr<MSRailSignalConstraint>) {
        self.myConstraints.entry(tripId.to_string()).or_default().push(constraint);
    }

    /// C++: `const std::map<std::string, std::vector<MSRailSignalConstraint *>> & MSRailSignal::getConstraints() const` [const]
    /// @{
    // const std::map<std::string, std::vector<MSRailSignalConstraint*> >&  getConstraints() const {
    //         return myConstraints;
    //     }
    #[allow(unused_variables)]
    pub fn get_constraints(&self) -> &BTreeMap<String, Vec<Ptr<MSRailSignalConstraint>>> {
        &self.myConstraints
    }

    /// C++: `bool MSRailSignal::removeConstraint(const std::string &, MSRailSignalConstraint *)`
    /// remove constraint for signal switching
    // bool
    // MSRailSignal::removeConstraint(const std::string& tripId, MSRailSignalConstraint* constraint) {
    //     if (myConstraints.count(tripId) != 0) {
    //         auto& constraints = myConstraints[tripId];
    //         auto it = std::find(constraints.begin(), constraints.end(), constraint);
    //         if (it != constraints.end()) {
    //             delete *it;
    //             constraints.erase(it);
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn remove_constraint(&mut self, tripId: &str, constraint: Ptr<MSRailSignalConstraint>) -> bool {
        if let Some(constraints) = self.myConstraints.get_mut(tripId) {
            if let Some(pos) = constraints.iter().position(|c| *c == constraint) {
                // the C++ code deletes the constraint here
                unsafe { drop(Box::from_raw(constraints[pos].as_raw())) };
                constraints.remove(pos);
                return true;
            }
        }
        false
    }

    /// C++: `void MSRailSignal::removeConstraints()`
    // void
    // MSRailSignal::removeConstraints() {
    //     for (auto item : myConstraints) {
    //         for (MSRailSignalConstraint* c : item.second) {
    //             delete c;
    //         }
    //     }
    //     myConstraints.clear();
    // }
    #[allow(unused_variables)]
    pub fn remove_constraints(&mut self) {
        for item in self.myConstraints.values() {
            for c in item {
                // the C++ code deletes the constraints here
                unsafe { drop(Box::from_raw(c.as_raw())) };
            }
        }
        self.myConstraints.clear();
    }

    /// C++: `void MSRailSignal::updateDriveway(int)`
    /// update driveway for extended deadlock protection
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn update_driveway(&mut self, numericalID: i32) {
        // C++ declares this method but never defines nor calls it
    }

    /// C++: `bool MSRailSignal::hasInsertionConstraint(MSLink *, const MSVehicle *, std::string &, bool &)` [static]
    // bool
    // MSRailSignal::hasInsertionConstraint(MSLink* link, const MSVehicle* veh, std::string& info, bool& isInsertionOrder) {
    //     if (link->getJunction() != nullptr && link->getJunction()->getType() == SumoXMLNodeType::RAIL_SIGNAL) {
    //         const MSRailSignal* rs = dynamic_cast<const MSRailSignal*>(link->getTLLogic());
    //         if (rs != nullptr && rs->myConstraints.size() > 0) {
    //             const std::string tripID = veh->getParameter().getParameter("tripId", veh->getID());
    //             auto it = rs->myConstraints.find(tripID);
    //             if (it != rs->myConstraints.end()) {
    //                 for (MSRailSignalConstraint* c : it->second) {
    //                     if (c->isInsertionConstraint() && !c->cleared()) {
    // #ifdef DEBUG_SIGNALSTATE
    //                         if (DEBUG_HELPER(rs)) {
    //                             std::cout << SIMTIME << " rsl=" << rs->getID() << " insertion constraint '" << c->getDescription() << "' for vehicle '" << veh->getID() << "' not cleared\n";
    //                         }
    // #endif
    //                         info = c->getDescription();
    //                         isInsertionOrder = c->getType() == MSRailSignalConstraint::ConstraintType::INSERTION_ORDER;
    //                         if (MSGlobals::gTimeToTeleportRSDeadlock > 0) {
    //                             const SUMOVehicle* foe = c->getFoe();
    //                             if (foe != nullptr) {
    //                                 MSRailSignalControl::getInstance().addWaitRelation(veh, rs, foe, c);
    //                             }
    //                         }
    //                         return true;
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn has_insertion_constraint(link: Ptr<MSLink>, veh: Ptr<MSVehicle>, info: &mut String, isInsertionOrder: &mut bool) -> bool {
        let junction = link.get_junction();
        if !junction.is_null() && junction.get_type() == SumoXMLNodeType::RAIL_SIGNAL {
            // the link is controlled by a rail signal
            let rs: Ptr<MSRailSignal> = link.get_tl_logic().cast::<MSRailSignal>();
            if !rs.is_null() && rs.myConstraints.len() > 0 {
                let trip_id = veh
                    .cast::<MSBaseVehicle>()
                    .get_parameter()
                    .get_parameter("tripId", veh.get_id());
                if let Some(constraints) = rs.myConstraints.get(&trip_id) {
                    for c in constraints {
                        if c.is_insertion_constraint() && !c.cleared() {
                            *info = c.get_description();
                            *isInsertionOrder = c.get_type() == MSRailSignalConstraint_ConstraintType::INSERTION_ORDER;
                            if MSGlobals::time_to_teleport_rs_deadlock() > 0 {
                                let foe = c.get_foe();
                                if !foe.is_null() {
                                    MSRailSignalControl::get_instance().add_wait_relation(veh.cast::<SUMOVehicle>(), rs, foe, *c);
                                }
                            }
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    /// C++: `void MSRailSignal::initDriveWays(const SUMOVehicle *, bool)` [static]
    // void
    // MSRailSignal::initDriveWays(const SUMOVehicle* ego, bool update) {
    //     const ConstMSEdgeVector& edges = ego->getRoute().getEdges();
    //     int endIndex = ego->getParameter().arrivalEdge;
    //     if (endIndex < 0) {
    //         endIndex = (int)edges.size() - 1;
    //     }
    //     int departIndex = ego->getParameter().departEdge;
    //     MSDriveWay* prev = nullptr;
    //     if (update && ego->hasDeparted()) {
    //         // find last rail signal on the route and obtain the driveway
    //         const MSEdge* next = ego->getEdge();
    //         for (int i = ego->getRoutePosition() - 1; i > departIndex; i--) {
    //             const MSEdge* e = ego->getRoute().getEdges()[i];
    //             if (e->getToJunction()->getType() == SumoXMLNodeType::RAIL_SIGNAL) {
    //                 const MSLink* link = e->getLanes().front()->getLinkTo(next->getLanes().front());
    //                 //std::cout << SIMTIME << " veh=" << ego->getID() << " rp=" << ego->getRoutePosition()
    //                 //    << " i=" << i << " e=" << e->getID() << " next=" << next->getID() << " link=" << (link == nullptr ? "NUL" : link->getDescription()) << "\n";
    //                 if (link != nullptr && link->isTLSControlled()) {
    //                     MSRailSignal* rs = const_cast<MSRailSignal*>(dynamic_cast<const MSRailSignal*>(link->getTLLogic()));
    //                     LinkInfo& li = rs->myLinkInfos[link->getTLIndex()];
    //                     prev = &li.getDriveWay(ego, i);
    //                     departIndex = ego->getRoutePosition();
    //                     break;
    //                 }
    //             }
    //             next = e;
    //         }
    //     }
    //     if (prev == nullptr) {
    //         prev = const_cast<MSDriveWay*>(MSDriveWay::getDepartureDriveway(ego, true));
    //     }
    //     if (update && ego->hasDeparted()) {
    //         MSBaseVehicle* veh = dynamic_cast<MSBaseVehicle*>(const_cast<SUMOVehicle*>(ego));
    //         if (!prev->hasTrain(veh) && prev->notifyEnter(*veh, prev->NOTIFICATION_REROUTE, nullptr) && !veh->hasReminder(prev)) {
    //             veh->addReminder(prev, 1);
    //         }
    //     }
    //     for (int i = departIndex; i <= endIndex - 1; i++) {
    //         const MSEdge* e = edges[i];
    //         if (e->isNormal() && e->getToJunction()->getType() == SumoXMLNodeType::RAIL_SIGNAL) {
    //             const MSEdge* e2 = edges[i + 1];
    //             for (MSLane* lane : e->getLanes()) {
    //                 for (MSLink* link : lane->getLinkCont()) {
    //                     if (&link->getLane()->getEdge() == e2) {
    //                         MSRailSignal* rs = const_cast<MSRailSignal*>(dynamic_cast<const MSRailSignal*>(link->getTLLogic()));
    //                         if (rs != nullptr) {
    //                             LinkInfo& li = rs->myLinkInfos[link->getTLIndex()];
    //                             // init driveway
    //                             MSDriveWay* dw = &li.getDriveWay(ego, i);
    //                             MSRailSignalControl::getInstance().addDrivewayFollower(prev, dw);
    //                             MSRailSignalControl::getInstance().addDWDeadlockChecks(rs, prev);
    //                             MSRailSignalControl::getInstance().notifyApproach(link);
    //                             prev = dw;
    //                             if (update && rs->isActive()) {
    //                                 // vehicle may have rerouted its intial trip
    //                                 // after the states have been set
    //                                 // @note: This is a hack because it could lead to invalid tls-output
    //                                 // (it's still an improvement over switching based on default driveways)
    //                                 if (!ego->hasDeparted()) {
    //                                     rs->updateCurrentPhase();
    //                                     rs->setTrafficLightSignals(SIMSTEP);
    //                                 } else if (ego->hasDeparted() && i <= ego->getRoutePosition()) {
    //                                     MSBaseVehicle* veh = dynamic_cast<MSBaseVehicle*>(const_cast<SUMOVehicle*>(ego));
    //                                     if (!dw->hasTrain(veh) && dw->notifyEnter(*veh, dw->NOTIFICATION_REROUTE, nullptr) && !veh->hasReminder(dw)) {
    //                                         veh->addReminder(dw, 1);
    //                                         for (MSDriveWay* sub : dw->getSubDriveWays()) {
    //                                             if (!sub->hasTrain(veh) && sub->notifyEnter(*veh, dw->NOTIFICATION_REROUTE, nullptr) && !veh->hasReminder(sub)) {
    //                                                 veh->addReminder(sub, 1);
    //                                             }
    //                                         }
    //                                     }
    //                                 }
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn init_drive_ways(ego: Ptr<SUMOVehicle>, update: bool) {
        let veh = ego.cast::<MSBaseVehicle>();
        let edges = veh.get_route().get_edges();
        let mut end_index = veh.get_parameter().arrivalEdge;
        if end_index < 0 {
            end_index = edges.len() as i32 - 1;
        }
        let mut depart_index = veh.get_parameter().departEdge;
        let mut prev: Ptr<MSDriveWay> = Ptr::null();
        if update && veh.has_departed() {
            // find last rail signal on the route and obtain the driveway
            let mut next = veh.get_edge();
            let mut i = veh.get_route_position() - 1;
            while i > depart_index {
                let e = edges[i as usize];
                if e.get_to_junction().get_type() == SumoXMLNodeType::RAIL_SIGNAL {
                    let link = e
                        .get_lanes()
                        .first()
                        .unwrap()
                        .get_link_to(*next.get_lanes().first().unwrap());
                    //std::cout << SIMTIME << " veh=" << ego->getID() << " rp=" << ego->getRoutePosition()
                    //    << " i=" << i << " e=" << e->getID() << " next=" << next->getID() << " link=" << (link == nullptr ? "NUL" : link->getDescription()) << "\n";
                    if !link.is_null() && link.is_tls_controlled() {
                        let rs: Ptr<MSRailSignal> = link.get_tl_logic().cast::<MSRailSignal>();
                        let driveway = rs.m().myLinkInfos[link.get_tl_index() as usize].get_drive_way(ego, i);
                        prev = Ptr::new(driveway);
                        depart_index = veh.get_route_position();
                        break;
                    }
                }
                next = e;
                i -= 1;
            }
        }
        if prev.is_null() {
            prev = MSDriveWay::get_departure_driveway(ego, true);
        }
        if update && veh.has_departed() {
            let veh_ref: &mut MSBaseVehicle = veh.m();
            if !prev.has_train(ego)
                && prev.m().notify_enter(veh_ref, MSMoveReminder_Notification::NOTIFICATION_REROUTE, Ptr::null())
                && !veh_ref.has_reminder(prev.cast::<MSMoveReminder>())
            {
                veh_ref.add_reminder(prev.cast::<MSMoveReminder>(), 1.);
            }
        }
        for i in depart_index..=end_index - 1 {
            let e = edges[i as usize];
            if e.is_normal() && e.get_to_junction().get_type() == SumoXMLNodeType::RAIL_SIGNAL {
                let e2 = edges[(i + 1) as usize];
                for lane in e.get_lanes() {
                    for link in lane.get_link_cont() {
                        if link.get_lane().myEdge == e2 {
                            let rs: Ptr<MSRailSignal> = link.get_tl_logic().cast::<MSRailSignal>();
                            if !rs.is_null() {
                                // init driveway
                                let driveway = rs.m().myLinkInfos[link.get_tl_index() as usize].get_drive_way(ego, i);
                                let dw = Ptr::new(driveway);
                                MSRailSignalControl::get_instance().add_driveway_follower(prev, dw);
                                MSRailSignalControl::get_instance().add_dw_deadlock_checks(rs, prev);
                                MSRailSignalControl::get_instance().notify_approach(*link);
                                prev = dw;
                                if update && rs.is_active() {
                                    // vehicle may have rerouted its intial trip
                                    // after the states have been set
                                    // @note: This is a hack because it could lead to invalid tls-output
                                    // (it's still an improvement over switching based on default driveways)
                                    if !veh.has_departed() {
                                        rs.m().update_current_phase();
                                        let now = MSNet::get_instance().get_current_time_step();
                                        rs.m().set_traffic_light_signals(now);
                                    } else if i <= veh.get_route_position() {
                                        let veh_ref: &mut MSBaseVehicle = veh.m();
                                        if !dw.has_train(ego)
                                            && dw.m().notify_enter(veh_ref, MSMoveReminder_Notification::NOTIFICATION_REROUTE, Ptr::null())
                                            && !veh_ref.has_reminder(dw.cast::<MSMoveReminder>())
                                        {
                                            veh_ref.add_reminder(dw.cast::<MSMoveReminder>(), 1.);
                                            for sub in dw.get_sub_drive_ways() {
                                                if !sub.has_train(ego)
                                                    && sub.m().notify_enter(veh_ref, MSMoveReminder_Notification::NOTIFICATION_REROUTE, Ptr::null())
                                                    && !veh_ref.has_reminder(sub.cast::<MSMoveReminder>())
                                                {
                                                    veh_ref.add_reminder(sub.cast::<MSMoveReminder>(), 1.);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /// C++: `const MSDriveWay & MSRailSignal::retrieveDriveWay(int) const` [const]
    // const MSDriveWay&
    // MSRailSignal::retrieveDriveWay(int numericalID) const {
    //     for (const LinkInfo& li : myLinkInfos) {
    //         for (const MSDriveWay* dw : li.myDriveways) {
    //             if (dw->getNumericalID() == numericalID) {
    //                 return *dw;
    //             }
    //         }
    //     }
    //     throw ProcessError("Invalid driveway id " + toString(numericalID) + " at railSignal '" + getID() + "'");
    // }
    #[allow(unused_variables)]
    pub fn retrieve_drive_way(&self, numericalID: i32) -> &MSDriveWay {
        for li in &self.myLinkInfos {
            for dw in &li.myDriveways {
                if dw.get_numerical_id() == numericalID {
                    return dw.get().unwrap();
                }
            }
        }
        panic!(
            "{}",
            ProcessError::new(format!(
                "Invalid driveway id {} at railSignal '{}'",
                numericalID,
                self.get_id()
            ))
        );
    }

    /// C++: `const MSDriveWay & MSRailSignal::retrieveDriveWayForVeh(int, const SUMOVehicle *)`
    // const MSDriveWay&
    // MSRailSignal::retrieveDriveWayForVeh(int tlIndex, const SUMOVehicle* veh) {
    //     return myLinkInfos[tlIndex].getDriveWay(veh);
    // }
    #[allow(unused_variables)]
    pub fn retrieve_drive_way_for_veh(&mut self, tlIndex: i32, veh: Ptr<SUMOVehicle>) -> &MSDriveWay {
        self.myLinkInfos[tlIndex as usize].get_drive_way(veh, -1)
    }

    /// C++: `const MSDriveWay & MSRailSignal::retrieveDriveWayForRoute(int, MSRouteIterator, MSRouteIterator)`
    // const MSDriveWay&
    // MSRailSignal::retrieveDriveWayForRoute(int tlIndex, MSRouteIterator first, MSRouteIterator end) {
    //     return myLinkInfos[tlIndex].getDriveWay(first, end);
    // }
    #[allow(unused_variables)]
    pub fn retrieve_drive_way_for_route(&mut self, tlIndex: i32, first: MSRouteIterator, end: MSRouteIterator) -> &MSDriveWay {
        self.myLinkInfos[tlIndex as usize].get_drive_way_2(&Vec::new(), first, end, "")
    }

    /// C++: `const std::vector<MSDriveWay *> MSRailSignal::retrieveDriveWays(int) const` [const]
    // const std::vector<MSDriveWay*>
    // MSRailSignal::retrieveDriveWays(int tlIndex) const {
    //     return myLinkInfos[tlIndex].myDriveways;
    // }
    #[allow(unused_variables)]
    pub fn retrieve_drive_ways(&self, tlIndex: i32) -> Vec<Ptr<MSDriveWay>> {
        self.myLinkInfos[tlIndex as usize].myDriveways.clone()
    }

    /// C++: `bool MSRailSignal::constraintsAllow(const SUMOVehicle *, bool) const` [const]
    /// whether the given vehicle is free to drive
    // bool
    // MSRailSignal::constraintsAllow(const SUMOVehicle* veh, bool storeWaitRelation) const {
    //     if (myConstraints.size() == 0) {
    //         return true;
    //     } else {
    //         const std::string tripID = veh->getParameter().getParameter("tripId", veh->getID());
    //         auto it = myConstraints.find(tripID);
    //         if (it != myConstraints.end()) {
    //             for (MSRailSignalConstraint* c : it->second) {
    //                 // ignore insertion constraints here
    //                 if (!c->isInsertionConstraint() && !c->cleared()) {
    // #ifdef DEBUG_SIGNALSTATE
    //                     if (gDebugFlag4) {
    //                         std::cout << "  constraint '" << c->getDescription() << "' not cleared\n";
    //                     }
    // #endif
    //                     if (storeWaitRelation && MSGlobals::gTimeToTeleportRSDeadlock > 0
    //                             && veh->getWaitingTime() > veh->getVehicleType().getCarFollowModel().getStartupDelay()) {
    //                         const SUMOVehicle* foe = c->getFoe();
    //                         if (foe != nullptr) {
    //                             MSRailSignalControl::getInstance().addWaitRelation(veh, this, foe, c);
    //                         }
    //                     }
    //                     if (myStoreVehicles) {
    //                         myConstraintInfo = c->getDescription();
    //                     }
    //                     return false;
    //                 }
    //             }
    //         }
    //         return true;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn constraints_allow(&self, veh: Ptr<SUMOVehicle>, storeWaitRelation: bool) -> bool {
        if self.myConstraints.len() == 0 {
            return true;
        } else {
            let trip_id = veh
                .cast::<MSBaseVehicle>()
                .get_parameter()
                .get_parameter("tripId", veh.get_id());
            if let Some(constraints) = self.myConstraints.get(&trip_id) {
                for c in constraints {
                    // ignore insertion constraints here
                    if !c.is_insertion_constraint() && !c.cleared() {
                        if storeWaitRelation
                            && MSGlobals::time_to_teleport_rs_deadlock() > 0
                            && veh.cast::<MSVehicle>().get_waiting_time(false)
                                > veh.cast::<MSBaseVehicle>()
                                    .get_vehicle_type()
                                    .get_car_follow_model()
                                    .get_startup_delay()
                        {
                            let foe = c.get_foe();
                            if !foe.is_null() {
                                MSRailSignalControl::get_instance().add_wait_relation(veh, Ptr::new(self), foe, *c);
                            }
                        }
                        if my_store_vehicles() {
                            *my_constraint_info() = c.get_description();
                        }
                        return false;
                    }
                }
            }
            true
        }
    }

    /// C++: `bool MSRailSignal::isMovingBlock() const` [const]
    // bool isMovingBlock() const {
    //         return myMovingBlock;
    //     }
    #[allow(unused_variables)]
    pub fn is_moving_block(&self) -> bool {
        self.myMovingBlock
    }

    /// C++: `const MSJunction * MSRailSignal::getJunction() const` [const]
    /// get the unique junction controlled by this signal
    // const MSJunction*
    // MSRailSignal::getJunction() const {
    //     return myLinkInfos.front().myLink->getJunction();
    // }
    #[allow(unused_variables)]
    pub fn get_junction(&self) -> Ptr<MSJunction> {
        self.myLinkInfos.first().unwrap().myLink.get_junction()
    }

    /// C++: `const MSDriveWay & MSRailSignal::getExistingDriveWay(const MSLink *, MSRouteIterator, MSRouteIterator)`
    // const MSDriveWay&
    // MSRailSignal::getExistingDriveWay(const MSLink* link, MSRouteIterator firstIt, MSRouteIterator endIt) {
    //     LinkInfo& li = myLinkInfos[link->getTLIndex()];
    //     const MSDriveWay& cand = li.getDriveWay(firstIt, endIt, "_tmp_");
    //     // an existing driveway should be found
    //     assert(cand.getFirstVehicle() != "_tmp_");
    //     return cand;
    // }
    #[allow(unused_variables)]
    pub fn get_existing_drive_way(&mut self, link: Ptr<MSLink>, firstIt: MSRouteIterator, endIt: MSRouteIterator) -> &MSDriveWay {
        let cand = self.myLinkInfos[link.get_tl_index() as usize].get_drive_way_2(&Vec::new(), firstIt, endIt, "_tmp_");
        // an existing driveway should be found
        debug_assert!(cand.get_first_vehicle() != "_tmp_");
        cand
    }

    /// C++: `bool MSRailSignal::storeVehicles()` [static]
    // static bool storeVehicles() {
    //         return myStoreVehicles;
    //     }
    #[allow(unused_variables)]
    pub fn store_vehicles() -> bool {
        my_store_vehicles()
    }

    /// C++: `VehicleVector & MSRailSignal::blockingVehicles()` [static]
    // static VehicleVector& blockingVehicles() {
    //         return myBlockingVehicles;
    //     }
    #[allow(unused_variables)]
    pub fn blocking_vehicles() -> &'static mut MSVehicleContainer_VehicleVector {
        my_blocking_vehicles()
    }

    /// C++: `std::vector<const MSDriveWay *> & MSRailSignal::blockingDriveWays()` [static]
    // static std::vector<const MSDriveWay*>& blockingDriveWays() {
    //         return myBlockingDriveWays;
    //     }
    #[allow(unused_variables)]
    pub fn blocking_drive_ways() -> &'static mut Vec<Ptr<MSDriveWay>> {
        my_blocking_drive_ways()
    }

    /// C++: `VehicleVector & MSRailSignal::rivalVehicles()` [static]
    // static VehicleVector& rivalVehicles() {
    //         return myRivalVehicles;
    //     }
    #[allow(unused_variables)]
    pub fn rival_vehicles() -> &'static mut MSVehicleContainer_VehicleVector {
        my_rival_vehicles()
    }

    /// C++: `VehicleVector & MSRailSignal::priorityVehicles()` [static]
    // static VehicleVector& priorityVehicles() {
    //         return myPriorityVehicles;
    //     }
    #[allow(unused_variables)]
    pub fn priority_vehicles() -> &'static mut MSVehicleContainer_VehicleVector {
        my_priority_vehicles()
    }

    /// C++: `std::string MSRailSignal::describeLinks(std::vector<MSLink *>)` [static]
    /// print link descriptions
    // std::string
    // MSRailSignal::describeLinks(std::vector<MSLink*> links) {
    //     std::string result;
    //     for (MSLink* link : links) {
    //         result += link->getDescription() + " ";
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn describe_links(links: Vec<Ptr<MSLink>>) -> String {
        let mut result = String::new();
        for link in links {
            result.push_str(&link.get_description());
            result.push(' ');
        }
        result
    }

    /// C++: `std::string MSRailSignal::getNewDrivewayID()` [protected]
    // std::string
    // MSRailSignal::getNewDrivewayID() {
    //     return getID() + "." + toString(myDriveWayIndex++);
    // }
    #[allow(unused_variables)]
    pub fn get_new_driveway_id(&mut self) -> String {
        let result = format!("{}.{}", self.get_id(), utils::common::to_string::to_string(&self.myDriveWayIndex));
        self.myDriveWayIndex += 1;
        result
    }

    /// C++: `void MSRailSignal::storeTraCIVehicles(int)` [protected]
    /// update vehicle lists for traci calls
    // void
    // MSRailSignal::storeTraCIVehicles(int linkIndex) {
    //     resetStored();
    //     myStoreVehicles = true;
    //     LinkInfo& li = myLinkInfos[linkIndex];
    //     if (li.myLink->getApproaching().size() > 0) {
    //         Approaching closest = li.myLink->getClosest();
    //         MSDriveWay& driveway = li.getDriveWay(closest.first);
    //         MSEdgeVector occupied;
    //         myRequestedDriveWay = driveway.getID();
    //         // call for side effects
    //         driveway.reserve(closest, occupied);
    //         constraintsAllow(closest.first);
    //     } else if (li.myDriveways.size() > 0) {
    //         li.myDriveways.front()->conflictLaneOccupied();
    //         li.myDriveways.front()->foeDriveWayApproached();
    //     }
    //     myStoreVehicles = false;
    // }
    #[allow(unused_variables)]
    pub fn store_tra_ci_vehicles(&mut self, linkIndex: i32) {
        MSRailSignal::reset_stored();
        set_my_store_vehicles(true);
        let has_approaching = self.myLinkInfos[linkIndex as usize].myLink.get_approaching_2().len() > 0;
        if has_approaching {
            let closest = self.myLinkInfos[linkIndex as usize].myLink.get_closest();
            let mut occupied: MSEdgeVector = Vec::new();
            {
                let li = &mut self.myLinkInfos[linkIndex as usize];
                let driveway = li.get_drive_way(closest.0, -1);
                *my_requested_drive_way() = driveway.base2.get_id().to_string();
                // call for side effects
                driveway.reserve(&closest, &mut occupied);
            }
            self.constraints_allow(closest.0, false);
        } else if !self.myLinkInfos[linkIndex as usize].myDriveways.is_empty() {
            let driveway = self.myLinkInfos[linkIndex as usize].myDriveways[0];
            driveway.conflict_lane_occupied(false, Ptr::null());
            driveway.foe_drive_way_approached();
        }
        set_my_store_vehicles(false);
    }

    /// C++: `void MSRailSignal::storeTraCIVehicles(const MSDriveWay *)` [static, protected]
    // void
    // MSRailSignal::storeTraCIVehicles(const MSDriveWay* dw) {
    //     resetStored();
    //     myStoreVehicles = true;
    //     MSEdgeVector occupied;
    //     // call for side effects
    //     dw->foeDriveWayOccupied(true, nullptr, occupied);
    //     myStoreVehicles = false;
    // }
    #[allow(unused_variables)]
    pub fn store_tra_ci_vehicles_2(dw: Ptr<MSDriveWay>) {
        MSRailSignal::reset_stored();
        set_my_store_vehicles(true);
        let mut occupied: MSEdgeVector = Vec::new();
        // call for side effects
        dw.foe_drive_way_occupied(true, Ptr::null(), &mut occupied);
        set_my_store_vehicles(false);
    }

    /// C++: `void MSRailSignal::resetStored()` [static, protected]
    /// reset temporary storage for injected conflict output
    // void
    // MSRailSignal::resetStored() {
    //     myBlockingVehicles.clear();
    //     myRivalVehicles.clear();
    //     myPriorityVehicles.clear();
    //     myConstraintInfo = "";
    //     myBlockingDriveWays.clear();
    //     myRequestedDriveWay = "";
    // }
    #[allow(unused_variables)]
    pub fn reset_stored() {
        my_blocking_vehicles().clear();
        my_rival_vehicles().clear();
        my_priority_vehicles().clear();
        *my_constraint_info() = String::new();
        my_blocking_drive_ways().clear();
        *my_requested_drive_way() = String::new();
    }

}

