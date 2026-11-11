//! Port of microsim/traffic_lights/MSSOTLTrafficLightLogic.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::map<const std::string, std::vector<MSPushButton *>> MSSOTLTrafficLightLogic::PhasePushButtons`
pub type MSSOTLTrafficLightLogic_PhasePushButtons = BTreeMap<String, Vec<Ptr<MSPushButton>>>;

/// A self-organizing traffic light logic
/// C++ class `MSSOTLTrafficLightLogic` (microsim/traffic_lights/MSSOTLTrafficLightLogic.h)
pub struct MSSOTLTrafficLightLogic {
    /// C++ base class `MSSimpleTrafficLightLogic`
    pub base: MSSimpleTrafficLightLogic,
    pub m_pushButtons: MSSOTLTrafficLightLogic_PhasePushButtons,
    pub mySensors: Ptr<MSSOTLSensors>,
    pub myCountSensors: Ptr<MSSOTLE2Sensors>,
    pub sensorsSelfBuilt: bool,
    pub targetPhasesCTS: BTreeMap<i32, SUMOTime>,
    pub lastCheckForTargetPhase: BTreeMap<i32, SUMOTime>,
    pub targetPhasesLastSelection: BTreeMap<i32, i32>,
    pub lastChain: i32,
    pub decayThreshold: f64,
}

impl std::ops::Deref for MSSOTLTrafficLightLogic {
    type Target = MSSimpleTrafficLightLogic;
    fn deref(&self) -> &MSSimpleTrafficLightLogic {
        &self.base
    }
}
impl std::ops::DerefMut for MSSOTLTrafficLightLogic {
    fn deref_mut(&mut self) -> &mut MSSimpleTrafficLightLogic {
        &mut self.base
    }
}

impl MSSOTLTrafficLightLogic {
    /// C++: `void MSSOTLTrafficLightLogic::MSSOTLTrafficLightLogic(MSTLLogicControl &, const std::string &, const std::string &, const TrafficLightType, const Phases &, int, SUMOTime, const Parameterised::Map &)`
    /// Constructor without sensors passed
    // MSSOTLTrafficLightLogic::MSSOTLTrafficLightLogic(
    //     MSTLLogicControl& tlcontrol,
    //     const std::string& id,
    //     const std::string& programID,
    //     const TrafficLightType logicType,
    //     const Phases& phases,
    //     int step,
    //     SUMOTime delay,
    //     const Parameterised::Map& parameters) :
    //     MSSimpleTrafficLightLogic(tlcontrol, id, programID, 0, logicType, phases, step, delay, parameters) {
    //     this->mySensors = nullptr;
    //     this->myCountSensors = nullptr;
    //     sensorsSelfBuilt = true;
    //     checkPhases();
    //     setupCTS();
    //     setToATargetPhase();
    // }
    #[allow(unused_variables)]
    pub fn new(tlcontrol: &mut MSTLLogicControl, id: &str, programID: &str, logicType: TrafficLightType, phases: &MSTrafficLightLogic_Phases, step: i32, delay: SUMOTime, parameters: &Unmapped /* Parameterised::Map */) -> Self {
        todo!("port MSSOTLTrafficLightLogic::MSSOTLTrafficLightLogic")
    }

    /// C++: `void MSSOTLTrafficLightLogic::MSSOTLTrafficLightLogic(MSTLLogicControl &, const std::string &, const std::string &, const TrafficLightType, const Phases &, int, SUMOTime, const Parameterised::Map &, MSSOTLSensors *)`
    /// Constructor with sensors passed
    // MSSOTLTrafficLightLogic::MSSOTLTrafficLightLogic(
    //     MSTLLogicControl& tlcontrol,
    //     const std::string& id,
    //     const std::string& programID,
    //     const TrafficLightType logicType,
    //     const Phases& phases,
    //     int step,
    //     SUMOTime delay,
    //     const Parameterised::Map& parameters,
    //     MSSOTLSensors* sensors) :
    //     MSSimpleTrafficLightLogic(tlcontrol, id, programID, 0, logicType, phases, step, delay, parameters) {
    //     this->mySensors = sensors;
    //     sensorsSelfBuilt = false;
    //     checkPhases();
    //     setupCTS();
    //     setToATargetPhase();
    // }
    #[allow(unused_variables)]
    pub fn new_2(tlcontrol: &mut MSTLLogicControl, id: &str, programID: &str, logicType: TrafficLightType, phases: &MSTrafficLightLogic_Phases, step: i32, delay: SUMOTime, parameters: &Unmapped /* Parameterised::Map */, sensors: Ptr<MSSOTLSensors>) -> Self {
        todo!("port MSSOTLTrafficLightLogic::MSSOTLTrafficLightLogic")
    }

    /// C++: `void MSSOTLTrafficLightLogic::~MSSOTLTrafficLightLogic() noexcept` [virtual]
    /// Destructor
    // MSSOTLTrafficLightLogic::~MSSOTLTrafficLightLogic() {
    //     for (PhasePushButtons::iterator mapIt = m_pushButtons.begin(); mapIt != m_pushButtons.end(); ++mapIt)
    //         for (std::vector<MSPushButton*>::iterator vIt = mapIt->second.begin(); vIt != mapIt->second.end(); ++vIt) {
    //             delete *vIt;
    //         }
    //     m_pushButtons.clear();
    //     if (sensorsSelfBuilt) {
    //         delete mySensors;
    // //		delete myCountSensors;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSSOTLTrafficLightLogic::~MSSOTLTrafficLightLogic")
    }

    /// C++: `void MSSOTLTrafficLightLogic::init(NLDetectorBuilder &)` [virtual]
    /// Initialises the tls with sensors on incoming and outgoing lanes Sensors are built in the simulation according to the type of sensor specified in the simulation parameter
    // void
    // MSSOTLTrafficLightLogic::init(NLDetectorBuilder& nb) {
    //
    //     MSTrafficLightLogic::init(nb);
    //
    //     if (isDecayThresholdActivated()) {
    //         decayThreshold = 1;
    //     }
    //     if (sensorsSelfBuilt) {
    //         //Building SOTLSensors
    //         switch (SENSORS_TYPE) {
    //             case SENSORS_TYPE_E1:
    //                 throw ProcessError(TL("SOTL can only handle E2 sensors."));
    //             case SENSORS_TYPE_E2:
    //
    //                 //Adding Sensors to the ingoing Lanes
    //
    //                 LaneVectorVector lvv = getLaneVectors();
    //
    // #ifdef SWARM_DEBUG
    //                 WRITE_MESSAGE("Listing lanes for TLS " + getID());
    //
    //                 for (int i = 0; i < lvv.size(); i++) {
    //                     LaneVector lv = lvv[i];
    //
    //                     for (int j = 0; j < lv.size(); j++) {
    //                         MSLane* lane = lv[j];
    //                         WRITE_MESSAGE(lane ->getID());
    //                     }
    //                 }
    // #endif
    //
    //                 mySensors = new MSSOTLE2Sensors(myID, &(getPhases()));
    //                 ((MSSOTLE2Sensors*)mySensors)->buildSensors(myLanes, nb, getInputSensorsLength());
    //                 mySensors->stepChanged(getCurrentPhaseIndex());
    //                 if (getParameter("USE_VEHICLE_TYPES_WEIGHTS", "0") == "1") {
    //                     ((MSSOTLE2Sensors*) mySensors)->setVehicleWeigths(getParameter("VEHICLE_TYPES_WEIGHTS", ""));
    //                 }
    //
    //                 //threshold speed param for tuning with irace
    //                 ((MSSOTLE2Sensors*)mySensors)->setSpeedThresholdParam(getSpeedThreshold());
    //
    //                 myCountSensors = new MSSOTLE2Sensors(myID + "Count", &(getPhases()));
    //                 myCountSensors->buildCountSensors(myLanes, nb);
    //                 myCountSensors->stepChanged(getCurrentPhaseIndex());
    //
    //                 //Adding Sensors to the outgoing Lanes
    //
    //                 LinkVectorVector links = getLinks();
    //
    // #ifdef SWARM_DEBUG
    //                 WRITE_MESSAGE(TL("Listing output lanes"));
    //                 for (int i = 0; i < links.size(); i++) {
    //                     LinkVector oneLink = getLinksAt(i);
    //                     for (int j = 0; j < oneLink.size(); j++) {
    //                         MSLane* lane  = oneLink[j]->getLane();
    //                         WRITE_MESSAGE(lane ->getID());
    //                     }
    //                 }
    // #endif
    //
    //                 LaneVectorVector myLaneVector;
    //                 LaneVector outLanes;
    //                 LinkVectorVector myoutLinks = getLinks();
    //
    //                 for (int i = 0; i < (int)links.size(); i++) {
    //                     LinkVector oneLink = getLinksAt(i);
    //                     for (int j = 0; j < (int)oneLink.size(); j++) {
    //                         MSLane* lane  = oneLink[j]->getLane();
    //                         outLanes.push_back(lane);
    //                     }
    //                 }
    //
    //                 if (outLanes.size() > 0) {
    //                     myLaneVector.push_back(outLanes);
    //                 }
    //                 if (myLaneVector.size() > 0) {
    //                     ((MSSOTLE2Sensors*)mySensors)->buildOutSensors(myLaneVector, nb, getOutputSensorsLength());
    //                     myCountSensors->buildCountOutSensors(myLaneVector, nb);
    //                 }
    //
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn init(&mut self, nb: &mut NLDetectorBuilder) {
        todo!("port MSSOTLTrafficLightLogic::init")
    }

    /// C++: `SUMOTime MSSOTLTrafficLightLogic::trySwitch()` [virtual]
    // SUMOTime
    // MSSOTLTrafficLightLogic::trySwitch() {
    //     if (MSNet::getInstance()->getCurrentTimeStep() % 1000 == 0) {
    //         WRITE_MESSAGE("MSSOTLTrafficLightLogic::trySwitch()");
    //         // To check if decideNextPhase changes the step
    //         int previousStep = getCurrentPhaseIndex() ;
    // #ifdef ANALYSIS_DEBUG
    //         SUMOTime elapsed = getCurrentPhaseElapsed();
    // #endif
    //         // Update CTS according to sensors
    //         updateCTS();
    //
    //         // Invoking the function member, specialized for each SOTL logic
    //         setStep(decideNextPhase());
    //         MSPhaseDefinition currentPhase = getCurrentPhaseDef();
    //
    //         //At the end, check if new step started
    //         if (getCurrentPhaseIndex() != previousStep) {
    //             //Check if a new steps chain started
    //             if (currentPhase.isTarget())  {
    //                 //Reset CTS for the ending steps chain
    //                 resetCTS(lastChain);
    //                 //Update lastTargetPhase
    //                 lastChain = getCurrentPhaseIndex();
    //                 for (std::map<int, int>::iterator it = targetPhasesLastSelection.begin(); it != targetPhasesLastSelection.end(); ++ it) {
    //                     if (it->first == lastChain) {
    //                         if (it->second >= getTargetPhaseMaxLastSelection()) {
    //                             std::ostringstream oss;
    //                             oss << "Forced selection of the phase " << lastChain << " since its last selection was " << it->second << " changes ago";
    //                             WRITE_MESSAGE(oss.str())
    //                         }
    //                         it->second = 0;
    //                     } else if (it->first != previousStep) {
    //                         ++it->second;
    //                     }
    //                 }
    //                 if (isDecayThresholdActivated()) {
    //                     decayThreshold = 1;
    //                 }
    //             }
    //             //Inform the sensors logic
    //             mySensors->stepChanged(getCurrentPhaseIndex());
    //             //Store the time the new phase started
    //             currentPhase.myLastSwitch = MSNet::getInstance()->getCurrentTimeStep();
    //             if (isDecayThresholdActivated()) {
    //                 decayThreshold = 1;
    //             }
    // #ifdef ANALYSIS_DEBUG
    //             std::ostringstream oss;
    //             oss << getID() << " from " << getPhase(previousStep).getState() << " to " << currentPhase.getState() << " after " << time2string(elapsed);
    //             WRITE_MESSAGE(time2string(MSNet::getInstance()->getCurrentTimeStep()) + "\tMSSOTLTrafficLightLogic::trySwitch " + oss.str());
    // #endif
    //         }
    //     }
    //     return computeReturnTime();
    // }
    #[allow(unused_variables)]
    pub fn try_switch(&mut self) -> SUMOTime {
        todo!("port MSSOTLTrafficLightLogic::trySwitch")
    }

    /// C++: `void MSSOTLTrafficLightLogic::logStatus()` [protected]
    // void MSSOTLTrafficLightLogic::logStatus() {
    //
    // }
    #[allow(unused_variables)]
    pub fn log_status(&mut self) {
        todo!("port MSSOTLTrafficLightLogic::logStatus")
    }

    /// C++: `int MSSOTLTrafficLightLogic::decideNextPhase()` [virtual, protected]
    // int
    // MSSOTLTrafficLightLogic::decideNextPhase() {
    //     MSPhaseDefinition currentPhase = getCurrentPhaseDef();
    //     //If the junction was in a commit step
    //     //=> go to the target step that gives green to the set with the current highest CTS
    //     //   and return computeReturnTime()
    //     if (currentPhase.isCommit()) {
    //         // decide which chain to activate. Gotta work on this
    //         return getPhaseIndexWithMaxCTS();
    //     }
    //     if (currentPhase.isTransient()) {
    //         //If the junction was in a transient step
    //         //=> go to the next step and return computeReturnTime()
    //         return getCurrentPhaseIndex() + 1;
    //     }
    //
    //     if (currentPhase.isDecisional()) {
    //
    //         if (canRelease()) {
    //             return getCurrentPhaseIndex() + 1;
    //         }
    //     }
    //
    //     return getCurrentPhaseIndex();
    // }
    #[allow(unused_variables)]
    pub fn decide_next_phase(&mut self) -> i32 {
        todo!("port MSSOTLTrafficLightLogic::decideNextPhase")
    }

    /// C++: `bool MSSOTLTrafficLightLogic::canRelease()` [virtual, pure virtual, protected]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn can_release(&mut self) -> bool {
        todo!("port MSSOTLTrafficLightLogic::canRelease")
    }

    /// C++: `SUMOTime MSSOTLTrafficLightLogic::getCurrentPhaseElapsed()` [protected]
    // SUMOTime
    // MSSOTLTrafficLightLogic::getCurrentPhaseElapsed() {
    //     MSPhaseDefinition currentPhase = getCurrentPhaseDef();
    //
    //     SUMOTime now = MSNet::getInstance()->getCurrentTimeStep();
    //     SUMOTime elapsed = now - currentPhase.myLastSwitch;
    //
    //     return elapsed;
    // }
    #[allow(unused_variables)]
    pub fn get_current_phase_elapsed(&mut self) -> SUMOTime {
        todo!("port MSSOTLTrafficLightLogic::getCurrentPhaseElapsed")
    }

    /// C++: `int MSSOTLTrafficLightLogic::countVehicles(MSPhaseDefinition)` [protected]
    // int
    // MSSOTLTrafficLightLogic::countVehicles(MSPhaseDefinition phase) {
    //
    //     if (!phase.isTarget()) {
    //         return 0;
    //     }
    //
    //     int accumulator = 0;
    //     //Iterate over the target lanes for the current target phase to get the number of approaching vehicles
    //     for (const std::string& lane : phase.getTargetLaneSet()) {
    //         //SWITCH between 3 counting vehicles function
    //         switch (getMode()) {
    //             case (0):
    //                 accumulator += mySensors->countVehicles(lane); //SUMO
    //                 break;
    //             case (1):
    //                 accumulator += ((MSSOTLE2Sensors*)mySensors)->estimateVehicles(lane);  //COMPLEX
    //                 break;
    //             case (2):
    //                 accumulator = MAX2((int)((MSSOTLE2Sensors*)mySensors)->getEstimateQueueLength(lane), accumulator);  //QUEUE
    //                 break;
    //             default:
    //                 WRITE_ERROR(TL("Unrecognized traffic threshold calculation mode"));
    //         }
    //     }
    //     return accumulator;
    // }
    #[allow(unused_variables)]
    pub fn count_vehicles(&mut self, phase: MSPhaseDefinition) -> i32 {
        todo!("port MSSOTLTrafficLightLogic::countVehicles")
    }

    /// C++: `bool MSSOTLTrafficLightLogic::isThresholdPassed()` [protected]
    // bool
    // MSSOTLTrafficLightLogic::isThresholdPassed() {
    // #ifdef SWARM_DEBUG
    //     //	WRITE_MESSAGEF(TL("\n% tlsid=%  //	WRITE_MESSAGEF(TL("\n% tlsid=" + getID()), ime2string(MSNet::getInstance()->getCurrentTimeStep()) +"\tMSSOTLTrafficLightLogic::isThresholdPassed()::  ", getID()), ime2string(MSNet::getInstance()->getCurrentTimeStep()) +"\tMSSOTLTrafficLightLogic::isThresholdPassed()::  ");
    //
    //     std::ostringstream threshold_str;
    //     //	threshold_str << "tlsid=" << getID() << " targetPhaseCTS size=" << targetPhasesCTS.size();
    // //			threshold_str << "\n";
    //     WRITE_MESSAGE(threshold_str.str());
    // #endif
    //     /*
    //      * if a dynamic threshold based on the exponential decrease, if passed we force the phase change
    //      */
    // //	double random = ((double) RandHelper::rand(RAND_MAX) / (RAND_MAX));
    //     double random = RandHelper::rand();
    // //	ANALYSIS_DBG(
    // #ifdef SWARM_DEBUG
    //     if (isDecayThresholdActivated()) {
    //         std::ostringstream str;
    //         str << time2string(MSNet::getInstance()->getCurrentTimeStep()) << "\tMSSOTLTrafficLightLogic::isThresholdPassed()::  "
    //             << " tlsid=" << getID() << " decayThreshold=" << decayThreshold << " random=" << random << ">" << (1 - decayThreshold)
    //             << (random > (1 - decayThreshold) ? " true" : " false");
    //
    //         WRITE_MESSAGE(str.str());
    //     }
    // #endif
    //     if (!isDecayThresholdActivated() || (isDecayThresholdActivated() && random > (1 - decayThreshold))) {
    //         for (std::map<int, SUMOTime>::const_iterator iterator =
    //                     targetPhasesCTS.begin(); iterator != targetPhasesCTS.end();
    //                 iterator++) {
    // #ifdef SWARM_DEBUG
    //             SUMOTime step = MSNet::getInstance()->getCurrentTimeStep();
    //             std::ostringstream threshold_str;
    //             //	threshold_str <<"\tTL " +getID()<<" time=" +time2string(step)<< "(getThreshold()= " << getThreshold()
    //             //		<< ", targetPhaseCTS= " << iterator->second << " )" << " phase="<<getPhase(iterator->first).getState();
    //             threshold_str << getCurrentPhaseDef().getState() << ";" << time2string(step) << ";" << getThreshold()
    //                           << ";" << iterator->second << ";" << getPhase(iterator->first).getState() << ";"
    //                           << iterator->first << "!=" << lastChain;
    //             WRITE_MESSAGE(threshold_str.str());
    // #endif
    //             //Note that the current chain is not eligible to be directly targeted again, it would be unfair
    //             if ((iterator->first != lastChain) && (getThreshold() <= iterator->second)) {
    //                 return true;
    //             }
    //         }
    //         return false;
    //     } else {
    //         return true;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn is_threshold_passed(&mut self) -> bool {
        todo!("port MSSOTLTrafficLightLogic::isThresholdPassed")
    }

    /// C++: `bool MSSOTLTrafficLightLogic::isPushButtonPressed()` [protected]
    /// Checks the if the pushbutton was activated for the current stage
    // bool MSSOTLTrafficLightLogic::isPushButtonPressed() {
    //     if (getParameter("USE_PUSH_BUTTON", "0") == "0") {
    //         return false;
    //     }
    //     const MSPhaseDefinition currentPhase = getCurrentPhaseDef();
    //     if (m_pushButtons.find(currentPhase.getState()) == m_pushButtons.end()) {
    //         m_pushButtons[currentPhase.getState()] = MSPedestrianPushButton::loadPushButtons(&currentPhase);
    //     }
    //     return MSPushButton::anyActive(m_pushButtons[currentPhase.getState()]);
    // }
    #[allow(unused_variables)]
    pub fn is_push_button_pressed(&mut self) -> bool {
        todo!("port MSSOTLTrafficLightLogic::isPushButtonPressed")
    }

    /// C++: `int MSSOTLTrafficLightLogic::getThreshold()` [protected]
    // int getThreshold() {
    //         return StringUtils::toInt(getParameter("THRESHOLD", "10"));
    //     }
    #[allow(unused_variables)]
    pub fn get_threshold(&mut self) -> i32 {
        todo!("port MSSOTLTrafficLightLogic::getThreshold")
    }

    /// C++: `double MSSOTLTrafficLightLogic::getSpeedThreshold()` [protected]
    // double getSpeedThreshold() {
    //         return StringUtils::toDouble(getParameter("THRESHOLDSPEED", "2"));
    //     }
    #[allow(unused_variables)]
    pub fn get_speed_threshold(&mut self) -> f64 {
        todo!("port MSSOTLTrafficLightLogic::getSpeedThreshold")
    }

    /// C++: `double MSSOTLTrafficLightLogic::getInputSensorsLength()` [protected]
    // double getInputSensorsLength() {
    //         return StringUtils::toDouble(getParameter("INSENSORSLENGTH", "100"));
    //     }
    #[allow(unused_variables)]
    pub fn get_input_sensors_length(&mut self) -> f64 {
        todo!("port MSSOTLTrafficLightLogic::getInputSensorsLength")
    }

    /// C++: `double MSSOTLTrafficLightLogic::getOutputSensorsLength()` [protected]
    // double getOutputSensorsLength() {
    //         return StringUtils::toDouble(getParameter("OUTSENSORSLENGTH", "80"));
    //     }
    #[allow(unused_variables)]
    pub fn get_output_sensors_length(&mut self) -> f64 {
        todo!("port MSSOTLTrafficLightLogic::getOutputSensorsLength")
    }

    /// C++: `int MSSOTLTrafficLightLogic::getPhaseIndexWithMaxCTS()` [protected]
    // int
    // MSSOTLTrafficLightLogic::getPhaseIndexWithMaxCTS() {
    //     SUMOTime maxCTS = 0;
    //     int maxLastStep = getTargetPhaseMaxLastSelection();
    //     bool usedMaxCTS = false;
    //     std::vector<int> equalIndexes;
    //     for (std::map<int, int>::const_iterator it = targetPhasesLastSelection.begin();
    //             it != targetPhasesLastSelection.end(); ++it) {
    //         if (it->first != lastChain) {
    //             if (maxLastStep < it->second) {
    //                 maxLastStep = it->second;
    //                 equalIndexes.clear();
    //                 equalIndexes.push_back(it->first);
    //             } else if (maxLastStep == it->second) {
    //                 equalIndexes.push_back(it->first);
    //             }
    //         }
    //     }
    //     if (equalIndexes.size() == 0) {
    //         usedMaxCTS = true;
    //         for (std::map<int, SUMOTime>::const_iterator iterator = targetPhasesCTS.begin();
    //                 iterator != targetPhasesCTS.end(); ++iterator) {
    //             if (iterator->first != lastChain) {
    //                 if (maxCTS < iterator->second) {
    //                     maxCTS = iterator->second;
    //                     equalIndexes.clear();
    //                     equalIndexes.push_back(iterator->first);
    //                 } else if (maxCTS == iterator->second) {
    //                     equalIndexes.push_back(iterator->first);
    //                 }
    //             }
    //         }
    //     }
    //
    //     std::ostringstream oss;
    //     oss << "MSSOTLTrafficLightLogic::getPhaseIndexWithMaxCTS-> TLC " << getID();
    //     if (usedMaxCTS) {
    //         oss << " maxCTS " << maxCTS;
    //     } else {
    //         oss << " forcing selection since not selected for " << maxLastStep;
    //     }
    //     if (equalIndexes.size() == 1) {
    //         oss << " phase " << equalIndexes[0];
    //         WRITE_MESSAGE(oss.str());
    //         return equalIndexes[0];
    //     } else {
    //         const int index = RandHelper::getRandomFrom(equalIndexes);
    //         oss << " phases [";
    //         for (std::vector<int>::const_iterator it = equalIndexes.begin(); it != equalIndexes.end(); ++it) {
    //             oss << *it << ", ";
    //         }
    //         oss << "]. Random select " << index;
    //         WRITE_MESSAGE(oss.str());
    //         return index;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_phase_index_with_max_cts(&mut self) -> i32 {
        todo!("port MSSOTLTrafficLightLogic::getPhaseIndexWithMaxCTS")
    }

    /// C++: `MSSOTLSensors * MSSOTLTrafficLightLogic::getSensors()` [protected]
    // MSSOTLSensors* getSensors() {
    //         return mySensors;
    //     }
    #[allow(unused_variables)]
    pub fn get_sensors(&mut self) -> Ptr<MSSOTLSensors> {
        todo!("port MSSOTLTrafficLightLogic::getSensors")
    }

    /// C++: `MSSOTLE2Sensors * MSSOTLTrafficLightLogic::getCountSensors()` [protected]
    /// Return the sensors that count the passage of vehicles in and out of the tl.
    // MSSOTLE2Sensors* getCountSensors() {
    //         return myCountSensors;
    //     }
    #[allow(unused_variables)]
    pub fn get_count_sensors(&mut self) -> Ptr<MSSOTLE2Sensors> {
        todo!("port MSSOTLTrafficLightLogic::getCountSensors")
    }

    /// C++: `SUMOTime MSSOTLTrafficLightLogic::computeReturnTime()` [virtual, protected]
    // virtual SUMOTime computeReturnTime() {
    //         if (getCurrentPhaseDef().isTransient()) {
    //             return getCurrentPhaseDef().duration;
    //
    //         }
    //         return DELTA_T;
    //
    //     }
    #[allow(unused_variables)]
    pub fn compute_return_time(&mut self) -> SUMOTime {
        todo!("port MSSOTLTrafficLightLogic::computeReturnTime")
    }

    /// C++: `void MSSOTLTrafficLightLogic::setStep(int)` [protected]
    /// Forces a specific step
    // void MSSOTLTrafficLightLogic::setStep(int step) {
    //     step = step % myPhases.size();
    //     if (myStep != step) {
    //         myStep = step;
    //         myPhases[myStep]->myLastSwitch = MSNet::getInstance()->getCurrentTimeStep();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_step(&mut self, step: i32) {
        todo!("port MSSOTLTrafficLightLogic::setStep")
    }

    /// C++: `int MSSOTLTrafficLightLogic::getTargetPhaseMaxLastSelection()` [private]
    // int getTargetPhaseMaxLastSelection() {
    //         //return 2 * targetPhasesCTS.size() - 1;
    //         return (int)targetPhasesCTS.size() - 1;
    //     }
    #[allow(unused_variables)]
    pub fn get_target_phase_max_last_selection(&mut self) -> i32 {
        todo!("port MSSOTLTrafficLightLogic::getTargetPhaseMaxLastSelection")
    }

    /// C++: `void MSSOTLTrafficLightLogic::checkPhases()` [private]
    // void
    // MSSOTLTrafficLightLogic::checkPhases() {
    //     for (int step = 0; step < (int)getPhases().size(); step++) {
    //         if (getPhase(step).isUndefined()) {
    //             MsgHandler::getErrorInstance()->inform("Step " + toString(step) + " of traffic light logic " + myID + " phases declaration has its type undeclared!");
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn check_phases(&mut self) {
        todo!("port MSSOTLTrafficLightLogic::checkPhases")
    }

    /// C++: `void MSSOTLTrafficLightLogic::setToATargetPhase()` [private]
    // void
    // MSSOTLTrafficLightLogic::setToATargetPhase() {
    //     for (int step = 0; step < (int)getPhases().size(); step++) {
    //         if (getPhase(step).isTarget()) {
    //             setStep(step);
    //             lastChain = step;
    //             return;
    //         }
    //     }
    //     MsgHandler::getErrorInstance()->inform("No phase of type target found for traffic light logic " + myID + " The logic could malfunction. Check phases declaration.");
    // }
    #[allow(unused_variables)]
    pub fn set_to_a_target_phase(&mut self) {
        todo!("port MSSOTLTrafficLightLogic::setToATargetPhase")
    }

    /// C++: `void MSSOTLTrafficLightLogic::setupCTS()` [private]
    // void
    // MSSOTLTrafficLightLogic::setupCTS() {
    //     for (int phaseStep = 0; phaseStep < (int)getPhases().size(); phaseStep++) {
    //         if (getPhase(phaseStep).isTarget()) {
    //             targetPhasesCTS[phaseStep] = 0;
    //             lastCheckForTargetPhase[phaseStep] = MSNet::getInstance()->getCurrentTimeStep();
    //             targetPhasesLastSelection[phaseStep] = 0;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn setup_cts(&mut self) {
        todo!("port MSSOTLTrafficLightLogic::setupCTS")
    }

    /// C++: `void MSSOTLTrafficLightLogic::updateCTS()` [private]
    // void
    // MSSOTLTrafficLightLogic::updateCTS() {
    //     SUMOTime elapsedTimeSteps = 0;
    //     SUMOTime now = MSNet::getInstance()->getCurrentTimeStep();
    //     //Iterate over the target phase map and update CTS value for every target phase except for the one belonging to the current steps chain
    //     for (std::map<int, SUMOTime>::iterator mapIterator = targetPhasesCTS.begin();
    //             mapIterator != targetPhasesCTS.end();
    //             mapIterator++) {
    //         int chain = mapIterator->first;
    //         SUMOTime oldVal = mapIterator->second;
    //         if (chain != lastChain) {
    //             //Get the number of timesteps since the last check for that phase
    //             elapsedTimeSteps = now - lastCheckForTargetPhase[chain];
    //             //Update the last check time
    //             lastCheckForTargetPhase[chain] = now;
    //             //Increment the CTS
    //             //SWITCH between 3 counting vehicles function
    //             switch (getMode()) {
    //                 case (0):
    //                     mapIterator->second += elapsedTimeSteps
    //                                            * countVehicles(getPhase(chain)); //SUMO
    //                     break;
    //                 case (1):
    //                     mapIterator->second += elapsedTimeSteps
    //                                            * countVehicles(getPhase(chain)); //COMPLEX
    //                     break;
    //                 case (2):
    //                     mapIterator->second = countVehicles(getPhase(chain)); //QUEUE
    //                     break;
    //                 default:
    //                     WRITE_ERROR(TL("Unrecognized traffic threshold calculation mode"));
    //             }
    //             std::ostringstream oss;
    //             oss << "MSSOTLTrafficLightLogic::updateCTS->TLC " << getID() << " chain " << chain << " oldVal " << oldVal << " newVal " << mapIterator->second;
    //             WRITE_MESSAGE(oss.str());
    //         }
    //         if (isDecayThresholdActivated()) {
    //             updateDecayThreshold();
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn update_cts(&mut self) {
        todo!("port MSSOTLTrafficLightLogic::updateCTS")
    }

    /// C++: `void MSSOTLTrafficLightLogic::resetCTS(int)` [private]
    // void
    // MSSOTLTrafficLightLogic::resetCTS(int phaseStep) {
    //     std::map<int, SUMOTime>::iterator phaseIterator = targetPhasesCTS.find(phaseStep);
    //     if (phaseIterator != targetPhasesCTS.end()) {
    //         phaseIterator->second = 0;
    //         lastCheckForTargetPhase[phaseStep] = MSNet::getInstance()->getCurrentTimeStep();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn reset_cts(&mut self, phaseStep: i32) {
        todo!("port MSSOTLTrafficLightLogic::resetCTS")
    }

    /// C++: `void MSSOTLTrafficLightLogic::updateDecayThreshold()` [private]
    // void
    // MSSOTLTrafficLightLogic::updateDecayThreshold() {
    //     if (getCurrentPhaseDef().isGreenPhase()) {
    //         decayThreshold = decayThreshold * exp(getDecayConstant());
    //     }
    // #ifdef SWARM_DEBUG
    //     std::stringstream out;
    //     out << decayThreshold;
    //     WRITE_MESSAGE("\n" + time2string(MSNet::getInstance()->getCurrentTimeStep()) + "\tMSSOTLTrafficLightLogic::updateDecayThreshold()::  " + out.str());
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn update_decay_threshold(&mut self) {
        todo!("port MSSOTLTrafficLightLogic::updateDecayThreshold")
    }

    /// C++: `int MSSOTLTrafficLightLogic::getMode()` [private]
    // int getMode() {
    //         return StringUtils::toInt(getParameter("MODE", "0"));
    //     }
    #[allow(unused_variables)]
    pub fn get_mode(&mut self) -> i32 {
        todo!("port MSSOTLTrafficLightLogic::getMode")
    }

    /// C++: `bool MSSOTLTrafficLightLogic::isDecayThresholdActivated()` [private]
    // bool isDecayThresholdActivated() {
    //         return StringUtils::toBool(getParameter("DECAY_THRESHOLD", "0"));
    //     }
    #[allow(unused_variables)]
    pub fn is_decay_threshold_activated(&mut self) -> bool {
        todo!("port MSSOTLTrafficLightLogic::isDecayThresholdActivated")
    }

    /// C++: `double MSSOTLTrafficLightLogic::getDecayConstant()` [private]
    // double getDecayConstant() {
    //         return StringUtils::toDouble(getParameter("DECAY_CONSTANT", "-0.001"));
    //     }
    #[allow(unused_variables)]
    pub fn get_decay_constant(&mut self) -> f64 {
        todo!("port MSSOTLTrafficLightLogic::getDecayConstant")
    }

}

