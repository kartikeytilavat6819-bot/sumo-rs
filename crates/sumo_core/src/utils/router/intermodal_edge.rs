//! Port of utils/router/IntermodalEdge.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use crate::utils::router::access_edge::AccessEdge;
use crate::utils::router::car_edge::CarEdge;
use crate::utils::router::pedestrian_edge::PedestrianEdge;
use crate::utils::router::public_transport_edge::PublicTransportEdge;
use crate::utils::router::stop_edge::StopEdge;
use crate::utils::router::router_edge::{RouterEdge, RouterEdgeInfo, RouterVehicle};

/// the base edge type that is given to the internal router (SUMOAbstractRouter)
/// C++ class `IntermodalEdge` (utils/router/IntermodalEdge.h)
#[repr(C)]
pub struct IntermodalEdge<E, L, N, V> {
    /// C++ base class `Named`
    pub base: Named,
    /// List of edges that may be approached from this edge
    pub myFollowingEdges: Vec<Ptr<IntermodalEdge<E, L, N, V>>>,
    /// List of edges that may be approached from this edge with optional internal vias
    pub myFollowingViaEdges: Vec<(Ptr<IntermodalEdge<E, L, N, V>>, Ptr<IntermodalEdge<E, L, N, V>>)>,
    /// the index in myEdges
    pub myNumericalID: i32,
    /// the original edge
    pub myEdge: Ptr<E>,
    /// public transport line or ped vs car
    pub myLine: String,
    /// adaptable length (for splitted edges)
    pub myLength: f64,
    /// Container for passing effort varying over time for the edge
    pub myEfforts: Ptr<Unmapped /* ValueTimeLine<double> */>,
    /// C++ virtual dispatch: which subclass this base object really is (dispatched by the virtual methods below)
    pub myKind: IntermodalEdgeKind,
    _p: std::marker::PhantomData<(E, L, N, V,)>,
}

/// The C++ subclasses of IntermodalEdge that override its virtual methods.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntermodalEdgeKind {
    Base,
    Access,
    Pedestrian,
    Car,
    Stop,
    PublicTransport,
}

impl<E, L, N, V> std::ops::Deref for IntermodalEdge<E, L, N, V> {
    type Target = Named;
    fn deref(&self) -> &Named {
        &self.base
    }
}
impl<E, L, N, V> std::ops::DerefMut for IntermodalEdge<E, L, N, V> {
    fn deref_mut(&mut self) -> &mut Named {
        &mut self.base
    }
}

impl<E, L, N, V> RouterEdgeInfo for IntermodalEdge<E, L, N, V>
where
    E: RouterEdge<V, Node = N, Lane = L> + 'static,
    L: 'static,
    N: 'static,
    V: RouterVehicle + 'static,
{
    fn edge_numerical_id(&self) -> i32 {
        <IntermodalEdge<E, L, N, V> as RouterEdge<IntermodalTrip<E, N, V>>>::numerical_id(self)
    }
    fn edge_id(&self) -> String {
        <IntermodalEdge<E, L, N, V> as RouterEdge<IntermodalTrip<E, N, V>>>::id(self).to_string()
    }
    // shortcut: IntermodalEdge has no getPermissions in C++ (its lanes carry them), so SPTree never reads it on an intermodal edge, upgrade when an intermodal edge becomes a CH node
    fn edge_permissions(&self) -> SVCPermissions {
        panic!("shortcut: IntermodalEdge has no getPermissions in C++")
    }
}

impl<E, L, N, V> RouterEdge<IntermodalTrip<E, N, V>> for IntermodalEdge<E, L, N, V>
where
    E: RouterEdge<V, Node = N, Lane = L> + 'static,
    L: 'static,
    N: 'static,
    V: RouterVehicle + 'static,
{
    type Node = N;
    type Lane = L;
    fn bidi_edge(&self) -> Ptr<IntermodalEdge<E, L, N, V>> {
        self.get_bidi_edge()
    }
    fn id(&self) -> &str {
        self.base.get_id()
    }
    fn numerical_id(&self) -> i32 {
        self.get_numerical_id()
    }
    fn length(&self) -> f64 {
        self.get_length()
    }
    fn length_geometry_factor(&self) -> f64 {
        self.get_length_geometry_factor()
    }
    // C++ IntermodalEdge::getDistanceTo takes no flag and always estimates the boundary
    fn distance_to(&self, other: Ptr<Self>, _do_boundary_estimate: bool) -> f64 {
        self.get_distance_to(other)
    }
    fn successors(&self, vclass: SUMOVehicleClass) -> &[Ptr<Self>] {
        self.get_successors(vclass)
    }
    fn via_successors(&self, vclass: SUMOVehicleClass, ignore_transient: bool) -> &[(Ptr<Self>, Ptr<Self>)] {
        self.get_via_successors(vclass, ignore_transient)
    }
    fn speed_limit(&self) -> f64 {
        self.get_speed_limit()
    }
    fn minimum_travel_time(&self, trip: Ptr<IntermodalTrip<E, N, V>>) -> f64 {
        self.get_minimum_travel_time(trip)
    }
    fn travel_time_static(edge: Ptr<Self>, veh: Ptr<IntermodalTrip<E, N, V>>, time: f64) -> f64 {
        Self::get_travel_time_static(edge, veh, time)
    }
    fn travel_time_aggregated(edge: Ptr<Self>, veh: Ptr<IntermodalTrip<E, N, V>>, time: f64) -> f64 {
        Self::get_travel_time_aggregated_2(edge, veh, time)
    }
    fn is_internal(&self) -> bool {
        self.is_internal()
    }
    fn prohibits(&self, veh: Ptr<IntermodalTrip<E, N, V>>, _check_restrictions: bool) -> bool {
        self.prohibits(veh)
    }
    fn restricts(&self, veh: Ptr<IntermodalTrip<E, N, V>>) -> bool {
        self.restricts(veh)
    }
}

impl<E, L, N, V> IntermodalEdge<E, L, N, V>
where
    E: RouterEdge<V, Node = N, Lane = L> + 'static,
    L: 'static,
    N: 'static,
    V: RouterVehicle + 'static,
{
    /// C++: `void IntermodalEdge::IntermodalEdge<E, L, N, V>(const std::string, int, const E *, const std::string &, const double)`
    // IntermodalEdge(const std::string id, int numericalID, const E* edge, const std::string& line, const double length = -1) :
    //         Named(id),
    //         myNumericalID(numericalID),
    //         myEdge(edge),
    //         myLine(line),
    //         myLength(edge == nullptr || length >= 0. ? MAX2(0.0, length) : edge->getLength()),
    //         myEfforts(nullptr) { }
    #[allow(unused_variables)]
    pub fn new(id: String, numericalID: i32, edge: Ptr<E>, line: &str, length: f64) -> Self {
        let my_length = if edge.is_null() || length >= 0. {
            if 0.0 > length { 0.0 } else { length }
        } else {
            edge.length()
        };
        Self {
            base: Named::new(&id),
            myFollowingEdges: Vec::new(),
            myFollowingViaEdges: Vec::new(),
            myNumericalID: numericalID,
            myEdge: edge,
            myLine: line.to_string(),
            myLength: my_length,
            myEfforts: Ptr::null(),
            myKind: IntermodalEdgeKind::Base,
            _p: std::marker::PhantomData,
        }
    }

    /// Tags a freshly built base object as one of the subclasses (the subclass ctor calls the base ctor first in C++)
    pub fn with_kind(mut self, kind: IntermodalEdgeKind) -> Self {
        self.myKind = kind;
        self
    }

    /// The subclass view of this base object (C++ virtual dispatch target); only valid when myKind matches.
    pub fn as_access(&self) -> &AccessEdge<E, L, N, V> {
        assert!(self.myKind == IntermodalEdgeKind::Access);
        unsafe { &*(self as *const Self as *const AccessEdge<E, L, N, V>) }
    }
    pub fn as_pedestrian(&self) -> &PedestrianEdge<E, L, N, V> {
        assert!(self.myKind == IntermodalEdgeKind::Pedestrian);
        unsafe { &*(self as *const Self as *const PedestrianEdge<E, L, N, V>) }
    }
    pub fn as_car(&self) -> &CarEdge<E, L, N, V> {
        assert!(self.myKind == IntermodalEdgeKind::Car);
        unsafe { &*(self as *const Self as *const CarEdge<E, L, N, V>) }
    }
    pub fn as_stop(&self) -> &StopEdge<E, L, N, V> {
        assert!(self.myKind == IntermodalEdgeKind::Stop);
        unsafe { &*(self as *const Self as *const StopEdge<E, L, N, V>) }
    }
    pub fn as_public_transport(&self) -> &PublicTransportEdge<E, L, N, V> {
        assert!(self.myKind == IntermodalEdgeKind::PublicTransport);
        unsafe { &*(self as *const Self as *const PublicTransportEdge<E, L, N, V>) }
    }

    /// C++: `void IntermodalEdge::~IntermodalEdge<E, L, N, V>()` [virtual]
    // virtual ~IntermodalEdge() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty; nothing to release
    }

    /// C++: `bool IntermodalEdge::includeInRoute(bool) const` [const, virtual]
    // virtual bool includeInRoute(bool /* allEdges */) const {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn include_in_route(&self, arg0: bool) -> bool {
        match self.myKind {
            IntermodalEdgeKind::Base => false,
            IntermodalEdgeKind::Access => false,
            IntermodalEdgeKind::Stop => self.as_stop().include_in_route(arg0),
            IntermodalEdgeKind::PublicTransport => self.as_public_transport().include_in_route(arg0),
            IntermodalEdgeKind::Pedestrian => self.as_pedestrian().include_in_route(arg0),
            IntermodalEdgeKind::Car => self.as_car().include_in_route(arg0),
        }
    }

    /// C++: `const std::string & IntermodalEdge::getLine() const` [const]
    // inline const std::string& getLine() const {
    //         return myLine;
    //     }
    #[allow(unused_variables)]
    pub fn get_line(&self) -> &str {
        self.myLine.as_str()
    }

    /// C++: `const E * IntermodalEdge::getEdge() const` [const]
    // inline const E* getEdge() const {
    //         return myEdge;
    //     }
    #[allow(unused_variables)]
    pub fn get_edge(&self) -> Ptr<E> {
        self.myEdge
    }

    /// C++: `int IntermodalEdge::getNumericalID() const` [const]
    // int getNumericalID() const {
    //         return myNumericalID;
    //     }
    #[allow(unused_variables)]
    pub fn get_numerical_id(&self) -> i32 {
        self.myNumericalID
    }

    /// C++: `void IntermodalEdge::addSuccessor(IntermodalEdge<E, L, N, V> *const, IntermodalEdge<E, L, N, V> *const)`
    // void addSuccessor(IntermodalEdge* const s, IntermodalEdge* const via = nullptr) {
    //         myFollowingEdges.push_back(s);
    //         myFollowingViaEdges.push_back(std::make_pair(s, via));
    //     }
    #[allow(unused_variables)]
    pub fn add_successor(&mut self, s: Ptr<IntermodalEdge<E, L, N, V>>, via: Ptr<IntermodalEdge<E, L, N, V>>) {
        self.myFollowingEdges.push(s);
        self.myFollowingViaEdges.push((s, via));
    }

    /// C++: `void IntermodalEdge::transferSuccessors(IntermodalEdge<E, L, N, V> *)`
    // void transferSuccessors(IntermodalEdge* to) {
    //         to->myFollowingEdges = myFollowingEdges;
    //         to->myFollowingViaEdges = myFollowingViaEdges;
    //         myFollowingEdges.clear();
    //         myFollowingViaEdges.clear();
    //     }
    #[allow(unused_variables)]
    pub fn transfer_successors(&mut self, to: Ptr<IntermodalEdge<E, L, N, V>>) {
        let to = to.m();
        to.myFollowingEdges = self.myFollowingEdges.clone();
        to.myFollowingViaEdges = self.myFollowingViaEdges.clone();
        self.myFollowingEdges.clear();
        self.myFollowingViaEdges.clear();
    }

    /// C++: `bool IntermodalEdge::removeSuccessor(const IntermodalEdge<E, L, N, V> *const)`
    // bool removeSuccessor(const IntermodalEdge* const edge) {
    //         auto it = std::find(myFollowingEdges.begin(), myFollowingEdges.end(), edge);
    //         if (it != myFollowingEdges.end()) {
    //             myFollowingEdges.erase(it);
    //         } else {
    //             return false;
    //         }
    //         for (auto viaIt = myFollowingViaEdges.begin(); viaIt != myFollowingViaEdges.end();) {
    //             if (viaIt->first == edge) {
    //                 viaIt = myFollowingViaEdges.erase(viaIt);
    //             } else {
    //                 ++viaIt;
    //             }
    //         }
    //         return true;
    //     }
    #[allow(unused_variables)]
    pub fn remove_successor(&mut self, edge: Ptr<IntermodalEdge<E, L, N, V>>) -> bool {
        match self.myFollowingEdges.iter().position(|e| *e == edge) {
            Some(i) => {
                self.myFollowingEdges.remove(i);
            }
            None => return false,
        }
        // determinism: the via list is filtered in place, keeping the C++ erase order
        self.myFollowingViaEdges.retain(|p| p.0 != edge);
        true
    }

    /// C++: `const std::vector<IntermodalEdge<E, L, N, V> *> & IntermodalEdge::getSuccessors(SUMOVehicleClass) const` [const, virtual]
    // virtual const std::vector<IntermodalEdge*>& getSuccessors(SUMOVehicleClass vClass = SVC_IGNORING) const {
    //         UNUSED_PARAMETER(vClass);
    //         // the network is already tailored. No need to check for permissions here
    //         return myFollowingEdges;
    //     }
    #[allow(unused_variables)]
    pub fn get_successors(&self, vClass: SUMOVehicleClass) -> &Vec<Ptr<IntermodalEdge<E, L, N, V>>> {
        match self.myKind {
            IntermodalEdgeKind::Car => self.as_car().get_successors(vClass),
            _ => &self.myFollowingEdges,
        }
    }

    /// C++: `const std::vector<std::pair<const IntermodalEdge<E, L, N, V> *, const IntermodalEdge<E, L, N, V> *>> & IntermodalEdge::getViaSuccessors(SUMOVehicleClass, bool) const` [const, virtual]
    // virtual const std::vector<std::pair<const IntermodalEdge*, const IntermodalEdge*> >& getViaSuccessors(SUMOVehicleClass vClass = SVC_IGNORING, bool ignoreTransientPermissions = false) const {
    //         UNUSED_PARAMETER(vClass);
    //         UNUSED_PARAMETER(ignoreTransientPermissions);
    //         // the network is already tailored. No need to check for permissions here
    //         return myFollowingViaEdges;
    //     }
    #[allow(unused_variables)]
    pub fn get_via_successors(&self, vClass: SUMOVehicleClass, ignoreTransientPermissions: bool) -> &Vec<(Ptr<IntermodalEdge<E, L, N, V>>, Ptr<IntermodalEdge<E, L, N, V>>)> {
        match self.myKind {
            IntermodalEdgeKind::Car => self.as_car().get_via_successors(vClass, ignoreTransientPermissions),
            _ => &self.myFollowingViaEdges,
        }
    }

    /// C++: `bool IntermodalEdge::prohibits(const IntermodalTrip<E, N, V> *const) const` [const, virtual]
    // virtual bool prohibits(const IntermodalTrip<E, N, V>* const /* trip */) const {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn prohibits(&self, arg0: Ptr<IntermodalTrip<E, N, V>>) -> bool {
        match self.myKind {
            IntermodalEdgeKind::Base | IntermodalEdgeKind::Stop => false,
            IntermodalEdgeKind::Access => self.as_access().prohibits(arg0),
            IntermodalEdgeKind::Pedestrian => self.as_pedestrian().prohibits(arg0),
            IntermodalEdgeKind::Car => self.as_car().prohibits(arg0),
            IntermodalEdgeKind::PublicTransport => self.as_public_transport().prohibits(arg0),
        }
    }

    /// C++: `bool IntermodalEdge::restricts(const IntermodalTrip<E, N, V> *const) const` [const, virtual]
    // virtual bool restricts(const IntermodalTrip<E, N, V>* const /* trip */) const {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn restricts(&self, arg0: Ptr<IntermodalTrip<E, N, V>>) -> bool {
        false
    }

    /// C++: `double IntermodalEdge::getPartialLength(const IntermodalTrip<E, N, V> *const) const` [const, virtual]
    // virtual inline double getPartialLength(const IntermodalTrip<E, N, V>* const /*trip*/) const {
    //         return myLength;
    //     }
    #[allow(unused_variables)]
    pub fn get_partial_length(&self, arg0: Ptr<IntermodalTrip<E, N, V>>) -> f64 {
        match self.myKind {
            IntermodalEdgeKind::Base | IntermodalEdgeKind::Access | IntermodalEdgeKind::Stop | IntermodalEdgeKind::PublicTransport => self.myLength,
            IntermodalEdgeKind::Pedestrian => self.as_pedestrian().get_partial_length(arg0),
            IntermodalEdgeKind::Car => self.as_car().get_partial_length(arg0),
        }
    }

    /// C++: `double IntermodalEdge::getTravelTime(const IntermodalTrip<E, N, V> *const, double) const` [const, virtual]
    // virtual inline double getTravelTime(const IntermodalTrip<E, N, V>* const /* trip */, double /* time */) const {
    //         return 0.;
    //     }
    #[allow(unused_variables)]
    pub fn get_travel_time(&self, arg0: Ptr<IntermodalTrip<E, N, V>>, arg1: f64) -> f64 {
        match self.myKind {
            IntermodalEdgeKind::Base | IntermodalEdgeKind::Stop => 0.,
            IntermodalEdgeKind::Access => self.as_access().get_travel_time(arg0, arg1),
            IntermodalEdgeKind::Pedestrian => self.as_pedestrian().get_travel_time(arg0, arg1),
            IntermodalEdgeKind::Car => self.as_car().get_travel_time(arg0, arg1),
            IntermodalEdgeKind::PublicTransport => self.as_public_transport().get_travel_time(arg0, arg1),
        }
    }

    /// C++: `double IntermodalEdge::getTravelTimeAggregated(const IntermodalTrip<E, N, V> *const, double) const` [const, virtual]
    // virtual inline double getTravelTimeAggregated(const IntermodalTrip<E, N, V>* const trip, double time) const {
    //         return getTravelTime(trip, time);
    //     }
    #[allow(unused_variables)]
    pub fn get_travel_time_aggregated(&self, trip: Ptr<IntermodalTrip<E, N, V>>, time: f64) -> f64 {
        match self.myKind {
            IntermodalEdgeKind::Car => self.as_car().get_travel_time_aggregated(trip, time),
            _ => self.get_travel_time(trip, time),
        }
    }

    /// C++: `double IntermodalEdge::getIntended(const double, std::string &) const` [const, virtual]
    /// get intended vehicle id and departure time of next public transport ride
    // virtual inline double getIntended(const double /* time */, std::string& /* intended */) const {
    //         return 0.;
    //     }
    #[allow(unused_variables)]
    pub fn get_intended(&self, arg0: f64, arg1: &mut String) -> f64 {
        match self.myKind {
            IntermodalEdgeKind::PublicTransport => self.as_public_transport().get_intended(arg0, arg1),
            _ => 0.,
        }
    }

    /// C++: `double IntermodalEdge::getTravelTimeStatic(const IntermodalEdge<E, L, N, V> *const, const IntermodalTrip<E, N, V> *const, double)` [static]
    // static inline double getTravelTimeStatic(const IntermodalEdge* const edge, const IntermodalTrip<E, N, V>* const trip, double time) {
    //         return edge == nullptr ? 0. : edge->getTravelTime(trip, time) * getRoutingFactor(edge, trip);
    //     }
    #[allow(unused_variables)]
    pub fn get_travel_time_static(edge: Ptr<IntermodalEdge<E, L, N, V>>, trip: Ptr<IntermodalTrip<E, N, V>>, time: f64) -> f64 {
        if edge.is_null() {
            0.
        } else {
            edge.get_travel_time(trip, time) * Self::get_routing_factor(edge, trip)
        }
    }

    /// C++: `double IntermodalEdge::getTravelTimeStaticRandomized(const IntermodalEdge<E, L, N, V> *const, const IntermodalTrip<E, N, V> *const, double)` [static]
    // static inline double getTravelTimeStaticRandomized(const IntermodalEdge* const edge, const IntermodalTrip<E, N, V>* const trip, double time) {
    //         return edge == nullptr ? 0. : edge->getTravelTime(trip, time) * RandHelper::rand(1., gWeightsRandomFactor) * getRoutingFactor(edge, trip);
    //     }
    #[allow(unused_variables)]
    pub fn get_travel_time_static_randomized(edge: Ptr<IntermodalEdge<E, L, N, V>>, trip: Ptr<IntermodalTrip<E, N, V>>, time: f64) -> f64 {
        if edge.is_null() {
            0.
        } else {
            let factor = utils::common::std_defs::g_weights_random_factor();
            edge.get_travel_time(trip, time)
                * utils::common::rand_helper::with_default(|r| r.rand_range(1., factor))
                * Self::get_routing_factor(edge, trip)
        }
    }

    /// C++: `double IntermodalEdge::getTravelTimeAggregated(const IntermodalEdge<E, L, N, V> *const, const IntermodalTrip<E, N, V> *const, double)` [static]
    // static inline double getTravelTimeAggregated(const IntermodalEdge* const edge, const IntermodalTrip<E, N, V>* const trip, double time) {
    //         return edge == nullptr ? 0. : edge->getTravelTimeAggregated(trip, time) * getRoutingFactor(edge, trip);
    //     }
    #[allow(unused_variables)]
    pub fn get_travel_time_aggregated_2(edge: Ptr<IntermodalEdge<E, L, N, V>>, trip: Ptr<IntermodalTrip<E, N, V>>, time: f64) -> f64 {
        if edge.is_null() {
            0.
        } else {
            edge.get_travel_time_aggregated(trip, time) * Self::get_routing_factor(edge, trip)
        }
    }

    /// C++: `double IntermodalEdge::getRoutingFactor(const IntermodalEdge<E, L, N, V> *const, const IntermodalTrip<E, N, V> *const)` [static]
    // static inline double getRoutingFactor(const IntermodalEdge* const edge, const IntermodalTrip<E, N, V>* const trip) {
    //         return (!gRoutingPreferences || edge == nullptr || edge->myEdge == nullptr || trip == nullptr) ? 1 : 1 / edge->myEdge->getPreference(trip->getVTypeParameter());
    //     }
    #[allow(unused_variables)]
    pub fn get_routing_factor(edge: Ptr<IntermodalEdge<E, L, N, V>>, trip: Ptr<IntermodalTrip<E, N, V>>) -> f64 {
        if !utils::common::std_defs::g_routing_preferences() || edge.is_null() || edge.myEdge.is_null() || trip.is_null() {
            1.
        } else {
            1. / edge.myEdge.preference(trip.get_v_type_parameter())
        }
    }

    /// C++: `double IntermodalEdge::getEffort(const IntermodalTrip<E, N, V> *const, double) const` [const, virtual]
    // virtual double getEffort(const IntermodalTrip<E, N, V>* const /* trip */, double /* time */) const {
    //         return 0.;
    //     }
    #[allow(unused_variables)]
    pub fn get_effort(&self, arg0: Ptr<IntermodalTrip<E, N, V>>, arg1: f64) -> f64 {
        0.
    }

    /// C++: `double IntermodalEdge::getEffortStatic(const IntermodalEdge<E, L, N, V> *const, const IntermodalTrip<E, N, V> *const, double)` [static]
    // static inline double getEffortStatic(const IntermodalEdge* const edge, const IntermodalTrip<E, N, V>* const trip, double time) {
    //         return edge == nullptr || !edge->hasEffort() ? 0. : edge->getEffort(trip, time);
    //     }
    #[allow(unused_variables)]
    pub fn get_effort_static(edge: Ptr<IntermodalEdge<E, L, N, V>>, trip: Ptr<IntermodalTrip<E, N, V>>, time: f64) -> f64 {
        if edge.is_null() || !edge.has_effort() {
            0.
        } else {
            edge.get_effort(trip, time)
        }
    }

    /// C++: `double IntermodalEdge::getLength() const` [const]
    /// required by DijkstraRouter et al for external effort computation
    // inline double getLength() const {
    //         return myLength;
    //     }
    #[allow(unused_variables)]
    pub fn get_length(&self) -> f64 {
        self.myLength
    }

    /// C++: `void IntermodalEdge::setLength(const double)`
    // inline void setLength(const double length) {
    //         assert(length >= 0);
    //         myLength = length;
    //     }
    #[allow(unused_variables)]
    pub fn set_length(&mut self, length: f64) {
        assert!(length >= 0.);
        self.myLength = length;
    }

    /// C++: `bool IntermodalEdge::isInternal() const` [const]
    // inline bool isInternal() const {
    //         return myEdge != nullptr && myEdge->isInternal();
    //     }
    #[allow(unused_variables)]
    pub fn is_internal(&self) -> bool {
        if self.myEdge.is_null() {
            false
        } else {
            self.myEdge.is_internal()
        }
    }

    /// C++: `bool IntermodalEdge::hasEffort() const` [const, virtual]
    // virtual bool hasEffort() const {
    //         return myEfforts != nullptr;
    //     }
    #[allow(unused_variables)]
    pub fn has_effort(&self) -> bool {
        !self.myEfforts.is_null()
    }

    /// C++: `double IntermodalEdge::getStartPos() const` [const, virtual]
    // virtual double getStartPos() const {
    //         return 0.;
    //     }
    #[allow(unused_variables)]
    pub fn get_start_pos(&self) -> f64 {
        match self.myKind {
            IntermodalEdgeKind::Base | IntermodalEdgeKind::Access | IntermodalEdgeKind::PublicTransport => 0.,
            IntermodalEdgeKind::Stop => self.as_stop().get_start_pos(),
            IntermodalEdgeKind::Pedestrian => self.as_pedestrian().get_start_pos(),
            IntermodalEdgeKind::Car => self.as_car().get_start_pos(),
        }
    }

    /// C++: `double IntermodalEdge::getEndPos() const` [const, virtual]
    // virtual double getEndPos() const {
    //         return myLength;
    //     }
    #[allow(unused_variables)]
    pub fn get_end_pos(&self) -> f64 {
        match self.myKind {
            IntermodalEdgeKind::Base | IntermodalEdgeKind::Access | IntermodalEdgeKind::PublicTransport => self.myLength,
            IntermodalEdgeKind::Stop => self.as_stop().get_end_pos(),
            IntermodalEdgeKind::Pedestrian => self.as_pedestrian().get_end_pos(),
            IntermodalEdgeKind::Car => self.as_car().get_end_pos(),
        }
    }

    /// C++: `double IntermodalEdge::getSpeedLimit() const` [const]
    // inline double getSpeedLimit() const {
    //         return myEdge != nullptr ? myEdge->getSpeedLimit() : 200. / 3.6;
    //     }
    #[allow(unused_variables)]
    pub fn get_speed_limit(&self) -> f64 {
        if self.myEdge.is_null() {
            200. / 3.6
        } else {
            self.myEdge.speed_limit()
        }
    }

    /// C++: `double IntermodalEdge::getLengthGeometryFactor() const` [const]
    // inline double getLengthGeometryFactor() const {
    //         return myEdge != nullptr ? myEdge->getLengthGeometryFactor() : 1;
    //     }
    #[allow(unused_variables)]
    pub fn get_length_geometry_factor(&self) -> f64 {
        if self.myEdge.is_null() {
            1.
        } else {
            self.myEdge.length_geometry_factor()
        }
    }

    /// C++: `double IntermodalEdge::getDistanceTo(const IntermodalEdge<E, L, N, V> *) const` [const]
    // inline double getDistanceTo(const IntermodalEdge* other) const {
    //         return myEdge != nullptr && other->myEdge != nullptr && myEdge != other->myEdge ? myEdge->getDistanceTo(other->myEdge, true) : 0.;
    //     }
    #[allow(unused_variables)]
    pub fn get_distance_to(&self, other: Ptr<IntermodalEdge<E, L, N, V>>) -> f64 {
        if self.myEdge.is_null() || other.myEdge.is_null() || self.myEdge == other.myEdge {
            0.
        } else {
            self.myEdge.distance_to(other.myEdge, true)
        }
    }

    /// C++: `double IntermodalEdge::getMinimumTravelTime(const IntermodalTrip<E, N, V> *const) const` [const]
    // inline double getMinimumTravelTime(const IntermodalTrip<E, N, V>* const trip) const {
    //         return myLength / trip->getMaxSpeed();
    //     }
    #[allow(unused_variables)]
    pub fn get_minimum_travel_time(&self, trip: Ptr<IntermodalTrip<E, N, V>>) -> f64 {
        self.myLength / trip.get_max_speed()
    }

    /// C++: `IntermodalEdge<E, L, N, V> * IntermodalEdge::getBidiEdge() const` [const]
    /// only used by mono-modal routing
    // IntermodalEdge* getBidiEdge() const {
    //         return nullptr;
    //     }
    #[allow(unused_variables)]
    pub fn get_bidi_edge(&self) -> Ptr<IntermodalEdge<E, L, N, V>> {
        Ptr::null()
    }

    /// C++: `void IntermodalEdge::IntermodalEdge<E, L, N, V>(const IntermodalEdge<E, L, N, V> &)` [private]
    /// Invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(src: &IntermodalEdge<E, L, N, V>) -> Self {
        panic!("IntermodalEdge copy constructor is private and invalidated in C++")
    }

    /// C++: `IntermodalEdge<E, L, N, V> & IntermodalEdge::operator=(const IntermodalEdge<E, L, N, V> &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, src: &IntermodalEdge<E, L, N, V>) -> &mut IntermodalEdge<E, L, N, V> {
        panic!("IntermodalEdge assignment operator is private and invalidated in C++")
    }

}

/// C++: `const L *(const E *, SUMOVehicleClass) getSidewalk`
// template <class E, class L>
// inline const L* getSidewalk(const E* edge, SUMOVehicleClass svc = SVC_PEDESTRIAN) {
//     if (edge == nullptr) {
//         return nullptr;
//     }
//     // prefer lanes that are exclusive to pedestrians
//     const std::vector<L*>& lanes = edge->getLanes();
//     for (const L* const lane : lanes) {
//         if (lane->getPermissions() == svc) {
//             return lane;
//         }
//     }
//     for (const L* const lane : lanes) {
//         if (lane->allowsVehicleClass(svc)) {
//             return lane;
//         }
//     }
//     if (svc != SVC_PEDESTRIAN) {
//         // persons should always be able to use the sidewalk
//         for (const L* const lane : lanes) {
//             if (lane->getPermissions() == SVC_PEDESTRIAN) {
//                 return lane;
//             }
//         }
//         for (const L* const lane : lanes) {
//             if (lane->allowsVehicleClass(SVC_PEDESTRIAN)) {
//                 return lane;
//             }
//         }
//     }
//     return nullptr;
// }
#[allow(unused_variables)]
pub fn get_sidewalk<E, L, V>(edge: Ptr<E>, svc: SUMOVehicleClass) -> Ptr<L>
where
    E: RouterEdge<V, Lane = L>,
{
    if edge.is_null() {
        return Ptr::null();
    }
    // prefer lanes that are exclusive to pedestrians
    let lanes: Vec<Ptr<L>> = edge.lanes().to_vec();
    for &lane in &lanes {
        if E::lane_permissions(lane) == svc {
            return lane;
        }
    }
    for &lane in &lanes {
        if E::lane_allows_vehicle_class(lane, svc) {
            return lane;
        }
    }
    if svc != SVC_PEDESTRIAN {
        // persons should always be able to use the sidewalk
        for &lane in &lanes {
            if E::lane_permissions(lane) == SVC_PEDESTRIAN {
                return lane;
            }
        }
        for &lane in &lanes {
            if E::lane_allows_vehicle_class(lane, SVC_PEDESTRIAN) {
                return lane;
            }
        }
    }
    Ptr::null()
}

