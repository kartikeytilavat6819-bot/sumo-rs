//! Port of jtrrouter/ROJTRRouter.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Computes routes using junction turning percentages
/// C++ class `ROJTRRouter` (jtrrouter/ROJTRRouter.h)
#[repr(C)]
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
        Self {
            base: SUMOAbstractRouter::new("JTRRouter", unbuildIsWarningOnly, Some(<crate::router::ro_edge::ROEdge as crate::utils::router::router_edge::RouterEdge<crate::router::ro_vehicle::ROVehicle>>::travel_time_static), None, false, false),
            myUnbuildIsWarningOnly: unbuildIsWarningOnly,
            myAcceptAllDestination: acceptAllDestinations,
            myMaxEdges: maxEdges,
            myIgnoreClasses: ignoreClasses,
            myAllowLoops: allowLoops,
            myDiscountSources: discountSources,
        }
    }

    /// C++: `void ROJTRRouter::~ROJTRRouter() noexcept` [virtual]
    /// Destructor
    // ROJTRRouter::~ROJTRRouter() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `SUMOAbstractRouter<ROEdge, ROVehicle> * ROJTRRouter::clone()` [virtual]
    // virtual SUMOAbstractRouter<ROEdge, ROVehicle>* clone() {
    //         return new ROJTRRouter(myUnbuildIsWarningOnly, myAcceptAllDestination, myMaxEdges, myIgnoreClasses, myAllowLoops, myDiscountSources);
    //     }
    #[allow(unused_variables)]
    pub fn clone(&mut self) -> Ptr<SUMOAbstractRouter<ROEdge, ROVehicle>> {
        let router = ROJTRRouter::new(self.myUnbuildIsWarningOnly, self.myAcceptAllDestination, self.myMaxEdges, self.myIgnoreClasses, self.myAllowLoops, self.myDiscountSources);
        Ptr::from_raw(Box::into_raw(Box::new(router))).cast::<SUMOAbstractRouter<ROEdge, ROVehicle>>()
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
        let mut current: Ptr<ROJTREdge> = from.cast::<ROJTREdge>();
        if self.myDiscountSources && current.get_source_flow() <= 0 {
            return true;
        }
        let mut timeS: f64 = steps2time(time);
        let mut avoidEdges: BTreeSet<Ptr<ROEdge>> = BTreeSet::new();
        // route until a sinks has been found
        while !current.is_null()
            && current.cast::<ROEdge>() != to
            && (!current.is_sink() || current.cast::<ROEdge>() == from || current.get_source_flow() > 0)
            && (into.len() as i32) < self.myMaxEdges
        {
            into.push(current.cast::<ROEdge>());
            current.m().change_source_flow(-1);
            if !self.myAllowLoops {
                avoidEdges.insert(current.cast::<ROEdge>());
            }
            timeS += current.get_travel_time(vehicle, timeS);
            current = current.choose_next(if self.myIgnoreClasses { Ptr::null() } else { vehicle }, timeS, &avoidEdges);
            debug_assert!(self.myIgnoreClasses || current.is_null() || !current.prohibits(vehicle, false));
        }
        // check whether no valid ending edge was found
        if current.is_null() || (into.len() as i32) >= self.myMaxEdges {
            if self.myAcceptAllDestination {
                return true;
            } else {
                if !silent {
                    let msg = format!("The route starting at edge '{}' could not be closed.", from.get_id());
                    if self.myUnbuildIsWarningOnly {
                        utils::common::msg_handler::write_warning(&msg);
                    } else {
                        utils::common::msg_handler::write_error(&msg);
                    }
                }
                return false;
            }
        }
        // append the sink
        if !current.is_null() {
            into.push(current.cast::<ROEdge>());
        }
        return true;
    }

}

