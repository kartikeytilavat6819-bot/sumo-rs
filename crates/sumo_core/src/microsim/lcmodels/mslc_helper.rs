//! Port of microsim/lcmodels/MSLCHelper.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::time::{accel2speed, speed2dist};
use utils::geom::POSITION_EPS;

/// C++ `MAX2(a, b)` (utils/common/StdDefs.h): `a > b ? a : b`
fn max2(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}

/// A lane change model developed by J. Erdmann
/// C++ class `MSLCHelper` (microsim/lcmodels/MSLCHelper.h)
pub struct MSLCHelper {
}

impl MSLCHelper {
    /// C++: `double MSLCHelper::getRoundaboutDistBonus(const MSVehicle &, double, const MSVehicle::LaneQ &, const MSVehicle::LaneQ &, const MSVehicle::LaneQ &)` [static]
    // double
    // MSLCHelper::getRoundaboutDistBonus(const MSVehicle& veh,
    //                                    double bonusParam,
    //                                    const MSVehicle::LaneQ& curr,
    //                                    const MSVehicle::LaneQ& neigh,
    //                                    const MSVehicle::LaneQ& best) {
    //     if (veh.getLaneChangeModel().isOpposite()) {
    //         return 0;
    //     }
    //     const MSVehicle::LaneQ& inner = neigh.lane->getIndex() > curr.lane->getIndex() ? neigh : curr;
    // #ifdef DEBUG_WANTS_CHANGE
    //     const bool debugVehicle = veh.getLaneChangeModel().debugVehicle();
    //     if (debugVehicle) {
    //         std::cout << SIMTIME << " veh=" << veh.getID() << " getRoundaboutDistBonus bonusParam=" << bonusParam
    //                   << " curr=" << curr.lane->getID()
    //                   << " neigh=" << neigh.lane->getID()
    //                   << " inner=" << inner.lane->getID()
    //                   << " best=" << best.lane->getID()
    //                   << "\n   innerCont=" << toString(inner.bestContinuations)
    //                   << "\n   bestCont=" << toString(best.bestContinuations)
    //                   << "\n";
    //     }
    // #endif
    //     if (neigh.lane == inner.lane && curr.bestContinuations.size() < neigh.bestContinuations.size()) {
    //         // the current lane does not continue to the roundabout and we need a strategic change first.
    //         return 0;
    //     }
    //
    //     int roundaboutJunctionsAhead = 0;
    //     bool enteredRoundabout = false;
    //     double seen = -veh.getPositionOnLane();
    //
    //     // first check using only normal lanes
    //     for (int i = 0; i < (int)best.bestContinuations.size(); i++) {
    //         const MSLane* lane = best.bestContinuations[i];
    //         if (lane == nullptr) {
    //             lane = veh.getLane();
    //         }
    //         if ((!enteredRoundabout || lane->getEdge().isRoundabout()) && i >= (int)inner.bestContinuations.size()) {
    //             // no bonus if we cannot continue on the inner lane until leaving the roundabout
    // #ifdef DEBUG_WANTS_CHANGE
    //             if (debugVehicle) {
    //                 std::cout << "   noBonus: inner does not continue (lane=" << lane->getID() << ")\n";
    //             }
    // #endif
    //             return 0;
    //         }
    //         if (seen > 300) {
    //             // avoid long look-ahead
    // #ifdef DEBUG_WANTS_CHANGE
    //             if (debugVehicle) {
    //                 std::cout << "   noBonus: seen=" << seen << " (lane=" << lane->getID() << ")\n";
    //             }
    // #endif
    //             return 0;
    //         }
    //         const MSJunction* junction = lane->getEdge().getToJunction();
    //         if (lane->getEdge().isRoundabout()) {
    //             enteredRoundabout = true;
    //             if (junction->getIncoming().size() + junction->getOutgoing().size() > 2) {
    //                 roundaboutJunctionsAhead++;
    //             }
    //         } else if (enteredRoundabout) {
    //             // only check the first roundabout
    //             break;
    //         }
    //         seen += lane->getLength();
    //     }
    //     // no bonus if we want to take the next exit
    //     if (roundaboutJunctionsAhead < 2) {
    // #ifdef DEBUG_WANTS_CHANGE
    //         if (debugVehicle) {
    //             std::cout << "   noBonus: roundaboutJunctionsAhead=" << roundaboutJunctionsAhead << "\n";
    //         }
    // #endif
    //         return 0;
    //     }
    //     // compute bonus value based on jamming and exact distances (taking into
    //     // account internal lanes)
    //     double occupancyOuter = 0;
    //     double occupancyInner = 0;
    //     double distanceInRoundabout = 0;
    //     MSLane* prevNormal = nullptr;
    //     MSLane* prevInner = nullptr;
    //     enteredRoundabout = false;
    //     for (int i = 0; i < (int)best.bestContinuations.size(); i++) {
    //         MSLane* lane = best.bestContinuations[i];
    //         if (lane == nullptr) {
    //             continue;
    //         }
    //         if (lane->getEdge().isRoundabout()) {
    //             enteredRoundabout = true;
    //         } else if (enteredRoundabout) {
    //             // only check the first roundabout
    //             break;
    //         }
    //         MSLane* via = nullptr;
    //         if (prevNormal != nullptr) {
    //             for (MSLink* link : prevNormal->getLinkCont()) {
    //                 if (link->getLane() == lane) {
    //                     via = link->getViaLane();
    //                 }
    //             }
    //         }
    //         if (enteredRoundabout) {
    //             distanceInRoundabout += lane->getLength();
    //             if (via != nullptr) {
    //                 distanceInRoundabout += via->getLength();
    //             }
    //         }
    //         // discount vehicles that are upstream from ego
    //         const double upstreamDiscount = &lane->getEdge() == &veh.getLane()->getEdge()
    //                                         ? (lane->getLength() - veh.getPositionOnLane()) / lane->getLength() : 1;
    //         prevNormal = lane;
    //         occupancyOuter += upstreamDiscount * lane->getBruttoVehLenSum();
    // #ifdef DEBUG_WANTS_CHANGE
    //         if (debugVehicle) {
    //             std::cout << " lane=" << lane->getID() << " occ=" << lane->getBruttoVehLenSum() << " discount=" << upstreamDiscount << " outer=" << occupancyOuter << "\n";
    //         }
    // #endif
    //         if (via != nullptr) {
    //             occupancyOuter += via->getBruttoVehLenSum();
    // #ifdef DEBUG_WANTS_CHANGE
    //             if (debugVehicle) {
    //                 std::cout << " via=" << via->getID() << " occ=" << via->getBruttoVehLenSum() << " outer=" << occupancyOuter << "\n";
    //             }
    // #endif
    //         }
    //         if (i < (int)inner.bestContinuations.size()) {
    //             MSLane* innerLane = inner.bestContinuations[i];
    //             occupancyInner += upstreamDiscount * innerLane->getBruttoVehLenSum();
    // #ifdef DEBUG_WANTS_CHANGE
    //             if (debugVehicle) {
    //                 std::cout << " inner=" << innerLane->getID() << " occ=" << innerLane->getBruttoVehLenSum() << " discount=" << upstreamDiscount << " inner=" << occupancyInner << "\n";
    //             }
    // #endif
    //             if (prevInner != nullptr) {
    //                 for (MSLink* link : prevInner->getLinkCont()) {
    //                     if (link->getLane() == innerLane && link->getViaLane() != nullptr) {
    //                         occupancyInner += link->getViaLane()->getBruttoVehLenSum();
    // #ifdef DEBUG_WANTS_CHANGE
    //                         if (debugVehicle) {
    //                             std::cout << " innerVia=" << link->getViaLane()->getID() << " occ=" << link->getViaLane()->getBruttoVehLenSum() << " inner=" << occupancyInner << "\n";
    //                         }
    // #endif
    //                     }
    //                 }
    //             }
    //             prevInner = innerLane;
    //         }
    //     }
    //
    // #ifdef DEBUG_WANTS_CHANGE
    //     if (debugVehicle) {
    //         std::cout << "   distanceInRoundabout=" << distanceInRoundabout
    //                   << " roundaboutJunctionsAhead=" << roundaboutJunctionsAhead
    //                   << " occupancyInner=" << occupancyInner
    //                   << " occupancyOuter=" << occupancyOuter
    //                   << "\n";
    //     }
    // #endif
    //     if (abs(curr.bestLaneOffset) > 1 && enteredRoundabout) {
    //         const double bGap = veh.getCarFollowModel().brakeGap(veh.getSpeed() + ACCEL2SPEED(veh.getCarFollowModel().getMaxAccel()), veh.getCarFollowModel().getMaxDecel(), veh.getActionStepLengthSecs());
    //         const double reservation = veh.getLaneChangeModel().getExtraReservation(curr.bestLaneOffset);
    //         const double leftSpace = distanceInRoundabout - reservation - veh.getPositionOnLane();
    // #ifdef DEBUG_WANTS_CHANGE
    //         if (debugVehicle) {
    //             std::cout << "   bGap=" << bGap << " reserving=" << reservation << " leftover=" << (leftSpace - bGap) << "\n";
    //         }
    // #endif
    //         if (bGap > leftSpace) {
    //             return 0;
    //         }
    //     }
    //
    //     const double maxOccupancy = MAX2(occupancyInner, occupancyOuter);
    //     // give some bonus for using the inside lane at equal occupancy
    //     const double bonus = roundaboutJunctionsAhead * 7.5;
    //     const double relativeJam = (occupancyOuter - occupancyInner + bonus) / (maxOccupancy + bonus);
    //     // no bonus if the inner lane or the left lane entering the roundabout is jammed
    //     double jamFactor = MAX2(0.0, relativeJam);
    //     if (veh.getLane()->getEdge().isRoundabout() && curr.lane->getIndex() > neigh.lane->getIndex()) {
    //         // only use jamFactor when deciding to move to the inside lane but prefer
    //         // staying inside if the distance allows it
    //         jamFactor = 1;
    //     }
    //     const double result = distanceInRoundabout * jamFactor * bonusParam * 9; // the 9 is abitrary and only there for backward compatibility
    // #ifdef DEBUG_WANTS_CHANGE
    //     if (debugVehicle) {
    //         std::cout << "   relativeJam=" << relativeJam
    //                   << " jamFactor=" << jamFactor
    //                   << " distanceBonus=" << result
    //                   << "\n";
    //     }
    // #endif
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_roundabout_dist_bonus(veh: &MSVehicle, bonusParam: f64, curr: &MSVehicle_LaneQ, neigh: &MSVehicle_LaneQ, best: &MSVehicle_LaneQ) -> f64 {
        // DEBUG_WANTS_CHANGE output is compiled out (macro not defined), so it is not ported.
        if Ptr::new(veh).m().get_lane_change_model().is_opposite() {
            return 0.0;
        }
        let inner: &MSVehicle_LaneQ = if neigh.lane.get_index() > curr.lane.get_index() { neigh } else { curr };
        if neigh.lane == inner.lane && curr.bestContinuations.len() < neigh.bestContinuations.len() {
            // the current lane does not continue to the roundabout and we need a strategic change first.
            return 0.0;
        }

        let mut roundaboutJunctionsAhead: i32 = 0;
        let mut enteredRoundabout = false;
        let mut seen: f64 = -veh.get_position_on_lane();

        // first check using only normal lanes
        for i in 0..best.bestContinuations.len() {
            let mut lane: Ptr<MSLane> = best.bestContinuations[i];
            if lane.is_null() {
                lane = veh.get_lane();
            }
            if (!enteredRoundabout || lane.get_edge().is_roundabout()) && i >= inner.bestContinuations.len() {
                // no bonus if we cannot continue on the inner lane until leaving the roundabout
                return 0.0;
            }
            if seen > 300.0 {
                // avoid long look-ahead
                return 0.0;
            }
            let junction = lane.get_edge().get_to_junction();
            if lane.get_edge().is_roundabout() {
                enteredRoundabout = true;
                if junction.get_incoming().len() + junction.get_outgoing().len() > 2 {
                    roundaboutJunctionsAhead += 1;
                }
            } else if enteredRoundabout {
                // only check the first roundabout
                break;
            }
            seen += lane.get_length();
        }
        // no bonus if we want to take the next exit
        if roundaboutJunctionsAhead < 2 {
            return 0.0;
        }
        // compute bonus value based on jamming and exact distances (taking into
        // account internal lanes)
        let mut occupancyOuter: f64 = 0.0;
        let mut occupancyInner: f64 = 0.0;
        let mut distanceInRoundabout: f64 = 0.0;
        let mut prevNormal: Ptr<MSLane> = Ptr::default();
        let mut prevInner: Ptr<MSLane> = Ptr::default();
        enteredRoundabout = false;
        for i in 0..best.bestContinuations.len() {
            let lane: Ptr<MSLane> = best.bestContinuations[i];
            if lane.is_null() {
                continue;
            }
            if lane.get_edge().is_roundabout() {
                enteredRoundabout = true;
            } else if enteredRoundabout {
                // only check the first roundabout
                break;
            }
            let mut via: Ptr<MSLane> = Ptr::default();
            if !prevNormal.is_null() {
                for &link in prevNormal.get_link_cont().iter() {
                    if link.get_lane() == lane {
                        via = link.get_via_lane();
                    }
                }
            }
            if enteredRoundabout {
                distanceInRoundabout += lane.get_length();
                if !via.is_null() {
                    distanceInRoundabout += via.get_length();
                }
            }
            // discount vehicles that are upstream from ego
            let upstreamDiscount: f64 = if std::ptr::eq(lane.get_edge() as *const MSEdge, veh.get_lane().get_edge() as *const MSEdge) {
                (lane.get_length() - veh.get_position_on_lane()) / lane.get_length()
            } else {
                1.0
            };
            prevNormal = lane;
            occupancyOuter += upstreamDiscount * lane.get_brutto_veh_len_sum();
            if !via.is_null() {
                occupancyOuter += via.get_brutto_veh_len_sum();
            }
            if i < inner.bestContinuations.len() {
                let innerLane: Ptr<MSLane> = inner.bestContinuations[i];
                occupancyInner += upstreamDiscount * innerLane.get_brutto_veh_len_sum();
                if !prevInner.is_null() {
                    for &link in prevInner.get_link_cont().iter() {
                        if link.get_lane() == innerLane && !link.get_via_lane().is_null() {
                            occupancyInner += link.get_via_lane().get_brutto_veh_len_sum();
                        }
                    }
                }
                prevInner = innerLane;
            }
        }

        if curr.bestLaneOffset.abs() > 1 && enteredRoundabout {
            let bGap = veh.get_car_follow_model().brake_gap_2(
                veh.get_speed() + accel2speed(veh.get_car_follow_model().get_max_accel()),
                veh.get_car_follow_model().get_max_decel(),
                veh.get_action_step_length_secs(),
            );
            let reservation = Ptr::new(veh).m().get_lane_change_model().get_extra_reservation(curr.bestLaneOffset, 0.0);
            let leftSpace = distanceInRoundabout - reservation - veh.get_position_on_lane();
            if bGap > leftSpace {
                return 0.0;
            }
        }

        let maxOccupancy = max2(occupancyInner, occupancyOuter);
        // give some bonus for using the inside lane at equal occupancy
        let bonus = roundaboutJunctionsAhead as f64 * 7.5;
        let relativeJam = (occupancyOuter - occupancyInner + bonus) / (maxOccupancy + bonus);
        // no bonus if the inner lane or the left lane entering the roundabout is jammed
        let mut jamFactor = max2(0.0, relativeJam);
        if veh.get_lane().get_edge().is_roundabout() && curr.lane.get_index() > neigh.lane.get_index() {
            // only use jamFactor when deciding to move to the inside lane but prefer
            // staying inside if the distance allows it
            jamFactor = 1.0;
        }
        let result = distanceInRoundabout * jamFactor * bonusParam * 9.0; // the 9 is abitrary and only there for backward compatibility
        result
    }

    /// C++: `bool MSLCHelper::updateBlockerLength(const MSVehicle &, MSVehicle *, int, double, bool, double &)` [static]
    // bool
    // MSLCHelper::updateBlockerLength(const MSVehicle& veh,  MSVehicle* blocker, int lcaCounter, double leftSpace, bool reliefConnection, double& leadingBlockerLength) {
    // #ifdef DEBUG_SAVE_BLOCKER_LENGTH
    //     if (DEBUG_COND) {
    //         std::cout << SIMTIME
    //                   << " veh=" << veh.getID()
    //                   << " saveBlockerLength blocker=" << Named::getIDSecure(blocker)
    //                   << " bState=" << (blocker == 0 ? "None" : toString((LaneChangeAction)blocker->getLaneChangeModel().getOwnState()))
    //                   << "\n";
    //     }
    // #endif
    //     if (blocker != nullptr && (blocker->getLaneChangeModel().getOwnState() & lcaCounter) != 0) {
    //         // is there enough space in front of us for the blocker?
    //         const double required = blocker->getVehicleType().getLengthWithGap() + veh.getVehicleType().getMinGap();
    //         const double potential = leftSpace - veh.getCarFollowModel().brakeGap(
    //                                      veh.getSpeed(), veh.getCarFollowModel().getMaxDecel(), 0);
    //         if (required <= potential) {
    //             // save at least his length in myLeadingBlockerLength
    //             leadingBlockerLength = MAX2(required, leadingBlockerLength);
    // #ifdef DEBUG_SAVE_BLOCKER_LENGTH
    //             if (DEBUG_COND) {
    //                 std::cout << SIMTIME
    //                           << " veh=" << veh.getID()
    //                           << " required=" << required
    //                           << " potential=" << potential
    //                           << " blocker=" << Named::getIDSecure(blocker)
    //                           << " saving myLeadingBlockerLength=" << leadingBlockerLength
    //                           << "\n";
    //             }
    // #endif
    //         } else {
    //             // we cannot save enough space for the blocker. It needs to save
    //             // space for ego instead
    //             const double required2 = veh.getVehicleType().getLengthWithGap() + blocker->getVehicleType().getMinGap();
    //             const double foeLeftSpace = leftSpace - veh.getPositionOnLane() + blocker->getPositionOnLane() - POSITION_EPS;
    //             const bool canReserve = blocker->getLaneChangeModel().saveBlockerLength(required2, foeLeftSpace);
    //             //reliefConnection ? std::numeric_limits<double>::max() : leftSpace);
    // #ifdef DEBUG_SAVE_BLOCKER_LENGTH
    //             if (DEBUG_COND) {
    //                 std::cout << SIMTIME
    //                           << " veh=" << veh.getID()
    //                           << " required=" << required
    //                           << " potential=" << potential
    //                           << " blocker=" << Named::getIDSecure(blocker)
    //                           << " required2=" << required2
    //                           << " foeCanReserve=" << canReserve
    //                           << " myReserved=" << leadingBlockerLength
    //                           << " reliefConnection=" << reliefConnection
    //                           << "\n";
    //             }
    // #endif
    //             if (!canReserve && !reliefConnection) {
    //                 const int blockerState = blocker->getLaneChangeModel().getOwnState();
    //                 if ((blockerState & LCA_STRATEGIC) != 0
    //                         && (blockerState & LCA_URGENT) != 0) {
    //                     // reserve anyway and try to avoid deadlock with emergency deceleration
    //                     leadingBlockerLength = MAX2(required, leadingBlockerLength);
    // #ifdef DEBUG_SAVE_BLOCKER_LENGTH
    //                     if (DEBUG_COND) {
    //                         std::cout << "   reserving anyway to avoid deadlock (will cause emergency braking)\n";
    //                     }
    // #endif
    //                 }
    //             }
    //             return canReserve;
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn update_blocker_length(veh: &MSVehicle, blocker: Ptr<MSVehicle>, lcaCounter: i32, leftSpace: f64, reliefConnection: bool, leadingBlockerLength: &mut f64) -> bool {
        // DEBUG_SAVE_BLOCKER_LENGTH output is compiled out, so it is not ported.
        if !blocker.is_null() && (blocker.m().get_lane_change_model().get_own_state() & lcaCounter) != 0 {
            // is there enough space in front of us for the blocker?
            let required = blocker.get_vehicle_type().get_length_with_gap() + veh.get_vehicle_type().get_min_gap();
            let potential = leftSpace - veh.get_car_follow_model().brake_gap_2(
                veh.get_speed(),
                veh.get_car_follow_model().get_max_decel(),
                0.0,
            );
            if required <= potential {
                // save at least his length in myLeadingBlockerLength
                *leadingBlockerLength = max2(required, *leadingBlockerLength);
            } else {
                // we cannot save enough space for the blocker. It needs to save
                // space for ego instead
                let required2 = veh.get_vehicle_type().get_length_with_gap() + blocker.get_vehicle_type().get_min_gap();
                let foeLeftSpace = leftSpace - veh.get_position_on_lane() + blocker.get_position_on_lane() - POSITION_EPS;
                let canReserve = blocker.m().get_lane_change_model().save_blocker_length(required2, foeLeftSpace);
                if !canReserve && !reliefConnection {
                    let blockerState = blocker.m().get_lane_change_model().get_own_state();
                    if (blockerState & LCA_STRATEGIC) != 0 && (blockerState & LCA_URGENT) != 0 {
                        // reserve anyway and try to avoid deadlock with emergency deceleration
                        *leadingBlockerLength = max2(required, *leadingBlockerLength);
                    }
                }
                return canReserve;
            }
        }
        true
    }

    /// C++: `bool MSLCHelper::canSaveBlockerLength(const MSVehicle &, double, double)` [static]
    // bool
    // MSLCHelper::canSaveBlockerLength(const MSVehicle& veh, double requested, double leftSpace) {
    //     const double potential = leftSpace - veh.getCarFollowModel().brakeGap(veh.getSpeed(), veh.getCarFollowModel().getMaxDecel(), veh.getActionStepLengthSecs());
    // #ifdef DEBUG_SAVE_BLOCKER_LENGTH
    //     if (DEBUG_COND) {
    //         std::cout << SIMTIME << " canSaveBlockerLength veh=" << veh.getID() << " requested=" << requested << " leftSpace=" << leftSpace << " potential=" << potential << "\n";
    //     }
    // #endif
    //     return potential >= requested;
    // }
    #[allow(unused_variables)]
    pub fn can_save_blocker_length(veh: &MSVehicle, requested: f64, leftSpace: f64) -> bool {
        let potential = leftSpace - veh.get_car_follow_model().brake_gap_2(
            veh.get_speed(),
            veh.get_car_follow_model().get_max_decel(),
            veh.get_action_step_length_secs(),
        );
        // DEBUG_SAVE_BLOCKER_LENGTH output is compiled out, so it is not ported.
        potential >= requested
    }

    /// C++: `bool MSLCHelper::divergentRoute(const MSVehicle &, const MSVehicle &)` [static]
    /// return whether the vehicles are on the same junction but on divergent paths
    // bool
    // MSLCHelper::divergentRoute(const MSVehicle& v1, const MSVehicle& v2) {
    //     // a sufficient, but not necessary condition for divergence
    //     return (v1.getLane()->isInternal() && v2.getLane()->isInternal()
    //             && v1.getLane()->getEdge().getFromJunction() == v2.getLane()->getEdge().getFromJunction()
    //             && &v1.getLane()->getEdge() != &v2.getLane()->getEdge());
    // }
    #[allow(unused_variables)]
    pub fn divergent_route(v1: &MSVehicle, v2: &MSVehicle) -> bool {
        // a sufficient, but not necessary condition for divergence
        v1.get_lane().is_internal()
            && v2.get_lane().is_internal()
            && v1.get_lane().get_edge().get_from_junction() == v2.get_lane().get_edge().get_from_junction()
            && !std::ptr::eq(v1.get_lane().get_edge() as *const MSEdge, v2.get_lane().get_edge() as *const MSEdge)
    }

    /// C++: `double MSLCHelper::getSpeedPreservingSecureGap(const MSVehicle &, const MSVehicle &, double, double)` [static]
    // double
    // MSLCHelper::getSpeedPreservingSecureGap(const MSVehicle& leader, const MSVehicle& follower, double currentGap, double leaderPlannedSpeed) {
    //     // whatever speed the follower choses in the next step, it will change both
    //     // the secureGap and the required followSpeed.
    //     // Let's assume the leader maintains speed
    //     const double nextGap = currentGap + SPEED2DIST(leaderPlannedSpeed - follower.getSpeed());
    //     double sGap = follower.getCarFollowModel().getSecureGap(&follower, &leader, follower.getSpeed(), leaderPlannedSpeed, leader.getCarFollowModel().getMaxDecel());
    //     if (nextGap >= sGap) {
    //         // follower may still accelerate
    //         const double nextGapMin = currentGap + SPEED2DIST(leaderPlannedSpeed - follower.getCarFollowModel().maxNextSpeed(follower.getSpeed(), &follower));
    //         const double vSafe = follower.getCarFollowModel().followSpeed(
    //                                  &follower, follower.getSpeed(), nextGapMin, leaderPlannedSpeed, leader.getCarFollowModel().getMaxDecel());
    //         return MAX2(vSafe, follower.getSpeed());
    //     } else {
    //         // follower must brake. The following brakes conservatively since the actual gap will be lower due to braking.
    //         const double vSafe = follower.getCarFollowModel().followSpeed(
    //                                  &follower, follower.getSpeed(), nextGap, leaderPlannedSpeed, leader.getCarFollowModel().getMaxDecel());
    //         // avoid emergency deceleration
    //         return MAX2(vSafe, follower.getCarFollowModel().minNextSpeed(follower.getSpeed(), &follower));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_speed_preserving_secure_gap(leader: &MSVehicle, follower: &MSVehicle, currentGap: f64, leaderPlannedSpeed: f64) -> f64 {
        // whatever speed the follower choses in the next step, it will change both
        // the secureGap and the required followSpeed.
        // Let's assume the leader maintains speed
        let nextGap = currentGap + speed2dist(leaderPlannedSpeed - follower.get_speed());
        let sGap = follower.get_car_follow_model().get_secure_gap(
            Ptr::new(follower),
            Ptr::new(leader),
            follower.get_speed(),
            leaderPlannedSpeed,
            leader.get_car_follow_model().get_max_decel(),
        );
        if nextGap >= sGap {
            // follower may still accelerate
            let nextGapMin = currentGap
                + speed2dist(leaderPlannedSpeed - follower.get_car_follow_model().max_next_speed(follower.get_speed(), Ptr::new(follower)));
            let vSafe = follower.get_car_follow_model().follow_speed(
                Ptr::new(follower),
                follower.get_speed(),
                nextGapMin,
                leaderPlannedSpeed,
                leader.get_car_follow_model().get_max_decel(),
                Ptr::default(),
                MSCFModel_CalcReason::CURRENT,
            );
            max2(vSafe, follower.get_speed())
        } else {
            // follower must brake. The following brakes conservatively since the actual gap will be lower due to braking.
            let vSafe = follower.get_car_follow_model().follow_speed(
                Ptr::new(follower),
                follower.get_speed(),
                nextGap,
                leaderPlannedSpeed,
                leader.get_car_follow_model().get_max_decel(),
                Ptr::default(),
                MSCFModel_CalcReason::CURRENT,
            );
            // avoid emergency deceleration
            max2(vSafe, follower.get_car_follow_model().min_next_speed(follower.get_speed(), Ptr::new(follower)))
        }
    }

    /// C++: `bool MSLCHelper::isBidiLeader(const MSVehicle *, const std::vector<MSLane *> &)` [static]
    // bool
    // MSLCHelper::isBidiLeader(const MSVehicle* leader, const std::vector<MSLane*>& cont) {
    //     if (leader == nullptr) {
    //         return false;
    //     }
    //     const MSLane* lane1 = leader->getLane()->getNormalSuccessorLane()->getBidiLane();
    //     const MSLane* lane2 = leader->getLane()->getNormalPredecessorLane()->getBidiLane();
    //     if (lane1 == nullptr && lane2 == nullptr) {
    //         return false;
    //     }
    //     bool result = std::find(cont.begin(), cont.end(), lane1) != cont.end();
    //     if (!result && lane1 != lane2 && lane2 != nullptr) {
    //         result = std::find(cont.begin(), cont.end(), lane2) != cont.end();
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn is_bidi_leader(leader: Ptr<MSVehicle>, cont: &Vec<Ptr<MSLane>>) -> bool {
        if leader.is_null() {
            return false;
        }
        let lane1 = leader.get_lane().get_normal_successor_lane().get_bidi_lane();
        let lane2 = leader.get_lane().get_normal_predecessor_lane().get_bidi_lane();
        if lane1.is_null() && lane2.is_null() {
            return false;
        }
        let mut result = cont.iter().any(|&l| l == lane1);
        if !result && lane1 != lane2 && !lane2.is_null() {
            result = cont.iter().any(|&l| l == lane2);
        }
        result
    }

    /// C++: `bool MSLCHelper::isBidiFollower(const MSVehicle *, const MSVehicle *)` [static]
    // bool
    // MSLCHelper::isBidiFollower(const MSVehicle* ego, const MSVehicle* follower) {
    //     if (follower == nullptr) {
    //         return false;
    //     }
    //     bool result = false;
    //     const MSLane* lane1 = follower->getLane()->getNormalSuccessorLane()->getBidiLane();
    //     const MSLane* lane2 = follower->getLane()->getNormalPredecessorLane()->getBidiLane();
    //     const ConstMSEdgeVector& route = ego->getRoute().getEdges();
    //     if (lane1 != nullptr) {
    //         result = std::find(route.begin(), route.end(), &lane1->getEdge()) != route.end();
    //     }
    //     if (!result && lane1 != lane2 && lane2 != nullptr) {
    //         result = std::find(route.begin(), route.end(), &lane2->getEdge()) != route.end();
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn is_bidi_follower(ego: Ptr<MSVehicle>, follower: Ptr<MSVehicle>) -> bool {
        if follower.is_null() {
            return false;
        }
        let mut result = false;
        let lane1 = follower.get_lane().get_normal_successor_lane().get_bidi_lane();
        let lane2 = follower.get_lane().get_normal_predecessor_lane().get_bidi_lane();
        let route = ego.get_route().get_edges();
        if !lane1.is_null() {
            let edge1 = Ptr::new(lane1.get_edge());
            result = route.iter().any(|&e| e == edge1);
        }
        if !result && lane1 != lane2 && !lane2.is_null() {
            let edge2 = Ptr::new(lane2.get_edge());
            result = route.iter().any(|&e| e == edge2);
        }
        result
    }

    /// C++: `bool MSLCHelper::unwillingToHelp(const MSVehicle &, double, const MSVehicle &)` [static]
    /// whether the neighboring vehicle nv is unwilling to yield to help ego merge
    // bool
    // MSLCHelper::unwillingToHelp(const MSVehicle& ego, double plannedSpeed, const MSVehicle& nv) {
    //     if (nv.getLaneChangeModel().getCooperativeHelpTime() < 0 || ego.getWaitingTime() < nv.getLaneChangeModel().getCooperativeHelpTime()) {
    //         // ego vehicle has not been waiting long enough to be eligible for unconditional help
    //         if (nv.getLaneChangeModel().getCooperativeHelpThreshold() >= 0 && (nv.getSpeed() - plannedSpeed) > nv.getLaneChangeModel().getCooperativeHelpThreshold()) {
    //             // neighhbor not willing to help because ego is much slower
    // #ifdef DEBUG_UNWILLING_TO_HELP
    //             if (DEBUG_COND) {
    //                 std::cout << "\n nv=" << nv.getID() << " not willing to help because ego is much slower\n";
    //             }
    // #endif
    //             return true;
    //         }
    //         const double nvLaneMax = nv.getLane()->getVehicleMaxSpeed(&nv);
    //         if (nv.getSpeed() / nvLaneMax < nv.getLaneChangeModel().getCooperativeMinSpeed()) {
    //             // neighbor not willing to help because it is already to slow and does not want to disturb the flow
    // #ifdef DEBUG_UNWILLING_TO_HELP
    //             if (DEBUG_COND) {
    //                 std::cout << "\n nv=" << nv.getID() << " not willing to help because it is already too slow\n";
    //             }
    // #endif
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn unwilling_to_help(ego: &MSVehicle, plannedSpeed: f64, nv: &MSVehicle) -> bool {
        let nvPtr = Ptr::new(nv);
        let nvLcm = nvPtr.m().get_lane_change_model();
        if nvLcm.get_cooperative_help_time() < 0 || ego.get_waiting_time(false) < nvLcm.get_cooperative_help_time() {
            // ego vehicle has not been waiting long enough to be eligible for unconditional help
            if nvLcm.get_cooperative_help_threshold() >= 0.0 && (nv.get_speed() - plannedSpeed) > nvLcm.get_cooperative_help_threshold() {
                // neighhbor not willing to help because ego is much slower
                return true;
            }
            let nvLaneMax = nv.get_lane().get_vehicle_max_speed(nvPtr.cast::<SUMOTrafficObject>());
            if nv.get_speed() / nvLaneMax < nvLcm.get_cooperative_min_speed() {
                // neighbor not willing to help because it is already to slow and does not want to disturb the flow
                return true;
            }
        }
        false
    }

}

