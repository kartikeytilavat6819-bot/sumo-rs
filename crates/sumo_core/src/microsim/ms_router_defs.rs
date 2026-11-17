//! Port of microsim/MSRouterDefs.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `CCHGraph<MSEdge, SUMOVehicle> MSCCHGraph`
pub type MSCCHGraph = CCHGraph<MSEdge, SUMOVehicle>;

/// C++ typedef `CCHMetricFamily<MSEdge, SUMOVehicle, MSVehicleType> MSCCHMetricFamily`
pub type MSCCHMetricFamily = CCHMetricFamily<MSEdge, SUMOVehicle, MSVehicleType>;

/// C++ typedef `std::vector<MSEdge *> MSEdgeVector`
pub type MSEdgeVector = Vec<Ptr<MSEdge>>;

/// C++ typedef `MapMatcher<MSEdge, MSLane, MSJunction> MSMapMatcher`
pub type MSMapMatcher = crate::utils::map_matcher::MapMatcher<MSEdge, MSLane, MSJunction>;

/// C++ typedef `PedestrianRouter<MSEdge, MSLane, MSJunction, SUMOVehicle> MSPedestrianRouter`
pub type MSPedestrianRouter = PedestrianRouter<MSEdge, MSLane, MSJunction, SUMOVehicle>;

/// C++ typedef `RouterProvider<MSEdge, MSLane, MSJunction, SUMOVehicle> MSRouterProvider`
pub type MSRouterProvider = RouterProvider<MSEdge, MSLane, MSJunction, SUMOVehicle>;

/// C++ typedef `IntermodalRouter<MSEdge, MSLane, MSJunction, SUMOVehicle> MSTransportableRouter`
pub type MSTransportableRouter = IntermodalRouter<MSEdge, MSLane, MSJunction, SUMOVehicle>;

/// C++ typedef `SUMOAbstractRouter<MSEdge, SUMOVehicle> MSVehicleRouter`
pub type MSVehicleRouter = SUMOAbstractRouter<MSEdge, SUMOVehicle>;

