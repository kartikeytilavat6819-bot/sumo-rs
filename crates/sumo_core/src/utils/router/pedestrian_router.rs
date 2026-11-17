//! Port of utils/router/PedestrianRouter.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use crate::utils::router::router_edge::{RouterEdge, RouterNode, RouterVehicle};
use crate::utils::router::intermodal_edge::get_sidewalk;
use crate::utils::router::intermodal_network::IntermodalNetwork;

/// C++ typedef `IntermodalEdge<E, L, N, V> PedestrianRouter::_IntermodalEdge`
pub type PedestrianRouter__IntermodalEdge = Unmapped /* IntermodalEdge<E, L, N, V> */;

/// C++ typedef `IntermodalNetwork<E, L, N, V> PedestrianRouter::_IntermodalNetwork`
pub type PedestrianRouter__IntermodalNetwork = Unmapped /* IntermodalNetwork<E, L, N, V> */;

/// C++ typedef `IntermodalTrip<E, N, V> PedestrianRouter::_IntermodalTrip`
pub type PedestrianRouter__IntermodalTrip<E, N, V> = IntermodalTrip<E, N, V>;

/// C++ typedef `DijkstraRouter<_IntermodalEdge, _IntermodalTrip> PedestrianRouter::_InternalRouter`
pub type PedestrianRouter__InternalRouter<E, L, N, V> = DijkstraRouter<AccessEdge__IntermodalEdge<E, L, N, V>, PedestrianRouter__IntermodalTrip<E, N, V>>;

/// The router for pedestrians (on a bidirectional network of sidewalks and crossings)
/// C++ class `PedestrianRouter` (utils/router/PedestrianRouter.h)
#[repr(C)]
pub struct PedestrianRouter<E, L, N, V> {
    /// C++ base class `SUMOAbstractRouter<E, IntermodalTrip<E, N, V>>`
    pub base: SUMOAbstractRouter<E, IntermodalTrip<E, N, V>>,
    pub myAmClone: bool,
    pub myInternalRouter: Ptr<DijkstraRouter<IntermodalEdge<E, L, N, V>, IntermodalTrip<E, N, V>>>,
    pub myPedNet: Ptr<IntermodalNetwork<E, L, N, V>>,
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

impl<E, L, N, V> PedestrianRouter<E, L, N, V>
where
    E: RouterEdge<V, Node = N, Lane = L> + 'static,
    N: RouterNode<Edge = E> + 'static,
    V: RouterVehicle + 'static,
    L: 'static,
{
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
    pub fn new() -> Self where E: RouterEdge<V, Node = N>, V: RouterVehicle {
        let net: Ptr<IntermodalNetwork<E, L, N, V>> = Ptr::from_raw(Box::into_raw(Box::new(IntermodalNetwork::<E, L, N, V>::new(&E::all_edges(), true, 0))));
        let edges: Vec<Ptr<IntermodalEdge<E, L, N, V>>> = net.m().get_all_edges().clone();
        let internal = DijkstraRouter::<IntermodalEdge<E, L, N, V>, IntermodalTrip<E, N, V>>::new(&edges, true, Some(IntermodalEdge::<E, L, N, V>::get_travel_time_static), None, false, Ptr::null(), true, false);
        Self {
            base: SUMOAbstractRouter::new("PedestrianRouter", true, None, None, false, false),
            myAmClone: false,
            myInternalRouter: Ptr::from_raw(Box::into_raw(Box::new(internal))),
            myPedNet: net,
            _p: std::marker::PhantomData,
        }
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
    pub fn new_2(net: Ptr<IntermodalNetwork<E, L, N, V>>) -> Self {
        // shortcut: gWeightsRandomFactor > 1 selects getTravelTimeStaticRandomized in C++; only the plain getTravelTimeStatic is wired here, upgrade when the random factor option is read here
        let edges: Vec<Ptr<IntermodalEdge<E, L, N, V>>> = net.m().get_all_edges().iter().map(|e| (*e).cast()).collect();
        let internal = DijkstraRouter::<IntermodalEdge<E, L, N, V>, IntermodalTrip<E, N, V>>::new(&edges, true, Some(IntermodalEdge::<E, L, N, V>::get_travel_time_static), None, false, Ptr::null(), true, false);
        Self {
            base: SUMOAbstractRouter::new("PedestrianRouterClone", true, None, None, false, false),
            myAmClone: true,
            myInternalRouter: Ptr::from_raw(Box::into_raw(Box::new(internal))),
            myPedNet: net,
            _p: std::marker::PhantomData,
        }
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
        drop(unsafe { Box::from_raw(self.myInternalRouter.as_raw()) });
        if !self.myAmClone {
            drop(unsafe { Box::from_raw(self.myPedNet.as_raw()) });
        }
    }

    /// C++: `SUMOAbstractRouter<E, _IntermodalTrip> * PedestrianRouter::clone()` [virtual]
    // virtual SUMOAbstractRouter<E, _IntermodalTrip>* clone() {
    //         return new PedestrianRouter<E, L, N, V>(myPedNet);
    //     }
    #[allow(unused_variables)]
    pub fn clone(&mut self) -> Ptr<SUMOAbstractRouter<E, IntermodalTrip<E, N, V>>> {
        Ptr::from_raw(Box::into_raw(Box::new(PedestrianRouter::new_2(self.myPedNet)))).cast()
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
    pub fn compute(&self, from: Ptr<E>, to: Ptr<E>, departPos: f64, arrivalPos: f64, speed: f64, msTime: SUMOTime, onlyNode: Ptr<N>, pars: &SUMOVTypeParameter, into: &mut Vec<Ptr<E>>, allEdges: bool) -> f64
    where
        E: RouterEdge<V>,
        V: RouterVehicle,
    {
        if get_sidewalk::<E, L, V>(from, SVC_PEDESTRIAN).is_null() {
            utils::common::msg_handler::write_warning(&format!("Departure edge '{}' does not allow pedestrians.", from.edge_id()));
            return 0.;
        }
        if get_sidewalk::<E, L, V>(to, SVC_PEDESTRIAN).is_null() {
            utils::common::msg_handler::write_warning(&format!("Destination edge '{}' does not allow pedestrians.", to.edge_id()));
            return 0.;
        }
        let trip = IntermodalTrip::new(from, to, departPos, arrivalPos, speed, msTime, onlyNode, pars, Ptr::null(), SVC_PEDESTRIAN, Ptr::null(), 0.);
        let mut intoPed: Vec<Ptr<IntermodalEdge<E, L, N, V>>> = Vec::new();
        let silent = allEdges; // no warning is needed when called from MSPModel_Striping
        let success = self.myInternalRouter.m().compute(
            self.myPedNet.get_depart_connector(from, 0).cast(),
            self.myPedNet.get_arrival_connector(to, 0).cast(),
            Ptr::new(&trip), msTime, &mut intoPed, silent);
        let mut time = 0.;
        if success {
            for pedEdge in intoPed {
                if pedEdge.include_in_route(allEdges) {
                    into.push(pedEdge.get_edge());
                }
                time += self.myInternalRouter.m().get_effort(pedEdge, Ptr::new(&trip), time);
            }
        }
        if success { time } else { -1. }
    }

    /// C++: `bool PedestrianRouter::compute(const E *, const E *, const _IntermodalTrip *const, SUMOTime, std::vector<const E *> &, bool)`
    /// Builds the route between the given edges using the minimum effort at the given time The definition of the effort depends on the wished routing scheme
    // bool compute(const E*, const E*, const _IntermodalTrip* const,
    //                  SUMOTime, std::vector<const E*>&, bool) {
    //         throw ProcessError(TL("Do not use this method"));
    //     }
    #[allow(unused_variables)]
    pub fn compute_2(&mut self, arg0: Ptr<E>, arg1: Ptr<E>, arg2: Ptr<PedestrianRouter__IntermodalTrip<E, N, V>>, arg3: SUMOTime, arg4: &mut Vec<Ptr<E>>, arg5: bool) -> bool {
        panic!("{}", ProcessError::new("Do not use this method"))
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
        let mut toProhibitPE: SUMOAbstractRouter_Prohibitions = Default::default();
        for (edge, prohibition) in toProhibit {
            let dirs = self.myPedNet.get_both_directions(*edge);
            // shortcut: SUMOAbstractRouter_Prohibitions is keyed by Ptr<Unmapped>, cast the address until that typedef is generic
            toProhibitPE.insert(dirs.0.cast(), *prohibition);
            toProhibitPE.insert(dirs.1.cast(), *prohibition);
        }
        self.myInternalRouter.m().prohibit(&toProhibitPE);
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
    pub fn recompute_walk_costs(&self, edges: &Vec<Ptr<E>>, speed: f64, fromPos: f64, toPos: f64, msTime: SUMOTime, pars: &SUMOVTypeParameter, length: &mut f64) -> f64 where E: RouterEdge<V>, V: RouterVehicle {
        if edges.len() == 0 {
            *length = 0.;
            0.
        } else if edges.len() == 1 {
            *length = (toPos - fromPos).abs();
            *length / speed
        } else {
            let mut cost = 0.;
            let last = edges.len() as i32 - 1;
            for i in 0..last {
                let mut into: Vec<Ptr<E>> = Vec::new();
                let from = edges[i as usize];
                let to = edges[(i + 1) as usize];
                let fp = if i == 0 { fromPos } else { from.length() / 2. };
                let tp = if i == last - 1 { toPos } else { to.length() / 2. };
                let node = self.get_common_node(from, to);
                if i == 0 {
                    if node == from.to_junction() {
                        *length += from.length() - fromPos;
                    } else {
                        *length += fromPos;
                    }
                } else {
                    *length += from.length();
                }
                if i == last - 1 {
                    if node == to.from_junction() {
                        *length += toPos;
                    } else {
                        *length += to.length() - toPos;
                    }
                }
                let time = self.compute(from, to, fp, tp, speed, msTime, node, pars, &mut into, true);
                if time >= 0. {
                    cost += time;
                    for edge in into {
                        if edge.is_crossing() {
                            *length += edge.length();
                        } else if edge.is_walking_area() {
                            // this is wrong because the length is path-dependent
                            *length += edge.length();
                        }
                    }
                } else {
                    panic!("{}", ProcessError::new(format!("Could not compute cost between edge '{}' and edge '{}'.", from.edge_id(), to.edge_id())));
                }
            }
            cost
        }
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
    pub fn get_common_node(&self, from: Ptr<E>, to: Ptr<E>) -> Ptr<N> where E: RouterEdge<V, Node = N>, V: RouterVehicle {
        if from.to_junction() == to.from_junction() || from.to_junction() == to.to_junction() {
            from.to_junction()
        } else if from.from_junction() == to.from_junction() || from.from_junction() == to.to_junction() {
            from.from_junction()
        } else {
            Ptr::null()
        }
    }

    /// C++: `PedestrianRouter<E, L, N, V> & PedestrianRouter::operator=(const PedestrianRouter<E, L, N, V> &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &PedestrianRouter<E, L, N, V>) -> &mut PedestrianRouter<E, L, N, V> {
        panic!("PedestrianRouter::operator= is declared but not defined")
    }

}

