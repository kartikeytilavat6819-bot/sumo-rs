//! Port of jtrrouter/ROJTRRouter.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Computes routes using junction turning percentages
/// C++ class `ROJTRRouter` (jtrrouter/ROJTRRouter.h)
pub struct ROJTRRouter {
    /// C++ base class `SUMOAbstractRouter<ROEdge, ROVehicle>`
    pub base: SUMOAbstractRouter<ROEdge, ROVehicle>,
    /// Whether unbuildable routes shall be reported as warniings, not errors
    pub myUnbuildIsWarningOnly: bool,
    /// Whether all edges may be used as route end
    pub myAcceptAllDestination: bool,
    /// The maximum number of edges a route may have
    pub myMaxEdges: i32,
    /// Whether vehicle class information shall be ignored
    pub myIgnoreClasses: bool,
    /// Whether a vehicle may reuse a road
    pub myAllowLoops: bool,
    /// Whether upstream flows shall be discounted from source flows
    pub myDiscountSources: bool,
}

impl std::ops::Deref for ROJTRRouter {
    type Target = SUMOAbstractRouter<ROEdge, ROVehicle>;
    fn deref(&self) -> &SUMOAbstractRouter<ROEdge, ROVehicle> {
        &self.base
    }
}
impl std::ops::DerefMut for ROJTRRouter {
    fn deref_mut(&mut self) -> &mut SUMOAbstractRouter<ROEdge, ROVehicle> {
        &mut self.base
    }
}

impl ROJTRRouter {
    /// C++: `void ROJTRRouter::ROJTRRouter(bool, bool, int, bool, bool, bool)`
    /// Constructor
    // ROJTRRouter::ROJTRRouter(bool unbuildIsWarningOnly, bool acceptAllDestinations,
    //                          int maxEdges, bool ignoreClasses,
    //                          bool allowLoops,
    //                          bool discountSources) :
    //     SUMOAbstractRouter<ROEdge, ROVehicle>("JTRRouter", unbuildIsWarningOnly, &ROEdge::getTravelTimeStatic, nullptr, false, false),
    //     myUnbuildIsWarningOnly(unbuildIsWarningOnly),
    //     myAcceptAllDestination(acceptAllDestinations), myMaxEdges(maxEdges),
    //     myIgnoreClasses(ignoreClasses),
    //     myAllowLoops(allowLoops),
    //     myDiscountSources(discountSources)
    // { }
    #[allow(unused_variables)]
    pub fn new(unbuildIsWarningOnly: bool, acceptAllDestinations: bool, maxEdges: i32, ignoreClasses: bool, allowLoops: bool, discountSources: bool) -> Self {
        todo!("port ROJTRRouter::ROJTRRouter")
    }

    /// C++: `void ROJTRRouter::~ROJTRRouter() noexcept` [virtual]
    /// Destructor
    // ROJTRRouter::~ROJTRRouter() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port ROJTRRouter::~ROJTRRouter")
    }

    /// C++: `SUMOAbstractRouter<ROEdge, ROVehicle> * ROJTRRouter::clone()` [virtual]
    // virtual SUMOAbstractRouter<ROEdge, ROVehicle>* clone() {
    //         return new ROJTRRouter(myUnbuildIsWarningOnly, myAcceptAllDestination, myMaxEdges, myIgnoreClasses, myAllowLoops, myDiscountSources);
    //     }
    #[allow(unused_variables)]
    pub fn clone(&mut self) -> Ptr<SUMOAbstractRouter<ROEdge, ROVehicle>> {
        todo!("port ROJTRRouter::clone")
    }

    /// C++: `bool ROJTRRouter::compute(const ROEdge *, const ROEdge *, const ROVehicle *const, SUMOTime, ConstROEdgeVector &, bool)` [virtual]
    /// Computes a route
    // bool
    // ROJTRRouter::compute(const ROEdge* from, const ROEdge* to,
    //                      const ROVehicle* const vehicle,
    //                      SUMOTime time, ConstROEdgeVector& into, bool silent) {
    //     const ROJTREdge* current = static_cast<const ROJTREdge*>(from);
    //     if (myDiscountSources && current->getSourceFlow() <= 0) {
    //         return true;
    //     }
    //     double timeS = STEPS2TIME(time);
    //     std::set<const ROEdge*> avoidEdges;
    //     // route until a sinks has been found
    //     while (current != nullptr && current != to &&
    //             (!current->isSink() || current == from || current->getSourceFlow() > 0) &&
    //             (int)into.size() < myMaxEdges) {
    //         into.push_back(current);
    //         const_cast<ROJTREdge*>(current)->changeSourceFlow(-1);
    //         if (!myAllowLoops) {
    //             avoidEdges.insert(current);
    //         }
    //         timeS += current->getTravelTime(vehicle, timeS);
    //         current = current->chooseNext(myIgnoreClasses ? nullptr : vehicle, timeS, avoidEdges);
    //         assert(myIgnoreClasses || current == 0 || !current->prohibits(vehicle));
    //     }
    //     // check whether no valid ending edge was found
    //     if (current == nullptr || (int) into.size() >= myMaxEdges) {
    //         if (myAcceptAllDestination) {
    //             return true;
    //         } else {
    //             if (!silent) {
    //                 MsgHandler* mh = myUnbuildIsWarningOnly ? MsgHandler::getWarningInstance() : MsgHandler::getErrorInstance();
    //                 mh->inform("The route starting at edge '" + from->getID() + "' could not be closed.");
    //             }
    //             return false;
    //         }
    //     }
    //     // append the sink
    //     if (current != nullptr) {
    //         into.push_back(current);
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn compute(&mut self, from: Ptr<ROEdge>, to: Ptr<ROEdge>, vehicle: Ptr<ROVehicle>, time: SUMOTime, into: &mut ConstROEdgeVector, silent: bool) -> bool {
        todo!("port ROJTRRouter::compute")
    }

}

