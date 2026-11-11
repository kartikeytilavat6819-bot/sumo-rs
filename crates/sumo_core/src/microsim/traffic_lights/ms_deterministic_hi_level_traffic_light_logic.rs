//! Port of microsim/traffic_lights/MSDeterministicHiLevelTrafficLightLogic.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `MSDeterministicHiLevelTrafficLightLogic` (microsim/traffic_lights/MSDeterministicHiLevelTrafficLightLogic.h)
pub struct MSDeterministicHiLevelTrafficLightLogic {
    /// C++ base class `MSSOTLHiLevelTrafficLightLogic`
    pub base: MSSOTLHiLevelTrafficLightLogic,
    /// This pheronome is an indicator of congestion on input lanes. Its levels refer to the average speed of vehicles passing the input lane: the lower the speed the higher the pheromone. These levels are updated on every input lane, independently on lights state.
    pub inputLanes: MSLaneID_set,
    /// This pheromone is an indicator of congestion on output lanes. Its levels refer to the average speed of vehicles passing the output lane: the lower the speed the higher the pheromone. These levels are updated on every output lane, independently on lights state.
    pub outputLanes: MSLaneID_set,
}

impl std::ops::Deref for MSDeterministicHiLevelTrafficLightLogic {
    type Target = MSSOTLHiLevelTrafficLightLogic;
    fn deref(&self) -> &MSSOTLHiLevelTrafficLightLogic {
        &self.base
    }
}
impl std::ops::DerefMut for MSDeterministicHiLevelTrafficLightLogic {
    fn deref_mut(&mut self) -> &mut MSSOTLHiLevelTrafficLightLogic {
        &mut self.base
    }
}

impl MSDeterministicHiLevelTrafficLightLogic {
    /// C++: `void MSDeterministicHiLevelTrafficLightLogic::MSDeterministicHiLevelTrafficLightLogic(MSTLLogicControl &, const std::string &, const std::string &, const Phases &, int, SUMOTime, const Parameterised::Map &)`
    /// Constructor without sensors passed
    // MSDeterministicHiLevelTrafficLightLogic::MSDeterministicHiLevelTrafficLightLogic(
    //     MSTLLogicControl& tlcontrol, const std::string& id,
    //     const std::string& programID, const Phases& phases, int step,
    //     SUMOTime delay, const Parameterised::Map& parameters) :
    //     MSSOTLHiLevelTrafficLightLogic(tlcontrol, id, programID, TrafficLightType::HILVL_DETERMINISTIC, phases, step,
    //                                    delay, parameters) {
    //
    //     addPolicy(new MSSOTLPlatoonPolicy(new MSSOTLPolicy3DStimulus("PLATOON", parameters), parameters));
    //     addPolicy(new MSSOTLPhasePolicy(new MSSOTLPolicy3DStimulus("PHASE", parameters), parameters));
    //     addPolicy(new MSSOTLMarchingPolicy(new MSSOTLPolicy3DStimulus("MARCHING", parameters), parameters));
    //     addPolicy(new MSSOTLCongestionPolicy(new MSSOTLPolicy3DStimulus("CONGESTION", parameters), parameters));
    //
    // }
    #[allow(unused_variables)]
    pub fn new(tlcontrol: &mut MSTLLogicControl, id: &str, programID: &str, phases: &MSTrafficLightLogic_Phases, step: i32, delay: SUMOTime, parameters: &Unmapped /* Parameterised::Map */) -> Self {
        todo!("port MSDeterministicHiLevelTrafficLightLogic::MSDeterministicHiLevelTrafficLightLogic")
    }

    /// C++: `void MSDeterministicHiLevelTrafficLightLogic::~MSDeterministicHiLevelTrafficLightLogic() noexcept` [virtual]
    // MSDeterministicHiLevelTrafficLightLogic::~MSDeterministicHiLevelTrafficLightLogic() {
    //
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSDeterministicHiLevelTrafficLightLogic::~MSDeterministicHiLevelTrafficLightLogic")
    }

    /// C++: `void MSDeterministicHiLevelTrafficLightLogic::init(NLDetectorBuilder &)` [virtual]
    /// Initialises the tls with sensors on incoming and outgoing lanes Sensors are built in the simulation according to the type of sensor specified in the simulation parameter
    // void MSDeterministicHiLevelTrafficLightLogic::init(NLDetectorBuilder& nb) {
    //     MSSOTLHiLevelTrafficLightLogic::init(nb);
    //     //Setting the startup policy
    //     choosePolicy(0, 0);
    //     WRITE_MESSAGE(
    //         "*** Intersection " + getID()
    //         + " will run using MSDeterministicHiLevelTrafficLightLogic ***");
    //
    //     MSLane* currentLane = nullptr;
    //     for (MSTrafficLightLogic::LaneVectorVector::const_iterator laneVector =
    //                 myLanes.begin(); laneVector != myLanes.end(); laneVector++) {
    //         for (MSTrafficLightLogic::LaneVector::const_iterator lane =
    //                     laneVector->begin(); lane != laneVector->end(); lane++) {
    //             currentLane = (*lane);
    //             if (inputLanes.find(currentLane->getID()) == inputLanes.end()) {
    //                 inputLanes.insert(currentLane->getID());
    // #ifdef SWARM_DEBUG
    //                 WRITE_MESSAGE("*** Intersection " + getID() + " inputLanes adding " + currentLane->getID());
    // #endif
    //             }
    //         }
    //     }
    //
    //     for (const LinkVector& oneLink : getLinks()) {
    //         for (int j = 0; j < (int)oneLink.size(); j++) {
    //             currentLane = oneLink[j]->getLane();
    //             if (outputLanes.find(currentLane->getID()) == outputLanes.end()) {
    //                 outputLanes.insert(currentLane->getID());
    // #ifdef SWARM_DEBUG
    //                 WRITE_MESSAGE("*** Intersection " + getID() + " outputLanes adding " + currentLane->getID());
    // #endif
    //             }
    //         }
    //     }
    //
    // }
    #[allow(unused_variables)]
    pub fn init(&mut self, nb: &mut NLDetectorBuilder) {
        todo!("port MSDeterministicHiLevelTrafficLightLogic::init")
    }

    /// C++: `const std::string MSDeterministicHiLevelTrafficLightLogic::getLogicType() const` [const]
    /// Returns the type of the logic as a string
    // const std::string getLogicType() const {
    //         return "DeterministicHighLevelTrafficLightLogic";
    //     }
    #[allow(unused_variables)]
    pub fn get_logic_type(&self) -> String {
        todo!("port MSDeterministicHiLevelTrafficLightLogic::getLogicType")
    }

    /// C++: `int MSDeterministicHiLevelTrafficLightLogic::decideNextPhase()` [virtual, protected]
    // int MSDeterministicHiLevelTrafficLightLogic::decideNextPhase() {
    // #ifdef SWARM_DEBUG
    //     MsgHandler::getMessageInstance()->inform("\n" + time2string(MSNet::getInstance()->getCurrentTimeStep()) + " MSDeterministicHiLevelTrafficLightLogic decideNextPhase()");
    //     std::ostringstream dnp;
    //     dnp << (MSNet::getInstance()->getCurrentTimeStep()) << " MSDeterministicHiLevelTrafficLightLogic::decideNextPhase:: " << "tlsid=" << getID() << " getCurrentPhaseDef().getState()=" << getCurrentPhaseDef().getState() << " is commit?" << getCurrentPhaseDef().isCommit();
    //     MsgHandler::getMessageInstance()->inform(dnp.str());
    // #endif
    //
    //     //Decide the current policy according to pheromone levels. this should be done only at the end of a chain, before selecting the new one
    //     if (getCurrentPhaseDef().isCommit()) {
    //         decidePolicy();
    //     }
    //
    // #ifdef SWARM_DEBUG
    //     std::ostringstream str;
    //     str << "tlsID=" << getID() << " currentPolicyname=" + getCurrentPolicy()->getName();
    //     WRITE_MESSAGE(str.str());
    // #endif
    //
    //     //Execute current policy. congestion "policy" must maintain the commit phase, and that must be an all-red one
    //     return myCurrentPolicy->decideNextPhase(getCurrentPhaseElapsed(),
    //                                             &getCurrentPhaseDef(), getCurrentPhaseIndex(),
    //                                             getPhaseIndexWithMaxCTS(), isThresholdPassed(), isPushButtonPressed(),
    //                                             countVehicles(getCurrentPhaseDef()));
    // }
    #[allow(unused_variables)]
    pub fn decide_next_phase(&mut self) -> i32 {
        todo!("port MSDeterministicHiLevelTrafficLightLogic::decideNextPhase")
    }

    /// C++: `bool MSDeterministicHiLevelTrafficLightLogic::canRelease()` [virtual, protected]
    // bool MSDeterministicHiLevelTrafficLightLogic::canRelease() {
    // #ifdef SWARM_DEBUG
    //     std::ostringstream phero_str;
    //     phero_str << "getCurrentPhaseElapsed()=" << time2string(getCurrentPhaseElapsed()) << " isThresholdPassed()=" << isThresholdPassed() << " currentPhase=" << (&getCurrentPhaseDef())->getState() << " countVehicles()=" << countVehicles(getCurrentPhaseDef());
    //     WRITE_MESSAGE("\nMSDeterministicHiLevelTrafficLightLogic::canRelease(): " + phero_str.str());
    // #endif
    //     return myCurrentPolicy->canRelease(getCurrentPhaseElapsed(),
    //                                        isThresholdPassed(), isPushButtonPressed(), &getCurrentPhaseDef(),
    //                                        countVehicles(getCurrentPhaseDef()));
    // }
    #[allow(unused_variables)]
    pub fn can_release(&mut self) -> bool {
        todo!("port MSDeterministicHiLevelTrafficLightLogic::canRelease")
    }

    /// C++: `double MSDeterministicHiLevelTrafficLightLogic::getMeanSpeedForInputLanes()` [protected]
    // double MSDeterministicHiLevelTrafficLightLogic::getMeanSpeedForInputLanes() {
    //     if (inputLanes.size() == 0) {
    //         return 0;
    //     }
    //     double vSpeedInTot = 0;
    //     for (MSLaneID_set::iterator laneIterator = inputLanes.begin();
    //             laneIterator != inputLanes.end(); laneIterator++) {
    //         std::string laneId = *laneIterator;
    //         double maxSpeed = getSensors()->meanVehiclesSpeed(laneId);
    //         if (maxSpeed > -1) {
    //             vSpeedInTot += (13.89 - maxSpeed) * 10. / 13.89;
    //         }
    // #ifdef SWARM_DEBUG
    //         std::ostringstream i_str;
    //         i_str << " meanVehiclesSpeed " << maxSpeed << " inputLane " << laneId << " ID " << getID() << " .";
    //         WRITE_MESSAGE(time2string(MSNet::getInstance()->getCurrentTimeStep()) + " MSDeterministicHiLevelTrafficLightLogic::getMeanSpeedForInputLanes:: in" + i_str.str());
    // #endif
    //     }
    //     return vSpeedInTot / (double)inputLanes.size();
    // }
    #[allow(unused_variables)]
    pub fn get_mean_speed_for_input_lanes(&mut self) -> f64 {
        todo!("port MSDeterministicHiLevelTrafficLightLogic::getMeanSpeedForInputLanes")
    }

    /// C++: `double MSDeterministicHiLevelTrafficLightLogic::getMeanSpeedForOutputLanes()` [protected]
    // double MSDeterministicHiLevelTrafficLightLogic::getMeanSpeedForOutputLanes() {
    //     if (outputLanes.size() == 0) {
    //         return 0;
    //     }
    //     double vSpeedOutTot = 0;
    //     for (MSLaneID_set::iterator laneIterator = outputLanes.begin();
    //             laneIterator != outputLanes.end(); laneIterator++) {
    //         std::string laneId = *laneIterator;
    //         double maxSpeed = getSensors()->meanVehiclesSpeed(laneId);
    //         if (maxSpeed > -1) {
    //             vSpeedOutTot += (13.89 - maxSpeed) * 10. / 13.89;
    //         }
    // #ifdef SWARM_DEBUG
    //         std::ostringstream i_str;
    //         i_str << " meanVehiclesSpeed " << maxSpeed << " outputLane " << laneId << " ID " << getID() << " .";
    //         WRITE_MESSAGE(time2string(MSNet::getInstance()->getCurrentTimeStep()) + " MSDeterministicHiLevelTrafficLightLogic::getMeanSpeedForOutputLanes:: out" + i_str.str());
    // #endif
    //     }
    //     return vSpeedOutTot / (double)outputLanes.size();
    // }
    #[allow(unused_variables)]
    pub fn get_mean_speed_for_output_lanes(&mut self) -> f64 {
        todo!("port MSDeterministicHiLevelTrafficLightLogic::getMeanSpeedForOutputLanes")
    }

    /// C++: `void MSDeterministicHiLevelTrafficLightLogic::decidePolicy()` [virtual, protected]
    /// Decide the current policy according to pheromone levels The decision reflects on currentPolicy value
    // void MSDeterministicHiLevelTrafficLightLogic::decidePolicy() {
    //     // Decide if it is the case to check for another plan
    //     double mean_vSpeed_in = getMeanSpeedForInputLanes();
    //     double mean_vSpeed_out = getMeanSpeedForOutputLanes();
    // #ifdef SWARM_DEBUG
    //     MSSOTLPolicy* oldPolicy = getCurrentPolicy();
    // #endif
    //     choosePolicy(mean_vSpeed_in, mean_vSpeed_out);
    //
    // #ifdef SWARM_DEBUG
    //     MSSOTLPolicy* newPolicy = getCurrentPolicy();
    //     if (newPolicy != oldPolicy) {
    //         SUMOTime step = MSNet::getInstance()->getCurrentTimeStep();
    //         std::ostringstream phero_str;
    //         phero_str << " (mean_vSpeed_in= " << mean_vSpeed_in << " ,mean_vSpeed_out= " << mean_vSpeed_out << " )";
    //         WRITE_MESSAGE("TL " + getID() + " time=" + time2string(step) + " Policy: " + newPolicy->getName() + phero_str.str() + " OldPolicy: " + oldPolicy->getName() + " id " + getID() + " .");
    //     } else { //debug purpose only
    //         std::ostringstream phero_str;
    //         phero_str << " (mean_vSpeed_in= " << mean_vSpeed_in << " ,mean_vSpeed_out= " << mean_vSpeed_out << " )";
    //         SUMOTime step = MSNet::getInstance()->getCurrentTimeStep();
    //         WRITE_MESSAGE("TL " + getID() + " time=" + time2string(step) + " Policy: Nochanges" + phero_str.str() + " OldPolicy: " + oldPolicy->getName() + " id " + getID() + " .");
    //     }
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn decide_policy(&mut self) {
        todo!("port MSDeterministicHiLevelTrafficLightLogic::decidePolicy")
    }

    /// C++: `void MSDeterministicHiLevelTrafficLightLogic::choosePolicy(double, double)` [protected]
    // void MSDeterministicHiLevelTrafficLightLogic::choosePolicy(
    //     double mean_vSpeed_in, double mean_vSpeed_out) {
    //
    //     int index_maxStimulus = 0;
    //     double maxStimulus = -1;
    //     // Compute simulus for each policy
    //     for (int i = 0; i < (int)myPolicies.size(); i++) {
    //         double stimulus = myPolicies[i]->computeDesirability(mean_vSpeed_in,
    //                           mean_vSpeed_out);
    //         if (stimulus > maxStimulus) {
    //             maxStimulus = stimulus;
    //             index_maxStimulus = i;
    //         }
    // #ifdef SWARM_DEBUG
    //         std::ostringstream so_str;
    //         so_str << " policy " << getPolicies()[i]->getName() << " stimulus " << stimulus;
    //         WRITE_MESSAGE("MSDeterministicHiLevelTrafficLightLogic::choosePolicy::" + so_str.str());
    // #endif
    //     }
    //     activate(myPolicies[index_maxStimulus]);
    //
    // }
    #[allow(unused_variables)]
    pub fn choose_policy(&mut self, mean_vSpeed_in: f64, mean_vSpeed_out: f64) {
        todo!("port MSDeterministicHiLevelTrafficLightLogic::choosePolicy")
    }

}

