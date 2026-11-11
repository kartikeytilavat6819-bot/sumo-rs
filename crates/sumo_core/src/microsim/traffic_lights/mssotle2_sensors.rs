//! Port of microsim/traffic_lights/MSSOTLE2Sensors.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `MSSOTLE2Sensors` (microsim/traffic_lights/MSSOTLE2Sensors.h)
pub struct MSSOTLE2Sensors {
    /// C++ base class `MSSOTLSensors`
    pub base: MSSOTLSensors,
    pub m_sensorMap: MSLaneID_MSE2CollectorMap,
    pub m_maxSpeedMap: MSLaneID_MaxSpeedMap,
    pub speedThresholdParam: f64,
    pub m_continueSensorOnLanes: BTreeMap<String, Vec<String>>,
    pub m_typeWeightMap: BTreeMap<String, i32>,
}

impl std::ops::Deref for MSSOTLE2Sensors {
    type Target = MSSOTLSensors;
    fn deref(&self) -> &MSSOTLSensors {
        &self.base
    }
}
impl std::ops::DerefMut for MSSOTLE2Sensors {
    fn deref_mut(&mut self) -> &mut MSSOTLSensors {
        &mut self.base
    }
}

impl MSSOTLE2Sensors {
    /// C++: `void MSSOTLE2Sensors::buildSensorForLane(MSLane *, NLDetectorBuilder &)` [virtual, protected]
    // void MSSOTLE2Sensors::buildSensorForLane(MSLane* lane, NLDetectorBuilder& nb) {
    //     buildSensorForLane(lane, nb, INPUT_SENSOR_LENGTH);
    // }
    #[allow(unused_variables)]
    pub fn build_sensor_for_lane(&mut self, lane: Ptr<MSLane>, nb: &mut NLDetectorBuilder) {
        todo!("port MSSOTLE2Sensors::buildSensorForLane")
    }

    /// C++: `void MSSOTLE2Sensors::buildSensorForLane(MSLane *, NLDetectorBuilder &, double)` [protected]
    // void MSSOTLE2Sensors::buildSensorForLane(MSLane* lane, NLDetectorBuilder& nb, double sensorLength) {
    //     const MSEdge* edge = &lane->getEdge();
    //     if (edge->isInternal() || edge->isWalkingArea() || edge->isCrossing()) {
    //         return;
    //     }
    //     double sensorPos;
    //     double lensorLength;
    //     MSE2Collector* newSensor = nullptr;
    //     //Check not to have more than a sensor for lane
    //     if (m_sensorMap.find(lane->getID()) == m_sensorMap.end()) {
    //
    //         //Check and set zero if the lane is not long enough for the specified sensor start
    //         sensorPos = SENSOR_START <= lane->getLength() ? SENSOR_START : 0;
    //         //Check and trim if the lane is not long enough for the specified sensor length
    //         lensorLength = sensorLength <= (lane->getLength() - sensorPos) ? sensorLength : (lane->getLength() - sensorPos);
    //         //TODO check this lengths
    // //        DBG(
    // //            std::ostringstream phero_str;
    // //            phero_str << " lane " << lane->getID() << " sensorPos= " << sensorPos
    // //            << " ,SENSOR_START  " << SENSOR_START << "; lane->getLength = "
    // //            << lane->getLength() << " ,lensorLength= " << lensorLength
    // //            << " ,SENSOR_LENGTH= " << INPUT_SENSOR_LENGTH;
    // //            WRITE_MESSAGE(
    // //                "MSSOTLE2Sensors::buildSensorForLane::" + phero_str.str());
    // //        )
    //
    //         //Create sensor for lane and insert it into the map<MSLane*, MSE2Collector*>
    //         newSensor = nb.createE2Detector(
    //                         "SOTL_E2_lane:" + lane->getID() + "_tl:" + tlLogicID,
    //                         DU_TL_CONTROL, lane,
    //                         (lane->getLength() - sensorPos - lensorLength), INVALID_POSITION, lensorLength,
    //                         HALTING_TIME_THRS, HALTING_SPEED_THRS, DIST_THRS, "", "", "", (int)PersonMode::NONE, true);
    //
    //         MSNet::getInstance()->getDetectorControl().add(SUMO_TAG_LANE_AREA_DETECTOR, newSensor);
    //
    //         m_sensorMap.insert(MSLaneID_MSE2Collector(lane->getID(), newSensor));
    //         m_maxSpeedMap.insert(MSLaneID_MaxSpeed(lane->getID(), lane->getSpeedLimit()));
    //         //Avoid creating really small sensors. Only add a further sensors if the one created is less than 90% the requested length
    //         if (lensorLength < sensorLength * 0.9) {
    //             std::ostringstream oss;
    //             oss << "Sensor on lane " << lane->getID() << " is long " << lensorLength << ", while it should be " << sensorLength << ". Continuing it on the other lanes if possible";
    //             WRITE_MESSAGE(oss.str())
    //             for (const MSLane::IncomingLaneInfo& it : lane->getIncomingLanes()) {
    //                 const MSEdge& inEdge = it.lane->getEdge();
    //                 if (!inEdge.isInternal() && !inEdge.isWalkingArea() && !inEdge.isCrossing()) {
    //                     buildContinueSensior(lane, nb, sensorLength, it.lane, lensorLength);
    //                 }
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_sensor_for_lane_2(&mut self, lane: Ptr<MSLane>, nb: &mut NLDetectorBuilder, sensorLength: f64) {
        todo!("port MSSOTLE2Sensors::buildSensorForLane")
    }

    /// C++: `void MSSOTLE2Sensors::buildContinueSensior(MSLane *, NLDetectorBuilder &, double, MSLane *, double)` [protected]
    // void MSSOTLE2Sensors::buildContinueSensior(MSLane* lane, NLDetectorBuilder& nb, double sensorLength, MSLane* continueOnLane, double usedLength) {
    //     double availableLength = sensorLength - usedLength;
    //     if (m_sensorMap.find(continueOnLane->getID()) == m_sensorMap.end()) {
    //         double length = availableLength <= continueOnLane->getLength() ? availableLength : continueOnLane->getLength();
    //         MSE2Collector* newSensor = nb.createE2Detector(
    //                                        "SOTL_E2_lane:" + continueOnLane->getID() + "_tl:" + tlLogicID,
    //                                        DU_TL_CONTROL, continueOnLane,
    //                                        (continueOnLane->getLength() - length), INVALID_POSITION, length,
    //                                        HALTING_TIME_THRS, HALTING_SPEED_THRS, DIST_THRS, "", "", "", (int)PersonMode::NONE, true);
    //         MSNet::getInstance()->getDetectorControl().add(SUMO_TAG_LANE_AREA_DETECTOR, newSensor);
    //         m_sensorMap.insert(MSLaneID_MSE2Collector(continueOnLane->getID(), newSensor));
    //         m_continueSensorOnLanes[lane->getID()].push_back(continueOnLane->getID());
    //         std::ostringstream oss;
    //         oss << "Continue sensor on lane " << continueOnLane->getID() << ". Current length " << (length + usedLength);
    //         WRITE_MESSAGE(oss.str())
    //         //Continue other line if needed.
    //         if (length + usedLength < sensorLength * 0.9) {
    //             for (std::vector<MSLane::IncomingLaneInfo>::const_iterator it = continueOnLane->getIncomingLanes().begin(); it != continueOnLane->getIncomingLanes().end(); ++it) {
    //                 const MSEdge* edge = &it->lane->getEdge();
    //                 if (!edge->isInternal() && !edge->isWalkingArea() && !edge->isCrossing()) {
    //                     buildContinueSensior(lane, nb, sensorLength, it->lane, length + usedLength);
    //                 }
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_continue_sensior(&mut self, lane: Ptr<MSLane>, nb: &mut NLDetectorBuilder, sensorLength: f64, continueOnLane: Ptr<MSLane>, usedLength: f64) {
        todo!("port MSSOTLE2Sensors::buildContinueSensior")
    }

    /// C++: `void MSSOTLE2Sensors::buildSensorForOutLane(MSLane *, NLDetectorBuilder &)` [virtual, protected]
    // void MSSOTLE2Sensors::buildSensorForOutLane(MSLane* lane,
    //         NLDetectorBuilder& nb) {
    //     buildSensorForOutLane(lane, nb, OUTPUT_SENSOR_LENGTH);
    // }
    #[allow(unused_variables)]
    pub fn build_sensor_for_out_lane(&mut self, lane: Ptr<MSLane>, nb: &mut NLDetectorBuilder) {
        todo!("port MSSOTLE2Sensors::buildSensorForOutLane")
    }

    /// C++: `void MSSOTLE2Sensors::buildSensorForOutLane(MSLane *, NLDetectorBuilder &, double)` [protected]
    // void MSSOTLE2Sensors::buildSensorForOutLane(MSLane* lane,
    //         NLDetectorBuilder& nb, double sensorLength) {
    //     double sensorPos;
    //     double lensorLength;
    //     MSE2Collector* newSensor = nullptr;
    //     //Check not to have more than a sensor for lane
    //     if (m_sensorMap.find(lane->getID()) == m_sensorMap.end()) {
    //
    //         //Original:
    //         //double sensorLength = OUTPUT_SENSOR_LENGTH;
    //         //Check and set zero if the lane is not long enough for the specified sensor start
    //         sensorPos = (lane->getLength() - sensorLength)
    //                     - (SENSOR_START <= lane->getLength() ? SENSOR_START : 0);
    //
    //         //Check and trim if the lane is not long enough for the specified sensor length
    //         lensorLength =
    //             sensorLength <= (lane->getLength() - sensorPos) ?
    //             sensorLength : (lane->getLength() - sensorPos);
    //
    //         //TODO check this lengths
    // //        DBG(
    // //            std::ostringstream phero_str;
    // //            phero_str << " lane " << lane->getID() << " sensorPos= " << sensorPos
    // //            << " ,SENSOR_START  " << SENSOR_START << "; lane->getLength = "
    // //            << lane->getLength() << " ,lensorLength= " << lensorLength
    // //            << " ,SENSOR_LENGTH= " << INPUT_SENSOR_LENGTH;
    // //            WRITE_MESSAGE(
    // //                "MSSOTLE2Sensors::buildSensorForLane::" + phero_str.str());
    // //        )
    //
    //         //Create sensor for lane and insert it into the map<MSLane*, MSE2Collector*>
    //         newSensor = nb.createE2Detector(
    //                         "SOTL_E2_lane:" + lane->getID() + "_tl:" + tlLogicID,
    //                         DU_TL_CONTROL, lane,
    //                         (lane->getLength() - sensorPos - lensorLength), INVALID_POSITION, lensorLength,
    //                         HALTING_TIME_THRS, HALTING_SPEED_THRS, DIST_THRS, "", "", "", (int)PersonMode::NONE, true);
    //
    //         MSNet::getInstance()->getDetectorControl().add(
    //             SUMO_TAG_LANE_AREA_DETECTOR, newSensor);
    //
    //         m_sensorMap.insert(MSLaneID_MSE2Collector(lane->getID(), newSensor));
    //         m_maxSpeedMap.insert(MSLaneID_MaxSpeed(lane->getID(), lane->getSpeedLimit()));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_sensor_for_out_lane_2(&mut self, lane: Ptr<MSLane>, nb: &mut NLDetectorBuilder, sensorLength: f64) {
        todo!("port MSSOTLE2Sensors::buildSensorForOutLane")
    }

    /// C++: `void MSSOTLE2Sensors::buildCountSensorForLane(MSLane *, NLDetectorBuilder &)` [protected]
    // void MSSOTLE2Sensors::buildCountSensorForLane(MSLane* lane, NLDetectorBuilder& nb) {
    //     double sensorPos;
    //     double lensorLength;
    //     MSE2Collector* newSensor = nullptr;
    //     //Check not to have more than a sensor for lane
    //     if (m_sensorMap.find(lane->getID()) == m_sensorMap.end()) {
    //
    //         //Check and set zero if the lane is not long enough for the specified sensor start
    //         sensorPos = COUNT_SENSOR_START <= lane->getLength() ? COUNT_SENSOR_START : 0;
    //
    //         //Original:
    //         double sensorLength = INPUT_COUNT_SENSOR_LENGTH;
    //
    //         //Check and trim if the lane is not long enough for the specified sensor length
    //         lensorLength = sensorLength <= (lane->getLength() - sensorPos) ? sensorLength : (lane->getLength() - sensorPos);
    //
    //         //TODO check this lengths
    // //        DBG(
    // //            std::ostringstream phero_str;
    // //            phero_str << " lane " << lane->getID() << " sensorPos= " << sensorPos
    // //            << " ,SENSOR_START  " << SENSOR_START << "; lane->getLength = "
    // //            << lane->getLength() << " ,lensorLength= " << lensorLength
    // //            << " ,SENSOR_LENGTH= " << INPUT_SENSOR_LENGTH;
    // //            WRITE_MESSAGE(
    // //                "MSSOTLE2Sensors::buildSensorForLane::" + phero_str.str());
    // //        )
    //
    //         //Create sensor for lane and insert it into the map<MSLane*, MSE2Collector*>
    //         newSensor = nb.createE2Detector(
    //                         "SOTL_E2_lane:" + lane->getID() + "_tl:" + tlLogicID,
    //                         DU_TL_CONTROL, lane,
    //                         (lane->getLength() - sensorPos - lensorLength), INVALID_POSITION, lensorLength,
    //                         HALTING_TIME_THRS, HALTING_SPEED_THRS, DIST_THRS, "", "", "", (int)PersonMode::NONE, true);
    //
    //         MSNet::getInstance()->getDetectorControl().add(
    //             SUMO_TAG_LANE_AREA_DETECTOR, newSensor);
    //
    //         m_sensorMap.insert(MSLaneID_MSE2Collector(lane->getID(), newSensor));
    //         m_maxSpeedMap.insert(MSLaneID_MaxSpeed(lane->getID(), lane->getSpeedLimit()));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_count_sensor_for_lane(&mut self, lane: Ptr<MSLane>, nb: &mut NLDetectorBuilder) {
        todo!("port MSSOTLE2Sensors::buildCountSensorForLane")
    }

    /// C++: `void MSSOTLE2Sensors::buildCountSensorForOutLane(MSLane *, NLDetectorBuilder &)` [protected]
    // void MSSOTLE2Sensors::buildCountSensorForOutLane(MSLane* lane, NLDetectorBuilder& nb) {
    //     double sensorPos;
    //     double lensorLength;
    //     MSE2Collector* newSensor = nullptr;
    //     //Check not to have more than a sensor for lane
    //     if (m_sensorMap.find(lane->getID()) == m_sensorMap.end()) {
    //
    //         //Original:
    //         double sensorLength = OUTPUT_COUNT_SENSOR_LENGTH;
    //         //Check and set zero if the lane is not long enough for the specified sensor start
    //         sensorPos = (lane->getLength() - sensorLength)
    //                     - (SENSOR_START <= lane->getLength() ? SENSOR_START : 0);
    //
    //         //Check and trim if the lane is not long enough for the specified sensor length
    //         lensorLength =
    //             sensorLength <= (lane->getLength() - sensorPos) ?
    //             sensorLength : (lane->getLength() - sensorPos);
    //
    //         //TODO check this lengths
    // //        DBG(
    // //            std::ostringstream phero_str;
    // //            phero_str << " lane " << lane->getID() << " sensorPos= " << sensorPos
    // //            << " ,SENSOR_START  " << SENSOR_START << "; lane->getLength = "
    // //            << lane->getLength() << " ,lensorLength= " << lensorLength
    // //            << " ,SENSOR_LENGTH= " << INPUT_SENSOR_LENGTH;
    // //            WRITE_MESSAGE(
    // //                "MSSOTLE2Sensors::buildSensorForLane::" + phero_str.str());
    // //        )
    //
    //         //Create sensor for lane and insert it into the map<MSLane*, MSE2Collector*>
    //         newSensor = nb.createE2Detector(
    //                         "SOTL_E2_lane:" + lane->getID() + "_tl:" + tlLogicID,
    //                         DU_TL_CONTROL, lane,
    //                         (lane->getLength() - sensorPos - lensorLength), INVALID_POSITION, lensorLength,
    //                         HALTING_TIME_THRS, HALTING_SPEED_THRS, DIST_THRS, "", "", "", (int)PersonMode::NONE, true);
    //
    //         MSNet::getInstance()->getDetectorControl().add(
    //             SUMO_TAG_LANE_AREA_DETECTOR, newSensor);
    //
    //         m_sensorMap.insert(MSLaneID_MSE2Collector(lane->getID(), newSensor));
    //         m_maxSpeedMap.insert(MSLaneID_MaxSpeed(lane->getID(), lane->getSpeedLimit()));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_count_sensor_for_out_lane(&mut self, lane: Ptr<MSLane>, nb: &mut NLDetectorBuilder) {
        todo!("port MSSOTLE2Sensors::buildCountSensorForOutLane")
    }

    /// C++: `void MSSOTLE2Sensors::MSSOTLE2Sensors(std::string, const MSTrafficLightLogic::Phases *)`
    // MSSOTLE2Sensors::MSSOTLE2Sensors(std::string tlLogicID,
    //                                  const MSTrafficLightLogic::Phases* phases) :
    //     MSSOTLSensors(tlLogicID, phases) {
    // }
    #[allow(unused_variables)]
    pub fn new(tlLogicID: String, phases: Ptr<MSTrafficLightLogic_Phases>) -> Self {
        todo!("port MSSOTLE2Sensors::MSSOTLE2Sensors")
    }

    /// C++: `void MSSOTLE2Sensors::~MSSOTLE2Sensors() noexcept` [virtual]
    // MSSOTLE2Sensors::~MSSOTLE2Sensors(void) {
    //     //Deleting sensors takes place in the detector control
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSSOTLE2Sensors::~MSSOTLE2Sensors")
    }

    /// C++: `void MSSOTLE2Sensors::buildSensors(MSTrafficLightLogic::LaneVectorVector, NLDetectorBuilder &)` [virtual]
    // void MSSOTLE2Sensors::buildSensors(
    //     MSTrafficLightLogic::LaneVectorVector controlledLanes,
    //     NLDetectorBuilder& nb) {
    //     buildSensors(controlledLanes, nb, INPUT_SENSOR_LENGTH);
    // }
    #[allow(unused_variables)]
    pub fn build_sensors(&mut self, controlledLanes: MSTrafficLightLogic_LaneVectorVector, nb: &mut NLDetectorBuilder) {
        todo!("port MSSOTLE2Sensors::buildSensors")
    }

    /// C++: `void MSSOTLE2Sensors::buildSensors(MSTrafficLightLogic::LaneVectorVector, NLDetectorBuilder &, double)`
    // void MSSOTLE2Sensors::buildSensors(
    //     MSTrafficLightLogic::LaneVectorVector controlledLanes,
    //     NLDetectorBuilder& nb, double sensorLength) {
    //     //for each lane build an appropriate sensor on it
    //     MSLane* currentLane = nullptr;
    //
    //     //input and output lanes
    //     for (MSTrafficLightLogic::LaneVectorVector::const_iterator laneVector =
    //                 controlledLanes.begin(); laneVector != controlledLanes.end();
    //             laneVector++) {
    //         for (MSTrafficLightLogic::LaneVector::const_iterator lane =
    //                     laneVector->begin(); lane != laneVector->end(); lane++) {
    //             currentLane = (*lane);
    //             buildSensorForLane(currentLane, nb, sensorLength);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_sensors_2(&mut self, controlledLanes: MSTrafficLightLogic_LaneVectorVector, nb: &mut NLDetectorBuilder, sensorLength: f64) {
        todo!("port MSSOTLE2Sensors::buildSensors")
    }

    /// C++: `void MSSOTLE2Sensors::buildOutSensors(MSTrafficLightLogic::LaneVectorVector, NLDetectorBuilder &)` [virtual]
    // void MSSOTLE2Sensors::buildOutSensors(
    //     MSTrafficLightLogic::LaneVectorVector controlledLanes,
    //     NLDetectorBuilder& nb) {
    //     buildOutSensors(controlledLanes, nb, OUTPUT_SENSOR_LENGTH);
    // }
    #[allow(unused_variables)]
    pub fn build_out_sensors(&mut self, controlledLanes: MSTrafficLightLogic_LaneVectorVector, nb: &mut NLDetectorBuilder) {
        todo!("port MSSOTLE2Sensors::buildOutSensors")
    }

    /// C++: `void MSSOTLE2Sensors::buildOutSensors(MSTrafficLightLogic::LaneVectorVector, NLDetectorBuilder &, double)`
    // void MSSOTLE2Sensors::buildOutSensors(
    //     MSTrafficLightLogic::LaneVectorVector controlledLanes,
    //     NLDetectorBuilder& nb, double sensorLength) {
    //     //for each lane build an appropriate sensor on it
    //     MSLane* currentLane = nullptr;
    //
    //     //input and output lanes
    //     for (MSTrafficLightLogic::LaneVectorVector::const_iterator laneVector =
    //                 controlledLanes.begin(); laneVector != controlledLanes.end();
    //             laneVector++) {
    //         for (MSTrafficLightLogic::LaneVector::const_iterator lane =
    //                     laneVector->begin(); lane != laneVector->end(); lane++) {
    //             currentLane = (*lane);
    //             buildSensorForOutLane(currentLane, nb, sensorLength);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_out_sensors_2(&mut self, controlledLanes: MSTrafficLightLogic_LaneVectorVector, nb: &mut NLDetectorBuilder, sensorLength: f64) {
        todo!("port MSSOTLE2Sensors::buildOutSensors")
    }

    /// C++: `void MSSOTLE2Sensors::buildCountSensors(MSTrafficLightLogic::LaneVectorVector, NLDetectorBuilder &)`
    // void MSSOTLE2Sensors::buildCountSensors(
    //     MSTrafficLightLogic::LaneVectorVector controlledLanes,
    //     NLDetectorBuilder& nb) {
    //     //for each lane build an appropriate sensor on it
    //     MSLane* currentLane = nullptr;
    //     //input and output lanes
    //     for (MSTrafficLightLogic::LaneVectorVector::const_iterator laneVector =
    //                 controlledLanes.begin(); laneVector != controlledLanes.end();
    //             laneVector++) {
    //         for (MSTrafficLightLogic::LaneVector::const_iterator lane =
    //                     laneVector->begin(); lane != laneVector->end(); lane++) {
    //             currentLane = (*lane);
    //             buildCountSensorForLane(currentLane, nb);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_count_sensors(&mut self, controlledLanes: MSTrafficLightLogic_LaneVectorVector, nb: &mut NLDetectorBuilder) {
        todo!("port MSSOTLE2Sensors::buildCountSensors")
    }

    /// C++: `void MSSOTLE2Sensors::buildCountOutSensors(MSTrafficLightLogic::LaneVectorVector, NLDetectorBuilder &)`
    // void MSSOTLE2Sensors::buildCountOutSensors(
    //     MSTrafficLightLogic::LaneVectorVector controlledLanes,
    //     NLDetectorBuilder& nb) {
    //     //for each lane build an appropriate sensor on it
    //     MSLane* currentLane = nullptr;
    //
    //     //input and output lanes
    //     for (MSTrafficLightLogic::LaneVectorVector::const_iterator laneVector =
    //                 controlledLanes.begin(); laneVector != controlledLanes.end();
    //             laneVector++) {
    //         for (MSTrafficLightLogic::LaneVector::const_iterator lane =
    //                     laneVector->begin(); lane != laneVector->end(); lane++) {
    //             currentLane = (*lane);
    //             buildCountSensorForOutLane(currentLane, nb);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_count_out_sensors(&mut self, controlledLanes: MSTrafficLightLogic_LaneVectorVector, nb: &mut NLDetectorBuilder) {
        todo!("port MSSOTLE2Sensors::buildCountOutSensors")
    }

    /// C++: `int MSSOTLE2Sensors::getPassedVeh(std::string, bool)`
    // int MSSOTLE2Sensors::getPassedVeh(std::string laneId, bool /* out */) {
    //     MSLaneID_MSE2CollectorMap::const_iterator sensorsIterator;
    //     sensorsIterator = m_sensorMap.find(laneId);
    //     if (sensorsIterator == m_sensorMap.end()) {
    //         assert(0);
    //         return 0;
    //     } else {
    //         int additional = 0;
    //         if (m_continueSensorOnLanes.find(laneId) != m_continueSensorOnLanes.end())
    //             for (std::vector<std::string>::iterator it = m_continueSensorOnLanes[laneId].begin(); it != m_continueSensorOnLanes[laneId].end(); ++ it) {
    //                 int tmp = 0;
    //                 if (getVelueFromSensor(*it, &MSE2Collector::getPassedVeh, tmp)) {
    //                     additional += tmp;
    //                 }
    //             }
    //         return sensorsIterator->second->getPassedVeh() + additional;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_passed_veh(&mut self, laneId: String, out: bool) -> i32 {
        todo!("port MSSOTLE2Sensors::getPassedVeh")
    }

    /// C++: `void MSSOTLE2Sensors::subtractPassedVeh(std::string, int)`
    // void
    // MSSOTLE2Sensors::subtractPassedVeh(std::string laneId, int passed) {
    //     //TODO Ivan: ciclare tutti i sensori di count e resettare passedVeh
    //     MSLaneID_MSE2CollectorMap::const_iterator sensorsIterator;
    //
    //     sensorsIterator = m_sensorMap.find(laneId);
    //     if (sensorsIterator != m_sensorMap.end()) {
    //         sensorsIterator->second->subtractPassedVeh(passed);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn subtract_passed_veh(&mut self, laneId: String, passed: i32) {
        todo!("port MSSOTLE2Sensors::subtractPassedVeh")
    }

    /// C++: `void MSSOTLE2Sensors::setSpeedThresholdParam(double)`
    // void setSpeedThresholdParam(double newThreshold) {
    //         speedThresholdParam = newThreshold;
    //     }
    #[allow(unused_variables)]
    pub fn set_speed_threshold_param(&mut self, newThreshold: f64) {
        todo!("port MSSOTLE2Sensors::setSpeedThresholdParam")
    }

    /// C++: `int MSSOTLE2Sensors::countVehicles(MSLane *)` [virtual]
    // int MSSOTLE2Sensors::countVehicles(MSLane* lane) {
    //     return countVehicles(lane->getID());
    // }
    #[allow(unused_variables)]
    pub fn count_vehicles(&mut self, lane: Ptr<MSLane>) -> i32 {
        todo!("port MSSOTLE2Sensors::countVehicles")
    }

    /// C++: `int MSSOTLE2Sensors::countVehicles(std::string)` [virtual]
    // int MSSOTLE2Sensors::countVehicles(std::string laneId) {
    //     MSLaneID_MSE2CollectorMap::const_iterator sensorsIterator = m_sensorMap.find(laneId);
    //     if (sensorsIterator == m_sensorMap.end()) {
    //         assert(0);
    //         return 0;
    //     }
    //     int additional = 0;
    //     if (m_continueSensorOnLanes.find(laneId) != m_continueSensorOnLanes.end()) {
    //         for (std::vector<std::string>::iterator it = m_continueSensorOnLanes[laneId].begin(); it != m_continueSensorOnLanes[laneId].end(); ++ it) {
    //             if (m_sensorMap.find(*it) != m_sensorMap.end()) {
    //                 additional += count(m_sensorMap[*it]);
    //             }
    //         }
    //     }
    //     return count(sensorsIterator->second) + additional;
    // }
    #[allow(unused_variables)]
    pub fn count_vehicles_2(&mut self, laneId: String) -> i32 {
        todo!("port MSSOTLE2Sensors::countVehicles")
    }

    /// C++: `int MSSOTLE2Sensors::estimateVehicles(std::string)`
    // int MSSOTLE2Sensors::estimateVehicles(std::string laneId) {
    //     MSLaneID_MSE2CollectorMap::const_iterator sensorsIterator = m_sensorMap.find(laneId);
    //     if (sensorsIterator == m_sensorMap.end()) {
    //         assert(0);
    //         return 0;
    //     }
    //     int additional = 0;
    //     if (m_continueSensorOnLanes.find(laneId) != m_continueSensorOnLanes.end())
    //         for (std::vector<std::string>::iterator it = m_continueSensorOnLanes[laneId].begin(); it != m_continueSensorOnLanes[laneId].end(); ++ it) {
    //             if (m_sensorMap.find(*it) != m_sensorMap.end()) {
    //                 additional += m_sensorMap[*it]->getEstimatedCurrentVehicleNumber(speedThresholdParam);
    //             }
    //         }
    //     return sensorsIterator->second->getEstimatedCurrentVehicleNumber(speedThresholdParam) + additional;
    // }
    #[allow(unused_variables)]
    pub fn estimate_vehicles(&mut self, laneId: String) -> i32 {
        todo!("port MSSOTLE2Sensors::estimateVehicles")
    }

    /// C++: `double MSSOTLE2Sensors::getEstimateQueueLength(std::string)`
    // double MSSOTLE2Sensors::getEstimateQueueLength(std::string laneId) {
    //     MSLaneID_MSE2CollectorMap::const_iterator sensorsIterator = m_sensorMap.find(laneId);
    //     if (sensorsIterator == m_sensorMap.end()) {
    //         assert(0);
    //         return 0;
    //     } else {
    //         double estQL = sensorsIterator->second->getEstimateQueueLength();
    //         if (estQL == -1) {
    //             return 0;
    //         } else
    // //            DBG(
    // //                std::ostringstream str;
    // //                str << "MSSOTLE2Sensors::getEstimateQueueLength lane " << sensorsIterator->second->getLane()->getID()
    // //                << " laneLength " << sensorsIterator->second->getLane()->getLength() << " estimateQueueLength " <<  estQL;
    // //                WRITE_MESSAGE(str.str());
    // //            )
    //         {
    //             return estQL;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_estimate_queue_length(&mut self, laneId: String) -> f64 {
        todo!("port MSSOTLE2Sensors::getEstimateQueueLength")
    }

    /// C++: `double MSSOTLE2Sensors::getMaxSpeed(std::string)` [virtual]
    // double MSSOTLE2Sensors::getMaxSpeed(std::string laneId) {
    //     MSLaneID_MaxSpeedMap::const_iterator sensorsIteratorIn = m_maxSpeedMap.find(laneId);
    //     if (sensorsIteratorIn == m_maxSpeedMap.end()) {
    //         assert(0);
    //         WRITE_ERROR("MSSOTLE2Sensors::meanVehiclesSpeed:: No lane found " + laneId);
    //         return 0;
    //     }
    //     return sensorsIteratorIn->second;
    // }
    #[allow(unused_variables)]
    pub fn get_max_speed(&mut self, laneId: String) -> f64 {
        todo!("port MSSOTLE2Sensors::getMaxSpeed")
    }

    /// C++: `double MSSOTLE2Sensors::meanVehiclesSpeed(MSLane *)` [virtual]
    // double MSSOTLE2Sensors::meanVehiclesSpeed(MSLane* lane) {
    //     return meanVehiclesSpeed(lane->getID());
    // }
    #[allow(unused_variables)]
    pub fn mean_vehicles_speed(&mut self, lane: Ptr<MSLane>) -> f64 {
        todo!("port MSSOTLE2Sensors::meanVehiclesSpeed")
    }

    /// C++: `double MSSOTLE2Sensors::meanVehiclesSpeed(std::string)` [virtual]
    // double MSSOTLE2Sensors::meanVehiclesSpeed(std::string laneId) {
    //     MSLaneID_MSE2CollectorMap::const_iterator sensorsIteratorOut = m_sensorMap.find(laneId);
    //     if (sensorsIteratorOut == m_sensorMap.end()) {
    //         assert(0);
    //         WRITE_ERROR("MSSOTLE2Sensors::meanVehiclesSpeed:: No lane found " + laneId);
    //         return 0;
    //     }
    //     double meanSpeedAcc = 0;
    //     int totalCarNumer = 0;
    //     if (m_continueSensorOnLanes.find(laneId) != m_continueSensorOnLanes.end())
    //         for (std::vector<std::string>::iterator it = m_continueSensorOnLanes[laneId].begin(); it != m_continueSensorOnLanes[laneId].end(); ++ it) {
    //             int number = 0;
    //             double mean = -1;
    //             if (!getVelueFromSensor(*it, &MSE2Collector::getCurrentVehicleNumber, number)) {
    //                 continue;
    //             }
    //             totalCarNumer += number;
    //             getVelueFromSensor(*it, &MSE2Collector::getCurrentMeanSpeed, mean);
    //             meanSpeedAcc += mean * (double) number;
    //         }
    //     int number = sensorsIteratorOut->second->getCurrentVehicleNumber();
    //     totalCarNumer += number;
    //     double mean = sensorsIteratorOut->second->getCurrentMeanSpeed();
    //     meanSpeedAcc += mean * (double) number;
    //     return totalCarNumer == 0 ? -1 : meanSpeedAcc / (double) totalCarNumer;
    // }
    #[allow(unused_variables)]
    pub fn mean_vehicles_speed_2(&mut self, laneId: String) -> f64 {
        todo!("port MSSOTLE2Sensors::meanVehiclesSpeed")
    }

    /// C++: `void MSSOTLE2Sensors::setVehicleWeigths(const std::string &)` [virtual]
    // void MSSOTLE2Sensors::setVehicleWeigths(const std::string& weightString) {
    //     std::vector<std::string> types;
    //     split(weightString, ';', types);
    //     std::ostringstream logstr;
    //     logstr << "[MSSOTLE2Sensors::setVehicleWeigths] ";
    //     for (std::vector<std::string>::iterator typesIt = types.begin(); typesIt != types.end(); ++typesIt) {
    //         std::vector<std::string> typeWeight;
    //         split(*typesIt, '=', typeWeight);
    //         if (typeWeight.size() == 2) {
    //             std::string type = trim(typeWeight[0]);
    //             int value = StringUtils::toInt(typeWeight[1]);
    //             logstr << type << "=" << value << " ";
    //             m_typeWeightMap[type] = value;
    //         }
    //     }
    //     WRITE_MESSAGE(logstr.str());
    // }
    #[allow(unused_variables)]
    pub fn set_vehicle_weigths(&mut self, weightString: &str) {
        todo!("port MSSOTLE2Sensors::setVehicleWeigths")
    }

    /// C++: `int MSSOTLE2Sensors::count(MSE2Collector *)` [protected]
    // int MSSOTLE2Sensors::count(MSE2Collector* sensor) {
    //     int totCars = sensor->getCurrentVehicleNumber();
    //     if (m_typeWeightMap.size() == 0) {
    //         return totCars;
    //     }
    //     int number = 0;
    //     const std::vector<MSE2Collector::VehicleInfo*> vehicles = sensor->getCurrentVehicles();
    //     std::ostringstream logstr;
    //     logstr << "[MSSOTLE2Sensors::count]";
    //     for (std::vector<MSE2Collector::VehicleInfo*>::const_iterator vit = vehicles.begin(); vit != vehicles.end(); ++vit) {
    //         if ((*vit)->onDetector) {
    //             const std::string vtype = (*vit)->type;
    //             if (m_typeWeightMap.find(vtype) != m_typeWeightMap.end()) {
    //                 number += m_typeWeightMap[vtype];
    // //                DBG(logstr << " Added " << m_typeWeightMap[vtype] << " for vtype " << vtype;)
    //             } else {
    //                 ++number;
    //             }
    //         }
    //     }
    // //    DBG(if (totCars != number) {
    // //    logstr << ". Real number " << totCars << "; weighted " << number;
    // //    WRITE_MESSAGE(logstr.str());
    // //    })
    //     return number;
    // }
    #[allow(unused_variables)]
    pub fn count(&mut self, sensor: Ptr<MSE2Collector>) -> i32 {
        todo!("port MSSOTLE2Sensors::count")
    }

    /// C++: `bool MSSOTLE2Sensors::getVelueFromSensor(std::string, Method, ValueType &)` [protected]
    // template<typename Method, typename ValueType>
    //     bool getVelueFromSensor(std::string laneId, Method function, ValueType& value) {
    //         if (m_sensorMap.find(laneId) != m_sensorMap.end()) {
    //             value = (m_sensorMap[laneId]->*function)();
    //             return true;
    //         }
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn get_velue_from_sensor(&mut self) -> bool {
        todo!("port MSSOTLE2Sensors::getVelueFromSensor")
    }

}

/// C++: `std::vector<std::string> &(const std::string &, char, std::vector<std::string> &) split`
// std::vector<std::string>& split(const std::string& s, char delim, std::vector<std::string>& elems) {
//     std::stringstream ss(s);
//     std::string item;
//     while (std::getline(ss, item, delim)) {
//         if (!item.empty()) {
//             elems.push_back(item);
//         }
//     }
//     return elems;
// }
#[allow(unused_variables)]
pub fn split(s: &str, delim: i8, elems: &mut Vec<String>) -> &'static mut Vec<String> {
    todo!("port split")
}

/// C++: `std::string (std::string &) trim`
// std::string trim(std::string& str) {
//     int first = (int)str.find_first_not_of(' ');
//     int last = (int)str.find_last_not_of(' ');
//     return str.substr(first, (last - first + 1));
// }
#[allow(unused_variables)]
pub fn trim(str: &mut String) -> String {
    todo!("port trim")
}

