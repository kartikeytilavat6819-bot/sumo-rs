//! Port of utils/router/RouterProvider.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// The encapsulation of the routers for vehicles and pedestrians
/// C++ class `RouterProvider` (utils/router/RouterProvider.h)
pub struct RouterProvider<E, L, N, V> {
    pub myVehRouter: Ptr<SUMOAbstractRouter<E, V>>,
    pub myPedRouter: Ptr<PedestrianRouter<E, L, N, V>>,
    pub myInterRouter: Ptr<IntermodalRouter<E, L, N, V>>,
    pub myRailRouter: Ptr<RailwayRouter<E, V>>,
    _p: std::marker::PhantomData<(E, L, N, V,)>,
}

impl<E, L, N, V> RouterProvider<E, L, N, V> {
    /// C++: `void RouterProvider::RouterProvider<E, L, N, V>(SUMOAbstractRouter<E, V> *, PedestrianRouter<E, L, N, V> *, IntermodalRouter<E, L, N, V> *, RailwayRouter<E, V> *)`
    // RouterProvider(SUMOAbstractRouter<E, V>* vehRouter,
    //                    PedestrianRouter<E, L, N, V>* pedRouter,
    //                    IntermodalRouter<E, L, N, V>* interRouter,
    //                    RailwayRouter<E, V>* railRouter) :
    //         myVehRouter(vehRouter),
    //         myPedRouter(pedRouter),
    //         myInterRouter(interRouter),
    //         myRailRouter(railRouter)
    //     {}
    #[allow(unused_variables)]
    pub fn new(vehRouter: Ptr<SUMOAbstractRouter<E, V>>, pedRouter: Ptr<PedestrianRouter<E, L, N, V>>, interRouter: Ptr<IntermodalRouter<E, L, N, V>>, railRouter: Ptr<RailwayRouter<E, V>>) -> Self {
        todo!("port RouterProvider::RouterProvider<E, L, N, V>")
    }

    /// C++: `void RouterProvider::RouterProvider<E, L, N, V>(const RouterProvider<E, L, N, V> &)`
    // RouterProvider(const RouterProvider& original) :
    //         myVehRouter(original.myVehRouter->clone()),
    //         myPedRouter(static_cast<PedestrianRouter<E, L, N, V>*>(original.myPedRouter == nullptr ? nullptr : original.myPedRouter->clone())),
    //         myInterRouter(static_cast<IntermodalRouter<E, L, N, V>*>(original.myInterRouter == nullptr ? nullptr : original.myInterRouter->clone())),
    //         myRailRouter(static_cast<RailwayRouter<E, V>*>(original.myRailRouter == nullptr ? nullptr : original.myRailRouter->clone()))
    //     {}
    #[allow(unused_variables)]
    pub fn new_2(original: &RouterProvider<E, L, N, V>) -> Self {
        todo!("port RouterProvider::RouterProvider<E, L, N, V>")
    }

    /// C++: `void RouterProvider::~RouterProvider<E, L, N, V>()` [virtual]
    // virtual ~RouterProvider() {
    //         delete myVehRouter;
    //         delete myPedRouter;
    //         delete myInterRouter;
    //         delete myRailRouter;
    //     }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port RouterProvider::~RouterProvider<E, L, N, V>")
    }

    /// C++: `RouterProvider<E, L, N, V> * RouterProvider::clone()`
    // RouterProvider* clone() {
    //         return new RouterProvider(*this);
    //     }
    #[allow(unused_variables)]
    pub fn clone(&mut self) -> Ptr<RouterProvider<E, L, N, V>> {
        todo!("port RouterProvider::clone")
    }

    /// C++: `SUMOAbstractRouter<E, V> & RouterProvider::getVehicleRouter(SUMOVehicleClass) const` [const]
    // SUMOAbstractRouter<E, V>& getVehicleRouter(SUMOVehicleClass svc) const {
    //         if (myRailRouter == nullptr || !isRailway(svc)) {
    //             return *myVehRouter;
    //         } else {
    //             return *myRailRouter;
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn get_vehicle_router(&self, svc: SUMOVehicleClass) -> &mut SUMOAbstractRouter<E, V> {
        todo!("port RouterProvider::getVehicleRouter")
    }

    /// C++: `PedestrianRouter<E, L, N, V> & RouterProvider::getPedestrianRouter() const` [const]
    // PedestrianRouter<E, L, N, V>& getPedestrianRouter() const {
    //         return *myPedRouter;
    //     }
    #[allow(unused_variables)]
    pub fn get_pedestrian_router(&self) -> &mut PedestrianRouter<E, L, N, V> {
        todo!("port RouterProvider::getPedestrianRouter")
    }

    /// C++: `IntermodalRouter<E, L, N, V> & RouterProvider::getIntermodalRouter() const` [const]
    // IntermodalRouter<E, L, N, V>& getIntermodalRouter() const {
    //         return *myInterRouter;
    //     }
    #[allow(unused_variables)]
    pub fn get_intermodal_router(&self) -> &mut IntermodalRouter<E, L, N, V> {
        todo!("port RouterProvider::getIntermodalRouter")
    }

    /// C++: `RailwayRouter<E, V> & RouterProvider::getRailwayRouter() const` [const]
    // RailwayRouter<E, V>& getRailwayRouter() const {
    //         return *myRailRouter;
    //     }
    #[allow(unused_variables)]
    pub fn get_railway_router(&self) -> &mut RailwayRouter<E, V> {
        todo!("port RouterProvider::getRailwayRouter")
    }

    /// C++: `void RouterProvider::setBulkMode(const bool) const` [const]
    // inline void setBulkMode(const bool mode) const {
    //         myVehRouter->setBulkMode(mode);
    //         if (myPedRouter != nullptr) {
    //             myPedRouter->setBulkMode(mode);
    //         }
    //         if (myInterRouter != nullptr) {
    //             myInterRouter->setBulkMode(mode);
    //         }
    //         if (myRailRouter != nullptr) {
    //             myRailRouter->setBulkMode(mode);
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn set_bulk_mode(&self, mode: bool) {
        todo!("port RouterProvider::setBulkMode")
    }

    /// C++: `RouterProvider<E, L, N, V> & RouterProvider::operator=(const RouterProvider<E, L, N, V> &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, src: &RouterProvider<E, L, N, V>) -> &mut RouterProvider<E, L, N, V> {
        todo!("port RouterProvider::operator=")
    }

}

