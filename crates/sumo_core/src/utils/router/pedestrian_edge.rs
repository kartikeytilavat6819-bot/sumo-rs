//! Port of utils/router/PedestrianEdge.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use crate::utils::router::intermodal_edge::{get_sidewalk, IntermodalEdge, IntermodalEdgeKind};
use crate::utils::router::intermodal_trip::IntermodalTrip;
use crate::utils::router::router_edge::{RouterEdge, RouterVehicle};
use utils::common::to_string::to_string;

/// C++ `#define TL_RED_PENALTY 20`
const TL_RED_PENALTY: i32 = 20;

/// the pedestrian edge type that is given to the internal router (SUMOAbstractRouter)
/// C++ class `PedestrianEdge` (utils/router/PedestrianEdge.h)
#[repr(C)]
pub struct PedestrianEdge<E, L, N, V> {
    /// C++ base class `IntermodalEdge<E, L, N, V>`
    pub base: IntermodalEdge<E, L, N, V>,
    /// the original edge
    pub myLane: Ptr<L>,
    /// the direction of this edge
    pub myForward: bool,
    /// the starting position for split edges
    pub myStartPos: f64,
    /// whether this edge goes against the flow of traffic
    pub myIsOpposite: bool,
    _p: std::marker::PhantomData<(E, L, N, V,)>,
}

impl<E, L, N, V> std::ops::Deref for PedestrianEdge<E, L, N, V> {
    type Target = IntermodalEdge<E, L, N, V>;
    fn deref(&self) -> &IntermodalEdge<E, L, N, V> {
        &self.base
    }
}
impl<E, L, N, V> std::ops::DerefMut for PedestrianEdge<E, L, N, V> {
    fn deref_mut(&mut self) -> &mut IntermodalEdge<E, L, N, V> {
        &mut self.base
    }
}

impl<E, L, N, V> PedestrianEdge<E, L, N, V>
where
    E: RouterEdge<V, Node = N, Lane = L> + 'static,
    L: 'static,
    N: 'static,
    V: RouterVehicle + 'static,
{
    /// C++: `void PedestrianEdge::PedestrianEdge<E, L, N, V>(int, const E *, const L *, bool, const double)`
    // PedestrianEdge(int numericalID, const E* edge, const L* lane, bool forward, const double pos = -1.) :
    //         IntermodalEdge<E, L, N, V>(edge->getID() + (edge->isWalkingArea() ? "" : (forward ? "_fwd" : "_bwd")) + toString(pos), numericalID, edge, "!ped"),
    //         myLane(lane),
    //         myForward(forward),
    //         myStartPos(pos >= 0 ? pos : (forward ? 0. : edge->getLength())),
    //         myIsOpposite(false) {
    //         if (!forward && (
    //                     edge->getFunction() == SumoXMLEdgeFunc::NORMAL
    //                     || edge->getFunction() == SumoXMLEdgeFunc::INTERNAL)) {
    //             const L* sidewalk = getSidewalk<E, L>(edge);
    //             if (sidewalk != nullptr && sidewalk->getPermissions() != SVC_PEDESTRIAN) {
    //                 // some non-pedestrian traffic is allowed
    //                 myIsOpposite = true;
    //             }
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn new(numericalID: i32, edge: Ptr<E>, lane: Ptr<L>, forward: bool, pos: f64) -> Self {
        let name = format!("{}{}{}", edge.id(), if edge.is_walking_area() { "" } else if forward { "_fwd" } else { "_bwd" }, to_string(&pos));
        let mut r = Self {
            base: IntermodalEdge::new(name, numericalID, edge, "!ped", -1.).with_kind(IntermodalEdgeKind::Pedestrian),
            myLane: lane,
            myForward: forward,
            myStartPos: if pos >= 0. { pos } else if forward { 0. } else { edge.length() },
            myIsOpposite: false,
            _p: std::marker::PhantomData,
        };
        if !forward && (edge.function() == SumoXMLEdgeFunc::NORMAL || edge.function() == SumoXMLEdgeFunc::INTERNAL) {
            let sidewalk = get_sidewalk::<E, L, V>(edge, SVC_PEDESTRIAN);
            if !sidewalk.is_null() && E::lane_permissions(sidewalk) != SVC_PEDESTRIAN {
                // some non-pedestrian traffic is allowed
                r.myIsOpposite = true;
            }
        }
        r
    }

    /// C++: `bool PedestrianEdge::includeInRoute(bool) const` [const]
    // bool includeInRoute(bool allEdges) const {
    //         return allEdges || (!this->getEdge()->isCrossing() && !this->getEdge()->isWalkingArea() && !this->getEdge()->isInternal());
    //     }
    #[allow(unused_variables)]
    pub fn include_in_route(&self, allEdges: bool) -> bool {
        if allEdges {
            true
        } else {
            let e = self.get_edge();
            !e.is_crossing() && !e.is_walking_area() && !e.is_internal()
        }
    }

    /// C++: `bool PedestrianEdge::prohibits(const IntermodalTrip<E, N, V> *const) const` [const]
    // bool prohibits(const IntermodalTrip<E, N, V>* const trip) const {
    //         if (trip->node == 0) {
    //             // network only includes IntermodalEdges
    //             return false;
    //         } else {
    //             // limit routing to the surroundings of the specified node
    //             return (this->getEdge()->getFromJunction() != trip->node
    //                     && this->getEdge()->getToJunction() != trip->node);
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn prohibits(&self, trip: Ptr<IntermodalTrip<E, N, V>>) -> bool {
        if trip.node.is_null() {
            false
        } else {
            let e = self.get_edge();
            e.from_junction() != trip.node && e.to_junction() != trip.node
        }
    }

    /// C++: `double PedestrianEdge::getPartialLength(const IntermodalTrip<E, N, V> *const) const` [const]
    // double getPartialLength(const IntermodalTrip<E, N, V>* const trip) const {
    //         double length = this->getLength();
    //         if (this->getEdge() == trip->from && !myForward && trip->departPos < myStartPos) {
    //             length = trip->departPos - (myStartPos - this->getLength());
    //         }
    //         if (this->getEdge() == trip->to && myForward && trip->arrivalPos < myStartPos + this->getLength()) {
    //             length = trip->arrivalPos - myStartPos;
    //         }
    //         if (this->getEdge() == trip->from && myForward && trip->departPos > myStartPos) {
    //             length -= (trip->departPos - myStartPos);
    //         }
    //         if (this->getEdge() == trip->to && !myForward && trip->arrivalPos > myStartPos - this->getLength()) {
    //             length -= (trip->arrivalPos - (myStartPos - this->getLength()));
    //         }
    //         // ensure that 'normal' edges always have a higher weight than connector edges
    //         length = MAX2(length, NUMERICAL_EPS);
    //         return length;
    //     }
    #[allow(unused_variables)]
    pub fn get_partial_length(&self, trip: Ptr<IntermodalTrip<E, N, V>>) -> f64 {
        let mut length = self.get_length();
        if self.get_edge() == trip.from && !self.myForward && trip.departPos < self.myStartPos {
            length = trip.departPos - (self.myStartPos - self.get_length());
        }
        if self.get_edge() == trip.to && self.myForward && trip.arrivalPos < self.myStartPos + self.get_length() {
            length = trip.arrivalPos - self.myStartPos;
        }
        if self.get_edge() == trip.from && self.myForward && trip.departPos > self.myStartPos {
            length -= trip.departPos - self.myStartPos;
        }
        if self.get_edge() == trip.to && !self.myForward && trip.arrivalPos > self.myStartPos - self.get_length() {
            length -= trip.arrivalPos - (self.myStartPos - self.get_length());
        }
        // ensure that 'normal' edges always have a higher weight than connector edges
        length = if length > utils::geom::NUMERICAL_EPS { length } else { utils::geom::NUMERICAL_EPS };
        length
    }

    /// C++: `double PedestrianEdge::getTravelTime(const IntermodalTrip<E, N, V> *const, double) const` [const]
    // double getTravelTime(const IntermodalTrip<E, N, V>* const trip, double time) const {
    //         const double length = getPartialLength(trip);
    //         double tlsDelay = 0;
    //         // @note pedestrian traffic lights should never have LINKSTATE_TL_REDYELLOW
    //         if (this->getEdge()->isCrossing()) {
    //             double timeToCrossing = (time - STEPS2TIME(trip->departTime));
    //             if (myLane->getIncomingLinkState() == LINKSTATE_TL_RED) {
    //                 // red traffic lights occurring later in the route may be green by the time we arrive
    //                 tlsDelay += MAX2(double(0), TL_RED_PENALTY - timeToCrossing);
    //             } else if (myLane->getIncomingLinkState() == LINKSTATE_TL_GREEN_MAJOR && timeToCrossing > TL_RED_PENALTY / 2) {
    //                 // green traffic lights occurring later in the route may be red by the time we arrive
    //                 tlsDelay += TL_RED_PENALTY;
    //             }
    //             tlsDelay += this->getEdge()->getTimePenalty();
    //         }
    // #ifdef IntermodalRouter_DEBUG_EFFORTS
    //         std::cout << " effort for " << trip->getID() << " at " << time << " edge=" << this->getID() << " effort=" << length / trip->speed + tlsDelay << " l=" << length << " fullLength=" << this->getLength() << " s=" << trip->speed << " tlsDelay=" << tlsDelay << "\n";
    // #endif
    //         return length / (trip->speed * (myIsOpposite ? gWeightsWalkOppositeFactor : 1)) + tlsDelay;
    //     }
    #[allow(unused_variables)]
    pub fn get_travel_time(&self, trip: Ptr<IntermodalTrip<E, N, V>>, time: f64) -> f64 {
        let length = self.get_partial_length(trip);
        let mut tlsDelay = 0.;
        // @note pedestrian traffic lights should never have LINKSTATE_TL_REDYELLOW
        let e = self.get_edge();
        if e.is_crossing() {
            let timeToCrossing = time - utils::common::time::steps2time(trip.departTime);
            let state = E::lane_incoming_link_state(self.myLane);
            if state == LINKSTATE_TL_RED {
                // red traffic lights occurring later in the route may be green by the time we arrive
                let penalty = TL_RED_PENALTY as f64 - timeToCrossing;
                tlsDelay += if 0. < penalty { penalty } else { 0. };
            } else if state == LINKSTATE_TL_GREEN_MAJOR && timeToCrossing > (TL_RED_PENALTY / 2) as f64 {
                // green traffic lights occurring later in the route may be red by the time we arrive
                tlsDelay += TL_RED_PENALTY as f64;
            }
            tlsDelay += e.time_penalty();
        }
        let oppositeFactor = if self.myIsOpposite { utils::common::std_defs::g_weights_walk_opposite_factor() } else { 1. };
        length / (trip.speed * oppositeFactor) + tlsDelay
    }

    /// C++: `double PedestrianEdge::getStartPos() const` [const]
    // double getStartPos() const {
    //         return myStartPos;
    //     }
    #[allow(unused_variables)]
    pub fn get_start_pos(&self) -> f64 {
        self.myStartPos
    }

    /// C++: `double PedestrianEdge::getEndPos() const` [const]
    // double getEndPos() const {
    //         return myForward ? myStartPos + this->getLength() : myStartPos - this->getLength();
    //     }
    #[allow(unused_variables)]
    pub fn get_end_pos(&self) -> f64 {
        if self.myForward { self.myStartPos + self.get_length() } else { self.myStartPos - self.get_length() }
    }

}

