//! Port of netload/NLDetectorBuilder.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Holds the incoming definitions of an e3 detector unless the detector is build.
/// C++ class `NLDetectorBuilder::E3DetectorDefinition` (netload/NLDetectorBuilder.h)
#[repr(C)]
pub struct NLDetectorBuilder_E3DetectorDefinition {
    /// C++ base class `Parameterised`
    pub base: Parameterised,
    /// The id of the detector
    pub myID: String,
    /// The device the detector shall use
    pub myDevice: String,
    /// The speed a vehicle's speed must be below to be assigned as jammed
    pub myHaltingSpeedThreshold: f64,
    /// The time a vehicle's speed must be below haltingSpeedThreshold to be assigned as jammed
    pub myHaltingTimeThreshold: SUMOTime,
    /// List of detector's entries
    pub myEntries: CrossSectionVector,
    /// List of detector's exits
    pub myExits: CrossSectionVector,
    /// The aggregation interval
    pub mySampleInterval: SUMOTime,
    /// name
    pub myName: String,
    /// The types to filter
    pub myVehicleTypes: String,
    /// The route edges to filter by
    pub myNextEdges: String,
    /// person detection mode
    pub myDetectPersons: i32,
    /// Whether the detector is declared as having incomplete entry detectors
    pub myOpenEntry: bool,
    /// Whether the detector expects vehicles to arrive inside (and doesn't issue a warning in this case)
    pub myExpectArrival: bool,
}

impl std::ops::Deref for NLDetectorBuilder_E3DetectorDefinition {
    type Target = Parameterised;
    fn deref(&self) -> &Parameterised {
        &self.base
    }
}
impl std::ops::DerefMut for NLDetectorBuilder_E3DetectorDefinition {
    fn deref_mut(&mut self) -> &mut Parameterised {
        &mut self.base
    }
}

impl NLDetectorBuilder_E3DetectorDefinition {
    /// C++: `void E3DetectorDefinition::E3DetectorDefinition(const std::string &, const std::string &, double, SUMOTime, SUMOTime, const std::string, const std::string &, const std::string &, int, bool, bool)`
    /// Constructor
    // NLDetectorBuilder::E3DetectorDefinition::E3DetectorDefinition(const std::string& id,
    //         const std::string& device, double haltingSpeedThreshold,
    //         SUMOTime haltingTimeThreshold, SUMOTime splInterval,
    //         const std::string name, const std::string& vTypes,
    //         const std::string& nextEdges,
    //         int detectPersons, bool openEntry, bool expectArrival) :
    //     myID(id), myDevice(device),
    //     myHaltingSpeedThreshold(haltingSpeedThreshold),
    //     myHaltingTimeThreshold(haltingTimeThreshold),
    //     mySampleInterval(splInterval),
    //     myName(name),
    //     myVehicleTypes(vTypes),
    //     myNextEdges(nextEdges),
    //     myDetectPersons(detectPersons),
    //     myOpenEntry(openEntry),
    //     myExpectArrival(expectArrival) {
    // }
    #[allow(unused_variables)]
    pub fn new(id: &str, device: &str, haltingSpeedThreshold: f64, haltingTimeThreshold: SUMOTime, splInterval: SUMOTime, name: String, vTypes: &str, nextEdges: &str, detectPersons: i32, openEntry: bool, expectArrival: bool) -> Self {
        todo!("port E3DetectorDefinition::E3DetectorDefinition")
    }

    /// C++: `void E3DetectorDefinition::~E3DetectorDefinition() noexcept` [virtual]
    /// Destructor
    // NLDetectorBuilder::E3DetectorDefinition::~E3DetectorDefinition() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port E3DetectorDefinition::~E3DetectorDefinition")
    }

    /// C++: `void E3DetectorDefinition::E3DetectorDefinition(const E3DetectorDefinition &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &NLDetectorBuilder_E3DetectorDefinition) -> Self {
        todo!("port E3DetectorDefinition::E3DetectorDefinition")
    }

    /// C++: `E3DetectorDefinition & E3DetectorDefinition::operator=(const E3DetectorDefinition &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &NLDetectorBuilder_E3DetectorDefinition) -> &mut NLDetectorBuilder_E3DetectorDefinition {
        todo!("port E3DetectorDefinition::operator=")
    }

}

/// Builds detectors for microsim
/// C++ class `NLDetectorBuilder` (netload/NLDetectorBuilder.h)
pub struct NLDetectorBuilder {
    /// The net to fill
    pub myNet: Ptr<MSNet>,
    /// definition of the currently parsed e3 detector
    pub myE3Definition: Ptr<NLDetectorBuilder_E3DetectorDefinition>,
}

impl NLDetectorBuilder {
    /// C++: `void NLDetectorBuilder::NLDetectorBuilder(MSNet &)`
    /// Constructor
    // NLDetectorBuilder::NLDetectorBuilder(MSNet& net)
    //     : myNet(net), myE3Definition(nullptr) {}
    #[allow(unused_variables)]
    pub fn new(net: &mut MSNet) -> Self {
        todo!("port NLDetectorBuilder::NLDetectorBuilder")
    }

    /// C++: `void NLDetectorBuilder::~NLDetectorBuilder() noexcept` [virtual]
    /// Destructor
    // NLDetectorBuilder::~NLDetectorBuilder() {
    //     delete myE3Definition;
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NLDetectorBuilder::~NLDetectorBuilder")
    }

    /// C++: `Parameterised * NLDetectorBuilder::buildInductLoop(const std::string &, const std::string &, double, double, SUMOTime, const std::string &, bool, const std::string, const std::string &, const std::string &, bool, int)`
    /// Builds an e1 detector and adds it to the net
    // Parameterised*
    // NLDetectorBuilder::buildInductLoop(const std::string& id,
    //                                    const std::string& lane, double pos, double length, SUMOTime splInterval,
    //                                    const std::string& device, bool friendlyPos,
    //                                    const std::string name,
    //                                    const std::string& vTypes,
    //                                    const std::string& nextEdges,
    //                                    bool mesoTLS,
    //                                    int detectPersons) {
    //     checkSampleInterval(splInterval, SUMO_TAG_E1DETECTOR, id);
    //     // get and check the lane
    //     MSLane* clane = getLaneChecking(lane, SUMO_TAG_E1DETECTOR, id);
    //     // get and check the position
    //     pos = getPositionChecking(pos, clane, friendlyPos, SUMO_TAG_E1DETECTOR, id);
    //     if (length < 0) {
    //         throw InvalidArgument("The length of " + toString(SUMO_TAG_E1DETECTOR) + " '" + id + "' cannot be negative");
    //     } else if (length > 0 && pos + length > clane->getLength()) {
    //         if (friendlyPos) {
    //             length = MIN2(length, clane->getLength());
    //             pos = clane->getLength() - length;
    //         } else {
    //             throw InvalidArgument("The length of " + toString(SUMO_TAG_E1DETECTOR) + " '" + id + "' puts it beyond the lane's '" + clane->getID() + "' end.");
    //         }
    //     }
    //     // build the loop
    //     MSDetectorFileOutput* loop = createInductLoop(id, clane, pos, length, name, vTypes, nextEdges, detectPersons, true, mesoTLS);
    //     // add the file output
    //     myNet.getDetectorControl().add(SUMO_TAG_INDUCTION_LOOP, loop, device, splInterval);
    //     return loop;
    // }
    #[allow(unused_variables)]
    pub fn build_induct_loop(&mut self, id: &str, lane: &str, pos: f64, length: f64, splInterval: SUMOTime, device: &str, friendlyPos: bool, name: String, vTypes: &str, nextEdges: &str, mesoTLS: bool, detectPersons: i32) -> Ptr<Parameterised> {
        todo!("port NLDetectorBuilder::buildInductLoop")
    }

    /// C++: `Parameterised * NLDetectorBuilder::buildInstantInductLoop(const std::string &, const std::string &, double, const std::string &, bool, const std::string, const std::string &, const std::string &, int)`
    /// Builds an instantenous induction and adds it to the net
    // Parameterised*
    // NLDetectorBuilder::buildInstantInductLoop(const std::string& id,
    //         const std::string& lane, double pos,
    //         const std::string& device, bool friendlyPos,
    //         const std::string name,
    //         const std::string& vTypes,
    //         const std::string& nextEdges,
    //         int detectPersons) {
    //     // get and check the lane
    //     MSLane* clane = getLaneChecking(lane, SUMO_TAG_INSTANT_INDUCTION_LOOP, id);
    //     // get and check the position
    //     pos = getPositionChecking(pos, clane, friendlyPos, SUMO_TAG_INSTANT_INDUCTION_LOOP, id);
    //     // build the loop
    //     MSDetectorFileOutput* loop = createInstantInductLoop(id, clane, pos, device, name, vTypes, nextEdges, detectPersons);
    //     // add the file output
    //     myNet.getDetectorControl().add(SUMO_TAG_INSTANT_INDUCTION_LOOP, loop);
    //     return loop;
    // }
    #[allow(unused_variables)]
    pub fn build_instant_induct_loop(&mut self, id: &str, lane: &str, pos: f64, device: &str, friendlyPos: bool, name: String, vTypes: &str, nextEdges: &str, detectPersons: i32) -> Ptr<Parameterised> {
        todo!("port NLDetectorBuilder::buildInstantInductLoop")
    }

    /// C++: `Parameterised * NLDetectorBuilder::buildE2Detector(const std::string &, MSLane *, double, double, double, const std::string &, SUMOTime, SUMOTime, double, double, const std::string, const std::string &, const std::string &, int, bool, bool, MSTLLogicControl::TLSLogicVariants *, MSLane *)`
    /// Builds a new E2 detector and adds it to the net's detector control. Also performs some consistency checks for the detector positioning and applies "friendly positioning"
    // Parameterised*
    // NLDetectorBuilder::buildE2Detector(const std::string& id, MSLane* lane, double pos, double endPos, double length,
    //                                    const std::string& device, SUMOTime frequency,
    //                                    SUMOTime haltingTimeThreshold, double haltingSpeedThreshold, double jamDistThreshold,
    //                                    const std::string name, const std::string& vTypes,
    //                                    const std::string& nextEdges,
    //                                    int detectPersons, bool friendlyPos, bool showDetector,
    //                                    MSTLLogicControl::TLSLogicVariants* tlls, MSLane* toLane) {
    //
    //     bool tlsGiven = tlls != nullptr;
    //     bool toLaneGiven = toLane != nullptr;
    //     bool posGiven = pos != std::numeric_limits<double>::max();
    //     bool endPosGiven = endPos != std::numeric_limits<double>::max();
    //
    //     assert(posGiven || endPosGiven);
    //
    //     // Check positioning
    //     if (posGiven) {
    //         if (pos >= lane->getLength() || (pos < 0 && -pos > lane->getLength())) {
    //             std::stringstream ss;
    //             ss << "The given position (=" << pos << ") for detector '" << id
    //                << "' does not lie on the given lane '" << lane->getID()
    //                << "' with length " << lane->getLength();
    //             if (friendlyPos) {
    //                 double newPos = pos > 0 ? lane->getLength() - POSITION_EPS : 0.;
    //                 ss << " (adjusting to new position " << newPos;
    //                 WRITE_WARNING(ss.str());
    //                 pos = newPos;
    //             } else {
    //                 ss << " (0 <= pos < lane->getLength() is required)";
    //                 throw InvalidArgument(ss.str());
    //             }
    //         }
    //     }
    //     if (endPosGiven) {
    //         if (endPos > lane->getLength() || (endPos <= 0 && -endPos >= lane->getLength())) {
    //             std::stringstream ss;
    //             ss << "The given end position (=" << endPos << ") for detector '" << id
    //                << "' does not lie on the given lane '" << lane->getID()
    //                << "' with length " << lane->getLength();
    //             if (friendlyPos) {
    //                 double newEndPos = endPos > 0 ? lane->getLength() : POSITION_EPS;
    //                 ss << " (adjusting to new position " << newEndPos;
    //                 WRITE_WARNING(ss.str());
    //                 pos = newEndPos;
    //             } else {
    //                 ss << " (0 <= pos < lane->getLength() is required)";
    //                 throw InvalidArgument(ss.str());
    //             }
    //         }
    //     }
    //
    //     MSE2Collector* det = nullptr;
    //     if (tlsGiven) {
    //         // Detector connected to TLS
    //         det =  createE2Detector(id, DU_USER_DEFINED, lane, pos, endPos, length, haltingTimeThreshold, haltingSpeedThreshold, jamDistThreshold, name, vTypes, nextEdges, detectPersons, showDetector);
    //         myNet.getDetectorControl().add(SUMO_TAG_LANE_AREA_DETECTOR, det);
    //         // add the file output (XXX: Where's the corresponding delete?)
    //         if (toLaneGiven) {
    //             // Detector also associated to specific link
    //             const MSLane* const lastLane = det->getLastLane();
    //             const MSLink* const link = lastLane->getLinkTo(toLane);
    //             if (link == nullptr) {
    //                 throw InvalidArgument(
    //                     "The detector '" + id + "' cannot be build as no connection between lanes '"
    //                     + lastLane->getID() + "' and '" + toLane->getID() + "' exists.");
    //             }
    //             new Command_SaveTLCoupledLaneDet(*tlls, det, myNet.getCurrentTimeStep(), OutputDevice::getDevice(device), link);
    //         } else {
    //             // detector for tls but without specific link
    //             new Command_SaveTLCoupledDet(*tlls, det, myNet.getCurrentTimeStep(), OutputDevice::getDevice(device));
    //         }
    //     } else {
    //         // User specified detector for xml-output
    //         checkSampleInterval(frequency, SUMO_TAG_E2DETECTOR, id);
    //         det = createE2Detector(id, DU_USER_DEFINED, lane, pos, endPos, length, haltingTimeThreshold, haltingSpeedThreshold, jamDistThreshold, name, vTypes, nextEdges, detectPersons, showDetector);
    //         myNet.getDetectorControl().add(SUMO_TAG_LANE_AREA_DETECTOR, det, device, frequency);
    //     }
    //     return det;
    // }
    #[allow(unused_variables)]
    pub fn build_e2_detector(&mut self, id: &str, lane: Ptr<MSLane>, pos: f64, endPos: f64, length: f64, device: &str, frequency: SUMOTime, haltingTimeThreshold: SUMOTime, haltingSpeedThreshold: f64, jamDistThreshold: f64, name: String, vTypes: &str, nextEdges: &str, detectPersons: i32, friendlyPos: bool, showDetector: bool, tlls: Ptr<MSTLLogicControl_TLSLogicVariants>, toLane: Ptr<MSLane>) -> Ptr<Parameterised> {
        todo!("port NLDetectorBuilder::buildE2Detector")
    }

    /// C++: `Parameterised * NLDetectorBuilder::buildE2Detector(const std::string &, std::vector<MSLane *>, double, double, const std::string &, SUMOTime, SUMOTime, double, double, const std::string, const std::string &, const std::string &, int, bool, bool, MSTLLogicControl::TLSLogicVariants *, MSLane *)`
    // Parameterised*
    // NLDetectorBuilder::buildE2Detector(const std::string& id, std::vector<MSLane*> lanes, double pos, double endPos,
    //                                    const std::string& device, SUMOTime frequency,
    //                                    SUMOTime haltingTimeThreshold, double haltingSpeedThreshold, double jamDistThreshold,
    //                                    const std::string name, const std::string& vTypes,
    //                                    const std::string& nextEdges,
    //                                    int detectPersons, bool friendlyPos, bool showDetector,
    //                                    MSTLLogicControl::TLSLogicVariants* tlls, MSLane* toLane) {
    //
    //     bool tlsGiven = tlls != nullptr;
    //     bool toLaneGiven = toLane != nullptr;
    //     assert(pos != std::numeric_limits<double>::max());
    //     assert(endPos != std::numeric_limits<double>::max());
    //     assert(lanes.size() != 0);
    //
    //     const MSLane* const firstLane = lanes[0];
    //     const MSLane* const lastLane = lanes.back();
    //
    //     // Check positioning
    //     if (pos >= firstLane->getLength() || (pos < 0 && -pos > firstLane->getLength())) {
    //         std::stringstream ss;
    //         ss << "The given position (=" << pos << ") for detector '" << id
    //            << "' does not lie on the given lane '" << firstLane->getID()
    //            << "' with length " << firstLane->getLength();
    //         if (friendlyPos) {
    //             double newPos = pos > 0 ? firstLane->getLength() - POSITION_EPS : 0.;
    //             ss << " (adjusting to new position " << newPos;
    //             WRITE_WARNING(ss.str());
    //             pos = newPos;
    //         } else {
    //             ss << " (0 <= pos < lane->getLength() is required)";
    //             throw InvalidArgument(ss.str());
    //         }
    //     }
    //     if (endPos > lastLane->getLength() || (endPos <= 0 && -endPos >= lastLane->getLength())) {
    //         std::stringstream ss;
    //         ss << "The given end position (=" << endPos << ") for detector '" << id
    //            << "' does not lie on the given lane '" << lastLane->getID()
    //            << "' with length " << lastLane->getLength();
    //         if (friendlyPos) {
    //             double newEndPos = endPos > 0 ? lastLane->getLength() : POSITION_EPS;
    //             ss << " (adjusting to new position " << newEndPos;
    //             WRITE_WARNING(ss.str());
    //             pos = newEndPos;
    //         } else {
    //             ss << " (0 <= pos < lane->getLength() is required)";
    //             throw InvalidArgument(ss.str());
    //         }
    //     }
    //
    //     MSE2Collector* det = nullptr;
    //     if (tlsGiven) {
    //         // Detector connected to TLS
    //         det = createE2Detector(id, DU_USER_DEFINED, lanes, pos, endPos, haltingTimeThreshold, haltingSpeedThreshold, jamDistThreshold, name, vTypes, nextEdges, detectPersons, showDetector);
    //         myNet.getDetectorControl().add(SUMO_TAG_LANE_AREA_DETECTOR, det);
    //         // add the file output (XXX: Where's the corresponding delete?)
    //         if (toLaneGiven) {
    //             // Detector also associated to specific link
    //             const MSLane* const lastDetLane = det->getLastLane();
    //             const MSLink* const link = lastDetLane->getLinkTo(toLane);
    //             if (link == nullptr) {
    //                 throw InvalidArgument(
    //                     "The detector '" + id + "' cannot be build as no connection between lanes '"
    //                     + lastDetLane->getID() + "' and '" + toLane->getID() + "' exists.");
    //             }
    //             new Command_SaveTLCoupledLaneDet(*tlls, det, myNet.getCurrentTimeStep(), OutputDevice::getDevice(device), link);
    //         } else {
    //             // detector for tls but without specific link
    //             new Command_SaveTLCoupledDet(*tlls, det, myNet.getCurrentTimeStep(), OutputDevice::getDevice(device));
    //         }
    //     } else {
    //         // User specified detector for xml-output
    //         checkSampleInterval(frequency, SUMO_TAG_E2DETECTOR, id);
    //
    //         det = createE2Detector(id, DU_USER_DEFINED, lanes, pos, endPos, haltingTimeThreshold, haltingSpeedThreshold, jamDistThreshold, name, vTypes, nextEdges, detectPersons, showDetector);
    //         myNet.getDetectorControl().add(SUMO_TAG_LANE_AREA_DETECTOR, det, device, frequency);
    //     }
    //     return det;
    // }
    #[allow(unused_variables)]
    pub fn build_e2_detector_2(&mut self, id: &str, lanes: Vec<Ptr<MSLane>>, pos: f64, endPos: f64, device: &str, frequency: SUMOTime, haltingTimeThreshold: SUMOTime, haltingSpeedThreshold: f64, jamDistThreshold: f64, name: String, vTypes: &str, nextEdges: &str, detectPersons: i32, friendlyPos: bool, showDetector: bool, tlls: Ptr<MSTLLogicControl_TLSLogicVariants>, toLane: Ptr<MSLane>) -> Ptr<Parameterised> {
        todo!("port NLDetectorBuilder::buildE2Detector")
    }

    /// C++: `Parameterised * NLDetectorBuilder::beginE3Detector(const std::string &, const std::string &, SUMOTime, double, SUMOTime, const std::string, const std::string &, const std::string &, int, bool, bool)`
    /// Stores temporary the initial information about an e3 detector to build
    // Parameterised*
    // NLDetectorBuilder::beginE3Detector(const std::string& id,
    //                                    const std::string& device, SUMOTime splInterval,
    //                                    double haltingSpeedThreshold,
    //                                    SUMOTime haltingTimeThreshold,
    //                                    const std::string name,
    //                                    const std::string& vTypes,
    //                                    const std::string& nextEdges,
    //                                    int detectPersons, bool openEntry, bool expectArrival) {
    //     checkSampleInterval(splInterval, SUMO_TAG_E3DETECTOR, id);
    //     myE3Definition = new E3DetectorDefinition(id, device, haltingSpeedThreshold, haltingTimeThreshold, splInterval, name, vTypes, nextEdges, detectPersons, openEntry, expectArrival);
    //     return myE3Definition;
    // }
    #[allow(unused_variables)]
    pub fn begin_e3_detector(&mut self, id: &str, device: &str, splInterval: SUMOTime, haltingSpeedThreshold: f64, haltingTimeThreshold: SUMOTime, name: String, vTypes: &str, nextEdges: &str, detectPersons: i32, openEntry: bool, expectArrival: bool) -> Ptr<Parameterised> {
        todo!("port NLDetectorBuilder::beginE3Detector")
    }

    /// C++: `void NLDetectorBuilder::addE3Entry(const std::string &, double, bool)`
    /// Builds an entry point of an e3 detector
    // void
    // NLDetectorBuilder::addE3Entry(const std::string& lane,
    //                               double pos, bool friendlyPos) {
    //     if (myE3Definition == nullptr) {
    //         return;
    //     }
    //     MSLane* clane = getLaneChecking(lane, SUMO_TAG_E3DETECTOR, myE3Definition->myID);
    //     // get and check the position
    //     pos = getPositionChecking(pos, clane, friendlyPos, SUMO_TAG_DET_ENTRY, myE3Definition->myID);
    //     // build and save the entry
    //     myE3Definition->myEntries.push_back(MSCrossSection(clane, pos));
    // }
    #[allow(unused_variables)]
    pub fn add_e3_entry(&mut self, lane: &str, pos: f64, friendlyPos: bool) {
        todo!("port NLDetectorBuilder::addE3Entry")
    }

    /// C++: `void NLDetectorBuilder::addE3Exit(const std::string &, double, bool)`
    /// Builds an exit point of an e3 detector
    // void
    // NLDetectorBuilder::addE3Exit(const std::string& lane,
    //                              double pos, bool friendlyPos) {
    //     if (myE3Definition == nullptr) {
    //         return;
    //     }
    //     MSLane* clane = getLaneChecking(lane, SUMO_TAG_E3DETECTOR, myE3Definition->myID);
    //     // get and check the position
    //     pos = getPositionChecking(pos, clane, friendlyPos, SUMO_TAG_DET_EXIT, myE3Definition->myID);
    //     // build and save the exit
    //     myE3Definition->myExits.push_back(MSCrossSection(clane, pos));
    // }
    #[allow(unused_variables)]
    pub fn add_e3_exit(&mut self, lane: &str, pos: f64, friendlyPos: bool) {
        todo!("port NLDetectorBuilder::addE3Exit")
    }

    /// C++: `void NLDetectorBuilder::endE3Detector()`
    /// Builds of an e3 detector using collected values
    // void
    // NLDetectorBuilder::endE3Detector() {
    //     if (myE3Definition == nullptr) {
    //         return;
    //     }
    //     // If E3 own entry or exit detectors
    //     if (myE3Definition->myEntries.size() > 0 || myE3Definition->myExits.size() > 0) {
    //         // create E3 detector
    //         MSDetectorFileOutput* det = createE3Detector(myE3Definition->myID,
    //                                     myE3Definition->myEntries, myE3Definition->myExits,
    //                                     myE3Definition->myHaltingSpeedThreshold, myE3Definition->myHaltingTimeThreshold,
    //                                     myE3Definition->myName,
    //                                     myE3Definition->myVehicleTypes,
    //                                     myE3Definition->myNextEdges,
    //                                     myE3Definition->myDetectPersons,
    //                                     myE3Definition->myOpenEntry,
    //                                     myE3Definition->myExpectArrival);
    //         det->updateParameters(myE3Definition->getParametersMap());
    //         // add to net
    //         myNet.getDetectorControl().add(SUMO_TAG_ENTRY_EXIT_DETECTOR, det, myE3Definition->myDevice, myE3Definition->mySampleInterval);
    //     } else {
    //         WRITE_WARNING(toString(SUMO_TAG_E3DETECTOR) + " with id = '" + myE3Definition->myID + "' will not be created because is empty (no " + toString(SUMO_TAG_DET_ENTRY) + " or " + toString(SUMO_TAG_DET_EXIT) + " was defined)")
    //     }
    //     // clean up
    //     delete myE3Definition;
    //     myE3Definition = nullptr;
    // }
    #[allow(unused_variables)]
    pub fn end_e3_detector(&mut self) {
        todo!("port NLDetectorBuilder::endE3Detector")
    }

    /// C++: `std::string NLDetectorBuilder::getCurrentE3ID() const` [const]
    /// Returns the id of the currently built e3 detector
    // std::string
    // NLDetectorBuilder::getCurrentE3ID() const {
    //     if (myE3Definition == nullptr) {
    //         return "<unknown>";
    //     }
    //     return myE3Definition->myID;
    // }
    #[allow(unused_variables)]
    pub fn get_current_e3_id(&self) -> String {
        todo!("port NLDetectorBuilder::getCurrentE3ID")
    }

    /// C++: `void NLDetectorBuilder::buildVTypeProbe(const std::string &, const std::string &, SUMOTime, const std::string &)`
    /// Builds a vTypeProbe and adds it to the net
    // void
    // NLDetectorBuilder::buildVTypeProbe(const std::string& id,
    //                                    const std::string& vtype, SUMOTime frequency,
    //                                    const std::string& device) {
    //     checkSampleInterval(frequency, SUMO_TAG_VTYPEPROBE, id);
    //     new MSVTypeProbe(id, vtype, OutputDevice::getDevice(device), frequency);
    // }
    #[allow(unused_variables)]
    pub fn build_v_type_probe(&mut self, id: &str, vtype: &str, frequency: SUMOTime, device: &str) {
        todo!("port NLDetectorBuilder::buildVTypeProbe")
    }

    /// C++: `void NLDetectorBuilder::buildRouteProbe(const std::string &, const std::string &, SUMOTime, SUMOTime, const std::string &, const std::string &)`
    /// Builds a routeProbe and adds it to the net
    // void
    // NLDetectorBuilder::buildRouteProbe(const std::string& id, const std::string& edge,
    //                                    SUMOTime frequency, SUMOTime begin,
    //                                    const std::string& device,
    //                                    const std::string& vTypes) {
    //     checkSampleInterval(frequency, SUMO_TAG_ROUTEPROBE, id);
    //     MSEdge* e = getEdgeChecking(edge, SUMO_TAG_ROUTEPROBE, id);
    //     MSRouteProbe* probe = new MSRouteProbe(id, e, id + "_" + toString(begin), id + "_" + toString(begin - frequency), vTypes);
    //     // add the file output
    //     myNet.getDetectorControl().add(SUMO_TAG_ROUTEPROBE, probe, device, frequency, begin);
    // }
    #[allow(unused_variables)]
    pub fn build_route_probe(&mut self, id: &str, edge: &str, frequency: SUMOTime, begin: SUMOTime, device: &str, vTypes: &str) {
        todo!("port NLDetectorBuilder::buildRouteProbe")
    }

    /// C++: `MSDetectorFileOutput * NLDetectorBuilder::createInductLoop(const std::string &, MSLane *, double, double, const std::string, const std::string &, const std::string &, int, bool, bool)` [virtual]
    /// Creates an instance of an e1 detector using the given values
    // MSDetectorFileOutput*
    // NLDetectorBuilder::createInductLoop(const std::string& id,
    //                                     MSLane* lane, double pos,
    //                                     double length,
    //                                     const std::string name,
    //                                     const std::string& vTypes,
    //                                     const std::string& nextEdges,
    //                                     int detectPersons,
    //                                     bool /*show*/,
    //                                     bool forceMicro) {
    //     if (MSGlobals::gUseMesoSim && !forceMicro) {
    //         return new MEInductLoop(id, MSGlobals::gMesoNet->getSegmentForEdge(lane->getEdge(), pos), pos, name, vTypes, nextEdges, detectPersons);
    //     }
    //     return new MSInductLoop(id, lane, pos, length, name, vTypes, nextEdges, detectPersons, false);
    // }
    #[allow(unused_variables)]
    pub fn create_induct_loop(&mut self, id: &str, lane: Ptr<MSLane>, pos: f64, length: f64, name: String, vTypes: &str, nextEdges: &str, detectPersons: i32, show: bool, forceMicro: bool) -> Ptr<MSDetectorFileOutput> {
        todo!("port NLDetectorBuilder::createInductLoop")
    }

    /// C++: `MSDetectorFileOutput * NLDetectorBuilder::createInstantInductLoop(const std::string &, MSLane *, double, const std::string &, const std::string, const std::string &, const std::string &, int)` [virtual]
    /// Creates an instance of an e1 detector using the given values
    // MSDetectorFileOutput*
    // NLDetectorBuilder::createInstantInductLoop(const std::string& id,
    //         MSLane* lane, double pos, const std::string& od,
    //         const std::string name, const std::string& vTypes,
    //         const std::string& nextEdges,
    //         int detectPersons) {
    //     return new MSInstantInductLoop(id, OutputDevice::getDevice(od), lane, pos, name, vTypes, nextEdges, detectPersons);
    // }
    #[allow(unused_variables)]
    pub fn create_instant_induct_loop(&mut self, id: &str, lane: Ptr<MSLane>, pos: f64, od: &str, name: String, vTypes: &str, nextEdges: &str, detectPersons: i32) -> Ptr<MSDetectorFileOutput> {
        todo!("port NLDetectorBuilder::createInstantInductLoop")
    }

    /// C++: `MSE2Collector * NLDetectorBuilder::createE2Detector(const std::string &, DetectorUsage, MSLane *, double, double, double, SUMOTime, double, double, const std::string, const std::string &, const std::string &, int, bool)` [virtual]
    /// Creates a MSE2Collector instance, overridden by GUIE2Collector::createE2Detector()
    // MSE2Collector*
    // NLDetectorBuilder::createE2Detector(const std::string& id,
    //                                     DetectorUsage usage, MSLane* lane, double pos, double endPos, double length,
    //                                     SUMOTime haltingTimeThreshold, double haltingSpeedThreshold, double jamDistThreshold,
    //                                     const std::string name, const std::string& vTypes,
    //                                     const std::string& nextEdges,
    //                                     int detectPersons, bool /* showDetector */) {
    //     return new MSE2Collector(id, usage, lane, pos, endPos, length, haltingTimeThreshold, haltingSpeedThreshold, jamDistThreshold, name, vTypes, nextEdges, detectPersons);
    // }
    #[allow(unused_variables)]
    pub fn create_e2_detector(&mut self, id: &str, usage: DetectorUsage, lane: Ptr<MSLane>, pos: f64, endPos: f64, length: f64, haltingTimeThreshold: SUMOTime, haltingSpeedThreshold: f64, jamDistThreshold: f64, name: String, vTypes: &str, nextEdges: &str, detectPersons: i32, showDetector: bool) -> Ptr<MSE2Collector> {
        todo!("port NLDetectorBuilder::createE2Detector")
    }

    /// C++: `MSE2Collector * NLDetectorBuilder::createE2Detector(const std::string &, DetectorUsage, std::vector<MSLane *>, double, double, SUMOTime, double, double, const std::string, const std::string &, const std::string &, int, bool)` [virtual]
    // MSE2Collector*
    // NLDetectorBuilder::createE2Detector(const std::string& id,
    //                                     DetectorUsage usage, std::vector<MSLane*> lanes, double pos, double endPos,
    //                                     SUMOTime haltingTimeThreshold, double haltingSpeedThreshold, double jamDistThreshold,
    //                                     const std::string name, const std::string& vTypes,
    //                                     const std::string& nextEdges,
    //                                     int detectPersons, bool /* showDetector */) {
    //     return new MSE2Collector(id, usage, lanes, pos, endPos, haltingTimeThreshold, haltingSpeedThreshold, jamDistThreshold, name, vTypes, nextEdges, detectPersons);
    // }
    #[allow(unused_variables)]
    pub fn create_e2_detector_2(&mut self, id: &str, usage: DetectorUsage, lanes: Vec<Ptr<MSLane>>, pos: f64, endPos: f64, haltingTimeThreshold: SUMOTime, haltingSpeedThreshold: f64, jamDistThreshold: f64, name: String, vTypes: &str, nextEdges: &str, detectPersons: i32, showDetector: bool) -> Ptr<MSE2Collector> {
        todo!("port NLDetectorBuilder::createE2Detector")
    }

    /// C++: `MSDetectorFileOutput * NLDetectorBuilder::createE3Detector(const std::string &, const CrossSectionVector &, const CrossSectionVector &, double, SUMOTime, const std::string, const std::string &, const std::string &, int, bool, bool)` [virtual]
    /// Creates an instance of an e3 detector using the given values
    // MSDetectorFileOutput*
    // NLDetectorBuilder::createE3Detector(const std::string& id,
    //                                     const CrossSectionVector& entries,
    //                                     const CrossSectionVector& exits,
    //                                     double haltingSpeedThreshold,
    //                                     SUMOTime haltingTimeThreshold,
    //                                     const std::string name, const std::string& vTypes,
    //                                     const std::string& nextEdges,
    //                                     int detectPersons,
    //                                     bool openEntry,
    //                                     bool expectArrival) {
    //     return new MSE3Collector(id, entries, exits, haltingSpeedThreshold, haltingTimeThreshold, name, vTypes, nextEdges, detectPersons, openEntry, expectArrival);
    // }
    #[allow(unused_variables)]
    pub fn create_e3_detector(&mut self, id: &str, entries: &CrossSectionVector, exits: &CrossSectionVector, haltingSpeedThreshold: f64, haltingTimeThreshold: SUMOTime, name: String, vTypes: &str, nextEdges: &str, detectPersons: i32, openEntry: bool, expectArrival: bool) -> Ptr<MSDetectorFileOutput> {
        todo!("port NLDetectorBuilder::createE3Detector")
    }

    /// C++: `void NLDetectorBuilder::createEdgeLaneMeanData(const std::string &, SUMOTime, SUMOTime, SUMOTime, const std::string &, const bool, const std::string &, const bool, const bool, const int, const double, const double, const double, const double, const std::string &, const std::string &, std::vector<MSEdge *>, AggregateType, const std::string &)`
    /// Creates edge based mean data collector using the given specification
    // void
    // NLDetectorBuilder::createEdgeLaneMeanData(const std::string& id, SUMOTime frequency,
    //         SUMOTime begin, SUMOTime end, const std::string& type,
    //         const bool useLanes, const std::string& excludeEmpty,
    //         const bool withInternal, const bool trackVehicles, const int detectPersons,
    //         const double maxTravelTime, const double minSamples,
    //         const double haltSpeed, const double haltSpeedRel, const std::string& vTypes,
    //         const std::string& writeAttributes,
    //         std::vector<MSEdge*> edges,
    //         AggregateType aggregate,
    //         const std::string& device) {
    //     if (begin < 0) {
    //         throw InvalidArgument("Negative begin time for meandata dump '" + id + "'.");
    //     }
    //     if (end < 0) {
    //         end = SUMOTime_MAX;
    //     }
    //     if (end <= begin) {
    //         throw InvalidArgument("End before or at begin for meandata dump '" + id + "'.");
    //     }
    //     checkStepLengthMultiple(begin, " for meandata dump '" + id + "'");
    //     std::vector<MeanDataType> types;
    //     if (type == SUMOXMLDefinitions::MeanDataTypes.getString(MeanDataType::DEFAULT)) {
    //         const SumoXMLAttrMask attrs = OutputDevice::parseWrittenAttributes(StringTokenizer(writeAttributes).getVector(), "meandata '" + id + "'");
    //         if (attrs.none()) {
    //             types.push_back(MeanDataType::TRAFFIC);
    //         } else {
    //             if (MSMeanData_Net::supports(attrs)) {
    //                 types.push_back(MeanDataType::TRAFFIC);
    //             }
    //             if (MSMeanData_Emissions::supports(attrs)) {
    //                 types.push_back(MeanDataType::EMISSIONS);
    //             }
    //             if (MSMeanData_Harmonoise::supports(attrs)) {
    //                 types.push_back(MeanDataType::HARMONOISE);
    //             }
    //         }
    //     } else {
    //         for (const std::string& t : StringTokenizer(type).getVector()) {
    //             if (SUMOXMLDefinitions::MeanDataTypes.hasString(t)) {
    //                 types.push_back(SUMOXMLDefinitions::MeanDataTypes.get(t));
    //             } else if (t == "performance") {
    //                 types.push_back(MeanDataType::TRAFFIC);
    //             } else if (t == "hbefa") {
    //                 WRITE_WARNING(TL("The meandata type 'hbefa' is deprecated. Please use the type 'emissions' instead."));
    //                 types.push_back(MeanDataType::EMISSIONS);
    //             } else {
    //                 throw InvalidArgument("Invalid type '" + t + "' for meandata dump '" + id + "'.");
    //             }
    //         }
    //     }
    //     if (frequency < 0) {
    //         frequency = end - begin;
    //     } else {
    //         checkStepLengthMultiple(frequency, " for meandata dump '" + id + "'");
    //     }
    //     MSMeanData* first = nullptr;
    //     MSMeanData* parent = nullptr;
    //     for (const MeanDataType t : types) {
    //         MSMeanData* det = nullptr;
    //         if (t == MeanDataType::TRAFFIC) {
    //             det = new MSMeanData_Net(id, begin, end, useLanes, excludeEmpty,
    //                                      withInternal, trackVehicles, detectPersons, maxTravelTime, minSamples, haltSpeed, haltSpeedRel, vTypes, writeAttributes, edges, aggregate);
    //         } else if (t == MeanDataType::EMISSIONS) {
    //             det = new MSMeanData_Emissions(id, begin, end, useLanes, excludeEmpty,
    //                                            withInternal, trackVehicles, maxTravelTime, minSamples, vTypes, writeAttributes, edges, aggregate);
    //         } else if (t == MeanDataType::HARMONOISE) {
    //             det = new MSMeanData_Harmonoise(id, begin, end, useLanes, excludeEmpty,
    //                                             withInternal, trackVehicles, maxTravelTime, minSamples, vTypes, writeAttributes, edges, aggregate);
    //         } else if (t == MeanDataType::AMITRAN) {
    //             det = new MSMeanData_Amitran(id, begin, end, useLanes, excludeEmpty,
    //                                          withInternal, trackVehicles, detectPersons, maxTravelTime, minSamples, haltSpeed, vTypes, writeAttributes, edges, aggregate);
    //         }
    //         if (det != nullptr) {
    //             if (first == nullptr) {
    //                 first = det;
    //             }
    //             if (parent != nullptr) {
    //                 parent->setSubData(det);
    //             }
    //             parent = det;
    //         }
    //     }
    //     if (first != nullptr) {
    //         // needs to be delayed because it might call init for the subdata
    //         MSNet::getInstance()->getDetectorControl().add(first, device, frequency, begin);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn create_edge_lane_mean_data(&mut self, id: &str, frequency: SUMOTime, begin: SUMOTime, end: SUMOTime, type_: &str, useLanes: bool, excludeEmpty: &str, withInternal: bool, trackVehicles: bool, detectPersons: i32, maxTravelTime: f64, minSamples: f64, haltSpeed: f64, haltSpeedRel: f64, vTypes: &str, writeAttributes: &str, edges: Vec<Ptr<MSEdge>>, aggregate: AggregateType, device: &str) {
        todo!("port NLDetectorBuilder::createEdgeLaneMeanData")
    }

    /// C++: `double NLDetectorBuilder::getPositionChecking(double, MSLane *, bool, SumoXMLTag, const std::string &)` [protected]
    /// Computes the position to use
    // double
    // NLDetectorBuilder::getPositionChecking(double pos, MSLane* lane, bool friendlyPos,
    //                                        SumoXMLTag tag,
    //                                        const std::string& detid) {
    //     // check whether it is given from the end
    //     if (pos < 0) {
    //         pos += lane->getLength();
    //     }
    //     // check whether it is on the lane
    //     if (pos > lane->getLength()) {
    //         if (friendlyPos) {
    //             pos = lane->getLength();
    //         } else {
    //             throw InvalidArgument("The position of " + toString(tag) + " '" + detid + "' lies beyond the lane's '" + lane->getID() + "' end.");
    //         }
    //     }
    //     if (pos < 0) {
    //         if (friendlyPos) {
    //             pos = 0.;
    //         } else {
    //             throw InvalidArgument("The position of " + toString(tag) + "  '" + detid + "' lies before the lane's '" + lane->getID() + "' begin.");
    //         }
    //     }
    //     return pos;
    // }
    #[allow(unused_variables)]
    pub fn get_position_checking(&mut self, pos: f64, lane: Ptr<MSLane>, friendlyPos: bool, tag: SumoXMLTag, detid: &str) -> f64 {
        todo!("port NLDetectorBuilder::getPositionChecking")
    }

    /// C++: `MSEdge * NLDetectorBuilder::getEdgeChecking(const std::string &, SumoXMLTag, const std::string &)` [protected]
    /// Returns the named edge
    // MSEdge*
    // NLDetectorBuilder::getEdgeChecking(const std::string& edgeID, SumoXMLTag type,
    //                                    const std::string& detid) {
    //     // get and check the lane
    //     MSEdge* edge = MSEdge::dictionary(edgeID);
    //     if (edge == nullptr) {
    //         throw InvalidArgument("The lane with the id '" + edgeID + "' is not known (while building " + toString(type) + " '" + detid + "').");
    //     }
    //     return edge;
    // }
    #[allow(unused_variables)]
    pub fn get_edge_checking(&mut self, edgeID: &str, type_: SumoXMLTag, detid: &str) -> Ptr<MSEdge> {
        todo!("port NLDetectorBuilder::getEdgeChecking")
    }

    /// C++: `MSLane * NLDetectorBuilder::getLaneChecking(const std::string &, SumoXMLTag, const std::string &)`
    /// Returns the named lane
    // MSLane*
    // NLDetectorBuilder::getLaneChecking(const std::string& laneID, SumoXMLTag type,
    //                                    const std::string& detid) {
    //     // get and check the lane
    //     MSLane* lane = MSLane::dictionary(laneID);
    //     if (lane == nullptr) {
    //         throw InvalidArgument("The lane with the id '" + laneID + "' is not known (while building " + toString(type) + " '" + detid + "').");
    //     }
    //     return lane;
    // }
    #[allow(unused_variables)]
    pub fn get_lane_checking(&mut self, laneID: &str, type_: SumoXMLTag, detid: &str) -> Ptr<MSLane> {
        todo!("port NLDetectorBuilder::getLaneChecking")
    }

    /// C++: `void NLDetectorBuilder::checkSampleInterval(SUMOTime, SumoXMLTag, const std::string &)` [protected]
    /// Checks whether the given frequency (sample interval) is valid
    // void
    // NLDetectorBuilder::checkSampleInterval(SUMOTime splInterval, SumoXMLTag type, const std::string& id) {
    //     if (splInterval < 0) {
    //         throw InvalidArgument("Negative sampling frequency (in " + toString(type) + " '" + id + "').");
    //     }
    //     if (splInterval == 0) {
    //         throw InvalidArgument("Sampling frequency must not be zero (in " + toString(type) + " '" + id + "').");
    //     }
    //     checkStepLengthMultiple(splInterval, " (in " + toString(type) + " '" + id + "')");
    // }
    #[allow(unused_variables)]
    pub fn check_sample_interval(&mut self, splInterval: SUMOTime, type_: SumoXMLTag, id: &str) {
        todo!("port NLDetectorBuilder::checkSampleInterval")
    }

    /// C++: `void NLDetectorBuilder::NLDetectorBuilder(const NLDetectorBuilder &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &NLDetectorBuilder) -> Self {
        todo!("port NLDetectorBuilder::NLDetectorBuilder")
    }

    /// C++: `NLDetectorBuilder & NLDetectorBuilder::operator=(const NLDetectorBuilder &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &NLDetectorBuilder) -> &mut NLDetectorBuilder {
        todo!("port NLDetectorBuilder::operator=")
    }

}

