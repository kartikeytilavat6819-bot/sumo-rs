//! Port of utils/router/CHRouterWrapper.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `CHRouter<E, V> CHRouterWrapper::CHRouterType`
pub type CHRouterWrapper_CHRouterType = Unmapped /* CHRouter<E, V> */;

/// C++ typedef `int CHRouterWrapper::RouterMap`
pub type CHRouterWrapper_RouterMap = i32;

/// Computes the shortest path through a contracted network
/// C++ class `CHRouterWrapper` (utils/router/CHRouterWrapper.h)
pub struct CHRouterWrapper<E, V> {
    /// C++ base class `SUMOAbstractRouter<E, V>`
    pub base: SUMOAbstractRouter<E, V>,
    pub myRouters: CHRouterWrapper_RouterMap,
    /// all edges with numerical ids
    pub myEdges: Ptr<i32>,
    pub myIgnoreErrors: bool,
    pub myBegin: SUMOTime,
    pub myEnd: SUMOTime,
    pub myWeightPeriod: SUMOTime,
    pub myMaxNumInstances: i32,
    _p: std::marker::PhantomData<(E, V,)>,
}

impl<E, V> std::ops::Deref for CHRouterWrapper<E, V> {
    type Target = SUMOAbstractRouter<E, V>;
    fn deref(&self) -> &SUMOAbstractRouter<E, V> {
        &self.base
    }
}
impl<E, V> std::ops::DerefMut for CHRouterWrapper<E, V> {
    fn deref_mut(&mut self) -> &mut SUMOAbstractRouter<E, V> {
        &mut self.base
    }
}

impl<E, V> CHRouterWrapper<E, V> {
    /// C++: `void CHRouterWrapper::CHRouterWrapper<E, V>(const int &, const bool, typename SUMOAbstractRouter<E, V>::Operation, const SUMOTime, const SUMOTime, const SUMOTime, bool, const int)`
    /// Constructor
    // CHRouterWrapper(const std::vector<E*>& edges, const bool ignoreErrors, typename SUMOAbstractRouter<E, V>::Operation operation,
    //                     const SUMOTime begin, const SUMOTime end, const SUMOTime weightPeriod, bool havePermissions, const int numThreads) :
    //         SUMOAbstractRouter<E, V>("CHRouterWrapper", ignoreErrors, operation, nullptr, havePermissions, false),
    //         myEdges(edges),
    //         myIgnoreErrors(ignoreErrors),
    //         myBegin(begin),
    //         myEnd(end),
    //         myWeightPeriod(weightPeriod),
    //         myMaxNumInstances(numThreads) {
    //     }
    #[allow(unused_variables)]
    pub fn new(edges: &i32, ignoreErrors: bool, operation: SUMOAbstractRouter_Operation, begin: SUMOTime, end: SUMOTime, weightPeriod: SUMOTime, havePermissions: bool, numThreads: i32) -> Self {
        todo!("port CHRouterWrapper::CHRouterWrapper<E, V>")
    }

    /// C++: `void CHRouterWrapper::~CHRouterWrapper<E, V>()`
    // ~CHRouterWrapper() {
    //         for (typename RouterMap::iterator i = myRouters.begin(); i != myRouters.end(); ++i) {
    //             delete i->second;
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port CHRouterWrapper::~CHRouterWrapper<E, V>")
    }

    /// C++: `void CHRouterWrapper::prohibit(const int &)` [virtual]
    // virtual void prohibit(const std::map<const E*, RouterProhibition>& toProhibit) {
    //         if (toProhibit.size() > 0) {
    //             WRITE_WARNINGF(TL("Routing algorithm CHWrapper does not support dynamic closing of edges%"), "");
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn prohibit(&mut self, toProhibit: &i32) {
        todo!("port CHRouterWrapper::prohibit")
    }

    /// C++: `bool CHRouterWrapper::supportsProhibitions() const` [const, virtual]
    // virtual bool supportsProhibitions() const {
    //         WRITE_WARNINGF(TL("Routing algorithm CHWrapper does not support dynamic closing of edges and lanes%"), "");
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn supports_prohibitions(&self) -> bool {
        todo!("port CHRouterWrapper::supportsProhibitions")
    }

    /// C++: `SUMOAbstractRouter<E, V> * CHRouterWrapper::clone()` [virtual]
    // virtual SUMOAbstractRouter<E, V>* clone() {
    //         CHRouterWrapper<E, V>* clone = new CHRouterWrapper<E, V>(myEdges, myIgnoreErrors, this->myOperation, myBegin, myEnd, myWeightPeriod, this->myHavePermissions, myMaxNumInstances);
    //         for (const auto& item : myRouters) {
    //             clone->myRouters[item.first] = static_cast<CHRouterType*>(item.second->clone());
    //         }
    //         return clone;
    //     }
    #[allow(unused_variables)]
    pub fn clone(&mut self) -> Ptr<SUMOAbstractRouter<E, V>> {
        todo!("port CHRouterWrapper::clone")
    }

    /// C++: `void CHRouterWrapper::setMsgHandler(MsgHandler *const)`
    // void setMsgHandler(MsgHandler* const errorMsgHandler) {
    //         this->myErrorMsgHandler = errorMsgHandler;
    //         for (auto item : myRouters) {
    //             item.second->setMsgHandler(errorMsgHandler);
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn set_msg_handler(&mut self, errorMsgHandler: Ptr<MsgHandler>) {
        todo!("port CHRouterWrapper::setMsgHandler")
    }

    /// C++: `bool CHRouterWrapper::compute(const E *, const E *, const V *const, SUMOTime, int &, bool)`
    // bool compute(const E* from, const E* to, const V* const vehicle,
    //                  SUMOTime msTime, std::vector<const E*>& into, bool silent = false) {
    //         const std::pair<const SUMOVehicleClass, const double> svc = std::make_pair(vehicle->getVClass(), vehicle->getMaxSpeed());
    //         if (myRouters.count(svc) == 0) {
    //             // create new router for the given permissions and maximum speed
    //             // XXX a new router may also be needed if vehicles differ in speed factor
    //             myRouters[svc] = new CHRouterType(myEdges, myIgnoreErrors, this->myOperation, svc.first, myWeightPeriod, false, false);
    //             myRouters[svc]->setMsgHandler(this->myErrorMsgHandler);
    //         }
    //         return myRouters[svc]->compute(from, to, vehicle, msTime, into, silent);
    //     }
    #[allow(unused_variables)]
    pub fn compute(&mut self, from: Ptr<E>, to: Ptr<E>, vehicle: Ptr<V>, msTime: SUMOTime, into: &mut i32, silent: bool) -> bool {
        todo!("port CHRouterWrapper::compute")
    }

}

