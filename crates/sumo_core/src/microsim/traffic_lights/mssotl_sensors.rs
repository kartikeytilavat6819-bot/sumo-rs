//! Port of microsim/traffic_lights/MSSOTLSensors.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `MSSOTLSensors` (microsim/traffic_lights/MSSOTLSensors.h)
pub struct MSSOTLSensors {
    pub myPhases: Ptr<MSTrafficLightLogic_Phases>,
    pub tlLogicID: String,
    pub currentStep: i32,
}

impl MSSOTLSensors {
    /// C++: `void MSSOTLSensors::buildSensorForLane(MSLane *, NLDetectorBuilder &)` [virtual, pure virtual, protected]
    /// This function member has to be extended to properly build a sensor for a specific input lane Sensors has to be constrained on lane dimension Built sensors has to be collected and associated properly to the respective MSLane for retrieval using MSSOTLSensors::countVehicles(std::string) See extension classes for further specifications on sensor building
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_sensor_for_lane(&mut self, lane: Ptr<MSLane>, nb: &mut NLDetectorBuilder) {
        panic!("MSSOTLSensors::buildSensorForLane is pure virtual; subclasses must override it")
    }

    /// C++: `void MSSOTLSensors::buildSensorForOutLane(MSLane *, NLDetectorBuilder &)` [virtual, pure virtual, protected]
    /// This function member has to be extended to properly build a sensor for a specific output lane Sensors has to be constrained on lane dimension Built sensors has to be collected and associated properly to the respective MSLane for retrieval using MSSOTLSensors::countVehicles(std::string) See extension classes for further specifications on sensor building
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_sensor_for_out_lane(&mut self, lane: Ptr<MSLane>, nb: &mut NLDetectorBuilder) {
        panic!("MSSOTLSensors::buildSensorForOutLane is pure virtual; subclasses must override it")
    }

    /// C++: `void MSSOTLSensors::MSSOTLSensors(std::string, const MSTrafficLightLogic::Phases *)`
    // MSSOTLSensors::MSSOTLSensors(std::string tlLogicID, const MSTrafficLightLogic::Phases* phases) {
    //     this->tlLogicID = tlLogicID;
    //     this->myPhases = phases;
    // }
    #[allow(unused_variables)]
    pub fn new(tlLogicID: String, phases: Ptr<MSTrafficLightLogic_Phases>) -> Self {
        // shortcut: C++ leaves currentStep uninitialized, Rust starts it at 0.
        Self { myPhases: phases, tlLogicID, currentStep: 0 }
    }

    /// C++: `void MSSOTLSensors::~MSSOTLSensors() noexcept` [virtual]
    // MSSOTLSensors::~MSSOTLSensors() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor is empty.
    }

    /// C++: `void MSSOTLSensors::buildSensors(MSTrafficLightLogic::LaneVectorVector, NLDetectorBuilder &)` [virtual, pure virtual]
    /// This function member has to be extended to properly build sensors for the input lanes Sensors has to be constrained on lane dimension Built sensors has to be collected and associated properly to theirs MSLane for retrieval using MSSOTLSensors::countVehicles(std::string) See extension classes for further specifications on sensor building
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_sensors(&mut self, controlledLanes: MSTrafficLightLogic_LaneVectorVector, nb: &mut NLDetectorBuilder) {
        panic!("MSSOTLSensors::buildSensors is pure virtual; subclasses must override it")
    }

    /// C++: `void MSSOTLSensors::buildOutSensors(MSTrafficLightLogic::LaneVectorVector, NLDetectorBuilder &)` [virtual, pure virtual]
    /// This function member has to be extended to properly build sensors for the output lanes Sensors has to be constrained on lane dimension Built sensors has to be collected and associated properly to theirs MSLane for retrieval using MSSOTLSensors::countVehicles(std::string) See extension classes for further specifications on sensor building
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_out_sensors(&mut self, controlledLanes: MSTrafficLightLogic_LaneVectorVector, nb: &mut NLDetectorBuilder) {
        panic!("MSSOTLSensors::buildOutSensors is pure virtual; subclasses must override it")
    }

    /// C++: `int MSSOTLSensors::countVehicles(MSLane *)` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn count_vehicles(&mut self, lane: Ptr<MSLane>) -> i32 {
        panic!("MSSOTLSensors::countVehicles is pure virtual; subclasses must override it")
    }

    /// C++: `int MSSOTLSensors::countVehicles(std::string)` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn count_vehicles_2(&mut self, laneId: String) -> i32 {
        panic!("MSSOTLSensors::countVehicles is pure virtual; subclasses must override it")
    }

    /// C++: `double MSSOTLSensors::meanVehiclesSpeed(MSLane *)` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn mean_vehicles_speed(&mut self, lane: Ptr<MSLane>) -> f64 {
        panic!("MSSOTLSensors::meanVehiclesSpeed is pure virtual; subclasses must override it")
    }

    /// C++: `double MSSOTLSensors::meanVehiclesSpeed(std::string)` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn mean_vehicles_speed_2(&mut self, laneId: String) -> f64 {
        panic!("MSSOTLSensors::meanVehiclesSpeed is pure virtual; subclasses must override it")
    }

    /// C++: `double MSSOTLSensors::getMaxSpeed(std::string)` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_max_speed(&mut self, laneId: String) -> f64 {
        panic!("MSSOTLSensors::getMaxSpeed is pure virtual; subclasses must override it")
    }

    /// C++: `void MSSOTLSensors::stepChanged(int)` [virtual]
    // virtual void stepChanged(int newStep) {
    //         currentStep = newStep;
    //     }
    #[allow(unused_variables)]
    pub fn step_changed(&mut self, newStep: i32) {
        self.currentStep = newStep;
    }

}

