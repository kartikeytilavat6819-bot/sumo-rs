//! Port of utils/router/PedestrianRouter.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `IntermodalEdge<E, L, N, V> PedestrianRouter::_IntermodalEdge`
pub type PedestrianRouter__IntermodalEdge = Unmapped /* IntermodalEdge<E, L, N, V> */;

/// C++ typedef `IntermodalNetwork<E, L, N, V> PedestrianRouter::_IntermodalNetwork`
pub type PedestrianRouter__IntermodalNetwork = Unmapped /* IntermodalNetwork<E, L, N, V> */;

/// C++ typedef `IntermodalTrip<E, N, V> PedestrianRouter::_IntermodalTrip`
pub type PedestrianRouter__IntermodalTrip = Unmapped /* IntermodalTrip<E, N, V> */;

/// C++ typedef `DijkstraRouter<_IntermodalEdge, _IntermodalTrip> PedestrianRouter::_InternalRouter`
pub type PedestrianRouter__InternalRouter = DijkstraRouter<AccessEdge__IntermodalEdge, PedestrianRouter__IntermodalTrip>;

/// The router for pedestrians (on a bidirectional network of sidewalks and crossings)
/// C++ class `PedestrianRouter` (utils/router/PedestrianRouter.h)
#[repr(C)]
pub struct PedestrianRouter<E, L, N, V> {
    /// C++ base class `SUMOAbstractRouter<E, IntermodalTrip<E, N, V>>`
    pub base: SUMOAbstractRouter<E, IntermodalTrip<E, N, V>>,
    pub myAmClone: bool,
    pub myInternalRouter: Ptr<PedestrianRouter__InternalRouter>,
    pub myPedNet: Ptr<PedestrianRouter__IntermodalNetwork>,
    _p: std::marker::PhantomData<(E, L, N, V,)>,
}

impl<E, L, N, V> std::ops::Deref for PedestrianRouter<E, L, N, V> {
    type Target = SUMOAbstractRouter<E, IntermodalTrip<E, N, V>>;
    fn deref(&self) -> &SUMOAbstractRouter<E, IntermodalTrip<E, N, V>> {
        &self.base
    }
}
impl<E, L, N, V> std::ops::DerefMut for PedestrianRouter<E, L, N, V> {
    fn deref_mut(&mut self) -> &mut SUMOAbstractRouter<E, IntermodalTrip<E, N, V>> {
        &mut self.base
    }
}

impl<E, L, N, V> PedestrianRouter<E, L, N, V> {
    /// C++: `void PedestrianRouter::PedestrianRouter<E, L, N, V>()`
    /// Constructor
    // PedestrianRouter():
    //         SUMOAbstractRouter<E, _IntermodalTrip>("PedestrianRouter", true, nullptr, nullptr, false, false), myAmClone(false) {
    //         myPedNet = new _IntermodalNetwork(E::getAllEdges(), true);
    //         myInternalRouter = new _InternalRouter(myPedNet->getAllEdges(), true,
    //                                                gWeightsRandomFactor > 1 ? &_IntermodalEdge::getTravelTimeStaticRandomized : &_IntermodalEdge::getTravelTimeStatic,
    //                                                nullptr, false, nullptr, true);
    //     }
    #[allow(unused_variables)]
    pub fn new() -> Self {
        todo!("port PedestrianRouter::PedestrianRouter<E, L, N, V>")
    }

    /// C++: `void PedestrianRouter::PedestrianRouter<E, L, N, V>(_IntermodalNetwork *)`
    // PedestrianRouter(_IntermodalNetwork* net):
    //         SUMOAbstractRouter<E, _IntermodalTrip>("PedestrianRouterClone", true, nullptr, nullptr, false, false), myAmClone(true) {
    //         myPedNet = net;
    //         myInternalRouter = new _InternalRouter(myPedNet->getAllEdges(), true,
    //                                                gWeightsRandomFactor > 1 ? &_IntermodalEdge::getTravelTimeStaticRandomized : &_IntermodalEdge::getTravelTimeStatic,
    //                                                nullptr, false, nullptr, true);
    //     }
    #[allow(unused_variables)]
    pub fn new_2(net: Ptr<PedestrianRouter__IntermodalNetwork>) -> Self {
        todo!("port PedestrianRouter::PedestrianRouter<E, L, N, V>")
    }

    /// C++: `void PedestrianRouter::~PedestrianRouter<E, L, N, V>()` [virtual]
    /// Destructor
    // virtual ~PedestrianRouter() {
    //         delete myInternalRouter;
    //         if (!myAmClone) {
    //             delete myPedNet;
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port PedestrianRouter::~PedestrianRouter<E, L, N, V>")
    }

    /// C++: `SUMOAbstractRouter<E, _IntermodalTrip> * PedestrianRouter::clone()` [virtual]
    // virtual SUMOAbstractRouter<E, _IntermodalTrip>* clone() {
    //         return new PedestrianRouter<E, L, N, V>(myPedNet);
    //     }
    #[allow(unused_variables)]
    pub fn clone(&mut self) -> Ptr<SUMOAbstractRouter<E, PedestrianRouter__IntermodalTrip>> {
        todo!("port PedestrianRouter::clone")
    }

    /// C++: `double PedestrianRouter::compute(const E *, const E *, double, double, double, SUMOTime, const N *, const SUMOVTypeParameter &, std::vector<const E *> &, bool) const` [const]
    /// Builds the route between the given edges using the minimum effort at the given time The definition of the effort depends on the wished routing scheme
    // double compute(const E* from, const E* to, double departPos, double arrivalPos, double speed,
    //                    SUMOTime msTime, const N* onlyNode, const SUMOVTypeParameter& pars, std::vector<const E*>& into, bool allEdges = false) const {
    //         if (getSidewalk<E, L>(from) == 0) {
    //             WRITE_WARNINGF(TL("Departure edge '%' does not allow pedestrians."), from->getID());
    //             return false;
    //         }
    //         if (getSidewalk<E, L>(to) == 0) {
    //             WRITE_WARNINGF(TL("Destination edge '%' does not allow pedestrians."), to->getID());
    //             return false;
    //         }
    //         _IntermodalTrip trip(from, to, departPos, arrivalPos, speed, msTime, onlyNode, pars);
    //         std::vector<const _IntermodalEdge*> intoPed;
    //         const bool silent = allEdges; // no warning is needed when called from MSPModel_Striping
    //         const bool success = myInternalRouter->compute(myPedNet->getDepartConnector(from),
    //                              myPedNet->getArrivalConnector(to),
    //                              &trip, msTime, intoPed, silent);
    //         double time = 0.;
    //         if (success) {
    //             for (const _IntermodalEdge* pedEdge : intoPed) {
    //                 if (pedEdge->includeInRoute(allEdges)) {
    //                     into.push_back(pedEdge->getEdge());
    //                 }
    //                 time += myInternalRouter->getEffort(pedEdge, &trip, time);
    //             }
    //         }
    // #ifdef PedestrianRouter_DEBUG_ROUTES
    //         std::cout << TIME2STEPS(msTime) << " trip from " << from->getID() << " to " << to->getID()
    //                   << " departPos=" << departPos
    //                   << " arrivalPos=" << arrivalPos
    //                   << " onlyNode=" << (onlyNode == 0 ? "NULL" : onlyNode->getID())
    //                   << " edges=" << toString(intoPed)
    //                   << " resultEdges=" << toString(into)
    //                   << " time=" << time
    //                   << "\n";
    // #endif
    //         return success ? time : -1.;
    //     }
    #[allow(unused_variables)]
    pub fn compute(&self, from: Ptr<E>, to: Ptr<E>, departPos: f64, arrivalPos: f64, speed: f64, msTime: SUMOTime, onlyNode: Ptr<N>, pars: &SUMOVTypeParameter, into: &mut Vec<Ptr<E>>, allEdges: bool) -> f64 {
        todo!("port PedestrianRouter::compute")
    }

    /// C++: `bool PedestrianRouter::compute(const E *, const E *, const _IntermodalTrip *const, SUMOTime, std::vector<const E *> &, bool)`
    /// Builds the route between the given edges using the minimum effort at the given time The definition of the effort depends on the wished routing scheme
    // bool compute(const E*, const E*, const _IntermodalTrip* const,
    //                  SUMOTime, std::vector<const E*>&, bool) {
    //         throw ProcessError(TL("Do not use this method"));
    //     }
    #[allow(unused_variables)]
    pub fn compute_2(&mut self, arg0: Ptr<E>, arg1: Ptr<E>, arg2: Ptr<PedestrianRouter__IntermodalTrip>, arg3: SUMOTime, arg4: &mut Vec<Ptr<E>>, arg5: bool) -> bool {
        todo!("port PedestrianRouter::compute")
    }

    /// C++: `void PedestrianRouter::prohibit(const std::map<const E *, RouterProhibition> &)`
    // void prohibit(const std::map<const E*, RouterProhibition>& toProhibit) {
    //         typename _InternalRouter::Prohibitions toProhibitPE;
    //         for (auto item : toProhibit) {
    //             toProhibitPE[myPedNet->getBothDirections(item.first).first] = item.second;
    //             toProhibitPE[myPedNet->getBothDirections(item.first).second] = item.second;
    //         }
    //         myInternalRouter->prohibit(toProhibitPE);
    //     }
    #[allow(unused_variables)]
    pub fn prohibit(&mut self, toProhibit: &BTreeMap<Ptr<E>, RouterProhibition>) {
        todo!("port PedestrianRouter::prohibit")
    }

    /// C++: `double PedestrianRouter::recomputeWalkCosts(const std::vector<const E *> &, double, double, double, SUMOTime, const SUMOVTypeParameter &, double &) const` [const]
    // double recomputeWalkCosts(const std::vector<const E*>& edges, double speed, double fromPos, double toPos, SUMOTime msTime, const SUMOVTypeParameter& pars, double& length) const {
    //         // edges are normal edges so we need to reconstruct paths across intersection
    //         if (edges.size() == 0) {
    //             length = 0;
    //             return 0;
    //         } else if (edges.size() == 1) {
    //             length = fabs(toPos - fromPos);
    //             return length / speed;
    //         } else {
    //             double cost = 0;
    //             int last = (int)edges.size() - 1;
    //             for (int i = 0; i < last; i++) {
    //                 std::vector<const E*> into;
    //                 const E* from = edges[i];
    //                 const E* to = edges[i + 1];
    //                 const double fp = (i == 0 ? fromPos : from->getLength() / 2);
    //                 const double tp = (i == (last - 1) ? toPos : to->getLength() / 2);
    //                 const N* node = getCommonNode(from, to);
    //                 if (i == 0) {
    //                     if (node == from->getToJunction()) {
    //                         length += from->getLength() - fromPos;
    //                     } else {
    //                         length += fromPos;
    //                     }
    //                 } else  {
    //                     length += from->getLength();
    //                 }
    //                 if (i == (last - 1)) {
    //                     if (node == to->getFromJunction()) {
    //                         length += toPos;
    //                     } else {
    //                         length += to->getLength() - toPos;
    //                     }
    //                 }
    //                 double time = this->compute(from, to, fp, tp, speed, msTime, node, pars, into, true);
    //                 if (time >= 0) {
    //                     cost += time;
    //                     for (const E* edge : into) {
    //                         if (edge->isCrossing()) {
    //                             length += edge->getLength();
    //                         } else if (edge->isWalkingArea()) {
    //                             // this is wrong because the length is path-dependent
    //                             length += edge->getLength();
    //                         }
    //                     }
    //                 } else {
    //                     throw ProcessError("Could not compute cost between edge '" + from->getID() + "' and edge '" + to->getID() + "'.");
    //                 }
    //             }
    //             return cost;
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn recompute_walk_costs(&self, edges: &Vec<Ptr<E>>, speed: f64, fromPos: f64, toPos: f64, msTime: SUMOTime, pars: &SUMOVTypeParameter, length: &mut f64) -> f64 {
        todo!("port PedestrianRouter::recomputeWalkCosts")
    }

    /// C++: `const N * PedestrianRouter::getCommonNode(const E *, const E *) const` [const, private]
    // const N* getCommonNode(const E* from, const E* to) const {
    //         if (from->getToJunction() == to->getFromJunction() || from->getToJunction() == to->getToJunction()) {
    //             return from->getToJunction();
    //         } else if (from->getFromJunction() == to->getFromJunction() || from->getFromJunction() == to->getToJunction()) {
    //             return from->getFromJunction();
    //         } else {
    //             return nullptr;
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn get_common_node(&self, from: Ptr<E>, to: Ptr<E>) -> Ptr<N> {
        todo!("port PedestrianRouter::getCommonNode")
    }

    /// C++: `PedestrianRouter<E, L, N, V> & PedestrianRouter::operator=(const PedestrianRouter<E, L, N, V> &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &PedestrianRouter<E, L, N, V>) -> &mut PedestrianRouter<E, L, N, V> {
        todo!("port PedestrianRouter::operator=")
    }

}

