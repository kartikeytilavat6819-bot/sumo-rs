//! Port of microsim/traffic_lights/MSSOTLDefinitions.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::map<MSLane *, bool> LaneCheckMap`
pub type LaneCheckMap = BTreeMap<Ptr<MSLane>, bool>;

/// C++ typedef `std::vector<std::string> LaneIdVector`
pub type LaneIdVector = Vec<String>;

/// C++ typedef `std::pair<std::string, MSE2Collector *> MSLaneID_MSE2Collector`
pub type MSLaneID_MSE2Collector = (String, Ptr<MSE2Collector>);

/// C++ typedef `std::map<std::string, MSE2Collector *> MSLaneID_MSE2CollectorMap`
pub type MSLaneID_MSE2CollectorMap = BTreeMap<String, Ptr<MSE2Collector>>;

/// C++ typedef `std::pair<std::string, double> MSLaneID_MaxSpeed`
pub type MSLaneID_MaxSpeed = (String, f64);

/// C++ typedef `std::map<std::string, double> MSLaneID_MaxSpeedMap`
pub type MSLaneID_MaxSpeedMap = BTreeMap<String, f64>;

/// C++ typedef `std::set<std::string> MSLaneID_set`
pub type MSLaneID_set = BTreeSet<String>;

/// C++ typedef `std::pair<std::string, double> MSLaneId_Pheromone`
pub type MSLaneId_Pheromone = (String, f64);

/// C++ typedef `std::map<std::string, double> MSLaneId_PheromoneMap`
pub type MSLaneId_PheromoneMap = BTreeMap<String, f64>;

/// C++ typedef `std::pair<MSLane *, MSE2Collector *> MSLane_MSE2Collector`
pub type MSLane_MSE2Collector = (Ptr<MSLane>, Ptr<MSE2Collector>);

/// C++ typedef `std::map<MSLane *, MSE2Collector *> MSLane_MSE2CollectorMap`
pub type MSLane_MSE2CollectorMap = BTreeMap<Ptr<MSLane>, Ptr<MSE2Collector>>;

