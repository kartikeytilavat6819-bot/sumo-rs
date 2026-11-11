//! Port of microsim/traffic_lights/MSDelayBasedTrafficLightLogic.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// An actuated traffic light logic based on time delay of approaching vehicles
/// C++ class `MSDelayBasedTrafficLightLogic` (microsim/traffic_lights/MSDelayBasedTrafficLightLogic.h)
pub struct MSDelayBasedTrafficLightLogic {
    /// C++ base class `MSSimpleTrafficLightLogic`
    pub base: MSSimpleTrafficLightLogic,
    /// A map from lanes to the corresponding lane detectors
    pub myLaneDetectors: BTreeMap<Ptr<MSLane>, Ptr<MSE2Collector>>,
    /// Range of the connected detector, which provides the information on approaching vehicles
    pub myDetectionRange: f64,
    /// If a vehicle's timeloss is below myTimeLossThreshold, this is counted as insignificant, since this may stem from dawdling, or driving only slightly slower than the maximal velocity on the lane.
    pub myTimeLossThreshold: f64,
    /// Whether the detectors shall be shown in the GUI
    pub myShowDetectors: bool,
    /// The output file for generated detectors
    pub myFile: String,
    /// The frequency for aggregating detector output
    pub myFreq: SUMOTime,
    /// Whether detector output separates by vType
    pub myVehicleTypes: String,
    /// Whether phases may be extended beyond maxDur in the absence of traffic
    pub myExtendMaxDur: bool,
}

impl std::ops::Deref for MSDelayBasedTrafficLightLogic {
    type Target = MSSimpleTrafficLightLogic;
    fn deref(&self) -> &MSSimpleTrafficLightLogic {
        &self.base
    }
}
impl std::ops::DerefMut for MSDelayBasedTrafficLightLogic {
    fn deref_mut(&mut self) -> &mut MSSimpleTrafficLightLogic {
        &mut self.base
    }
}

impl MSDelayBasedTrafficLightLogic {
    /// C++: `void MSDelayBasedTrafficLightLogic::MSDelayBasedTrafficLightLogic(MSTLLogicControl &, const std::string &, const std::string &, const SUMOTime, const MSSimpleTrafficLightLogic::Phases &, int, SUMOTime, const Parameterised::Map &, const std::string &)`
    /// Constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new(tlcontrol: &mut MSTLLogicControl, id: &str, programID: &str, offset: SUMOTime, phases: &MSTrafficLightLogic_Phases, step: i32, delay: SUMOTime, parameter: &Unmapped /* Parameterised::Map */, basePath: &str) -> Self {
        todo!("port MSDelayBasedTrafficLightLogic::MSDelayBasedTrafficLightLogic")
    }

    /// C++: `void MSDelayBasedTrafficLightLogic::init(NLDetectorBuilder &)` [virtual]
    /// Initializes the tls with information about incoming lanes
    // void
    // MSDelayBasedTrafficLightLogic::init(NLDetectorBuilder& nb) {
    //     MSTrafficLightLogic::init(nb);
    //     assert(myLanes.size() > 0);
    //     LaneVectorVector::const_iterator i2;
    //     LaneVector::const_iterator i;
    //     // build the E2 detectors
    //     for (i2 = myLanes.begin(); i2 != myLanes.end(); ++i2) {
    //         const LaneVector& lanes = *i2;
    //         for (i = lanes.begin(); i != lanes.end(); i++) {
    //             MSLane* lane = (*i);
    //             if (noVehicles(lane->getPermissions())) {
    //                 // do not build detectors on green verges or sidewalks
    //                 continue;
    //             }
    //             // Build the detectors and register them at the detector control
    //             if (myLaneDetectors.find(lane) == myLaneDetectors.end()) {
    //                 MSE2Collector* det = nullptr;
    //                 const std::string customID = getParameter(lane->getID());
    //                 if (customID != "") {
    //                     det = dynamic_cast<MSE2Collector*>(MSNet::getInstance()->getDetectorControl().getTypedDetectors(SUMO_TAG_LANE_AREA_DETECTOR).get(customID));
    //                     if (det == nullptr) {
    //                         WRITE_ERRORF(TL("Unknown laneAreaDetector '%' given as custom detector for delay_based tlLogic '%', program '%."), customID, getID(), getProgramID());
    //                         continue;
    //                     }
    //                     det->setVisible(myShowDetectors);
    //                 } else {
    //                     // check whether the lane (or unamibiguous lane sequence) is long enough and avoid overlapping detectors
    //                     double length = lane->getLength();
    //                     MSLane* firstLane = lane;
    //                     while (length < myDetectionRange && firstLane->getIncomingLanes().size() == 1
    //                             && firstLane->getLogicalPredecessorLane() != nullptr
    //                             && firstLane->getIncomingLanes().front().viaLink->getCorrespondingEntryLink()->getTLLogic() == nullptr) {
    //                         firstLane = firstLane->getLogicalPredecessorLane();
    //                         if (firstLane->getLinkCont().size() > 1) {
    //                             break;
    //                         }
    //                         length += firstLane->getLength();
    //                     }
    //                     length = MIN2(length, myDetectionRange);
    //
    //                     std::string id = "TLS" + myID + "_" + myProgramID + "_E2CollectorOn_" + lane->getID();
    //                     det = nb.createE2Detector(id, DU_TL_CONTROL, lane, INVALID_POSITION, lane->getLength(), length, TIME2STEPS(1.0), 5.0 / 3.6, 10.0, "", myVehicleTypes, "", (int)PersonMode::NONE, myShowDetectors);
    //                     MSNet::getInstance()->getDetectorControl().add(SUMO_TAG_LANE_AREA_DETECTOR, det, myFile, myFreq);
    //                 }
    //                 myLaneDetectors[lane] = det;
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn init(&mut self, nb: &mut NLDetectorBuilder) {
        todo!("port MSDelayBasedTrafficLightLogic::init")
    }

    /// C++: `void MSDelayBasedTrafficLightLogic::~MSDelayBasedTrafficLightLogic() noexcept` [virtual]
    /// Destructor
    // MSDelayBasedTrafficLightLogic::~MSDelayBasedTrafficLightLogic() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSDelayBasedTrafficLightLogic::~MSDelayBasedTrafficLightLogic")
    }

    /// C++: `SUMOTime MSDelayBasedTrafficLightLogic::trySwitch()` [virtual]
    /// Switches to the next phase, if possible
    // SUMOTime
    // MSDelayBasedTrafficLightLogic::trySwitch() {
    //     /* check if the actual phase should be prolonged */
    //     const MSPhaseDefinition& currentPhase = getCurrentPhaseDef();
    //     // time since last switch
    //     const SUMOTime actDuration = MSNet::getInstance()->getCurrentTimeStep() - currentPhase.myLastSwitch;
    //
    // #ifdef DEBUG_TIMELOSS_CONTROL
    //     std::cout << "last switch = " << currentPhase.myLastSwitch
    //               << "\nactDuration = " << actDuration
    //               << "\nmaxDuration = " << currentPhase.maxDuration
    //               << std::endl;
    // #endif
    //
    //     // flag whether to prolong or not
    //     if (currentPhase.isGreenPhase() && !MSGlobals::gUseMesoSim) {
    //         bool othersEmpty = true; // whether no vehicles are present on concurrent approaches
    //         SUMOTime proposedProlongation = proposeProlongation(actDuration, currentPhase.maxDuration, othersEmpty);
    //
    // #ifdef DEBUG_TIMELOSS_CONTROL
    //         std::cout << "othersEmpty = " << othersEmpty
    //                   << std::endl;
    // #endif
    //
    //         // assure minimal duration
    //         proposedProlongation = MAX3(SUMOTime(0), proposedProlongation, currentPhase.minDuration - actDuration);
    //         if (othersEmpty) {
    //             // prolong by one second if no vehicles on other approaches
    //             proposedProlongation = MAX2(proposedProlongation, TIME2STEPS(1.));
    //         } else {
    //             // vehicles are present on other approaches -> prolong no further than the max green time
    //             proposedProlongation = MIN2(proposedProlongation, MAX2(SUMOTime(0), currentPhase.maxDuration - actDuration));
    //         }
    //         if (!myExtendMaxDur) {
    //             proposedProlongation = MIN2(proposedProlongation, MAX2(SUMOTime(0), currentPhase.maxDuration - actDuration));
    //         }
    //
    // #ifdef DEBUG_TIMELOSS_CONTROL
    //         std::cout << "Proposed prolongation = " << proposedProlongation << std::endl;
    // #endif
    //
    //         if (proposedProlongation > 0) {
    //             // check again after the prolonged period (must be positive...)
    //             // XXX: Can it be harmful not to return a duration of integer seconds?
    //             return proposedProlongation;
    //         }
    //     }
    //     // Don't prolong... switch to the next phase
    //     const SUMOTime prevStart = myPhases[myStep]->myLastSwitch;
    //     myStep = (myStep + 1) % (int)myPhases.size();
    //     myPhases[myStep]->myLastSwitch = SIMSTEP;
    //     MSPhaseDefinition* newPhase = myPhases[myStep];
    //     //stores the time the phase started
    //     newPhase->myLastSwitch = MSNet::getInstance()->getCurrentTimeStep();
    //     // set the next event
    //     return MAX2(newPhase->minDuration, getEarliest(prevStart));
    // }
    #[allow(unused_variables)]
    pub fn try_switch(&mut self) -> SUMOTime {
        todo!("port MSDelayBasedTrafficLightLogic::trySwitch")
    }

    /// C++: `bool MSDelayBasedTrafficLightLogic::showDetectors() const` [const]
    /// @}
    // bool showDetectors() const {
    //         return myShowDetectors;
    //     }
    #[allow(unused_variables)]
    pub fn show_detectors(&self) -> bool {
        todo!("port MSDelayBasedTrafficLightLogic::showDetectors")
    }

    /// C++: `void MSDelayBasedTrafficLightLogic::setShowDetectors(bool)`
    // void
    // MSDelayBasedTrafficLightLogic::setShowDetectors(bool show) {
    //     myShowDetectors = show;
    //     for (auto& item : myLaneDetectors) {
    //         item.second->setVisible(myShowDetectors);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_show_detectors(&mut self, show: bool) {
        todo!("port MSDelayBasedTrafficLightLogic::setShowDetectors")
    }

    /// C++: `std::map<std::string, double> MSDelayBasedTrafficLightLogic::getDetectorStates() const` [const, virtual]
    /// retrieve all detectors used by this program
    // std::map<std::string, double>
    // MSDelayBasedTrafficLightLogic::getDetectorStates() const {
    //     std::map<std::string, double> result;
    //     for (auto item : myLaneDetectors) {
    //         result[item.second->getID()] = item.second->getCurrentVehicleNumber();
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_detector_states(&self) -> BTreeMap<String, f64> {
        todo!("port MSDelayBasedTrafficLightLogic::getDetectorStates")
    }

    /// C++: `double MSDelayBasedTrafficLightLogic::getDetectorState(const std::string) const` [const, virtual]
    /// retrieve a specific detector used by this program
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_detector_state(&self, laneID: String) -> f64 {
        todo!("port MSDelayBasedTrafficLightLogic::getDetectorState")
    }

    /// C++: `double MSDelayBasedTrafficLightLogic::getTLQueueLength(const std::string) const` [const, virtual]
    /// return the estimated queue length at the upcoming traffic light
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_tl_queue_length(&self, laneID: String) -> f64 {
        todo!("port MSDelayBasedTrafficLightLogic::getTLQueueLength")
    }

    /// C++: `SUMOTime MSDelayBasedTrafficLightLogic::checkForWaitingTime()` [protected]
    /// Checks for approaching vehicles on the lanes associated with green signals and returns the minimal time to keep the green phase going. This is zero if no vehicle on the lane has gathered any waiting time or if the green time is exhausted (maximal green time has passed since switch).
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn check_for_waiting_time(&mut self) -> SUMOTime {
        todo!("port MSDelayBasedTrafficLightLogic::checkForWaitingTime")
    }

    /// C++: `SUMOTime MSDelayBasedTrafficLightLogic::proposeProlongation(const SUMOTime, const SUMOTime, bool &)` [protected]
    /// The returned, proposed prolongation for the green phase is oriented on the largest estimated passing time among the vehicles with waiting time.
    // SUMOTime
    // MSDelayBasedTrafficLightLogic::proposeProlongation(const SUMOTime actDuration, const SUMOTime maxDuration, bool& othersEmpty) {
    // #ifdef DEBUG_TIMELOSS_CONTROL
    //     std::cout << "\n" << SIMTIME << " MSDelayBasedTrafficLightLogic::proposeProlongation() for TLS '" << this->getID() << "' (current phase = " << myStep << ")" << std::endl;
    // #endif
    //     SUMOTime prolongation = 0;
    //     const std::string& state = getCurrentPhaseDef().getState();
    //     // iterate over green lanes, eventually increase the proposed prolongationTime to the estimated passing time for each lane.
    //     for (int i = 0; i < (int) state.size(); i++)  {
    //         // this lane index corresponds to a non-green time
    //         bool igreen = state[i] == LINKSTATE_TL_GREEN_MAJOR || state[i] == LINKSTATE_TL_GREEN_MINOR;
    //         for (const MSLane* const lane : getLanesAt(i)) {
    //             std::map<const MSLane*, MSE2Collector*>::iterator it = myLaneDetectors.find(lane);
    //             if (it == myLaneDetectors.end()) {
    // #ifdef DEBUG_TIMELOSS_CONTROL
    //                 // no detector for this lane!? maybe noVehicles allowed
    //                 std::cout << "no detector on lane '" << lane->getID() << std::endl;
    // #endif
    //                 continue;
    //             }
    //             const std::vector<MSE2Collector::VehicleInfo*> vehInfos = it->second->getCurrentVehicles();
    // #ifdef DEBUG_TIMELOSS_CONTROL
    //             int nrVehs = 0; // count vehicles on detector
    // #endif
    //             if (igreen) {
    //                 // green phase
    //                 for (const MSE2Collector::VehicleInfo* const iv : vehInfos) {
    //                     if (iv->accumulatedTimeLoss > myTimeLossThreshold && iv->distToDetectorEnd > 0) {
    //                         const SUMOTime estimatedTimeToJunction = TIME2STEPS((iv->distToDetectorEnd) / lane->getSpeedLimit());
    //                         if (actDuration + estimatedTimeToJunction  <= maxDuration && getLatest() > 0) {
    //                             // only prolong if vehicle has a chance to pass until max duration is reached
    //                             prolongation = MAX2(prolongation, estimatedTimeToJunction);
    //                         }
    // #ifdef DEBUG_TIMELOSS_CONTROL
    //                         nrVehs++;
    // #endif
    //
    // #ifdef DEBUG_TIMELOSS_CONTROL
    //                         std::cout << "vehicle '" << iv->id << "' with accumulated timeloss: " << iv->accumulatedTimeLoss
    //                                   << "\nestimated passing time: " << estimatedTimeToJunction << std::endl;
    //                     } else {
    //                         std::string reason = iv->accumulatedTimeLoss <= myTimeLossThreshold ? " (time loss below threshold)" : " (front already left detector)";
    //                         std::cout << "disregarded: (vehicle '" << iv->id << "' with accumulated timeloss " << iv->accumulatedTimeLoss << ")" << reason << std::endl;
    // #endif
    //                     }
    //                 }
    //             } else {
    //                 // non-green phase
    //                 if (vehInfos.size() > 0) {
    //                     // here is a car on a non-green approach
    //                     othersEmpty = false;
    //                     if (actDuration >= getCurrentPhaseDef().maxDuration) {
    // #ifdef DEBUG_TIMELOSS_CONTROL
    //                         std::cout << "Actual duration exceeds maxDuration and a vehicle is on concurrent approach: " << nrVehs << std::endl;
    // #endif
    //                         // don't prolong
    //                         return 0;
    //                     }
    //                     break;
    //                 }
    // #ifdef DEBUG_TIMELOSS_CONTROL
    //                 std::cout << "Number of current vehicles on detector: " << nrVehs << std::endl;
    // #endif
    //             }
    //         }
    //     }
    // #ifdef DEBUG_TIMELOSS_CONTROL
    //     std::cout << "Proposed prolongation (maximal estimated passing time): " << prolongation << std::endl; // debug
    // #endif
    //     return prolongation;
    // }
    #[allow(unused_variables)]
    pub fn propose_prolongation(&mut self, actDuration: SUMOTime, maxDuration: SUMOTime, othersEmpty: &mut bool) -> SUMOTime {
        todo!("port MSDelayBasedTrafficLightLogic::proposeProlongation")
    }

}

