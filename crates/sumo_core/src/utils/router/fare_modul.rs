//! Port of utils/router/FareModul.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use crate::utils::router::fare_zones::{fare_zone_to_rep, get_overlay_zone};

/// The fare modul responsible for calculating prices
/// C++ class `FareModul` (utils/router/FareModul.h)
#[repr(C)]
pub struct FareModul {
    /// C++ base class `EffortCalculator`
    pub base: EffortCalculator,
    /// List of all fare states *
    pub myFareStates: i32,
    /// List of all edge line attributes *
    pub myEdges: i32,
    /// the fare zone this stop is a part of *
    pub myStopFareZone: i32,
    /// the faretoken that can be collected at this station *
    pub myStopFareToken: i32,
    /// the faretoken that is used when a trip is started at this station *
    pub myStopStartToken: i32,
    /// List of the prices *
    pub prices: Prices,
}

impl std::ops::Deref for FareModul {
    type Target = EffortCalculator;
    fn deref(&self) -> &EffortCalculator {
        &self.base
    }
}
impl std::ops::DerefMut for FareModul {
    fn deref_mut(&mut self) -> &mut EffortCalculator {
        &mut self.base
    }
}

impl FareModul {
    /// C++: `void FareModul::FareModul()`
    /// Constructor **
    // FareModul() :
    //         myFareStates()
    //     {}
    #[allow(unused_variables)]
    pub fn new() -> Self {
        todo!("port FareModul::FareModul")
    }

    /// C++: `void FareModul::init(const int &)`
    /// Implementation of EffortCalculator *
    // void init(const std::vector<std::string>& edges) override {
    //         myEdges = edges;
    //         myFareStates.resize(edges.size());
    //     }
    #[allow(unused_variables)]
    pub fn init(&mut self, edges: &i32) {
        todo!("port FareModul::init")
    }

    /// C++: `void FareModul::addStop(const int, const Parameterised &)` [virtual]
    // void addStop(const int stopEdge, const Parameterised& params) override {
    //         myStopFareZone[stopEdge] = StringUtils::toInt(params.getParameter("fareZone"));
    //         myStopFareToken[stopEdge] = FareUtil::stringToToken(params.getParameter("fareToken"));
    //         myStopStartToken[stopEdge] = FareUtil::stringToToken(params.getParameter("startToken"));
    //     }
    #[allow(unused_variables)]
    pub fn add_stop(&mut self, stopEdge: i32, params: &Parameterised) {
        todo!("port FareModul::addStop")
    }

    /// C++: `double FareModul::getEffort(const int) const` [const, virtual]
    /// Implementation of EffortCalculator *
    // double getEffort(const int numericalID) const override {
    //         double  effort = 0;
    //         FareState const& state =  myFareStates.at(numericalID);
    //         if (state.isValid()) {
    //             effort = state.myPriceDiff;
    //         } else {
    //             effort = std::numeric_limits<double>::max();
    //         }
    //         return effort;
    //     }
    #[allow(unused_variables)]
    pub fn get_effort(&self, numericalID: i32) -> f64 {
        todo!("port FareModul::getEffort")
    }

    /// C++: `void FareModul::update(const int, const int, const double)` [virtual]
    /// Implementation of EffortCalculator *
    // void update(const int edge, const int prev, const double length) override {
    //
    //         std::string const& edgeType = myEdges[edge];
    //
    //         //get propagated fare state
    //         FareState& state  = myFareStates.at(prev);
    //
    //         double oldPr;
    //         if (state.myFareToken == FareToken::START) {
    //             oldPr = 0;
    //         } else {
    //             oldPr = computePrice(state);
    //         }
    //         //treat  public transport edges
    //         if (edgeType.c_str()[0] != '!') {
    //             updateFareStatePublic(state, edge, length);
    //         } else if (edgeType ==  "!stop") {
    //             updateFareStateStop(state, edge);
    //         } else if (edgeType == "!ped") {
    //             updateFareStatePedestrian(state, edge);
    //         } else if (edgeType == "!access") {
    //             updateFareStateAccess(state, edge, prev);
    //         } else {
    //             updateFareState(state, edge);
    //         }
    //         FareState& stateAtE = myFareStates[edge];
    //         double newPr = computePrice(stateAtE);
    //         stateAtE.myPriceDiff = newPr - oldPr;
    //
    //         assert(stateAtE.myPriceDiff  >= 0);
    //
    //     }
    #[allow(unused_variables)]
    pub fn update(&mut self, edge: i32, prev: i32, length: f64) {
        todo!("port FareModul::update")
    }

    /// C++: `void FareModul::setInitialState(const int)` [virtual]
    /// Implementation of EffortCalculator _IntermodalEdge should be an Connector Edge *
    // void setInitialState(const int edge) override {
    // //    assert( edge->getLine() == "!connector");
    //
    //         myFareStates[edge] = FareState(FareToken::START);
    //
    //     }
    #[allow(unused_variables)]
    pub fn set_initial_state(&mut self, edge: i32) {
        todo!("port FareModul::setInitialState")
    }

    /// C++: `double FareModul::computePrice(const FareState &) const` [const, private]
    // double computePrice(FareState const& fareState) const {
    //         switch (fareState.myFareToken) {
    //             case FareToken ::H:
    //                 return prices.halle;
    //             case FareToken ::L:
    //                 return prices.leipzig;
    //             case FareToken ::T1:
    //                 return prices.t1;
    //             case FareToken ::T2:
    //                 return prices.t2;
    //             case FareToken ::T3:
    //                 return prices.t3;
    //             case FareToken::U:
    //                 return prices.zonePrices[0];
    //             case FareToken ::Z:
    //                 return prices.zonePrices[fareState.myCounter.numZones() - 1];
    //             case FareToken ::M:
    //                 return prices.maxPrice;
    //             case FareToken ::K:
    //                 return prices.shortTrip;
    //             case FareToken ::KL:
    //             case FareToken ::KLZ:
    //             case FareToken ::KLU:
    //                 return prices.shortTripLeipzig;
    //             case FareToken ::KH:
    //             case FareToken ::KHU:
    //             case FareToken ::KHZ:
    //                 return prices.shortTripHalle;
    //             case FareToken::Free:
    //                 return 1.4;
    //             case FareToken ::START:
    //                 return 0;
    //             case FareToken::ZU:
    //             case FareToken::None:
    //                 assert(false);
    //
    //         }
    //         return std::numeric_limits<double>::max();
    //     }
    #[allow(unused_variables)]
    pub fn compute_price(&self, fareState: &FareState) -> f64 {
        todo!("port FareModul::computePrice")
    }

    /// C++: `std::string FareModul::output(const int) const` [const, virtual, private]
    // std::string output(const int edge) const override {
    //
    //         FareState const&   my = myFareStates[edge];
    //         std::stringstream msg;
    //         /*
    //                 msg << "Final fare state at edge of type: " << myEdges[edge] << std::endl;
    //                 msg << "Faretoken" << FareUtil::tokenToString(my.myFareToken) << std::endl;
    //                 msg << "Price:" << computePrice(my) << std::endl;
    //                 msg << "Zones " << my.myCounter.numZones() << std::endl;
    //                 msg << "Stations: " << my.myVisistedStops << std::endl;
    //                 msg << "Distance:" << my.myTravelledDistance << std::endl;
    //         */
    //         msg << FareUtil::tokenToTicket(my.myFareToken) << " ";
    //         if (my.myFareToken == FareToken::Z) {
    //             msg << my.myCounter.numZones() << " ";
    //             if (my.myCounter.numZones() == 1) {
    //                 msg << "Zone";
    //             } else {
    //                 msg << "Zonen";
    //             }
    //
    //         } else if (my.myFareToken == FareToken::U) {
    //             msg << my.myCounter.numZones() << "1 Zone";
    //
    //         }
    //         msg << ":" << computePrice(my);
    //         return msg.str();
    //     }
    #[allow(unused_variables)]
    pub fn output(&self, edge: i32) -> String {
        todo!("port FareModul::output")
    }

    /// C++: `void FareModul::updateFareStateStop(const FareState &, const int)` [private]
    // void updateFareStateStop(FareState const& currentFareState, const int stopEdge) {
    //
    //         FareToken  collectedToken = myStopFareToken[stopEdge];
    //
    //         //if station has no fare information, just propagate
    //         if (collectedToken  == FareToken::None) {
    //             std::cout << "Propagating fare state for stop w/o a price!" << std::endl;
    //             return;
    //         }
    //
    //         FareToken const& token = currentFareState.myFareToken;
    //
    //         FareState& stateAtE = myFareStates[stopEdge];
    //
    //         stateAtE = currentFareState;
    //
    //         stateAtE.myCounter.addZone(myStopFareZone[stopEdge]);
    //
    //         stateAtE.myVisistedStops++;
    //
    //         switch (token) {
    //             case FareToken ::Free:
    //                 stateAtE.myFareToken = myStopStartToken[stopEdge];
    //                 break;
    //             case FareToken::M :
    //                 break;
    //
    //             case FareToken::Z :
    //                 if (stateAtE.myCounter.numZones() > 6) {
    //                     stateAtE.myFareToken = FareToken::M;
    //                 }
    //                 break;
    //
    //             case FareToken::T1 :
    //             case FareToken::T2 :
    //             case FareToken::T3 :
    //                 if (collectedToken == FareToken::Z) {
    //                     stateAtE.myFareToken = stateAtE.myTravelledDistance <= 4000 ? FareToken::K : FareToken::Z;
    //                 }
    //                 break;
    //             case FareToken::U :
    //                 if (collectedToken == FareToken::H) {
    //                     stateAtE.myFareToken = FareToken::H;
    //                 }
    //                 if (collectedToken == FareToken::L) {
    //                     stateAtE.myFareToken = FareToken::L;
    //                 }
    //                 if (collectedToken == FareToken::Z) {
    //                     stateAtE.myFareToken = FareToken::Z;
    //                 }
    //                 break;
    //             case FareToken::H:
    //             case FareToken::L:
    //                 if (collectedToken == FareToken::Z) {
    //                     stateAtE.myFareToken = FareToken::Z;
    //                 }
    //                 break;
    //             case FareToken::KH:
    //                 if (stateAtE.myVisistedStops <= 4) {
    //                     if (collectedToken == FareToken::U) {
    //                         stateAtE.myFareToken = FareToken::KHU;
    //                     }
    //                     if (collectedToken == FareToken::Z) {
    //                         stateAtE.myFareToken = FareToken::KHZ;
    //                     }
    //                 } else {
    //                     if (collectedToken == FareToken::H) {
    //                         stateAtE.myFareToken = FareToken ::H;
    //                     }
    //                     if (collectedToken == FareToken::Z) {
    //                         stateAtE.myFareToken = FareToken ::Z;
    //                     }
    //                     if (collectedToken == FareToken::U) {
    //                         stateAtE.myFareToken = FareToken ::U;
    //                     }
    //                 }
    //                 break;
    //             case FareToken::KL:
    //                 if (stateAtE.myVisistedStops <= 4) {
    //                     if (collectedToken == FareToken::U) {
    //                         stateAtE.myFareToken = FareToken::KLU;
    //                     }
    //                     if (collectedToken == FareToken::Z) {
    //                         stateAtE.myFareToken = FareToken::KLZ;
    //                     }
    //                 } else {
    //                     if (collectedToken == FareToken::L) {
    //                         stateAtE.myFareToken = FareToken ::L;
    //                     }
    //                     if (collectedToken == FareToken::Z) {
    //                         stateAtE.myFareToken = FareToken ::Z;
    //                     }
    //                     if (collectedToken == FareToken::U) {
    //                         stateAtE.myFareToken = FareToken ::U;
    //                     }
    //                 }
    //                 break;
    //             case FareToken::K:
    //                 if (stateAtE.myTravelledDistance > 4000) {
    //                     if (collectedToken == FareToken::U) {
    //                         stateAtE.myFareToken = FareToken ::U;
    //                     }
    //                     if (collectedToken == FareToken::Z) {
    //                         stateAtE.myFareToken = FareToken ::Z;
    //                     }
    //                 }
    //                 break;
    //             case FareToken::KHU :
    //             case FareToken::KLU :
    //                 if (stateAtE.myVisistedStops > 4) {
    //                     if (collectedToken == FareToken::U) {
    //                         stateAtE.myFareToken = FareToken::U;
    //                     }
    //                 }
    //                 break;
    //
    //             case FareToken::KLZ:
    //             case FareToken::KHZ:
    //                 if (stateAtE.myVisistedStops > 4) {
    //                     if (collectedToken == FareToken::Z) {
    //                         stateAtE.myFareToken = FareToken::Z;
    //                     }
    //                 }
    //                 break;
    //             case FareToken::ZU :
    //                 assert(false);
    //                 if (collectedToken == FareToken::U) {
    //                     stateAtE.myFareToken = FareToken::U;
    //                 } else {
    //                     stateAtE.myFareToken = FareToken::Z;
    //                 }
    //
    //                 break;
    //             default:
    //                 std::cout << "Reached invalid position in fareToken selection!" << std::endl;
    //                 assert(false);
    //                 break;
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn update_fare_state_stop(&mut self, currentFareState: &FareState, stopEdge: i32) {
        todo!("port FareModul::updateFareStateStop")
    }

    /// C++: `void FareModul::updateFareStatePedestrian(const FareState &, const int)` [private]
    // void updateFareStatePedestrian(FareState const& currentFareState, const int pedestrianEdge) {
    //
    //         //only propagates the fare state
    //         FareState& stateAtE = myFareStates[pedestrianEdge];
    //
    //         stateAtE = currentFareState;
    //
    //         if (currentFareState.myFareToken == FareToken::START) {
    //             stateAtE.myFareToken = FareToken::Free;
    //         }
    //
    //     }
    #[allow(unused_variables)]
    pub fn update_fare_state_pedestrian(&mut self, currentFareState: &FareState, pedestrianEdge: i32) {
        todo!("port FareModul::updateFareStatePedestrian")
    }

    /// C++: `void FareModul::updateFareStatePublic(const FareState &, const int, const double)` [private]
    // void updateFareStatePublic(FareState const& currentFareState, const int publicTransportEdge, const double length) {
    //
    //
    //         if (currentFareState.myFareToken == FareToken::None) {
    //             return;
    //         }
    //
    //         FareState& stateAtE = myFareStates[publicTransportEdge];
    //
    //         stateAtE = currentFareState;
    //         stateAtE.myTravelledDistance += length;
    //     }
    #[allow(unused_variables)]
    pub fn update_fare_state_public(&mut self, currentFareState: &FareState, publicTransportEdge: i32, length: f64) {
        todo!("port FareModul::updateFareStatePublic")
    }

    /// C++: `void FareModul::updateFareState(const FareState &, const int)` [private]
    // void updateFareState(FareState const& currentFareState, const int intermodalEdge) {
    //
    //         if (currentFareState.myFareToken == FareToken::None) {
    //             return;
    //         }
    //
    //         FareState& stateAtE = myFareStates[intermodalEdge];
    //
    //         stateAtE = currentFareState;
    //
    //         if (currentFareState.myFareToken == FareToken::START) {
    //             stateAtE.myFareToken = FareToken::Free;
    //         }
    //
    //     }
    #[allow(unused_variables)]
    pub fn update_fare_state(&mut self, currentFareState: &FareState, intermodalEdge: i32) {
        todo!("port FareModul::updateFareState")
    }

    /// C++: `void FareModul::updateFareStateAccess(const FareState &, const int, const int)` [private]
    // void updateFareStateAccess(FareState const& currentFareState, const int accessEdge, const int prev) {
    //
    //         FareToken const& token = currentFareState.myFareToken;
    //
    //         FareState& stateAtE = myFareStates[accessEdge];
    //
    //         stateAtE = currentFareState;
    //
    //         if (currentFareState.myFareToken == FareToken::START) {
    //             stateAtE.myFareToken = FareToken::Free;
    //         }
    //
    //         if (myEdges[prev] == "!ped") {
    //             switch (token) {
    //
    //                 case FareToken::Free ://we have not yet taken public transport
    //                     break;
    //                 case  FareToken::K :
    //                     if (currentFareState.myCounter.numZones() == 0) {
    //                         stateAtE.myFareToken = FareToken::U;
    //                     } else {
    //                         stateAtE.myFareToken = FareToken::Z;
    //                     }
    //                     break;
    //                 case  FareToken::KH :
    //                     stateAtE.myFareToken = FareToken::H;
    //                     break;
    //                 case  FareToken::KL :
    //                     stateAtE.myFareToken = FareToken::L;
    //                     break;
    //                 case  FareToken::KLU :
    //                     stateAtE.myFareToken = FareToken::L;
    //                     break;
    //                 case  FareToken::KHU:
    //                     stateAtE.myFareToken = FareToken::H;
    //                     break;
    //                 case  FareToken::KLZ :
    //                     stateAtE.myFareToken = FareToken::Z;
    //                     break;
    //                 case  FareToken::KHZ:
    //                     stateAtE.myFareToken = FareToken::Z;
    //                     break;
    //                 default:
    //                     return;
    //             }
    //         }
    //
    //     }
    #[allow(unused_variables)]
    pub fn update_fare_state_access(&mut self, currentFareState: &FareState, accessEdge: i32, prev: i32) {
        todo!("port FareModul::updateFareStateAccess")
    }

}

/// A fare state collects all the information that is necessary to compute the price. Is used as an edge label in IntermodalRouter
/// C++ class `FareState` (utils/router/FareModul.h)
pub struct FareState {
    /// fare token *
    pub myFareToken: FareToken,
    /// zone counter *
    pub myCounter: ZoneCounter,
    /// travelled distance in km*
    pub myTravelledDistance: f64,
    /// num of visited stops*
    pub myVisistedStops: i32,
    /// price diff to previous edge *
    pub myPriceDiff: f64,
}

impl FareState {
    /// C++: `void FareState::FareState()`
    /// default constructor for unlabeled edges*
    // explicit  FareState():
    //         myFareToken(FareToken::None),
    //         myCounter(std::numeric_limits<int>::max()),
    //         myTravelledDistance(std::numeric_limits<double>::max()),
    //         myVisistedStops(std::numeric_limits<int>::max()),
    //         myPriceDiff(0) {
    //     }
    #[allow(unused_variables)]
    pub fn new() -> Self {
        FareState {
            myFareToken: FareToken::None,
            myCounter: ZoneCounter::new(i32::MAX as u32),
            myTravelledDistance: f64::MAX,
            myVisistedStops: i32::MAX,
            myPriceDiff: 0.0,
        }
    }

    /// C++: `void FareState::FareState(FareToken)`
    // explicit FareState(FareToken token):
    //         myFareToken(token),
    //         myCounter(0),
    //         myTravelledDistance(0),
    //         myVisistedStops(0),
    //         myPriceDiff(0) {}
    #[allow(unused_variables)]
    pub fn new_2(token: FareToken) -> Self {
        FareState {
            myFareToken: token,
            myCounter: ZoneCounter::new(0),
            myTravelledDistance: 0.0,
            myVisistedStops: 0,
            myPriceDiff: 0.0,
        }
    }

    /// C++: `void FareState::~FareState() noexcept`
    /// Destructor *
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // implicit destructor: nothing to release
    }

    /// C++: `bool FareState::isValid() const` [const]
    /// returns true if fare state is set and not on default
    // bool isValid() const {
    //         return !(myFareToken == FareToken::None);
    //     }
    #[allow(unused_variables)]
    pub fn is_valid(&self) -> bool {
        self.myFareToken != FareToken::None
    }

}

/// C++ class `Prices` (utils/router/FareModul.h)
pub struct Prices {
    /// Prices for zones *
    pub zonePrices: i32,
    pub halle: f64,
    pub leipzig: f64,
    pub t1: f64,
    pub t2: f64,
    pub t3: f64,
    pub shortTrip: f64,
    pub shortTripLeipzig: f64,
    pub shortTripHalle: f64,
    pub maxPrice: f64,
}

impl Prices {
}

/// C++ class `ZoneCounter` (utils/router/FareModul.h)
pub struct ZoneCounter {
    pub myCount: i64,
}

impl ZoneCounter {
    /// C++: `void ZoneCounter::ZoneCounter(unsigned int)`
    // explicit ZoneCounter(unsigned int ct) :
    //         myCount(ct) {
    //
    //     }
    #[allow(unused_variables)]
    pub fn new(ct: u32) -> Self {
        ZoneCounter { myCount: ct as i64 }
    }

    /// C++: `void ZoneCounter::addZone(int)`
    // inline void addZone(int zoneNumber) {
    //         zoneNumber = getOverlayZone(zoneNumber);
    //         if (zoneNumber == 0) {
    //             return;
    //         }
    //         long long int repNum = fareZoneToRep[zoneNumber];
    //         //assert power of 2
    //         if (bitcount(repNum) == 0) {
    //             return;
    //         }
    //         myCount = myCount | repNum;
    //     }
    #[allow(unused_variables)]
    pub fn add_zone(&mut self, zoneNumber: i32) {
        let zoneNumber = get_overlay_zone(zoneNumber);
        if zoneNumber == 0 {
            return;
        }
        let repNum = fare_zone_to_rep(zoneNumber);
        // assert power of 2
        if self.bitcount(repNum) == 0 {
            return;
        }
        self.myCount = self.myCount | repNum;
    }

    /// C++: `int ZoneCounter::numZones() const` [const]
    // int numZones() const {
    //         return bitcount(myCount);
    //     }
    #[allow(unused_variables)]
    pub fn num_zones(&self) -> i32 {
        self.bitcount(self.myCount)
    }

    /// C++: `int ZoneCounter::bitcount(long long) const` [const, private]
    // inline int bitcount(long long int intVal) const {
    //         int count = 0;
    //         long long int counter = intVal;
    //
    //         while (counter != 0) {
    //             counter = counter & (counter - 1);
    //             ++count;
    //         }
    //         return count;
    //     }
    #[allow(unused_variables)]
    pub fn bitcount(&self, intVal: i64) -> i32 {
        let mut count = 0;
        let mut counter = intVal;
        while counter != 0 {
            counter = counter & counter.wrapping_sub(1); // C++ signed overflow at LLONG_MIN is UB; wrap instead
            count += 1;
        }
        count
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zone_counter_counts_distinct_zones() {
        let mut c = ZoneCounter::new(0);
        assert_eq!(c.num_zones(), 0);
        c.add_zone(110); // rep 1
        c.add_zone(121); // rep 2
        c.add_zone(121); // same bit again
        assert_eq!(c.num_zones(), 2);
        c.add_zone(511); // overlay -> 165 -> rep 2^30
        assert_eq!(c.num_zones(), 3);
        c.add_zone(999); // unknown zone maps to rep 0, ignored
        assert_eq!(c.num_zones(), 3);
    }

    #[test]
    fn bitcount_matches_popcount() {
        let c = ZoneCounter::new(0);
        assert_eq!(c.bitcount(7), 3);
        assert_eq!(c.bitcount(0), 0);
        assert_eq!(c.bitcount(i64::MIN), 1);
    }

    #[test]
    fn fare_state_validity() {
        assert!(!FareState::new().is_valid());
        assert!(FareState::new_2(FareToken::H).is_valid());
    }
}
