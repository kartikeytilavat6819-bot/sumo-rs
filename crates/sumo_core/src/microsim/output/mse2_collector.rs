//! Port of microsim/output/MSE2Collector.{h,cpp}
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::msg_handler::write_warning;
use utils::common::std_defs::INVALID_DOUBLE;
use utils::geom::{NUMERICAL_EPS, POSITION_EPS};

/// C++ MAX2 (StdDefs.h): (a) > (b) ? (a) : (b)
fn max2(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}
/// C++ MIN2 (StdDefs.h): (a) < (b) ? (a) : (b)
fn min2(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}
/// C++ macro `SIMSTEP`: MSNet::getInstance()->getCurrentTimeStep()
fn sim_step() -> SUMOTime {
    MSNet::get_instance().get_current_time_step()
}
/// C++ `MSPModel::FORWARD` (MSPModel.cpp)
const MSPMODEL_FORWARD: i32 = 1;

/// C++ typedef `std::map<std::string, VehicleInfo *> MSE2Collector::VehicleInfoMap`
pub type MSE2Collector_VehicleInfoMap = BTreeMap<String, Ptr<MSE2Collector_VehicleInfo>>;

/// Internal representation of a jam
/// C++ class `MSE2Collector::JamInfo` (microsim/output/MSE2Collector.h)
pub struct MSE2Collector_JamInfo {
    /// The first standing vehicle
    /// Index into myMoveNotifications (C++ const_iterator)
    pub firstStandingVehicle: usize,
    /// The last standing vehicle
    /// Index into myMoveNotifications (C++ const_iterator)
    pub lastStandingVehicle: usize,
}

impl MSE2Collector_JamInfo {
}

/// An areal detector corresponding to a sequence of consecutive lanes
/// C++ class `MSE2Collector` (microsim/output/MSE2Collector.h)
#[repr(C)]
pub struct MSE2Collector {
    /// C++ base class `MSMoveReminder`
    pub base: MSMoveReminder,
    /// C++ base class `MSDetectorFileOutput`
    pub base2: MSDetectorFileOutput,
    /// Information about how this detector is used
    pub myUsage: DetectorUsage,
    /// name
    pub myName: String,
    /// The detector's lane sequence
    pub myLanes: Vec<String>,
    /// The distances of the lane-beginnings from the detector start-point
    pub myOffsets: Vec<f64>,
    /// The first lane of the detector's lane sequence
    pub myFirstLane: Ptr<MSLane>,
    /// The last lane of the detector's lane sequence
    pub myLastLane: Ptr<MSLane>,
    /// The position the detector starts at on the first lane
    pub myStartPos: f64,
    /// The position the detector ends at on the last lane
    pub myEndPos: f64,
    /// The total detector length
    pub myDetectorLength: f64,
    /// A vehicle must driver slower than this to be counted as a part of a jam
    pub myJamHaltingSpeedThreshold: f64,
    /// A vehicle must be that long beyond myJamHaltingSpeedThreshold to be counted as a part of a jam
    pub myJamHaltingTimeThreshold: SUMOTime,
    /// Two standing vehicles must be closer than this to be counted into the same jam
    pub myJamDistanceThreshold: f64,
    /// List of informations about the vehicles currently on the detector
    pub myVehicleInfos: MSE2Collector_VehicleInfoMap,
    /// Temporal storage for notifications from vehicles that did call the detector's notifyMove() in the last time step.
    pub myMoveNotifications: Vec<Ptr<MSE2Collector_MoveNotificationInfo>>,
    /// Keep track of vehicles that left the detector by a regular move along a junction (not lanechange, teleport, etc.) and should be removed from myVehicleInfos after taking into account their movement. Non-longitudinal exits are processed immediately in notifyLeave()
    pub myLeftVehicles: BTreeSet<String>,
    /// Storage for halting durations of known vehicles (for halting vehicles)
    pub myHaltingVehicleDurations: BTreeMap<String, SUMOTime>,
    /// Storage for halting durations of known vehicles (current interval)
    pub myIntervalHaltingVehicleDurations: BTreeMap<String, SUMOTime>,
    /// Halting durations of ended halts [s]
    pub myPastStandingDurations: Vec<SUMOTime>,
    /// Halting durations of ended halts for the current interval [s]
    pub myPastIntervalStandingDurations: Vec<SUMOTime>,
    /// The number of collected samples [time x vehicle] since the last reset
    pub myVehicleSamples: f64,
    /// The total amount of all time losses [time x vehicle] since the last reset
    pub myTotalTimeLoss: f64,
    /// The sum of collected vehicle speeds [m/s]
    pub mySpeedSum: f64,
    /// The number of started halts [#]
    pub myStartedHalts: i32,
    /// The sum of jam lengths [m]
    pub myJamLengthInMetersSum: f64,
    /// The sum of jam lengths [#veh]
    pub myJamLengthInVehiclesSum: i32,
    /// The current aggregation duration [#steps]
    pub myTimeSamples: i32,
    /// The sum of occupancies [%]
    pub myOccupancySum: f64,
    /// The maximum occupancy [%]
    pub myMaxOccupancy: f64,
    /// The mean jam length [#veh]
    pub myMeanMaxJamInVehicles: i32,
    /// The mean jam length [m]
    pub myMeanMaxJamInMeters: f64,
    /// The max jam length [#veh]
    pub myMaxJamInVehicles: i32,
    /// The max jam length [m]
    pub myMaxJamInMeters: f64,
    /// The mean number of vehicles [#veh]
    pub myMeanVehicleNumber: i32,
    /// The number of vehicles, which have entered the detector since the last reset
    pub myNumberOfEnteredVehicles: i32,
    /// The number of vehicles, present on the detector at the last reset
    pub myNumberOfSeenVehicles: i32,
    /// The number of vehicles, which have left the detector since the last reset
    pub myNumberOfLeftVehicles: i32,
    /// The maximal number of vehicles located on the detector simultaneously since the last reset
    pub myMaxVehicleNumber: i32,
    /// The current vehicle samples
    pub myCurrentVehicleSamples: f64,
    /// The current occupancy
    pub myCurrentOccupancy: f64,
    /// The current mean speed
    pub myCurrentMeanSpeed: f64,
    /// The current mean timeLoss
    pub myCurrentMeanTimeLoss: f64,
    /// The current mean length
    pub myCurrentMeanLength: f64,
    /// The current jam number
    pub myCurrentJamNo: i32,
    /// the current maximum jam length in meters
    pub myCurrentMaxJamLengthInMeters: f64,
    /// The current maximum jam length in vehicles
    pub myCurrentMaxJamLengthInVehicles: i32,
    /// The overall jam length in meters
    pub myCurrentJamLengthInMeters: f64,
    /// The overall jam length in vehicles
    pub myCurrentJamLengthInVehicles: i32,
    /// The overall jam duration in s
    pub myCurrentJamDuration: f64,
    /// The number of started halts in the last step
    pub myCurrentStartedHalts: i32,
    /// The number of halted vehicles [#]
    pub myCurrentHaltingsNumber: i32,
    /// @{
    pub myPreviousMeanOccupancy: f64,
    pub myPreviousMeanSpeed: f64,
    pub myPreviousMeanTimeLoss: f64,
    pub myPreviousMaxJamLengthInMeters: f64,
    pub myPreviousNumberOfSeenVehicles: i32,
    /// stores the overriden (via Traci) number of vehicles on detector
    pub myOverrideVehNumber: i32,
}

impl std::ops::Deref for MSE2Collector {
    type Target = MSMoveReminder;
    fn deref(&self) -> &MSMoveReminder {
        &self.base
    }
}
impl std::ops::DerefMut for MSE2Collector {
    fn deref_mut(&mut self) -> &mut MSMoveReminder {
        &mut self.base
    }
}

impl MSE2Collector {
    /// Builds the fields shared by both constructors (C++ member initialiser lists) with all
    /// other members at their C++ in-class defaults. Errors come from MSDetectorFileOutput.
    fn new_base(id: &str, lane: Ptr<MSLane>, usage: DetectorUsage, name: String, haltingTimeThreshold: SUMOTime, haltingSpeedThreshold: f64, jamDistThreshold: f64, vTypes: &str, nextEdges: &str, detectPersons: i32) -> Result<Self, ProcessError> {
        Ok(Self {
            base: MSMoveReminder::new(id, lane, false),
            base2: MSDetectorFileOutput::new(id, vTypes, nextEdges, detectPersons)?,
            myUsage: usage,
            myName: name,
            myLanes: Vec::new(),
            myOffsets: Vec::new(),
            myFirstLane: Ptr::null(),
            myLastLane: Ptr::null(),
            myStartPos: 0.0,
            myEndPos: 0.0,
            myDetectorLength: 0.0,
            myJamHaltingSpeedThreshold: haltingSpeedThreshold,
            myJamHaltingTimeThreshold: haltingTimeThreshold,
            myJamDistanceThreshold: jamDistThreshold,
            myVehicleInfos: BTreeMap::new(),
            myMoveNotifications: Vec::new(),
            myLeftVehicles: BTreeSet::new(),
            myHaltingVehicleDurations: BTreeMap::new(),
            myIntervalHaltingVehicleDurations: BTreeMap::new(),
            myPastStandingDurations: Vec::new(),
            myPastIntervalStandingDurations: Vec::new(),
            myVehicleSamples: 0.0,
            myTotalTimeLoss: 0.0,
            mySpeedSum: 0.0,
            myStartedHalts: 0,
            myJamLengthInMetersSum: 0.0,
            myJamLengthInVehiclesSum: 0,
            myTimeSamples: 0,
            myOccupancySum: 0.0,
            myMaxOccupancy: 0.0,
            myMeanMaxJamInVehicles: 0,
            myMeanMaxJamInMeters: 0.0,
            myMaxJamInVehicles: 0,
            myMaxJamInMeters: 0.0,
            myMeanVehicleNumber: 0,
            myNumberOfEnteredVehicles: 0,
            myNumberOfSeenVehicles: 0,
            myNumberOfLeftVehicles: 0,
            myMaxVehicleNumber: 0,
            myCurrentVehicleSamples: 0.0,
            myCurrentOccupancy: 0.0,
            myCurrentMeanSpeed: 0.0,
            myCurrentMeanTimeLoss: 0.0,
            myCurrentMeanLength: 0.0,
            myCurrentJamNo: 0,
            myCurrentMaxJamLengthInMeters: 0.0,
            myCurrentMaxJamLengthInVehicles: 0,
            myCurrentJamLengthInMeters: 0.0,
            myCurrentJamLengthInVehicles: 0,
            myCurrentJamDuration: 0.0,
            myCurrentStartedHalts: 0,
            myCurrentHaltingsNumber: 0,
            myPreviousMeanOccupancy: 0.0,
            myPreviousMeanSpeed: 0.0,
            myPreviousMeanTimeLoss: 0.0,
            myPreviousMaxJamLengthInMeters: 0.0,
            myPreviousNumberOfSeenVehicles: 0,
            myOverrideVehNumber: -1,
        })
    }

    /// C++: `void MSE2Collector::MSE2Collector(const std::string &, DetectorUsage, MSLane *, double, double, double, SUMOTime, double, double, const std::string, const std::string &, const std::string &, int)`
    /// Constructor with given end position and detector length
    // MSE2Collector::MSE2Collector(const std::string& id,
    //                              DetectorUsage usage, MSLane* lane, double startPos, double endPos, double length,
    //                              SUMOTime haltingTimeThreshold, double haltingSpeedThreshold, double jamDistThreshold,
    //                              const std::string name, const std::string& vTypes,
    //                              const std::string& nextEdges,
    //                              int detectPersons) :
    //     MSMoveReminder(id, lane, false),
    //     MSDetectorFileOutput(id, vTypes, nextEdges, detectPersons),
    //     myUsage(usage),
    //     myName(name),
    //     myJamHaltingSpeedThreshold(haltingSpeedThreshold),
    //     myJamHaltingTimeThreshold(haltingTimeThreshold),
    //     myJamDistanceThreshold(jamDistThreshold) {
    //
    // #ifdef DEBUG_E2_CONSTRUCTOR
    //     if (DEBUG_COND) {
    //         std::cout << "\n" << "Creating MSE2Collector " << id
    //                   << " with lane = " << lane->getID()
    //                   << " startPos = " << startPos
    //                   << " endPos = " << endPos
    //                   << " length = " << length
    //                   << " haltingTimeThreshold = " << haltingTimeThreshold
    //                   << " haltingSpeedThreshold = " << haltingSpeedThreshold
    //                   << " jamDistThreshold = " << jamDistThreshold
    //                   << std::endl;
    //     }
    // #endif
    //
    //     assert(lane != 0);
    //
    //     // check that exactly one of length, startPos, endPos is invalid
    //     bool lengthInvalid = length == std::numeric_limits<double>::max() || length <= 0;
    //     bool endPosInvalid = endPos == std::numeric_limits<double>::max();
    //     bool posInvalid = startPos == std::numeric_limits<double>::max();
    //
    //     // check and normalize positions (assure positive values for pos and endPos, snap to lane-ends)
    //     if (lengthInvalid) {
    //         // assume that the detector is only located on a single lane
    //         if (posInvalid) {
    //             WRITE_WARNING(TL("No valid detector length and start position given. Assuming startPos = 0 and length = end position"));
    //             startPos = 0;
    //         }
    //         if (endPosInvalid) {
    //             WRITE_WARNING(TL("No valid detector length and end position given. Assuming endPos = lane length and length = endPos-startPos"));
    //             endPos = lane->getLength();
    //         }
    //         endPos = endPos < 0 ? lane->getLength() + endPos : endPos;
    //         startPos = startPos < 0 ? lane->getLength() + startPos : startPos;
    //         bool valid = endPos <= lane->getLength() && 0 <= startPos && startPos < endPos;
    //         if (!valid) {
    //             throw InvalidArgument("Error in specification for E2Detector '" + id + "'. Positional argument is malformed. 0 <= pos < endPos <= lane.getLength() is required.");
    //         }
    //         // snap detector ends to lane ends
    //         endPos = snap(endPos, lane->getLength(), POSITION_EPS);
    //         startPos = snap(startPos, 0., POSITION_EPS);
    //         length = endPos - startPos;
    //     } else if (posInvalid) {
    //         // endPosInvalid == false
    //         endPos = endPos < 0 ? lane->getLength() + endPos : endPos;
    //         endPos = snap(endPos, lane->getLength(), POSITION_EPS);
    //     } else {
    //         // posInvalid == false
    //         startPos = startPos < 0 ? lane->getLength() + startPos : startPos;
    //         startPos = snap(startPos, 0., POSITION_EPS);
    //     }
    //
    //     myStartPos = startPos;
    //     myEndPos = endPos;
    //
    //     std::vector<MSLane*> lanes;
    //     if (posInvalid) {
    //         lanes = selectLanes(lane, length, "bw");
    //     } else if (endPosInvalid) {
    //         lanes = selectLanes(lane, length, "fw");
    //     } else {
    //         // assuming detector is only located at a single lane
    //         lanes.push_back(lane);
    //     }
    //
    //     initAuxiliaries(lanes);
    //     checkPositioning(endPosInvalid, length);
    //     addDetectorToLanes(lanes);
    // }
    #[allow(unused_variables)]
    pub fn new(id: &str, usage: DetectorUsage, lane: Ptr<MSLane>, startPos: f64, endPos: f64, length: f64, haltingTimeThreshold: SUMOTime, haltingSpeedThreshold: f64, jamDistThreshold: f64, name: String, vTypes: &str, nextEdges: &str, detectPersons: i32) -> Result<Box<Self>, ProcessError> {
        let mut startPos = startPos;
        let mut endPos = endPos;
        let mut length = length;
        let mut this = Box::new(Self::new_base(id, lane, usage, name, haltingTimeThreshold, haltingSpeedThreshold, jamDistThreshold, vTypes, nextEdges, detectPersons)?);
        // check that exactly one of length, startPos, endPos is invalid
        let lengthInvalid = length == f64::MAX || length <= 0.0;
        let endPosInvalid = endPos == f64::MAX;
        let posInvalid = startPos == f64::MAX;
        // check and normalize positions (assure positive values for pos and endPos, snap to lane-ends)
        if lengthInvalid {
            // assume that the detector is only located on a single lane
            if posInvalid {
                write_warning("No valid detector length and start position given. Assuming startPos = 0 and length = end position");
                startPos = 0.0;
            }
            if endPosInvalid {
                write_warning("No valid detector length and end position given. Assuming endPos = lane length and length = endPos-startPos");
                endPos = lane.get_length();
            }
            endPos = if endPos < 0.0 { lane.get_length() + endPos } else { endPos };
            startPos = if startPos < 0.0 { lane.get_length() + startPos } else { startPos };
            let valid = endPos <= lane.get_length() && 0.0 <= startPos && startPos < endPos;
            if !valid {
                return Err(ProcessError::new(format!("Error in specification for E2Detector '{}'. Positional argument is malformed. 0 <= pos < endPos <= lane.getLength() is required.", id)));
            }
            // snap detector ends to lane ends
            endPos = Self::snap(endPos, lane.get_length(), POSITION_EPS);
            startPos = Self::snap(startPos, 0.0, POSITION_EPS);
            length = endPos - startPos;
        } else if posInvalid {
            // endPosInvalid == false
            endPos = if endPos < 0.0 { lane.get_length() + endPos } else { endPos };
            endPos = Self::snap(endPos, lane.get_length(), POSITION_EPS);
        } else {
            // posInvalid == false
            startPos = if startPos < 0.0 { lane.get_length() + startPos } else { startPos };
            startPos = Self::snap(startPos, 0.0, POSITION_EPS);
        }
        this.myStartPos = startPos;
        this.myEndPos = endPos;
        let mut lanes: Vec<Ptr<MSLane>> = if posInvalid {
            this.select_lanes(lane, length, "bw".to_string())
        } else if endPosInvalid {
            this.select_lanes(lane, length, "fw".to_string())
        } else {
            // assuming detector is only located at a single lane
            vec![lane]
        };
        this.init_auxiliaries(&mut lanes)?;
        this.check_positioning(endPosInvalid, length);
        this.add_detector_to_lanes(&mut lanes);
        Ok(this)
    }

    /// C++: `void MSE2Collector::MSE2Collector(const std::string &, DetectorUsage, std::vector<MSLane *>, double, double, SUMOTime, double, double, const std::string, const std::string &, const std::string &, int)`
    /// Constructor with a sequence of lanes and given start and end position on the first and last lanes
    // MSE2Collector::MSE2Collector(const std::string& id,
    //                              DetectorUsage usage, std::vector<MSLane*> lanes, double startPos, double endPos,
    //                              SUMOTime haltingTimeThreshold, double haltingSpeedThreshold, double jamDistThreshold,
    //                              const std::string name, const std::string& vTypes,
    //                              const std::string& nextEdges,
    //                              int detectPersons) :
    //     MSMoveReminder(id, lanes[lanes.size() - 1], false), // assure that lanes.size() > 0 at caller side!!!
    //     MSDetectorFileOutput(id, vTypes, nextEdges, detectPersons),
    //     myUsage(usage),
    //     myName(name),
    //     myFirstLane(lanes[0]),
    //     myLastLane(lanes[lanes.size() - 1]),
    //     myStartPos(startPos),
    //     myEndPos(endPos),
    //     myJamHaltingSpeedThreshold(haltingSpeedThreshold),
    //     myJamHaltingTimeThreshold(haltingTimeThreshold),
    //     myJamDistanceThreshold(jamDistThreshold) {
    //
    //     for (std::vector<MSLane*>::const_iterator i = lanes.begin(); i != lanes.end(); ++i) {
    //         assert((*i) != 0);
    //     }
    //
    // #ifdef DEBUG_E2_CONSTRUCTOR
    //     if (DEBUG_COND) {
    //         std::cout << "\n" << "Creating MSE2Collector " << id
    //                   << " with endLane = " << myLastLane->getID()
    //                   << " endPos = " << endPos
    //                   << " startLane = " << myFirstLane->getID()
    //                   << " startPos = " << startPos
    //                   << " haltingTimeThreshold = " << haltingTimeThreshold
    //                   << " haltingSpeedThreshold = " << haltingSpeedThreshold
    //                   << " jamDistThreshold = " << jamDistThreshold
    //                   << std::endl;
    //     }
    // #endif
    //
    //     myStartPos = myStartPos < 0 ? lanes[0]->getLength() + myStartPos : myStartPos;
    //     myEndPos = myEndPos < 0 ? lanes[lanes.size() - 1]->getLength() + myEndPos : myEndPos;
    //
    //     if (myStartPos < POSITION_EPS) {
    //         myStartPos = 0;
    //     }
    //     if (myEndPos > lanes[lanes.size() - 1]->getLength() - POSITION_EPS) {
    //         myEndPos = lanes[lanes.size() - 1]->getLength();
    //     }
    //
    //
    //     initAuxiliaries(lanes);
    //     checkPositioning();
    //     addDetectorToLanes(lanes);
    // }
    #[allow(unused_variables)]
    pub fn new_2(id: &str, usage: DetectorUsage, lanes: Vec<Ptr<MSLane>>, startPos: f64, endPos: f64, haltingTimeThreshold: SUMOTime, haltingSpeedThreshold: f64, jamDistThreshold: f64, name: String, vTypes: &str, nextEdges: &str, detectPersons: i32) -> Result<Box<Self>, ProcessError> {
        let mut lanes = lanes;
        let last = lanes.len() - 1;
        let mut this = Box::new(Self::new_base(id, lanes[last], usage, name, haltingTimeThreshold, haltingSpeedThreshold, jamDistThreshold, vTypes, nextEdges, detectPersons)?);
        this.myFirstLane = lanes[0];
        this.myLastLane = lanes[last];
        this.myStartPos = startPos;
        this.myEndPos = endPos;
        this.myStartPos = if this.myStartPos < 0.0 { lanes[0].get_length() + this.myStartPos } else { this.myStartPos };
        this.myEndPos = if this.myEndPos < 0.0 { lanes[last].get_length() + this.myEndPos } else { this.myEndPos };
        if this.myStartPos < POSITION_EPS {
            this.myStartPos = 0.0;
        }
        if this.myEndPos > lanes[last].get_length() - POSITION_EPS {
            this.myEndPos = lanes[last].get_length();
        }
        this.init_auxiliaries(&mut lanes)?;
        this.check_positioning(false, 0.0);
        this.add_detector_to_lanes(&mut lanes);
        Ok(this)
    }

    /// C++: `void MSE2Collector::~MSE2Collector() noexcept` [virtual]
    /// Destructor
    // MSE2Collector::~MSE2Collector() {
    //     // clear move notifications
    //     clearState(SUMOTime_MAX);
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        self.clear_state(SUMOTIME_MAX);
    }

    /// C++: `DetectorUsage MSE2Collector::getUsageType() const` [const, virtual]
    /// Returns the detector's usage type
    // virtual DetectorUsage getUsageType() const {
    //         return myUsage;
    //     }
    #[allow(unused_variables)]
    pub fn get_usage_type(&self) -> DetectorUsage {
        self.myUsage
    }

    /// C++: `bool MSE2Collector::notifyMove(SUMOTrafficObject &, double, double, double)` [virtual]
    /// Adds/removes vehicles from the list of vehicles to regard
    // bool
    // MSE2Collector::notifyMove(SUMOTrafficObject& veh, double oldPos,
    //                           double newPos, double newSpeed) {
    //
    //     if (myDetectPersons > (int)PersonMode::WALK && !veh.isPerson()) {
    //         bool keep = false;
    //         MSBaseVehicle& v = dynamic_cast<MSBaseVehicle&>(veh);
    //         for (MSTransportable* p : v.getPersons()) {
    //             keep = notifyMove(*p, oldPos, newPos, newSpeed);
    //         }
    //         return keep;
    //     }
    // #ifdef HAVE_FOX
    //     ScopedLocker<> lock(myNotificationMutex, MSGlobals::gNumSimThreads > 1);
    // #endif
    //     VehicleInfoMap::iterator vi = myVehicleInfos.find(veh.getID());
    //     if (vi == myVehicleInfos.end()) {
    //         const std::string objectType = veh.isPerson() ? "Person" : "Vehicle";
    //         if (myNextEdges.size() > 0) {
    //             WRITE_WARNING(objectType + " '" + veh.getID() + "' appeared inside detector '" + getID() + "' after previously being filtered out. time=" + time2string(SIMSTEP) + ".");
    //         } else {
    //             WRITE_WARNING(objectType + " '" + veh.getID() + "' suddenly appeared inside detector '" + getID() + "'. time=" + time2string(SIMSTEP) + ".");
    //         }
    //         return false;
    //     }
    //
    //     const std::string& vehID = veh.getID();
    //     VehicleInfo& vehInfo = *(vi->second);
    //
    //     // position relative to the detector start
    //     double relPos = vehInfo.entryOffset + newPos;
    //
    //     // update current distance to the detector end
    //     vehInfo.distToDetectorEnd = myDetectorLength - relPos;
    //
    // #ifdef DEBUG_E2_NOTIFY_MOVE
    //     if (DEBUG_COND) {
    //         std::cout << "\n" << SIMTIME
    //                   << " MSE2Collector::notifyMove() (detID = " << myID << " on lane '" << myLane->getID() << "')"
    //                   << " called by vehicle '" << vehID << "'"
    //                   << " at relative position " << relPos
    //                   << ", distToDetectorEnd = " << vehInfo.distToDetectorEnd << std::endl;
    //     }
    // #endif
    //
    //     // Check whether vehicle has reached the detector begin
    //     if (relPos <= 0) {
    //         // detector not yet reached, request being informed further
    // #ifdef DEBUG_E2_NOTIFY_MOVE
    //         if (DEBUG_COND) {
    //             std::cout << "Vehicle has not yet reached the detector start position." << std::endl;
    //         }
    // #endif
    //         return true;
    //     } else if (!vehInfo.hasEntered) {
    //         vehInfo.hasEntered = true;
    //         myNumberOfEnteredVehicles++;
    //         myNumberOfSeenVehicles++;
    //     }
    //
    //
    //     // determine whether vehicle has moved beyond the detector's end
    //     bool vehPassedDetectorEnd = - vehInfo.exitOffset <= newPos - veh.getVehicleType().getLength();
    //
    //     // determine whether vehicle has been on the detector at all
    //     bool vehicleEnteredLaneAfterDetector = vehPassedDetectorEnd && (-vehInfo.exitOffset <= oldPos - veh.getVehicleType().getLength());
    //     // ... if not, dont create any notification at all
    //     if (vehicleEnteredLaneAfterDetector) {
    // #ifdef DEBUG_E2_NOTIFY_MOVE
    //         if (DEBUG_COND) {
    //             std::cout << "Vehicle entered lane behind detector." << std::endl;
    //         }
    // #endif
    //     } else {
    //         myMoveNotifications.push_back(makeMoveNotification(veh, oldPos, newPos, newSpeed, vehInfo));
    //     }
    //
    //
    //     if (vehPassedDetectorEnd) {
    // #ifdef DEBUG_E2_NOTIFY_MOVE
    //         if (DEBUG_COND) {
    //             std::cout << "Vehicle has left the detector longitudinally." << std::endl;
    //         }
    // #endif
    //         // Vehicle is beyond the detector, unsubscribe and register removal from myVehicleInfos
    //         myLeftVehicles.insert(vehID);
    //         return false;
    //     } else {
    //         // Receive further notifications
    //         return true;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn notify_move(&mut self, veh: &mut SUMOTrafficObject, oldPos: f64, newPos: f64, newSpeed: f64) -> bool {
        if self.base2.myDetectPersons > PersonMode::WALK as i32 && !veh.is_person() {
            let mut keep = false;
            let v: Ptr<MSBaseVehicle> = Ptr::from_mut(veh).cast::<MSBaseVehicle>();
            for p in v.get_persons().clone() {
                keep = self.notify_move(p.m(), oldPos, newPos, newSpeed);
            }
            return keep;
        }
        let vehID = veh.get_id().to_string();
        let vi: Ptr<MSE2Collector_VehicleInfo> = match self.myVehicleInfos.get(&vehID) {
            Some(vi) => *vi,
            None => {
                let objectType = if veh.is_person() { "Person" } else { "Vehicle" };
                if !self.base2.myNextEdges.is_empty() {
                    write_warning(&format!("{} '{}' appeared inside detector '{}' after previously being filtered out. time={}.", objectType, vehID, self.base2.get_id(), time2string(sim_step())));
                } else {
                    write_warning(&format!("{} '{}' suddenly appeared inside detector '{}'. time={}.", objectType, vehID, self.base2.get_id(), time2string(sim_step())));
                }
                return false;
            }
        };
        let vehInfo = vi.m();
        // position relative to the detector start
        let relPos = vehInfo.entryOffset + newPos;
        // update current distance to the detector end
        vehInfo.distToDetectorEnd = self.myDetectorLength - relPos;
        // Check whether vehicle has reached the detector begin
        if relPos <= 0.0 {
            // detector not yet reached, request being informed further
            return true;
        } else if !vehInfo.hasEntered {
            vehInfo.hasEntered = true;
            self.myNumberOfEnteredVehicles += 1;
            self.myNumberOfSeenVehicles += 1;
        }
        // determine whether vehicle has moved beyond the detector's end
        let vehPassedDetectorEnd = -vehInfo.exitOffset <= newPos - veh.get_vehicle_type().get_length();
        // determine whether vehicle has been on the detector at all
        let vehicleEnteredLaneAfterDetector = vehPassedDetectorEnd && (-vehInfo.exitOffset <= oldPos - veh.get_vehicle_type().get_length());
        // ... if not, dont create any notification at all
        if !vehicleEnteredLaneAfterDetector {
            let mni = self.make_move_notification(veh, oldPos, newPos, newSpeed, vehInfo);
            self.myMoveNotifications.push(mni);
        }
        if vehPassedDetectorEnd {
            // Vehicle is beyond the detector, unsubscribe and register removal from myVehicleInfos
            self.myLeftVehicles.insert(vehID);
            false
        } else {
            // Receive further notifications
            true
        }
    }

    /// C++: `bool MSE2Collector::notifyLeave(SUMOTrafficObject &, double, MSMoveReminder::Notification, const MSLane *)` [virtual]
    /// Removes a known vehicle due to its lane-change
    // bool
    // MSE2Collector::notifyLeave(SUMOTrafficObject& veh, double /*lastPos*/, MSMoveReminder::Notification reason, const MSLane* enteredLane) {
    // #ifdef DEBUG_E2_NOTIFY_ENTER_AND_LEAVE
    //     if (DEBUG_COND) {
    //         std::cout << "\n" << SIMTIME << " notifyLeave() (detID = " << myID << " on lane '" << myLane->getID() << "')"
    //                   << "called by vehicle '" << veh.getID() << "'" << std::endl;
    //     }
    // #endif
    //
    // #ifdef HAVE_FOX
    //     ScopedLocker<> lock(myNotificationMutex, MSGlobals::gNumSimThreads > 1);
    // #endif
    //     if (reason == MSMoveReminder::NOTIFICATION_JUNCTION && !veh.isPerson()) {
    //         // vehicle left lane via junction, unsubscription and registering in myLeftVehicles when
    //         // moving beyond the detector end is controlled in notifyMove.
    // #ifdef DEBUG_E2_NOTIFY_ENTER_AND_LEAVE
    //         if (DEBUG_COND) {
    //             std::cout << SIMTIME << " Left longitudinally (along junction) -> keep subscription [handle exit in notifyMove()]" << std::endl;
    //         }
    // #endif
    //
    //         if (enteredLane == nullptr || std::find(myLanes.begin(), myLanes.end(), enteredLane->getID()) == myLanes.end()) {
    //             // Entered lane is not part of the detector
    //             VehicleInfoMap::iterator vi = myVehicleInfos.find(veh.getID());
    //             // Determine exit offset, where vehicle left the detector
    //             double exitOffset = vi->second->entryOffset - myOffsets[vi->second->currentOffsetIndex] - vi->second->currentLane->getLength();
    //             vi->second->exitOffset = MAX2(vi->second->exitOffset, exitOffset);
    // #ifdef DEBUG_E2_NOTIFY_ENTER_AND_LEAVE
    //             if (DEBUG_COND) {
    //                 std::cout << SIMTIME << " Vehicle '" << veh.getID() << "' leaves the detector. Exit offset = " << vi->second->exitOffset << std::endl;
    //             }
    // #endif
    //         }
    //
    //         return true;
    //     } else {
    //         VehicleInfoMap::iterator vi = myVehicleInfos.find(veh.getID());
    //         if (vi != myVehicleInfos.end()) {
    //             // erase vehicle, which leaves in a non-longitudinal way, immediately
    //             if (vi->second->hasEntered) {
    //                 myNumberOfLeftVehicles++;
    //             }
    //             delete vi->second;
    //             myVehicleInfos.erase(vi);
    // #ifdef DEBUG_E2_NOTIFY_ENTER_AND_LEAVE
    //             if (DEBUG_COND) {
    //                 std::cout << SIMTIME << " Left non-longitudinally (lanechange, teleport, parking, etc) -> discard subscription" << std::endl;
    //             }
    // #endif
    //         } else {
    //             assert(veh.isPerson() || myDetectPersons > (int)PersonMode::WALK);
    //         }
    //         return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn notify_leave(&mut self, veh: &mut SUMOTrafficObject, lastPos: f64, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        if reason == MSMoveReminder_Notification::NOTIFICATION_JUNCTION && !veh.is_person() {
            // vehicle left lane via junction, unsubscription and registering in myLeftVehicles when
            // moving beyond the detector end is controlled in notifyMove.
            if enteredLane.is_null() || !self.myLanes.iter().any(|l| l == enteredLane.get_id()) {
                // Entered lane is not part of the detector
                let vehID = veh.get_id().to_string();
                let vi = self.myVehicleInfos[&vehID];
                let vi = vi.m();
                // Determine exit offset, where vehicle left the detector
                let exitOffset = vi.entryOffset - self.myOffsets[vi.currentOffsetIndex] - vi.currentLane.get_length();
                vi.exitOffset = max2(vi.exitOffset, exitOffset);
            }
            true
        } else {
            let vehID = veh.get_id().to_string();
            if let Some(vi) = self.myVehicleInfos.get(&vehID).copied() {
                // erase vehicle, which leaves in a non-longitudinal way, immediately
                if vi.hasEntered {
                    self.myNumberOfLeftVehicles += 1;
                }
                drop(unsafe { Box::from_raw(vi.as_raw()) });
                self.myVehicleInfos.remove(&vehID);
            } else {
                debug_assert!(veh.is_person() || self.base2.myDetectPersons > PersonMode::WALK as i32);
            }
            false
        }
    }

    /// C++: `bool MSE2Collector::notifyEnter(SUMOTrafficObject &, MSMoveReminder::Notification, const MSLane *)` [virtual]
    /// Adds the vehicle to known vehicles if not beyond the dector
    // bool
    // MSE2Collector::notifyEnter(SUMOTrafficObject& veh, MSMoveReminder::Notification reason, const MSLane* enteredLane) {
    // #ifdef DEBUG_E2_NOTIFY_ENTER_AND_LEAVE
    //     if (DEBUG_COND) {
    //         std::cout << std::endl << SIMTIME << " notifyEnter() (detID = " << myID << " on lane '" << myLane->getID() << "')"
    //                   << " called by vehicle '" << veh.getID()
    //                   << "' entering lane '" << (enteredLane != 0 ? enteredLane->getID() : "NULL") << "'" << std::endl;
    //     }
    // #endif
    //     // notifyEnter() should only be called for lanes of the detector
    //     assert(std::find(myLanes.begin(), myLanes.end(), enteredLane->getID()) != myLanes.end());
    //     assert(veh.getLane() == enteredLane || !veh.isVehicle());
    //
    //     // vehicles must be kept if the "inductionloop" wants to detect passeengers
    //     if (!vehicleApplies(veh) && (veh.isPerson() || myDetectPersons <= (int)PersonMode::WALK)) {
    //         // That's not my type...
    //         return false;
    //     }
    //     if (myDetectPersons > (int)PersonMode::WALK && !veh.isPerson()) {
    //         bool keep = false;
    //         MSBaseVehicle& v = dynamic_cast<MSBaseVehicle&>(veh);
    //         for (MSTransportable* p : v.getPersons()) {
    //             keep = notifyEnter(*p, reason, enteredLane);
    //         }
    //         return keep;
    //     }
    //
    //     // determine whether the vehicle entered the lane behind the detector end
    //     // e.g. due to lane change manoeuver
    //     if (reason != NOTIFICATION_JUNCTION) {
    //         const double vehBackPos = veh.getBackPositionOnLane(enteredLane);
    //         bool vehEnteredBehindDetectorEnd = (enteredLane == myLastLane) && myEndPos <= vehBackPos;
    //         if (vehEnteredBehindDetectorEnd) {
    //             // this vehicle cannot influence detector readings, do not subscribe
    //             // to move notifications
    // #ifdef DEBUG_E2_NOTIFY_ENTER_AND_LEAVE
    //             if (DEBUG_COND) {
    //                 std::cout << "Vehicle entered the lane behind the detector, ignoring it." << std::endl;
    //                 std::cout << "(myEndPos = " << this->myEndPos << ", veh.getBackPositionOnLane() = " << vehBackPos << ")" << std::endl;
    //             }
    // #endif
    //             return false;
    //         }
    //     }
    //
    // #ifdef DEBUG_E2_NOTIFY_ENTER_AND_LEAVE
    //     if (DEBUG_COND) {
    //         if (veh.isVehicle() && !dynamic_cast<SUMOVehicle&>(veh).isOnRoad()) {
    //             // Vehicle is teleporting over the edge
    //             std::cout << "Vehicle is off road (teleporting over edge)..." << std::endl;
    //         }
    //     }
    // #endif
    //
    // #ifdef HAVE_FOX
    //     ScopedLocker<> lock(myNotificationMutex, MSGlobals::gNumSimThreads > 1);
    // #endif
    //     const std::string& vehID = veh.getID();
    //     VehicleInfoMap::iterator vi = myVehicleInfos.find(vehID);
    //     if (vi != myVehicleInfos.end()) {
    //         // Register move current offset to the next lane
    //         if (vi->second->currentLane != enteredLane) {
    //             vi->second->currentOffsetIndex++;
    //             vi->second->currentLane = enteredLane;
    //         }
    //
    // #ifdef DEBUG_E2_NOTIFY_ENTER_AND_LEAVE
    //         if (DEBUG_COND) {
    //             std::cout << SIMTIME << " Vehicle '" << veh.getID() << "' on lane '" << veh.getLane()->getID()
    //                       << "' already known. No new VehicleInfo is created.\n"
    //                       << "enteredLane = " << enteredLane->getID() << "\nmyLanes[vi->offset] = " << myLanes[vi->second->currentOffsetIndex]
    //                       << std::endl;
    //         }
    // #endif
    //         assert(myLanes[vi->second->currentOffsetIndex] == enteredLane->getID());
    //
    //         // but don't add a second subscription for another lane
    //         return false;
    //     }
    //
    //
    //
    // #ifdef DEBUG_E2_NOTIFY_ENTER_AND_LEAVE
    //     if (DEBUG_COND) {
    //         std::cout << SIMTIME << " Adding VehicleInfo for vehicle '" << veh.getID() << "'." << std::endl;
    //     }
    // #endif
    //
    //     // Add vehicle info
    //     myVehicleInfos.insert(std::make_pair(vehID, makeVehicleInfo(veh, enteredLane)));
    //     // Subscribe to vehicle's movement notifications
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn notify_enter(&mut self, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        // vehicles must be kept if the "inductionloop" wants to detect passeengers
        if !self.base2.vehicle_applies(veh) && (veh.is_person() || self.base2.myDetectPersons <= PersonMode::WALK as i32) {
            // That's not my type...
            return false;
        }
        if self.base2.myDetectPersons > PersonMode::WALK as i32 && !veh.is_person() {
            let mut keep = false;
            let v: Ptr<MSBaseVehicle> = Ptr::from_mut(veh).cast::<MSBaseVehicle>();
            for p in v.get_persons().clone() {
                keep = self.notify_enter(p.m(), reason, enteredLane);
            }
            return keep;
        }
        // determine whether the vehicle entered the lane behind the detector end
        // e.g. due to lane change manoeuver
        if reason != MSMoveReminder_Notification::NOTIFICATION_JUNCTION {
            let vehBackPos = veh.get_back_position_on_lane(enteredLane);
            let vehEnteredBehindDetectorEnd = enteredLane == self.myLastLane && self.myEndPos <= vehBackPos;
            if vehEnteredBehindDetectorEnd {
                // this vehicle cannot influence detector readings, do not subscribe
                // to move notifications
                return false;
            }
        }
        let vehID = veh.get_id().to_string();
        if let Some(vi) = self.myVehicleInfos.get(&vehID).copied() {
            // Register move current offset to the next lane
            if vi.currentLane != enteredLane {
                let vi = vi.m();
                vi.currentOffsetIndex += 1;
                vi.currentLane = enteredLane;
            }
            // but don't add a second subscription for another lane
            return false;
        }
        // Add vehicle info
        let vi = self.make_vehicle_info(veh, enteredLane);
        self.myVehicleInfos.insert(vehID, vi);
        // Subscribe to vehicle's movement notifications
        true
    }

    /// C++: `void MSE2Collector::detectorUpdate(const SUMOTime)` [virtual]
    /// Computes the detector values in each time step
    // void
    // MSE2Collector::detectorUpdate(const SUMOTime /* step */) {
    //
    //     if (myDetectPersons != (int)PersonMode::NONE) {
    //         if (myLanes.size() > 1) {
    //             /// code is more complicated because we have to make persons with
    //             //dir=BACKWARD send a virtual forward-lane-sequence
    //             throw ProcessError(TL("Multi-lane e2Detector does not support detecting persons yet"));
    //         }
    //         for (MSLane* lane : getLanes()) {
    //             if (lane->hasPedestrians()) {
    //                 for (MSTransportable* p : myLane->getEdge().getPersons()) {
    //                     if (p->getLane() == lane && vehicleApplies(*p)) {
    //                         notifyMovePerson(p, p->getDirection(), p->getPositionOnLane());
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //
    // #ifdef DEBUG_E2_DETECTOR_UPDATE
    //     if (DEBUG_COND) {
    //         std::cout << "\n" << SIMTIME << " detectorUpdate() for detector '" << myID << "'"
    //                   << "\nmyCurrentMeanSpeed = " << myCurrentMeanSpeed
    //                   << "\nmyCurrentMeanLength = " << myCurrentMeanLength
    //                   << "\nmyNumberOfEnteredVehicles = " << myNumberOfEnteredVehicles
    //                   << "\nmyNumberOfLeftVehicles = " << myNumberOfLeftVehicles
    //                   << "\nmyNumberOfSeenVehicles = " << myNumberOfSeenVehicles
    //                   << std::endl;
    //     }
    // #endif
    //
    // // sort myMoveNotifications (required for jam processing) ascendingly according to vehicle's distance to the detector end
    // // (min = myMoveNotifications[0].distToDetectorEnd)
    //     std::sort(myMoveNotifications.begin(), myMoveNotifications.end(), compareMoveNotification);
    //
    //     // reset values concerning current time step (these are updated in integrateMoveNotification() and aggregateOutputValues())
    //     myCurrentVehicleSamples = 0;
    //     myCurrentMeanSpeed = 0;
    //     myCurrentMeanLength = 0;
    //     myCurrentStartedHalts = 0;
    //     myCurrentHaltingsNumber = 0;
    //
    //     JamInfo* currentJam = nullptr;
    //     std::vector<JamInfo*> jams;
    //     std::map<std::string, SUMOTime> haltingVehicles;
    //     std::map<std::string, SUMOTime> intervalHaltingVehicles;
    //
    //     // go through the list of vehicles positioned on the detector
    //     for (std::vector<MoveNotificationInfo*>::iterator i = myMoveNotifications.begin(); i != myMoveNotifications.end(); ++i) {
    //         // The ID of the vehicle that has sent this notification in the last step
    //         const std::string& vehID = (*i)->id;
    //         VehicleInfoMap::iterator vi = myVehicleInfos.find(vehID);
    //
    //         if (vi == myVehicleInfos.end()) {
    //             // The vehicle has already left the detector by lanechange, teleport, etc. (not longitudinal)
    //             integrateMoveNotification(nullptr, *i);
    //         } else {
    //             // Add move notification infos to detector values and VehicleInfo
    //             integrateMoveNotification(vi->second, *i);
    //         }
    //         // construct jam structure
    //         bool isInJam = checkJam(i, haltingVehicles, intervalHaltingVehicles);
    //         buildJam(isInJam, i, currentJam, jams);
    //     }
    //
    //     // extract some aggregated values from the jam structure
    //     processJams(jams, currentJam);
    //
    //     // Aggregate and normalize values for the detector output
    //     aggregateOutputValues();
    //
    //     // save information about halting vehicles
    //     myHaltingVehicleDurations = haltingVehicles;
    //     myIntervalHaltingVehicleDurations = intervalHaltingVehicles;
    //
    // #ifdef DEBUG_E2_DETECTOR_UPDATE
    //     if (DEBUG_COND) {
    //         std::cout << "\n" << SIMTIME << " Current lanes for vehicles still on or approaching the detector:" << std::endl;
    //     }
    // #endif
    // // update current and entered lanes for remaining vehicles
    //     VehicleInfoMap::iterator iv;
    //     for (iv = myVehicleInfos.begin(); iv != myVehicleInfos.end(); ++iv) {
    // #ifdef DEBUG_E2_DETECTOR_UPDATE
    //         if (DEBUG_COND) {
    //             std::cout << " Vehicle '" << iv->second->id << "'" << ": '"
    //                       << iv->second->currentLane->getID() << "'"
    //                       << std::endl;
    //         }
    // #endif
    //     }
    //
    // #ifdef DEBUG_E2_DETECTOR_UPDATE
    //     if (DEBUG_COND) {
    //         std::cout << SIMTIME << " Discarding vehicles that have left the detector:" << std::endl;
    //     }
    // #endif
    // // Remove the vehicles that have left the detector
    //     std::set<std::string>::const_iterator i;
    //     for (i = myLeftVehicles.begin(); i != myLeftVehicles.end(); ++i) {
    //         VehicleInfoMap::iterator j = myVehicleInfos.find(*i);
    //         delete j->second;
    //         myVehicleInfos.erase(*i);
    //         myNumberOfLeftVehicles++;
    // #ifdef DEBUG_E2_DETECTOR_UPDATE
    //         if (DEBUG_COND) {
    //             std::cout << "Erased vehicle '" << *i << "'" << std::endl;
    //         }
    // #endif
    //     }
    //     myLeftVehicles.clear();
    //
    //     // reset move notifications
    //     for (std::vector<MoveNotificationInfo*>::iterator j = myMoveNotifications.begin(); j != myMoveNotifications.end(); ++j) {
    //         delete *j;
    //     }
    //     myMoveNotifications.clear();
    // }
    #[allow(unused_variables)]
    pub fn detector_update(&mut self, step: SUMOTime) {
        if self.base2.myDetectPersons != PersonMode::NONE as i32 {
            if self.myLanes.len() > 1 {
                // shortcut: C++ throws ProcessError here; this void signature panics instead, change to Result when callers can propagate it
                panic!("Multi-lane e2Detector does not support detecting persons yet");
            }
            for lane in self.get_lanes() {
                if lane.has_pedestrians() {
                    let persons: Vec<Ptr<MSTransportable>> = self.base.myLane.get_edge().get_persons().iter().copied().collect();
                    for p in persons {
                        if p.get_lane() == lane && self.base2.vehicle_applies(&*p) {
                            let dir = p.get_direction();
                            let pos = p.get_position_on_lane();
                            self.notify_move_person(p, dir, pos);
                        }
                    }
                }
            }
        }
        // sort myMoveNotifications (required for jam processing) ascendingly according to vehicle's distance to the detector end
        // determinism: C++ std::sort is unstable; the order of equal distances may differ from SUMO
        self.myMoveNotifications.sort_by(|a, b| {
            if Self::compare_move_notification(*a, *b) {
                std::cmp::Ordering::Less
            } else if Self::compare_move_notification(*b, *a) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });

        // reset values concerning current time step (these are updated in integrateMoveNotification() and aggregateOutputValues())
        self.myCurrentVehicleSamples = 0.0;
        self.myCurrentMeanSpeed = 0.0;
        self.myCurrentMeanLength = 0.0;
        self.myCurrentStartedHalts = 0;
        self.myCurrentHaltingsNumber = 0;

        let mut currentJam: Ptr<MSE2Collector_JamInfo> = Ptr::null();
        let mut jams: Vec<Ptr<MSE2Collector_JamInfo>> = Vec::new();
        let mut haltingVehicles: BTreeMap<String, SUMOTime> = BTreeMap::new();
        let mut intervalHaltingVehicles: BTreeMap<String, SUMOTime> = BTreeMap::new();

        // go through the list of vehicles positioned on the detector
        for i in 0..self.myMoveNotifications.len() {
            let mni = self.myMoveNotifications[i];
            let vehID = mni.id.clone();
            match self.myVehicleInfos.get(&vehID).copied() {
                // The vehicle has already left the detector by lanechange, teleport, etc. (not longitudinal)
                None => self.integrate_move_notification(Ptr::null(), mni),
                // Add move notification infos to detector values and VehicleInfo
                Some(vi) => self.integrate_move_notification(vi, mni),
            }
            // construct jam structure
            let isInJam = self.check_jam(i, &mut haltingVehicles, &mut intervalHaltingVehicles);
            self.build_jam(isInJam, i, &mut currentJam, &mut jams);
        }

        // extract some aggregated values from the jam structure
        self.process_jams(&mut jams, currentJam);

        // Aggregate and normalize values for the detector output
        self.aggregate_output_values();

        // save information about halting vehicles
        self.myHaltingVehicleDurations = haltingVehicles;
        self.myIntervalHaltingVehicleDurations = intervalHaltingVehicles;

        // Remove the vehicles that have left the detector
        let leftVehicles = std::mem::take(&mut self.myLeftVehicles);
        for id in leftVehicles.iter() {
            if let Some(j) = self.myVehicleInfos.remove(id) {
                drop(unsafe { Box::from_raw(j.as_raw()) });
            }
            self.myNumberOfLeftVehicles += 1;
        }

        // reset move notifications
        for j in self.myMoveNotifications.drain(..) {
            drop(unsafe { Box::from_raw(j.as_raw()) });
        }
    }

    /// C++: `void MSE2Collector::writeXMLOutput(OutputDevice &, SUMOTime, SUMOTime)` [virtual]
    /// Write the generated output to the given device
    // void
    // MSE2Collector::writeXMLOutput(OutputDevice& dev, SUMOTime startTime, SUMOTime stopTime) {
    //     const double meanSpeed = getIntervalMeanSpeed();
    //     const double meanOccupancy = getIntervalOccupancy();
    //     const double meanTimeLoss = getIntervalMeanTimeLoss();
    //     myPreviousMeanOccupancy = meanOccupancy;
    //     myPreviousMeanSpeed = meanSpeed;
    //     myPreviousMaxJamLengthInMeters = myMaxJamInMeters;
    //     myPreviousNumberOfSeenVehicles = myNumberOfSeenVehicles;
    //     myPreviousMeanTimeLoss = meanTimeLoss;
    //
    //     if (dev.isNull()) {
    //         reset();
    //         return;
    //     }
    //     const double meanJamLengthInMeters = myTimeSamples != 0 ? myMeanMaxJamInMeters / (double) myTimeSamples : 0;
    //     const double meanJamLengthInVehicles = myTimeSamples != 0 ? myMeanMaxJamInVehicles / (double) myTimeSamples : 0;
    //     const double meanVehicleNumber = myTimeSamples != 0 ? (double) myMeanVehicleNumber / (double) myTimeSamples : 0;
    //
    //     SUMOTime haltingDurationSum = 0;
    //     SUMOTime maxHaltingDuration = 0;
    //     int haltingNo = 0;
    //     for (std::vector<SUMOTime>::iterator i = myPastStandingDurations.begin(); i != myPastStandingDurations.end(); ++i) {
    //         haltingDurationSum += (*i);
    //         maxHaltingDuration = MAX2(maxHaltingDuration, (*i));
    //         haltingNo++;
    //     }
    //     for (std::map<std::string, SUMOTime> ::iterator i = myHaltingVehicleDurations.begin(); i != myHaltingVehicleDurations.end(); ++i) {
    //         haltingDurationSum += (*i).second;
    //         maxHaltingDuration = MAX2(maxHaltingDuration, (*i).second);
    //         haltingNo++;
    //     }
    //     const SUMOTime meanHaltingDuration = haltingNo != 0 ? haltingDurationSum / haltingNo : 0;
    //
    //     SUMOTime intervalHaltingDurationSum = 0;
    //     SUMOTime intervalMaxHaltingDuration = 0;
    //     int intervalHaltingNo = 0;
    //     for (std::vector<SUMOTime>::iterator i = myPastIntervalStandingDurations.begin(); i != myPastIntervalStandingDurations.end(); ++i) {
    //         intervalHaltingDurationSum += (*i);
    //         intervalMaxHaltingDuration = MAX2(intervalMaxHaltingDuration, (*i));
    //         intervalHaltingNo++;
    //     }
    //     for (std::map<std::string, SUMOTime> ::iterator i = myIntervalHaltingVehicleDurations.begin(); i != myIntervalHaltingVehicleDurations.end(); ++i) {
    //         intervalHaltingDurationSum += (*i).second;
    //         intervalMaxHaltingDuration = MAX2(intervalMaxHaltingDuration, (*i).second);
    //         intervalHaltingNo++;
    //     }
    //     const SUMOTime intervalMeanHaltingDuration = intervalHaltingNo != 0 ? intervalHaltingDurationSum / intervalHaltingNo : 0;
    //
    // #ifdef DEBUG_E2_XML_OUT
    //     if (DEBUG_COND) {
    //         std::stringstream ss;
    //         ss  << "sampledSeconds=\"" << myVehicleSamples << "\" "
    //             << "myTimeSamples=\"" << myTimeSamples << "\" "
    //             << "myOccupancySum=\"" << myOccupancySum << "\" "
    //             << "myMeanVehicleNumber=\"" << myMeanVehicleNumber << "\" "
    //             << "nVehEntered=\"" << myNumberOfEnteredVehicles << "\" "
    //             << "meanSpeed=\"" << meanSpeed << "\"";
    //         std::cout << ss.str() << std::endl;
    //     }
    // #endif
    //
    //     dev.openTag(SUMO_TAG_INTERVAL).writeTime(SUMO_ATTR_BEGIN, startTime).writeTime(SUMO_ATTR_END, stopTime);
    //     dev.writeAttr(SUMO_ATTR_ID, StringUtils::escapeXML(getID()));
    //     dev.writeAttr("sampledSeconds", myVehicleSamples)
    //     .writeAttr("nVehEntered", myNumberOfEnteredVehicles)
    //     .writeAttr("nVehLeft", myNumberOfLeftVehicles)
    //     .writeAttr("nVehSeen", myNumberOfSeenVehicles)
    //     .writeAttr("meanSpeed", meanSpeed)
    //     .writeAttr("meanTimeLoss", meanTimeLoss)
    //     .writeAttr("meanOccupancy", meanOccupancy)
    //     .writeAttr("maxOccupancy", myMaxOccupancy)
    //     .writeAttr("meanMaxJamLengthInVehicles", meanJamLengthInVehicles)
    //     .writeAttr("meanMaxJamLengthInMeters", meanJamLengthInMeters)
    //     .writeAttr("maxJamLengthInVehicles", myMaxJamInVehicles)
    //     .writeAttr("maxJamLengthInMeters", myMaxJamInMeters)
    //     .writeAttr("jamLengthInVehiclesSum", myJamLengthInVehiclesSum)
    //     .writeAttr("jamLengthInMetersSum", myJamLengthInMetersSum)
    //     .writeAttr("meanHaltingDuration", STEPS2TIME(meanHaltingDuration))
    //     .writeAttr("maxHaltingDuration", STEPS2TIME(maxHaltingDuration))
    //     .writeAttr("haltingDurationSum", STEPS2TIME(haltingDurationSum))
    //     .writeAttr("meanIntervalHaltingDuration", STEPS2TIME(intervalMeanHaltingDuration))
    //     .writeAttr("maxIntervalHaltingDuration", STEPS2TIME(intervalMaxHaltingDuration))
    //     .writeAttr("intervalHaltingDurationSum", STEPS2TIME(intervalHaltingDurationSum))
    //     .writeAttr("startedHalts", myStartedHalts)
    //     .writeAttr("meanVehicleNumber", meanVehicleNumber)
    //     .writeAttr("maxVehicleNumber", myMaxVehicleNumber)
    //     .closeTag();
    //
    //     reset();
    // }
    #[allow(unused_variables)]
    pub fn write_xml_output(&mut self, dev: &mut OutputDevice, startTime: SUMOTime, stopTime: SUMOTime) {
        let meanSpeed = self.get_interval_mean_speed();
        let meanOccupancy = self.get_interval_occupancy();
        let meanTimeLoss = self.get_interval_mean_time_loss();
        self.myPreviousMeanOccupancy = meanOccupancy;
        self.myPreviousMeanSpeed = meanSpeed;
        self.myPreviousMaxJamLengthInMeters = self.myMaxJamInMeters;
        self.myPreviousNumberOfSeenVehicles = self.myNumberOfSeenVehicles;
        self.myPreviousMeanTimeLoss = meanTimeLoss;

        if dev.is_null() {
            self.reset();
            return;
        }
        let meanJamLengthInMeters = if self.myTimeSamples != 0 { self.myMeanMaxJamInMeters / self.myTimeSamples as f64 } else { 0.0 };
        let meanJamLengthInVehicles = if self.myTimeSamples != 0 { self.myMeanMaxJamInVehicles as f64 / self.myTimeSamples as f64 } else { 0.0 };
        let meanVehicleNumber = if self.myTimeSamples != 0 { self.myMeanVehicleNumber as f64 / self.myTimeSamples as f64 } else { 0.0 };

        let mut haltingDurationSum: SUMOTime = 0;
        let mut maxHaltingDuration: SUMOTime = 0;
        let mut haltingNo: i32 = 0;
        for &d in self.myPastStandingDurations.iter() {
            haltingDurationSum += d;
            maxHaltingDuration = std::cmp::max(maxHaltingDuration, d);
            haltingNo += 1;
        }
        for &d in self.myHaltingVehicleDurations.values() {
            haltingDurationSum += d;
            maxHaltingDuration = std::cmp::max(maxHaltingDuration, d);
            haltingNo += 1;
        }
        let meanHaltingDuration: SUMOTime = if haltingNo != 0 { haltingDurationSum / haltingNo as i64 } else { 0 };

        let mut intervalHaltingDurationSum: SUMOTime = 0;
        let mut intervalMaxHaltingDuration: SUMOTime = 0;
        let mut intervalHaltingNo: i32 = 0;
        for &d in self.myPastIntervalStandingDurations.iter() {
            intervalHaltingDurationSum += d;
            intervalMaxHaltingDuration = std::cmp::max(intervalMaxHaltingDuration, d);
            intervalHaltingNo += 1;
        }
        for &d in self.myIntervalHaltingVehicleDurations.values() {
            intervalHaltingDurationSum += d;
            intervalMaxHaltingDuration = std::cmp::max(intervalMaxHaltingDuration, d);
            intervalHaltingNo += 1;
        }
        let intervalMeanHaltingDuration: SUMOTime = if intervalHaltingNo != 0 { intervalHaltingDurationSum / intervalHaltingNo as i64 } else { 0 };

        dev.open_tag_id(SUMO_TAG_INTERVAL)
            .write_time(SUMO_ATTR_BEGIN, startTime)
            .write_time(SUMO_ATTR_END, stopTime);
        let escapedID = utils::common::string_utils::escape_xml(self.base2.get_id(), false);
        dev.write_attr(SUMO_ATTR_ID, &escapedID);
        dev.write_attr("sampledSeconds", &self.myVehicleSamples)
            .write_attr("nVehEntered", &self.myNumberOfEnteredVehicles)
            .write_attr("nVehLeft", &self.myNumberOfLeftVehicles)
            .write_attr("nVehSeen", &self.myNumberOfSeenVehicles)
            .write_attr("meanSpeed", &meanSpeed)
            .write_attr("meanTimeLoss", &meanTimeLoss)
            .write_attr("meanOccupancy", &meanOccupancy)
            .write_attr("maxOccupancy", &self.myMaxOccupancy)
            .write_attr("meanMaxJamLengthInVehicles", &meanJamLengthInVehicles)
            .write_attr("meanMaxJamLengthInMeters", &meanJamLengthInMeters)
            .write_attr("maxJamLengthInVehicles", &self.myMaxJamInVehicles)
            .write_attr("maxJamLengthInMeters", &self.myMaxJamInMeters)
            .write_attr("jamLengthInVehiclesSum", &self.myJamLengthInVehiclesSum)
            .write_attr("jamLengthInMetersSum", &self.myJamLengthInMetersSum)
            .write_attr("meanHaltingDuration", &steps2time(meanHaltingDuration))
            .write_attr("maxHaltingDuration", &steps2time(maxHaltingDuration))
            .write_attr("haltingDurationSum", &steps2time(haltingDurationSum))
            .write_attr("meanIntervalHaltingDuration", &steps2time(intervalMeanHaltingDuration))
            .write_attr("maxIntervalHaltingDuration", &steps2time(intervalMaxHaltingDuration))
            .write_attr("intervalHaltingDurationSum", &steps2time(intervalHaltingDurationSum))
            .write_attr("startedHalts", &self.myStartedHalts)
            .write_attr("meanVehicleNumber", &meanVehicleNumber)
            .write_attr("maxVehicleNumber", &self.myMaxVehicleNumber);
        dev.close_tag("");

        self.reset();
    }

    /// C++: `void MSE2Collector::writeXMLDetectorProlog(OutputDevice &) const` [const, virtual]
    /// Open the XML-output
    // void
    // MSE2Collector::writeXMLDetectorProlog(OutputDevice& dev) const {
    //     dev.writeXMLHeader("detector", "det_e2_file.xsd");
    // }
    #[allow(unused_variables)]
    pub fn write_xml_detector_prolog(&self, dev: &mut OutputDevice) {
        dev.write_xml_header("detector", "det_e2_file.xsd", BTreeMap::new(), true);
    }

    /// C++: `const std::string & MSE2Collector::getName()`
    /// get name
    // const std::string& getName() {
    //         return myName;
    //     }
    #[allow(unused_variables)]
    pub fn get_name(&mut self) -> &str {
        &self.myName
    }

    /// C++: `double MSE2Collector::getStartPos() const` [const]
    /// Returns the begin position of the detector
    // double getStartPos() const {
    //         return myStartPos;
    //     }
    #[allow(unused_variables)]
    pub fn get_start_pos(&self) -> f64 {
        self.myStartPos
    }

    /// C++: `double MSE2Collector::getEndPos() const` [const]
    /// Returns the end position of the detector
    // double getEndPos() const {
    //         return myEndPos;
    //     }
    #[allow(unused_variables)]
    pub fn get_end_pos(&self) -> f64 {
        self.myEndPos
    }

    /// C++: `double MSE2Collector::getLength() const` [const]
    /// Returns the length of the detector
    // double getLength() const {
    //         return myDetectorLength;
    //     }
    #[allow(unused_variables)]
    pub fn get_length(&self) -> f64 {
        self.myDetectorLength
    }

    /// C++: `MSLane * MSE2Collector::getLastLane() const` [const]
    /// Returns the id of the detector's last lane
    // MSLane* getLastLane() const {
    //         return myLastLane;
    //     }
    #[allow(unused_variables)]
    pub fn get_last_lane(&self) -> Ptr<MSLane> {
        self.myLastLane
    }

    /// C++: `std::vector<MSLane *> MSE2Collector::getLanes()`
    /// Returns a vector containing pointers to the lanes covered by the detector ordered from its first to its last lane
    // std::vector<MSLane*>
    // MSE2Collector::getLanes() {
    //     std::vector<MSLane*> res;
    //     for (std::vector<std::string>::const_iterator i = myLanes.begin(); i != myLanes.end(); ++i) {
    //         res.push_back(MSLane::dictionary(*i));
    //     }
    //     return res;
    // }
    #[allow(unused_variables)]
    pub fn get_lanes(&mut self) -> Vec<Ptr<MSLane>> {
        self.myLanes.iter().map(|id| MSLane::dictionary_2(id)).collect()
    }

    /// C++: `void MSE2Collector::reset()` [virtual]
    /// Resets all values
    // void
    // MSE2Collector::reset() {
    //     myVehicleSamples = 0;
    //     myTotalTimeLoss = 0.;
    //     myNumberOfEnteredVehicles = 0;
    //     myNumberOfSeenVehicles -= myNumberOfLeftVehicles;
    //     myNumberOfLeftVehicles = 0;
    //     myMaxVehicleNumber = 0;
    //
    //     mySpeedSum = 0;
    //     myStartedHalts = 0;
    //     myJamLengthInMetersSum = 0;
    //     myJamLengthInVehiclesSum = 0;
    //     myOccupancySum = 0;
    //     myMaxOccupancy = 0;
    //     myMeanMaxJamInVehicles = 0;
    //     myMeanMaxJamInMeters = 0;
    //     myMaxJamInVehicles = 0;
    //     myMaxJamInMeters = 0;
    //     myTimeSamples = 0;
    //     myMeanVehicleNumber = 0;
    //     for (std::map<std::string, SUMOTime>::iterator i = myIntervalHaltingVehicleDurations.begin(); i != myIntervalHaltingVehicleDurations.end(); ++i) {
    //         (*i).second = 0;
    //     }
    //     myPastStandingDurations.clear();
    //     myPastIntervalStandingDurations.clear();
    // }
    #[allow(unused_variables)]
    pub fn reset(&mut self) {
        self.myVehicleSamples = 0.0;
        self.myTotalTimeLoss = 0.0;
        self.myNumberOfEnteredVehicles = 0;
        self.myNumberOfSeenVehicles -= self.myNumberOfLeftVehicles;
        self.myNumberOfLeftVehicles = 0;
        self.myMaxVehicleNumber = 0;

        self.mySpeedSum = 0.0;
        self.myStartedHalts = 0;
        self.myJamLengthInMetersSum = 0.0;
        self.myJamLengthInVehiclesSum = 0;
        self.myOccupancySum = 0.0;
        self.myMaxOccupancy = 0.0;
        self.myMeanMaxJamInVehicles = 0;
        self.myMeanMaxJamInMeters = 0.0;
        self.myMaxJamInVehicles = 0;
        self.myMaxJamInMeters = 0.0;
        self.myTimeSamples = 0;
        self.myMeanVehicleNumber = 0;
        for v in self.myIntervalHaltingVehicleDurations.values_mut() {
            *v = 0;
        }
        self.myPastStandingDurations.clear();
        self.myPastIntervalStandingDurations.clear();
    }

    /// C++: `int MSE2Collector::getCurrentVehicleNumber() const` [const]
    /// Returns the number of vehicles currently on the detector
    // int
    // MSE2Collector::getCurrentVehicleNumber() const {
    //     int result = 0;
    //     if (myOverrideVehNumber >= 0) {
    //         result = myOverrideVehNumber;
    //     } else {
    //         for (VehicleInfoMap::const_iterator it = myVehicleInfos.begin(); it != myVehicleInfos.end(); it++) {
    //             if (it->second->onDetector) {
    //                 result++;
    //             }
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_current_vehicle_number(&self) -> i32 {
        let mut result = 0;
        if self.myOverrideVehNumber >= 0 {
            result = self.myOverrideVehNumber;
        } else {
            for it in self.myVehicleInfos.values() {
                if it.onDetector {
                    result += 1;
                }
            }
        }
        result
    }

    /// C++: `double MSE2Collector::getCurrentOccupancy() const` [const]
    /// Returns the current detector occupancy
    // double getCurrentOccupancy() const {
    //         return myCurrentOccupancy;
    //     }
    #[allow(unused_variables)]
    pub fn get_current_occupancy(&self) -> f64 {
        self.myCurrentOccupancy
    }

    /// C++: `double MSE2Collector::getCurrentMeanSpeed() const` [const]
    /// Returns the mean vehicle speed of vehicles currently on the detector
    // double getCurrentMeanSpeed() const {
    //         return myCurrentMeanSpeed;
    //     }
    #[allow(unused_variables)]
    pub fn get_current_mean_speed(&self) -> f64 {
        self.myCurrentMeanSpeed
    }

    /// C++: `double MSE2Collector::getCurrentMeanLength() const` [const]
    /// Returns the mean vehicle length of vehicles currently on the detector
    // double getCurrentMeanLength() const {
    //         return myCurrentMeanLength;
    //     }
    #[allow(unused_variables)]
    pub fn get_current_mean_length(&self) -> f64 {
        self.myCurrentMeanLength
    }

    /// C++: `int MSE2Collector::getCurrentJamNumber() const` [const]
    /// Returns the current number of jams
    // int getCurrentJamNumber() const {
    //         return myCurrentJamNo;
    //     }
    #[allow(unused_variables)]
    pub fn get_current_jam_number(&self) -> i32 {
        self.myCurrentJamNo
    }

    /// C++: `int MSE2Collector::getCurrentMaxJamLengthInVehicles() const` [const]
    /// Returns the length in vehicles of the currently largest jam
    // int getCurrentMaxJamLengthInVehicles() const {
    //         return myCurrentMaxJamLengthInVehicles;
    //     }
    #[allow(unused_variables)]
    pub fn get_current_max_jam_length_in_vehicles(&self) -> i32 {
        self.myCurrentMaxJamLengthInVehicles
    }

    /// C++: `double MSE2Collector::getCurrentMaxJamLengthInMeters() const` [const]
    /// Returns the length in meters of the currently largest jam
    // double getCurrentMaxJamLengthInMeters() const {
    //         return myCurrentMaxJamLengthInMeters;
    //     }
    #[allow(unused_variables)]
    pub fn get_current_max_jam_length_in_meters(&self) -> f64 {
        self.myCurrentMaxJamLengthInMeters
    }

    /// C++: `int MSE2Collector::getCurrentJamLengthInVehicles() const` [const]
    /// Returns the length of all jams in vehicles
    // int getCurrentJamLengthInVehicles() const {
    //         return myCurrentJamLengthInVehicles;
    //     }
    #[allow(unused_variables)]
    pub fn get_current_jam_length_in_vehicles(&self) -> i32 {
        self.myCurrentJamLengthInVehicles
    }

    /// C++: `double MSE2Collector::getCurrentJamLengthInMeters() const` [const]
    /// Returns the length of all jams in meters
    // double getCurrentJamLengthInMeters() const {
    //         return myCurrentJamLengthInMeters;
    //     }
    #[allow(unused_variables)]
    pub fn get_current_jam_length_in_meters(&self) -> f64 {
        self.myCurrentJamLengthInMeters
    }

    /// C++: `double MSE2Collector::getCurrentJamDuration() const` [const]
    /// Returns the length of the longest-duration jam in s
    // double getCurrentJamDuration() const {
    //         return myCurrentJamDuration;
    //     }
    #[allow(unused_variables)]
    pub fn get_current_jam_duration(&self) -> f64 {
        self.myCurrentJamDuration
    }

    /// C++: `int MSE2Collector::getCurrentStartedHalts() const` [const]
    /// Returns the length of all jams in meters
    // int getCurrentStartedHalts() const {
    //         return myCurrentStartedHalts;
    //     }
    #[allow(unused_variables)]
    pub fn get_current_started_halts(&self) -> i32 {
        self.myCurrentStartedHalts
    }

    /// C++: `int MSE2Collector::getCurrentHaltingNumber() const` [const]
    /// Returns the number of current haltings within the area
    // int getCurrentHaltingNumber() const {
    //         return myCurrentHaltingsNumber;
    //     }
    #[allow(unused_variables)]
    pub fn get_current_halting_number(&self) -> i32 {
        self.myCurrentHaltingsNumber
    }

    /// C++: `std::vector<std::string> MSE2Collector::getCurrentVehicleIDs() const` [const]
    /// Returns the IDs of the vehicles within the area
    // std::vector<std::string>
    // MSE2Collector::getCurrentVehicleIDs() const {
    //     std::vector<std::string> ret;
    //     for (VehicleInfoMap::const_iterator i = myVehicleInfos.begin(); i != myVehicleInfos.end(); ++i) {
    //         if (i->second->onDetector) {
    //             ret.push_back(i->second->id);
    //         }
    //     }
    //     std::sort(ret.begin(), ret.end());
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn get_current_vehicle_i_ds(&self) -> Vec<String> {
        let mut ret: Vec<String> = self.myVehicleInfos.values().filter(|vi| vi.onDetector).map(|vi| vi.id.clone()).collect();
        ret.sort();
        ret
    }

    /// C++: `std::vector<VehicleInfo *> MSE2Collector::getCurrentVehicles() const` [const]
    /// Returns the VehicleInfos for the vehicles currently on the detector
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_current_vehicles(&self) -> Vec<Ptr<MSE2Collector_VehicleInfo>> {
        self.myVehicleInfos.values().filter(|vi| vi.onDetector).copied().collect()
    }

    /// C++: `int MSE2Collector::getPassedVeh()`
    /// Returns the number of vehicles passed over the sensor (i.e. entered the sensor)
    // int getPassedVeh() {
    //         return myNumberOfEnteredVehicles;
    //     }
    #[allow(unused_variables)]
    pub fn get_passed_veh(&mut self) -> i32 {
        self.myNumberOfEnteredVehicles
    }

    /// C++: `void MSE2Collector::subtractPassedVeh(int)`
    /// Subtract the number of vehicles indicated from passed from the sensor count.
    // void subtractPassedVeh(int passed) {
    //         myNumberOfEnteredVehicles -= passed;
    //     }
    #[allow(unused_variables)]
    pub fn subtract_passed_veh(&mut self, passed: i32) {
        self.myNumberOfEnteredVehicles -= passed;
    }

    /// C++: `double MSE2Collector::getIntervalOccupancy() const` [const]
    /// @{
    // double getIntervalOccupancy() const {
    //         return myTimeSamples != 0 ? myOccupancySum / (double) myTimeSamples : 0;
    //     }
    #[allow(unused_variables)]
    pub fn get_interval_occupancy(&self) -> f64 {
        if self.myTimeSamples != 0 {
            self.myOccupancySum / self.myTimeSamples as f64
        } else {
            0.0
        }
    }

    /// C++: `double MSE2Collector::getIntervalMeanSpeed() const` [const]
    // double getIntervalMeanSpeed() const {
    //         return myVehicleSamples != 0 ? mySpeedSum / myVehicleSamples : -1;
    //     }
    #[allow(unused_variables)]
    pub fn get_interval_mean_speed(&self) -> f64 {
        if self.myVehicleSamples != 0.0 {
            self.mySpeedSum / self.myVehicleSamples
        } else {
            -1.0
        }
    }

    /// C++: `double MSE2Collector::getIntervalMeanTimeLoss() const` [const]
    // double getIntervalMeanTimeLoss() const {
    //         return myNumberOfSeenVehicles != 0 ? myTotalTimeLoss / myNumberOfSeenVehicles : -1;
    //     }
    #[allow(unused_variables)]
    pub fn get_interval_mean_time_loss(&self) -> f64 {
        if self.myNumberOfSeenVehicles != 0 {
            self.myTotalTimeLoss / self.myNumberOfSeenVehicles as f64
        } else {
            -1.0
        }
    }

    /// C++: `double MSE2Collector::getIntervalMaxJamLengthInMeters() const` [const]
    // double getIntervalMaxJamLengthInMeters() const {
    //         return myMaxJamInMeters;
    //     }
    #[allow(unused_variables)]
    pub fn get_interval_max_jam_length_in_meters(&self) -> f64 {
        self.myMaxJamInMeters
    }

    /// C++: `int MSE2Collector::getIntervalVehicleNumber() const` [const]
    // int getIntervalVehicleNumber() const {
    //         return myNumberOfSeenVehicles;
    //     }
    #[allow(unused_variables)]
    pub fn get_interval_vehicle_number(&self) -> i32 {
        self.myNumberOfSeenVehicles
    }

    /// C++: `double MSE2Collector::getLastIntervalOccupancy() const` [const]
    // double getLastIntervalOccupancy() const {
    //         return myPreviousMeanOccupancy;
    //     }
    #[allow(unused_variables)]
    pub fn get_last_interval_occupancy(&self) -> f64 {
        self.myPreviousMeanOccupancy
    }

    /// C++: `double MSE2Collector::getLastIntervalMeanSpeed() const` [const]
    // double getLastIntervalMeanSpeed() const {
    //         return myPreviousMeanSpeed;
    //     }
    #[allow(unused_variables)]
    pub fn get_last_interval_mean_speed(&self) -> f64 {
        self.myPreviousMeanSpeed
    }

    /// C++: `double MSE2Collector::getLastIntervalMeanTimeLoss() const` [const]
    // double getLastIntervalMeanTimeLoss() const {
    //         return myPreviousMeanTimeLoss;
    //     }
    #[allow(unused_variables)]
    pub fn get_last_interval_mean_time_loss(&self) -> f64 {
        self.myPreviousMeanTimeLoss
    }

    /// C++: `double MSE2Collector::getLastIntervalMaxJamLengthInMeters() const` [const]
    // double getLastIntervalMaxJamLengthInMeters() const {
    //         return myPreviousMaxJamLengthInMeters;
    //     }
    #[allow(unused_variables)]
    pub fn get_last_interval_max_jam_length_in_meters(&self) -> f64 {
        self.myPreviousMaxJamLengthInMeters
    }

    /// C++: `int MSE2Collector::getLastIntervalVehicleNumber() const` [const]
    // int getLastIntervalVehicleNumber() const {
    //         return myPreviousNumberOfSeenVehicles;
    //     }
    #[allow(unused_variables)]
    pub fn get_last_interval_vehicle_number(&self) -> i32 {
        self.myPreviousNumberOfSeenVehicles
    }

    /// C++: `int MSE2Collector::getEstimatedCurrentVehicleNumber(double) const` [const]
    /// Returns an estimate of the number of vehicles currently on the detector
    // int
    // MSE2Collector::getEstimatedCurrentVehicleNumber(double speedThreshold) const {
    //
    //     //    double distance = std::numeric_limits<double>::max();
    //     double thresholdSpeed = myLane->getSpeedLimit() / speedThreshold;
    //
    //     int count = 0;
    //     for (VehicleInfoMap::const_iterator it = myVehicleInfos.begin();
    //             it != myVehicleInfos.end(); it++) {
    //         if (it->second->onDetector) {
    //             //            if (it->position < distance) {
    //             //                distance = it->position;
    //             //            }
    //             //            const double realDistance = myLane->getLength() - distance; // the closer vehicle get to the light the greater is the distance
    //             const double realDistance = it->second->distToDetectorEnd;
    //             if (it->second->lastSpeed <= thresholdSpeed || it->second->lastAccel > 0) { //TODO speed less half of the maximum speed for the lane NEED TUNING
    //                 count = (int)(realDistance / (it->second->length + it->second->minGap)) + 1;
    //             }
    //         }
    //     }
    //
    //     return count;
    // }
    #[allow(unused_variables)]
    pub fn get_estimated_current_vehicle_number(&self, speedThreshold: f64) -> i32 {
        let thresholdSpeed = self.base.myLane.get_speed_limit() / speedThreshold;

        let mut count: i32 = 0;
        for it in self.myVehicleInfos.values() {
            if it.onDetector {
                let realDistance = it.distToDetectorEnd;
                if it.lastSpeed <= thresholdSpeed || it.lastAccel > 0.0 {
                    count = (realDistance / (it.length + it.minGap)) as i32 + 1;
                }
            }
        }
        count
    }

    /// C++: `double MSE2Collector::getEstimateQueueLength() const` [const]
    /// Returns an estimate of the length of the queue of vehicles currently stopped on the detector
    // double
    // MSE2Collector::getEstimateQueueLength() const {
    //
    //     if (myVehicleInfos.empty()) {
    //         return 0;
    //     }
    //
    //     double distance = 0;
    //     double realDistance = 0;
    //     bool flowing =  true;
    //     for (VehicleInfoMap::const_iterator it = myVehicleInfos.begin();
    //             it != myVehicleInfos.end(); it++) {
    //         if (it->second->onDetector && it->second->totalTimeOnDetector > 0) {
    //             //  double distanceTemp = myLane->getLength() - distance;
    //             if (it->second->lastSpeed <= myJamHaltingSpeedThreshold) {
    //                 distance = MAX2(it->second->distToDetectorEnd, distance);
    //                 realDistance = distance + it->second->length;
    //                 flowing = false;
    //             }
    //             //            DBG(
    //             //                std::ostringstream str;
    //             //                str << time2string(MSNet::getInstance()->getCurrentTimeStep())
    //             //                << " MSE2Collector::getEstimateQueueLength::"
    //             //                << " lane " << myLane->getID()
    //             //                << " vehicle " << it->second.id
    //             //                << " positionOnLane " << it->second.position
    //             //                << " vel " << it->second.speed
    //             //                << " realDistance " << realDistance;
    //             //                WRITE_MESSAGE(str.str());
    //             //            )
    //         }
    //     }
    //     if (flowing) {
    //         return 0;
    //     } else {
    //         return realDistance;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_estimate_queue_length(&self) -> f64 {
        if self.myVehicleInfos.is_empty() {
            return 0.0;
        }

        let mut distance = 0.0;
        let mut realDistance = 0.0;
        let mut flowing = true;
        for it in self.myVehicleInfos.values() {
            if it.onDetector && it.totalTimeOnDetector > 0.0 {
                if it.lastSpeed <= self.myJamHaltingSpeedThreshold {
                    distance = max2(it.distToDetectorEnd, distance);
                    realDistance = distance + it.length;
                    flowing = false;
                }
            }
        }
        if flowing {
            0.0
        } else {
            realDistance
        }
    }

    /// C++: `double MSE2Collector::getArrivalDelay() const` [const]
    /// Returns the maximum stop arrival delay of public transport vehicles that are on the detector or passed the detector in the last step or -DOUBLE_VALUE
    // double
    // MSE2Collector::getArrivalDelay() const {
    // #ifdef HAVE_FOX
    //     ScopedLocker<> lock(myNotificationMutex, MSGlobals::gNumSimThreads > 1);
    // #endif
    //     MSVehicleControl& vc = MSNet::getInstance()->getVehicleControl();
    //     double result = -INVALID_DOUBLE;
    //     for (auto item : myVehicleInfos) {
    //         if (item.second->onDetector) {
    //             SUMOVehicle* v = vc.getVehicle(item.first);
    //             if (v != nullptr) {
    //                 MSBaseVehicle* veh = dynamic_cast<MSBaseVehicle*>(v);
    //                 double ad = veh->getStopArrivalDelay();
    //                 if (ad != INVALID_DOUBLE) {
    //                     result = MAX2(result, ad);
    //                 }
    //             }
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_arrival_delay(&self) -> f64 {
        let net = MSNet::get_instance();
        let vc: &mut MSVehicleControl = net.m().get_vehicle_control();
        let mut result = -INVALID_DOUBLE;
        for (id, item) in self.myVehicleInfos.iter() {
            if item.onDetector {
                let v = vc.get_vehicle(id);
                if !v.is_null() {
                    // shortcut: C++ dynamic_cast<MSBaseVehicle*>; the cast is unchecked here, only vehicles from MSVehicleControl reach this detector
                    let veh: Ptr<MSBaseVehicle> = v.cast::<MSBaseVehicle>();
                    let ad = veh.get_stop_arrival_delay();
                    if ad != INVALID_DOUBLE {
                        result = max2(result, ad);
                    }
                }
            }
        }
        result
    }

    /// C++: `void MSE2Collector::setVisible(bool)` [virtual]
    // virtual void setVisible(bool /*show*/) {}
    #[allow(unused_variables)]
    pub fn set_visible(&mut self, arg0: bool) {
        // C++ body is empty.
        let _ = arg0;
    }

    /// C++: `void MSE2Collector::clearState(SUMOTime)` [virtual]
    /// Remove all vehicles before quick-loading state
    // void
    // MSE2Collector::clearState(SUMOTime /* step */) {
    //     for (std::vector<MoveNotificationInfo*>::iterator j = myMoveNotifications.begin(); j != myMoveNotifications.end(); ++j) {
    //         delete *j;
    //     }
    //     myMoveNotifications.clear();
    //
    //     // clear vehicle infos
    //     for (VehicleInfoMap::iterator j = myVehicleInfos.begin(); j != myVehicleInfos.end(); ++j) {
    //         delete j->second;
    //     }
    //     myVehicleInfos.clear();
    // }
    #[allow(unused_variables)]
    pub fn clear_state(&mut self, step: SUMOTime) {
        for j in self.myMoveNotifications.drain(..) {
            drop(unsafe { Box::from_raw(j.as_raw()) });
        }

        // clear vehicle infos
        for (_, j) in std::mem::take(&mut self.myVehicleInfos) {
            drop(unsafe { Box::from_raw(j.as_raw()) });
        }
    }

    /// C++: `void MSE2Collector::overrideVehicleNumber(int)`
    /// Persistently overrides the number of vehicles on top of the detector Setting a negative value removes the override
    // void
    // MSE2Collector::overrideVehicleNumber(int num) {
    //     myOverrideVehNumber = num;
    // }
    #[allow(unused_variables)]
    pub fn override_vehicle_number(&mut self, num: i32) {
        self.myOverrideVehNumber = num;
    }

    /// C++: `double MSE2Collector::getOverrideVehNumber() const` [const]
    // double getOverrideVehNumber() const {
    //         return myOverrideVehNumber;
    //     }
    #[allow(unused_variables)]
    pub fn get_override_veh_number(&self) -> f64 {
        self.myOverrideVehNumber as f64
    }

    /// C++: `bool MSE2Collector::checkJam(std::vector<MoveNotificationInfo *>::const_iterator, std::map<std::string, SUMOTime> &, std::map<std::string, SUMOTime> &)` [private]
    /// checks whether the vehicle stands in a jam
    // bool
    // MSE2Collector::checkJam(std::vector<MoveNotificationInfo*>::const_iterator mni, std::map<std::string, SUMOTime>& haltingVehicles, std::map<std::string, SUMOTime>& intervalHaltingVehicles) {
    // #ifdef DEBUG_E2_JAMS
    //     if (DEBUG_COND) {
    //         std::cout << SIMTIME << " CheckJam() for vehicle '" << (*mni)->id << "'" << std::endl;
    //     }
    // #endif
    //     // jam-checking begins
    //     bool isInJam = false;
    //     // first, check whether the vehicle is slow enough to be counted as halting
    //     if ((*mni)->speed < myJamHaltingSpeedThreshold) {
    //         myCurrentHaltingsNumber++;
    //         // we have to track the time it was halting;
    //         // so let's look up whether it was halting before and compute the overall halting time
    //         bool wasHalting = myHaltingVehicleDurations.count((*mni)->id) > 0;
    //         if (wasHalting) {
    //             haltingVehicles[(*mni)->id] = myHaltingVehicleDurations[(*mni)->id] + DELTA_T;
    //             intervalHaltingVehicles[(*mni)->id] = myIntervalHaltingVehicleDurations[(*mni)->id] + DELTA_T;
    //         } else {
    // #ifdef DEBUG_E2_JAMS
    //             if (DEBUG_COND) {
    //                 std::cout << SIMTIME << " vehicle '" << (*mni)->id << "' starts halting." << std::endl;
    //             }
    // #endif
    //             haltingVehicles[(*mni)->id] = DELTA_T;
    //             intervalHaltingVehicles[(*mni)->id] = DELTA_T;
    //             myCurrentStartedHalts++;
    //             myStartedHalts++;
    //         }
    //         // we now check whether the halting time is large enough
    //         if (haltingVehicles[(*mni)->id] > myJamHaltingTimeThreshold) {
    //             // yep --> the vehicle is a part of a jam
    //             isInJam = true;
    //         }
    //     } else {
    //         // is not standing anymore; keep duration information
    //         std::map<std::string, SUMOTime>::iterator v = myHaltingVehicleDurations.find((*mni)->id);
    //         if (v != myHaltingVehicleDurations.end()) {
    //             myPastStandingDurations.push_back(v->second);
    //             myHaltingVehicleDurations.erase(v);
    //         }
    //         v = myIntervalHaltingVehicleDurations.find((*mni)->id);
    //         if (v != myIntervalHaltingVehicleDurations.end()) {
    //             myPastIntervalStandingDurations.push_back((*v).second);
    //             myIntervalHaltingVehicleDurations.erase(v);
    //         }
    //     }
    // #ifdef DEBUG_E2_JAMS
    //     if (DEBUG_COND) {
    //         std::cout << SIMTIME << " vehicle '" << (*mni)->id << "'" << (isInJam ? "is jammed." : "is not jammed.") << std::endl;
    //     }
    // #endif
    //     return isInJam;
    // }
    #[allow(unused_variables)]
    pub fn check_jam(&mut self, mni: usize, haltingVehicles: &mut BTreeMap<String, SUMOTime>, intervalHaltingVehicles: &mut BTreeMap<String, SUMOTime>) -> bool {
        let m = self.myMoveNotifications[mni];
        // first, check whether the vehicle is slow enough to be counted as halting
        let mut isInJam = false;
        if m.speed < self.myJamHaltingSpeedThreshold {
            self.myCurrentHaltingsNumber += 1;
            // we have to track the time it was halting;
            // so let's look up whether it was halting before and compute the overall halting time
            let wasHalting = self.myHaltingVehicleDurations.contains_key(&m.id);
            if wasHalting {
                let v = *self.myHaltingVehicleDurations.entry(m.id.clone()).or_insert(0) + delta_t();
                haltingVehicles.insert(m.id.clone(), v);
                let iv = *self.myIntervalHaltingVehicleDurations.entry(m.id.clone()).or_insert(0) + delta_t();
                intervalHaltingVehicles.insert(m.id.clone(), iv);
            } else {
                haltingVehicles.insert(m.id.clone(), delta_t());
                intervalHaltingVehicles.insert(m.id.clone(), delta_t());
                self.myCurrentStartedHalts += 1;
                self.myStartedHalts += 1;
            }
            // we now check whether the halting time is large enough
            if haltingVehicles[&m.id] > self.myJamHaltingTimeThreshold {
                // yep --> the vehicle is a part of a jam
                isInJam = true;
            }
        } else {
            // is not standing anymore; keep duration information
            if let Some(v) = self.myHaltingVehicleDurations.remove(&m.id) {
                self.myPastStandingDurations.push(v);
            }
            if let Some(v) = self.myIntervalHaltingVehicleDurations.remove(&m.id) {
                self.myPastIntervalStandingDurations.push(v);
            }
        }
        isInJam
    }

    /// C++: `void MSE2Collector::buildJam(bool, std::vector<MoveNotificationInfo *>::const_iterator, JamInfo *&, std::vector<JamInfo *> &)` [private]
    /// Either adds the vehicle to the end of an existing jam, or closes the last jam, and/or creates a new jam
    // void
    // MSE2Collector::buildJam(bool isInJam, std::vector<MoveNotificationInfo*>::const_iterator mni, JamInfo*& currentJam, std::vector<JamInfo*>& jams) {
    // #ifdef DEBUG_E2_JAMS
    //     if (DEBUG_COND) {
    //         std::cout << SIMTIME << " buildJam() for vehicle '" << (*mni)->id << "'" << std::endl;
    //     }
    // #endif
    //     if (isInJam) {
    //         // The vehicle is in a jam;
    //         //  it may be a new one or already an existing one
    //         if (currentJam == nullptr) {
    // #ifdef DEBUG_E2_JAMS
    //             if (DEBUG_COND) {
    //                 std::cout << SIMTIME << " vehicle '" << (*mni)->id << "' forms the start of the first jam" << std::endl;
    //             }
    // #endif
    //             // the vehicle is the first vehicle in a jam
    //             currentJam = new JamInfo();
    //             currentJam->firstStandingVehicle = mni;
    //         } else {
    //             // ok, we have a jam already. But - maybe it is too far away
    //             //  ... honestly, I can hardly find a reason for doing this,
    //             //  but jams were defined this way in an earlier version...
    //             MoveNotificationInfo* lastVeh = *currentJam->lastStandingVehicle;
    //             MoveNotificationInfo* currVeh = *mni;
    //             if (lastVeh->distToDetectorEnd - currVeh->distToDetectorEnd > myJamDistanceThreshold) {
    // #ifdef DEBUG_E2_JAMS
    //                 if (DEBUG_COND) {
    //                     std::cout << SIMTIME << " vehicle '" << (*mni)->id << "' forms the start of a new jam" << std::endl;
    //                 }
    // #endif
    //                 // yep, yep, yep - it's a new one...
    //                 //  close the frist, build a new
    //                 jams.push_back(currentJam);
    //                 currentJam = new JamInfo();
    //                 currentJam->firstStandingVehicle = mni;
    //             }
    //         }
    //         currentJam->lastStandingVehicle = mni;
    //     } else {
    //         // the vehicle is not part of a jam...
    //         //  maybe we have to close an already computed jam
    //         if (currentJam != nullptr) {
    // #ifdef DEBUG_E2_JAMS
    //             if (DEBUG_COND) {
    //                 std::cout << SIMTIME << " Closing current jam." << std::endl;
    //             }
    // #endif
    //             jams.push_back(currentJam);
    //             currentJam = nullptr;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_jam(&mut self, isInJam: bool, mni: usize, currentJam: &mut Ptr<MSE2Collector_JamInfo>, jams: &mut Vec<Ptr<MSE2Collector_JamInfo>>) {
        if isInJam {
            // The vehicle is in a jam;
            //  it may be a new one or already an existing one
            if currentJam.is_null() {
                // the vehicle is the first vehicle in a jam
                *currentJam = Ptr::from_raw(Box::into_raw(Box::new(MSE2Collector_JamInfo { firstStandingVehicle: mni, lastStandingVehicle: mni })));
            } else {
                // ok, we have a jam already. But - maybe it is too far away
                //  ... honestly, I can hardly find a reason for doing this,
                //  but jams were defined this way in an earlier version...
                let lastVeh = self.myMoveNotifications[currentJam.lastStandingVehicle];
                let currVeh = self.myMoveNotifications[mni];
                if lastVeh.distToDetectorEnd - currVeh.distToDetectorEnd > self.myJamDistanceThreshold {
                    // yep, yep, yep - it's a new one...
                    //  close the frist, build a new
                    jams.push(*currentJam);
                    *currentJam = Ptr::from_raw(Box::into_raw(Box::new(MSE2Collector_JamInfo { firstStandingVehicle: mni, lastStandingVehicle: mni })));
                }
            }
            currentJam.m().lastStandingVehicle = mni;
        } else {
            // the vehicle is not part of a jam...
            //  maybe we have to close an already computed jam
            if !currentJam.is_null() {
                jams.push(*currentJam);
                *currentJam = Ptr::null();
            }
        }
    }

    /// C++: `void MSE2Collector::processJams(std::vector<JamInfo *> &, JamInfo *)` [private]
    /// Calculates aggregated values from the given jam structure, deletes all jam-pointers
    // void
    // MSE2Collector::processJams(std::vector<JamInfo*>& jams, JamInfo* currentJam) {
    //     // push last jam
    //     if (currentJam != nullptr) {
    //         jams.push_back(currentJam);
    //         currentJam = nullptr;
    //     }
    //
    // #ifdef DEBUG_E2_JAMS
    //     if (DEBUG_COND) {
    //         std::cout << "\n" << SIMTIME << " processJams()"
    //                   << "\nNumber of jams: " << jams.size() << std::endl;
    //     }
    // #endif
    //
    //     // process jam information
    //     myCurrentMaxJamLengthInMeters = 0;
    //     myCurrentMaxJamLengthInVehicles = 0;
    //     myCurrentJamLengthInMeters = 0;
    //     myCurrentJamLengthInVehicles = 0;
    //     myCurrentJamDuration = 0;
    //     for (std::vector<JamInfo*>::const_iterator i = jams.begin(); i != jams.end(); ++i) {
    //         // compute current jam's values
    //         MoveNotificationInfo* lastVeh = *((*i)->lastStandingVehicle);
    //         MoveNotificationInfo* firstVeh = *((*i)->firstStandingVehicle);
    //         const double jamLengthInMeters = MAX2(lastVeh->distToDetectorEnd, 0.) -
    //                                          MAX2(firstVeh->distToDetectorEnd, 0.) +
    //                                          lastVeh->lengthOnDetector;
    //         const int jamLengthInVehicles = (int) distance((*i)->firstStandingVehicle, (*i)->lastStandingVehicle) + 1;
    //         // apply them to the statistics
    //         myCurrentMaxJamLengthInMeters = MAX2(myCurrentMaxJamLengthInMeters, jamLengthInMeters);
    //         myCurrentMaxJamLengthInVehicles = MAX2(myCurrentMaxJamLengthInVehicles, jamLengthInVehicles);
    //         myJamLengthInMetersSum += jamLengthInMeters;
    //         myJamLengthInVehiclesSum += jamLengthInVehicles;
    //         myCurrentJamLengthInMeters += jamLengthInMeters;
    //         myCurrentJamLengthInVehicles += jamLengthInVehicles;
    //         myCurrentJamDuration = MAX2(myCurrentJamDuration, firstVeh->waitingTime);
    // #ifdef DEBUG_E2_JAMS
    //         if (DEBUG_COND) {
    //             std::cout << SIMTIME << " processing jam nr." << ((int) distance((std::vector<JamInfo*>::const_iterator) jams.begin(), i) + 1)
    //                       << "\njamLengthInMeters = " << jamLengthInMeters
    //                       << " jamLengthInVehicles = " << jamLengthInVehicles
    //                       << std::endl;
    //         }
    // #endif
    //     }
    //     myCurrentJamNo = (int) jams.size();
    //
    //     // clean up jam structure
    //     for (std::vector<JamInfo*>::iterator i = jams.begin(); i != jams.end(); ++i) {
    //         delete *i;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn process_jams(&mut self, jams: &mut Vec<Ptr<MSE2Collector_JamInfo>>, currentJam: Ptr<MSE2Collector_JamInfo>) {
        // push last jam
        if !currentJam.is_null() {
            jams.push(currentJam);
        }

        // process jam information
        self.myCurrentMaxJamLengthInMeters = 0.0;
        self.myCurrentMaxJamLengthInVehicles = 0;
        self.myCurrentJamLengthInMeters = 0.0;
        self.myCurrentJamLengthInVehicles = 0;
        self.myCurrentJamDuration = 0.0;
        for i in jams.iter() {
            // compute current jam's values
            let lastVeh = self.myMoveNotifications[i.lastStandingVehicle];
            let firstVeh = self.myMoveNotifications[i.firstStandingVehicle];
            let jamLengthInMeters = max2(lastVeh.distToDetectorEnd, 0.0) - max2(firstVeh.distToDetectorEnd, 0.0) + lastVeh.lengthOnDetector;
            let jamLengthInVehicles = (i.lastStandingVehicle as i64 - i.firstStandingVehicle as i64) as i32 + 1;
            // apply them to the statistics
            self.myCurrentMaxJamLengthInMeters = max2(self.myCurrentMaxJamLengthInMeters, jamLengthInMeters);
            self.myCurrentMaxJamLengthInVehicles = std::cmp::max(self.myCurrentMaxJamLengthInVehicles, jamLengthInVehicles);
            self.myJamLengthInMetersSum += jamLengthInMeters;
            self.myJamLengthInVehiclesSum += jamLengthInVehicles;
            self.myCurrentJamLengthInMeters += jamLengthInMeters;
            self.myCurrentJamLengthInVehicles += jamLengthInVehicles;
            self.myCurrentJamDuration = max2(self.myCurrentJamDuration, firstVeh.waitingTime);
        }
        self.myCurrentJamNo = jams.len() as i32;

        // clean up jam structure
        for j in jams.drain(..) {
            drop(unsafe { Box::from_raw(j.as_raw()) });
        }
    }

    /// C++: `void MSE2Collector::calculateTimeLossAndTimeOnDetector(const SUMOTrafficObject &, double, double, const VehicleInfo &, double &, double &) const` [const, private]
    /// Calculates the time spent on the detector in the last step and the timeloss suffered in the last step for the given vehicle
    // void
    // MSE2Collector::calculateTimeLossAndTimeOnDetector(const SUMOTrafficObject& veh, double oldPos, double newPos, const VehicleInfo& vi, double& timeOnDetector, double& timeLoss) const {
    //     assert(veh.getID() == vi.id);
    //     assert(newPos + vi.entryOffset >= 0);
    //
    //     if (oldPos == newPos) {
    //         // vehicle is stopped
    //         timeLoss = TS;
    //         timeOnDetector = TS;
    //         return;
    //     }
    //
    //     // Eventual positional offset of the detector start from the lane's start
    //     double entryPos = MAX2(-vi.entryOffset, 0.);
    //     // Time of this vehicle entering the detector in the last time step
    //     double entryTime = 0;
    //     // Take into account the time before entering the detector, if there is.
    //     if (oldPos < entryPos) {
    //         // Vehicle entered the detector in the last step, either traversing the detector start or somewhere in the middle.
    //         entryTime = MSCFModel::passingTime(oldPos, entryPos, newPos, veh.getPreviousSpeed(), veh.getSpeed());
    //     }
    //     // speed at detector entry
    //     double entrySpeed = MSCFModel::speedAfterTime(entryTime, veh.getPreviousSpeed(), newPos - oldPos);
    //     // Calculate time spent on detector until reaching newPos or a detector exit
    //     double exitPos = MIN2(newPos, -vi.exitOffset + vi.length);
    //     assert(entryPos < exitPos);
    //
    //     // calculate vehicle's time spent on the detector
    //     double exitTime;
    //     if (exitPos == newPos) {
    //         exitTime  = TS;
    //     } else {
    //         exitTime = MSCFModel::passingTime(oldPos, exitPos, newPos, veh.getPreviousSpeed(), veh.getSpeed());
    //     }
    //
    //     // Vehicle's Speed when leaving the detector
    //     double exitSpeed = MSCFModel::speedAfterTime(exitTime, veh.getPreviousSpeed(), newPos - oldPos);
    //
    //     // Maximal speed on vehicle's current lane (== lane before last time step)
    //     // Note: this disregards the possibility of different maximal speeds on different traversed lanes.
    //     //       (we accept this as discretization error)
    //     double vmax = MAX2(veh.getLane()->getVehicleMaxSpeed(&veh), NUMERICAL_EPS);
    //
    //     // Time loss suffered on the detector
    //     timeOnDetector = exitTime - entryTime;
    //     timeLoss = MAX2(0., timeOnDetector * (vmax - (entrySpeed + exitSpeed) / 2) / vmax);
    //
    // #ifdef DEBUG_E2_TIME_ON_DETECTOR
    //     if (DEBUG_COND) {
    //         std::cout << SIMTIME << " calculateTimeLoss() for vehicle '" << veh.getID() << "'"
    //                   << " oldPos = " << oldPos << " newPos = " << newPos
    //                   << " entryPos = " << entryPos << " exitPos = " << exitPos
    //                   << " timeOnDetector = " << timeOnDetector
    //                   << " timeLoss = " << timeLoss
    //                   << std::endl;
    //     }
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn calculate_time_loss_and_time_on_detector(&self, veh: &SUMOTrafficObject, oldPos: f64, newPos: f64, vi: &MSE2Collector_VehicleInfo, timeOnDetector: &mut f64, timeLoss: &mut f64) {
        // Assumes that the vehicle is not stopped if oldPos == newPos is false
        if oldPos == newPos {
            // vehicle is stopped
            *timeLoss = ts();
            *timeOnDetector = ts();
            return;
        }

        // Eventual positional offset of the detector start from the lane's start
        let entryPos = max2(-vi.entryOffset, 0.0);
        // Time of this vehicle entering the detector in the last time step
        let mut entryTime = 0.0;
        // Take into account the time before entering the detector, if there is.
        if oldPos < entryPos {
            // Vehicle entered the detector in the last step, either traversing the detector start or somewhere in the middle.
            entryTime = MSCFModel::passing_time(oldPos, entryPos, newPos, veh.get_previous_speed(), veh.get_speed());
        }
        // speed at detector entry
        let entrySpeed = MSCFModel::speed_after_time(entryTime, veh.get_previous_speed(), newPos - oldPos);
        // Calculate time spent on detector until reaching newPos or a detector exit
        let exitPos = min2(newPos, -vi.exitOffset + vi.length);

        // calculate vehicle's time spent on the detector
        let exitTime = if exitPos == newPos {
            ts()
        } else {
            MSCFModel::passing_time(oldPos, exitPos, newPos, veh.get_previous_speed(), veh.get_speed())
        };

        // Vehicle's Speed when leaving the detector
        let exitSpeed = MSCFModel::speed_after_time(exitTime, veh.get_previous_speed(), newPos - oldPos);

        // Maximal speed on vehicle's current lane (== lane before last time step)
        // Note: this disregards the possibility of different maximal speeds on different traversed lanes.
        //       (we accept this as discretization error)
        let vmax = max2(veh.get_lane().get_vehicle_max_speed(Ptr::new(veh)), NUMERICAL_EPS);

        // Time loss suffered on the detector
        *timeOnDetector = exitTime - entryTime;
        *timeLoss = max2(0.0, *timeOnDetector * (vmax - (entrySpeed + exitSpeed) / 2.0) / vmax);
    }

    /// C++: `void MSE2Collector::initAuxiliaries(std::vector<MSLane *> &)` [private]
    /// Checks integrity of myLanes, adds internal-lane information, inits myLength, myFirstLane, myLastLane, myOffsets Called once at construction. myLanes should form a continuous sequence.
    // void
    // MSE2Collector::initAuxiliaries(std::vector<MSLane*>& lanes) {
    //     // Checks integrity of myLanes, adds internal-lane information, inits myLength, myFirstLane, myLastLane, myOffsets, myEndPos/myStartPos
    //     myFirstLane = lanes[0];
    //     myLastLane = lanes[lanes.size() - 1];
    //
    // #ifdef DEBUG_E2_CONSTRUCTOR
    //     if (DEBUG_COND) {
    //         std::cout << "\n" << "Initializing auxiliaries:"
    //                   << "\nFirst lane: " << myFirstLane->getID() << " (startPos = " << myStartPos << ")"
    //                   << "\nLast lane: " << myLastLane->getID() << " (endPos = " << myEndPos << ")"
    //                   << std::endl;
    //     }
    // #endif
    //
    //     // Init myOffsets and myDetectorLength.
    //     // The loop below runs through the given lanes assuming the possibility that only non-internal lanes are given
    //     // or at least not all relevant internal lanes are considered. During this a new, complete list of lane ids is
    //     // built into myLanes.
    //     myLanes.clear();
    //
    //     // myDetectorLength will be increased in the loop below, always giving
    //     // the offset of the currently considered lane to the detector start
    //     myDetectorLength = -myStartPos;
    //     myOffsets.clear();
    //
    //     // loop over detector lanes and accumulate offsets with respect to the first lane's begin
    //     // (these will be corrected afterwards by subtracting the start position.)
    //     std::vector<MSLane*>::iterator il = lanes.begin();
    //
    //     // start on an internal lane?
    //     // (This may happen if specifying the detector by its upstream
    //     //  length starting from a given end position)
    //     const MSLane* internal = (*il)->isInternal() ? *il : nullptr;
    //
    // #ifdef DEBUG_E2_CONSTRUCTOR
    //     if (DEBUG_COND) {
    //         std::cout << "\n" << "Initializing offsets:" << std::endl;
    //     }
    // #endif
    //
    //     while (true) {
    //         // Consider the next internal lanes
    //         while (internal != nullptr) {
    //             myLanes.push_back(internal->getID());
    //             myOffsets.push_back(myDetectorLength);
    //
    // #ifdef DEBUG_E2_CONSTRUCTOR
    //             if (DEBUG_COND) {
    //                 std::cout << "Offset for lane " << internal->getID() << " = " << myDetectorLength
    //                           << std::endl;
    //             }
    // #endif
    //
    //             myDetectorLength += internal->getLength();
    //             if (internal->getID() == myLastLane->getID()) {
    //                 break;
    //             }
    //
    //             // There should be a unique continuation for each internal lane
    //             assert(internal->getLinkCont().size() == 1);
    //
    //             internal = internal->getLinkCont()[0]->getViaLaneOrLane();
    //             if (!internal->isInternal()) {
    //                 // passed the junction
    //                 internal = nullptr;
    //                 break;
    //             }
    //         }
    //
    //         // Consider the next non-internal lane
    //         // This is the first lane in the first iteration, if it is non-internal
    //         // However, it can equal myLanes.end() if the myLastLane is internal. In that case we break.
    //
    //         // Move il to next non-internal
    //         while (il != lanes.end() && (*il)->isInternal()) {
    //             il++;
    //         }
    //         if (il == lanes.end()) {
    //             break;
    //         }
    //
    //         // There is still a non-internal lane to consider
    //         MSLane* lane = *il;
    //         myLanes.push_back(lane->getID());
    //
    // #ifdef DEBUG_E2_CONSTRUCTOR
    //         if (DEBUG_COND) {
    //             std::cout << "Offset for lane " << lane->getID() << " = " << myDetectorLength
    //                       << std::endl;
    //         }
    // #endif
    //
    //         // Offset to detector start for this lane
    //         myOffsets.push_back(myDetectorLength);
    //
    //         // Add the lanes length to the detector offset
    //         myDetectorLength += lane->getLength();
    //
    //         // Get the next lane if this lane isn't the last one
    //         if (++il == lanes.end()) {
    //             break;
    //         }
    //
    //         if ((*il)->isInternal()) {
    //             // next lane in myLanes is internal
    //             internal = *il;
    //             continue;
    //         }
    //
    //         // find the connection to next
    //         const MSLink* link = lane->getLinkTo(*il);
    //         if (link == nullptr) {
    //             throw InvalidArgument("Lanes '" + lane->getID() + "' and '" + (*il)->getID() + "' are not consecutive in definition of e2Detector '" + getID() + "'");
    //         }
    //
    //         if (!MSGlobals::gUsingInternalLanes) {
    //             myDetectorLength += link->getLength();
    //         } else {
    //             internal = link->getViaLane();
    //         }
    //     }
    //
    //     // Subtract distance not covered on the last considered lane
    //     bool fw = myEndPos == std::numeric_limits<double>::max();
    //     if (fw) {
    //         myDetectorLength -= myStartPos;
    //     } else {
    //         myDetectorLength -= myLastLane->getLength() - myEndPos;
    //     }
    //
    // #ifdef DEBUG_E2_CONSTRUCTOR
    //     if (DEBUG_COND) {
    //         std::cout << "Total detector length after initAuxiliaries() = " << myDetectorLength << std::endl;
    //     }
    // #endif
    //
    // // make lanes a complete list including internal lanes
    //     lanes = getLanes();
    // }
    #[allow(unused_variables)]
    pub fn init_auxiliaries(&mut self, lanes: &mut Vec<Ptr<MSLane>>) -> Result<(), ProcessError> {
        // Checks integrity of myLanes, adds internal-lane information, inits myLength, myFirstLane, myLastLane, myOffsets, myEndPos/myStartPos
        self.myFirstLane = lanes[0];
        self.myLastLane = lanes[lanes.len() - 1];

        // Init myOffsets and myDetectorLength.
        // The loop below runs through the given lanes assuming the possibility that only non-internal lanes are given
        // or at least not all relevant internal lanes are considered. During this a new, complete list of lane ids is
        // built into myLanes.
        self.myLanes.clear();

        // myDetectorLength will be increased in the loop below, always giving
        // the offset of the currently considered lane to the detector start
        self.myDetectorLength = -self.myStartPos;
        self.myOffsets.clear();

        // loop over detector lanes and accumulate offsets with respect to the first lane's begin
        // (these will be corrected afterwards by subtracting the start position.)
        let mut il: usize = 0;

        // start on an internal lane?
        // (This may happen if specifying the detector by its upstream
        //  length starting from a given end position)
        let mut internal: Ptr<MSLane> = if lanes[il].is_internal() { lanes[il] } else { Ptr::null() };

        loop {
            // Consider the next internal lanes
            while !internal.is_null() {
                self.myLanes.push(internal.get_id().to_string());
                self.myOffsets.push(self.myDetectorLength);

                self.myDetectorLength += internal.get_length();
                if internal.get_id() == self.myLastLane.get_id() {
                    break;
                }

                // There should be a unique continuation for each internal lane
                internal = internal.get_link_cont()[0].get_via_lane_or_lane();
                if !internal.is_internal() {
                    // passed the junction
                    internal = Ptr::null();
                    break;
                }
            }

            // Consider the next non-internal lane
            // This is the first lane in the first iteration, if it is non-internal
            // However, it can equal myLanes.end() if the myLastLane is internal. In that case we break.

            // Move il to next non-internal
            while il < lanes.len() && lanes[il].is_internal() {
                il += 1;
            }
            if il == lanes.len() {
                break;
            }

            // There is still a non-internal lane to consider
            let lane = lanes[il];
            self.myLanes.push(lane.get_id().to_string());

            // Offset to detector start for this lane
            self.myOffsets.push(self.myDetectorLength);

            // Add the lanes length to the detector offset
            self.myDetectorLength += lane.get_length();

            // Get the next lane if this lane isn't the last one
            il += 1;
            if il == lanes.len() {
                break;
            }

            if lanes[il].is_internal() {
                // next lane in myLanes is internal
                internal = lanes[il];
                continue;
            }

            // find the connection to next
            let link = lane.get_link_to(lanes[il]);
            if link.is_null() {
                return Err(ProcessError::new(format!("Lanes '{}' and '{}' are not consecutive in definition of e2Detector '{}'", lane.get_id(), lanes[il].get_id(), self.base2.get_id())));
            }

            if !MSGlobals::using_internal_lanes() {
                self.myDetectorLength += link.get_length();
            } else {
                internal = link.get_via_lane();
            }
        }

        // Subtract distance not covered on the last considered lane
        let fw = self.myEndPos == f64::MAX;
        if fw {
            self.myDetectorLength -= self.myStartPos;
        } else {
            self.myDetectorLength -= self.myLastLane.get_length() - self.myEndPos;
        }

        // make lanes a complete list including internal lanes
        *lanes = self.get_lanes();
        Ok(())
    }

    /// C++: `void MSE2Collector::checkPositioning(bool, double)` [private]
    /// Adjusts positioning if the detector length is less than POSITION_EPS and tests some assertions
    // void
    // MSE2Collector::checkPositioning(bool posGiven, double desiredLength) {
    //     // check if detector was truncated
    //     if (desiredLength > 0 && myDetectorLength < desiredLength - NUMERICAL_EPS) {
    //         std::stringstream ss;
    //         ss << "Cannot build detector of length " << desiredLength
    //            << " because no further continuation lane was found for lane '" << (posGiven ? myLastLane->getID() : myFirstLane->getID())
    //            << "'! Truncated detector at length " << myDetectorLength << ".";
    //         WRITE_WARNING(ss.str());
    //     }
    //
    //     if (myDetectorLength < POSITION_EPS && (myStartPos > 0. || myEndPos < myLastLane->getLength())) {
    //         // assure minimal detector length
    //         double prolong = POSITION_EPS - myDetectorLength;
    //         double startPos = MAX2(0., myStartPos - prolong); // new startPos
    //         prolong -= myStartPos - startPos;
    //         myStartPos = startPos;
    //         if (prolong > 0.) {
    //             myEndPos = MIN2(myEndPos + prolong, myLastLane->getLength());
    //         }
    //         WRITE_WARNING("Adjusted detector positioning to meet requirement length >= " + toString(POSITION_EPS)
    //                       + ". New position is [" + toString(myStartPos) + "," + toString(myEndPos) + "]");
    //     }
    //
    //     // do some regularization snapping...
    //     myStartPos = snap(myStartPos, 0., POSITION_EPS);
    //     myStartPos = snap(myStartPos, myFirstLane->getLength() - POSITION_EPS, POSITION_EPS);
    //     myStartPos = snap(myStartPos, 0., POSITION_EPS);
    //     myEndPos = snap(myEndPos, myLastLane->getLength(), POSITION_EPS);
    //     myEndPos = snap(myEndPos, POSITION_EPS, POSITION_EPS);
    //     myEndPos = snap(myEndPos, myLastLane->getLength(), POSITION_EPS);
    //     recalculateDetectorLength();
    //
    // #ifdef DEBUG_E2_CONSTRUCTOR
    //     if (DEBUG_COND) {
    //         std::stringstream ss;
    //         //    ss << std::setprecision(32) << myEndPos << " : " << POSITION_EPS;
    //         //    std::cout << ss.str() << std::endl;
    //         std::cout << "myStartPos = " << myStartPos << std::endl;
    //         std::cout << "myEndPos = " << myEndPos << std::endl;
    //         std::cout << "myLastLane->getLength() = " << myLastLane->getLength() << std::endl;
    //     }
    // #endif
    //
    //
    //     assert((myStartPos >= POSITION_EPS  || myStartPos == 0) && myStartPos < myFirstLane->getLength());
    //     assert(myEndPos <= myLastLane->getLength() - POSITION_EPS || myEndPos == myLastLane->getLength());
    //     assert(myFirstLane != myLastLane || myEndPos - myStartPos > 0);
    // }
    #[allow(unused_variables)]
    pub fn check_positioning(&mut self, posGiven: bool, desiredLength: f64) {
        // check if detector was truncated
        if desiredLength > 0.0 && self.myDetectorLength < desiredLength - NUMERICAL_EPS {
            let laneID = if posGiven { self.myLastLane.get_id().to_string() } else { self.myFirstLane.get_id().to_string() };
            write_warning(&format!("Cannot build detector of length {} because no further continuation lane was found for lane '{}'! Truncated detector at length {}.", utils::common::to_string::f64_to_string_default(desiredLength), laneID, utils::common::to_string::f64_to_string_default(self.myDetectorLength)));
        }

        if self.myDetectorLength < POSITION_EPS && (self.myStartPos > 0.0 || self.myEndPos < self.myLastLane.get_length()) {
            // assure minimal detector length
            let mut prolong = POSITION_EPS - self.myDetectorLength;
            let startPos = max2(0.0, self.myStartPos - prolong); // new startPos
            prolong -= self.myStartPos - startPos;
            self.myStartPos = startPos;
            if prolong > 0.0 {
                self.myEndPos = min2(self.myEndPos + prolong, self.myLastLane.get_length());
            }
            write_warning(&format!("Adjusted detector positioning to meet requirement length >= {}. New position is [{},{}]", utils::common::to_string::to_string(&POSITION_EPS), utils::common::to_string::to_string(&self.myStartPos), utils::common::to_string::to_string(&self.myEndPos)));
        }

        // do some regularization snapping...
        self.myStartPos = Self::snap(self.myStartPos, 0.0, POSITION_EPS);
        self.myStartPos = Self::snap(self.myStartPos, self.myFirstLane.get_length() - POSITION_EPS, POSITION_EPS);
        self.myStartPos = Self::snap(self.myStartPos, 0.0, POSITION_EPS);
        self.myEndPos = Self::snap(self.myEndPos, self.myLastLane.get_length(), POSITION_EPS);
        self.myEndPos = Self::snap(self.myEndPos, POSITION_EPS, POSITION_EPS);
        self.myEndPos = Self::snap(self.myEndPos, self.myLastLane.get_length(), POSITION_EPS);
        self.recalculate_detector_length();
    }

    /// C++: `double MSE2Collector::snap(double, double, double)` [static, private]
    /// Snaps value to snpPoint if they are closer than snapDist
    // double
    // MSE2Collector::snap(double value, double snapPoint, double snapDist) {
    //     if (fabs(value - snapPoint) < snapDist) {
    //         return snapPoint;
    //     } else {
    //         return value;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn snap(value: f64, snapPoint: f64, snapDist: f64) -> f64 {
        if (value - snapPoint).abs() < snapDist {
            snapPoint
        } else {
            value
        }
    }

    /// C++: `void MSE2Collector::recalculateDetectorLength()` [private]
    /// Updates the detector length after myStartPos and myEndPos have been modified
    // void
    // MSE2Collector::recalculateDetectorLength() {
    //     std::vector<std::string>::const_iterator i;
    //     std::vector<MSLane*> lanes;
    //     // get real lanes
    //     for (i = myLanes.begin(); i != myLanes.end(); ++i) {
    //         MSLane* lane = MSLane::dictionary(*i);
    //         lanes.push_back(lane);
    //     }
    //
    //     // sum up their lengths
    //     std::vector<MSLane*>::const_iterator j;
    //     MSLane* previous = nullptr;
    //     myDetectorLength = 0;
    //     for (j = lanes.begin(); j != lanes.end(); ++j) {
    //         // lane length
    //         myDetectorLength += (*j)->getLength();
    //         if (previous != nullptr && !MSGlobals::gUsingInternalLanes) {
    //             // eventually link length
    //             myDetectorLength += previous->getLinkTo(*j)->getLength();
    //         }
    //         previous = *j;
    //     }
    //     // subtract uncovered area on first and last lane
    //     myDetectorLength -= myStartPos;
    //     myDetectorLength -= myLastLane->getLength() - myEndPos;
    //
    // #ifdef DEBUG_E2_CONSTRUCTOR
    //     if (DEBUG_COND) {
    //         std::cout << "Total detector length after recalculation = " << myDetectorLength << std::endl;
    //     }
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn recalculate_detector_length(&mut self) {
        // get real lanes
        let lanes: Vec<Ptr<MSLane>> = self.myLanes.iter().map(|id| MSLane::dictionary_2(id)).collect();

        // sum up their lengths
        let mut previous: Ptr<MSLane> = Ptr::null();
        self.myDetectorLength = 0.0;
        for j in lanes.iter() {
            // lane length
            self.myDetectorLength += j.get_length();
            if !previous.is_null() && !MSGlobals::using_internal_lanes() {
                // eventually link length
                self.myDetectorLength += previous.get_link_to(*j).get_length();
            }
            previous = *j;
        }
        // subtract uncovered area on first and last lane
        self.myDetectorLength -= self.myStartPos;
        self.myDetectorLength -= self.myLastLane.get_length() - self.myEndPos;
    }

    /// C++: `std::vector<MSLane *> MSE2Collector::selectLanes(MSLane *, double, std::string)` [private]
    /// This is called if no lane sequence is given to the constructor. Builds myLanes from the given information. Also inits startPos (case dir=="bw") / endPos (case dir=="fw"). Selects lanes heuristically if no unambiguous continuation exists.
    // std::vector<MSLane*>
    // MSE2Collector::selectLanes(MSLane* lane, double length, std::string dir) {
    //     // direction of detector extension
    //     assert(dir == "fw" || dir == "bw");
    //     bool fw = dir == "fw";
    //     double linkLength = 0; // linkLength (used if no internal lanes are present)
    //     bool subtractedLinkLength = false; // whether linkLength was subtracted during the last iteration.
    //
    // #ifdef DEBUG_E2_CONSTRUCTOR
    //     if (DEBUG_COND) {
    //         std::cout << "\n" << "selectLanes()" << (fw ? "(forward)" : "(backward)") << std::endl;
    //     }
    // #endif
    //     std::vector<MSLane*> lanes;
    //     // Selected lanes are stacked into vector 'lanes'. If dir == "bw" lanes will be reversed after this is done.
    //     // The length is reduced while adding lanes to the detector
    //     // First we adjust the starting value for length (in the first iteration, the whole length of the first considered lane is subtracted,
    //     // while it might only be partially covered by the detector)
    //     if (fw) {
    //         assert(myStartPos != std::numeric_limits<double>::max());
    //         length += myStartPos;
    //     } else {
    //         assert(myEndPos != std::numeric_limits<double>::max());
    //         length += lane->getLength() - myEndPos;
    //     }
    //     length = MAX2(POSITION_EPS, length); // this assures to add at least one lane to lanes
    //     while (length >= POSITION_EPS && lane != nullptr) {
    //         // Break loop for length <= NUMERICAL_EPS to avoid placement of very small
    //         // detector piece on the end or beginning of one lane due to numerical rounding errors.
    //         lanes.push_back(lane);
    // #ifdef DEBUG_E2_CONSTRUCTOR
    //         if (DEBUG_COND) {
    //             std::cout << "Added lane " << lane->getID()
    //                       << " (length: " << lane->getLength() << ")" << std::endl;
    //         }
    // #endif
    //
    //         length -= lane->getLength();
    //
    //         // proceed to upstream predecessor
    //         if (fw) {
    //             lane = lane->getCanonicalSuccessorLane();
    //         } else {
    //             lane = lane->getCanonicalPredecessorLane();
    //         }
    //
    //
    //         subtractedLinkLength = false;
    //         if (lane != nullptr && !MSGlobals::gUsingInternalLanes && length > POSITION_EPS) {
    //             // In case wher no internal lanes are used,
    //             // take into account the link length for the detector range
    //             linkLength = 0;
    //             if (fw) {
    //                 linkLength = lanes.back()->getLinkTo(lane)->getLength();
    //             } else {
    //                 linkLength = lane->getLinkTo(lanes.back())->getLength();
    //             }
    //             length -= linkLength;
    //             subtractedLinkLength = true;
    //         }
    //
    //
    // #ifdef DEBUG_E2_CONSTRUCTOR
    //         if (DEBUG_COND) {
    //             if (lane != 0) {
    //                 std::cout << (fw ? "Successor lane: " : "Predecessor lane: ") << "'" << lane->getID() << "'";
    //             }
    //             std::cout << std::endl;
    //         }
    // #endif
    //     }
    //
    //     if (subtractedLinkLength) {
    //         // if the link's length was subtracted during the last step,
    //         // the start/endPos would lie on a non-existing internal lane,
    //         // therefore revert and truncate detector part on the non-existing internal lane.
    //         length += linkLength;
    //     }
    //
    //
    //     // 1) At this point a negative <length> means that not the whole last stored lane lanes[lanes.size()-1]
    //     // should be added to the detector, but the detector should spare out a part with length = -<length>
    //     // If this part is too small (of length < POSITION_EPS) we take the whole lane
    //     // 2) The whole lane is also taken for the case that <length> is positive. This corresponds to on
    //     // of three situations: i) <length> < POSITION_EPS (break condition -> don't take too small pieces on the next lane)
    //     //                 ii&iii) <length> >= POS_EPSILON may arise either if no continuation lane was found (lane==0), or
    //     //                         in case of not using internal lanes if the detector end/start falls on a link.
    //     // In all cases we take the whole last lane.
    //     if (fw) {
    //         if (length > -POSITION_EPS) {
    //             myEndPos = lanes[lanes.size() - 1]->getLength();
    //         } else if (length < 0) {
    //             myEndPos = lanes[lanes.size() - 1]->getLength() + length;
    //         }
    //     } else {
    //         if (length > -POSITION_EPS) {
    //             myStartPos = 0;
    //         } else if (length < 0) {
    //             myStartPos = -length;
    //         }
    //     }
    //
    //     // reverse lanes if lane selection was backwards
    //     if (!fw) {
    //         std::reverse(lanes.begin(), lanes.end());
    //     }
    //
    //     return lanes;
    // }
    #[allow(unused_variables)]
    pub fn select_lanes(&mut self, endLane: Ptr<MSLane>, length: f64, dir: String) -> Vec<Ptr<MSLane>> {
        // direction of detector extension
        let fw = dir == "fw";
        let mut linkLength = 0.0; // linkLength (used if no internal lanes are present)
        let mut subtractedLinkLength = false; // whether linkLength was subtracted during the last iteration.

        let mut lane = endLane;
        let mut length = length;
        let mut lanes: Vec<Ptr<MSLane>> = Vec::new();
        // Selected lanes are stacked into vector 'lanes'. If dir == "bw" lanes will be reversed after this is done.
        // The length is reduced while adding lanes to the detector
        // First we adjust the starting value for length (in the first iteration, the whole length of the first considered lane is subtracted,
        // while it might only be partially covered by the detector)
        if fw {
            length += self.myStartPos;
        } else {
            length += lane.get_length() - self.myEndPos;
        }
        length = max2(POSITION_EPS, length); // this assures to add at least one lane to lanes
        while length >= POSITION_EPS && !lane.is_null() {
            // Break loop for length <= NUMERICAL_EPS to avoid placement of very small
            // detector piece on the end or beginning of one lane due to numerical rounding errors.
            lanes.push(lane);

            length -= lane.get_length();

            // proceed to upstream predecessor
            if fw {
                lane = lane.get_canonical_successor_lane();
            } else {
                lane = lane.get_canonical_predecessor_lane();
            }

            subtractedLinkLength = false;
            if !lane.is_null() && !MSGlobals::using_internal_lanes() && length > POSITION_EPS {
                // In case wher no internal lanes are used,
                // take into account the link length for the detector range
                linkLength = 0.0;
                let lastAdded = *lanes.last().unwrap();
                if fw {
                    linkLength = lastAdded.get_link_to(lane).get_length();
                } else {
                    linkLength = lane.get_link_to(lastAdded).get_length();
                }
                length -= linkLength;
                subtractedLinkLength = true;
            }
        }

        if subtractedLinkLength {
            // if the link's length was subtracted during the last step,
            // the start/endPos would lie on a non-existing internal lane,
            // therefore revert and truncate detector part on the non-existing internal lane.
            length += linkLength;
        }

        // 1) At this point a negative <length> means that not the whole last stored lane lanes[lanes.size()-1]
        // should be added to the detector, but the detector should spare out a part with length = -<length>
        // If this part is too small (of length < POSITION_EPS) we take the whole lane
        // 2) The whole lane is also taken for the case that <length> is positive. This corresponds to on
        // of three situations: i) <length> < POSITION_EPS (break condition -> don't take too small pieces on the next lane)
        //                 ii&iii) <length> >= POS_EPSILON may arise either if no continuation lane was found (lane==0), or
        //                         in case of not using internal lanes if the detector end/start falls on a link.
        // In all cases we take the whole last lane.
        let lastLaneLength = lanes.last().unwrap().get_length();
        if fw {
            if length > -POSITION_EPS {
                self.myEndPos = lastLaneLength;
            } else if length < 0.0 {
                self.myEndPos = lastLaneLength + length;
            }
        } else {
            if length > -POSITION_EPS {
                self.myStartPos = 0.0;
            } else if length < 0.0 {
                self.myStartPos = -length;
            }
        }

        // reverse lanes if lane selection was backwards
        if !fw {
            lanes.reverse();
        }

        lanes
    }

    /// C++: `void MSE2Collector::addDetectorToLanes(std::vector<MSLane *> &)` [private]
    /// This adds the detector as a MoveReminder to the associated lanes.
    // void
    // MSE2Collector::addDetectorToLanes(std::vector<MSLane*>& lanes) {
    // #ifdef DEBUG_E2_CONSTRUCTOR
    //     if (DEBUG_COND) {
    //         std::cout << "\n" << "Adding detector " << myID << " to lanes:" << std::endl;
    //     }
    // #endif
    //     for (std::vector<MSLane*>::iterator l = lanes.begin(); l != lanes.end(); ++l) {
    //         (*l)->addMoveReminder(this);
    // #ifdef DEBUG_E2_CONSTRUCTOR
    //         if (DEBUG_COND) {
    //             std::cout << (*l)->getID() << std::endl;
    //         }
    // #endif
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_detector_to_lanes(&mut self, lanes: &mut Vec<Ptr<MSLane>>) {
        for l in lanes.iter() {
            // C++ passes `this`; the registered pointer is the object's current address (see the constructor shortcut)
            l.m().add_move_reminder(Ptr::from_mut(self).cast::<MSMoveReminder>(), true);
        }
    }

    /// C++: `void MSE2Collector::aggregateOutputValues()` [private]
    /// Aggregates and normalize some values for the detector output during detectorUpdate()
    // void
    // MSE2Collector::aggregateOutputValues() {
    //     myTimeSamples += 1;
    //     // compute occupancy values (note myCurrentMeanLength is still not normalized here, but holds the sum of all vehicle lengths)
    //     const double currentOccupancy = myCurrentMeanLength / myDetectorLength * (double) 100.;
    //     myCurrentOccupancy = currentOccupancy;
    //     myOccupancySum += currentOccupancy;
    //     myMaxOccupancy = MAX2(myMaxOccupancy, currentOccupancy);
    //     // compute jam values
    //     myMeanMaxJamInVehicles += myCurrentMaxJamLengthInVehicles;
    //     myMeanMaxJamInMeters += myCurrentMaxJamLengthInMeters;
    //     myMaxJamInVehicles = MAX2(myMaxJamInVehicles, myCurrentMaxJamLengthInVehicles);
    //     myMaxJamInMeters = MAX2(myMaxJamInMeters, myCurrentMaxJamLengthInMeters);
    //     // compute information about vehicle numbers
    //     const int numVehicles = (int)myMoveNotifications.size();
    //     myMeanVehicleNumber += numVehicles;
    //     myMaxVehicleNumber = MAX2(numVehicles, myMaxVehicleNumber);
    //     // norm current values
    //     myCurrentMeanSpeed = numVehicles != 0 ? myCurrentMeanSpeed / myCurrentVehicleSamples : -1;
    //     myCurrentMeanLength = numVehicles != 0 ? myCurrentMeanLength / (double) numVehicles : -1;
    // }
    #[allow(unused_variables)]
    pub fn aggregate_output_values(&mut self) {
        self.myTimeSamples += 1;
        // compute occupancy values (note myCurrentMeanLength is still not normalized here, but holds the sum of all vehicle lengths)
        let currentOccupancy = self.myCurrentMeanLength / self.myDetectorLength * 100.0;
        self.myCurrentOccupancy = currentOccupancy;
        self.myOccupancySum += currentOccupancy;
        self.myMaxOccupancy = max2(self.myMaxOccupancy, currentOccupancy);
        // compute jam values
        self.myMeanMaxJamInVehicles += self.myCurrentMaxJamLengthInVehicles;
        self.myMeanMaxJamInMeters += self.myCurrentMaxJamLengthInMeters;
        self.myMaxJamInVehicles = std::cmp::max(self.myMaxJamInVehicles, self.myCurrentMaxJamLengthInVehicles);
        self.myMaxJamInMeters = max2(self.myMaxJamInMeters, self.myCurrentMaxJamLengthInMeters);
        // compute information about vehicle numbers
        let numVehicles = self.myMoveNotifications.len() as i32;
        self.myMeanVehicleNumber += numVehicles;
        self.myMaxVehicleNumber = std::cmp::max(numVehicles, self.myMaxVehicleNumber);
        // norm current values
        self.myCurrentMeanSpeed = if numVehicles != 0 { self.myCurrentMeanSpeed / self.myCurrentVehicleSamples } else { -1.0 };
        self.myCurrentMeanLength = if numVehicles != 0 { self.myCurrentMeanLength / numVehicles as f64 } else { -1.0 };
    }

    /// C++: `void MSE2Collector::integrateMoveNotification(VehicleInfo *, const MoveNotificationInfo *)` [private]
    /// This updates the detector values and the VehicleInfo of a vehicle on the detector with the given MoveNotificationInfo generated by the vehicle during the last time step.
    // void
    // MSE2Collector::integrateMoveNotification(VehicleInfo* vi, const MoveNotificationInfo* mni) {
    //
    // #ifdef DEBUG_E2_DETECTOR_UPDATE
    //     if (DEBUG_COND) {
    //         std::cout << SIMTIME << " integrateMoveNotification() for vehicle '" << mni->id << "'"
    //                   << "\ntimeOnDetector = " << mni->timeOnDetector
    //                   << "\nlengthOnDetector = " << mni->lengthOnDetector
    //                   << "\ntimeLoss = " << mni->timeLoss
    //                   << "\nspeed = " << mni->speed
    //                   << std::endl;
    //     }
    // #endif
    //
    // // Accumulate detector values
    //     myVehicleSamples += mni->timeOnDetector;
    //     myTotalTimeLoss += mni->timeLoss;
    //     mySpeedSum += mni->speed * mni->timeOnDetector;
    //     myCurrentVehicleSamples += mni->timeOnDetector;
    //     myCurrentMeanSpeed += mni->speed * mni->timeOnDetector;
    //     myCurrentMeanLength += mni->lengthOnDetector;
    //
    //     if (vi != nullptr) {
    //         // Accumulate individual values for the vehicle.
    //         // @note vi==0 occurs, if the vehicle info has been erased at
    //         //       notifyLeave() in case of a non-longitudinal exit (lanechange, teleport, etc.)
    //         vi->totalTimeOnDetector += mni->timeOnDetector;
    //         vi->accumulatedTimeLoss += mni->timeLoss;
    //         vi->lastAccel = mni->accel;
    //         vi->lastSpeed = mni->speed;
    //         vi->lastPos = myStartPos + vi->entryOffset + mni->newPos;
    //         vi->onDetector = mni->onDetector;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn integrate_move_notification(&mut self, vi: Ptr<MSE2Collector_VehicleInfo>, mni: Ptr<MSE2Collector_MoveNotificationInfo>) {
        // Accumulate detector values
        self.myVehicleSamples += mni.timeOnDetector;
        self.myTotalTimeLoss += mni.timeLoss;
        self.mySpeedSum += mni.speed * mni.timeOnDetector;
        self.myCurrentVehicleSamples += mni.timeOnDetector;
        self.myCurrentMeanSpeed += mni.speed * mni.timeOnDetector;
        self.myCurrentMeanLength += mni.lengthOnDetector;

        if !vi.is_null() {
            // Accumulate individual values for the vehicle.
            // @note vi==0 occurs, if the vehicle info has been erased at
            //       notifyLeave() in case of a non-longitudinal exit (lanechange, teleport, etc.)
            let vi = vi.m();
            vi.totalTimeOnDetector += mni.timeOnDetector;
            vi.accumulatedTimeLoss += mni.timeLoss;
            vi.lastAccel = mni.accel;
            vi.lastSpeed = mni.speed;
            vi.lastPos = self.myStartPos + vi.entryOffset + mni.newPos;
            vi.onDetector = mni.onDetector;
        }
    }

    /// C++: `MoveNotificationInfo * MSE2Collector::makeMoveNotification(const SUMOTrafficObject &, double, double, double, const VehicleInfo &) const` [const, private]
    /// Creates and returns a MoveNotificationInfo containing detector specific information on the vehicle's last movement
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn make_move_notification(&self, veh: &SUMOTrafficObject, oldPos: f64, newPos: f64, newSpeed: f64, vehInfo: &MSE2Collector_VehicleInfo) -> Ptr<MSE2Collector_MoveNotificationInfo> {
        // Timefraction in [0,TS] the vehicle has spend on the detector in the last step
        // Note that at this point, vehInfo.currentLane points to the lane at the beginning of the last timestep,
        // and vehInfo.enteredLanes is a list of lanes entered in the last timestep
        let mut timeOnDetector = 0.0;
        let mut timeLoss = 0.0;
        self.calculate_time_loss_and_time_on_detector(veh, oldPos, newPos, vehInfo, &mut timeOnDetector, &mut timeLoss);

        // The length of the part of the vehicle on the detector at the end of the last time step
        // may be shorter than vehicle's length if its back reaches out
        let mut lengthOnDetector = max2(min2(vehInfo.length, newPos + vehInfo.entryOffset), 0.0);

        // XXX: Fulfulling the specifications of the documentation (lengthOnDetector = time integral
        //      over length of the vehicle's part on the detector) would be much more cumbersome.
        let distToExit = -vehInfo.exitOffset - newPos;
        // Eventually decrease further to account for the front reaching out
        lengthOnDetector = max2(0.0, lengthOnDetector + min2(0.0, distToExit));

        // whether the vehicle is still on the detector at the end of the time step
        let stillOnDetector = -distToExit < vehInfo.length;

        /* Store new infos */
        Ptr::from_raw(Box::into_raw(Box::new(MSE2Collector_MoveNotificationInfo::new(
            veh.get_id().to_string(), oldPos, newPos, newSpeed, veh.get_acceleration(),
            self.myDetectorLength - (vehInfo.entryOffset + newPos),
            timeOnDetector, lengthOnDetector, timeLoss,
            steps2time(veh.get_waiting_time(false)), stillOnDetector,
        ))))
    }

    /// C++: `VehicleInfo * MSE2Collector::makeVehicleInfo(const SUMOTrafficObject &, const MSLane *) const` [const, private]
    /// Creates and returns a VehicleInfo (called at the vehicle's entry)
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn make_vehicle_info(&self, veh: &SUMOTrafficObject, enteredLane: Ptr<MSLane>) -> Ptr<MSE2Collector_VehicleInfo> {
        // The vehicle's distance to the detector end
        let j: usize = self.myLanes.iter().position(|l| l == enteredLane.get_id()).expect("entered lane is not part of the detector");
        let entryOffset = self.myOffsets[j];
        let distToDetectorEnd = self.myDetectorLength - (entryOffset + veh.get_position_on_lane());
        let onDetector = -entryOffset < veh.get_position_on_lane() && distToDetectorEnd > -veh.get_vehicle_type().get_length();

        Ptr::from_raw(Box::into_raw(Box::new(MSE2Collector_VehicleInfo::new(
            veh.get_id().to_string(),
            veh.get_vehicle_type().get_id().to_string(),
            veh.get_vehicle_type().get_length(),
            veh.get_vehicle_type().get_min_gap(),
            enteredLane,
            entryOffset,
            j,
            self.myOffsets[j] - self.myDetectorLength,
            distToDetectorEnd,
            onDetector,
        ))))
    }

    /// C++: `double MSE2Collector::calculateSegmentTimeLoss(double, double, double, double)` [static, private]
    /// Calculates the time loss for a segment with constant vmax
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn calculate_segment_time_loss(timespan: f64, initialSpeed: f64, accel: f64, vmax: f64) -> f64 {
        // shortcut: declared in SUMO but never defined (and never called), so there is no C++ behaviour to port
        panic!("MSE2Collector::calculateSegmentTimeLoss is declared in C++ but has no definition")
    }

    /// C++: `bool MSE2Collector::compareMoveNotification(MoveNotificationInfo *, MoveNotificationInfo *)` [static, private]
    /// brief returns true if the vehicle corresponding to mni1 is closer to the detector end than the vehicle corresponding to mni2
    // static bool compareMoveNotification(MoveNotificationInfo* mni1, MoveNotificationInfo* mni2) {
    //         return mni1->distToDetectorEnd < mni2->distToDetectorEnd;
    //     }
    #[allow(unused_variables)]
    pub fn compare_move_notification(mni1: Ptr<MSE2Collector_MoveNotificationInfo>, mni2: Ptr<MSE2Collector_MoveNotificationInfo>) -> bool {
        mni1.distToDetectorEnd < mni2.distToDetectorEnd
    }

    /// C++: `void MSE2Collector::notifyMovePerson(MSTransportable *, int, double)` [private]
    // void
    // MSE2Collector::notifyMovePerson(MSTransportable* p, int dir, double pos) {
    //     if (personApplies(*p, dir)) {
    //         const double newSpeed = p->getSpeed();
    //         const double newPos = (dir == MSPModel::FORWARD
    //                                ? pos
    //                                // position relative to detector end position
    //                                : myEndPos - (pos - myEndPos));
    //         const double oldPos = newPos - SPEED2DIST(newSpeed);
    //         if (oldPos - p->getVehicleType().getLength() <= myEndPos) {
    //             notifyMove(*p, oldPos, newPos, newSpeed);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn notify_move_person(&mut self, p: Ptr<MSTransportable>, dir: i32, pos: f64) {
        if self.base2.person_applies(&*p, dir) {
            let newSpeed = p.get_speed();
            let newPos = if dir == MSPMODEL_FORWARD {
                pos
            } else {
                // position relative to detector end position
                self.myEndPos - (pos - self.myEndPos)
            };
            let oldPos = newPos - speed2dist(newSpeed);
            if oldPos - p.get_vehicle_type().get_length() <= self.myEndPos {
                self.notify_move(p.m(), oldPos, newPos, newSpeed);
            }
        }
    }

    /// C++: `void MSE2Collector::MSE2Collector(const MSE2Collector &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_3(arg0: &MSE2Collector) -> Self {
        panic!("MSE2Collector copy constructor is private and invalidated in C++")
    }

    /// C++: `MSE2Collector & MSE2Collector::operator=(const MSE2Collector &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSE2Collector) -> &mut MSE2Collector {
        panic!("MSE2Collector assignment operator is private and invalidated in C++")
    }

}

/// Values collected in notifyMove and needed in detectorUpdate() to calculate the accumulated quantities for the detector. These are temporarily stored in myMoveNotifications for each step.
/// C++ class `MSE2Collector::MoveNotificationInfo` (microsim/output/MSE2Collector.h)
pub struct MSE2Collector_MoveNotificationInfo {
    /// Vehicle's id
    pub id: String,
    /// Position before the last integration step (relative to the vehicle's entry lane on the detector)
    pub oldPos: f64,
    /// Position after the last integration step (relative to the vehicle's entry lane on the detector)
    pub newPos: f64,
    /// Speed after the last integration step
    pub speed: f64,
    /// Acceleration in the last integration step
    pub accel: f64,
    /// Distance left till the detector end after the last integration step (may become negative if the vehicle passes beyond the detector end)
    pub distToDetectorEnd: f64,
    /// Time spent on the detector during the last integration step
    pub timeOnDetector: f64,
    /// The length of the part of the vehicle on the detector at the end of the last time step
    pub lengthOnDetector: f64,
    /// timeloss during the last integration step
    pub timeLoss: f64,
    /// the current (consecutive) waitingTime on the detector in s
    pub waitingTime: f64,
    /// whether the vehicle is on the detector at the end of the current timestep
    pub onDetector: bool,
}

impl MSE2Collector_MoveNotificationInfo {
    /// C++: `void MoveNotificationInfo::MoveNotificationInfo(std::string, double, double, double, double, double, double, double, double, double, bool)`
    // MoveNotificationInfo(std::string _vehID, double _oldPos, double _newPos, double _speed, double _accel, double _distToDetectorEnd, double _timeOnDetector, double _lengthOnDetector, double _timeLoss, double _waitingTime, bool _onDetector) :
    //             id(_vehID),
    //             oldPos(_oldPos),
    //             newPos(_newPos),
    //             speed(_speed),
    //             accel(_accel),
    //             distToDetectorEnd(_distToDetectorEnd),
    //             timeOnDetector(_timeOnDetector),
    //             lengthOnDetector(_lengthOnDetector),
    //             timeLoss(_timeLoss),
    //             waitingTime(_waitingTime),
    //             onDetector(_onDetector) {}
    #[allow(unused_variables)]
    pub fn new(_vehID: String, _oldPos: f64, _newPos: f64, _speed: f64, _accel: f64, _distToDetectorEnd: f64, _timeOnDetector: f64, _lengthOnDetector: f64, _timeLoss: f64, _waitingTime: f64, _onDetector: bool) -> Self {
        MSE2Collector_MoveNotificationInfo {
            id: _vehID,
            oldPos: _oldPos,
            newPos: _newPos,
            speed: _speed,
            accel: _accel,
            distToDetectorEnd: _distToDetectorEnd,
            timeOnDetector: _timeOnDetector,
            lengthOnDetector: _lengthOnDetector,
            timeLoss: _timeLoss,
            waitingTime: _waitingTime,
            onDetector: _onDetector,
        }
    }

    /// C++: `void MoveNotificationInfo::~MoveNotificationInfo() noexcept` [virtual]
    // virtual ~MoveNotificationInfo() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty.
    }

}

/// A VehicleInfo stores values that are tracked for the individual vehicles on the detector, e.g., accumulated timeloss. These infos are stored in myVehicles. If a vehicle leaves the detector (may it be temporarily), the entry in myVehicles is discarded, i.e. all information on the vehicle is reset.
/// C++ class `MSE2Collector::VehicleInfo` (microsim/output/MSE2Collector.h)
pub struct MSE2Collector_VehicleInfo {
    /// vehicle's ID
    pub id: String,
    /// vehicle's type
    pub type_: String,
    /// vehicle's length
    pub length: f64,
    /// vehicle's minGap
    pub minGap: f64,
    /// ID of the lane, on which the vehicle entered the detector
    pub entryLaneID: String,
    /// Distance of the vehicle's entry lane's beginning to the detector start (can be negative for the first lane) In notifyMove(), the positional input arguments are relative to that position (since the vehicle picks up the MoveReminder on the entry lane)
    pub entryOffset: f64,
    /// Lane, on which the vehicle currently resides (always the one for which the last notifyEnter was received)
    pub currentLane: Ptr<MSLane>,
    /// Index of currentLane in the detector's myLanes vector.
    pub currentOffsetIndex: usize,
    /// Offset from the detector start, where the vehicle has leaves the detector (defaults to detector length and is updated if the vehicle leaves the detector via a junction before reaching its end, i.e. enters a lane not part of the detector)
    pub exitOffset: f64,
    /// Distance left till the detector end after the last integration step (may become negative if the vehicle passes beyond the detector end)
    pub distToDetectorEnd: f64,
    /// Accumulated time that this vehicle has spent on the detector since its last entry
    pub totalTimeOnDetector: f64,
    /// Accumulated time loss that this vehicle suffered since it entered the detector
    pub accumulatedTimeLoss: f64,
    /// whether the vehicle is on the detector at the end of the current timestep
    pub onDetector: bool,
    /// Whether the vehicle has already entered the detector (don't count twice!)
    pub hasEntered: bool,
    /// Last value of the acceleration
    pub lastAccel: f64,
    /// Last value of the speed
    pub lastSpeed: f64,
    /// Last value of the vehicle position in reference to the start lane
    pub lastPos: f64,
}

impl MSE2Collector_VehicleInfo {
    /// C++: `void VehicleInfo::VehicleInfo(std::string, std::string, double, double, const MSLane *, double, std::size_t, double, double, bool)`
    // VehicleInfo(std::string id, std::string type, double length, double minGap, const MSLane* entryLane, double entryOffset,
    //                     std::size_t currentOffsetIndex, double exitOffset, double distToDetectorEnd, bool onDetector) :
    //             id(id),
    //             type(type),
    //             length(length),
    //             minGap(minGap),
    //             entryLaneID(entryLane->getID()),
    //             entryOffset(entryOffset),
    //             currentLane(entryLane),
    //             currentOffsetIndex(currentOffsetIndex),
    //             exitOffset(exitOffset),
    //             distToDetectorEnd(distToDetectorEnd),
    //             totalTimeOnDetector(0.),
    //             accumulatedTimeLoss(0.),
    //             onDetector(onDetector),
    //             hasEntered(false),
    //             lastAccel(0),
    //             lastSpeed(0),
    //             lastPos(0) {
    //             assert(exitOffset < 0);
    //         }
    #[allow(unused_variables)]
    pub fn new(id: String, type_: String, length: f64, minGap: f64, entryLane: Ptr<MSLane>, entryOffset: f64, currentOffsetIndex: usize, exitOffset: f64, distToDetectorEnd: f64, onDetector: bool) -> Self {
        debug_assert!(exitOffset < 0.0);
        MSE2Collector_VehicleInfo {
            id,
            type_,
            length,
            minGap,
            entryLaneID: entryLane.get_id().to_string(),
            entryOffset,
            currentLane: entryLane,
            currentOffsetIndex,
            exitOffset,
            distToDetectorEnd,
            totalTimeOnDetector: 0.0,
            accumulatedTimeLoss: 0.0,
            onDetector,
            hasEntered: false,
            lastAccel: 0.0,
            lastSpeed: 0.0,
            lastPos: 0.0,
        }
    }

    /// C++: `void VehicleInfo::~VehicleInfo() noexcept` [virtual]
    // virtual ~VehicleInfo() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty.
    }

}

