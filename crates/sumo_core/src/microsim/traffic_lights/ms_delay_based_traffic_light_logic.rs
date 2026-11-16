//! Port of microsim/traffic_lights/MSDelayBasedTrafficLightLogic.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ `dynamic_cast<MSE2Collector*>(MSDetectorFileOutput*)`: MSDetectorFileOutput is the MSE2Collector base2 field,
/// not its first base, so step back by the field offset.
// shortcut: no runtime type check; relies on the lane-area detector map holding only MSE2Collector, upgrade with a type flag check
fn e2_from_detector_file_output(det: Ptr<MSDetectorFileOutput>) -> Ptr<MSE2Collector> {
    if det.is_null() {
        return Ptr::null();
    }
    Ptr::from_raw((det.as_raw() as *mut u8).wrapping_sub(std::mem::offset_of!(MSE2Collector, base2)) as *mut MSE2Collector)
}

/// An actuated traffic light logic based on time delay of approaching vehicles
/// C++ class `MSDelayBasedTrafficLightLogic` (microsim/traffic_lights/MSDelayBasedTrafficLightLogic.h)
#[repr(C)]
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
    pub fn new(tlcontrol: &mut MSTLLogicControl, id: &str, programID: &str, offset: SUMOTime, phases: &MSTrafficLightLogic_Phases, step: i32, delay: SUMOTime, parameter: &ParameterisedMap, basePath: &str) -> Self {
        let base = MSSimpleTrafficLightLogic::new(tlcontrol, id, programID, offset, utils::xml::sumo_xml_definitions::TrafficLightType::DELAYBASED, phases, step, delay, parameter);
        let myShowDetectors = utils::common::string_utils::to_bool(&base.get_parameter("show-detectors", "false".to_string())).unwrap();
        let myDetectionRange = utils::common::string_utils::to_double(&base.get_parameter("detectorRange", utils::common::to_string::to_string(&utils::options::options_cont::get_float("tls.delay_based.detector-range")))).unwrap();
        let myTimeLossThreshold = utils::common::string_utils::to_double(&base.get_parameter("minTimeloss", "1.0".to_string())).unwrap();
        let myFile = utils::common::file_helpers::check_for_relativity(&base.get_parameter("file", "NUL".to_string()), basePath);
        let myFreq = utils::common::time::time2steps(utils::common::string_utils::to_double(&base.get_parameter("freq", "300".to_string())).unwrap());
        let myVehicleTypes = base.get_parameter("vTypes", String::new());
        let myExtendMaxDur = utils::common::string_utils::to_bool(&base.get_parameter("extendMaxDur", "false".to_string())).unwrap();
        Self {
            base,
            myLaneDetectors: BTreeMap::new(),
            myDetectionRange,
            myTimeLossThreshold,
            myShowDetectors,
            myFile,
            myFreq,
            myVehicleTypes,
            myExtendMaxDur,
        }
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
        self.base.base.init(nb);
        debug_assert!(self.base.base.myLanes.len() > 0);
        let myId = self.get_id().to_string();
        let myProgramID = self.get_program_id().to_string();
        // build the E2 detectors
        for lanes in self.base.base.myLanes.iter() {
            for &lane in lanes.iter() {
                if utils::common::sumo_vehicle_class::no_vehicles(lane.get_permissions()) {
                    // do not build detectors on green verges or sidewalks
                    continue;
                }
                // Build the detectors and register them at the detector control
                if !self.myLaneDetectors.contains_key(&lane) {
                    let mut det: Ptr<MSE2Collector>;
                    let customID = self.base.get_parameter(lane.get_id(), String::new());
                    if customID != "" {
                        let found = MSNet::get_instance().m().get_detector_control().get_typed_detectors(utils::xml::sumo_xml_definitions::SUMO_TAG_LANE_AREA_DETECTOR).get(&customID).copied().unwrap_or(Ptr::null());
                        det = e2_from_detector_file_output(found);
                        if det.is_null() {
                            utils::write_errorf!("Unknown laneAreaDetector '%' given as custom detector for delay_based tlLogic '%', program '%.", customID, myId, myProgramID);
                            continue;
                        }
                        det.m().set_visible(self.myShowDetectors);
                    } else {
                        // check whether the lane (or unamibiguous lane sequence) is long enough and avoid overlapping detectors
                        let mut length = lane.get_length();
                        let mut firstLane = lane;
                        while length < self.myDetectionRange
                            && firstLane.get_incoming_lanes().len() == 1
                            && !firstLane.get_logical_predecessor_lane().is_null()
                            && firstLane.get_incoming_lanes()[0].viaLink.get_corresponding_entry_link().get_tl_logic().is_null()
                        {
                            firstLane = firstLane.get_logical_predecessor_lane();
                            if firstLane.get_link_cont().len() > 1 {
                                break;
                            }
                            length += firstLane.get_length();
                        }
                        // MIN2(length, myDetectionRange)
                        length = if length < self.myDetectionRange { length } else { self.myDetectionRange };

                        let id = format!("TLS{}_{}_E2CollectorOn_{}", myId, myProgramID, lane.get_id());
                        det = nb.create_e2_detector(&id, DetectorUsage::DU_TL_CONTROL, lane, f64::MAX, lane.get_length(), length, utils::common::time::time2steps(1.0), 5.0 / 3.6, 10.0, String::new(), &self.myVehicleTypes, "", PersonMode::NONE as i32, self.myShowDetectors);
                        MSNet::get_instance().m().get_detector_control().add(utils::xml::sumo_xml_definitions::SUMO_TAG_LANE_AREA_DETECTOR, Ptr::from_mut(&mut det.m().base2), &self.myFile, self.myFreq, 0).unwrap();
                    }
                    self.myLaneDetectors.insert(lane, det);
                }
            }
        }
    }

    /// C++: `void MSDelayBasedTrafficLightLogic::~MSDelayBasedTrafficLightLogic() noexcept` [virtual]
    /// Destructor
    // MSDelayBasedTrafficLightLogic::~MSDelayBasedTrafficLightLogic() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ body is empty
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
        // check if the actual phase should be prolonged
        let currentPhase: Ptr<MSPhaseDefinition> = Ptr::new(self.get_current_phase_def());
        let actDuration = MSNet::get_instance().get_current_time_step() - currentPhase.myLastSwitch;
        let useMesoSim = crate::microsim::ms_globals::use_meso_sim();
        // flag whether to prolong or not
        if currentPhase.is_green_phase() && !useMesoSim {
            let mut othersEmpty = true; // whether no vehicles are present on concurrent approaches
            let mut proposedProlongation = self.propose_prolongation(actDuration, currentPhase.maxDuration, &mut othersEmpty);
            // assure minimal duration
            proposedProlongation = std::cmp::max(std::cmp::max(0, proposedProlongation), currentPhase.minDuration - actDuration);
            if othersEmpty {
                // prolong by one second if no vehicles on other approaches
                proposedProlongation = std::cmp::max(proposedProlongation, utils::common::time::time2steps(1.0));
            } else {
                // vehicles are present on other approaches -> prolong no further than the max green time
                proposedProlongation = std::cmp::min(proposedProlongation, std::cmp::max(0, currentPhase.maxDuration - actDuration));
            }
            if !self.myExtendMaxDur {
                proposedProlongation = std::cmp::min(proposedProlongation, std::cmp::max(0, currentPhase.maxDuration - actDuration));
            }
            if proposedProlongation > 0 {
                // check again after the prolonged period (must be positive...)
                return proposedProlongation;
            }
        }
        // Don't prolong... switch to the next phase
        let prevStart = self.myPhases[self.myStep as usize].myLastSwitch;
        self.myStep = (self.myStep + 1) % (self.myPhases.len() as i32);
        let now = MSNet::get_instance().get_current_time_step();
        self.myPhases[self.myStep as usize].m().myLastSwitch = now;
        let newPhase = self.myPhases[self.myStep as usize];
        //stores the time the phase started
        newPhase.m().myLastSwitch = MSNet::get_instance().get_current_time_step();
        // set the next event
        std::cmp::max(newPhase.minDuration, self.get_earliest(prevStart))
    }

    /// C++: `bool MSDelayBasedTrafficLightLogic::showDetectors() const` [const]
    /// @}
    // bool showDetectors() const {
    //         return myShowDetectors;
    //     }
    #[allow(unused_variables)]
    pub fn show_detectors(&self) -> bool {
        self.myShowDetectors
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
        self.myShowDetectors = show;
        for item in self.myLaneDetectors.values() {
            item.m().set_visible(self.myShowDetectors);
        }
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
        let mut result: BTreeMap<String, f64> = BTreeMap::new();
        for item in self.myLaneDetectors.values() {
            result.insert(item.base2.base.get_id().to_string(), item.get_current_vehicle_number() as f64);
        }
        result
    }

    /// C++: `double MSDelayBasedTrafficLightLogic::getDetectorState(const std::string) const` [const, virtual]
    /// retrieve a specific detector used by this program
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_detector_state(&self, laneID: String) -> f64 {
        let mut result: f64 = 0.0;
        for (lane, det) in self.myLaneDetectors.iter() {
            if lane.get_id() == laneID.as_str() {
                result = det.get_current_vehicle_number() as f64;
                break;
            }
        }
        result
    }

    /// C++: `double MSDelayBasedTrafficLightLogic::getTLQueueLength(const std::string) const` [const, virtual]
    /// return the estimated queue length at the upcoming traffic light
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_tl_queue_length(&self, laneID: String) -> f64 {
        let mut result: f64 = 0.0;
        for (lane, det) in self.myLaneDetectors.iter() {
            if lane.get_id() == laneID.as_str() {
                result = det.get_estimate_queue_length();
                break;
            }
        }
        result
    }

    /// C++: `SUMOTime MSDelayBasedTrafficLightLogic::checkForWaitingTime()` [protected]
    /// Checks for approaching vehicles on the lanes associated with green signals and returns the minimal time to keep the green phase going. This is zero if no vehicle on the lane has gathered any waiting time or if the green time is exhausted (maximal green time has passed since switch).
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn check_for_waiting_time(&mut self) -> SUMOTime {
        // declared in MSDelayBasedTrafficLightLogic.h but never defined in C++, and trySwitch never calls it
        unreachable!("MSDelayBasedTrafficLightLogic::checkForWaitingTime has no C++ definition")
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
        let mut prolongation: SUMOTime = 0;
        let state = self.get_current_phase_def().get_state().to_string();
        let stateBytes = state.as_bytes();
        // iterate over green lanes, eventually increase the proposed prolongationTime to the estimated passing time for each lane.
        for i in 0..state.len() {
            // this lane index corresponds to a non-green time
            let c = stateBytes[i] as i32;
            let igreen = c == utils::xml::sumo_xml_definitions::LINKSTATE_TL_GREEN_MAJOR || c == utils::xml::sumo_xml_definitions::LINKSTATE_TL_GREEN_MINOR;
            for &lane in self.get_lanes_at(i as i32).iter() {
                // no detector for this lane (e.g. noVehicles allowed): skip
                let it = match self.myLaneDetectors.get(&lane) {
                    Some(d) => *d,
                    None => continue,
                };
                let vehInfos = it.get_current_vehicles();
                if igreen {
                    // green phase
                    for &iv in vehInfos.iter() {
                        if iv.accumulatedTimeLoss > self.myTimeLossThreshold && iv.distToDetectorEnd > 0.0 {
                            let estimatedTimeToJunction = utils::common::time::time2steps(iv.distToDetectorEnd / lane.get_speed_limit());
                            if actDuration + estimatedTimeToJunction <= maxDuration && self.get_latest() > 0 {
                                // only prolong if vehicle has a chance to pass until max duration is reached
                                prolongation = std::cmp::max(prolongation, estimatedTimeToJunction);
                            }
                        }
                    }
                } else {
                    // non-green phase
                    if vehInfos.len() > 0 {
                        // here is a car on a non-green approach
                        *othersEmpty = false;
                        if actDuration >= self.get_current_phase_def().maxDuration {
                            // don't prolong
                            return 0;
                        }
                        break;
                    }
                }
            }
        }
        prolongation
    }

}

