//! Port of microsim/lcmodels/MSLCM_SL2015.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::pair<double, int> MSLCM_SL2015::Info`
pub type MSLCM_SL2015_Info = (f64, i32);

/// A lane change model developed by J. Erdmann
/// C++ class `MSLCM_SL2015` (microsim/lcmodels/MSLCM_SL2015.h)
pub struct MSLCM_SL2015 {
    /// C++ base class `MSAbstractLaneChangeModel`
    pub base: MSAbstractLaneChangeModel,
    /// a value for tracking the probability that a change to the right is beneficial
    pub mySpeedGainProbabilityRight: f64,
    /// a value for tracking the probability that a change to the left is beneficial
    pub mySpeedGainProbabilityLeft: f64,
    pub myKeepRightProbability: f64,
    pub myLeadingBlockerLength: f64,
    pub myLeftSpace: f64,
    pub myLookAheadSpeed: f64,
    /// expected travel speeds on all sublanes on the current edge(!)
    pub myExpectedSublaneSpeeds: Vec<f64>,
    /// expected travel speeds on all sublanes on the current edge(!)
    pub myLastEdge: Ptr<MSEdge>,
    /// flag to prevent speed adaptation by slowing down
    pub myDontBrake: bool,
    /// whether the current lane changing maneuver can be finished in a single step
    pub myCanChangeFully: bool,
    /// the lateral distance the vehicle can safely move in the currently considered direction
    pub mySafeLatDistRight: f64,
    pub mySafeLatDistLeft: f64,
    /// set of vehicles that are in a car-following relationship with ego (leader of followers)
    pub myCFRelated: BTreeSet<Ptr<MSVehicle>>,
    pub myCFRelatedReady: bool,
    pub myStrategicParam: f64,
    pub myCooperativeParam: f64,
    pub mySpeedGainParam: f64,
    pub myKeepRightParam: f64,
    pub myOppositeParam: f64,
    pub mySublaneParam: f64,
    pub myMinGapLat: f64,
    pub myPushy: f64,
    pub myImpatience: f64,
    pub myMinImpatience: f64,
    pub myTimeToImpatience: f64,
    pub myAccelLat: f64,
    pub myTurnAlignmentDist: f64,
    pub myLookaheadLeft: f64,
    pub mySpeedGainRight: f64,
    pub myLaneDiscipline: f64,
    pub mySpeedGainLookahead: f64,
    pub mySpeedGainRemainTime: f64,
    pub myRoundaboutBonus: f64,
    pub myCooperativeSpeed: f64,
    pub myKeepRightAcceptanceTime: f64,
    pub myOvertakeDeltaSpeedFactor: f64,
    pub myChangeProbThresholdRight: f64,
    pub myChangeProbThresholdLeft: f64,
    pub mySpeedLossProbThreshold: f64,
    pub mySigmaState: f64,
}

impl std::ops::Deref for MSLCM_SL2015 {
    type Target = MSAbstractLaneChangeModel;
    fn deref(&self) -> &MSAbstractLaneChangeModel {
        &self.base
    }
}
impl std::ops::DerefMut for MSLCM_SL2015 {
    fn deref_mut(&mut self) -> &mut MSAbstractLaneChangeModel {
        &mut self.base
    }
}

impl MSLCM_SL2015 {
    /// C++: `void MSLCM_SL2015::MSLCM_SL2015(MSVehicle &)`
    // MSLCM_SL2015::MSLCM_SL2015(MSVehicle& v) :
    //     MSAbstractLaneChangeModel(v, LaneChangeModel::SL2015),
    //     mySpeedGainProbabilityRight(0),
    //     mySpeedGainProbabilityLeft(0),
    //     myKeepRightProbability(0),
    //     myLeadingBlockerLength(0),
    //     myLeftSpace(0),
    //     myLookAheadSpeed(LOOK_AHEAD_MIN_SPEED),
    //     myLastEdge(nullptr),
    //     myCanChangeFully(true),
    //     mySafeLatDistRight(0),
    //     mySafeLatDistLeft(0),
    //     myStrategicParam(v.getVehicleType().getParameter().getLCParam(SUMO_ATTR_LCA_STRATEGIC_PARAM, 1)),
    //     myCooperativeParam(v.getVehicleType().getParameter().getLCParam(SUMO_ATTR_LCA_COOPERATIVE_PARAM, 1)),
    //     mySpeedGainParam(v.getVehicleType().getParameter().getLCParam(SUMO_ATTR_LCA_SPEEDGAIN_PARAM, 1)),
    //     myKeepRightParam(v.getVehicleType().getParameter().getLCParam(SUMO_ATTR_LCA_KEEPRIGHT_PARAM, 1)),
    //     myOppositeParam(v.getVehicleType().getParameter().getLCParam(SUMO_ATTR_LCA_OPPOSITE_PARAM, 1)),
    //     mySublaneParam(v.getVehicleType().getParameter().getLCParam(SUMO_ATTR_LCA_SUBLANE_PARAM, 1)),
    //     // by default use SUMO_ATTR_LCA_PUSHY. If that is not set, try SUMO_ATTR_LCA_PUSHYGAP
    //     myMinGapLat(v.getVehicleType().getMinGapLat()),
    //     myPushy(v.getVehicleType().getParameter().getLCParam(SUMO_ATTR_LCA_PUSHY,
    //             1 - (v.getVehicleType().getParameter().getLCParam(SUMO_ATTR_LCA_PUSHYGAP,
    //                     MAX2(NUMERICAL_EPS, myMinGapLat)) /
    //                  MAX2(NUMERICAL_EPS, myMinGapLat)))),
    //     myImpatience(v.getVehicleType().getParameter().getLCParam(SUMO_ATTR_LCA_IMPATIENCE, 0)),
    //     myMinImpatience(myImpatience),
    //     myTimeToImpatience(v.getVehicleType().getParameter().getLCParam(SUMO_ATTR_LCA_TIME_TO_IMPATIENCE, std::numeric_limits<double>::max())),
    //     myAccelLat(v.getVehicleType().getParameter().getLCParam(SUMO_ATTR_LCA_ACCEL_LAT, 1.0)),
    //     myTurnAlignmentDist(v.getVehicleType().getParameter().getLCParam(SUMO_ATTR_LCA_TURN_ALIGNMENT_DISTANCE, 0.0)),
    //     myLookaheadLeft(v.getVehicleType().getParameter().getLCParam(SUMO_ATTR_LCA_LOOKAHEADLEFT, 2.0)),
    //     mySpeedGainRight(v.getVehicleType().getParameter().getLCParam(SUMO_ATTR_LCA_SPEEDGAINRIGHT, 0.1)),
    //     myLaneDiscipline(v.getVehicleType().getParameter().getLCParam(SUMO_ATTR_LCA_LANE_DISCIPLINE, 0.0)),
    //     mySpeedGainLookahead(v.getVehicleType().getParameter().getLCParam(SUMO_ATTR_LCA_SPEEDGAIN_LOOKAHEAD, 5)),
    //     mySpeedGainRemainTime(v.getVehicleType().getParameter().getLCParam(SUMO_ATTR_LCA_SPEEDGAIN_REMAIN_TIME, 20)),
    //     myRoundaboutBonus(v.getVehicleType().getParameter().getLCParam(SUMO_ATTR_LCA_COOPERATIVE_ROUNDABOUT, myCooperativeParam)),
    //     myCooperativeSpeed(v.getVehicleType().getParameter().getLCParam(SUMO_ATTR_LCA_COOPERATIVE_SPEED, myCooperativeParam)),
    //     myKeepRightAcceptanceTime(v.getVehicleType().getParameter().getLCParam(SUMO_ATTR_LCA_KEEPRIGHT_ACCEPTANCE_TIME, -1)),
    //     myOvertakeDeltaSpeedFactor(v.getVehicleType().getParameter().getLCParam(SUMO_ATTR_LCA_OVERTAKE_DELTASPEED_FACTOR, 0)),
    //     mySigmaState(0) {
    //     initDerivedParameters();
    // }
    #[allow(unused_variables)]
    pub fn new(v: &mut MSVehicle) -> Self {
        todo!("port MSLCM_SL2015::MSLCM_SL2015")
    }

    /// C++: `void MSLCM_SL2015::~MSLCM_SL2015() noexcept` [virtual]
    // MSLCM_SL2015::~MSLCM_SL2015() {
    //     changed();
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSLCM_SL2015::~MSLCM_SL2015")
    }

    /// C++: `LaneChangeModel MSLCM_SL2015::getModelID() const` [const, virtual]
    /// Returns the model's id
    // LaneChangeModel getModelID() const override {
    //         return LaneChangeModel::SL2015;
    //     }
    #[allow(unused_variables)]
    pub fn get_model_id(&self) -> LaneChangeModel {
        todo!("port MSLCM_SL2015::getModelID")
    }

    /// C++: `void MSLCM_SL2015::initDerivedParameters()`
    /// init cached parameters derived directly from model parameters
    // void
    // MSLCM_SL2015::initDerivedParameters() {
    //     if (mySpeedGainParam <= 0) {
    //         myChangeProbThresholdRight = std::numeric_limits<double>::max();
    //         myChangeProbThresholdLeft = std::numeric_limits<double>::max();
    //     } else {
    //         myChangeProbThresholdRight = (0.2 / mySpeedGainRight) / mySpeedGainParam;
    //         myChangeProbThresholdLeft = 0.2 / mySpeedGainParam;
    //     }
    //     mySpeedLossProbThreshold = (-0.1 + (1 - mySublaneParam));
    // }
    #[allow(unused_variables)]
    pub fn init_derived_parameters(&mut self) {
        todo!("port MSLCM_SL2015::initDerivedParameters")
    }

    /// C++: `int MSLCM_SL2015::wantsChangeSublane(int, LaneChangeAction, const MSLeaderDistanceInfo &, const MSLeaderDistanceInfo &, const MSLeaderDistanceInfo &, const MSLeaderDistanceInfo &, const MSLeaderDistanceInfo &, const MSLeaderDistanceInfo &, const MSLane &, const std::vector<MSVehicle::LaneQ> &, MSVehicle **, MSVehicle **, double &, double &, int &)` [virtual]
    /// Called to examine whether the vehicle wants to change with the given laneOffset (using the sublane model) This method gets the information about the surrounding vehicles and whether another lane may be more preferable
    // int
    // MSLCM_SL2015::wantsChangeSublane(
    //     int laneOffset,
    //     LaneChangeAction alternatives,
    //     const MSLeaderDistanceInfo& leaders,
    //     const MSLeaderDistanceInfo& followers,
    //     const MSLeaderDistanceInfo& blockers,
    //     const MSLeaderDistanceInfo& neighLeaders,
    //     const MSLeaderDistanceInfo& neighFollowers,
    //     const MSLeaderDistanceInfo& neighBlockers,
    //     const MSLane& neighLane,
    //     const std::vector<MSVehicle::LaneQ>& preb,
    //     MSVehicle** lastBlocked,
    //     MSVehicle** firstBlocked,
    //     double& latDist, double& maneuverDist, int& blocked) {
    //
    //     gDebugFlag2 = DEBUG_COND;
    //     const std::string changeType = laneOffset == -1 ? "right" : (laneOffset == 1 ? "left" : "current");
    //
    // #ifdef DEBUG_MANEUVER
    //     if (gDebugFlag2) {
    //         std::cout << "\n" << SIMTIME
    //                   << std::setprecision(gPrecision)
    //                   << " veh=" << myVehicle.getID()
    //                   << " lane=" << myVehicle.getLane()->getID()
    //                   << " neigh=" << neighLane.getID()
    //                   << " pos=" << myVehicle.getPositionOnLane()
    //                   << " posLat=" << myVehicle.getLateralPositionOnLane()
    //                   << " posLatError=" << mySigmaState
    //                   << " speed=" << myVehicle.getSpeed()
    //                   << " considerChangeTo=" << changeType
    //                   << "\n";
    //     }
    // #endif
    //
    //     int result = _wantsChangeSublane(laneOffset,
    //                                      alternatives,
    //                                      leaders, followers, blockers,
    //                                      neighLeaders, neighFollowers, neighBlockers,
    //                                      neighLane, preb,
    //                                      lastBlocked, firstBlocked, latDist, maneuverDist, blocked);
    //
    //     result = keepLatGap(result, leaders, followers, blockers,
    //                         neighLeaders, neighFollowers, neighBlockers,
    //                         neighLane, laneOffset, latDist, maneuverDist, blocked);
    //
    //     result |= getLCA(result, latDist);
    //     // take into account lateral acceleration
    // #if defined(DEBUG_MANEUVER) || defined(DEBUG_STATE)
    //     double latDistTmp = latDist;
    // #endif
    //     latDist = SPEED2DIST(computeSpeedLat(latDist, maneuverDist, (result & LCA_URGENT) != 0));
    // #if defined(DEBUG_MANEUVER) || defined(DEBUG_STATE)
    //     if (gDebugFlag2 && latDist != latDistTmp) {
    //         std::cout << SIMTIME << " veh=" << myVehicle.getID() << " maneuverDist=" << maneuverDist << " latDist=" << latDistTmp << " mySpeedPrev=" << mySpeedLat << " speedLat=" << DIST2SPEED(latDist) << " latDist2=" << latDist << "\n";
    //     }
    //
    //     if (gDebugFlag2) {
    //         if (result & LCA_WANTS_LANECHANGE) {
    //             std::cout << SIMTIME
    //                       << " veh=" << myVehicle.getID()
    //                       << " wantsChangeTo=" << changeType
    //                       << " latDist=" << latDist
    //                       << " maneuverDist=" << maneuverDist
    //                       << " state=" << toString((LaneChangeAction)result)
    //                       << ((blocked & LCA_BLOCKED) ? " (blocked)" : "")
    //                       << ((blocked & LCA_OVERLAPPING) ? " (overlap)" : "")
    //                       << "\n\n";
    //         } else {
    //             std::cout << SIMTIME
    //                       << " veh=" << myVehicle.getID()
    //                       << " wantsNoChangeTo=" << changeType
    //                       << " state=" << toString((LaneChangeAction)result)
    //                       << "\n\n";
    //         }
    //     }
    // #endif
    //     gDebugFlag2 = false;
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn wants_change_sublane(&mut self, laneOffset: i32, alternatives: LaneChangeAction, leaders: &MSLeaderDistanceInfo, followers: &MSLeaderDistanceInfo, blockers: &MSLeaderDistanceInfo, neighLeaders: &MSLeaderDistanceInfo, neighFollowers: &MSLeaderDistanceInfo, neighBlockers: &MSLeaderDistanceInfo, neighLane: &MSLane, preb: &Vec<MSVehicle_LaneQ>, lastBlocked: Ptr<Ptr<MSVehicle>>, firstBlocked: Ptr<Ptr<MSVehicle>>, latDist: &mut f64, maneuverDist: &mut f64, blocked: &mut i32) -> i32 {
        todo!("port MSLCM_SL2015::wantsChangeSublane")
    }

    /// C++: `int MSLCM_SL2015::wantsChange(int, MSAbstractLaneChangeModel::MSLCMessager &, int, const std::pair<MSVehicle *, double> &, const std::pair<MSVehicle *, double> &, const std::pair<MSVehicle *, double> &, const std::pair<MSVehicle *, double> &, const MSLane &, const std::vector<MSVehicle::LaneQ> &, MSVehicle **, MSVehicle **)` [virtual]
    /// Called to examine whether the vehicle wants to change using the given laneOffset (this is a wrapper around wantsChangeSublane). XXX: no, it wraps _wantsChangeSublane This method gets the information about the surrounding vehicles and whether another lane may be more preferable
    // int
    // MSLCM_SL2015::wantsChange(
    //     int laneOffset,
    //     MSAbstractLaneChangeModel::MSLCMessager& /* msgPass */,
    //     int blocked,
    //     const std::pair<MSVehicle*, double>& leader,
    //     const std::pair<MSVehicle*, double>& follower,
    //     const std::pair<MSVehicle*, double>& neighLead,
    //     const std::pair<MSVehicle*, double>& neighFollow,
    //     const MSLane& neighLane,
    //     const std::vector<MSVehicle::LaneQ>& preb,
    //     MSVehicle** lastBlocked,
    //     MSVehicle** firstBlocked) {
    //
    //     const LaneChangeAction alternatives = LCA_NONE; // @todo pas this data
    //
    // #ifdef DEBUG_WANTSCHANGE
    //     if (DEBUG_COND) {
    //         std::cout << "\nWANTS_CHANGE\n" << SIMTIME
    //                   //<< std::setprecision(10)
    //                   << " veh=" << myVehicle.getID()
    //                   << " lane=" << myVehicle.getLane()->getID()
    //                   << " neigh=" << neighLane.getID()
    //                   << " pos=" << myVehicle.getPositionOnLane()
    //                   << " posLat=" << myVehicle.getLateralPositionOnLane()
    //                   << " speed=" << myVehicle.getSpeed()
    //                   << " considerChangeTo=" << (laneOffset == -1  ? "right" : "left")
    //                   << "\n";
    //     }
    // #endif
    //
    //     double latDist = 0;
    //     const double laneWidth = myVehicle.getLane()->getWidth();
    //     MSLeaderDistanceInfo leaders(leader, laneWidth);
    //     MSLeaderDistanceInfo followers(follower, laneWidth);
    //     MSLeaderDistanceInfo blockers(std::make_pair((MSVehicle*)nullptr, -1), laneWidth);
    //     MSLeaderDistanceInfo neighLeaders(neighLead, laneWidth);
    //     MSLeaderDistanceInfo neighFollowers(neighFollow, laneWidth);
    //     MSLeaderDistanceInfo neighBlockers(std::make_pair((MSVehicle*)nullptr, -1), laneWidth);
    //
    //     double maneuverDist;
    //     int result = _wantsChangeSublane(laneOffset,
    //                                      alternatives,
    //                                      leaders, followers, blockers,
    //                                      neighLeaders, neighFollowers, neighBlockers,
    //                                      neighLane, preb,
    //                                      lastBlocked, firstBlocked, latDist, maneuverDist, blocked);
    //
    //     myCanChangeFully = true;
    //     // ignore sublane motivation
    //     result &= ~LCA_SUBLANE;
    //     result |= getLCA(result, latDist);
    //
    // #if defined(DEBUG_WANTSCHANGE) || defined(DEBUG_STATE)
    //     if (DEBUG_COND) {
    //         if (result & LCA_WANTS_LANECHANGE) {
    //             std::cout << SIMTIME
    //                       << " veh=" << myVehicle.getID()
    //                       << " wantsChangeTo=" << (laneOffset == -1  ? "right" : "left")
    //                       << ((result & LCA_URGENT) ? " (urgent)" : "")
    //                       << ((result & LCA_CHANGE_TO_HELP) ? " (toHelp)" : "")
    //                       << ((result & LCA_STRATEGIC) ? " (strat)" : "")
    //                       << ((result & LCA_COOPERATIVE) ? " (coop)" : "")
    //                       << ((result & LCA_SPEEDGAIN) ? " (speed)" : "")
    //                       << ((result & LCA_KEEPRIGHT) ? " (keepright)" : "")
    //                       << ((result & LCA_TRACI) ? " (traci)" : "")
    //                       << ((blocked & LCA_BLOCKED) ? " (blocked)" : "")
    //                       << ((blocked & LCA_OVERLAPPING) ? " (overlap)" : "")
    //                       << "\n\n\n";
    //         }
    //     }
    // #endif
    //
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn wants_change(&mut self, laneOffset: i32, msgPass: &mut MSAbstractLaneChangeModel_MSLCMessager, blocked: i32, leader: &(Ptr<MSVehicle>, f64), follower: &(Ptr<MSVehicle>, f64), neighLead: &(Ptr<MSVehicle>, f64), neighFollow: &(Ptr<MSVehicle>, f64), neighLane: &MSLane, preb: &Vec<MSVehicle_LaneQ>, lastBlocked: Ptr<Ptr<MSVehicle>>, firstBlocked: Ptr<Ptr<MSVehicle>>) -> i32 {
        todo!("port MSLCM_SL2015::wantsChange")
    }

    /// C++: `void * MSLCM_SL2015::inform(void *, MSVehicle *)` [virtual]
    // void*
    // MSLCM_SL2015::inform(void* info, MSVehicle* sender) {
    //     Info* pinfo = (Info*) info;
    //     if (pinfo->first >= 0) {
    //         addLCSpeedAdvice(pinfo->first, LCA_CHANGE_TO_HELP);
    //     }
    //     //myOwnState &= 0xffffffff; // reset all bits of MyLCAEnum but only those
    //     myOwnState |= pinfo->second;
    // #ifdef DEBUG_INFORM
    //     if (gDebugFlag2 || DEBUG_COND || sender->getLaneChangeModel().debugVehicle()) {
    //         std::cout << SIMTIME
    //                   << " veh=" << myVehicle.getID()
    //                   << " informedBy=" << sender->getID()
    //                   << " info=" << pinfo->second
    //                   << " vSafe=" << pinfo->first
    //                   << "\n";
    //     }
    // #else
    //     UNUSED_PARAMETER(sender);
    // #endif
    //     delete pinfo;
    //     return (void*) true;
    // }
    #[allow(unused_variables)]
    pub fn inform(&mut self, info: Ptr<()>, sender: Ptr<MSVehicle>) -> Ptr<()> {
        todo!("port MSLCM_SL2015::inform")
    }

    /// C++: `double MSLCM_SL2015::patchSpeed(const double, const double, const double, const MSCFModel &)` [virtual]
    /// Called to adapt the speed in order to allow a lane change. It uses information on LC-related desired speed-changes from the call to wantsChange() at the end of the previous simulation step
    // double
    // MSLCM_SL2015::patchSpeed(const double min, const double wanted, const double max, const MSCFModel& cfModel) {
    //     gDebugFlag2 = DEBUG_COND;
    //     // negative min speed may be passed when using ballistic updated
    //     const double newSpeed = _patchSpeed(MAX2(min, 0.0), wanted, max, cfModel);
    // #ifdef DEBUG_PATCHSPEED
    //     if (gDebugFlag2) {
    //         const std::string patched = (wanted != newSpeed ? " patched=" + toString(newSpeed) : "");
    //         std::cout << SIMTIME
    //                   << " veh=" << myVehicle.getID()
    //                   << " lane=" << myVehicle.getLane()->getID()
    //                   << " pos=" << myVehicle.getPositionOnLane()
    //                   << " v=" << myVehicle.getSpeed()
    //                   << " min=" << min
    //                   << " wanted=" << wanted
    //                   << " max=" << max
    //                   << patched
    //                   << "\n\n";
    //     }
    // #endif
    //     gDebugFlag2 = false;
    //     return newSpeed;
    // }
    #[allow(unused_variables)]
    pub fn patch_speed(&mut self, min: f64, wanted: f64, max: f64, cfModel: &MSCFModel) -> f64 {
        todo!("port MSLCM_SL2015::patchSpeed")
    }

    /// C++: `void MSLCM_SL2015::changed()` [virtual]
    // void
    // MSLCM_SL2015::changed() {
    //     if (!myCanChangeFully) {
    //         // do not reset state yet so we can continue our maneuver but acknowledge
    //         // a change to the right (movement should continue due to lane alignment desire)
    //         if (getManeuverDist() < 0) {
    //             myKeepRightProbability = 0;
    //         }
    // #ifdef DEBUG_STATE
    //         if (DEBUG_COND) {
    //             std::cout << SIMTIME << " veh=" << myVehicle.getID() << " state not reset. maneuverDist=" << getManeuverDist() << "\n";
    //         }
    // #endif
    //         return;
    //     }
    //     myOwnState = 0;
    //     // XX do not reset values for unfinished maneuvers
    //     mySpeedGainProbabilityRight = 0;
    //     mySpeedGainProbabilityLeft = 0;
    //     myKeepRightProbability = 0;
    //
    //     if (myVehicle.getBestLaneOffset() == 0) {
    //         // if we are not yet on our best lane there might still be unseen blockers
    //         // (during patchSpeed)
    //         myLeadingBlockerLength = 0;
    //         myLeftSpace = 0;
    //     }
    //     myLookAheadSpeed = LOOK_AHEAD_MIN_SPEED;
    //     myLCAccelerationAdvices.clear();
    //     myDontBrake = false;
    //     myLeadingBlockerLength = 0;
    // #if defined(DEBUG_MANEUVER) || defined(DEBUG_STATE)
    //     if (DEBUG_COND) {
    //         std::cout << SIMTIME << " veh=" << myVehicle.getID() << " changed()\n";
    //     }
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn changed(&mut self) {
        todo!("port MSLCM_SL2015::changed")
    }

    /// C++: `void MSLCM_SL2015::resetState()` [virtual]
    // void
    // MSLCM_SL2015::resetState() {
    //     myOwnState = 0;
    //     mySpeedGainProbabilityRight = 0;
    //     mySpeedGainProbabilityLeft = 0;
    //     myKeepRightProbability = 0;
    //     myLeadingBlockerLength = 0;
    //     myLeftSpace = 0;
    //     myLookAheadSpeed = LOOK_AHEAD_MIN_SPEED;
    //     myLCAccelerationAdvices.clear();
    //     myDontBrake = false;
    // }
    #[allow(unused_variables)]
    pub fn reset_state(&mut self) {
        todo!("port MSLCM_SL2015::resetState")
    }

    /// C++: `double MSLCM_SL2015::getSafetyFactor() const` [const, virtual]
    // double
    // MSLCM_SL2015::getSafetyFactor() const {
    //     return 1 / ((1 + 0.5 * myImpatience) * myAssertive);
    // }
    #[allow(unused_variables)]
    pub fn get_safety_factor(&self) -> f64 {
        todo!("port MSLCM_SL2015::getSafetyFactor")
    }

    /// C++: `double MSLCM_SL2015::getOppositeSafetyFactor() const` [const, virtual]
    // double
    // MSLCM_SL2015::getOppositeSafetyFactor() const {
    //     return myOppositeParam <= 0 ? std::numeric_limits<double>::max() : 1 / myOppositeParam;
    // }
    #[allow(unused_variables)]
    pub fn get_opposite_safety_factor(&self) -> f64 {
        todo!("port MSLCM_SL2015::getOppositeSafetyFactor")
    }

    /// C++: `void MSLCM_SL2015::prepareStep()` [virtual]
    // void
    // MSLCM_SL2015::prepareStep() {
    //     MSAbstractLaneChangeModel::prepareStep();
    //     // keep information about strategic change direction
    //     myOwnState = (myOwnState & (LCA_STRATEGIC | LCA_COOPERATIVE)) ? (myOwnState & LCA_WANTS_LANECHANGE) : 0;
    // #ifdef DEBUG_INFORM
    //     if (debugVehicle()) {
    //         std::cout << SIMTIME
    //                   << " veh=" << myVehicle.getID()
    //                   << " prepareStep"
    //                   << " myCanChangeFully=" << myCanChangeFully
    //                   << "\n";
    //     }
    // #endif
    //     myLeadingBlockerLength = 0;
    //     myLeftSpace = 0;
    //     myLCAccelerationAdvices.clear();
    //     myDontBrake = false;
    //     myCFRelated.clear();
    //     myCFRelatedReady = false;
    //     const double halfWidth = getWidth() * 0.5;
    //     // only permit changing within lane bounds but open up the range depending on the checked duration in _wantsChangeSublane()
    //     mySafeLatDistRight = myVehicle.getLane()->getWidth() * 0.5 + myVehicle.getLateralPositionOnLane() - halfWidth;
    //     mySafeLatDistLeft = myVehicle.getLane()->getWidth() * 0.5 - myVehicle.getLateralPositionOnLane() - halfWidth;
    //     if (isOpposite()) {
    //         std::swap(mySafeLatDistLeft, mySafeLatDistRight);
    //     }
    //     // truncate to work around numerical instability between different builds
    //     mySpeedGainProbabilityRight = ceil(mySpeedGainProbabilityRight * 100000.0) * 0.00001;
    //     mySpeedGainProbabilityLeft = ceil(mySpeedGainProbabilityLeft * 100000.0) * 0.00001;
    //     myKeepRightProbability = ceil(myKeepRightProbability * 100000.0) * 0.00001;
    //     // updated myExpectedSublaneSpeeds
    //     // XXX only do this when (sub)lane changing is possible
    //     std::vector<double> newExpectedSpeeds;
    // #ifdef DEBUG_INFORM
    //     if (DEBUG_COND) {
    //         std::cout << SIMTIME << " veh=" << myVehicle.getID() << " myExpectedSublaneSpeeds=" << toString(myExpectedSublaneSpeeds) << "\n";
    //     }
    // #endif
    //     if (myExpectedSublaneSpeeds.size() != myVehicle.getLane()->getEdge().getSubLaneSides().size()) {
    //         // initialize
    //         const MSEdge* currEdge = &myVehicle.getLane()->getEdge();
    //         const std::vector<MSLane*>& lanes = currEdge->getLanes();
    //         for (std::vector<MSLane*>::const_iterator it_lane = lanes.begin(); it_lane != lanes.end(); ++it_lane) {
    //             const int subLanes = MAX2(1, int(ceil((*it_lane)->getWidth() / MSGlobals::gLateralResolution)));
    //             for (int i = 0; i < subLanes; ++i) {
    //                 newExpectedSpeeds.push_back((*it_lane)->getVehicleMaxSpeed(&myVehicle));
    //             }
    //         }
    //         if (currEdge->canChangeToOpposite()) {
    //             MSLane* opposite = lanes.back()->getOpposite();
    //             const int subLanes = MAX2(1, int(ceil(opposite->getWidth() / MSGlobals::gLateralResolution)));
    //             for (int i = 0; i < subLanes; ++i) {
    //                 newExpectedSpeeds.push_back(lanes.back()->getVehicleMaxSpeed(&myVehicle));
    //             }
    //         }
    //         if (myExpectedSublaneSpeeds.size() > 0) {
    //             // copy old values
    //             assert(myLastEdge != 0);
    //             if (myLastEdge->getSubLaneSides().size() == myExpectedSublaneSpeeds.size()) {
    //                 const int subLaneShift = computeSublaneShift(myLastEdge, currEdge);
    //                 if (subLaneShift < std::numeric_limits<int>::max()) {
    //                     for (int i = 0; i < (int)myExpectedSublaneSpeeds.size(); ++i) {
    //                         const int newI = i + subLaneShift;
    //                         if (newI > 0 && newI < (int)newExpectedSpeeds.size()) {
    //                             newExpectedSpeeds[newI] = myExpectedSublaneSpeeds[i];
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //         myExpectedSublaneSpeeds = newExpectedSpeeds;
    //         myLastEdge = currEdge;
    //     }
    //     assert(myExpectedSublaneSpeeds.size() == myVehicle.getLane()->getEdge().getSubLaneSides().size());
    //     if (mySigma > 0) {
    //         mySigmaState += getLateralDrift();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn prepare_step(&mut self) {
        todo!("port MSLCM_SL2015::prepareStep")
    }

    /// C++: `bool MSLCM_SL2015::debugVehicle() const` [const, virtual]
    /// whether the current vehicles shall be debugged
    // bool
    // MSLCM_SL2015::debugVehicle() const {
    //     return DEBUG_COND;
    // }
    #[allow(unused_variables)]
    pub fn debug_vehicle(&self) -> bool {
        todo!("port MSLCM_SL2015::debugVehicle")
    }

    /// C++: `void MSLCM_SL2015::setOwnState(const int)` [virtual]
    // void
    // MSLCM_SL2015::setOwnState(const int state) {
    //     MSAbstractLaneChangeModel::setOwnState(state);
    //     if (myVehicle.isActive()) {
    //         if ((state & (LCA_STRATEGIC | LCA_SPEEDGAIN)) != 0 && (state & LCA_BLOCKED) != 0) {
    //             myImpatience = MIN2(1.0, myImpatience + myVehicle.getActionStepLengthSecs() / myTimeToImpatience);
    //         } else {
    //             // impatience decays only to the driver-specific level
    //             myImpatience = MAX2(myMinImpatience, myImpatience - myVehicle.getActionStepLengthSecs() / myTimeToImpatience);
    //         }
    // #ifdef DEBUG_STATE
    //         if (DEBUG_COND) {
    //             std::cout << SIMTIME << " veh=" << myVehicle.getID()
    //                       << " setOwnState=" << toString((LaneChangeAction)state)
    //                       << " myMinImpatience=" << myMinImpatience
    //                       << " myImpatience=" << myImpatience
    //                       << "\n";
    //         }
    // #endif
    //         if ((state & LCA_STAY) != 0) {
    //             myCanChangeFully = true;
    // //            if (DEBUG_COND) {
    // //                std::cout << "    myCanChangeFully=true\n";
    // //            }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_own_state(&mut self, state: i32) {
        todo!("port MSLCM_SL2015::setOwnState")
    }

    /// C++: `void MSLCM_SL2015::updateSafeLatDist(const double)` [virtual]
    /// Updates the value of safe lateral distances (mySafeLatDistLeft and mySafeLatDistRight) during maneuver continuation in non-action steps.
    // void
    // MSLCM_SL2015::updateSafeLatDist(const double travelledLatDist) {
    //     mySafeLatDistLeft -= travelledLatDist;
    //     mySafeLatDistRight += travelledLatDist;
    //
    //     if (fabs(mySafeLatDistLeft) < NUMERICAL_EPS) {
    //         mySafeLatDistLeft = 0.;
    //     }
    //     if (fabs(mySafeLatDistRight) < NUMERICAL_EPS) {
    //         mySafeLatDistRight = 0.;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn update_safe_lat_dist(&mut self, travelledLatDist: f64) {
        todo!("port MSLCM_SL2015::updateSafeLatDist")
    }

    /// C++: `std::string MSLCM_SL2015::getParameter(const std::string &) const` [const, virtual]
    /// try to retrieve the given parameter from this device. Throw exception for unsupported key
    // std::string
    // MSLCM_SL2015::getParameter(const std::string& key) const {
    //     if (key == toString(SUMO_ATTR_LCA_STRATEGIC_PARAM)) {
    //         return toString(myStrategicParam);
    //     } else if (key == toString(SUMO_ATTR_LCA_COOPERATIVE_PARAM)) {
    //         return toString(myCooperativeParam);
    //     } else if (key == toString(SUMO_ATTR_LCA_SPEEDGAIN_PARAM)) {
    //         return toString(mySpeedGainParam);
    //     } else if (key == toString(SUMO_ATTR_LCA_KEEPRIGHT_PARAM)) {
    //         return toString(myKeepRightParam);
    //     } else if (key == toString(SUMO_ATTR_LCA_OPPOSITE_PARAM)) {
    //         return toString(myOppositeParam);
    //     } else if (key == toString(SUMO_ATTR_LCA_SUBLANE_PARAM)) {
    //         return toString(mySublaneParam);
    //     } else if (key == toString(SUMO_ATTR_MINGAP_LAT)) {
    //         return toString(myMinGapLat);
    //     } else if (key == toString(SUMO_ATTR_LCA_PUSHY)) {
    //         return toString(myPushy);
    //     } else if (key == toString(SUMO_ATTR_LCA_PUSHYGAP)) {
    //         return toString((myPushy - 1) * myMinGapLat);
    //     } else if (key == toString(SUMO_ATTR_LCA_ASSERTIVE)) {
    //         return toString(myAssertive);
    //     } else if (key == toString(SUMO_ATTR_LCA_IMPATIENCE)) {
    //         return toString(myImpatience);
    //     } else if (key == toString(SUMO_ATTR_LCA_TIME_TO_IMPATIENCE)) {
    //         return toString(myTimeToImpatience);
    //     } else if (key == toString(SUMO_ATTR_LCA_ACCEL_LAT)) {
    //         return toString(myAccelLat);
    //     } else if (key == toString(SUMO_ATTR_LCA_LOOKAHEADLEFT)) {
    //         return toString(myLookaheadLeft);
    //     } else if (key == toString(SUMO_ATTR_LCA_SPEEDGAINRIGHT)) {
    //         return toString(mySpeedGainRight);
    //     } else if (key == toString(SUMO_ATTR_LCA_LANE_DISCIPLINE)) {
    //         return toString(myLaneDiscipline);
    //     } else if (key == toString(SUMO_ATTR_LCA_SIGMA)) {
    //         return toString(mySigma);
    //     } else if (key == toString(SUMO_ATTR_LCA_KEEPRIGHT_ACCEPTANCE_TIME)) {
    //         return toString(myKeepRightAcceptanceTime);
    //     } else if (key == toString(SUMO_ATTR_LCA_OVERTAKE_DELTASPEED_FACTOR)) {
    //         return toString(myOvertakeDeltaSpeedFactor);
    //     } else if (key == toString(SUMO_ATTR_LCA_SPEEDGAIN_LOOKAHEAD)) {
    //         return toString(mySpeedGainLookahead);
    //     } else if (key == toString(SUMO_ATTR_LCA_SPEEDGAIN_REMAIN_TIME)) {
    //         return toString(mySpeedGainRemainTime);
    //     } else if (key == toString(SUMO_ATTR_LCA_COOPERATIVE_ROUNDABOUT)) {
    //         return toString(myRoundaboutBonus);
    //     } else if (key == toString(SUMO_ATTR_LCA_COOPERATIVE_SPEED)) {
    //         return toString(myCooperativeSpeed);
    //     } else if (key == toString(SUMO_ATTR_LCA_MAXSPEEDLATSTANDING)) {
    //         return toString(myMaxSpeedLatStanding);
    //     } else if (key == toString(SUMO_ATTR_LCA_MAXSPEEDLATFACTOR)) {
    //         return toString(myMaxSpeedLatFactor);
    //     } else if (key == toString(SUMO_ATTR_LCA_MAXDISTLATSTANDING)) {
    //         return toString(myMaxDistLatStanding);
    //         // access to internal state for debugging in sumo-gui (not documented since it may change at any time)
    //     } else if (key == "speedGainProbabilityRight") {
    //         return toString(mySpeedGainProbabilityRight);
    //     } else if (key == "speedGainProbabilityLeft") {
    //         return toString(mySpeedGainProbabilityLeft);
    //     } else if (key == "keepRightProbability") {
    //         return toString(myKeepRightProbability);
    //     } else if (key == "lookAheadSpeed") {
    //         return toString(myLookAheadSpeed);
    //     } else if (key == "sigmaState") {
    //         return toString(mySigmaState);
    //         // motivation relative to threshold
    //     } else if (key == "speedGainRP") {
    //         return toString(mySpeedGainProbabilityRight / myChangeProbThresholdRight);
    //     } else if (key == "speedGainLP") {
    //         return toString(mySpeedGainProbabilityLeft / myChangeProbThresholdLeft);
    //     } else if (key == "keepRightP") {
    //         return toString(myKeepRightProbability * myKeepRightParam / myChangeProbThresholdRight);
    //     }
    //     throw InvalidArgument("Parameter '" + key + "' is not supported for laneChangeModel of type '" + toString(myModel) + "'");
    // }
    #[allow(unused_variables)]
    pub fn get_parameter(&self, key: &str) -> String {
        todo!("port MSLCM_SL2015::getParameter")
    }

    /// C++: `void MSLCM_SL2015::setParameter(const std::string &, const std::string &)` [virtual]
    /// try to set the given parameter for this laneChangeModel. Throw exception for unsupported key
    // void
    // MSLCM_SL2015::setParameter(const std::string& key, const std::string& value) {
    //     double doubleValue;
    //     try {
    //         doubleValue = StringUtils::toDouble(value);
    //     } catch (NumberFormatException&) {
    //         throw InvalidArgument("Setting parameter '" + key + "' requires a number for laneChangeModel of type '" + toString(myModel) + "'");
    //     }
    //     if (key == toString(SUMO_ATTR_LCA_STRATEGIC_PARAM)) {
    //         myStrategicParam = doubleValue;
    //     } else if (key == toString(SUMO_ATTR_LCA_COOPERATIVE_PARAM)) {
    //         myCooperativeParam = doubleValue;
    //     } else if (key == toString(SUMO_ATTR_LCA_SPEEDGAIN_PARAM)) {
    //         mySpeedGainParam = doubleValue;
    //     } else if (key == toString(SUMO_ATTR_LCA_KEEPRIGHT_PARAM)) {
    //         myKeepRightParam = doubleValue;
    //     } else if (key == toString(SUMO_ATTR_LCA_OPPOSITE_PARAM)) {
    //         myOppositeParam = doubleValue;
    //     } else if (key == toString(SUMO_ATTR_LCA_SUBLANE_PARAM)) {
    //         mySublaneParam = doubleValue;
    //     } else if (key == toString(SUMO_ATTR_MINGAP_LAT)) {
    //         myMinGapLat = doubleValue;
    //     } else if (key == toString(SUMO_ATTR_LCA_PUSHY)) {
    //         myPushy = doubleValue;
    //     } else if (key == toString(SUMO_ATTR_LCA_PUSHYGAP)) {
    //         myPushy = 1 - doubleValue / myMinGapLat;
    //     } else if (key == toString(SUMO_ATTR_LCA_ASSERTIVE)) {
    //         myAssertive = doubleValue;
    //     } else if (key == toString(SUMO_ATTR_LCA_IMPATIENCE)) {
    //         myImpatience = doubleValue;
    //         myMinImpatience = doubleValue;
    //     } else if (key == toString(SUMO_ATTR_LCA_TIME_TO_IMPATIENCE)) {
    //         myTimeToImpatience = doubleValue;
    //     } else if (key == toString(SUMO_ATTR_LCA_ACCEL_LAT)) {
    //         myAccelLat = doubleValue;
    //     } else if (key == toString(SUMO_ATTR_LCA_TURN_ALIGNMENT_DISTANCE)) {
    //         myTurnAlignmentDist = doubleValue;
    //     } else if (key == toString(SUMO_ATTR_LCA_LOOKAHEADLEFT)) {
    //         myLookaheadLeft = doubleValue;
    //     } else if (key == toString(SUMO_ATTR_LCA_SPEEDGAINRIGHT)) {
    //         mySpeedGainRight = doubleValue;
    //     } else if (key == toString(SUMO_ATTR_LCA_LANE_DISCIPLINE)) {
    //         myLaneDiscipline = doubleValue;
    //     } else if (key == toString(SUMO_ATTR_LCA_SIGMA)) {
    //         mySigma = doubleValue;
    //     } else if (key == toString(SUMO_ATTR_LCA_KEEPRIGHT_ACCEPTANCE_TIME)) {
    //         myKeepRightAcceptanceTime = doubleValue;
    //     } else if (key == toString(SUMO_ATTR_LCA_OVERTAKE_DELTASPEED_FACTOR)) {
    //         myOvertakeDeltaSpeedFactor = doubleValue;
    //     } else if (key == toString(SUMO_ATTR_LCA_SPEEDGAIN_LOOKAHEAD)) {
    //         mySpeedGainLookahead = doubleValue;
    //     } else if (key == toString(SUMO_ATTR_LCA_SPEEDGAIN_REMAIN_TIME)) {
    //         mySpeedGainRemainTime = doubleValue;
    //     } else if (key == toString(SUMO_ATTR_LCA_COOPERATIVE_ROUNDABOUT)) {
    //         myRoundaboutBonus = doubleValue;
    //     } else if (key == toString(SUMO_ATTR_LCA_COOPERATIVE_SPEED)) {
    //         myCooperativeSpeed = doubleValue;
    //     } else if (key == toString(SUMO_ATTR_LCA_MAXSPEEDLATSTANDING)) {
    //         myMaxSpeedLatStanding = doubleValue;
    //     } else if (key == toString(SUMO_ATTR_LCA_MAXSPEEDLATFACTOR)) {
    //         myMaxSpeedLatFactor = doubleValue;
    //     } else if (key == toString(SUMO_ATTR_LCA_MAXDISTLATSTANDING)) {
    //         myMaxDistLatStanding = doubleValue;
    //         // access to internal state
    //     } else if (key == "speedGainProbabilityRight") {
    //         mySpeedGainProbabilityRight = doubleValue;
    //     } else if (key == "speedGainProbabilityLeft") {
    //         mySpeedGainProbabilityLeft = doubleValue;
    //     } else if (key == "keepRightProbability") {
    //         myKeepRightProbability = doubleValue;
    //     } else if (key == "lookAheadSpeed") {
    //         myLookAheadSpeed = doubleValue;
    //     } else if (key == "sigmaState") {
    //         mySigmaState = doubleValue;
    //     } else {
    //         throw InvalidArgument("Setting parameter '" + key + "' is not supported for laneChangeModel of type '" + toString(myModel) + "'");
    //     }
    //     initDerivedParameters();
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(&mut self, key: &str, value: &str) {
        todo!("port MSLCM_SL2015::setParameter")
    }

    /// C++: `double MSLCM_SL2015::computeSpeedLat(double, double &, bool) const` [const, virtual]
    /// decides the next lateral speed depending on the remaining lane change distance to be covered and updates maneuverDist according to lateral safety constraints.
    // double
    // MSLCM_SL2015::computeSpeedLat(double latDist, double& maneuverDist, bool urgent) const {
    //     int currentDirection = mySpeedLat >= 0 ? 1 : -1;
    //     int directionWish = latDist >= 0 ? 1 : -1;
    //     double maxSpeedLat = myVehicle.getVehicleType().getMaxSpeedLat();
    //     double accelLat = myAccelLat;
    //     if (!urgent && (myLeftSpace > POSITION_EPS || myMaxSpeedLatFactor < 0)) {
    //         const double speedBound = myMaxSpeedLatStanding + myMaxSpeedLatFactor * myVehicle.getSpeed();
    //         if (myMaxSpeedLatFactor >= 0) {
    //             // speedbound increases with speed and needs an upper bound
    //             maxSpeedLat = MIN2(maxSpeedLat, speedBound);
    //         } else {
    //             // speedbound decreases with speed and needs a lower bound
    //             // (only useful if myMaxSpeedLatStanding > maxSpeedLat)
    //             maxSpeedLat = MAX2(maxSpeedLat, speedBound);
    //             // increase (never decrease) lateral acceleration in proportion
    //             accelLat *= MAX2(1.0, speedBound / myVehicle.getVehicleType().getMaxSpeedLat());
    //         }
    //     }
    //     if (getWidth() < myVehicle.getCurrentEdge()->getWidth() && !isOpposite()) {
    //         const double rightVehSide = myVehicle.getRightSideOnEdge();
    //         const double edgeOverlap = MAX2(-rightVehSide, rightVehSide + myVehicle.getVehicleType().getWidth() - myVehicle.getCurrentEdge()->getWidth());
    //         // if vehicle is outside edge bounds. Permit stronger lateral maneuvering
    //         accelLat = MAX2(accelLat, 2 * edgeOverlap);
    //         maxSpeedLat = MAX2(maxSpeedLat, edgeOverlap);
    //     }
    //
    // #ifdef DEBUG_MANEUVER
    //     if (debugVehicle()) {
    //         std::cout << SIMTIME
    //                   << " veh=" << myVehicle.getID()
    //                   << " computeSpeedLat()"
    //                   << " latDist=" << latDist
    //                   << " maneuverDist=" << maneuverDist
    //                   << " urgent=" << urgent
    //                   << " speedLat=" << mySpeedLat
    //                   << " currentDirection=" << currentDirection
    //                   << " directionWish=" << directionWish
    //                   << " myLeftSpace=" << myLeftSpace
    //                   << " maxSpeedLat=" << maxSpeedLat
    //                   << std::endl;
    //     }
    // #endif
    //     // reduced lateral speed (in the desired direction). Don't change direction against desired.
    //     double speedDecel;
    //     if (directionWish == 1) {
    //         speedDecel = MAX2(mySpeedLat - ACCEL2SPEED(accelLat), 0.);
    //     } else {
    //         speedDecel = MIN2(mySpeedLat + ACCEL2SPEED(accelLat), 0.);
    //     }
    //     // increased lateral speed (in the desired direction)
    //     double speedAccel = MAX2(MIN2(mySpeedLat + directionWish * ACCEL2SPEED(accelLat), maxSpeedLat), -maxSpeedLat);
    //
    //     // can we reach the target distance in a single step? (XXX: assumes "Euler" update)
    //     double speedBound = DIST2SPEED(latDist);
    //     // for lat-gap keeping maneuvres myOrigLatDist may be 0
    //     const double fullLatDist = latDist > 0 ? MIN2(mySafeLatDistLeft, MAX2(maneuverDist, latDist)) : MAX2(-mySafeLatDistRight, MIN2(maneuverDist, latDist));
    //
    //     // update maneuverDist, if safety constraints apply in its direction
    //     if (maneuverDist * latDist > 0) {
    //         maneuverDist = fullLatDist;
    //     }
    //
    // #ifdef DEBUG_MANEUVER
    //     if (debugVehicle()) {
    //         std::cout << "     mySafeLatDistRight=" << mySafeLatDistRight
    //                   << " mySafeLatDistLeft=" << mySafeLatDistLeft
    //                   << " fullLatDist=" << fullLatDist
    //                   << " speedAccel=" << speedAccel
    //                   << " speedDecel=" << speedDecel
    //                   << " speedBound=" << speedBound
    //                   << std::endl;
    //     }
    // #endif
    //     if (speedDecel * speedAccel <= 0 && (
    //                 // speedAccel and speedDecel bracket speed 0. This means we can end the maneuver
    //                 (latDist >= 0 && speedAccel >= speedBound && speedBound >= speedDecel)
    //                 || (latDist <= 0 && speedAccel <= speedBound && speedBound <= speedDecel))) {
    //         // we can reach the desired value in this step
    // #ifdef DEBUG_MANEUVER
    //         if (debugVehicle()) {
    //             std::cout << "   computeSpeedLat a)\n";
    //         }
    // #endif
    //         return speedBound;
    //     }
    //     // are we currently moving in the wrong direction?
    //     if (latDist * mySpeedLat < 0) {
    // #ifdef DEBUG_MANEUVER
    //         if (debugVehicle()) {
    //             std::cout << "   computeSpeedLat b)\n";
    //         }
    // #endif
    //         return emergencySpeedLat(speedAccel);
    //     }
    //     // check if the remaining distance allows to accelerate laterally
    //     double minDistAccel = SPEED2DIST(speedAccel) + currentDirection * MSCFModel::brakeGapEuler(fabs(speedAccel), accelLat, 0); // most we can move in the target direction
    //     if ((fabs(minDistAccel) < fabs(fullLatDist)) || (fabs(minDistAccel - fullLatDist) < NUMERICAL_EPS)) {
    // #ifdef DEBUG_MANEUVER
    //         if (debugVehicle()) {
    //             std::cout << "   computeSpeedLat c)\n";
    //         }
    // #endif
    //         return speedAccel;
    //     } else {
    // #ifdef DEBUG_MANEUVER
    //         if (debugVehicle()) {
    //             std::cout << "      minDistAccel=" << minDistAccel << "\n";
    //         }
    // #endif
    //         // check if the remaining distance allows to maintain current lateral speed
    //         double minDistCurrent = SPEED2DIST(mySpeedLat) + currentDirection * MSCFModel::brakeGapEuler(fabs(mySpeedLat), accelLat, 0);
    //         if ((fabs(minDistCurrent) < fabs(fullLatDist)) || (fabs(minDistCurrent - fullLatDist) < NUMERICAL_EPS)) {
    // #ifdef DEBUG_MANEUVER
    //             if (debugVehicle()) {
    //                 std::cout << "   computeSpeedLat d)\n";
    //             }
    // #endif
    //             return mySpeedLat;
    //         }
    //     }
    //     // reduce lateral speed
    // #ifdef DEBUG_MANEUVER
    //     if (debugVehicle()) {
    //         std::cout << "   computeSpeedLat e)\n";
    //     }
    // #endif
    //     return emergencySpeedLat(speedDecel);
    // }
    #[allow(unused_variables)]
    pub fn compute_speed_lat(&self, latDist: f64, maneuverDist: &mut f64, urgent: bool) -> f64 {
        todo!("port MSLCM_SL2015::computeSpeedLat")
    }

    /// C++: `LatAlignmentDefinition MSLCM_SL2015::getDesiredAlignment() const` [const, virtual]
    // LatAlignmentDefinition
    // MSLCM_SL2015::getDesiredAlignment() const {
    //     LatAlignmentDefinition align = MSAbstractLaneChangeModel::getDesiredAlignment();
    //     // Check whether the vehicle should adapt its alignment to an upcoming turn
    //     if (myTurnAlignmentDist > 0) {
    //         const std::pair<double, const MSLink*>& turnInfo = myVehicle.getNextTurn();
    //         const LinkDirection turnDir = turnInfo.second == nullptr ? LinkDirection::NODIR : turnInfo.second->getDirection();
    //         const bool indirect = turnInfo.second == nullptr ? false : turnInfo.second->isIndirect();
    //         if (turnInfo.first < myTurnAlignmentDist) {
    //             // Vehicle is close enough to the link to change its default alignment
    //             switch (turnDir) {
    //                 case LinkDirection::TURN:
    //                 case LinkDirection::LEFT:
    //                 case LinkDirection::PARTLEFT:
    //                     if (myVehicle.getLane()->getBidiLane() == nullptr) {
    //                         // no left alignment on bidi lane to avoid blocking oncoming traffic
    //                         align = MSGlobals::gLefthand != indirect ? LatAlignmentDefinition::RIGHT : LatAlignmentDefinition::LEFT;
    //                     }
    //                     break;
    //                 case LinkDirection::TURN_LEFTHAND:
    //                 case LinkDirection::RIGHT:
    //                 case LinkDirection::PARTRIGHT:
    //                     align = MSGlobals::gLefthand != indirect ? LatAlignmentDefinition::LEFT : LatAlignmentDefinition::RIGHT;
    //                     break;
    //                 case LinkDirection::STRAIGHT:
    //                 case LinkDirection::NODIR:
    //                 default:
    //                     break;
    //             }
    //         }
    //     }
    //     return align;
    // }
    #[allow(unused_variables)]
    pub fn get_desired_alignment(&self) -> LatAlignmentDefinition {
        todo!("port MSLCM_SL2015::getDesiredAlignment")
    }

    /// C++: `double MSLCM_SL2015::_patchSpeed(double, const double, double, const MSCFModel &)` [protected]
    /// helper function which contains the actual logic
    // double
    // MSLCM_SL2015::_patchSpeed(double min, const double wanted, double max, const MSCFModel& cfModel) {
    //     if (wanted <= 0) {
    //         return wanted;
    //     }
    //
    //     int state = myOwnState;
    //
    //     double nVSafe = wanted;
    //     bool gotOne = false;
    //     // letting vehicles merge in at the end of the lane in case of counter-lane change, step#2
    //     //   if we want to change and have a blocking leader and there is enough room for him in front of us
    //     if (myLeadingBlockerLength != 0) {
    //         double space = myLeftSpace - myLeadingBlockerLength - POSITION_EPS;
    // #ifdef DEBUG_PATCHSPEED
    //         if (gDebugFlag2) {
    //             std::cout << SIMTIME << " veh=" << myVehicle.getID() << " myLeadingBlockerLength=" << myLeadingBlockerLength << " space=" << space << "\n";
    //         }
    // #endif
    //         if (space >= 0) {
    //             // compute speed for decelerating towards a place which allows the blocking leader to merge in in front
    //             double safe = cfModel.stopSpeed(&myVehicle, myVehicle.getSpeed(), space, MSCFModel::CalcReason::LANE_CHANGE);
    //             max = MIN2(max, safe);
    //             // if we are approaching this place
    //             if (safe < wanted) {
    //                 if (safe < min) {
    //                     const double vMinEmergency = myVehicle.getCarFollowModel().minNextSpeedEmergency(myVehicle.getSpeed(), &myVehicle);
    //                     if (safe >= vMinEmergency) {
    //                         // permit harder braking if needed and helpful
    //                         min = MAX2(vMinEmergency, safe);
    //                     }
    //                 }
    // #ifdef DEBUG_PATCHSPEED
    //                 if (gDebugFlag2) {
    //                     std::cout << SIMTIME << " veh=" << myVehicle.getID() << " slowing down for leading blocker, safe=" << safe << (safe + NUMERICAL_EPS < min ? " (not enough)" : "") << "\n";
    //                 }
    // #endif
    //                 nVSafe = MAX2(min, safe);
    //                 gotOne = true;
    //             }
    //         }
    //     }
    //     const double coopWeight = MAX2(0.0, MIN2(1.0, myCooperativeSpeed));
    //     for (auto i : myLCAccelerationAdvices) {
    //         double accel = i.first;
    //         double v = myVehicle.getSpeed() + ACCEL2SPEED(accel);
    //         if (v >= min && v <= max) {
    //             if (i.second & LCA_CHANGE_TO_HELP) {
    //                 nVSafe = MIN2(v * coopWeight + (1 - coopWeight) * wanted, nVSafe);
    //             } else {
    //                 // own advice, no scaling needed
    //                 nVSafe = MIN2(v, nVSafe);
    //             }
    //             gotOne = true;
    // #ifdef DEBUG_PATCHSPEED
    //             if (gDebugFlag2) {
    //                 std::cout << SIMTIME << " veh=" << myVehicle.getID() << " got accel=" << accel << " nVSafe=" << nVSafe << "\n";
    //             }
    // #endif
    //         } else {
    // #ifdef DEBUG_PATCHSPEED
    //             if (v < min) {
    //                 if (gDebugFlag2) {
    //                     std::cout << SIMTIME << " veh=" << myVehicle.getID() << " ignoring low nVSafe=" << v << " (accel=" << accel << ") min=" << min << "\n";
    //                 }
    //             } else {
    //                 if (gDebugFlag2) {
    //                     std::cout << SIMTIME << " veh=" << myVehicle.getID() << " ignoring high nVSafe=" << v << " (accel=" << accel << ") max=" << max << "\n";
    //                 }
    //             }
    // #endif
    //         }
    //     }
    //
    //     if (gotOne && !myDontBrake) {
    // #ifdef DEBUG_PATCHSPEED
    //         if (gDebugFlag2) {
    //             std::cout << SIMTIME << " veh=" << myVehicle.getID() << " got vSafe\n";
    //         }
    // #endif
    //         return nVSafe;
    //     }
    //
    //     // check whether the vehicle is blocked
    //     if ((state & LCA_WANTS_LANECHANGE) != 0 && (state & LCA_BLOCKED) != 0) {
    //         if ((state & LCA_STRATEGIC) != 0) {
    //             // necessary decelerations are controlled via vSafe. If there are
    //             // none it means we should speed up
    // #if defined(DEBUG_PATCHSPEED) || defined(DEBUG_STATE)
    //             if (gDebugFlag2) {
    //                 std::cout << SIMTIME << " veh=" << myVehicle.getID() << " LCA_WANTS_LANECHANGE (strat, no vSafe)\n";
    //             }
    // #endif
    //             return (max + wanted) / 2.0;
    //         } else if ((state & LCA_COOPERATIVE) != 0) {
    //             // only minor adjustments in speed should be done
    //             if ((state & LCA_BLOCKED_BY_LEADER) != 0) {
    // #if defined(DEBUG_PATCHSPEED) || defined(DEBUG_STATE)
    //                 if (gDebugFlag2) {
    //                     std::cout << SIMTIME << " veh=" << myVehicle.getID() << " LCA_BLOCKED_BY_LEADER (coop)\n";
    //                 }
    // #endif
    //                 return (min + wanted) / 2.0;
    //             }
    //             if ((state & LCA_BLOCKED_BY_FOLLOWER) != 0) {
    // #if defined(DEBUG_PATCHSPEED) || defined(DEBUG_STATE)
    //                 if (gDebugFlag2) {
    //                     std::cout << SIMTIME << " veh=" << myVehicle.getID() << " LCA_BLOCKED_BY_FOLLOWER (coop)\n";
    //                 }
    // #endif
    //                 return (max + wanted) / 2.0;
    //             }
    //             //} else { // VARIANT_16
    //             //    // only accelerations should be performed
    //             //    if ((state & LCA_BLOCKED_BY_FOLLOWER) != 0) {
    //             //        if (gDebugFlag2) std::cout << SIMTIME << " veh=" << myVehicle.getID() << " LCA_BLOCKED_BY_FOLLOWER\n";
    //             //        return (max + wanted) / 2.0;
    //             //    }
    //         }
    //     }
    //
    //     /*
    //     // decelerate if being a blocking follower
    //     //  (and does not have to change lanes)
    //     if ((state & LCA_AMBLOCKINGFOLLOWER) != 0) {
    //         if (fabs(max - myVehicle.getCarFollowModel().maxNextSpeed(myVehicle.getSpeed(), &myVehicle)) < 0.001 && min == 0) { // !!! was standing
    //             if (gDebugFlag2) std::cout << SIMTIME << " veh=" << myVehicle.getID() << " LCA_AMBLOCKINGFOLLOWER (standing)\n";
    //             return 0;
    //         }
    //         if (gDebugFlag2) std::cout << SIMTIME << " veh=" << myVehicle.getID() << " LCA_AMBLOCKINGFOLLOWER\n";
    //
    //         //return min; // VARIANT_3 (brakeStrong)
    //         return (min + wanted) / 2.0;
    //     }
    //     if ((state & LCA_AMBACKBLOCKER) != 0) {
    //         if (max <= myVehicle.getCarFollowModel().maxNextSpeed(myVehicle.getSpeed(), &myVehicle) && min == 0) { // !!! was standing
    //             if (gDebugFlag2) std::cout << SIMTIME << " veh=" << myVehicle.getID() << " LCA_AMBACKBLOCKER (standing)\n";
    //             //return min; VARIANT_9 (backBlockVSafe)
    //             return nVSafe;
    //         }
    //     }
    //     if ((state & LCA_AMBACKBLOCKER_STANDING) != 0) {
    //         if (gDebugFlag2) std::cout << SIMTIME << " veh=" << myVehicle.getID() << " LCA_AMBACKBLOCKER_STANDING\n";
    //         //return min;
    //         return nVSafe;
    //     }
    //     */
    //
    //     // accelerate if being a blocking leader or blocking follower not able to brake
    //     //  (and does not have to change lanes)
    //     if ((state & LCA_AMBLOCKINGLEADER) != 0) {
    // #if defined(DEBUG_PATCHSPEED) || defined(DEBUG_STATE)
    //         if (gDebugFlag2) {
    //             std::cout << SIMTIME << " veh=" << myVehicle.getID() << " LCA_AMBLOCKINGLEADER\n";
    //         }
    // #endif
    //         return (max + wanted) / 2.0;
    //     }
    //
    //     if ((state & LCA_AMBLOCKINGFOLLOWER_DONTBRAKE) != 0) {
    // #if defined(DEBUG_PATCHSPEED) || defined(DEBUG_STATE)
    //         if (gDebugFlag2) {
    //             std::cout << SIMTIME << " veh=" << myVehicle.getID() << " LCA_AMBLOCKINGFOLLOWER_DONTBRAKE\n";
    //         }
    // #endif
    //         /*
    //         // VARIANT_4 (dontbrake)
    //         if (max <= myVehicle.getCarFollowModel().maxNextSpeed(myVehicle.getSpeed(), &myVehicle) && min == 0) { // !!! was standing
    //             return wanted;
    //         }
    //         return (min + wanted) / 2.0;
    //         */
    //     }
    //     return wanted;
    // }
    #[allow(unused_variables)]
    pub fn patch_speed_2(&mut self, min: f64, wanted: f64, max: f64, cfModel: &MSCFModel) -> f64 {
        todo!("port MSLCM_SL2015::_patchSpeed")
    }

    /// C++: `int MSLCM_SL2015::_wantsChangeSublane(int, LaneChangeAction, const MSLeaderDistanceInfo &, const MSLeaderDistanceInfo &, const MSLeaderDistanceInfo &, const MSLeaderDistanceInfo &, const MSLeaderDistanceInfo &, const MSLeaderDistanceInfo &, const MSLane &, const std::vector<MSVehicle::LaneQ> &, MSVehicle **, MSVehicle **, double &, double &, int &)` [protected]
    /// helper function for doing the actual work
    // int
    // MSLCM_SL2015::_wantsChangeSublane(
    //     int laneOffset,
    //     LaneChangeAction alternatives,
    //     const MSLeaderDistanceInfo& leaders,
    //     const MSLeaderDistanceInfo& followers,
    //     const MSLeaderDistanceInfo& blockers,
    //     const MSLeaderDistanceInfo& neighLeaders,
    //     const MSLeaderDistanceInfo& neighFollowers,
    //     const MSLeaderDistanceInfo& neighBlockers,
    //     const MSLane& neighLane,
    //     const std::vector<MSVehicle::LaneQ>& preb,
    //     MSVehicle** lastBlocked,
    //     MSVehicle** firstBlocked,
    //     double& latDist, double& maneuverDist, int& blocked) {
    //
    //     if (laneOffset != 0) {
    //         // update mySafeLatDist w.r.t. the direction being checkd
    //         const double halfWidth = getWidth() * 0.5;
    //         double center = getVehicleCenter();
    //         if (laneOffset < 0) {
    //             mySafeLatDistRight = center - halfWidth;
    //         } else  {
    //             mySafeLatDistLeft = getLeftBorder() - center - halfWidth;
    //         }
    //     }
    //
    //     const SUMOTime currentTime = MSNet::getInstance()->getCurrentTimeStep();
    //     // compute bestLaneOffset
    //     MSVehicle::LaneQ curr, neigh, best;
    //     int bestLaneOffset = 0;
    //     double currentDist = 0;
    //     double neighDist = 0;
    //     const MSLane* prebLane = myVehicle.getLane();
    //     if (prebLane->getEdge().isInternal()) {
    //         // internal edges are not kept inside the bestLanes structure
    //         if (isOpposite()) {
    //             prebLane = prebLane->getNormalPredecessorLane();
    //         } else {
    //             prebLane = prebLane->getLinkCont()[0]->getLane();
    //         }
    //     }
    //     // special case: vehicle considers changing to the opposite direction edge
    //     const bool checkOpposite = &neighLane.getEdge() != &myVehicle.getLane()->getEdge();
    //     const int prebOffset = (checkOpposite ? 0 : laneOffset);
    //     for (int p = 0; p < (int) preb.size(); ++p) {
    //         if (preb[p].lane == prebLane && p + laneOffset >= 0) {
    //             assert(p + prebOffset < (int)preb.size());
    //             curr = preb[p];
    //             neigh = preb[p + prebOffset];
    //             currentDist = curr.length;
    //             neighDist = neigh.length;
    //             bestLaneOffset = curr.bestLaneOffset;
    //             // VARIANT_13 (equalBest)
    //             if (bestLaneOffset == 0 && preb[p + prebOffset].bestLaneOffset == 0 && !checkOpposite) {
    // #ifdef DEBUG_WANTSCHANGE
    //                 if (gDebugFlag2) {
    //                     std::cout << STEPS2TIME(currentTime)
    //                               << " veh=" << myVehicle.getID()
    //                               << " bestLaneOffsetOld=" << bestLaneOffset
    //                               << " bestLaneOffsetNew=" << laneOffset
    //                               << "\n";
    //                 }
    // #endif
    //                 bestLaneOffset = prebOffset;
    //             }
    //             best = preb[p + bestLaneOffset];
    //             break;
    //         }
    //     }
    //     assert(curr.lane != nullptr);
    //     assert(neigh.lane != nullptr);
    //     assert(best.lane != nullptr);
    //     double driveToNextStop = -std::numeric_limits<double>::max();
    //     UNUSED_PARAMETER(driveToNextStop); // XXX use when computing usableDist
    //     if (myVehicle.nextStopDist() < std::numeric_limits<double>::max()
    //             && &myVehicle.getNextStop().lane->getEdge() == &myVehicle.getLane()->getEdge()) {
    //         // vehicle can always drive up to stop distance
    //         // @note this information is dynamic and thus not available in updateBestLanes()
    //         // @note: nextStopDist was compute before the vehicle moved
    //         driveToNextStop = myVehicle.nextStopDist();
    //         const double stopPos = getForwardPos() + myVehicle.nextStopDist() - myVehicle.getLastStepDist();
    // #ifdef DEBUG_WANTS_CHANGE
    //         if (DEBUG_COND) {
    //             std::cout << SIMTIME << std::setprecision(gPrecision) << " veh=" << myVehicle.getID()
    //                       << " stopDist=" << myVehicle.nextStopDist()
    //                       << " lastDist=" << myVehicle.getLastStepDist()
    //                       << " stopPos=" << stopPos
    //                       << " currentDist=" << currentDist
    //                       << " neighDist=" << neighDist
    //                       << "\n";
    //         }
    // #endif
    //         currentDist = MAX2(currentDist, stopPos);
    //         neighDist = MAX2(neighDist, stopPos);
    //     }
    //     // direction specific constants
    //     const bool right = (laneOffset == -1);
    //     const bool left = (laneOffset == 1);
    //     const int myLca = (right ? LCA_MRIGHT : (left ? LCA_MLEFT : 0));
    //     const int lcaCounter = (right ? LCA_LEFT : (left ? LCA_RIGHT : LCA_NONE));
    //     const bool changeToBest = (right && bestLaneOffset < 0) || (left && bestLaneOffset > 0) || (laneOffset == 0 && bestLaneOffset == 0);
    //     // keep information about being a leader/follower but remove information
    //     // about previous lane change request or urgency
    //     int ret = (myOwnState & 0xffff0000);
    //
    //     // compute the distance when changing to the neighboring lane
    //     // (ensure we do not lap into the line behind neighLane since there might be unseen blockers)
    //     // minimum distance to move the vehicle fully onto the new lane
    //     double latLaneDist = laneOffset == 0 ? 0. : myVehicle.lateralDistanceToLane(laneOffset);
    //
    //     // VARIANT_5 (disableAMBACKBLOCKER1)
    //     /*
    //     if (leader.first != 0
    //             && (myOwnState & LCA_AMBLOCKINGFOLLOWER_DONTBRAKE) != 0
    //             && (leader.first->getLaneChangeModel().getOwnState() & LCA_AMBLOCKINGFOLLOWER_DONTBRAKE) != 0) {
    //
    //         myOwnState &= (0xffffffff - LCA_AMBLOCKINGFOLLOWER_DONTBRAKE);
    //         if (myVehicle.getSpeed() > SUMO_const_haltingSpeed) {
    //             myOwnState |= LCA_AMBACKBLOCKER;
    //         } else {
    //             ret |= LCA_AMBACKBLOCKER;
    //             myDontBrake = true;
    //         }
    //     }
    //     */
    //
    // #ifdef DEBUG_WANTSCHANGE
    //     if (gDebugFlag2) {
    //         std::cout << STEPS2TIME(currentTime)
    //                   << " veh=" << myVehicle.getID()
    //                   << " myState=" << toString((LaneChangeAction)myOwnState)
    //                   << " firstBlocked=" << Named::getIDSecure(*firstBlocked)
    //                   << " lastBlocked=" << Named::getIDSecure(*lastBlocked)
    //                   << "\n         leaders=" << leaders.toString()
    //                   << "\n       followers=" << followers.toString()
    //                   << "\n        blockers=" << blockers.toString()
    //                   << "\n    neighLeaders=" << neighLeaders.toString()
    //                   << "\n  neighFollowers=" << neighFollowers.toString()
    //                   << "\n   neighBlockers=" << neighBlockers.toString()
    //                   << "\n   changeToBest=" << changeToBest
    //                   << " latLaneDist=" << latLaneDist
    //                   << " alts=" << toString((LaneChangeAction)alternatives)
    //                   << "\n   expectedSpeeds=" << toString(myExpectedSublaneSpeeds)
    //                   << std::endl;
    //     }
    // #endif
    //
    //     ret = slowDownForBlocked(lastBlocked, ret);
    //     // VARIANT_14 (furtherBlock)
    //     if (lastBlocked != firstBlocked) {
    //         ret = slowDownForBlocked(firstBlocked, ret);
    //     }
    //
    //
    //     // we try to estimate the distance which is necessary to get on a lane
    //     //  we have to get on in order to keep our route
    //     // we assume we need something that depends on our velocity
    //     // and compare this with the free space on our wished lane
    //     //
    //     // if the free space is somehow less than the space we need, we should
    //     //  definitely try to get to the desired lane
    //     //
    //     // this rule forces our vehicle to change the lane if a lane changing is necessary soon
    //     // lookAheadDistance:
    //     // we do not want the lookahead distance to change all the time so we discrectize the speed a bit
    //
    //     // VARIANT_18 (laHyst)
    //     if (myVehicle.getSpeed() > myLookAheadSpeed) {
    //         myLookAheadSpeed = myVehicle.getSpeed();
    //     } else {
    //         // FIXME: This strongly dependent on the value of TS, see LC2013 for the fix (l.1153, currently)
    //         myLookAheadSpeed = MAX2(LOOK_AHEAD_MIN_SPEED,
    //                                 (LOOK_AHEAD_SPEED_MEMORY * myLookAheadSpeed + (1 - LOOK_AHEAD_SPEED_MEMORY) * myVehicle.getSpeed()));
    //     }
    //     //myLookAheadSpeed = myVehicle.getLane()->getVehicleMaxSpeed(&myVehicle);
    //
    //     //double laDist = laSpeed > LOOK_FORWARD_SPEED_DIVIDER
    //     //              ? laSpeed *  LOOK_FORWARD_FAR
    //     //              : laSpeed *  LOOK_FORWARD_NEAR;
    //     double laDist = myLookAheadSpeed * LOOK_FORWARD * myStrategicParam * (right ? 1 : myLookaheadLeft);
    //     laDist += myVehicle.getVehicleType().getLengthWithGap() * 2.;
    //     // aggressive drivers may elect to use reduced strategic lookahead to optimize speed
    //     /*
    //     if (mySpeedGainProbabilityRight > myChangeProbThresholdRight
    //             || mySpeedGainProbabilityLeft > myChangeProbThresholdLeft) {
    //         laDist *= MAX2(0.0, (1 - myPushy));
    //         laDist *= MAX2(0,0, (1 - myAssertive));
    //         laDist *= MAX2(0,0, (2 - mySpeedGainParam));
    //     }
    //     */
    //
    //     // react to a stopped leader on the current lane
    //     if (bestLaneOffset == 0 && leaders.hasStoppedVehicle()) {
    //         // value is doubled for the check since we change back and forth
    //         // laDist = 0.5 * (myVehicle.getVehicleType().getLengthWithGap() + leader.first->getVehicleType().getLengthWithGap());
    //         // XXX determine length of longest stopped vehicle
    //         laDist = myVehicle.getVehicleType().getLengthWithGap();
    //     } else if (checkOpposite && isOpposite() && neighLeaders.hasStoppedVehicle()) {
    //         // compute exact distance to overtake stopped vehicle
    //         laDist = 0;
    //         for (int i = 0; i < neighLeaders.numSublanes(); ++i) {
    //             CLeaderDist vehDist = neighLeaders[i];
    //             if (vehDist.first != nullptr && vehDist.first->isStopped()) {
    //                 laDist = MAX2(laDist, myVehicle.getVehicleType().getMinGap() + vehDist.second + vehDist.first->getVehicleType().getLengthWithGap());
    //             }
    //         }
    //         laDist += myVehicle.getVehicleType().getLength();
    //     }
    //     if (myStrategicParam < 0) {
    //         laDist = -1e3; // never perform strategic change
    //     }
    //
    //     // free space that is available for changing
    //     //const double neighSpeed = (neighLead.first != 0 ? neighLead.first->getSpeed() :
    //     //        neighFollow.first != 0 ? neighFollow.first->getSpeed() :
    //     //        best.lane->getSpeedLimit());
    //     // @note: while this lets vehicles change earlier into the correct direction
    //     // it also makes the vehicles more "selfish" and prevents changes which are necessary to help others
    //
    //     const double roundaboutBonus = MSLCHelper::getRoundaboutDistBonus(myVehicle, myRoundaboutBonus, curr, neigh, best);
    //     currentDist += roundaboutBonus;
    //     neighDist += roundaboutBonus;
    //
    //     ret = checkStrategicChange(ret,
    //                                neighLane,
    //                                laneOffset,
    //                                leaders,
    //                                neighLeaders,
    //                                curr, neigh, best,
    //                                bestLaneOffset,
    //                                changeToBest,
    //                                currentDist,
    //                                neighDist,
    //                                laDist,
    //                                roundaboutBonus,
    //                                latLaneDist,
    //                                checkOpposite,
    //                                latDist);
    //
    //
    //     if ((ret & LCA_STAY) != 0 && latDist == 0) {
    //         // ensure that mySafeLatDistLeft / mySafeLatDistRight are up to date for the
    //         // subsquent check with laneOffset = 0
    //         const double center = myVehicle.getCenterOnEdge();
    //         const double neighRight = getNeighRight(neighLane);
    //         updateGaps(neighLeaders, neighRight, center, 1.0, mySafeLatDistRight, mySafeLatDistLeft);
    //         updateGaps(neighFollowers, neighRight, center, 1.0, mySafeLatDistRight, mySafeLatDistLeft);
    //         // remove TraCI flags because it should not be included in "state-without-traci"
    //         ret = getCanceledState(laneOffset);
    //         return ret;
    //     }
    //     if ((ret & LCA_URGENT) != 0) {
    //         // prepare urgent lane change maneuver
    //         if (changeToBest && abs(bestLaneOffset) > 1
    //                 && curr.bestContinuations.back()->getLinkCont().size() != 0
    //            ) {
    //             // there might be a vehicle which needs to counter-lane-change one lane further and we cannot see it yet
    //             const double reserve = MIN2(myLeftSpace - POSITION_EPS, getExtraReservation(bestLaneOffset, neighDist - currentDist));
    //             myLeadingBlockerLength = MAX2(reserve, myLeadingBlockerLength);
    // #ifdef DEBUG_WANTSCHANGE
    //             if (gDebugFlag2) {
    //                 std::cout << "  reserving space for unseen blockers myLeadingBlockerLength=" << myLeadingBlockerLength << "\n";
    //             }
    // #endif
    //         }
    //
    //         // letting vehicles merge in at the end of the lane in case of counter-lane change, step#1
    //         //   if there is a leader and he wants to change to the opposite direction
    //         MSVehicle* neighLeadLongest = const_cast<MSVehicle*>(getLongest(neighLeaders).first);
    //         const bool canContinue = curr.bestContinuations.size() > 1;
    // #ifdef DEBUG_WANTSCHANGE
    //         if (DEBUG_COND) {
    //             std::cout << SIMTIME << " veh=" << myVehicle.getID() << " neighLeaders=" << neighLeaders.toString() << " longest=" << Named::getIDSecure(neighLeadLongest) << " firstBlocked=" << Named::getIDSecure(*firstBlocked) << "\n";
    //         }
    // #endif
    //         bool canReserve = MSLCHelper::updateBlockerLength(myVehicle, neighLeadLongest, lcaCounter, myLeftSpace - POSITION_EPS, canContinue, myLeadingBlockerLength);
    //         if (*firstBlocked != neighLeadLongest && tieBrakeLeader(*firstBlocked)) {
    //             canReserve &= MSLCHelper::updateBlockerLength(myVehicle, *firstBlocked, lcaCounter, myLeftSpace - POSITION_EPS, canContinue, myLeadingBlockerLength);
    //         }
    //         if (!canReserve && !isOpposite()) {
    //             // we have a low-priority relief connection
    //             // std::cout << SIMTIME << " veh=" << myVehicle.getID() << " cannotReserve for blockers\n";
    //             myDontBrake = canContinue;
    //         }
    //
    //         std::vector<CLeaderDist> collectLeadBlockers;
    //         std::vector<CLeaderDist> collectFollowBlockers;
    //         int blockedFully = 0; // whether execution of the full maneuver is blocked
    //         maneuverDist = latDist;
    //         const double gapFactor = computeGapFactor(LCA_STRATEGIC);
    //         blocked = checkBlocking(neighLane, latDist, maneuverDist, laneOffset,
    //                                 leaders, followers, blockers,
    //                                 neighLeaders, neighFollowers, neighBlockers, &collectLeadBlockers, &collectFollowBlockers,
    //                                 false, gapFactor, &blockedFully);
    //
    //         const double absLaneOffset = fabs(bestLaneOffset != 0 ? bestLaneOffset : latDist / SUMO_const_laneWidth);
    //         const double remainingSeconds = ((ret & LCA_TRACI) == 0 ?
    //                                          MAX2(STEPS2TIME(TS), myLeftSpace / MAX2(myLookAheadSpeed, NUMERICAL_EPS) / absLaneOffset / URGENCY) :
    //                                          myVehicle.getInfluencer().changeRequestRemainingSeconds(currentTime));
    //         const double plannedSpeed = informLeaders(blocked, myLca, collectLeadBlockers, remainingSeconds);
    //         // coordinate with direct obstructions
    //         if (plannedSpeed >= 0) {
    //             // maybe we need to deal with a blocking follower
    //             informFollowers(blocked, myLca, collectFollowBlockers, remainingSeconds, plannedSpeed);
    //         }
    //         if (plannedSpeed > 0) {
    //             commitManoeuvre(blocked, blockedFully, leaders, neighLeaders, neighLane, maneuverDist);
    //         }
    // #if defined(DEBUG_WANTSCHANGE) || defined(DEBUG_STATE)
    //         if (gDebugFlag2) {
    //             std::cout << STEPS2TIME(currentTime)
    //                       << " veh=" << myVehicle.getID()
    //                       << " myLeftSpace=" << myLeftSpace
    //                       << " changeFully=" << myCanChangeFully
    //                       << " blockedFully=" << toString((LaneChangeAction)blockedFully)
    //                       << " remainingSeconds=" << remainingSeconds
    //                       << " plannedSpeed=" << plannedSpeed
    //                       << " mySafeLatDistRight=" << mySafeLatDistRight
    //                       << " mySafeLatDistLeft=" << mySafeLatDistLeft
    //                       << "\n";
    //         }
    // #endif
    //         // remove TraCI flags because it should not be included in "state-without-traci"
    //         ret = getCanceledState(laneOffset);
    //         return ret;
    //     }
    //     // VARIANT_15
    //     if (roundaboutBonus > 0) {
    //
    // #ifdef DEBUG_WANTS_CHANGE
    //         if (DEBUG_COND) {
    //             std::cout << STEPS2TIME(currentTime)
    //                       << " veh=" << myVehicle.getID()
    //                       << " roundaboutBonus=" << roundaboutBonus
    //                       << " myLeftSpace=" << myLeftSpace
    //                       << "\n";
    //         }
    // #endif
    //         // try to use the inner lanes of a roundabout to increase throughput
    //         // unless we are approaching the exit
    //         if (left) {
    //             ret |= LCA_COOPERATIVE;
    //             if (!cancelRequest(ret | LCA_LEFT, laneOffset)) {
    //                 if ((ret & LCA_STAY) == 0) {
    //                     latDist = latLaneDist;
    //                     maneuverDist = latLaneDist;
    //                     blocked = checkBlocking(neighLane, latDist, maneuverDist, laneOffset,
    //                                             leaders, followers, blockers,
    //                                             neighLeaders, neighFollowers, neighBlockers);
    //                 }
    //                 return ret;
    //             } else {
    //                 ret &= ~LCA_COOPERATIVE;
    //             }
    //         } else {
    //             myKeepRightProbability = 0;
    //         }
    //     }
    //
    //     // --------
    //
    //     // -------- make place on current lane if blocking follower
    //     //if (amBlockingFollowerPlusNB()) {
    //     //    std::cout << myVehicle.getID() << ", " << currentDistAllows(neighDist, bestLaneOffset, laDist)
    //     //        << " neighDist=" << neighDist
    //     //        << " currentDist=" << currentDist
    //     //        << "\n";
    //     //}
    //     const double inconvenience = (latLaneDist < 0
    //                                   ? -mySpeedGainProbabilityRight / myChangeProbThresholdRight
    //                                   : -mySpeedGainProbabilityLeft / myChangeProbThresholdLeft);
    // #ifdef DEBUG_COOPERATE
    //     if (gDebugFlag2) {
    //         std::cout << STEPS2TIME(currentTime)
    //                   << " veh=" << myVehicle.getID()
    //                   << " amBlocking=" << amBlockingFollowerPlusNB()
    //                   << " state=" << toString((LaneChangeAction)myOwnState)
    //                   << " myLca=" << toString((LaneChangeAction)myLca)
    //                   << " prevState=" << toString((LaneChangeAction)myPreviousState)
    //                   << " inconvenience=" << inconvenience
    //                   << " origLatDist=" << getManeuverDist()
    //                   << " wantsChangeToHelp=" << (right ? "right" : "left")
    //                   << " state=" << myOwnState
    //                   << "\n";
    //     }
    // #endif
    //
    //     if (laneOffset != 0
    //             && ((amBlockingFollowerPlusNB()
    //                  // VARIANT_6 : counterNoHelp
    //                  && ((myOwnState & myLca) != 0))
    //                 ||
    //                 // continue previous cooperative change
    //                 ((myPreviousState & LCA_COOPERATIVE) != 0
    //                  && !myCanChangeFully
    //                  // change is in the right direction
    //                  && (laneOffset * getManeuverDist() > 0)))
    //             && (inconvenience < myCooperativeParam)
    //             && (changeToBest || currentDistAllows(neighDist, abs(bestLaneOffset) + 1, laDist))) {
    //
    //         // VARIANT_2 (nbWhenChangingToHelp)
    // #ifdef DEBUG_COOPERATE
    //         if (gDebugFlag2) {
    //             std::cout << "   wants cooperative change\n";
    //         }
    // #endif
    //
    //         ret |= LCA_COOPERATIVE | LCA_URGENT ;//| LCA_CHANGE_TO_HELP;
    //         if (!cancelRequest(ret | getLCA(ret, latLaneDist), laneOffset)) {
    //             latDist = amBlockingFollowerPlusNB() ? latLaneDist : getManeuverDist();
    //             maneuverDist = latDist;
    //             blocked = checkBlocking(neighLane, latDist, maneuverDist, laneOffset,
    //                                     leaders, followers, blockers,
    //                                     neighLeaders, neighFollowers, neighBlockers);
    //             return ret;
    //         } else {
    //             ret &= ~(LCA_COOPERATIVE | LCA_URGENT);
    //         }
    //     }
    //
    //     // --------
    //
    //
    //     //// -------- security checks for krauss
    //     ////  (vsafe fails when gap<0)
    //     //if ((blocked & LCA_BLOCKED) != 0) {
    //     //    return ret;
    //     //}
    //     //// --------
    //
    //     // -------- higher speed
    //     //if ((congested(neighLead.first) && neighLead.second < 20) || predInteraction(leader.first)) { //!!!
    //     //    return ret;
    //     //}
    //
    //     // iterate over all possible combinations of sublanes this vehicle might cover and check the potential speed
    //     const MSEdge& edge = (isOpposite() ? myVehicle.getLane()->getParallelOpposite() : myVehicle.getLane())->getEdge();
    //     const std::vector<double>& sublaneSides = edge.getSubLaneSides();
    //     assert(sublaneSides.size() == myExpectedSublaneSpeeds.size());
    //     const double vehWidth = getWidth();
    //     const double rightVehSide = getVehicleCenter() - 0.5 * vehWidth;
    //     const double leftVehSide = rightVehSide + vehWidth;
    //     // figure out next speed when staying where we are
    //     double defaultNextSpeed = std::numeric_limits<double>::max();
    //     /// determine the leftmost and rightmost sublanes currently occupied
    //     int leftmostOnEdge = (int)sublaneSides.size() - 1;
    //     while (leftmostOnEdge > 0 && sublaneSides[leftmostOnEdge] > leftVehSide) {
    //         leftmostOnEdge--;
    //     }
    //     int rightmostOnEdge = leftmostOnEdge;
    //     while (rightmostOnEdge > 0 && sublaneSides[rightmostOnEdge] > rightVehSide + NUMERICAL_EPS) {
    //         defaultNextSpeed = MIN2(defaultNextSpeed, myExpectedSublaneSpeeds[rightmostOnEdge]);
    // #ifdef DEBUG_WANTSCHANGE
    //         if (gDebugFlag2) {
    //             std::cout << "   adapted to current sublane=" << rightmostOnEdge << " defaultNextSpeed=" << defaultNextSpeed << "\n";
    //             std::cout << "   sublaneSides[rightmostOnEdge]=" << sublaneSides[rightmostOnEdge] << " rightVehSide=" << rightVehSide << "\n";
    //         }
    // #endif
    //         rightmostOnEdge--;
    //     }
    //     defaultNextSpeed = MIN2(defaultNextSpeed, myExpectedSublaneSpeeds[rightmostOnEdge]);
    // #ifdef DEBUG_WANTSCHANGE
    //     if (gDebugFlag2) {
    //         std::cout << "   adapted to current sublane=" << rightmostOnEdge << " defaultNextSpeed=" << defaultNextSpeed << "\n";
    //         std::cout << "   sublaneSides[rightmostOnEdge]=" << sublaneSides[rightmostOnEdge] << " rightVehSide=" << rightVehSide << "\n";
    //     }
    // #endif
    //     double maxGain = -std::numeric_limits<double>::max();
    //     double maxGainRight = -std::numeric_limits<double>::max();
    //     double maxGainLeft = -std::numeric_limits<double>::max();
    //     double latDistNice = std::numeric_limits<double>::max();
    //
    //     const int iMin = MIN2(myVehicle.getLane()->getRightmostSublane(), neighLane.getRightmostSublane());
    //     double leftMax = MAX2(
    //                          myVehicle.getLane()->getRightSideOnEdge() + myVehicle.getLane()->getWidth(),
    //                          neighLane.getRightSideOnEdge() + neighLane.getWidth());
    //     double rightMin = MIN2(myVehicle.getLane()->getRightSideOnEdge(), neighLane.getRightSideOnEdge());
    //     if (checkOpposite || isOpposite()) {
    //         leftMax = getLeftBorder();
    //     } else {
    //         assert(leftMax <= edge.getWidth());
    //     }
    //     int sublaneCompact = MAX2(iMin, rightmostOnEdge - 1); // try to compactify to the right by default
    //
    //     const double laneBoundary = laneOffset < 0 ? myVehicle.getLane()->getRightSideOnEdge() : neighLane.getRightSideOnEdge();
    //     // if there is a neighboring lane we could change to, check sublanes on all lanes of the edge
    //     // but restrict maneuver to the currently visible lanes (current, neigh) to ensure safety
    //     // This way we can discover a fast lane beyond the immediate neighbor lane
    //     const double maxLatDist = leftMax - leftVehSide;
    //     const double minLatDist = rightMin - rightVehSide;
    //     const int iStart = laneOffset == 0 ? iMin : 0;
    //     const double rightEnd = laneOffset == 0 ? leftMax : (checkOpposite ? getLeftBorder() : edge.getWidth());
    // #ifdef DEBUG_WANTSCHANGE
    //     if (gDebugFlag2) std::cout
    //                 << "  checking sublanes rightmostOnEdge=" << rightmostOnEdge
    //                 << " rightEnd=" << rightEnd
    //                 << " leftmostOnEdge=" << leftmostOnEdge
    //                 << " iStart=" << iStart
    //                 << " iMin=" << iMin
    //                 << " sublaneSides=" << sublaneSides.size()
    //                 << " leftMax=" << leftMax
    //                 << " minLatDist=" << minLatDist
    //                 << " maxLatDist=" << maxLatDist
    //                 << " sublaneCompact=" << sublaneCompact
    //                 << "\n";
    // #endif
    //     for (int i = iStart; i < (int)sublaneSides.size(); ++i) {
    //         if (sublaneSides[i] + vehWidth < rightEnd) {
    //             // i is the rightmost sublane and the left side of vehicles still fits on the edge,
    //             // compute min speed of all sublanes covered by the vehicle in this case
    //             double vMin = myExpectedSublaneSpeeds[i];
    //             //std::cout << "   i=" << i << "\n";
    //             int j = i;
    //             while (vMin > 0 && j < (int)sublaneSides.size() && sublaneSides[j] < sublaneSides[i] + vehWidth) {
    //                 vMin = MIN2(vMin, myExpectedSublaneSpeeds[j]);
    // #ifdef DEBUG_WANTSCHANGE
    //                 if (gDebugFlag2) {
    //                     //std::cout << "     j=" << j << " vMin=" << vMin << " sublaneSides[j]=" << sublaneSides[j] << " leftVehSide=" << leftVehSide << " rightVehSide=" << rightVehSide << "\n";
    //                 }
    // #endif
    //                 ++j;
    //             }
    //             // check whether the vehicle is between lanes
    //             if (laneOffset != 0 && overlap(sublaneSides[i], sublaneSides[i] + vehWidth, laneBoundary, laneBoundary)) {
    //                 vMin *= (1 - myLaneDiscipline);
    //             }
    //             double relativeGain = (vMin - defaultNextSpeed) / MAX2(vMin, RELGAIN_NORMALIZATION_MIN_SPEED);
    //             double currentLatDist = sublaneSides[i] - rightVehSide;
    //             if ((laneOffset == 0 && (currentLatDist > maxLatDist || currentLatDist < minLatDist))
    //                     || (laneOffset < 0 && currentLatDist > maxLatDist)
    //                     || (laneOffset > 0 && currentLatDist < minLatDist)) {
    // #ifdef DEBUG_WANTSCHANGE
    //                 if (gDebugFlag2) {
    //                     std::cout << "      i=" << i << " currentLatDist=" << currentLatDist << " outOfBounds\n";
    //                 }
    // #endif
    //                 continue;
    //             }
    //             currentLatDist = MIN2(MAX2(currentLatDist, minLatDist), maxLatDist);
    //             if (currentLatDist > 0 && myVehicle.getLane()->getBidiLane() != nullptr) {
    //                 // penalize overtaking on the left if the lane is used in both
    //                 // directions
    //                 relativeGain *= 0.5;
    //             }
    //             // @note only consider change if it is compatible with the current direction (same sign or laneOffset == 0)
    //             if (relativeGain > maxGain && currentLatDist * laneOffset >= 0) {
    //                 maxGain = relativeGain;
    //                 if (maxGain > GAIN_PERCEPTION_THRESHOLD) {
    //                     sublaneCompact = i;
    //                     latDist = currentLatDist;
    // #ifdef DEBUG_WANTSCHANGE
    //                     if (gDebugFlag2) {
    //                         std::cout << "      i=" << i << " vMin=" << vMin << " newLatDist=" << latDist << " relGain=" << relativeGain << "\n";
    //                     }
    // #endif
    //                 }
    //             } else {
    //                 // if anticipated gains to the left are higher then to the right and current gains are equal, prefer left
    //                 if (currentLatDist > 0
    //                         //&& latDist < 0 // #7184 compensates for #7185
    //                         && mySpeedGainProbabilityLeft > mySpeedGainProbabilityRight
    //                         && relativeGain > GAIN_PERCEPTION_THRESHOLD
    //                         && maxGain - relativeGain < NUMERICAL_EPS) {
    //                     latDist = currentLatDist;
    //                 }
    //             }
    // #ifdef DEBUG_WANTSCHANGE
    //             if (gDebugFlag2) {
    //                 std::cout << "    i=" << i << " rightmostOnEdge=" << rightmostOnEdge << " vMin=" << vMin << " relGain=" << relativeGain << " sublaneCompact=" << sublaneCompact << " curLatDist=" << currentLatDist << "\n";
    //             }
    // #endif
    //             if (currentLatDist < -NUMERICAL_EPS * myVehicle.getActionStepLengthSecs()) {
    //                 maxGainRight = MAX2(maxGainRight, relativeGain);
    //             } else if (currentLatDist > NUMERICAL_EPS * myVehicle.getActionStepLengthSecs()) {
    //                 maxGainLeft = MAX2(maxGainLeft, relativeGain);
    //             }
    //             const double subAlignDist = sublaneSides[i] - rightVehSide;
    //             if (fabs(subAlignDist) < fabs(latDistNice)) {
    //                 latDistNice = subAlignDist;
    // #ifdef DEBUG_WANTSCHANGE
    //                 if (gDebugFlag2) std::cout
    //                             << "    nicest sublane=" << i
    //                             << " side=" << sublaneSides[i]
    //                             << " rightSide=" << rightVehSide
    //                             << " latDistNice=" << latDistNice
    //                             << " maxGainR=" << (maxGainRight == -std::numeric_limits<double>::max() ? "n/a" : toString(maxGainRight))
    //                             << " maxGainL=" << (maxGainLeft == -std::numeric_limits<double>::max() ? "n/a" : toString(maxGainLeft))
    //                             << "\n";
    // #endif
    //             }
    //         }
    //     }
    //     // updated change probabilities
    //     if (maxGainRight != -std::numeric_limits<double>::max()) {
    // #ifdef DEBUG_WANTSCHANGE
    //         if (gDebugFlag2) {
    //             std::cout << "  speedGainR_old=" << mySpeedGainProbabilityRight;
    //         }
    // #endif
    //         mySpeedGainProbabilityRight += myVehicle.getActionStepLengthSecs() * maxGainRight;
    // #ifdef DEBUG_WANTSCHANGE
    //         if (gDebugFlag2) {
    //             std::cout << "  speedGainR_new=" << mySpeedGainProbabilityRight << "\n";
    //         }
    // #endif
    //     }
    //     if (maxGainLeft != -std::numeric_limits<double>::max()) {
    // #ifdef DEBUG_WANTSCHANGE
    //         if (gDebugFlag2) {
    //             std::cout << "  speedGainL_old=" << mySpeedGainProbabilityLeft;
    //         }
    // #endif
    //         mySpeedGainProbabilityLeft += myVehicle.getActionStepLengthSecs() * maxGainLeft;
    // #ifdef DEBUG_WANTSCHANGE
    //         if (gDebugFlag2) {
    //             std::cout << "  speedGainL_new=" << mySpeedGainProbabilityLeft << "\n";
    //         }
    // #endif
    //     }
    //     // decay if there is no reason for or against changing (only if we have enough information)
    //     if ((fabs(maxGainRight) < NUMERICAL_EPS || maxGainRight == -std::numeric_limits<double>::max())
    //             && (right || (alternatives & LCA_RIGHT) == 0)) {
    //         mySpeedGainProbabilityRight *= pow(SPEEDGAIN_DECAY_FACTOR, myVehicle.getActionStepLengthSecs());
    //     }
    //     if ((fabs(maxGainLeft) < NUMERICAL_EPS || maxGainLeft == -std::numeric_limits<double>::max())
    //             && (left || (alternatives & LCA_LEFT) == 0)) {
    //         mySpeedGainProbabilityLeft *= pow(SPEEDGAIN_DECAY_FACTOR, myVehicle.getActionStepLengthSecs());
    //     }
    //
    //
    // #ifdef DEBUG_WANTSCHANGE
    //     if (gDebugFlag2) std::cout << SIMTIME
    //                                    << " veh=" << myVehicle.getID()
    //                                    << " defaultNextSpeed=" << defaultNextSpeed
    //                                    << " maxGain=" << maxGain
    //                                    << " maxGainRight=" << maxGainRight
    //                                    << " maxGainLeft=" << maxGainLeft
    //                                    << " probRight=" << mySpeedGainProbabilityRight
    //                                    << " probLeft=" << mySpeedGainProbabilityLeft
    //                                    << " latDist=" << latDist
    //                                    << " latDistNice=" << latDistNice
    //                                    << " sublaneCompact=" << sublaneCompact
    //                                    << "\n";
    // #endif
    //
    //     if (!left) {
    //         // ONLY FOR CHANGING TO THE RIGHT
    //         // start keepRight maneuver when no speed loss is expected and continue
    //         // started maneuvers if the loss isn't too big
    //         if (right && myVehicle.getSpeed() > 0 && (maxGainRight >= 0
    //                 || ((myPreviousState & LCA_KEEPRIGHT) != 0 && maxGainRight >= -myKeepRightParam))) {
    //             // honor the obligation to keep right (Rechtsfahrgebot)
    //             const double vMax = myVehicle.getLane()->getVehicleMaxSpeed(&myVehicle);
    //             const double roadSpeedFactor = vMax / myVehicle.getLane()->getSpeedLimit(); // differse from speedFactor if vMax < speedLimit
    //             double acceptanceTime;
    //             if (myKeepRightAcceptanceTime == -1) {
    //                 // legacy behavior: scale acceptance time with current speed and
    //                 // use old hard-coded constant
    //                 acceptanceTime = 7 * roadSpeedFactor * MAX2(1.0, myVehicle.getSpeed());
    //             } else {
    //                 acceptanceTime = myKeepRightAcceptanceTime * roadSpeedFactor;
    //                 if (followers.hasVehicles()) {
    //                     // reduce acceptanceTime if a follower vehicle is faster or wants to drive faster
    //                     double minFactor = 1.0;
    //                     for (int i = 0; i < followers.numSublanes(); ++i) {
    //                         CLeaderDist follower = followers[i];
    //                         if (follower.first != nullptr && follower.second < 2 * follower.first->getCarFollowModel().brakeGap(follower.first->getSpeed())) {
    //                             if (follower.first->getSpeed() >= myVehicle.getSpeed()) {
    //                                 double factor = MAX2(1.0, myVehicle.getSpeed()) / MAX2(1.0, follower.first->getSpeed());
    //                                 const double fRSF = follower.first->getLane()->getVehicleMaxSpeed(follower.first) / follower.first->getLane()->getSpeedLimit();
    //                                 if (fRSF > roadSpeedFactor) {
    //                                     factor /= fRSF;
    //                                 }
    //                                 if (factor < minFactor) {
    //                                     minFactor = factor;
    //                                 }
    //                             }
    //                         }
    //                     }
    //                     acceptanceTime *= minFactor;
    //                 }
    //             }
    //             double fullSpeedGap = MAX2(0., neighDist - myVehicle.getCarFollowModel().brakeGap(vMax));
    //             double fullSpeedDrivingSeconds = MIN2(acceptanceTime, fullSpeedGap / vMax);
    //             CLeaderDist neighLead = getSlowest(neighLeaders);
    //             if (neighLead.first != 0 && neighLead.first->getSpeed() < vMax) {
    //                 fullSpeedGap = MAX2(0., MIN2(fullSpeedGap,
    //                                              neighLead.second - myVehicle.getCarFollowModel().getSecureGap(&myVehicle, neighLead.first,
    //                                                      vMax, neighLead.first->getSpeed(), neighLead.first->getCarFollowModel().getMaxDecel())));
    //                 fullSpeedDrivingSeconds = MIN2(fullSpeedDrivingSeconds, fullSpeedGap / (vMax - neighLead.first->getSpeed()));
    //             }
    //             const double deltaProb = (myChangeProbThresholdRight * (fullSpeedDrivingSeconds / acceptanceTime) / KEEP_RIGHT_TIME) * myVehicle.getActionStepLengthSecs();
    //             const bool isSlide = preventSliding(latLaneDist);
    //             // stay below threshold
    //             if (!isSlide || !wantsKeepRight(myKeepRightProbability + deltaProb)) {
    //                 myKeepRightProbability += deltaProb;
    //             }
    //
    // #ifdef DEBUG_WANTSCHANGE
    //             if (gDebugFlag2) {
    //                 std::cout << STEPS2TIME(currentTime)
    //                           << " considering keepRight:"
    //                           << " vMax=" << vMax
    //                           << " neighDist=" << neighDist
    //                           << " brakeGap=" << myVehicle.getCarFollowModel().brakeGap(myVehicle.getSpeed())
    //                           << " leaderSpeed=" << (neighLead.first == 0 ? -1 : neighLead.first->getSpeed())
    //                           << " secGap=" << (neighLead.first == 0 ? -1 : myVehicle.getCarFollowModel().getSecureGap(&myVehicle, neighLead.first,
    //                                             myVehicle.getSpeed(), neighLead.first->getSpeed(), neighLead.first->getCarFollowModel().getMaxDecel()))
    //                           << " acceptanceTime=" << acceptanceTime
    //                           << " fullSpeedGap=" << fullSpeedGap
    //                           << " fullSpeedDrivingSeconds=" << fullSpeedDrivingSeconds
    //                           << " dProb=" << deltaProb
    //                           << " isSlide=" << isSlide
    //                           << " keepRight=" << myKeepRightProbability
    //                           << " speedGainL=" << mySpeedGainProbabilityLeft
    //                           << "\n";
    //             }
    // #endif
    //             if (wantsKeepRight(myKeepRightProbability)
    //                     /*&& latLaneDist <= -NUMERICAL_EPS * myVehicle.getActionStepLengthSecs()*/) {
    //                 ret |= LCA_KEEPRIGHT;
    //                 assert(myVehicle.getLane()->getIndex() > neighLane.getIndex() || isOpposite());
    //                 if (!cancelRequest(ret | LCA_RIGHT, laneOffset)) {
    //                     latDist = latLaneDist;
    //                     maneuverDist = latLaneDist;
    //                     blocked = checkBlocking(neighLane, latDist, maneuverDist, laneOffset,
    //                                             leaders, followers, blockers,
    //                                             neighLeaders, neighFollowers, neighBlockers);
    //                     return ret;
    //                 } else {
    //                     ret &= ~LCA_KEEPRIGHT;
    //                 }
    //             }
    //         }
    //
    //         const double bidiRightFactor = myVehicle.getLane()->getBidiLane() == nullptr && !isOpposite() ? 1 : 0.05;
    // #ifdef DEBUG_WANTSCHANGE
    //         if (gDebugFlag2) {
    //             std::cout << STEPS2TIME(currentTime)
    //                       << " speedGainR=" << mySpeedGainProbabilityRight
    //                       << " speedGainL=" << mySpeedGainProbabilityLeft
    //                       << " neighDist=" << neighDist
    //                       << " neighTime=" << neighDist / MAX2(.1, myVehicle.getSpeed())
    //                       << " rThresh=" << myChangeProbThresholdRight
    //                       << " rThresh2=" << myChangeProbThresholdRight* bidiRightFactor
    //                       << " latDist=" << latDist
    //                       << "\n";
    //         }
    // #endif
    //
    //         // make changing on the right more attractive on bidi edges
    //         if (latDist < 0 && mySpeedGainProbabilityRight >= myChangeProbThresholdRight * bidiRightFactor
    //                 && neighDist / MAX2(.1, myVehicle.getSpeed()) > mySpeedGainRemainTime) {
    //             ret |= LCA_SPEEDGAIN;
    //             if (!cancelRequest(ret | getLCA(ret, latDist), laneOffset)) {
    //                 int blockedFully = 0;
    //                 maneuverDist = latDist;
    //                 blocked = checkBlocking(neighLane, latDist, maneuverDist, laneOffset,
    //                                         leaders, followers, blockers,
    //                                         neighLeaders, neighFollowers, neighBlockers,
    //                                         nullptr, nullptr, false, 0, &blockedFully);
    //                 //commitManoeuvre(blocked, blockedFully, leaders, neighLeaders, neighLane);
    //                 return ret;
    //             } else {
    //                 // @note: restore ret so subsequent calls to cancelRequest work correctly
    //                 latDist = 0;
    //                 ret &= ~LCA_SPEEDGAIN;
    //             }
    //         }
    //     }
    //     if (!right || isOpposite()) {
    //
    //         const bool stayInLane = myVehicle.getLateralPositionOnLane() + latDist < 0.5 * myVehicle.getLane()->getWidth();
    // #ifdef DEBUG_WANTSCHANGE
    //         if (gDebugFlag2) {
    //             std::cout << STEPS2TIME(currentTime)
    //                       << " speedGainL=" << mySpeedGainProbabilityLeft
    //                       << " speedGainR=" << mySpeedGainProbabilityRight
    //                       << " latDist=" << latDist
    //                       << " neighDist=" << neighDist
    //                       << " neighTime=" << neighDist / MAX2(.1, myVehicle.getSpeed())
    //                       << " lThresh=" << myChangeProbThresholdLeft
    //                       << " stayInLane=" << stayInLane
    //                       << "\n";
    //         }
    // #endif
    //
    //         if (latDist > 0 && mySpeedGainProbabilityLeft > myChangeProbThresholdLeft &&
    //                 // if we leave our lane, we should be able to stay in the new
    //                 // lane for some time
    //                 (stayInLane || neighDist / MAX2(.1, myVehicle.getSpeed()) > mySpeedGainRemainTime)) {
    //             ret |= LCA_SPEEDGAIN;
    //             if (!cancelRequest(ret + getLCA(ret, latDist), laneOffset)) {
    //                 int blockedFully = 0;
    //                 maneuverDist = latDist;
    //                 blocked = checkBlocking(neighLane, latDist, maneuverDist, laneOffset,
    //                                         leaders, followers, blockers,
    //                                         neighLeaders, neighFollowers, neighBlockers,
    //                                         nullptr, nullptr, false, 0, &blockedFully);
    //                 //commitManoeuvre(blocked, blockedFully, leaders, neighLeaders, neighLane);
    //                 return ret;
    //             } else {
    //                 latDist = 0;
    //                 ret &= ~LCA_SPEEDGAIN;
    //             }
    //         }
    //     }
    //
    //     double latDistSublane = 0.;
    //     const double halfLaneWidth = myVehicle.getLane()->getWidth() * 0.5;
    //     const double halfVehWidth = getWidth() * 0.5;
    //     if (myVehicle.getParameter().arrivalPosLatProcedure != ArrivalPosLatDefinition::DEFAULT
    //             && myVehicle.getRoute().getLastEdge() == &myVehicle.getLane()->getEdge()
    //             && bestLaneOffset == 0
    //             && (myVehicle.getArrivalPos() - getForwardPos()) < ARRIVALPOS_LAT_THRESHOLD) {
    //         // vehicle is on its final edge, on the correct lane and close to
    //         // its arrival position. Change to the desired lateral position
    //         switch (myVehicle.getParameter().arrivalPosLatProcedure) {
    //             case ArrivalPosLatDefinition::GIVEN:
    //                 latDistSublane = myVehicle.getParameter().arrivalPosLat - myVehicle.getLateralPositionOnLane();
    //                 break;
    //             case ArrivalPosLatDefinition::RIGHT:
    //                 latDistSublane = -halfLaneWidth + halfVehWidth - myVehicle.getLateralPositionOnLane();
    //                 break;
    //             case ArrivalPosLatDefinition::CENTER:
    //                 latDistSublane = -myVehicle.getLateralPositionOnLane();
    //                 break;
    //             case ArrivalPosLatDefinition::LEFT:
    //                 latDistSublane = halfLaneWidth - halfVehWidth - myVehicle.getLateralPositionOnLane();
    //                 break;
    //             default:
    //                 assert(false);
    //         }
    // #ifdef DEBUG_WANTSCHANGE
    //         if (gDebugFlag2) std::cout << SIMTIME
    //                                        << " arrivalPosLatProcedure=" << (int)myVehicle.getParameter().arrivalPosLatProcedure
    //                                        << " arrivalPosLat=" << myVehicle.getParameter().arrivalPosLat << "\n";
    // #endif
    //
    //     } else {
    //
    //         LatAlignmentDefinition align = getDesiredAlignment();
    //         switch (align) {
    //             case LatAlignmentDefinition::RIGHT:
    //                 latDistSublane = -halfLaneWidth + halfVehWidth - getPosLat();
    //                 break;
    //             case LatAlignmentDefinition::LEFT:
    //                 latDistSublane = halfLaneWidth - halfVehWidth - getPosLat();
    //                 break;
    //             case LatAlignmentDefinition::CENTER:
    //             case LatAlignmentDefinition::DEFAULT:
    //                 latDistSublane = -getPosLat();
    //                 break;
    //             case LatAlignmentDefinition::NICE:
    //                 latDistSublane = latDistNice;
    //                 break;
    //             case LatAlignmentDefinition::COMPACT:
    //                 latDistSublane = sublaneSides[sublaneCompact] - rightVehSide;
    //                 break;
    //             case LatAlignmentDefinition::ARBITRARY: {
    //                 latDistSublane = myVehicle.getLateralPositionOnLane() - getPosLat();
    //                 const double hLW = myVehicle.getLane()->getWidth() * 0.5;
    //                 const double posLat = myVehicle.getLateralPositionOnLane();
    //                 if (fabs(posLat) > hLW) {
    //                     // vehicle is not within it's current lane
    //                     if (posLat > 0) {
    //                         latDistSublane -= (posLat - hLW);
    //                     } else {
    //                         latDistSublane += (-posLat - hLW);
    //                     }
    //                 } else {
    //                     const double edgeWidth = myVehicle.getCurrentEdge()->getWidth();
    //                     if (getWidth() < edgeWidth) {
    //                         if (rightVehSide < 0) {
    //                             latDistSublane -= rightVehSide;
    //                         } else if (leftVehSide > edgeWidth) {
    //                             latDistSublane -= leftVehSide - edgeWidth;
    //                         }
    //                     }
    //                 }
    //                 break;
    //             }
    //             case LatAlignmentDefinition::GIVEN: {
    //                 // sublane alignment should not cause the vehicle to leave the lane
    //                 const double hw = myVehicle.getLane()->getWidth() / 2 - NUMERICAL_EPS;
    //                 const double offset = MAX2(-hw, MIN2(hw, myVehicle.getVehicleType().getPreferredLateralAlignmentOffset()));
    //                 latDistSublane = -getPosLat() + offset;
    //             }
    //             break;
    //             default:
    //                 break;
    //         }
    //     }
    //     // only factor in preferred lateral alignment if there is no speedGain motivation or it runs in the same direction
    //     if (fabs(latDist) <= NUMERICAL_EPS * myVehicle.getActionStepLengthSecs() ||
    //             latDistSublane * latDist > 0) {
    //
    // #if defined(DEBUG_WANTSCHANGE) || defined(DEBUG_STATE) || defined(DEBUG_MANEUVER)
    //         if (gDebugFlag2) std::cout << SIMTIME
    //                                        << " alignment=" << toString(myVehicle.getVehicleType().getPreferredLateralAlignment())
    //                                        << " mySpeedGainR=" << mySpeedGainProbabilityRight
    //                                        << " mySpeedGainL=" << mySpeedGainProbabilityLeft
    //                                        << " latDist=" << latDist
    //                                        << " latDistSublane=" << latDistSublane
    //                                        << " relGainSublane=" << computeSpeedGain(latDistSublane, defaultNextSpeed)
    //                                        << " maneuverDist=" << maneuverDist
    //                                        << " myCanChangeFully=" << myCanChangeFully
    //                                        << " myTurnAlignmentDist=" << myTurnAlignmentDist
    //                                        << " nextTurn=" << myVehicle.getNextTurn().first << ":" << toString(myVehicle.getNextTurn().second)
    //                                        << " prevState=" << toString((LaneChangeAction)myPreviousState)
    //                                        << "\n";
    // #endif
    //
    //         if ((latDistSublane < 0 && mySpeedGainProbabilityRight < mySpeedLossProbThreshold)
    //                 || (latDistSublane > 0 && mySpeedGainProbabilityLeft < mySpeedLossProbThreshold)
    //                 || computeSpeedGain(latDistSublane, defaultNextSpeed) < -mySublaneParam) {
    //             // do not risk losing speed
    // #if defined(DEBUG_WANTSCHANGE)
    //             if (gDebugFlag2) std::cout << "   aborting sublane change to avoid speed loss (mySpeedLossProbThreshold=" << mySpeedLossProbThreshold
    //                                            << " speedGain=" << computeSpeedGain(latDistSublane, defaultNextSpeed) << ")\n";
    // #endif
    //             latDistSublane = 0;
    //         }
    //         // Ignore preferred lateral alignment if we are in the middle of an unfinished non-alignment maneuver into the opposite direction
    //         if (!myCanChangeFully
    //                 && (myPreviousState & (LCA_STRATEGIC | LCA_COOPERATIVE | LCA_KEEPRIGHT | LCA_SPEEDGAIN)) != 0
    //                 && ((getManeuverDist() < 0 && latDistSublane > 0) || (getManeuverDist() > 0 && latDistSublane < 0))) {
    // #if defined(DEBUG_WANTSCHANGE)
    //             if (gDebugFlag2) {
    //                 std::cout << "   aborting sublane change due to prior maneuver\n";
    //             }
    // #endif
    //             latDistSublane = 0;
    //         }
    //         latDist = latDistSublane * (isOpposite() ? -1 : 1);
    //         // XXX first compute preferred adaptation and then override with speed
    //         // (this way adaptation is still done if changing for speedgain is
    //         // blocked)
    //         if (fabs(latDist) >= NUMERICAL_EPS * myVehicle.getActionStepLengthSecs()) {
    // #ifdef DEBUG_WANTSCHANGE
    //             if (gDebugFlag2) std::cout << SIMTIME
    //                                            << " adapting to preferred alignment=" << toString(myVehicle.getVehicleType().getPreferredLateralAlignment())
    //                                            << " latDist=" << latDist
    //                                            << "\n";
    // #endif
    //             ret |= LCA_SUBLANE;
    //             // include prior motivation when sublane-change is part of finishing an ongoing maneuver in the same direction
    //             if (getPreviousManeuverDist() * latDist > 0) {
    //                 int priorReason = (myPreviousState & LCA_CHANGE_REASONS & ~LCA_SUBLANE);
    //                 ret |= priorReason;
    // #ifdef DEBUG_WANTSCHANGE
    //                 if (gDebugFlag2 && priorReason != 0) std::cout << "   including prior reason " << toString((LaneChangeAction)priorReason)
    //                             << " prevManeuverDist=" << getPreviousManeuverDist() << "\n";
    // #endif
    //             }
    //             if (!cancelRequest(ret + getLCA(ret, latDist), laneOffset)) {
    //                 maneuverDist = latDist;
    //                 blocked = checkBlocking(neighLane, latDist, maneuverDist, laneOffset,
    //                                         leaders, followers, blockers,
    //                                         neighLeaders, neighFollowers, neighBlockers);
    //                 return ret;
    //             } else {
    //                 ret &= ~LCA_SUBLANE;
    //             }
    //         } else {
    //             return ret | LCA_SUBLANE | LCA_STAY;
    //         }
    //     }
    //     latDist = 0;
    //
    //
    //     // --------
    //     /*
    //     if (changeToBest && bestLaneOffset == curr.bestLaneOffset && laneOffset != 0
    //             && (right
    //                 ? mySpeedGainProbabilityRight > MAX2(0., mySpeedGainProbabilityLeft)
    //                 : mySpeedGainProbabilityLeft  > MAX2(0., mySpeedGainProbabilityRight))) {
    //         // change towards the correct lane, speedwise it does not hurt
    //         ret |= LCA_STRATEGIC;
    //         if (!cancelRequest(ret, laneOffset)) {
    //             latDist = latLaneDist;
    //             blocked = checkBlocking(neighLane, latDist, laneOffset,
    //                     leaders, followers, blockers,
    //                     neighLeaders, neighFollowers, neighBlockers);
    //             return ret;
    //         }
    //     }
    //     */
    // #ifdef DEBUG_WANTSCHANGE
    //     if (gDebugFlag2) {
    //         std::cout << STEPS2TIME(currentTime)
    //                   << " veh=" << myVehicle.getID()
    //                   << " mySpeedGainR=" << mySpeedGainProbabilityRight
    //                   << " mySpeedGainL=" << mySpeedGainProbabilityLeft
    //                   << " myKeepRight=" << myKeepRightProbability
    //                   << "\n";
    //     }
    // #endif
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn wants_change_sublane_2(&mut self, laneOffset: i32, alternatives: LaneChangeAction, leaders: &MSLeaderDistanceInfo, followers: &MSLeaderDistanceInfo, blockers: &MSLeaderDistanceInfo, neighLeaders: &MSLeaderDistanceInfo, neighFollowers: &MSLeaderDistanceInfo, neighBlockers: &MSLeaderDistanceInfo, neighLane: &MSLane, preb: &Vec<MSVehicle_LaneQ>, lastBlocked: Ptr<Ptr<MSVehicle>>, firstBlocked: Ptr<Ptr<MSVehicle>>, latDist: &mut f64, maneuverDist: &mut f64, blocked: &mut i32) -> i32 {
        todo!("port MSLCM_SL2015::_wantsChangeSublane")
    }

    /// C++: `double MSLCM_SL2015::informLeaders(int, int, const std::vector<CLeaderDist> &, double)` [protected]
    // double
    // MSLCM_SL2015::informLeaders(int blocked, int dir,
    //                             const std::vector<CLeaderDist>& blockers,
    //                             double remainingSeconds) {
    //     double plannedSpeed = myVehicle.getSpeed();
    //     double space = myLeftSpace;
    //     if (myLeadingBlockerLength != 0) {
    //         // see patchSpeed @todo: refactor
    //         space -= myLeadingBlockerLength - POSITION_EPS - myVehicle.getVehicleType().getMinGap();
    //         if (space <= 0) {
    //             // ignore leading blocker
    //             space = myLeftSpace;
    //         }
    //     }
    //     double safe = myVehicle.getCarFollowModel().stopSpeed(&myVehicle, myVehicle.getSpeed(), space);
    //     plannedSpeed = MIN2(plannedSpeed, safe);
    //
    //     for (std::vector<CLeaderDist>::const_iterator it = blockers.begin(); it != blockers.end(); ++it) {
    //         plannedSpeed = MIN2(plannedSpeed, informLeader(blocked, dir, *it, remainingSeconds));
    //     }
    //     return plannedSpeed;
    // }
    #[allow(unused_variables)]
    pub fn inform_leaders(&mut self, blocked: i32, dir: i32, blockers: &Vec<CLeaderDist>, remainingSeconds: f64) -> f64 {
        todo!("port MSLCM_SL2015::informLeaders")
    }

    /// C++: `void MSLCM_SL2015::informFollowers(int, int, const std::vector<CLeaderDist> &, double, double)` [protected]
    /// call informFollower for multiple followers
    // void
    // MSLCM_SL2015::informFollowers(int blocked, int dir,
    //                               const std::vector<CLeaderDist>& blockers,
    //                               double remainingSeconds,
    //                               double plannedSpeed) {
    //     // #3727
    //     for (std::vector<CLeaderDist>::const_iterator it = blockers.begin(); it != blockers.end(); ++it) {
    //         informFollower(blocked, dir, *it, remainingSeconds, plannedSpeed);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn inform_followers(&mut self, blocked: i32, dir: i32, blockers: &Vec<CLeaderDist>, remainingSeconds: f64, plannedSpeed: f64) {
        todo!("port MSLCM_SL2015::informFollowers")
    }

    /// C++: `double MSLCM_SL2015::informLeader(int, int, const CLeaderDist &, double)` [protected]
    // double
    // MSLCM_SL2015::informLeader(int blocked,
    //                            int dir,
    //                            const CLeaderDist& neighLead,
    //                            double remainingSeconds) {
    //     double plannedSpeed = MIN2(myVehicle.getSpeed(),
    //                                myVehicle.getCarFollowModel().stopSpeed(&myVehicle, myVehicle.getSpeed(), myLeftSpace - myLeadingBlockerLength));
    //     for (auto i : myLCAccelerationAdvices) {
    //         double v = myVehicle.getSpeed() + ACCEL2SPEED(i.first);
    //         if (v >= myVehicle.getSpeed() - ACCEL2SPEED(myVehicle.getCarFollowModel().getMaxDecel())) {
    //             plannedSpeed = MIN2(plannedSpeed, v);
    //         }
    //     }
    // #ifdef DEBUG_INFORM
    //     if (gDebugFlag2) {
    //         std::cout << " informLeader speed=" <<  myVehicle.getSpeed() << " planned=" << plannedSpeed << "\n";
    //     }
    // #endif
    //
    //     if ((blocked & LCA_BLOCKED_BY_LEADER) != 0 && neighLead.first != 0) {
    //         const MSVehicle* nv = neighLead.first;
    //         if (MSLCHelper::divergentRoute(myVehicle, *nv)) {
    //             //std::cout << SIMTIME << " ego=" << myVehicle.getID() << " ignoresDivergentBlockingLeader=" << nv->getID() << "\n";
    //             return plannedSpeed;
    //         }
    // #ifdef DEBUG_INFORM
    //         if (gDebugFlag2) std::cout << " blocked by leader nv=" <<  nv->getID() << " nvSpeed=" << nv->getSpeed() << " needGap="
    //                                        << myVehicle.getCarFollowModel().getSecureGap(&myVehicle, nv, myVehicle.getSpeed(), nv->getSpeed(), nv->getCarFollowModel().getMaxDecel()) << "\n";
    // #endif
    //         // decide whether we want to overtake the leader or follow it
    //         const double dv = plannedSpeed - nv->getSpeed();
    //         const double overtakeDist = (neighLead.second // drive to back of follower
    //                                      + nv->getVehicleType().getLengthWithGap() // drive to front of follower
    //                                      + myVehicle.getVehicleType().getLength() // ego back reaches follower front
    //                                      + nv->getCarFollowModel().getSecureGap( // save gap to follower
    //                                          nv, &myVehicle, nv->getSpeed(), myVehicle.getSpeed(), myVehicle.getCarFollowModel().getMaxDecel()));
    //
    //         if ((dv < myOvertakeDeltaSpeedFactor * myVehicle.getLane()->getSpeedLimit() + NUMERICAL_EPS
    //                 // overtaking on the right on an uncongested highway is forbidden (noOvertakeLCLeft)
    //                 || (dir == LCA_MLEFT && !myVehicle.congested() && !myAllowOvertakingRight)
    //                 // not enough space to overtake? (we will start to brake when approaching a dead end)
    //                 || myLeftSpace - myLeadingBlockerLength - myVehicle.getCarFollowModel().brakeGap(myVehicle.getSpeed()) < overtakeDist
    //                 // not enough time to overtake?
    //                 || dv * remainingSeconds < overtakeDist)
    //                 && (!neighLead.first->isStopped() || (isOpposite() && neighLead.second >= 0))) {
    //             // cannot overtake
    //             msg(neighLead, -1, dir | LCA_AMBLOCKINGLEADER);
    //             // slow down smoothly to follow leader
    //             const double targetSpeed = getCarFollowModel().followSpeed(
    //                                            &myVehicle, myVehicle.getSpeed(), neighLead.second, nv->getSpeed(), nv->getCarFollowModel().getMaxDecel());
    //             if (targetSpeed < myVehicle.getSpeed()) {
    //                 // slow down smoothly to follow leader
    //                 const double decel = ACCEL2SPEED(MIN2(myVehicle.getCarFollowModel().getMaxDecel(),
    //                                                       MAX2(MIN_FALLBEHIND, (myVehicle.getSpeed() - targetSpeed) / remainingSeconds)));
    //                 //const double nextSpeed = MAX2(0., MIN2(plannedSpeed, myVehicle.getSpeed() - decel));
    //                 const double nextSpeed = MIN2(plannedSpeed, MAX2(0.0, myVehicle.getSpeed() - decel));
    // #ifdef DEBUG_INFORM
    //                 if (gDebugFlag2) {
    //                     std::cout << SIMTIME
    //                               << " cannot overtake leader nv=" << nv->getID()
    //                               << " dv=" << dv
    //                               << " remainingSeconds=" << remainingSeconds
    //                               << " targetSpeed=" << targetSpeed
    //                               << " nextSpeed=" << nextSpeed
    //                               << "\n";
    //                 }
    // #endif
    //                 addLCSpeedAdvice(nextSpeed, dir);
    //                 return nextSpeed;
    //             } else {
    //                 // leader is fast enough anyway
    // #ifdef DEBUG_INFORM
    //                 if (gDebugFlag2) {
    //                     std::cout << SIMTIME
    //                               << " cannot overtake fast leader nv=" << nv->getID()
    //                               << " dv=" << dv
    //                               << " remainingSeconds=" << remainingSeconds
    //                               << " targetSpeed=" << targetSpeed
    //                               << "\n";
    //                 }
    // #endif
    //                 addLCSpeedAdvice(targetSpeed, dir);
    //                 return plannedSpeed;
    //             }
    //         } else {
    // #ifdef DEBUG_INFORM
    //             if (gDebugFlag2) {
    //                 std::cout << SIMTIME
    //                           << " wants to overtake leader nv=" << nv->getID()
    //                           << " dv=" << dv
    //                           << " remainingSeconds=" << remainingSeconds
    //                           << " currentGap=" << neighLead.second
    //                           << " secureGap=" << nv->getCarFollowModel().getSecureGap(nv, &myVehicle, nv->getSpeed(), myVehicle.getSpeed(), myVehicle.getCarFollowModel().getMaxDecel())
    //                           << " overtakeDist=" << overtakeDist
    //                           << " leftSpace=" << myLeftSpace
    //                           << " blockerLength=" << myLeadingBlockerLength
    //                           << "\n";
    //             }
    // #endif
    //             // overtaking, leader should not accelerate
    //             msg(neighLead, nv->getSpeed(), dir | LCA_AMBLOCKINGLEADER);
    //             return -1;
    //         }
    //     } else if (neighLead.first != 0) { // (remainUnblocked)
    //         // we are not blocked now. make sure we stay far enough from the leader
    //         const MSVehicle* nv = neighLead.first;
    //         double dv, nextNVSpeed;
    //         if (MSGlobals::gSemiImplicitEulerUpdate) {
    //             // XXX: the decrement (HELP_OVERTAKE) should be scaled with timestep length, I think.
    //             //      It seems to function as an estimate nv's speed in the next simstep!? (so HELP_OVERTAKE should be an acceleration value.)
    //             nextNVSpeed = nv->getSpeed() - HELP_OVERTAKE; // conservative
    //             dv = SPEED2DIST(myVehicle.getSpeed() - nextNVSpeed);
    //         } else {
    //             // Estimate neigh's speed after actionstep length
    //             // @note The possible breaking can be underestimated by the formula, so this is a potential
    //             //       source of collisions if actionsteplength>simsteplength.
    //             const double nvMaxDecel = HELP_OVERTAKE;
    //             nextNVSpeed = nv->getSpeed() - nvMaxDecel * myVehicle.getActionStepLengthSecs(); // conservative
    //             // Estimated gap reduction until next action step if own speed stays constant
    //             dv = SPEED2DIST(myVehicle.getSpeed() - nextNVSpeed);
    //         }
    //         const double targetSpeed = getCarFollowModel().followSpeed(
    //                                        &myVehicle, myVehicle.getSpeed(), neighLead.second - dv, nextNVSpeed, nv->getCarFollowModel().getMaxDecel());
    //         addLCSpeedAdvice(targetSpeed, dir);
    // #ifdef DEBUG_INFORM
    //         if (gDebugFlag2) {
    //             std::cout << " not blocked by leader nv=" <<  nv->getID()
    //                       << " nvSpeed=" << nv->getSpeed()
    //                       << " gap=" << neighLead.second
    //                       << " nextGap=" << neighLead.second - dv
    //                       << " needGap=" << myVehicle.getCarFollowModel().getSecureGap(&myVehicle, nv, myVehicle.getSpeed(), nv->getSpeed(), nv->getCarFollowModel().getMaxDecel())
    //                       << " targetSpeed=" << targetSpeed
    //                       << "\n";
    //         }
    // #endif
    //         return MIN2(targetSpeed, plannedSpeed);
    //     } else {
    //         // not overtaking
    //         return plannedSpeed;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn inform_leader(&mut self, blocked: i32, dir: i32, neighLead: &CLeaderDist, remainingSeconds: f64) -> f64 {
        todo!("port MSLCM_SL2015::informLeader")
    }

    /// C++: `void MSLCM_SL2015::informFollower(int, int, const CLeaderDist &, double, double)` [protected]
    /// decide whether we will try cut in before the follower or allow to be overtaken
    // void
    // MSLCM_SL2015::informFollower(int blocked,
    //                              int dir,
    //                              const CLeaderDist& neighFollow,
    //                              double remainingSeconds,
    //                              double plannedSpeed) {
    //
    //     const MSVehicle* nv = neighFollow.first;
    //     // decide whether we will request help to cut in before the follower or allow to be overtaken
    //     if (nv != nullptr && MSLCHelper::unwillingToHelp(myVehicle, plannedSpeed, *nv)) {
    //         // @note: this check needs to come first because even if the follower is not blocking, getSpeedPreservingSecureGap may request a slow-down
    // #ifdef DEBUG_INFORMER
    //         if (DEBUG_COND) {
    //             std::cout << "\n nv=" << nv->getID() << " not willing to help\n";
    //         }
    // #endif
    //         return;
    //     }
    //
    //     if ((blocked & LCA_BLOCKED_BY_FOLLOWER) != 0 && neighFollow.first != 0) {
    //         if (MSLCHelper::divergentRoute(myVehicle, *nv)) {
    //             //std::cout << SIMTIME << " ego=" << myVehicle.getID() << " ignoresDivergentBlockingFollower=" << nv->getID() << "\n";
    //             return;
    //         }
    // #ifdef DEBUG_INFORM
    //         if (gDebugFlag2) std::cout << " blocked by follower nv=" <<  nv->getID() << " nvSpeed=" << nv->getSpeed() << " needGap="
    //                                        << nv->getCarFollowModel().getSecureGap(nv, &myVehicle, nv->getSpeed(), myVehicle.getSpeed(), myVehicle.getCarFollowModel().getMaxDecel()) << "\n";
    // #endif
    //
    //         // are we fast enough to cut in without any help?
    //         if (plannedSpeed - nv->getSpeed() >= HELP_OVERTAKE) {
    //             const double neededGap = nv->getCarFollowModel().getSecureGap(nv, &myVehicle, nv->getSpeed(), plannedSpeed, myVehicle.getCarFollowModel().getMaxDecel());
    //             if ((neededGap - neighFollow.second) / remainingSeconds < (plannedSpeed - nv->getSpeed())) {
    // #ifdef DEBUG_INFORM
    //                 if (gDebugFlag2) {
    //                     std::cout << " wants to cut in before  nv=" << nv->getID() << " without any help neededGap=" << neededGap << "\n";
    //                 }
    // #endif
    //                 // follower might even accelerate but not to much
    //                 msg(neighFollow, plannedSpeed - HELP_OVERTAKE, dir | LCA_AMBLOCKINGFOLLOWER);
    //                 return;
    //             }
    //         }
    //
    //         // PARAMETERS
    //         // assume other vehicle will assume the equivalent of 1 second of
    //         // maximum deceleration to help us (will probably be spread over
    //         // multiple seconds)
    //         // -----------
    //         const double helpDecel = nv->getCarFollowModel().getMaxDecel() * HELP_DECEL_FACTOR ;
    //
    //         // change in the gap between ego and blocker over 1 second (not STEP!)
    //         const double neighNewSpeed = MAX2(0., nv->getSpeed() - ACCEL2SPEED(helpDecel));
    //         const double neighNewSpeed1s = MAX2(0., nv->getSpeed() - helpDecel);
    //         const double dv = plannedSpeed - neighNewSpeed1s;
    //         // new gap between follower and self in case the follower does brake for 1s
    //         const double decelGap = neighFollow.second + dv;
    //         const double secureGap = nv->getCarFollowModel().getSecureGap(nv, &myVehicle, neighNewSpeed1s, plannedSpeed, myVehicle.getCarFollowModel().getMaxDecel());
    // #ifdef DEBUG_INFORM
    //         if (gDebugFlag2) {
    //             std::cout << SIMTIME
    //                       << " egoV=" << myVehicle.getSpeed()
    //                       << " egoNV=" << plannedSpeed
    //                       << " nvNewSpeed=" << neighNewSpeed
    //                       << " nvNewSpeed1s=" << neighNewSpeed1s
    //                       << " deltaGap=" << dv
    //                       << " decelGap=" << decelGap
    //                       << " secGap=" << secureGap
    //                       << "\n";
    //         }
    // #endif
    //         if (decelGap > 0 && decelGap >= secureGap) {
    //             // if the blocking neighbor brakes it could actually help
    //             // how hard does it actually need to be?
    //             // to be safe in the next step the following equation has to hold:
    //             //   vsafe <= followSpeed(gap=currentGap - SPEED2DIST(vsafe), ...)
    //             // we compute an upper bound on vsafe by doing the computation twice
    //             const double vsafe1 = MAX2(neighNewSpeed, nv->getCarFollowModel().followSpeed(
    //                                            nv, nv->getSpeed(), neighFollow.second + SPEED2DIST(plannedSpeed), plannedSpeed, myVehicle.getCarFollowModel().getMaxDecel()));
    //             const double vsafe = MAX2(neighNewSpeed, nv->getCarFollowModel().followSpeed(
    //                                           nv, nv->getSpeed(), neighFollow.second + SPEED2DIST(plannedSpeed - vsafe1), plannedSpeed, myVehicle.getCarFollowModel().getMaxDecel()));
    //             // the following assertion cannot be guaranteed because the CFModel handles small gaps differently, see MSCFModel::maximumSafeStopSpeed
    //             // assert(vsafe <= vsafe1);
    //             msg(neighFollow, vsafe, dir | LCA_AMBLOCKINGFOLLOWER);
    // #ifdef DEBUG_INFORM
    //             if (gDebugFlag2) {
    //                 std::cout << " wants to cut in before nv=" << nv->getID()
    //                           << " vsafe1=" << vsafe1
    //                           << " vsafe=" << vsafe
    //                           << " newSecGap=" << nv->getCarFollowModel().getSecureGap(nv, &myVehicle, vsafe, plannedSpeed, myVehicle.getCarFollowModel().getMaxDecel())
    //                           << "\n";
    //             }
    // #endif
    //         } else if (dv > 0 && dv * remainingSeconds > (secureGap - decelGap + POSITION_EPS)) {
    //             // decelerating once is sufficient to open up a large enough gap in time
    //             msg(neighFollow, neighNewSpeed, dir | LCA_AMBLOCKINGFOLLOWER);
    // #ifdef DEBUG_INFORM
    //             if (gDebugFlag2) {
    //                 std::cout << " wants to cut in before nv=" << nv->getID() << " (eventually)\n";
    //             }
    // #endif
    //         } else if (dir == LCA_MRIGHT && !myAllowOvertakingRight && !nv->congested()) {
    //             const double vhelp = MAX2(neighNewSpeed, HELP_OVERTAKE);
    //             msg(neighFollow, vhelp, dir | LCA_AMBLOCKINGFOLLOWER);
    // #ifdef DEBUG_INFORM
    //             if (gDebugFlag2) {
    //                 std::cout << " wants to cut in before nv=" << nv->getID() << " (nv cannot overtake right)\n";
    //             }
    // #endif
    //         } else {
    //             double vhelp = MAX2(nv->getSpeed(), myVehicle.getSpeed() + HELP_OVERTAKE);
    //             if (nv->getSpeed() > myVehicle.getSpeed() &&
    //                     ((dir == LCA_MRIGHT && myVehicle.getWaitingSeconds() > LCA_RIGHT_IMPATIENCE)
    //                      || (dir == LCA_MLEFT && plannedSpeed > CUT_IN_LEFT_SPEED_THRESHOLD) // VARIANT_22 (slowDownLeft)
    //                      // XXX this is a hack to determine whether the vehicles is on an on-ramp. This information should be retrieved from the network itself
    //                      || (dir == LCA_MLEFT && myLeftSpace > MAX_ONRAMP_LENGTH)
    //                     )) {
    //                 // let the follower slow down to increase the likelyhood that later vehicles will be slow enough to help
    //                 // follower should still be fast enough to open a gap
    //                 vhelp = MAX2(neighNewSpeed, myVehicle.getSpeed() + HELP_OVERTAKE);
    // #ifdef DEBUG_INFORM
    //                 if (gDebugFlag2) {
    //                     std::cout << " wants right follower to slow down a bit\n";
    //                 }
    // #endif
    //                 if ((nv->getSpeed() - myVehicle.getSpeed()) / helpDecel < remainingSeconds) {
    // #ifdef DEBUG_INFORM
    //                     if (gDebugFlag2) {
    //                         std::cout << " wants to cut in before right follower nv=" << nv->getID() << " (eventually)\n";
    //                     }
    // #endif
    //                     msg(neighFollow, vhelp, dir | LCA_AMBLOCKINGFOLLOWER);
    //                     return;
    //                 }
    //             }
    //             msg(neighFollow, vhelp, dir | LCA_AMBLOCKINGFOLLOWER);
    //             // this follower is supposed to overtake us. slow down smoothly to allow this
    //             const double overtakeDist = (neighFollow.second // follower reaches ego back
    //                                          + myVehicle.getVehicleType().getLengthWithGap() // follower reaches ego front
    //                                          + nv->getVehicleType().getLength() // follower back at ego front
    //                                          + myVehicle.getCarFollowModel().getSecureGap( // follower has safe dist to ego
    //                                              &myVehicle, nv, plannedSpeed, vhelp, nv->getCarFollowModel().getMaxDecel()));
    //             // speed difference to create a sufficiently large gap
    //             const double needDV = overtakeDist / remainingSeconds;
    //             // make sure the deceleration is not to strong
    //             addLCSpeedAdvice(MAX2(vhelp - needDV, myVehicle.getSpeed() - ACCEL2SPEED(myVehicle.getCarFollowModel().getMaxDecel())), dir);
    //
    // #ifdef DEBUG_INFORM
    //             if (gDebugFlag2) {
    //                 std::cout << SIMTIME
    //                           << " veh=" << myVehicle.getID()
    //                           << " wants to be overtaken by=" << nv->getID()
    //                           << " overtakeDist=" << overtakeDist
    //                           << " vneigh=" << nv->getSpeed()
    //                           << " vhelp=" << vhelp
    //                           << " needDV=" << needDV
    //                           << " vsafe=" << myVehicle.getSpeed() + ACCEL2SPEED(myLCAccelerationAdvices.back().first)
    //                           << "\n";
    //             }
    // #endif
    //         }
    //     } else if (neighFollow.first != 0) {
    //         const double vsafe = MSLCHelper::getSpeedPreservingSecureGap(myVehicle, *neighFollow.first, neighFollow.second, plannedSpeed);
    //         msg(neighFollow, vsafe, dir | LCA_AMBLOCKINGFOLLOWER);
    // #ifdef DEBUG_INFORM
    //         if (gDebugFlag2) {
    //             std::cout << " wants to cut in before non-blocking follower nv=" << neighFollow.first->getID() << "\n";
    //         }
    // #endif
    //     }
    // }
    #[allow(unused_variables)]
    pub fn inform_follower(&mut self, blocked: i32, dir: i32, neighFollow: &CLeaderDist, remainingSeconds: f64, plannedSpeed: f64) {
        todo!("port MSLCM_SL2015::informFollower")
    }

    /// C++: `int MSLCM_SL2015::slowDownForBlocked(MSVehicle **, int)` [protected]
    /// compute useful slowdowns for blocked vehicles
    // int
    // MSLCM_SL2015::slowDownForBlocked(MSVehicle** blocked, int state) {
    //     //  if this vehicle is blocking someone in front, we maybe decelerate to let him in
    //     if ((*blocked) != nullptr) {
    //         double gap = (*blocked)->getPositionOnLane() - (*blocked)->getVehicleType().getLength() - myVehicle.getPositionOnLane() - myVehicle.getVehicleType().getMinGap();
    // #ifdef DEBUG_SLOWDOWN
    //         if (gDebugFlag2) {
    //             std::cout << SIMTIME
    //                       << " veh=" << myVehicle.getID()
    //                       << " blocked=" << Named::getIDSecure(*blocked)
    //                       << " gap=" << gap
    //                       << "\n";
    //         }
    // #endif
    //         if (gap > POSITION_EPS) {
    //             //const bool blockedWantsUrgentRight = (((*blocked)->getLaneChangeModel().getOwnState() & LCA_RIGHT != 0)
    //             //    && ((*blocked)->getLaneChangeModel().getOwnState() & LCA_URGENT != 0));
    //
    //             if (myVehicle.getSpeed() < myVehicle.getCarFollowModel().getMaxDecel()
    //                     //|| blockedWantsUrgentRight  // VARIANT_10 (helpblockedRight)
    //                ) {
    //                 if ((*blocked)->getSpeed() < SUMO_const_haltingSpeed) {
    //                     state |= LCA_AMBACKBLOCKER_STANDING;
    //                 } else {
    //                     state |= LCA_AMBACKBLOCKER;
    //                 }
    //                 const double targetSpeed = getCarFollowModel().followSpeed(
    //                                                &myVehicle, myVehicle.getSpeed(), (gap - POSITION_EPS),
    //                                                (*blocked)->getSpeed(), (*blocked)->getCarFollowModel().getMaxDecel());
    // #ifdef DEBUG_INFORM
    //                 if (gDebugFlag2) {
    //                     std::cout << "   slowing down for blocked " << Named::getIDSecure(*blocked) << " targetSpeed=" << targetSpeed << "\n";
    //                 }
    // #endif
    //                 addLCSpeedAdvice(targetSpeed, LCA_CHANGE_TO_HELP);
    //                 //(*blocked) = 0; // VARIANT_14 (furtherBlock)
    //             }
    //         }
    //     }
    //     return state;
    // }
    #[allow(unused_variables)]
    pub fn slow_down_for_blocked(&mut self, blocked: Ptr<Ptr<MSVehicle>>, state: i32) -> i32 {
        todo!("port MSLCM_SL2015::slowDownForBlocked")
    }

    /// C++: `bool MSLCM_SL2015::saveBlockerLength(double, double)` [virtual, protected]
    /// reserve space at the end of the lane to avoid dead locks
    // bool
    // MSLCM_SL2015::saveBlockerLength(double length, double foeLeftSpace) {
    //     const bool canReserve = MSLCHelper::canSaveBlockerLength(myVehicle, length, myLeftSpace);
    //     if (!isOpposite() && (canReserve || myLeftSpace > foeLeftSpace)) {
    //         myLeadingBlockerLength = MAX2(length, myLeadingBlockerLength);
    //         if (myLeftSpace == 0 && foeLeftSpace < 0) {
    //             // called from opposite overtaking, myLeftSpace must be initialized
    //             myLeftSpace = myVehicle.getBestLanes()[myVehicle.getLane()->getIndex()].length - myVehicle.getPositionOnLane();
    //         }
    //         return true;
    //     } else {
    //         return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn save_blocker_length(&mut self, length: f64, foeLeftSpace: f64) -> bool {
        todo!("port MSLCM_SL2015::saveBlockerLength")
    }

    /// C++: `bool MSLCM_SL2015::outsideEdge() const` [const, protected]
    /// whether the ego vehicle is driving outside edgebounds
    // bool
    // MSLCM_SL2015::outsideEdge() const {
    //     return myVehicle.getLeftSideOnEdge() < 0 || myVehicle.getRightSideOnEdge() > myVehicle.getLane()->getEdge().getWidth();
    // }
    #[allow(unused_variables)]
    pub fn outside_edge(&self) -> bool {
        todo!("port MSLCM_SL2015::outsideEdge")
    }

    /// C++: `bool MSLCM_SL2015::amBlockingLeader()` [protected]
    // inline bool amBlockingLeader() {
    //         return (myOwnState & LCA_AMBLOCKINGLEADER) != 0;
    //     }
    #[allow(unused_variables)]
    pub fn am_blocking_leader(&mut self) -> bool {
        todo!("port MSLCM_SL2015::amBlockingLeader")
    }

    /// C++: `bool MSLCM_SL2015::amBlockingFollower()` [protected]
    // inline bool amBlockingFollower() {
    //         return (myOwnState & LCA_AMBLOCKINGFOLLOWER) != 0;
    //     }
    #[allow(unused_variables)]
    pub fn am_blocking_follower(&mut self) -> bool {
        todo!("port MSLCM_SL2015::amBlockingFollower")
    }

    /// C++: `bool MSLCM_SL2015::amBlockingFollowerNB()` [protected]
    // inline bool amBlockingFollowerNB() {
    //         return (myOwnState & LCA_AMBLOCKINGFOLLOWER_DONTBRAKE) != 0;
    //     }
    #[allow(unused_variables)]
    pub fn am_blocking_follower_nb(&mut self) -> bool {
        todo!("port MSLCM_SL2015::amBlockingFollowerNB")
    }

    /// C++: `bool MSLCM_SL2015::amBlockingFollowerPlusNB()` [protected]
    // inline bool amBlockingFollowerPlusNB() {
    //         return (myOwnState & (LCA_AMBLOCKINGFOLLOWER | LCA_AMBLOCKINGFOLLOWER_DONTBRAKE)) != 0;
    //     }
    #[allow(unused_variables)]
    pub fn am_blocking_follower_plus_nb(&mut self) -> bool {
        todo!("port MSLCM_SL2015::amBlockingFollowerPlusNB")
    }

    /// C++: `bool MSLCM_SL2015::currentDistDisallows(double, int, double)` [protected]
    // inline bool currentDistDisallows(double dist, int laneOffset, double lookForwardDist) {
    //         return dist / (abs(laneOffset)) < lookForwardDist;
    //     }
    #[allow(unused_variables)]
    pub fn current_dist_disallows(&mut self, dist: f64, laneOffset: i32, lookForwardDist: f64) -> bool {
        todo!("port MSLCM_SL2015::currentDistDisallows")
    }

    /// C++: `bool MSLCM_SL2015::currentDistAllows(double, int, double)` [protected]
    // inline bool currentDistAllows(double dist, int laneOffset, double lookForwardDist) {
    //         return dist / abs(laneOffset) > lookForwardDist;
    //     }
    #[allow(unused_variables)]
    pub fn current_dist_allows(&mut self, dist: f64, laneOffset: i32, lookForwardDist: f64) -> bool {
        todo!("port MSLCM_SL2015::currentDistAllows")
    }

    /// C++: `void MSLCM_SL2015::updateExpectedSublaneSpeeds(const MSLeaderDistanceInfo &, int, int)` [virtual, protected]
    /// update expected speeds for each sublane of the current edge
    // void
    // MSLCM_SL2015::updateExpectedSublaneSpeeds(const MSLeaderDistanceInfo& ahead, int sublaneOffset, int laneIndex) {
    //     const std::vector<MSLane*>& lanes = myVehicle.getLane()->getEdge().getLanes();
    //     const std::vector<MSVehicle::LaneQ>& preb = myVehicle.getBestLanes();
    //     const MSLane* lane = isOpposite() ? myVehicle.getLane()->getParallelOpposite() : lanes[laneIndex];
    //     const MSLane* next = myVehicle.getBestLanesContinuation().size() > 1 ? myVehicle.getBestLanesContinuation()[1] : nullptr;
    //     const MSLink* link = next != nullptr ? lane->getLinkTo(next) : nullptr;
    //     const double shift = link != nullptr ? link->getLateralShift() + 0.5 * (lane->getWidth() - next->getWidth()) : 0;
    //     const MSLane* bidi = myVehicle.getLane()->getBidiLane();
    //     const double vMax = lane->getVehicleMaxSpeed(&myVehicle);
    //     assert(preb.size() == lanes.size() || isOpposite());
    // #ifdef DEBUG_EXPECTED_SLSPEED
    //     if (DEBUG_COND) {
    //         std::cout << SIMTIME << " veh=" << myVehicle.getID() << " updateExpectedSublaneSpeeds opposite=" << isOpposite()
    //                   << " sublaneOffset=" << sublaneOffset << " laneIndex=" << laneIndex << " lane=" << lane->getID() << " ahead=" << ahead.toString() << "\n";
    //     }
    // #endif
    //
    //     for (int sublane = 0; sublane < (int)ahead.numSublanes(); ++sublane) {
    //         const int edgeSublane = sublane + sublaneOffset;
    //         if (edgeSublane >= (int)myExpectedSublaneSpeeds.size()) {
    //             // this may happen if a sibling lane is wider than the changer lane
    //             continue;
    //         }
    //         if (link != nullptr && lane->getWidth() > next->getWidth() + NUMERICAL_EPS && MSGlobals::gLateralResolution > 0 && sublaneEnds(sublane, next, shift)) {
    //             // sublane does not continue, discourage from use
    //             myExpectedSublaneSpeeds[edgeSublane] = 0;
    // #ifdef DEBUG_EXPECTED_SLSPEED
    //             if (DEBUG_COND) {
    //                 std::cout << "   updateExpectedSublaneSpeeds sublane=" << sublane << " doesNotContinue\n";
    //             }
    // #endif
    //             continue;
    //         } else if (lane->allowsVehicleClass(myVehicle.getVehicleType().getVehicleClass())) {
    //             // lane allowed, find potential leaders and compute safe speeds
    //             // XXX anticipate future braking if leader has a lower speed than myVehicle
    //             const MSVehicle* leader = ahead[sublane].first;
    //             const double gap = ahead[sublane].second;
    //             double vSafe;
    //             if (leader == nullptr) {
    //                 if (hasBlueLight()) {
    //                     // can continue from any lane if necessary
    //                     vSafe = vMax;
    //                 } else {
    //                     const int prebIndex = isOpposite() ? (int)preb.size() - 1 : laneIndex;
    //                     const double dist = preb[prebIndex].length - myVehicle.getPositionOnLane();
    //                     vSafe = getCarFollowModel().followSpeed(&myVehicle, vMax, dist, 0, 0);
    //                 }
    //             } else if (bidi != nullptr && leader->getLane()->getBidiLane() != nullptr && isBidi(leader->getLane())) {
    //                 // oncoming
    //                 if (gap < (1 + mySpeedGainLookahead * 2) * (vMax + leader->getSpeed())) {
    //                     vSafe = 0;
    //                 } else {
    //                     vSafe = vMax;
    //                 }
    // #ifdef DEBUG_EXPECTED_SLSPEED
    //                 if (DEBUG_COND) {
    //                     std::cout << SIMTIME << " updateExpectedSublaneSpeeds sublane=" << sublane << " leader=" << leader->getID() << " bidi=" << bidi->getID() << " gap=" << gap << " vSafe=" << vSafe << "\n";
    //                 }
    // #endif
    //             } else {
    //                 if (leader->getAcceleration() > 0.5 * leader->getCarFollowModel().getMaxAccel()) {
    //                     // assume that the leader will continue accelerating to its maximum speed
    //                     vSafe = leader->getLane()->getVehicleMaxSpeed(leader);
    //                 } else {
    //                     vSafe = getCarFollowModel().followSpeed(
    //                                 &myVehicle, vMax, gap, leader->getSpeed(), leader->getCarFollowModel().getMaxDecel());
    // #ifdef DEBUG_EXPECTED_SLSPEED
    //                     if (DEBUG_COND) {
    //                         std::cout << "   updateExpectedSublaneSpeeds edgeSublane=" << edgeSublane << " leader=" << leader->getID() << " gap=" << gap << " vSafe=" << vSafe << "\n";
    //                     }
    // #endif
    //                     vSafe = forecastAverageSpeed(vSafe, vMax, gap, leader->getSpeed());
    //                 }
    //             }
    //             // take pedestrians into account
    //             if (lane->getEdge().getPersons().size() > 0 && lane->hasPedestrians()) {
    //                 /// XXX this could be done faster by checking all sublanes at once (but would complicate the MSPModel API)
    //                 double foeRight, foeLeft;
    //                 ahead.getSublaneBorders(sublane, 0, foeRight, foeLeft);
    //                 // get all leaders ahead or overlapping
    //                 const PersonDist pedLeader = lane->nextBlocking(myVehicle.getPositionOnLane() - myVehicle.getVehicleType().getLength(), foeRight, foeLeft);
    //                 if (pedLeader.first != 0) {
    //                     const double pedGap = pedLeader.second - myVehicle.getVehicleType().getMinGap() - myVehicle.getVehicleType().getLength();
    //                     // we do not know the walking direction here so we take the pedestrian speed as 0
    //                     vSafe = MIN2(getCarFollowModel().stopSpeed(&myVehicle, vMax, pedGap),
    //                                  forecastAverageSpeed(vSafe, vMax, pedGap, 0));
    // #ifdef DEBUG_EXPECTED_SLSPEED
    //                     if (DEBUG_COND) {
    //                         std::cout << "   updateExpectedSublaneSpeeds edgeSublane=" << edgeSublane << " pedLeader=" << pedLeader.first->getID() << " gap=" << pedGap << " vSafe=" << vSafe << "\n";
    //                     }
    // #endif
    //                 }
    //             }
    //             // take bidi pedestrians into account
    //             if (bidi != nullptr && bidi->getEdge().getPersons().size() > 0 && bidi->hasPedestrians()) {
    //                 /// XXX this could be done faster by checking all sublanes at once (but would complicate the MSPModel API)
    //                 double foeRight, foeLeft;
    //                 ahead.getSublaneBorders(sublane, 0, foeRight, foeLeft);
    //                 const double foeRightBidi = bidi->getWidth() - foeLeft;
    //                 const double foeLeftBidi = bidi->getWidth() - foeRight;
    //                 // get all leaders ahead or overlapping
    //                 const double relativeBackPos = myVehicle.getLane()->getLength() - myVehicle.getPositionOnLane() + myVehicle.getLength();
    //                 const double stopTime = ceil(myVehicle.getSpeed() / myVehicle.getCarFollowModel().getMaxDecel());
    //                 PersonDist pedLeader = bidi->nextBlocking(relativeBackPos, foeRightBidi, foeLeftBidi, stopTime, true);
    //                 if (pedLeader.first != 0) {
    //                     const double pedGap = pedLeader.second - myVehicle.getVehicleType().getMinGap() - myVehicle.getVehicleType().getLength();
    //                     // we do not know the walking direction here so we take the pedestrian speed as 0
    //                     vSafe = MIN2(getCarFollowModel().stopSpeed(&myVehicle, vMax, pedGap),
    //                                  forecastAverageSpeed(vSafe, vMax, pedGap, 0));
    // #ifdef DEBUG_EXPECTED_SLSPEED
    //                     if (DEBUG_COND) {
    //                         std::cout << "   updateExpectedSublaneSpeeds edgeSublane=" << edgeSublane << " pedLeader=" << pedLeader.first->getID() << " (bidi) gap=" << pedGap << " vSafe=" << vSafe << "\n";
    //                     }
    // #endif
    //                 }
    //             }
    //             vSafe = MIN2(vMax, vSafe);
    //             // forget old data when on the opposite side
    //             const double memoryFactor = isOpposite() ? 0 : pow(SPEEDGAIN_MEMORY_FACTOR, myVehicle.getActionStepLengthSecs());
    //             myExpectedSublaneSpeeds[edgeSublane] = memoryFactor * myExpectedSublaneSpeeds[edgeSublane] + (1 - memoryFactor) * vSafe;
    //         } else {
    //             // lane forbidden
    //             myExpectedSublaneSpeeds[edgeSublane] = -1;
    // #ifdef DEBUG_EXPECTED_SLSPEED
    //             if (DEBUG_COND) {
    //                 std::cout << "   updateExpectedSublaneSpeeds edgeSublane=" << edgeSublane << " lane " << lane->getID() << " forbidden\n";
    //             }
    // #endif
    //         }
    //     }
    //     // XXX deal with leaders on subsequent lanes based on preb
    // }
    #[allow(unused_variables)]
    pub fn update_expected_sublane_speeds(&mut self, ahead: &MSLeaderDistanceInfo, sublaneOffset: i32, laneIndex: i32) {
        todo!("port MSLCM_SL2015::updateExpectedSublaneSpeeds")
    }

    /// C++: `StateAndDist MSLCM_SL2015::decideDirection(StateAndDist, StateAndDist) const` [const, virtual, protected]
    /// decide in which direction to move in case both directions are desirable
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn decide_direction(&self, sd1: MSAbstractLaneChangeModel_StateAndDist, sd2: MSAbstractLaneChangeModel_StateAndDist) -> MSAbstractLaneChangeModel_StateAndDist {
        todo!("port MSLCM_SL2015::decideDirection")
    }

    /// C++: `int MSLCM_SL2015::lowest_bit(int)` [static, protected]
    /// return the most important change reason
    // int
    // MSLCM_SL2015::lowest_bit(int changeReason) {
    //     if ((changeReason & LCA_STRATEGIC) != 0) {
    //         return LCA_STRATEGIC;
    //     }
    //     if ((changeReason & LCA_COOPERATIVE) != 0) {
    //         return LCA_COOPERATIVE;
    //     }
    //     if ((changeReason & LCA_SPEEDGAIN) != 0) {
    //         return LCA_SPEEDGAIN;
    //     }
    //     if ((changeReason & LCA_KEEPRIGHT) != 0) {
    //         return LCA_KEEPRIGHT;
    //     }
    //     if ((changeReason & LCA_TRACI) != 0) {
    //         return LCA_TRACI;
    //     }
    //     return changeReason;
    // }
    #[allow(unused_variables)]
    pub fn lowest_bit(changeReason: i32) -> i32 {
        todo!("port MSLCM_SL2015::lowest_bit")
    }

    /// C++: `void MSLCM_SL2015::msg(const CLeaderDist &, double, int)` [protected]
    /// send a speed recommendation to the given vehicle
    // void
    // MSLCM_SL2015::msg(const CLeaderDist& cld, double speed, int state) {
    //     assert(cld.first != 0);
    //     ((MSVehicle*)cld.first)->getLaneChangeModel().inform(new Info(speed, state), &myVehicle);
    // }
    #[allow(unused_variables)]
    pub fn msg(&mut self, cld: &CLeaderDist, speed: f64, state: i32) {
        todo!("port MSLCM_SL2015::msg")
    }

    /// C++: `int MSLCM_SL2015::computeSublaneShift(const MSEdge *, const MSEdge *)` [protected]
    /// compute shift so that prevSublane + shift = newSublane
    // int
    // MSLCM_SL2015::computeSublaneShift(const MSEdge* prevEdge, const MSEdge* curEdge) {
    //     // find the first lane that targets the new edge
    //     int prevShift = 0;
    //     for (const MSLane* const lane : prevEdge->getLanes()) {
    //         for (const MSLink* const link : lane->getLinkCont()) {
    //             if (&link->getLane()->getEdge() == curEdge) {
    //                 int curShift = 0;
    //                 const MSLane* target = link->getLane();
    //                 const std::vector<MSLane*>& lanes2 = curEdge->getLanes();
    //                 for (std::vector<MSLane*>::const_iterator it_lane2 = lanes2.begin(); it_lane2 != lanes2.end(); ++it_lane2) {
    //                     const MSLane* lane2 = *it_lane2;
    //                     if (lane2 == target) {
    //                         return prevShift + curShift;
    //                     }
    //                     MSLeaderInfo ahead(lane2->getWidth());
    //                     curShift += ahead.numSublanes();
    //                 }
    //                 assert(false);
    //             }
    //         }
    //         MSLeaderInfo ahead(lane->getWidth());
    //         prevShift -= ahead.numSublanes();
    //     }
    //     return std::numeric_limits<int>::max();
    // }
    #[allow(unused_variables)]
    pub fn compute_sublane_shift(&mut self, prevEdge: Ptr<MSEdge>, curEdge: Ptr<MSEdge>) -> i32 {
        todo!("port MSLCM_SL2015::computeSublaneShift")
    }

    /// C++: `CLeaderDist MSLCM_SL2015::getLongest(const MSLeaderDistanceInfo &) const` [const, protected]
    /// get the longest vehicle in the given info
    // CLeaderDist
    // MSLCM_SL2015::getLongest(const MSLeaderDistanceInfo& ldi) const {
    //     int iMax = -1;
    //     double maxLength = -1;
    //     for (int i = 0; i < ldi.numSublanes(); ++i) {
    //         const MSVehicle* veh = ldi[i].first;
    //         if (veh) {
    //             const double length = veh->getVehicleType().getLength();
    //             if (length > maxLength && tieBrakeLeader(veh)) {
    //                 maxLength = length;
    //                 iMax = i;
    //             }
    //         }
    //     }
    //     return iMax >= 0 ? ldi[iMax] : std::make_pair(nullptr, -1);
    // }
    #[allow(unused_variables)]
    pub fn get_longest(&self, ldi: &MSLeaderDistanceInfo) -> CLeaderDist {
        todo!("port MSLCM_SL2015::getLongest")
    }

    /// C++: `bool MSLCM_SL2015::tieBrakeLeader(const MSVehicle *) const` [const, protected]
    // bool
    // MSLCM_SL2015::tieBrakeLeader(const MSVehicle* veh) const {
    //     // tie braker if the leader is at the same lane position
    //     return veh != nullptr && (veh->getPositionOnLane() != myVehicle.getPositionOnLane()
    //                               || veh->getSpeed() < myVehicle.getSpeed()
    //                               || &veh->getLane()->getEdge() != &myVehicle.getLane()->getEdge()
    //                               || veh->getLane()->getIndex() > myVehicle.getLane()->getIndex());
    // }
    #[allow(unused_variables)]
    pub fn tie_brake_leader(&self, veh: Ptr<MSVehicle>) -> bool {
        todo!("port MSLCM_SL2015::tieBrakeLeader")
    }

    /// C++: `CLeaderDist MSLCM_SL2015::getSlowest(const MSLeaderDistanceInfo &)` [static, protected]
    /// get the slowest vehicle in the given info
    // CLeaderDist
    // MSLCM_SL2015::getSlowest(const MSLeaderDistanceInfo& ldi) {
    //     int iMax = 0;
    //     double minSpeed = std::numeric_limits<double>::max();
    //     for (int i = 0; i < ldi.numSublanes(); ++i) {
    //         if (ldi[i].first != 0) {
    //             const double speed = ldi[i].first->getSpeed();
    //             if (speed < minSpeed) {
    //                 minSpeed = speed;
    //                 iMax = i;
    //             }
    //         }
    //     }
    //     return ldi[iMax];
    // }
    #[allow(unused_variables)]
    pub fn get_slowest(ldi: &MSLeaderDistanceInfo) -> CLeaderDist {
        todo!("port MSLCM_SL2015::getSlowest")
    }

    /// C++: `const MSVehicle * MSLCM_SL2015::getStopped(const MSLeaderDistanceInfo &)` [static, protected]
    /// get a stopped vehicle in the given info
    // const MSVehicle*
    // MSLCM_SL2015::getStopped(const MSLeaderDistanceInfo& ldi) {
    //     if (ldi.hasVehicles()) {
    //         for (const MSVehicle* const v : ldi.getVehicles()) {
    //             if (v != nullptr && v->isStopped()) {
    //                 return v;
    //             }
    //         }
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_stopped(ldi: &MSLeaderDistanceInfo) -> Ptr<MSVehicle> {
        todo!("port MSLCM_SL2015::getStopped")
    }

    /// C++: `bool MSLCM_SL2015::hasBidiLeader(const MSLeaderDistanceInfo &, const std::vector<MSLane *> &)` [protected]
    /// return true if any leader on the upcoming lanes is driving in the opposite direction
    // bool
    // MSLCM_SL2015::hasBidiLeader(const MSLeaderDistanceInfo& ldi, const std::vector<MSLane*>& conts) {
    //     if (ldi.hasVehicles()) {
    //         for (const MSVehicle* const v : ldi.getVehicles()) {
    //             if (MSLCHelper::isBidiLeader(v, conts)) {
    //                 return true;
    //             }
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn has_bidi_leader(&mut self, ldi: &MSLeaderDistanceInfo, conts: &Vec<Ptr<MSLane>>) -> bool {
        todo!("port MSLCM_SL2015::hasBidiLeader")
    }

    /// C++: `int MSLCM_SL2015::checkBlocking(const MSLane &, double &, double, int, const MSLeaderDistanceInfo &, const MSLeaderDistanceInfo &, const MSLeaderDistanceInfo &, const MSLeaderDistanceInfo &, const MSLeaderDistanceInfo &, const MSLeaderDistanceInfo &, std::vector<CLeaderDist> *, std::vector<CLeaderDist> *, bool, double, int *)` [protected]
    /// restrict latDist to permissible speed and determine blocking state depending on that distance
    // int
    // MSLCM_SL2015::checkBlocking(const MSLane& neighLane, double& latDist, double maneuverDist, int laneOffset,
    //                             const MSLeaderDistanceInfo& leaders,
    //                             const MSLeaderDistanceInfo& followers,
    //                             const MSLeaderDistanceInfo& /*blockers */,
    //                             const MSLeaderDistanceInfo& neighLeaders,
    //                             const MSLeaderDistanceInfo& neighFollowers,
    //                             const MSLeaderDistanceInfo& /* neighBlockers */,
    //                             std::vector<CLeaderDist>* collectLeadBlockers,
    //                             std::vector<CLeaderDist>* collectFollowBlockers,
    //                             bool keepLatGapManeuver,
    //                             double gapFactor,
    //                             int* retBlockedFully) {
    //     // truncate latDist according to maxSpeedLat
    //     const double maxDist = SPEED2DIST(getMaxSpeedLat2());
    //     latDist = MAX2(MIN2(latDist, maxDist), -maxDist);
    //     if (myVehicle.hasInfluencer() && myVehicle.getInfluencer().getLatDist() != 0 && myVehicle.getInfluencer().ignoreOverlap()) {
    //         return 0;
    //     }
    //
    //     const double neighRight = getNeighRight(neighLane);
    //     if (!myCFRelatedReady) {
    //         updateCFRelated(followers, myVehicle.getLane()->getRightSideOnEdge(), false);
    //         updateCFRelated(leaders, myVehicle.getLane()->getRightSideOnEdge(), true);
    //         if (laneOffset != 0) {
    //             updateCFRelated(neighFollowers, neighRight, false);
    //             updateCFRelated(neighLeaders, neighRight, true);
    //         }
    //         myCFRelatedReady = true;
    //     }
    //
    //     // reduce latDist to avoid blockage with overlapping vehicles (no minGapLat constraints)
    //     const double center = myVehicle.getCenterOnEdge();
    //     updateGaps(leaders, myVehicle.getLane()->getRightSideOnEdge(), center, gapFactor, mySafeLatDistRight, mySafeLatDistLeft, false, 0, latDist, collectLeadBlockers);
    //     updateGaps(followers, myVehicle.getLane()->getRightSideOnEdge(), center, gapFactor, mySafeLatDistRight, mySafeLatDistLeft, false, 0, latDist, collectFollowBlockers);
    //     if (laneOffset != 0) {
    //         updateGaps(neighLeaders, neighRight, center, gapFactor, mySafeLatDistRight, mySafeLatDistLeft, false, 0, latDist, collectLeadBlockers);
    //         updateGaps(neighFollowers, neighRight, center, gapFactor, mySafeLatDistRight, mySafeLatDistLeft, false, 0, latDist, collectFollowBlockers);
    //     }
    // #ifdef DEBUG_BLOCKING
    //     if (gDebugFlag2) {
    //         std::cout << "    checkBlocking latDist=" << latDist << " mySafeLatDistRight=" << mySafeLatDistRight << " mySafeLatDistLeft=" << mySafeLatDistLeft << "\n";
    //     }
    // #endif
    //     // if we can move at least a little bit in the desired direction, do so (rather than block)
    //     const bool forcedTraCIChange = (myVehicle.hasInfluencer()
    //                                     && myVehicle.getInfluencer().getLatDist() != 0
    //                                     && myVehicle.getInfluencer().ignoreOverlap());
    //     if (latDist < 0) {
    //         if (mySafeLatDistRight <= NUMERICAL_EPS) {
    //             return LCA_BLOCKED_RIGHT | LCA_OVERLAPPING;
    //         } else if (!forcedTraCIChange) {
    //             latDist = MAX2(latDist, -mySafeLatDistRight);
    //         }
    //     } else {
    //         if (mySafeLatDistLeft <= NUMERICAL_EPS) {
    //             return LCA_BLOCKED_LEFT | LCA_OVERLAPPING;
    //         } else if (!forcedTraCIChange) {
    //             latDist = MIN2(latDist, mySafeLatDistLeft);
    //         }
    //     }
    //
    //     myCanChangeFully = (maneuverDist == 0 || latDist == maneuverDist);
    // #ifdef DEBUG_BLOCKING
    //     if (gDebugFlag2) {
    //         std::cout << "    checkBlocking latDist=" << latDist << " maneuverDist=" << maneuverDist << "\n";
    //     }
    // #endif
    //     // destination sublanes must be safe
    //     // intermediate sublanes must not be blocked by overlapping vehicles
    //
    //     // XXX avoid checking the same leader multiple times
    //     // XXX ensure that only changes within the same lane are undertaken if laneOffset = 0
    //
    //     int blocked = 0;
    //     blocked |= checkBlockingVehicles(&myVehicle, leaders, laneOffset, latDist, myVehicle.getLane()->getRightSideOnEdge(), true,
    //                                      mySafeLatDistRight, mySafeLatDistLeft, collectLeadBlockers);
    //     blocked |= checkBlockingVehicles(&myVehicle, followers, laneOffset, latDist, myVehicle.getLane()->getRightSideOnEdge(), false,
    //                                      mySafeLatDistRight, mySafeLatDistLeft, collectFollowBlockers);
    //     if (laneOffset != 0) {
    //         blocked |= checkBlockingVehicles(&myVehicle, neighLeaders, laneOffset, latDist, neighRight, true,
    //                                          mySafeLatDistRight, mySafeLatDistLeft, collectLeadBlockers);
    //         blocked |= checkBlockingVehicles(&myVehicle, neighFollowers, laneOffset, latDist, neighRight, false,
    //                                          mySafeLatDistRight, mySafeLatDistLeft, collectFollowBlockers);
    //     }
    //
    //     int blockedFully = 0;
    //     blockedFully |= checkBlockingVehicles(&myVehicle, leaders, laneOffset, maneuverDist, myVehicle.getLane()->getRightSideOnEdge(), true,
    //                                           mySafeLatDistRight, mySafeLatDistLeft, collectLeadBlockers);
    //     blockedFully |= checkBlockingVehicles(&myVehicle, followers, laneOffset, maneuverDist, myVehicle.getLane()->getRightSideOnEdge(), false,
    //                                           mySafeLatDistRight, mySafeLatDistLeft, collectFollowBlockers);
    //     if (laneOffset != 0) {
    //         blockedFully |= checkBlockingVehicles(&myVehicle, neighLeaders, laneOffset, maneuverDist, neighRight, true,
    //                                               mySafeLatDistRight, mySafeLatDistLeft, collectLeadBlockers);
    //         blockedFully |= checkBlockingVehicles(&myVehicle, neighFollowers, laneOffset, maneuverDist, neighRight, false,
    //                                               mySafeLatDistRight, mySafeLatDistLeft, collectFollowBlockers);
    //     }
    //     if (retBlockedFully != nullptr) {
    //         *retBlockedFully = blockedFully;
    //     }
    // #ifdef DEBUG_BLOCKING
    //     if (gDebugFlag2) {
    //         std::cout << "    blocked=" << blocked << " (" << toString((LaneChangeAction)blocked) << ") blockedFully=" << toString((LaneChangeAction)blockedFully)
    //                   << " canChangeFully=" << myCanChangeFully << " keepLatGapManeuver=" << keepLatGapManeuver << "\n";
    //     }
    // #endif
    //     if (blocked == 0 && !myCanChangeFully && myPushy == 0 && !keepLatGapManeuver) {
    //         // aggressive drivers immediately start moving towards potential
    //         // blockers and only check that the start of their maneuver (latDist) is safe. In
    //         // contrast, cautious drivers need to check latDist and origLatDist to
    //         // ensure that the maneuver can be finished without encroaching on other vehicles.
    //         blocked |= blockedFully;
    //     } else {
    //         // XXX: in case of action step length > simulation step length, pushing may lead to collisions,
    //         //      because maneuver is continued until maneuverDist is reached (perhaps set maneuverDist=latDist)
    //     }
    // #ifdef DEBUG_BLOCKING
    //     if (gDebugFlag2) {
    //         std::cout << "    blocked2=" << blocked << " (" << toString((LaneChangeAction)blocked) << ")\n";
    //     }
    // #endif
    //     if (collectFollowBlockers != nullptr && collectLeadBlockers != nullptr) {
    //         // prevent vehicles from being classified as leader and follower simultaneously
    //         for (std::vector<CLeaderDist>::const_iterator it2 = collectLeadBlockers->begin(); it2 != collectLeadBlockers->end(); ++it2) {
    //             for (std::vector<CLeaderDist>::iterator it = collectFollowBlockers->begin(); it != collectFollowBlockers->end();) {
    //                 if ((*it2).first == (*it).first) {
    // #ifdef DEBUG_BLOCKING
    //                     if (gDebugFlag2) {
    //                         std::cout << "    removed follower " << (*it).first->getID() << " because it is already a leader\n";
    //                     }
    // #endif
    //                     it = collectFollowBlockers->erase(it);
    //                 } else {
    //                     ++it;
    //                 }
    //             }
    //         }
    //     }
    //     return blocked;
    // }
    #[allow(unused_variables)]
    pub fn check_blocking(&mut self, neighLane: &MSLane, latDist: &mut f64, maneuverDist: f64, laneOffset: i32, leaders: &MSLeaderDistanceInfo, followers: &MSLeaderDistanceInfo, blockers: &MSLeaderDistanceInfo, neighLeaders: &MSLeaderDistanceInfo, neighFollowers: &MSLeaderDistanceInfo, neighBlockers: &MSLeaderDistanceInfo, collectLeadBlockers: Ptr<Vec<CLeaderDist>>, collectFollowBlockers: Ptr<Vec<CLeaderDist>>, keepLatGapManeuver: bool, gapFactor: f64, retBlockedFully: Ptr<i32>) -> i32 {
        todo!("port MSLCM_SL2015::checkBlocking")
    }

    /// C++: `int MSLCM_SL2015::checkBlockingVehicles(const MSVehicle *, const MSLeaderDistanceInfo &, int, double, double, bool, double &, double &, std::vector<CLeaderDist> *) const` [const, protected]
    /// check whether any of the vehicles overlaps with ego
    // int
    // MSLCM_SL2015::checkBlockingVehicles(
    //     const MSVehicle* ego, const MSLeaderDistanceInfo& vehicles,
    //     int laneOffset, double latDist, double foeOffset, bool leaders,
    //     double& safeLatGapRight, double& safeLatGapLeft,
    //     std::vector<CLeaderDist>* collectBlockers) const {
    //     // determine borders where safety/no-overlap conditions must hold
    //     const LaneChangeAction blockType = (laneOffset == 0
    //                                         ? (leaders ? LCA_BLOCKED_BY_LEADER : LCA_BLOCKED_BY_FOLLOWER)
    //                                         : (laneOffset > 0
    //                                            ? (leaders ? LCA_BLOCKED_BY_LEFT_LEADER : LCA_BLOCKED_BY_LEFT_FOLLOWER)
    //                                            : (leaders ? LCA_BLOCKED_BY_RIGHT_LEADER : LCA_BLOCKED_BY_RIGHT_FOLLOWER)));
    //     const double vehWidth = getWidth();
    //     const double rightVehSide = ego->getRightSideOnEdge();
    //     const double leftVehSide = rightVehSide + vehWidth;
    //     const double rightVehSideDest = rightVehSide + latDist;
    //     const double leftVehSideDest = leftVehSide + latDist;
    //     const double rightNoOverlap = MIN2(rightVehSideDest, rightVehSide);
    //     const double leftNoOverlap = MAX2(leftVehSideDest, leftVehSide);
    // #ifdef DEBUG_BLOCKING
    //     if (gDebugFlag2) {
    //         std::cout << "  checkBlockingVehicles"
    //                   << " laneOffset=" << laneOffset
    //                   << " latDist=" << latDist
    //                   << " foeOffset=" << foeOffset
    //                   << " vehRight=" << rightVehSide
    //                   << " vehLeft=" << leftVehSide
    //                   << " rightNoOverlap=" << rightNoOverlap
    //                   << " leftNoOverlap=" << leftNoOverlap
    //                   << " destRight=" << rightVehSideDest
    //                   << " destLeft=" << leftVehSideDest
    //                   << " leaders=" << leaders
    //                   << " blockType=" << toString((LaneChangeAction) blockType)
    //                   << "\n";
    //     }
    // #endif
    //     int result = 0;
    //     for (int i = 0; i < vehicles.numSublanes(); ++i) {
    //         CLeaderDist vehDist = vehicles[i];
    //         if (vehDist.first != 0 && myCFRelated.count(vehDist.first) == 0) {
    //             const MSVehicle* leader = vehDist.first;
    //             const MSVehicle* follower = ego;
    //             if (!leaders) {
    //                 std::swap(leader, follower);
    //             }
    //             // only check the current stripe occupied by foe (transform into edge-coordinates)
    //             double foeRight, foeLeft;
    //             vehicles.getSublaneBorders(i, foeOffset, foeRight, foeLeft);
    //             const bool overlapBefore = overlap(rightVehSide, leftVehSide, foeRight, foeLeft);
    //             const bool overlapDest = overlap(rightVehSideDest, leftVehSideDest, foeRight, foeLeft);
    //             const bool overlapAny = overlap(rightNoOverlap, leftNoOverlap, foeRight, foeLeft);
    // #ifdef DEBUG_BLOCKING
    //             if (gDebugFlag2) {
    //                 std::cout << "   foe=" << vehDist.first->getID()
    //                           << " gap=" << vehDist.second
    //                           << " secGap=" << follower->getCarFollowModel().getSecureGap(follower, leader, follower->getSpeed(), leader->getSpeed(), leader->getCarFollowModel().getMaxDecel())
    //                           << " foeRight=" << foeRight
    //                           << " foeLeft=" << foeLeft
    //                           << " overlapBefore=" << overlapBefore
    //                           << " overlap=" << overlapAny
    //                           << " overlapDest=" << overlapDest
    //                           << "\n";
    //             }
    // #endif
    //             if (overlapAny) {
    //                 if (vehDist.second < 0) {
    //                     if (overlapBefore && !overlapDest && !outsideEdge()) {
    // #ifdef DEBUG_BLOCKING
    //                         if (gDebugFlag2) {
    //                             std::cout << "    ignoring current overlap to come clear\n";
    //                         }
    // #endif
    //                     } else {
    // #ifdef DEBUG_BLOCKING
    //                         if (gDebugFlag2) {
    //                             std::cout << "    overlap (" << toString((LaneChangeAction)blockType) << ")\n";
    //                         }
    // #endif
    //                         result |= (blockType | LCA_OVERLAPPING);
    //                         if (collectBlockers == nullptr) {
    //                             return result;
    //                         } else {
    //                             collectBlockers->push_back(vehDist);
    //                         }
    //                     }
    //                 } else if (overlapDest || !myCanChangeFully) {
    //                     // Estimate state after actionstep (follower may be accelerating!)
    //                     // A comparison between secure gap depending on the expected speeds and the extrapolated gap
    //                     // determines whether the s is blocking the lane change.
    //                     // (Note that the longitudinal state update has already taken effect before LC dynamics (thus "-TS" below), would be affected by #3665)
    //
    //                     // Use conservative estimate for time until next action step
    //                     // (XXX: how can the ego know the foe's action step length?)
    //                     const double timeTillAction = MAX2(follower->getActionStepLengthSecs(), leader->getActionStepLengthSecs()) - TS;
    //                     // Ignore decel for follower
    //                     const double followerAccel = MAX2(0., follower->getAcceleration());
    //                     const double leaderAccel = leader->getAcceleration();
    //                     // Expected gap after next actionsteps
    //                     const double expectedGap = MSCFModel::gapExtrapolation(timeTillAction, vehDist.second, leader->getSpeed(), follower->getSpeed(), leaderAccel, followerAccel, std::numeric_limits<double>::max(), std::numeric_limits<double>::max());
    //
    //                     // Determine expected speeds and corresponding secure gap at the extrapolated timepoint
    //                     const double followerExpectedSpeed = follower->getSpeed() + timeTillAction * followerAccel;
    //                     const double leaderExpectedSpeed = MAX2(0., leader->getSpeed() + timeTillAction * leaderAccel);
    //                     const double expectedSecureGap = follower->getCarFollowModel().getSecureGap(follower, leader, followerExpectedSpeed, leaderExpectedSpeed, leader->getCarFollowModel().getMaxDecel());
    //
    // #if defined(DEBUG_ACTIONSTEPS) && defined(DEBUG_BLOCKING)
    //                     if (gDebugFlag2) {
    //                         std::cout << "    timeTillAction=" << timeTillAction
    //                                   << " followerAccel=" << followerAccel
    //                                   << " followerExpectedSpeed=" << followerExpectedSpeed
    //                                   << " leaderAccel=" << leaderAccel
    //                                   << " leaderExpectedSpeed=" << leaderExpectedSpeed
    //                                   << "\n    gap=" << vehDist.second
    //                                   << " gapChange=" << (expectedGap - vehDist.second)
    //                                   << " expectedGap=" << expectedGap
    //                                   << " expectedSecureGap=" << expectedSecureGap
    //                                   << " safeLatGapLeft=" << safeLatGapLeft
    //                                   << " safeLatGapRight=" << safeLatGapRight
    //                                   << std::endl;
    //                     }
    // #endif
    //
    //                     // @note for euler-update, a different value for secureGap2 may be obtained when applying safetyFactor to followerDecel rather than secureGap
    //                     const double secureGap2 = expectedSecureGap * getSafetyFactor();
    //                     if (expectedGap < secureGap2) {
    //                         // Foe is a blocker. Update lateral safe gaps accordingly.
    //                         if (foeRight > leftVehSide) {
    //                             safeLatGapLeft = MIN2(safeLatGapLeft, foeRight - leftVehSide);
    //                         } else if (foeLeft < rightVehSide) {
    //                             safeLatGapRight = MIN2(safeLatGapRight, rightVehSide - foeLeft);
    //                         }
    //
    // #ifdef DEBUG_BLOCKING
    //                         if (gDebugFlag2) {
    //                             std::cout << "    blocked by " << vehDist.first->getID() << " gap=" << vehDist.second << " expectedGap=" << expectedGap
    //                                       << " expectedSecureGap=" << expectedSecureGap << " secGap2=" << secureGap2 << " safetyFactor=" << getSafetyFactor()
    //                                       << " safeLatGapLeft=" << safeLatGapLeft << " safeLatGapRight=" << safeLatGapRight
    //                                       << "\n";
    //                         }
    // #endif
    //                         result |= blockType;
    //                         if (collectBlockers == nullptr) {
    //                             return result;
    //                         }
    // #ifdef DEBUG_BLOCKING
    //                     } else if (gDebugFlag2 && expectedGap < expectedSecureGap) {
    //                         std::cout << "    ignore blocker " << vehDist.first->getID() << " gap=" << vehDist.second << " expectedGap=" << expectedGap
    //                                   << " expectedSecureGap=" << expectedSecureGap << " secGap2=" << secureGap2 << " safetyFactor=" << getSafetyFactor() << "\n";
    // #endif
    //                     }
    //                     if (collectBlockers != nullptr) {
    //                         // collect non-blocking followers as well to make sure
    //                         // they remain non-blocking
    //                         collectBlockers->push_back(vehDist);
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     return result;
    //
    // }
    #[allow(unused_variables)]
    pub fn check_blocking_vehicles(&self, ego: Ptr<MSVehicle>, vehicles: &MSLeaderDistanceInfo, laneOffset: i32, latDist: f64, foeOffset: f64, leaders: bool, safeLatGapRight: &mut f64, safeLatGapLeft: &mut f64, collectBlockers: Ptr<Vec<CLeaderDist>>) -> i32 {
        todo!("port MSLCM_SL2015::checkBlockingVehicles")
    }

    /// C++: `bool MSLCM_SL2015::overlap(double, double, double, double)` [static, protected]
    /// return whether the given intervals overlap
    // bool
    // MSLCM_SL2015::overlap(double right, double left, double right2, double left2) {
    //     assert(right <= left);
    //     assert(right2 <= left2);
    //     return left2 >= right + NUMERICAL_EPS && left >= right2 + NUMERICAL_EPS;
    // }
    #[allow(unused_variables)]
    pub fn overlap(right: f64, left: f64, right2: f64, left2: f64) -> bool {
        todo!("port MSLCM_SL2015::overlap")
    }

    /// C++: `LaneChangeAction MSLCM_SL2015::getLCA(int, double)` [static, protected]
    /// compute lane change action from desired lateral distance
    // LaneChangeAction
    // MSLCM_SL2015::getLCA(int state, double latDist) {
    //     return ((latDist == 0 || (state & LCA_CHANGE_REASONS) == 0)
    //             ? LCA_NONE : (latDist < 0 ? LCA_RIGHT : LCA_LEFT));
    // }
    #[allow(unused_variables)]
    pub fn get_lca(state: i32, latDist: f64) -> LaneChangeAction {
        todo!("port MSLCM_SL2015::getLCA")
    }

    /// C++: `int MSLCM_SL2015::checkStrategicChange(int, const MSLane &, int, const MSLeaderDistanceInfo &, const MSLeaderDistanceInfo &, const MSVehicle::LaneQ &, const MSVehicle::LaneQ &, const MSVehicle::LaneQ &, int, bool, double &, double, double, double, double, bool, double &)` [protected]
    /// compute strategic lane change actions TODO: Better documentation, refs #2
    // int
    // MSLCM_SL2015::checkStrategicChange(int ret,
    //                                    const MSLane& neighLane,
    //                                    int laneOffset,
    //                                    const MSLeaderDistanceInfo& leaders,
    //                                    const MSLeaderDistanceInfo& neighLeaders,
    //                                    const MSVehicle::LaneQ& curr,
    //                                    const MSVehicle::LaneQ& neigh,
    //                                    const MSVehicle::LaneQ& best,
    //                                    int bestLaneOffset,
    //                                    bool changeToBest,
    //                                    double& currentDist,
    //                                    double neighDist,
    //                                    double laDist,
    //                                    double roundaboutBonus,
    //                                    double latLaneDist,
    //                                    bool checkOpposite,
    //                                    double& latDist
    //                                   ) {
    //     const bool right = (laneOffset == -1);
    //     const bool left = (laneOffset == 1);
    //
    //     const double forwardPos = getForwardPos();
    //     if (laneOffset != 0) {
    //         myLeftSpace = currentDist - forwardPos;
    //     }
    //     const double usableDist = (currentDist - forwardPos - best.occupation *  JAM_FACTOR);
    //     //- (best.lane->getVehicleNumber() * neighSpeed)); // VARIANT 9 jfSpeed
    //     const double maxJam = MAX2(neigh.occupation, curr.occupation);
    //     const double neighLeftPlace = MAX2(0., neighDist - forwardPos - maxJam);
    //     const double overlap = myVehicle.getLateralOverlap();
    //     // save the left space
    //
    // #ifdef DEBUG_STRATEGIC_CHANGE
    //     if (gDebugFlag2) {
    //         std::cout << SIMTIME
    //                   << " veh=" << myVehicle.getID()
    //                   << " forwardPos=" << forwardPos
    //                   << " laSpeed=" << myLookAheadSpeed
    //                   << " laDist=" << laDist
    //                   << " currentDist=" << currentDist
    //                   << " usableDist=" << usableDist
    //                   << " bestLaneOffset=" << bestLaneOffset
    //                   << " best.length=" << best.length
    //                   << " maxJam=" << maxJam
    //                   << " neighLeftPlace=" << neighLeftPlace
    //                   << " myLeftSpace=" << myLeftSpace
    //                   << " overlap=" << overlap
    //                   << "\n";
    //     }
    // #endif
    //
    //     if (laneOffset == 0) {
    //         if (overlap > MAX2(POSITION_EPS, MSGlobals::gLateralResolution)
    //                 && (getShadowLane() == nullptr || !getShadowLane()->allowsVehicleClass(myVehicle.getVClass()))
    //                 && getWidth() < myVehicle.getLane()->getWidth()) {
    //             // @brief we urgently need to return to within lane bounds
    //             latDist = myVehicle.getLateralPositionOnLane() > 0 ? -overlap : overlap;
    //             ret |= LCA_STRATEGIC | LCA_URGENT;
    // #ifdef DEBUG_STRATEGIC_CHANGE
    //             if (gDebugFlag2) {
    //                 std::cout << SIMTIME << " returnToLaneBounds\n";
    //             }
    // #endif
    //             //std::cout << SIMTIME << " veh=" << myVehicle.getID() << " overlap=" << overlap << " returnToLaneBounds\n";
    //         } else if (myVehicle.getBestLanesContinuation().size() > 1 && myVehicle.getLane()->getWidth() > myVehicle.getBestLanesContinuation()[1]->getWidth()) {
    //             const MSLane* cur = myVehicle.getLane();
    //             const MSLane* next = myVehicle.getBestLanesContinuation()[1];
    //             const MSLink* link = cur->getLinkTo(next);
    //             const double distOnLane = cur->getLength() - myVehicle.getPositionOnLane();
    //             if (link != nullptr && getWidth() < next->getWidth() && distOnLane < 100) {
    //                 double hwDiff = 0.5 * (cur->getWidth() - next->getWidth());
    //                 double rightVehSide = myVehicle.getRightSideOnLane() + link->getLateralShift() - hwDiff;
    //                 double leftVehSide = myVehicle.getLeftSideOnLane() + link->getLateralShift() - hwDiff;
    //                 const double res = MSGlobals::gLateralResolution > 0 ? MSGlobals::gLateralResolution : next->getWidth();
    //                 if (rightVehSide < -res && (next->getParallelLane(-1) == nullptr || !next->getParallelLane(-1)->allowsVehicleClass(myVehicle.getVClass()))) {
    //                     latDist = -rightVehSide;
    //                     myLeftSpace = distOnLane;
    //                     ret |= LCA_STRATEGIC | LCA_URGENT;
    // #ifdef DEBUG_STRATEGIC_CHANGE
    //                     if (gDebugFlag2) {
    //                         std::cout << SIMTIME << " rightSublaneEnds rVSide=" << myVehicle.getRightSideOnLane()
    //                                   << " shift=" << link->getLateralShift() << " rVSide2=" << rightVehSide << " myLeftSpace=" << myLeftSpace << " \n";
    //                     }
    // #endif
    //                 } else if (leftVehSide > next->getWidth() + res && (next->getParallelLane(1) == nullptr || !next->getParallelLane(1)->allowsVehicleClass(myVehicle.getVClass()))) {
    //                     latDist = -(leftVehSide - next->getWidth());
    //                     myLeftSpace = distOnLane;
    //                     ret |= LCA_STRATEGIC | LCA_URGENT;
    // #ifdef DEBUG_STRATEGIC_CHANGE
    //                     if (gDebugFlag2) {
    //                         std::cout << SIMTIME << " leftSublaneEnds lVSide=" << myVehicle.getLeftSideOnLane()
    //                                   << " shift=" << link->getLateralShift() << " lVSide2=" << leftVehSide << " myLeftSpace=" << myLeftSpace << "\n";
    //                     }
    // #endif
    //                 }
    //             }
    //         }
    //     } else if (laneOffset != 0 && changeToBest && bestLaneOffset == curr.bestLaneOffset
    //                && currentDistDisallows(usableDist, bestLaneOffset, laDist)) {
    //         /// @brief we urgently need to change lanes to follow our route
    //         if (!mustOvertakeStopped(false, neighLane, neighLeaders, leaders, forwardPos, neighDist, right, latLaneDist, neigh.bestContinuations, currentDist, latDist)) {
    //             latDist = latLaneDist;
    //             ret |= LCA_STRATEGIC | LCA_URGENT;
    // #ifdef DEBUG_STRATEGIC_CHANGE
    //             if (gDebugFlag2) {
    //                 std::cout << SIMTIME << " mustChangeToBest\n";
    //             }
    // #endif
    //         } else {
    // #ifdef DEBUG_STRATEGIC_CHANGE
    //             if (gDebugFlag2) {
    //                 std::cout << " veh=" << myVehicle.getID() << " avoidStoppedNeigh\n";
    //             }
    // #endif
    //         }
    //     } else {
    //         // VARIANT_20 (noOvertakeRight)
    //         const MSVehicle* const stoppedLeader = getStopped(neighLeaders);
    //         if (left && avoidOvertakeRight(stoppedLeader, true) && neighLeaders.hasVehicles()) {
    //             // check for slower leader on the left. we should not overtake but
    //             // rather move left ourselves (unless congested)
    //             // XXX only adapt as much as possible to get a lateral gap
    //             CLeaderDist cld = getSlowest(neighLeaders);
    //             const MSVehicle* nv = cld.first;
    //             double deltaV = 0.;
    //             double vSafe = 0.;
    //             if (canOvertakeRight(nv, cld.second, myVehicle.getLane()->getVehicleMaxSpeed(&myVehicle) - neighLane.getVehicleMaxSpeed(nv), HELP_OVERTAKE, vSafe, deltaV)) {
    //                 addLCSpeedAdvice(vSafe, LCA_MLEFT);
    //                 if (vSafe < myVehicle.getSpeed()) {
    //                     mySpeedGainProbabilityRight += myVehicle.getActionStepLengthSecs() * myChangeProbThresholdLeft / 3;
    //                 }
    // #ifdef DEBUG_STRATEGIC_CHANGE
    //                 if (gDebugFlag2) {
    //                     std::cout << SIMTIME
    //                               << " avoid overtaking on the right nv=" << nv->getID()
    //                               << " nvSpeed=" << nv->getSpeed()
    //                               << " mySpeedGainProbabilityR=" << mySpeedGainProbabilityRight
    //                               << " plannedSpeed=" << myVehicle.getSpeed() + ACCEL2SPEED(myLCAccelerationAdvices.back().first)
    //                               << "\n";
    //                 }
    // #endif
    //             }
    //         }
    //
    //         // handling reaction to stopped for opposite direction driving NYI
    //         const bool noOpposites = &myVehicle.getLane()->getEdge() == &neighLane.getEdge();
    //         if (laneOffset != 0 && myStrategicParam >= 0 && noOpposites && mustOvertakeStopped(true, neighLane, leaders, neighLeaders, forwardPos, neighDist, right, latLaneDist, curr.bestContinuations,  currentDist, latDist)) {
    // #ifdef DEBUG_STRATEGIC_CHANGE
    //             if (gDebugFlag2) {
    //                 std::cout << " veh=" << myVehicle.getID() << " mustOvertakeStopped\n";
    //             }
    // #endif
    //             if (latDist == 0) {
    //                 ret |= LCA_STAY | LCA_STRATEGIC;
    //             } else {
    //                 ret |= LCA_STRATEGIC | LCA_URGENT;
    //             }
    //
    //         } else if (!changeToBest && (currentDistDisallows(neighLeftPlace, abs(bestLaneOffset) + 2, laDist))) {
    //             // the opposite lane-changing direction should be done than the one examined herein
    //             //  we'll check whether we assume we could change anyhow and get back in time...
    //             //
    //             // this rule prevents the vehicle from moving in opposite direction of the best lane
    //             //  unless the way till the end where the vehicle has to be on the best lane
    //             //  is long enough
    // #ifdef DEBUG_STRATEGIC_CHANGE
    //             if (gDebugFlag2) {
    //                 std::cout << " veh=" << myVehicle.getID() << " could not change back and forth in time (1) neighLeftPlace=" << neighLeftPlace << "\n";
    //             }
    // #endif
    //             ret |= LCA_STAY | LCA_STRATEGIC;
    //         } else if (
    //             laneOffset != 0
    //             && bestLaneOffset == 0
    //             && !leaders.hasStoppedVehicle()
    //             && neigh.bestContinuations.back()->getLinkCont().size() != 0
    //             && roundaboutBonus == 0
    //             && !checkOpposite
    //             && neighDist < TURN_LANE_DIST
    //             && myStrategicParam >= 0) {
    //             // VARIANT_21 (stayOnBest)
    //             // we do not want to leave the best lane for a lane which leads elsewhere
    //             // unless our leader is stopped or we are approaching a roundabout
    // #ifdef DEBUG_STRATEGIC_CHANGE
    //             if (gDebugFlag2) {
    //                 std::cout << " veh=" << myVehicle.getID() << " does not want to leave the bestLane (neighDist=" << neighDist << ")\n";
    //             }
    // #endif
    //             ret |= LCA_STAY | LCA_STRATEGIC;
    //         } else if (right
    //                    && bestLaneOffset == 0
    //                    && myVehicle.getLane()->getSpeedLimit() > 80. / 3.6
    //                    && myLookAheadSpeed > SUMO_const_haltingSpeed
    //                   ) {
    //             // let's also regard the case where the vehicle is driving on a highway...
    //             //  in this case, we do not want to get to the dead-end of an on-ramp
    // #ifdef DEBUG_STRATEGIC_CHANGE
    //             if (gDebugFlag2) {
    //                 std::cout << " veh=" << myVehicle.getID() << " does not want to get stranded on the on-ramp of a highway\n";
    //             }
    // #endif
    //             ret |= LCA_STAY | LCA_STRATEGIC;
    //         }
    //     }
    //     if (laneOffset != 0 && (ret & LCA_URGENT) == 0 && getShadowLane() != nullptr &&
    //             // ignore overlap if it goes in the correct direction
    //             bestLaneOffset * myVehicle.getLateralPositionOnLane() <= 0) {
    //         // no decision or decision to stay
    //         // make sure to stay within lane bounds in case the shadow lane ends
    //         //const double requiredDist = MAX2(2 * myVehicle.getLateralOverlap(), getSublaneWidth()) / SUMO_const_laneWidth * laDist;
    //         const double requiredDist = 2 * overlap / SUMO_const_laneWidth * laDist;
    //         double currentShadowDist = -myVehicle.getPositionOnLane();
    //         MSLane* shadowPrev = nullptr;
    //         for (std::vector<MSLane*>::const_iterator it = curr.bestContinuations.begin(); it != curr.bestContinuations.end(); ++it) {
    //             if (*it == nullptr) {
    //                 continue;
    //             }
    //             MSLane* shadow = getShadowLane(*it);
    //             if (shadow == nullptr || currentShadowDist >= requiredDist) {
    //                 break;
    //             }
    //             if (shadowPrev != nullptr) {
    //                 currentShadowDist += shadowPrev->getEdge().getInternalFollowingLengthTo(&shadow->getEdge(), myVehicle.getVClass());
    //             }
    //             currentShadowDist += shadow->getLength();
    //             shadowPrev = shadow;
    // #ifdef DEBUG_STRATEGIC_CHANGE
    //             if (gDebugFlag2) {
    //                 std::cout << "    shadow=" << shadow->getID() << " currentShadowDist=" << currentShadowDist << "\n";
    //             }
    // #endif
    //         }
    // #ifdef DEBUG_STRATEGIC_CHANGE
    //         if (gDebugFlag2) {
    //             std::cout << " veh=" << myVehicle.getID() << " currentShadowDist=" << currentShadowDist << " requiredDist=" << requiredDist << " overlap=" << overlap << "\n";
    //         }
    // #endif
    //         if (currentShadowDist < requiredDist && currentShadowDist < usableDist) {
    //             myLeftSpace = currentShadowDist;
    //             latDist = myVehicle.getLateralPositionOnLane() < 0 ? overlap : -overlap;
    // #ifdef DEBUG_STRATEGIC_CHANGE
    //             if (gDebugFlag2) {
    //                 std::cout << "    must change for shadowLane end latDist=" << latDist << " myLeftSpace=" << myLeftSpace << "\n";
    //             }
    // #endif
    //             ret |= LCA_STRATEGIC | LCA_URGENT | LCA_STAY ;
    //         }
    //     }
    //
    //     // check for overriding TraCI requests
    // #if defined(DEBUG_STRATEGIC_CHANGE) || defined(DEBUG_TRACI)
    //     if (gDebugFlag2) {
    //         std::cout << SIMTIME << " veh=" << myVehicle.getID() << " ret=" << ret;
    //     }
    // #endif
    //     // store state before canceling
    //     getCanceledState(laneOffset) |= ret;
    //     int retTraCI = myVehicle.influenceChangeDecision(ret);
    //     if ((retTraCI & LCA_TRACI) != 0) {
    //         if ((retTraCI & LCA_STAY) != 0) {
    //             ret = retTraCI;
    //             latDist = 0;
    //         } else if (((retTraCI & LCA_RIGHT) != 0 && laneOffset < 0)
    //                    || ((retTraCI & LCA_LEFT) != 0 && laneOffset > 0)) {
    //             ret = retTraCI;
    //             latDist = latLaneDist;
    //         }
    //     }
    // #if defined(DEBUG_STRATEGIC_CHANGE) || defined(DEBUG_TRACI)
    //     if (gDebugFlag2) {
    //         std::cout << " reqAfterInfluence=" << toString((LaneChangeAction)retTraCI) << " ret=" << toString((LaneChangeAction)ret) << "\n";
    //     }
    // #endif
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn check_strategic_change(&mut self, ret: i32, neighLane: &MSLane, laneOffset: i32, leaders: &MSLeaderDistanceInfo, neighLeaders: &MSLeaderDistanceInfo, curr: &MSVehicle_LaneQ, neigh: &MSVehicle_LaneQ, best: &MSVehicle_LaneQ, bestLaneOffset: i32, changeToBest: bool, currentDist: &mut f64, neighDist: f64, laDist: f64, roundaboutBonus: f64, latLaneDist: f64, checkOpposite: bool, latDist: &mut f64) -> i32 {
        todo!("port MSLCM_SL2015::checkStrategicChange")
    }

    /// C++: `bool MSLCM_SL2015::mustOvertakeStopped(bool, const MSLane &, const MSLeaderDistanceInfo &, const MSLeaderDistanceInfo &, double, double, bool, double, const std::vector<MSLane *> &, double &, double &)` [protected]
    // bool
    // MSLCM_SL2015::mustOvertakeStopped(bool checkCurrent, const MSLane& neighLane, const MSLeaderDistanceInfo& leaders, const MSLeaderDistanceInfo& neighLead,
    //                                   double posOnLane, double neighDist, bool right, double latLaneDist, const std::vector<MSLane*>& conts, double& currentDist, double& latDist) {
    //     bool mustOvertake = false;
    //     const MSVehicle* const stoppedLeader = getStopped(leaders);
    //     const bool checkOverTakeRight = avoidOvertakeRight(stoppedLeader, true);
    //     int rightmost;
    //     int leftmost;
    //     const bool curHasStopped = stoppedLeader != nullptr;
    //     const int dir = latLaneDist < 0 ? -1 : 1;
    //     const MSLane* neighBeyond = neighLane.getParallelLane(dir);
    //     const bool hasLaneBeyond = checkCurrent && neighBeyond != nullptr && neighBeyond->allowsVehicleClass(myVehicle.getVClass());
    //     UNUSED_PARAMETER(hasLaneBeyond);
    //     if (curHasStopped || hasBidiLeader(leaders, conts)) {
    //         leaders.getSubLanes(&myVehicle, 0, rightmost, leftmost);
    //         for (int i = rightmost; i <= leftmost; i++) {
    //             const CLeaderDist& leader = leaders[i];
    //             if (leader.first != 0 && (leader.first->isStopped() || MSLCHelper::isBidiLeader(leader.first, conts)) && leader.second < REACT_TO_STOPPED_DISTANCE) {
    //                 const double overtakeDist = leader.second + myVehicle.getVehicleType().getLength() + leader.first->getVehicleType().getLengthWithGap();
    //                 const double remaining = MIN2(neighDist, currentDist) - posOnLane;
    // #ifdef DEBUG_STRATEGIC_CHANGE
    //                 if (DEBUG_COND) {
    //                     std::cout << "  overtakeDist=" << overtakeDist << " remaining=" << remaining
    //                               << " minDistToStopped=" << neighLead.getMinDistToStopped()
    //                               << " hasLaneBeyond=" << hasLaneBeyond
    //                               << "\n";
    //                 }
    // #endif
    //                 if (// current destination leaves enough space to overtake the leader
    //                     remaining > overtakeDist
    //                     // maybe do not overtake on the right at high speed
    //                     && (!checkCurrent || !checkOverTakeRight || !right)
    //                     && (!neighLead.hasStoppedVehicle() || neighLead.getMinDistToStopped() > overtakeDist /*|| (hasLaneBeyond && hasFreeLaneBeyond(neighBeyond, dir))*/)
    //                     //&& (neighLead.first == 0 || !neighLead.first->isStopped()
    //                     //    // neighboring stopped vehicle leaves enough space to overtake leader
    //                     //    || neighLead.second > overtakeDist))
    //                 ) {
    //                     // avoid becoming stuck behind a stopped leader
    //                     currentDist = myVehicle.getPositionOnLane() + leader.second;
    //                     myLeftSpace = currentDist - posOnLane;
    //                     latDist = latLaneDist;
    //                     mustOvertake = true;
    // #ifdef DEBUG_STRATEGIC_CHANGE
    //                     if (DEBUG_COND) {
    //                         std::cout << " veh=" << myVehicle.getID()
    //                                   << " overtake " << (leader.first->isStopped() ? "stopped" : "bidi") << " leader=" << leader.first->getID()
    //                                   << " newCurrentDist=" << currentDist
    //                                   << " overtakeDist=" << overtakeDist
    //                                   << " remaining=" << remaining
    //                                   << "\n";
    //                     }
    // #endif
    //                 }
    //             }
    //
    //         }
    //     }
    //     if (!mustOvertake && !curHasStopped && neighLead.hasStoppedVehicle()) {
    //         // #todo fix this if the neigh lane has a different width
    //         const double offset = (latLaneDist < 0 ? -1 : 1) * myVehicle.getLane()->getWidth();
    //         neighLead.getSubLanes(&myVehicle, offset, rightmost, leftmost);
    //         for (int i = 0; i < leaders.numSublanes(); i++) {
    //             const CLeaderDist& leader = leaders[i];
    //             if (leader.first != 0 && leader.first->isStopped() && leader.second < REACT_TO_STOPPED_DISTANCE) {
    //                 mustOvertake = true;
    //                 if (i >= rightmost && i <= leftmost) {
    //                     latDist = myVehicle.getLateralOverlap() * (latLaneDist > 0 ? -1 : 1);
    //                     break;
    //                 }
    //             }
    //         }
    //     }
    //     return mustOvertake;
    // }
    #[allow(unused_variables)]
    pub fn must_overtake_stopped(&mut self, checkCurrent: bool, neighLane: &MSLane, leaders: &MSLeaderDistanceInfo, neighLead: &MSLeaderDistanceInfo, posOnLane: f64, neighDist: f64, right: bool, latLaneDist: f64, conts: &Vec<Ptr<MSLane>>, currentDist: &mut f64, latDist: &mut f64) -> bool {
        todo!("port MSLCM_SL2015::mustOvertakeStopped")
    }

    /// C++: `int MSLCM_SL2015::keepLatGap(int, const MSLeaderDistanceInfo &, const MSLeaderDistanceInfo &, const MSLeaderDistanceInfo &, const MSLeaderDistanceInfo &, const MSLeaderDistanceInfo &, const MSLeaderDistanceInfo &, const MSLane &, int, double &, double &, int &)` [protected]
    /// check whether lateral gap requirements are met override the current maneuver if necessary
    // int
    // MSLCM_SL2015::keepLatGap(int state,
    //                          const MSLeaderDistanceInfo& leaders,
    //                          const MSLeaderDistanceInfo& followers,
    //                          const MSLeaderDistanceInfo& blockers,
    //                          const MSLeaderDistanceInfo& neighLeaders,
    //                          const MSLeaderDistanceInfo& neighFollowers,
    //                          const MSLeaderDistanceInfo& neighBlockers,
    //                          const MSLane& neighLane,
    //                          int laneOffset,
    //                          double& latDist,
    //                          double& maneuverDist,
    //                          int& blocked) {
    //
    //     /* @notes
    //      * vehicles may need to compromise between fulfilling lane change objectives
    //      * (LCA_STRATEGIC, LCA_SPEED etc) and maintaining lateral gap. The minimum
    //      * acceptable lateral gap depends on
    //      * - the cultural context (China vs Europe)
    //      * - the driver agressiveness (willingness to encroach on other vehicles to force them to move laterally as well)
    //      *    - see @note in checkBlocking
    //      * - the vehicle type (car vs motorcycle)
    //      * - the current speed
    //      * - the speed difference
    //      * - the importance / urgency of the desired maneuver
    //      *
    //      * the object of this method is to evaluate the above circumstances and
    //      * either:
    //      * - allow the current maneuver (state, latDist)
    //      * - to override the current maneuver with a distance-keeping maneuver
    //      *
    //      *
    //      * laneChangeModel/driver parameters
    //      * - bool pushy (willingness to encroach)
    //      * - float minGap at 100km/h (to be interpolated for lower speeds (assume 0 at speed 0)
    //      * - gapFactors (a factor for each of the change reasons
    //      *
    //      * further assumptions
    //      * - the maximum of egoSpeed and deltaSpeed can be used when interpolating minGap
    //      * - distance keeping to the edges of the road can be ignored (for now)
    //      *
    //      * currentMinGap = minGap * min(1.0, max(v, abs(v - vOther)) / 100) * gapFactor[lc_reason]
    //      *
    //      * */
    //
    //     /// XXX to be made configurable
    //     double gapFactor = computeGapFactor(state);
    //     const double oldLatDist = latDist;
    //     const double oldManeuverDist = maneuverDist;
    //     /// passed state is without traci-influence but we need it here
    //     const int traciState = myVehicle.influenceChangeDecision(state);
    //
    //     // compute gaps after maneuver
    //     const double halfWidth = getWidth() * 0.5;
    //     // if the current maneuver is blocked we will stay where we are
    //     const double oldCenter = myVehicle.getCenterOnEdge();
    //     // surplus gaps. these are used to collect various constraints
    //     // if they do not permit the desired maneuvre, should override it to better maintain distance
    //     // stay within the current edge
    //     double surplusGapRight = oldCenter - halfWidth;
    //     double surplusGapLeft = getLeftBorder(laneOffset != 0) - oldCenter - halfWidth;
    //     const bool stayInLane = (laneOffset == 0
    //                              || ((traciState & LCA_STRATEGIC) != 0
    //                                  && (traciState & LCA_STAY) != 0
    //                                  // permit wide vehicles to stay on the road
    //                                  && (surplusGapLeft >= 0 && surplusGapRight >= 0)));
    //
    //     if (isOpposite()) {
    //         std::swap(surplusGapLeft, surplusGapRight);
    //     }
    // #ifdef DEBUG_KEEP_LATGAP
    //     if (gDebugFlag2) {
    //         std::cout << "\n  " << SIMTIME << " keepLatGap() laneOffset=" << laneOffset
    //                   << " latDist=" << latDist
    //                   << " maneuverDist=" << maneuverDist
    //                   << " state=" << toString((LaneChangeAction)state)
    //                   << " traciState=" << toString((LaneChangeAction)traciState)
    //                   << " blocked=" << toString((LaneChangeAction)blocked)
    //                   << " gapFactor=" << gapFactor
    //                   << " stayInLane=" << stayInLane << "\n"
    //                   << "       stayInEdge: surplusGapRight=" << surplusGapRight << " surplusGapLeft=" << surplusGapLeft << "\n";
    //     }
    // #endif
    //     // staying within the edge overrides all minGap considerations
    //     if (surplusGapLeft < 0 || surplusGapRight < 0) {
    //         gapFactor = 0;
    //     }
    //
    //     // maintain gaps to vehicles on the current lane
    //     // ignore vehicles that are too far behind
    //     const double netOverlap = -myVehicle.getVehicleType().getLength() * 0.5;
    //     updateGaps(leaders, myVehicle.getLane()->getRightSideOnEdge(), oldCenter, gapFactor, surplusGapRight, surplusGapLeft, true);
    //     updateGaps(followers, myVehicle.getLane()->getRightSideOnEdge(), oldCenter, gapFactor, surplusGapRight, surplusGapLeft, true, netOverlap);
    //
    //     if (laneOffset != 0) {
    //         // maintain gaps to vehicles on the target lane
    //         const double neighRight = getNeighRight(neighLane);
    //         updateGaps(neighLeaders, neighRight, oldCenter, gapFactor, surplusGapRight, surplusGapLeft, true);
    //         updateGaps(neighFollowers, neighRight, oldCenter, gapFactor, surplusGapRight, surplusGapLeft, true, netOverlap);
    //     }
    // #ifdef DEBUG_KEEP_LATGAP
    //     if (gDebugFlag2) {
    //         std::cout << "       minGapLat: surplusGapRight=" << surplusGapRight << " surplusGapLeft=" << surplusGapLeft << "\n"
    //                   << "       lastGaps: right=" << myLastLateralGapRight << " left=" << myLastLateralGapLeft << "\n";
    //     }
    // #endif
    //     // we also need to track the physical gap, in addition to the psychological gap
    //     double physicalGapLeft = myLastLateralGapLeft == NO_NEIGHBOR ? surplusGapLeft : myLastLateralGapLeft;
    //     double physicalGapRight = myLastLateralGapRight == NO_NEIGHBOR ? surplusGapRight : myLastLateralGapRight;
    //
    //     const double halfLaneWidth = myVehicle.getLane()->getWidth() * 0.5;
    //     const double posLat = myVehicle.getLateralPositionOnLane() * (isOpposite() ? -1 : 1);
    //     if (stayInLane || laneOffset == 1) {
    //         // do not move past the right boundary of the current lane (traffic wasn't checked there)
    //         // but assume it's ok to be where we are in case we are already beyond
    //         surplusGapRight  = MIN2(surplusGapRight,  MAX2(0.0, halfLaneWidth + posLat - halfWidth));
    //         physicalGapRight = MIN2(physicalGapRight, MAX2(0.0, halfLaneWidth + posLat - halfWidth));
    //     }
    //     if (stayInLane || laneOffset == -1) {
    //         // do not move past the left boundary of the current lane (traffic wasn't checked there)
    //         // but assume it's ok to be where we are in case we are already beyond
    //         surplusGapLeft  = MIN2(surplusGapLeft,  MAX2(0.0, halfLaneWidth - posLat - halfWidth));
    //         physicalGapLeft = MIN2(physicalGapLeft, MAX2(0.0, halfLaneWidth - posLat - halfWidth));
    //     }
    // #ifdef DEBUG_KEEP_LATGAP
    //     if (gDebugFlag2) {
    //         std::cout << "       stayInLane: surplusGapRight=" << surplusGapRight << " surplusGapLeft=" << surplusGapLeft << "\n";
    //     }
    // #endif
    //
    //     if (surplusGapRight + surplusGapLeft < 0) {
    //         // insufficient lateral space to fulfill all requirements. apportion space proportionally
    //         if ((state & LCA_CHANGE_REASONS) == 0) {
    //             state |= LCA_SUBLANE;
    //         }
    //         const double equalDeficit = 0.5 * (surplusGapLeft + surplusGapRight);
    //         if (surplusGapRight < surplusGapLeft) {
    //             // shift further to the left but no further than there is physical space
    //             const double delta = MIN2(equalDeficit - surplusGapRight, physicalGapLeft);
    //             latDist = delta;
    //             maneuverDist = delta;
    // #ifdef DEBUG_KEEP_LATGAP
    //             if (gDebugFlag2) {
    //                 std::cout << "    insufficient latSpace, move left: delta=" << delta << "\n";
    //             }
    // #endif
    //         } else {
    //             // shift further to the right but no further than there is physical space
    //             const double delta = MIN2(equalDeficit - surplusGapLeft, physicalGapRight);
    //             latDist = -delta;
    //             maneuverDist = -delta;
    // #ifdef DEBUG_KEEP_LATGAP
    //             if (gDebugFlag2) {
    //                 std::cout << "    insufficient latSpace, move right: delta=" << delta << "\n";
    //             }
    // #endif
    //         }
    //     } else {
    //         // sufficient space. move as far as the gaps permit
    //         latDist = MAX2(MIN2(latDist, surplusGapLeft), -surplusGapRight);
    //         maneuverDist = MAX2(MIN2(maneuverDist, surplusGapLeft), -surplusGapRight);
    //         if ((state & LCA_KEEPRIGHT) != 0 && maneuverDist != oldManeuverDist) {
    //             // don't start keepRight unless it can be completed
    //             latDist = oldLatDist;
    //             maneuverDist = oldManeuverDist;
    //         }
    // #ifdef DEBUG_KEEP_LATGAP
    //         if (gDebugFlag2) {
    //             std::cout << "     adapted latDist=" << latDist << " maneuverDist=" << maneuverDist << " (old=" << oldLatDist << ")\n";
    //         }
    // #endif
    //     }
    //     // take into account overriding traci sublane-request
    //     if (myVehicle.hasInfluencer() && myVehicle.getInfluencer().getLatDist() != 0) {
    //         // @note: the influence is reset in MSAbstractLaneChangeModel::setOwnState at the end of the lane-changing code for this vehicle
    //         latDist = myVehicle.getInfluencer().getLatDist();
    //         maneuverDist = myVehicle.getInfluencer().getLatDist();
    //         if (latDist < 0) {
    //             mySafeLatDistRight = MAX2(-latDist, mySafeLatDistRight);
    //         } else {
    //             mySafeLatDistLeft = MAX2(latDist, mySafeLatDistLeft);
    //         }
    //         state |= LCA_TRACI;
    // #ifdef DEBUG_KEEP_LATGAP
    //         if (gDebugFlag2) {
    //             std::cout << "     traci influenced latDist=" << latDist << "\n";
    //         }
    // #endif
    //     }
    //     // if we cannot move in the desired direction, consider the maneuver blocked anyway
    //     const bool nonSublaneChange = (state & (LCA_STRATEGIC | LCA_COOPERATIVE | LCA_SPEEDGAIN | LCA_KEEPRIGHT)) != 0;
    //     const bool traciChange = ((state | traciState) & LCA_TRACI) != 0;
    //     if (nonSublaneChange && !traciChange) {
    //         if ((latDist < NUMERICAL_EPS * myVehicle.getActionStepLengthSecs()) && (oldLatDist > 0)) {
    // #ifdef DEBUG_KEEP_LATGAP
    //             if (gDebugFlag2) {
    //                 std::cout << "     wanted changeToLeft oldLatDist=" << oldLatDist << ", blocked latGap changeToRight\n";
    //             }
    // #endif
    //             latDist = oldLatDist; // restore old request for usage in decideDirection()
    //             blocked = LCA_OVERLAPPING | LCA_BLOCKED_LEFT;
    //         } else if ((latDist > -NUMERICAL_EPS * myVehicle.getActionStepLengthSecs()) && (oldLatDist < 0)) {
    // #ifdef DEBUG_KEEP_LATGAP
    //             if (gDebugFlag2) {
    //                 std::cout << "     wanted changeToRight oldLatDist=" << oldLatDist << ", blocked latGap changeToLeft\n";
    //             }
    // #endif
    //             latDist = oldLatDist; // restore old request for usage in decideDirection()
    //             blocked = LCA_OVERLAPPING | LCA_BLOCKED_RIGHT;
    //         }
    //     }
    //     // if we move, even though we wish to stay, update the change reason (except for TraCI)
    //     if (fabs(latDist) > NUMERICAL_EPS * myVehicle.getActionStepLengthSecs() && oldLatDist == 0) {
    //         state &= (~(LCA_CHANGE_REASONS | LCA_STAY) | LCA_TRACI);
    //     }
    //     // update blocked status
    //     if (fabs(latDist - oldLatDist) > NUMERICAL_EPS * myVehicle.getActionStepLengthSecs()) {
    // #ifdef DEBUG_KEEP_LATGAP
    //         if (gDebugFlag2) {
    //             std::cout << "     latDistUpdated=" << latDist << " oldLatDist=" << oldLatDist << "\n";
    //         }
    // #endif
    //         blocked = checkBlocking(neighLane, latDist, maneuverDist, laneOffset, leaders, followers, blockers, neighLeaders, neighFollowers, neighBlockers, nullptr, nullptr, nonSublaneChange);
    //     }
    //     if (fabs(latDist) > NUMERICAL_EPS * myVehicle.getActionStepLengthSecs()) {
    //         state = (state & ~LCA_STAY);
    //         if ((state & LCA_CHANGE_REASONS) == 0) {
    //             state |= LCA_SUBLANE;
    //         }
    //     } else {
    //         if ((state & LCA_SUBLANE) != 0) {
    //             state |= LCA_STAY;
    //         }
    //         // avoid setting blinker due to numerical issues
    //         latDist = 0;
    //     }
    // #if defined(DEBUG_KEEP_LATGAP) || defined(DEBUG_STATE)
    //     if (gDebugFlag2) {
    //         std::cout << "       latDist2=" << latDist
    //                   << " state2=" << toString((LaneChangeAction)state)
    //                   << " lastGapLeft=" << myLastLateralGapLeft
    //                   << " lastGapRight=" << myLastLateralGapRight
    //                   << " blockedAfter=" << toString((LaneChangeAction)blocked)
    //                   << "\n";
    //     }
    // #endif
    //     return state;
    // }
    #[allow(unused_variables)]
    pub fn keep_lat_gap(&mut self, state: i32, leaders: &MSLeaderDistanceInfo, followers: &MSLeaderDistanceInfo, blockers: &MSLeaderDistanceInfo, neighLeaders: &MSLeaderDistanceInfo, neighFollowers: &MSLeaderDistanceInfo, neighBlockers: &MSLeaderDistanceInfo, neighLane: &MSLane, laneOffset: i32, latDist: &mut f64, maneuverDist: &mut f64, blocked: &mut i32) -> i32 {
        todo!("port MSLCM_SL2015::keepLatGap")
    }

    /// C++: `void MSLCM_SL2015::updateGaps(const MSLeaderDistanceInfo &, double, double, double, double &, double &, bool, double, double, std::vector<CLeaderDist> *)` [protected]
    /// check remaining lateral gaps for the given foe vehicles and optionally update minimum lateral gaps
    // void
    // MSLCM_SL2015::updateGaps(const MSLeaderDistanceInfo& others, double foeOffset, double oldCenter, double gapFactor,
    //                          double& surplusGapRight, double& surplusGapLeft,
    //                          bool saveMinGap, double netOverlap,
    //                          double latDist,
    //                          std::vector<CLeaderDist>* collectBlockers) {
    //     if (others.hasVehicles()) {
    //         const double halfWidth = getWidth() * 0.5 + NUMERICAL_EPS;
    //         const double baseMinGap = myMinGapLat;
    //         for (int i = 0; i < others.numSublanes(); ++i) {
    //             if (others[i].first != 0 && others[i].second <= 0
    //                     && myCFRelated.count(others[i].first) == 0
    //                     && (netOverlap == 0 || others[i].second + others[i].first->getVehicleType().getMinGap() < netOverlap)) {
    //                 /// foe vehicle occupies full sublanes
    //                 const MSVehicle* foe = others[i].first;
    //                 const double res = MSGlobals::gLateralResolution > 0 ? MSGlobals::gLateralResolution : others[i].first->getLane()->getWidth();
    //                 double foeRight, foeLeft;
    //                 others.getSublaneBorders(i, foeOffset, foeRight, foeLeft);
    //                 const double foeCenter = foeRight + 0.5 * res;
    //                 const double gap = MIN2(fabs(foeRight - oldCenter), fabs(foeLeft - oldCenter)) - halfWidth;
    //                 const double deltaV = MIN2(LATGAP_SPEED_THRESHOLD, MAX3(LATGAP_SPEED_THRESHOLD2, myVehicle.getSpeed(), fabs(myVehicle.getSpeed() - foe->getSpeed())));
    //                 const double desiredMinGap = baseMinGap * deltaV / LATGAP_SPEED_THRESHOLD;
    //                 const double currentMinGap = desiredMinGap * gapFactor; // pushy vehicles may accept a lower lateral gap temporarily
    //                 /*
    //                 if (netOverlap != 0) {
    //                     // foe vehicle is follower with its front ahead of the ego midpoint
    //                     // scale gap requirements so it gets lower for foe which are further behind ego
    //                     //
    //                     // relOverlap approaches 0 as the foe gets closer to the midpoint and it equals 1 if the foe is driving head-to-head
    //                     const double relOverlap = 1 - (others[i].second + others[i].first->getVehicleType().getMinGap()) / netOverlap;
    //                     currentMinGap *= currOverlap * relOverlap;
    //                 }
    //                 */
    // #if defined(DEBUG_BLOCKING) || defined(DEBUG_KEEP_LATGAP)
    //                 if (debugVehicle()) {
    //                     std::cout << "  updateGaps"
    //                               << " i=" << i
    //                               << " foe=" << foe->getID()
    //                               << " foeRight=" << foeRight
    //                               << " foeLeft=" << foeLeft
    //                               << " oldCenter=" << oldCenter
    //                               << " gap=" << others[i].second
    //                               << " latgap=" << gap
    //                               << " currentMinGap=" << currentMinGap
    //                               << " surplusGapRight=" << surplusGapRight
    //                               << " surplusGapLeft=" << surplusGapLeft
    //                               << "\n";
    //                 }
    // #endif
    //
    //                 // If foe is maneuvering towards ego, reserve some additional distance.
    //                 // But don't expect the foe to come closer than currentMinGap if it isn't already there.
    //                 //   (XXX: How can the ego know the foe's maneuver dist?)
    //                 if (foeCenter < oldCenter) { // && foe->getLaneChangeModel().getSpeedLat() > 0) {
    //                     const double foeManeuverDist = MAX2(0., foe->getLaneChangeModel().getManeuverDist());
    //                     surplusGapRight = MIN3(surplusGapRight, gap - currentMinGap, MAX2(currentMinGap, gap - foeManeuverDist));
    //                 } else { //if (foeCenter > oldCenter && foe->getLaneChangeModel().getSpeedLat() < 0) {
    //                     const double foeManeuverDist = -MIN2(0., foe->getLaneChangeModel().getManeuverDist());
    //                     surplusGapLeft = MIN3(surplusGapLeft, gap - currentMinGap, MAX2(currentMinGap, gap - foeManeuverDist));
    //                 }
    //                 if (saveMinGap) {
    //                     if (foeCenter < oldCenter) {
    // #if defined(DEBUG_BLOCKING) || defined(DEBUG_KEEP_LATGAP)
    //                         if (gDebugFlag2 && gap < myLastLateralGapRight) {
    //                             std::cout << "    new minimum rightGap=" << gap << "\n";
    //                         }
    // #endif
    //                         myLastLateralGapRight = MIN2(myLastLateralGapRight, gap);
    //                     } else {
    // #if defined(DEBUG_BLOCKING) || defined(DEBUG_KEEP_LATGAP)
    //                         if (gDebugFlag2 && gap < myLastLateralGapLeft) {
    //                             std::cout << "    new minimum leftGap=" << gap << "\n";
    //                         }
    // #endif
    //                         myLastLateralGapLeft = MIN2(myLastLateralGapLeft, gap);
    //                     }
    //                 }
    //                 if (collectBlockers != nullptr) {
    //                     // check if the vehicle is blocking a desire lane change
    //                     if ((foeCenter < oldCenter && latDist < 0 && gap < (desiredMinGap - latDist))
    //                             || (foeCenter > oldCenter && latDist > 0 && gap < (desiredMinGap + latDist))) {
    //                         collectBlockers->push_back(others[i]);
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn update_gaps(&mut self, others: &MSLeaderDistanceInfo, foeOffset: f64, oldCenter: f64, gapFactor: f64, surplusGapRight: &mut f64, surplusGapLeft: &mut f64, saveMinGap: bool, netOverlap: f64, latDist: f64, collectBlockers: Ptr<Vec<CLeaderDist>>) {
        todo!("port MSLCM_SL2015::updateGaps")
    }

    /// C++: `double MSLCM_SL2015::computeGapFactor(int) const` [const, protected]
    /// compute the gap factor for the given state
    // double
    // MSLCM_SL2015::computeGapFactor(int state) const {
    //     return (state & LCA_STRATEGIC) != 0 ? MAX2(0.0, (1.0 - myPushy * (1 + 0.5 * myImpatience))) : 1.0;
    // }
    #[allow(unused_variables)]
    pub fn compute_gap_factor(&self, state: i32) -> f64 {
        todo!("port MSLCM_SL2015::computeGapFactor")
    }

    /// C++: `double MSLCM_SL2015::getWidth() const` [const, protected]
    /// return the width of this vehicle (padded for numerical stability)
    // double
    // MSLCM_SL2015::getWidth() const {
    //     return myVehicle.getVehicleType().getWidth() + NUMERICAL_EPS;
    // }
    #[allow(unused_variables)]
    pub fn get_width(&self) -> f64 {
        todo!("port MSLCM_SL2015::getWidth")
    }

    /// C++: `void MSLCM_SL2015::updateCFRelated(const MSLeaderDistanceInfo &, double, bool)` [protected]
    /// find leaders/followers that are already in a car-following relationship with ego
    // void
    // MSLCM_SL2015::updateCFRelated(const MSLeaderDistanceInfo& vehicles, double foeOffset, bool leaders) {
    //     // to ensure that we do not ignore the wrong vehicles due to numerical
    //     // instability we slightly reduce the width
    //     const double vehWidth = myVehicle.getVehicleType().getWidth() - NUMERICAL_EPS;
    //     const double rightVehSide = myVehicle.getCenterOnEdge() - 0.5 * vehWidth;
    //     const double leftVehSide = rightVehSide + vehWidth;
    // #ifdef DEBUG_BLOCKING
    //     if (gDebugFlag2) {
    //         std::cout << " updateCFRelated foeOffset=" << foeOffset << " vehicles=" << vehicles.toString() << "\n";
    //     }
    // #endif
    //     for (int i = 0; i < vehicles.numSublanes(); ++i) {
    //         CLeaderDist vehDist = vehicles[i];
    //         if (vehDist.first != 0 && (myCFRelated.count(vehDist.first) == 0 || vehDist.second < 0)) {
    //             double foeRight, foeLeft;
    //             vehicles.getSublaneBorders(i, foeOffset, foeRight, foeLeft);
    // #ifdef DEBUG_BLOCKING
    //             if (gDebugFlag2) {
    //                 std::cout << "    foe=" << vehDist.first->getID() << " gap=" << vehDist.second
    //                           << " sublane=" << i
    //                           << " foeOffset=" << foeOffset
    //                           << " egoR=" << rightVehSide << " egoL=" << leftVehSide
    //                           << " iR=" << foeRight << " iL=" << foeLeft
    //                           << " egoV=" << myVehicle.getSpeed() << " foeV=" << vehDist.first->getSpeed()
    //                           << " egoE=" << myVehicle.getLane()->getEdge().getID() << " foeE=" << vehDist.first->getLane()->getEdge().getID()
    //                           << "\n";
    //             }
    // #endif
    //             if (overlap(rightVehSide, leftVehSide, foeRight, foeLeft) && !outsideEdge()
    //                     // account for imprecise driving by some models
    //                     && (vehDist.second >= vehDist.first->getVehicleType().getMinGap() * (1 - vehDist.first->getCarFollowModel().getCollisionMinGapFactor())
    //                         || (vehDist.second < 0 && myVehicle.getLane()->isNormal() && vehDist.first->getLane()->isNormal()
    //                             && vehDist.second >= -myVehicle.getVehicleType().getMinGap() * (1 - myVehicle.getCarFollowModel().getCollisionMinGapFactor()))
    //                         // avoid deadlock due to #3729
    //                         || (!leaders
    //                             && myVehicle.getPositionOnLane() >= myVehicle.getVehicleType().getLength()
    //                             && myVehicle.getSpeed() < SUMO_const_haltingSpeed
    //                             && vehDist.first->getSpeed() < SUMO_const_haltingSpeed
    //                             && -vehDist.second < vehDist.first->getVehicleType().getMinGap()
    //                             && &(myVehicle.getLane()->getEdge()) != &(vehDist.first->getLane()->getEdge()))
    //                        )) {
    // #ifdef DEBUG_BLOCKING
    //                 if (gDebugFlag2) {
    //                     std::cout << "       ignoring cfrelated foe=" << vehDist.first->getID()  << "\n";
    //                 }
    // #endif
    //                 myCFRelated.insert(vehDist.first);
    //             } else {
    //                 const int erased = (int)myCFRelated.erase(vehDist.first);
    // #ifdef DEBUG_BLOCKING
    //                 if (gDebugFlag2 && erased > 0) {
    //                     std::cout << "       restoring cfrelated foe=" << vehDist.first->getID()  << "\n";
    //                 }
    // #else
    //                 UNUSED_PARAMETER(erased);
    // #endif
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn update_cf_related(&mut self, vehicles: &MSLeaderDistanceInfo, foeOffset: f64, leaders: bool) {
        todo!("port MSLCM_SL2015::updateCFRelated")
    }

    /// C++: `double MSLCM_SL2015::getSublaneWidth()` [protected]
    /// return the current sublane width (and return a sensible value when running without sublanes)
    // double getSublaneWidth() {
    //         return MSGlobals::gLateralResolution > 0 ? MSGlobals::gLateralResolution : myVehicle.getLane()->getWidth();
    //     }
    #[allow(unused_variables)]
    pub fn get_sublane_width(&mut self) -> f64 {
        todo!("port MSLCM_SL2015::getSublaneWidth")
    }

    /// C++: `void MSLCM_SL2015::commitManoeuvre(int, int, const MSLeaderDistanceInfo &, const MSLeaderDistanceInfo &, const MSLane &, double)` [protected]
    /// commit to lane change maneuver potentially overriding safe speed
    // void
    // MSLCM_SL2015::commitManoeuvre(int blocked, int blockedFully,
    //                               const MSLeaderDistanceInfo& leaders,
    //                               const MSLeaderDistanceInfo& neighLeaders,
    //                               const MSLane& neighLane,
    //                               double maneuverDist) {
    //     if (!blocked && !blockedFully && !myCanChangeFully) {
    //         // round to full action steps
    //         double secondsToLeaveLane;
    //         if (MSGlobals::gSemiImplicitEulerUpdate) {
    //             secondsToLeaveLane = ceil(fabs(maneuverDist) / myVehicle.getVehicleType().getMaxSpeedLat() / myVehicle.getActionStepLengthSecs()) * myVehicle.getActionStepLengthSecs();
    //             // XXX myAccelLat must be taken into account (refs #3601, see ballistic case for solution)
    //
    //             // XXX This also causes probs: if the difference between the current speed and the committed is higher than the maximal decel,
    //             //     the vehicle may pass myLeftSpace before completing the maneuver.
    //             myCommittedSpeed = MIN3(myLeftSpace / secondsToLeaveLane,
    //                                     myVehicle.getCarFollowModel().maxNextSpeed(myVehicle.getSpeed(), &myVehicle),
    //                                     myVehicle.getLane()->getVehicleMaxSpeed(&myVehicle));
    // #if defined(DEBUG_MANEUVER) || defined(DEBUG_COMMITTED_SPEED)
    //             if (debugVehicle()) {
    //                 std::cout << SIMTIME << " veh=" << myVehicle.getID() << " myCommittedSpeed=" << myCommittedSpeed << " leftSpace=" << myLeftSpace << " secondsToLeave=" << secondsToLeaveLane << "\n";
    //             }
    // #endif
    //         } else {
    //
    //             // Calculate seconds needed for leaving lane assuming start from lateral speed zero, and lat.accel == -lat.decel
    //             secondsToLeaveLane = MSCFModel::estimateArrivalTime(fabs(maneuverDist), 0., 0., myVehicle.getVehicleType().getMaxSpeedLat(), myAccelLat, myAccelLat);
    //             // round to full action steps
    //             secondsToLeaveLane = ceil(secondsToLeaveLane / myVehicle.getActionStepLengthSecs()) * myVehicle.getActionStepLengthSecs();
    //
    //             // committed speed will eventually be pushed into a drive item during the next planMove() step. This item
    //             // will not be read before the next action step at current time + actionStepLength-TS, so we need to schedule the corresponding speed.
    //             const double timeTillActionStep = myVehicle.getActionStepLengthSecs() - TS;
    //             const double nextActionStepSpeed = MAX2(0., myVehicle.getSpeed() + timeTillActionStep * myVehicle.getAcceleration());
    //             double nextLeftSpace;
    //             if (nextActionStepSpeed > 0.) {
    //                 nextLeftSpace = myLeftSpace - timeTillActionStep * (myVehicle.getSpeed() + nextActionStepSpeed) * 0.5;
    //             } else if (myVehicle.getAcceleration() == 0) {
    //                 nextLeftSpace = myLeftSpace;
    //             } else {
    //                 assert(myVehicle.getAcceleration() < 0.);
    //                 nextLeftSpace = myLeftSpace + (myVehicle.getSpeed() * myVehicle.getSpeed() / myVehicle.getAcceleration()) * 0.5;
    //             }
    //             const double avoidArrivalSpeed = nextActionStepSpeed + ACCEL2SPEED(MSCFModel::avoidArrivalAccel(
    //                                                  nextLeftSpace, secondsToLeaveLane - timeTillActionStep, nextActionStepSpeed, myVehicle.getCarFollowModel().getEmergencyDecel()));
    //
    //             myCommittedSpeed = MIN3(avoidArrivalSpeed,
    //                                     myVehicle.getSpeed() + myVehicle.getCarFollowModel().getMaxAccel() * myVehicle.getActionStepLengthSecs(),
    //                                     myVehicle.getLane()->getVehicleMaxSpeed(&myVehicle));
    //
    // #if defined(DEBUG_MANEUVER) || defined(DEBUG_COMMITTED_SPEED)
    //             if (gDebugFlag2) {
    //                 std::cout << SIMTIME
    //                           << " veh=" << myVehicle.getID()
    //                           << " avoidArrivalSpeed=" << avoidArrivalSpeed
    //                           << " currentSpeed=" << myVehicle.getSpeed()
    //                           << " myLeftSpace=" << myLeftSpace
    //                           << "\n             nextLeftSpace=" << nextLeftSpace
    //                           << " nextActionStepSpeed=" << nextActionStepSpeed
    //                           << " nextActionStepRemainingSeconds=" << secondsToLeaveLane - timeTillActionStep
    //                           << "\n";
    //             }
    // #endif
    //         }
    //         myCommittedSpeed = commitFollowSpeed(myCommittedSpeed, maneuverDist, secondsToLeaveLane, leaders, myVehicle.getLane()->getRightSideOnEdge());
    //         myCommittedSpeed = commitFollowSpeed(myCommittedSpeed, maneuverDist, secondsToLeaveLane, neighLeaders, neighLane.getRightSideOnEdge());
    //         if (myCommittedSpeed < myVehicle.getCarFollowModel().minNextSpeed(myVehicle.getSpeed(), &myVehicle)) {
    //             myCommittedSpeed = 0;
    //         }
    // #if defined(DEBUG_MANEUVER) || defined(DEBUG_COMMITTED_SPEED)
    //         if (gDebugFlag2) {
    //             std::cout << SIMTIME
    //                       << " veh=" << myVehicle.getID()
    //                       << " secondsToLeave=" << secondsToLeaveLane
    //                       << " maxNext=" << myVehicle.getCarFollowModel().maxNextSpeed(myVehicle.getSpeed(), &myVehicle)
    //                       << " committed=" << myCommittedSpeed
    //                       << "\n";
    //         }
    // #endif
    //     }
    // }
    #[allow(unused_variables)]
    pub fn commit_manoeuvre(&mut self, blocked: i32, blockedFully: i32, leaders: &MSLeaderDistanceInfo, neighLeaders: &MSLeaderDistanceInfo, neighLane: &MSLane, maneuverDist: f64) {
        todo!("port MSLCM_SL2015::commitManoeuvre")
    }

    /// C++: `double MSLCM_SL2015::commitFollowSpeed(double, double, double, const MSLeaderDistanceInfo &, double) const` [const, protected]
    /// compute speed when committing to an urgent change that is safe in regard to leading vehicles
    // double
    // MSLCM_SL2015::commitFollowSpeed(double speed, double latDist, double secondsToLeaveLane, const MSLeaderDistanceInfo& leaders, double foeOffset) const {
    //     if (leaders.hasVehicles()) {
    //         // we distinguish 3 cases
    //         // - vehicles with lateral overlap at the end of the maneuver: try to follow safely
    //         // - vehicles with overlap at the start of the maneuver: avoid collision within secondsToLeaveLane
    //         // - vehicles without overlap: ignore
    //
    //         const double maxDecel = myVehicle.getCarFollowModel().getMaxDecel();
    //         // temporarily use another decel value
    //         MSCFModel& cfmodel = const_cast<MSCFModel&>(myVehicle.getCarFollowModel());
    //         cfmodel.setMaxDecel(maxDecel / getSafetyFactor());
    //
    //         const double vehWidth = getWidth();
    //         const double rightVehSide = myVehicle.getCenterOnEdge() - 0.5 * vehWidth;
    //         const double leftVehSide = rightVehSide + vehWidth;
    //         const double rightVehSideDest = rightVehSide + latDist;
    //         const double leftVehSideDest = leftVehSide + latDist;
    // #if defined(DEBUG_MANEUVER) || defined(DEBUG_COMMITTED_SPEED)
    //         if (gDebugFlag2) {
    //             std::cout << "  commitFollowSpeed"
    //                       << " latDist=" << latDist
    //                       << " foeOffset=" << foeOffset
    //                       << " vehRight=" << rightVehSide
    //                       << " vehLeft=" << leftVehSide
    //                       << " destRight=" << rightVehSideDest
    //                       << " destLeft=" << leftVehSideDest
    //                       << "\n";
    //         }
    // #endif
    //         for (int i = 0; i < leaders.numSublanes(); ++i) {
    //             CLeaderDist vehDist = leaders[i];
    //             if (vehDist.first != 0) {
    //                 const MSVehicle* leader = vehDist.first;
    //                 // only check the current stripe occuped by foe (transform into edge-coordinates)
    //                 double foeRight, foeLeft;
    //                 leaders.getSublaneBorders(i, foeOffset, foeRight, foeLeft);
    // #if defined(DEBUG_MANEUVER) || defined(DEBUG_COMMITTED_SPEED)
    //                 if (gDebugFlag2) {
    //                     std::cout << "   foe=" << vehDist.first->getID()
    //                               << " gap=" << vehDist.second
    //                               << " secGap=" << myVehicle.getCarFollowModel().getSecureGap(&myVehicle, leader, myVehicle.getSpeed(), leader->getSpeed(), leader->getCarFollowModel().getMaxDecel())
    //                               << " foeRight=" << foeRight
    //                               << " foeLeft=" << foeLeft
    //                               << " overlapBefore=" << overlap(rightVehSide, leftVehSide, foeRight, foeLeft)
    //                               << " overlapDest=" << overlap(rightVehSideDest, leftVehSideDest, foeRight, foeLeft)
    //                               << "\n";
    //                 }
    // #endif
    //                 if (overlap(rightVehSideDest, leftVehSideDest, foeRight, foeLeft)) {
    //                     // case 1
    //                     const double vSafe = myVehicle.getCarFollowModel().followSpeed(
    //                                              &myVehicle, speed, vehDist.second, leader->getSpeed(), leader->getCarFollowModel().getMaxDecel());
    //                     speed = MIN2(speed, vSafe);
    // #if defined(DEBUG_MANEUVER) || defined(DEBUG_COMMITTED_SPEED)
    //                     if (gDebugFlag2) {
    //                         std::cout << "     case1 vsafe=" << vSafe << " speed=" << speed << "\n";
    //                     }
    // #endif
    //                 } else if (overlap(rightVehSide, leftVehSide, foeRight, foeLeft)) {
    //                     // case 2
    //                     const double vSafe = myVehicle.getCarFollowModel().followSpeedTransient(
    //                                              secondsToLeaveLane,
    //                                              &myVehicle, speed, vehDist.second, leader->getSpeed(), leader->getCarFollowModel().getMaxDecel());
    //                     speed = MIN2(speed, vSafe);
    // #if defined(DEBUG_MANEUVER) || defined(DEBUG_COMMITTED_SPEED)
    //                     if (gDebugFlag2) {
    //                         std::cout << "     case2 vsafe=" << vSafe << " speed=" << speed << "\n";
    //                     }
    // #endif
    //                 }
    //             }
    //         }
    //         // restore original deceleration
    //         cfmodel.setMaxDecel(maxDecel);
    //
    //     }
    //     return speed;
    // }
    #[allow(unused_variables)]
    pub fn commit_follow_speed(&self, speed: f64, latDist: f64, secondsToLeaveLane: f64, leaders: &MSLeaderDistanceInfo, foeOffset: f64) -> f64 {
        todo!("port MSLCM_SL2015::commitFollowSpeed")
    }

    /// C++: `bool MSLCM_SL2015::sublaneEnds(int, const MSLane *, double)` [protected]
    /// check whether the sublane continues on the next lane
    // bool
    // MSLCM_SL2015::sublaneEnds(int i, const MSLane* next, double shift) {
    //     const double side = i * MSGlobals::gLateralResolution + shift;
    //     return ((side < -NUMERICAL_EPS
    //              && (next->getParallelLane(-1) == nullptr || !next->getParallelLane(-1)->allowsVehicleClass(myVehicle.getVClass())))
    //             || (side + MSGlobals::gLateralResolution > next->getWidth()
    //                 && (next->getParallelLane(1) == nullptr || !next->getParallelLane(1)->allowsVehicleClass(myVehicle.getVClass()))));
    // }
    #[allow(unused_variables)]
    pub fn sublane_ends(&mut self, i: i32, next: Ptr<MSLane>, shift: f64) -> bool {
        todo!("port MSLCM_SL2015::sublaneEnds")
    }

    /// C++: `double MSLCM_SL2015::forecastAverageSpeed(double, double, double, double) const` [const, protected]
    /// estimate average speed over mySpeedGainLookahead time
    // double
    // MSLCM_SL2015::forecastAverageSpeed(double vSafe, double vMax, double gap, double vLeader) const {
    //     const double deltaV = vMax - vLeader;
    //     if (deltaV > 0 && gap / deltaV < mySpeedGainLookahead && mySpeedGainLookahead > 0) {
    //         // anticipate future braking by computing the average
    //         // speed over the next few seconds
    //         const double foreCastTime = mySpeedGainLookahead * 2;
    //         const double gapClosingTime = MAX2(0.0, gap / deltaV);
    //         const double vSafe2 = (gapClosingTime * vSafe + (foreCastTime - gapClosingTime) * vLeader) / foreCastTime;
    // #ifdef DEBUG_EXPECTED_SLSPEED
    //         if (DEBUG_COND && vSafe2 != vSafe) {
    //             std::cout << "     foreCastTime=" << foreCastTime << " gapClosingTime=" << gapClosingTime << " extrapolated vSafe=" << vSafe2 << "\n";
    //         }
    // #endif
    //         vSafe = vSafe2;
    //     }
    //     return vSafe;
    // }
    #[allow(unused_variables)]
    pub fn forecast_average_speed(&self, vSafe: f64, vMax: f64, gap: f64, vLeader: f64) -> f64 {
        todo!("port MSLCM_SL2015::forecastAverageSpeed")
    }

    /// C++: `double MSLCM_SL2015::computeSpeedGain(double, double) const` [const, protected]
    /// compute speedGain when moving by the given amount
    // double
    // MSLCM_SL2015::computeSpeedGain(double latDistSublane, double defaultNextSpeed) const {
    //     double result = std::numeric_limits<double>::max();
    //     const std::vector<double>& sublaneSides = myVehicle.getLane()->getEdge().getSubLaneSides();
    //     const double vehWidth = getWidth();
    //     const double rightVehSide = myVehicle.getCenterOnEdge() - vehWidth * 0.5 + latDistSublane;
    //     const double leftVehSide = rightVehSide + vehWidth;
    //     for (int i = 0; i < (int)sublaneSides.size(); ++i) {
    //         const double leftSide = i + 1 < (int)sublaneSides.size() ? sublaneSides[i + 1] : MAX2(myVehicle.getLane()->getEdge().getWidth(), sublaneSides[i] + POSITION_EPS);
    //         if (overlap(rightVehSide, leftVehSide, sublaneSides[i], leftSide)) {
    //             result = MIN2(result, myExpectedSublaneSpeeds[i]);
    //         }
    //         //std::cout << "    i=" << i << " rightVehSide=" << rightVehSide << " leftVehSide=" << leftVehSide << " sublaneR=" << sublaneSides[i] << " sublaneL=" << leftSide << " overlap=" << overlap(rightVehSide, leftVehSide, sublaneSides[i], leftSide) << " speed=" << myExpectedSublaneSpeeds[i] << " result=" << result << "\n";
    //     }
    //     return result - defaultNextSpeed;
    // }
    #[allow(unused_variables)]
    pub fn compute_speed_gain(&self, latDistSublane: f64, defaultNextSpeed: f64) -> f64 {
        todo!("port MSLCM_SL2015::computeSpeedGain")
    }

    /// C++: `double MSLCM_SL2015::getPosLat()` [protected]
    /// get lateral position of this vehicle
    // double
    // MSLCM_SL2015::getPosLat() {
    //     return myVehicle.getLateralPositionOnLane() + mySigmaState;
    // }
    #[allow(unused_variables)]
    pub fn get_pos_lat(&mut self) -> f64 {
        todo!("port MSLCM_SL2015::getPosLat")
    }

    /// C++: `double MSLCM_SL2015::getLateralDrift()` [protected]
    /// get lateral drift for the current step
    // double
    // MSLCM_SL2015::getLateralDrift() {
    //     //OUProcess::step(double state, double dt, double timeScale, double noiseIntensity)
    //     const double deltaState = OUProcess::step(mySigmaState,
    //                               myVehicle.getActionStepLengthSecs(),
    //                               MAX2(NUMERICAL_EPS, (1 - mySigma) * 100), mySigma) - mySigmaState;
    //     const double scaledDelta = deltaState * myVehicle.getSpeed() / myVehicle.getLane()->getSpeedLimit();
    //     return scaledDelta;
    // }
    #[allow(unused_variables)]
    pub fn get_lateral_drift(&mut self) -> f64 {
        todo!("port MSLCM_SL2015::getLateralDrift")
    }

    /// C++: `double MSLCM_SL2015::getLeftBorder(bool) const` [const, protected]
    /// return current edge width optionally extended by opposite direction lane width
    // double
    // MSLCM_SL2015::getLeftBorder(bool checkOpposite) const {
    //     return (myVehicle.getLane()->getEdge().getWidth()
    //             + ((myVehicle.getLane()->getParallelOpposite() != nullptr && checkOpposite) ? myVehicle.getLane()->getParallelOpposite()->getEdge().getWidth() : 0));
    // }
    #[allow(unused_variables)]
    pub fn get_left_border(&self, checkOpposite: bool) -> f64 {
        todo!("port MSLCM_SL2015::getLeftBorder")
    }

    /// C++: `double MSLCM_SL2015::getVehicleCenter() const` [const, protected]
    /// return vehicle position relative to the current edge (extend by another virtual lane for opposite-direction driving)
    // double
    // MSLCM_SL2015::getVehicleCenter() const {
    //     if (isOpposite()) {
    //         return myVehicle.getEdge()->getWidth() + myVehicle.getLane()->getWidth() * 0.5 - myVehicle.getLateralPositionOnLane();
    //     } else {
    //         return myVehicle.getCenterOnEdge();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_vehicle_center(&self) -> f64 {
        todo!("port MSLCM_SL2015::getVehicleCenter")
    }

    /// C++: `double MSLCM_SL2015::getNeighRight(const MSLane &) const` [const, protected]
    /// return the right offset of the neighboring lane relative to the current edge
    // double
    // MSLCM_SL2015::getNeighRight(const MSLane& neighLane) const {
    //     if (isOpposite()) {
    //         return myVehicle.getLane()->getRightSideOnEdge() - neighLane.getWidth() + 2 * myVehicle.getLateralPositionOnLane();
    //     } else if ((&myVehicle.getLane()->getEdge() != &neighLane.getEdge())) {
    //         return myVehicle.getLane()->getRightSideOnEdge() + myVehicle.getLane()->getWidth();
    //     } else {
    //         // the normal case
    //         return neighLane.getRightSideOnEdge();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_neigh_right(&self, neighLane: &MSLane) -> f64 {
        todo!("port MSLCM_SL2015::getNeighRight")
    }

    /// C++: `bool MSLCM_SL2015::preventSliding(double) const` [const, protected]
    // bool
    // MSLCM_SL2015::preventSliding(double maneuverDist) const {
    //     // prevent wide maneuvers with unsufficient forward space
    //     if (fabs(maneuverDist) > myMaxDistLatStanding) {
    //         // emergency vehicles should not be restricted (TODO solve this with LCA_URGENT)
    //         if (myVehicle.getVehicleType().getVehicleClass() == SVC_EMERGENCY) {
    //             return false;
    //         }
    //         const double brakeGap = myVehicle.getCarFollowModel().brakeGap(myVehicle.getSpeed());
    //         const bool isSlide = fabs(maneuverDist) > myMaxDistLatStanding + brakeGap * fabs(myMaxSpeedLatFactor);
    // #ifdef DEBUG_SLIDING
    //         if (gDebugFlag2) {
    //             std::cout << SIMTIME << " veh=" << myVehicle.getID() << " bgap=" << brakeGap << " maneuverDist=" << maneuverDist
    //                       << " mds=" << myMaxDistLatStanding << " isSlide=" << isSlide << "\n";
    //         }
    // #endif
    //         return isSlide;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn prevent_sliding(&self, maneuverDist: f64) -> bool {
        todo!("port MSLCM_SL2015::preventSliding")
    }

    /// C++: `bool MSLCM_SL2015::wantsKeepRight(double) const` [const, protected]
    /// check against thresholds
    // bool
    // MSLCM_SL2015::wantsKeepRight(double keepRightProb) const {
    //     return keepRightProb * myKeepRightParam > MAX2(myChangeProbThresholdRight, mySpeedGainProbabilityLeft);
    // }
    #[allow(unused_variables)]
    pub fn wants_keep_right(&self, keepRightProb: f64) -> bool {
        todo!("port MSLCM_SL2015::wantsKeepRight")
    }

    /// C++: `bool MSLCM_SL2015::isBidi(const MSLane *) const` [const, protected]
    /// check whether lane is an upcoming bidi lane
    // bool
    // MSLCM_SL2015::isBidi(const MSLane* lane) const {
    //     if (!MSNet::getInstance()->hasBidiEdges()) {
    //         return false;
    //     }
    //     if (lane == myVehicle.getLane()->getBidiLane()) {
    //         return true;
    //     }
    //     for (const MSLane* cand : myVehicle.getBestLanesContinuation()) {
    //         if (cand != nullptr && cand->getBidiLane() == lane) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn is_bidi(&self, lane: Ptr<MSLane>) -> bool {
        todo!("port MSLCM_SL2015::isBidi")
    }

    /// C++: `double MSLCM_SL2015::emergencySpeedLat(double) const` [const, protected]
    /// avoid unsafe lateral speed (overruling lcAccelLat)
    // double
    // MSLCM_SL2015::emergencySpeedLat(double speedLat) const {
    //     // reduce lateral speed for safety purposes
    //     if (speedLat < 0 && SPEED2DIST(-speedLat) > mySafeLatDistRight) {
    //         speedLat = -DIST2SPEED(mySafeLatDistRight);
    // #ifdef DEBUG_MANEUVER
    //         if (debugVehicle()) {
    //             std::cout << "   rightDanger speedLat=" << speedLat << "\n";
    //         }
    // #endif
    //     } else if (speedLat > 0 && SPEED2DIST(speedLat) > mySafeLatDistLeft) {
    //         speedLat = DIST2SPEED(mySafeLatDistLeft);
    // #ifdef DEBUG_MANEUVER
    //         if (debugVehicle()) {
    //             std::cout << "   leftDanger speedLat=" << speedLat << "\n";
    //         }
    // #endif
    //     }
    //     return speedLat;
    // }
    #[allow(unused_variables)]
    pub fn emergency_speed_lat(&self, speedLat: f64) -> f64 {
        todo!("port MSLCM_SL2015::emergencySpeedLat")
    }

}

