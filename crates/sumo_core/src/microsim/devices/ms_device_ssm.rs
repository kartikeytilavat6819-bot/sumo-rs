//! Port of microsim/devices/MSDevice_SSM.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ enum `MSDevice_SSM::EncounterType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum MSDevice_SSM_EncounterType {
    ENCOUNTER_TYPE_NOCONFLICT_AHEAD = 0,
    ENCOUNTER_TYPE_FOLLOWING = 1,
    ENCOUNTER_TYPE_FOLLOWING_FOLLOWER = 2,
    ENCOUNTER_TYPE_FOLLOWING_LEADER = 3,
    ENCOUNTER_TYPE_ON_ADJACENT_LANES = 4,
    ENCOUNTER_TYPE_MERGING = 5,
    ENCOUNTER_TYPE_MERGING_LEADER = 6,
    ENCOUNTER_TYPE_MERGING_FOLLOWER = 7,
    ENCOUNTER_TYPE_MERGING_ADJACENT = 8,
    ENCOUNTER_TYPE_CROSSING = 9,
    ENCOUNTER_TYPE_CROSSING_LEADER = 10,
    ENCOUNTER_TYPE_CROSSING_FOLLOWER = 11,
    ENCOUNTER_TYPE_EGO_ENTERED_CONFLICT_AREA = 12,
    ENCOUNTER_TYPE_FOE_ENTERED_CONFLICT_AREA = 13,
    ENCOUNTER_TYPE_EGO_LEFT_CONFLICT_AREA = 14,
    ENCOUNTER_TYPE_FOE_LEFT_CONFLICT_AREA = 15,
    ENCOUNTER_TYPE_BOTH_ENTERED_CONFLICT_AREA = 16,
    ENCOUNTER_TYPE_BOTH_LEFT_CONFLICT_AREA = 17,
    ENCOUNTER_TYPE_FOLLOWING_PASSED = 18,
    ENCOUNTER_TYPE_MERGING_PASSED = 19,
    ENCOUNTER_TYPE_ONCOMING = 20,
    ENCOUNTER_TYPE_COLLISION = 111,
}

/// C++ enum `MSDevice_SSM::SSMParameterWarning`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum MSDevice_SSM_SSMParameterWarning {
    SSM_WARN_MEASURES = 1,
    SSM_WARN_THRESHOLDS = 2,
    SSM_WARN_TRAJECTORIES = 4,
    SSM_WARN_RANGE = 8,
    SSM_WARN_EXTRATIME = 16,
    SSM_WARN_FILE = 32,
    SSM_WARN_GEO = 64,
    SSM_WARN_POS = 128,
    SSM_WARN_LANEPOS = 256,
    SSM_WARN_CONFLICTFILTER = 512,
}
impl MSDevice_SSM_SSMParameterWarning {
    pub const SSM_WARN_MDRAC_PRT: Self = Self::SSM_WARN_RANGE;
}

/// C++ typedef `int MSDevice_SSM::EncounterQueue`
pub type MSDevice_SSM_EncounterQueue = i32;

/// C++ typedef `std::vector<Encounter*> MSDevice_SSM::EncounterVector`
pub type MSDevice_SSM_EncounterVector = Vec<Ptr<MSDevice_SSM_Encounter>>;

/// C++ typedef `std::map<const MSVehicle*, FoeInfo*> MSDevice_SSM::FoeInfoMap`
// determinism: C++ std::map orders by pointer value; BTreeMap over Ptr (Ord by address) keeps that order.
pub type MSDevice_SSM_FoeInfoMap = BTreeMap<Ptr<MSVehicle>, Ptr<MSDevice_SSM_FoeInfo>>;

/// C++ typedef `bool compare::value_type`
pub type compare_value_type = bool;

/// ConflictPointInfo stores some information on a specific conflict point (used to store information on ssm-extremal values)
/// C++ class `Encounter::ConflictPointInfo` (microsim/devices/MSDevice_SSM.h)
pub struct Encounter_ConflictPointInfo {
    /// time point of the conflict
    pub time: f64,
    /// Predicted location of the conflict: In case of MERGING and CROSSING: entry point to conflict area for follower In case of FOLLOWING: position of leader's back
    pub pos: Position,
    /// Type of the conflict
    pub type_: MSDevice_SSM_EncounterType,
    /// value of the corresponding SSM
    pub value: f64,
    /// speed of the reporting vehicle at the given time/position
    pub speed: f64,
}

impl Encounter_ConflictPointInfo {
    /// C++: `void ConflictPointInfo::ConflictPointInfo(double, Position, EncounterType, double, double)`
    // ConflictPointInfo(double time, Position x, EncounterType type, double ssmValue, double speed) :
    //                 time(time), pos(x), type(type), value(ssmValue), speed(speed) {}
    #[allow(unused_variables)]
    pub fn new(time: f64, x: Position, type_: MSDevice_SSM_EncounterType, ssmValue: f64, speed: f64) -> Self {
        Self { time, pos: x, type_, value: ssmValue, speed }
    }

}

/// An encounter is an episode involving two vehicles, which are closer to each other than some specified distance.
/// C++ class `MSDevice_SSM::Encounter` (microsim/devices/MSDevice_SSM.h)
pub struct MSDevice_SSM_Encounter {
    pub ego: Ptr<MSVehicle>,
    pub foe: Ptr<MSVehicle>,
    pub egoID: String,
    pub foeID: String,
    pub begin: f64,
    pub end: f64,
    pub currentType: MSDevice_SSM_EncounterType,
    /// Remaining extra time (decreases after an encounter ended)
    pub remainingExtraTime: f64,
    /// Times when the ego vehicle entered/left the conflict area. Currently only applies for crossing situations. Used for PET calculation. (May be defined for merge conflicts in the future)
    pub egoConflictEntryTime: f64,
    /// Times when the ego vehicle entered/left the conflict area. Currently only applies for crossing situations. Used for PET calculation. (May be defined for merge conflicts in the future)
    pub egoConflictExitTime: f64,
    /// Times when the foe vehicle entered/left the conflict area. Currently only applies for crossing situations. Used for PET calculation. (May be defined for merge conflicts in the future)
    pub foeConflictEntryTime: f64,
    /// Times when the foe vehicle entered/left the conflict area. Currently only applies for crossing situations. Used for PET calculation. (May be defined for merge conflicts in the future)
    pub foeConflictExitTime: f64,
    /// time points corresponding to the trajectories
    pub timeSpan: Vec<f64>,
    /// Evolution of the encounter classification (
    pub typeSpan: Vec<i32>,
    /// Trajectory of the ego vehicle
    pub egoTrajectory: Encounter_Trajectory,
    /// Trajectory of the foe vehicle
    pub foeTrajectory: Encounter_Trajectory,
    /// Evolution of the ego vehicle's distance to the conflict point
    pub egoDistsToConflict: Vec<f64>,
    /// Evolution of the foe vehicle's distance to the conflict point
    pub foeDistsToConflict: Vec<f64>,
    /// Predicted location of the conflict: In case of MERGING and CROSSING: entry point to conflict area for follower In case of FOLLOWING: position of leader's back
    pub conflictPointSpan: PositionVector,
    /// All values for TTC
    pub TTCspan: Vec<f64>,
    /// All values for DRAC
    pub DRACspan: Vec<f64>,
    /// All values for MDRAC
    pub MDRACspan: Vec<f64>,
    /// All values for PPET
    pub PPETspan: Vec<f64>,
    /// @{
    pub minTTC: Encounter_ConflictPointInfo,
    pub maxDRAC: Encounter_ConflictPointInfo,
    pub maxMDRAC: Encounter_ConflictPointInfo,
    pub PET: Encounter_ConflictPointInfo,
    pub minPPET: Encounter_ConflictPointInfo,
    /// this flag is set by updateEncounter() or directly in processEncounters(), where encounters are closed if it is true.
    pub closingRequested: bool,
}

impl MSDevice_SSM_Encounter {
    /// C++: `void Encounter::Encounter(const MSVehicle *, const MSVehicle *const, double, double)`
    /// Constructor
    // MSDevice_SSM::Encounter::Encounter(const MSVehicle* _ego, const MSVehicle* const _foe, double _begin, double extraTime) :
    //     ego(_ego),
    //     foe(_foe),
    //     egoID(_ego->getID()),
    //     foeID(_foe->getID()),
    //     begin(_begin),
    //     end(-INVALID_DOUBLE),
    //     currentType(ENCOUNTER_TYPE_NOCONFLICT_AHEAD),
    //     remainingExtraTime(extraTime),
    //     egoConflictEntryTime(INVALID_DOUBLE),
    //     egoConflictExitTime(INVALID_DOUBLE),
    //     foeConflictEntryTime(INVALID_DOUBLE),
    //     foeConflictExitTime(INVALID_DOUBLE),
    //     minTTC(INVALID_DOUBLE, Position::INVALID, ENCOUNTER_TYPE_NOCONFLICT_AHEAD, INVALID_DOUBLE, INVALID_DOUBLE),
    //     maxDRAC(INVALID_DOUBLE, Position::INVALID, ENCOUNTER_TYPE_NOCONFLICT_AHEAD, INVALID_DOUBLE, INVALID_DOUBLE),
    //     maxMDRAC(INVALID_DOUBLE, Position::INVALID, ENCOUNTER_TYPE_NOCONFLICT_AHEAD, INVALID_DOUBLE, INVALID_DOUBLE),
    //     PET(INVALID_DOUBLE, Position::INVALID, ENCOUNTER_TYPE_NOCONFLICT_AHEAD, INVALID_DOUBLE, INVALID_DOUBLE),
    //     minPPET(INVALID_DOUBLE, Position::INVALID, ENCOUNTER_TYPE_NOCONFLICT_AHEAD, INVALID_DOUBLE, INVALID_DOUBLE),
    //     closingRequested(false) {
    // #ifdef DEBUG_ENCOUNTER
    //     if (DEBUG_COND_ENCOUNTER(this)) {
    //         std::cout << "\n" << SIMTIME << " Constructing encounter of '" << ego->getID() << "' and '" << foe->getID() << "'" << std::endl;
    //     }
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn new(_ego: Ptr<MSVehicle>, _foe: Ptr<MSVehicle>, _begin: f64, extraTime: f64) -> Self {
        let invalid_info = || Encounter_ConflictPointInfo::new(INVALID_DOUBLE, Position::INVALID, MSDevice_SSM_EncounterType::ENCOUNTER_TYPE_NOCONFLICT_AHEAD, INVALID_DOUBLE, INVALID_DOUBLE);
        Self {
            ego: _ego,
            foe: _foe,
            egoID: _ego.get_id().to_string(),
            foeID: _foe.get_id().to_string(),
            begin: _begin,
            end: -INVALID_DOUBLE,
            currentType: MSDevice_SSM_EncounterType::ENCOUNTER_TYPE_NOCONFLICT_AHEAD,
            remainingExtraTime: extraTime,
            egoConflictEntryTime: INVALID_DOUBLE,
            egoConflictExitTime: INVALID_DOUBLE,
            foeConflictEntryTime: INVALID_DOUBLE,
            foeConflictExitTime: INVALID_DOUBLE,
            timeSpan: Vec::new(),
            typeSpan: Vec::new(),
            egoTrajectory: Encounter_Trajectory { x: PositionVector(Vec::new()), lane: Vec::new(), lanePos: Vec::new(), v: PositionVector(Vec::new()) },
            foeTrajectory: Encounter_Trajectory { x: PositionVector(Vec::new()), lane: Vec::new(), lanePos: Vec::new(), v: PositionVector(Vec::new()) },
            egoDistsToConflict: Vec::new(),
            foeDistsToConflict: Vec::new(),
            conflictPointSpan: PositionVector(Vec::new()),
            TTCspan: Vec::new(),
            DRACspan: Vec::new(),
            MDRACspan: Vec::new(),
            PPETspan: Vec::new(),
            minTTC: invalid_info(),
            maxDRAC: invalid_info(),
            maxMDRAC: invalid_info(),
            PET: invalid_info(),
            minPPET: invalid_info(),
            closingRequested: false,
        }
    }

    /// C++: `void Encounter::~Encounter()`
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor only logs under DEBUG_ENCOUNTER; members are dropped implicitly.
    }

    /// C++: `void Encounter::add(double, EncounterType, Position, std::string, double, Position, Position, std::string, double, Position, Position, double, double, double, double, std::pair<double, double>, double, double)`
    /// add a new data point and update encounter type
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn add(&mut self, time: f64, type_: MSDevice_SSM_EncounterType, egoX: Position, egoLane: String, egoLanePos: f64, egoV: Position, foeX: Position, foeLane: String, foeLanePos: f64, foeV: Position, conflictPoint: Position, egoDistToConflict: f64, foeDistToConflict: f64, ttc: f64, drac: f64, pet: (f64, f64), ppet: f64, mdrac: f64) {
        let origin = Position::new(0., 0.);
        self.currentType = type_;
        self.timeSpan.push(time);
        self.typeSpan.push(type_ as i32);
        self.egoTrajectory.x.push(egoX);
        self.egoTrajectory.lane.push(egoLane);
        self.egoTrajectory.lanePos.push(egoLanePos);
        self.egoTrajectory.v.push(egoV);
        self.foeTrajectory.x.push(foeX);
        self.foeTrajectory.lane.push(foeLane);
        self.foeTrajectory.lanePos.push(foeLanePos);
        self.foeTrajectory.v.push(foeV);
        self.conflictPointSpan.push(conflictPoint);
        self.egoDistsToConflict.push(egoDistToConflict);
        self.foeDistsToConflict.push(foeDistToConflict);

        self.TTCspan.push(ttc);
        if ttc != INVALID_DOUBLE && (ttc < self.minTTC.value || self.minTTC.value == INVALID_DOUBLE) {
            self.minTTC.value = ttc;
            self.minTTC.time = time;
            self.minTTC.pos = conflictPoint;
            self.minTTC.type_ = if ttc <= 0. { MSDevice_SSM_EncounterType::ENCOUNTER_TYPE_COLLISION } else { type_ };
            self.minTTC.speed = egoV.distance_to(&origin);
        }

        self.DRACspan.push(drac);
        if drac != INVALID_DOUBLE && (drac > self.maxDRAC.value || self.maxDRAC.value == INVALID_DOUBLE) {
            self.maxDRAC.value = drac;
            self.maxDRAC.time = time;
            self.maxDRAC.pos = conflictPoint;
            self.maxDRAC.type_ = type_;
            self.maxDRAC.speed = egoV.distance_to(&origin);
        }

        if pet.0 != INVALID_DOUBLE && (self.PET.value >= pet.1 || self.PET.value == INVALID_DOUBLE) {
            self.PET.value = pet.1;
            self.PET.time = pet.0;
            self.PET.pos = conflictPoint;
            self.PET.type_ = if self.PET.value <= 0. { MSDevice_SSM_EncounterType::ENCOUNTER_TYPE_COLLISION } else { type_ };
            self.PET.speed = egoV.distance_to(&origin);
        }
        self.PPETspan.push(ppet);
        if ppet != INVALID_DOUBLE && (ppet < self.minPPET.value || self.minPPET.value == INVALID_DOUBLE) {
            self.minPPET.value = ppet;
            self.minPPET.time = time;
            self.minPPET.pos = conflictPoint;
            self.minPPET.type_ = if ppet <= 0. { MSDevice_SSM_EncounterType::ENCOUNTER_TYPE_COLLISION } else { type_ };
            self.minPPET.speed = egoV.distance_to(&origin);
        }
        self.MDRACspan.push(mdrac);
        if mdrac != INVALID_DOUBLE && (mdrac > self.maxMDRAC.value || self.maxMDRAC.value == INVALID_DOUBLE) {
            self.maxMDRAC.value = mdrac;
            self.maxMDRAC.time = time;
            self.maxMDRAC.pos = conflictPoint;
            self.maxMDRAC.type_ = type_;
            self.maxMDRAC.speed = egoV.distance_to(&origin);
        }
    }

    /// C++: `std::size_t Encounter::size() const` [const]
    /// Returns the number of trajectory points stored
    // std::size_t size() const {
    //             return timeSpan.size();
    //         }
    #[allow(unused_variables)]
    pub fn size(&self) -> usize {
        self.timeSpan.len()
    }

    /// C++: `void Encounter::resetExtraTime(double)`
    /// resets remainingExtraTime to the given value
    // void
    // MSDevice_SSM::Encounter::resetExtraTime(double value) {
    //     remainingExtraTime = value;
    // }
    #[allow(unused_variables)]
    pub fn reset_extra_time(&mut self, value: f64) {
        self.remainingExtraTime = value;
    }

    /// C++: `void Encounter::countDownExtraTime(double)`
    /// decreases myRemaingExtraTime by given amount in seconds
    // void
    // MSDevice_SSM::Encounter::countDownExtraTime(double amount) {
    //     remainingExtraTime -= amount;
    // }
    #[allow(unused_variables)]
    pub fn count_down_extra_time(&mut self, amount: f64) {
        self.remainingExtraTime -= amount;
    }

    /// C++: `double Encounter::getRemainingExtraTime() const` [const]
    /// returns the remaining extra time
    // double
    // MSDevice_SSM::Encounter::getRemainingExtraTime() const {
    //     return remainingExtraTime;
    // }
    #[allow(unused_variables)]
    pub fn get_remaining_extra_time(&self) -> f64 {
        self.remainingExtraTime
    }

    /// C++: `void Encounter::Encounter(const Encounter &)` [private]
    /// Invalidated Constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSDevice_SSM_Encounter) -> Self {
        // C++ declares this constructor private and never defines it: any use is a compile error in C++.
        panic!("MSDevice_SSM::Encounter copy constructor is private and invalidated")
    }

    /// C++: `Encounter & Encounter::operator=(const Encounter &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSDevice_SSM_Encounter) -> &mut MSDevice_SSM_Encounter {
        // C++ declares this operator private and never defines it: any use is a compile error in C++.
        panic!("MSDevice_SSM::Encounter assignment operator is private and invalidated")
    }

}

/// Structure to collect some info on the encounter needed during ssm calculation by various functions.
/// C++ class `MSDevice_SSM::EncounterApproachInfo` (microsim/devices/MSDevice_SSM.h)
pub struct MSDevice_SSM_EncounterApproachInfo {
    pub encounter: Ptr<MSDevice_SSM_Encounter>,
    pub type_: MSDevice_SSM_EncounterType,
    pub conflictPoint: Position,
    pub egoConflictEntryDist: f64,
    pub foeConflictEntryDist: f64,
    pub egoConflictExitDist: f64,
    pub foeConflictExitDist: f64,
    pub egoEstimatedConflictEntryTime: f64,
    pub foeEstimatedConflictEntryTime: f64,
    pub egoEstimatedConflictExitTime: f64,
    pub foeEstimatedConflictExitTime: f64,
    pub egoConflictAreaLength: f64,
    pub foeConflictAreaLength: f64,
    pub ttc: f64,
    pub drac: f64,
    pub mdrac: f64,
    pub pet: (f64, f64),
    pub ppet: f64,
    pub egoConflictEntryCrossSection: (Ptr<MSLane>, f64),
    pub foeConflictEntryCrossSection: (Ptr<MSLane>, f64),
}

impl MSDevice_SSM_EncounterApproachInfo {
    /// C++: `void EncounterApproachInfo::EncounterApproachInfo(Encounter *)`
    // MSDevice_SSM::EncounterApproachInfo::EncounterApproachInfo(Encounter* e) :
    //     encounter(e),
    //     type(ENCOUNTER_TYPE_NOCONFLICT_AHEAD),
    //     conflictPoint(Position::INVALID),
    //     egoConflictEntryDist(INVALID_DOUBLE),
    //     foeConflictEntryDist(INVALID_DOUBLE),
    //     egoConflictExitDist(INVALID_DOUBLE),
    //     foeConflictExitDist(INVALID_DOUBLE),
    //     egoEstimatedConflictEntryTime(INVALID_DOUBLE),
    //     foeEstimatedConflictEntryTime(INVALID_DOUBLE),
    //     egoEstimatedConflictExitTime(INVALID_DOUBLE),
    //     foeEstimatedConflictExitTime(INVALID_DOUBLE),
    //     egoConflictAreaLength(INVALID_DOUBLE),
    //     foeConflictAreaLength(INVALID_DOUBLE),
    //     ttc(INVALID_DOUBLE),
    //     drac(INVALID_DOUBLE),
    //     mdrac(INVALID_DOUBLE),
    //     pet(std::make_pair(INVALID_DOUBLE, INVALID_DOUBLE)),
    //     ppet(INVALID_DOUBLE) {
    // }
    #[allow(unused_variables)]
    pub fn new(e: Ptr<MSDevice_SSM_Encounter>) -> Self {
        Self {
            encounter: e,
            type_: MSDevice_SSM_EncounterType::ENCOUNTER_TYPE_NOCONFLICT_AHEAD,
            conflictPoint: Position::INVALID,
            egoConflictEntryDist: INVALID_DOUBLE,
            foeConflictEntryDist: INVALID_DOUBLE,
            egoConflictExitDist: INVALID_DOUBLE,
            foeConflictExitDist: INVALID_DOUBLE,
            egoEstimatedConflictEntryTime: INVALID_DOUBLE,
            foeEstimatedConflictEntryTime: INVALID_DOUBLE,
            egoEstimatedConflictExitTime: INVALID_DOUBLE,
            foeEstimatedConflictExitTime: INVALID_DOUBLE,
            egoConflictAreaLength: INVALID_DOUBLE,
            foeConflictAreaLength: INVALID_DOUBLE,
            ttc: INVALID_DOUBLE,
            drac: INVALID_DOUBLE,
            mdrac: INVALID_DOUBLE,
            pet: (INVALID_DOUBLE, INVALID_DOUBLE),
            ppet: INVALID_DOUBLE,
            // not initialised in the C++ constructor: std::pair default-initialises to (nullptr, 0.)
            egoConflictEntryCrossSection: (Ptr::default(), 0.),
            foeConflictEntryCrossSection: (Ptr::default(), 0.),
        }
    }

}

/// A new FoeInfo is created during findSurroundingVehicles() to memorize, where the potential conflict corresponding to the encounter might occur. Each FoeInfo ends up in a call to updateEncounter() and is deleted there.
/// C++ class `MSDevice_SSM::FoeInfo` (microsim/devices/MSDevice_SSM.h)
pub struct MSDevice_SSM_FoeInfo {
    pub egoConflictLane: Ptr<MSLane>,
    pub egoDistToConflictLane: f64,
}

impl MSDevice_SSM_FoeInfo {
    /// C++: `void FoeInfo::~FoeInfo() noexcept` [virtual]
    // virtual ~FoeInfo() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ virtual destructor is empty (`virtual ~FoeInfo() {}`): nothing to release.
    }

}

/// C++ class `MSDevice_SSM` (microsim/devices/MSDevice_SSM.h)
pub struct MSDevice_SSM {
    /// C++ base class `MSVehicleDevice`
    pub base: MSVehicleDevice,
    /// thresholds for the ssms, i.e., critical values above or below which a value indicates that a conflict has occurred. These are used in qualifiesAsConflict() and decide whether an encounter is saved.
    pub myThresholds: i32,
    /// This determines whether the whole trajectories of the vehicles (position, speed, ssms) shall be saved in the ssm-output or only the most critical value shall be reported.
    pub mySaveTrajectories: bool,
    /// Detection range. For vehicles closer than this distance from the ego vehicle, SSMs are traced
    pub myRange: f64,
    /// perception reaction time for MDRAC
    pub myMDRACPRT: f64,
    /// Extra time in seconds to be logged after a conflict is over
    pub myExtraTime: f64,
    /// Whether to use the original coordinate system for output
    pub myUseGeoCoords: bool,
    /// Whether to print the positions for all timesteps
    pub myWritePositions: bool,
    /// Whether to print the lanes and positions for all timesteps and conflicts
    pub myWriteLanesPositions: bool,
    /// Whether to write measures with NA entries
    pub myWriteNA: bool,
    /// Whether to exclude certain conflicts containing certain conflict types from the output
    pub myFilterConflictTypes: bool,
    /// Which conflict types to exclude from the output
    pub myDroppedConflictTypes: i32,
    /// Flags for switching on / off computation of different SSMs, derived from myMeasures
    pub myComputeTTC: bool,
    /// Flags for switching on / off computation of different SSMs, derived from myMeasures
    pub myComputeDRAC: bool,
    /// Flags for switching on / off computation of different SSMs, derived from myMeasures
    pub myComputePET: bool,
    /// Flags for switching on / off computation of different SSMs, derived from myMeasures
    pub myComputeBR: bool,
    /// Flags for switching on / off computation of different SSMs, derived from myMeasures
    pub myComputeSGAP: bool,
    /// Flags for switching on / off computation of different SSMs, derived from myMeasures
    pub myComputeTGAP: bool,
    /// Flags for switching on / off computation of different SSMs, derived from myMeasures
    pub myComputePPET: bool,
    /// Flags for switching on / off computation of different SSMs, derived from myMeasures
    pub myComputeMDRAC: bool,
    pub myHolderMS: Ptr<MSVehicle>,
    /// Currently observed encounters/conflicts
    pub myActiveEncounters: MSDevice_SSM_EncounterVector,
    /// begin time of the oldest active encounter
    pub myOldestActiveEncounterBegin: f64,
    /// Past encounters that where qualified as conflicts and are not yet flushed to the output file
    pub myPastConflicts: MSDevice_SSM_EncounterQueue,
    /// @{
    pub myGlobalMeasuresTimeSpan: i32,
    /// All values for positions (coordinates)
    pub myGlobalMeasuresPositions: PositionVector,
    /// All values for lanes
    pub myGlobalMeasuresLaneIDs: i32,
    /// All values for positions on the lanes
    pub myGlobalMeasuresLanesPositions: i32,
    /// All values for brake rate
    pub myBRspan: i32,
    /// All values for space gap
    pub mySGAPspan: i32,
    /// All values for time gap
    pub myTGAPspan: i32,
    /// Extremal values for the global measures (as <<<time, Position>, value>, [leaderID]>-pairs) @{
    pub myMaxBR: ((f64, Position), f64),
    pub myMinSGAP: (((f64, Position), f64), String),
    pub myMinTGAP: (((f64, Position), f64), String),
    /// Output device
    pub myOutputFile: Ptr<OutputDevice>,
}

impl std::ops::Deref for MSDevice_SSM {
    type Target = MSVehicleDevice;
    fn deref(&self) -> &MSVehicleDevice {
        &self.base
    }
}
impl std::ops::DerefMut for MSDevice_SSM {
    fn deref_mut(&mut self) -> &mut MSVehicleDevice {
        &mut self.base
    }
}

// C++ static members of MSDevice_SSM (port as module-level statics, see AGENTS.md):
//   static int * MSDevice_SSM::myInstances;
//   static int MSDevice_SSM::myEdgeFilter;
//   static bool MSDevice_SSM::myEdgeFilterInitialized;
//   static bool MSDevice_SSM::myEdgeFilterActive;
//   static int MSDevice_SSM::myCreatedOutputFiles;
//   static int MSDevice_SSM::myIssuedParameterWarnFlags;
//   static const int MSDevice_SSM::FOE_ENCOUNTERTYPES;
//   static const int MSDevice_SSM::EGO_ENCOUNTERTYPES;

thread_local! {
    /// C++ static `MSDevice_SSM::myEdgeFilter` (std::set<const MSEdge*>; ordered by pointer)
    pub static MY_EDGE_FILTER: std::cell::RefCell<BTreeSet<Ptr<MSEdge>>> = std::cell::RefCell::new(BTreeSet::new());
    /// C++ static `MSDevice_SSM::myEdgeFilterActive`
    pub static MY_EDGE_FILTER_ACTIVE: std::cell::Cell<bool> = std::cell::Cell::new(false);
}

impl MSDevice_SSM {
    /// C++: `std::string MSDevice_SSM::encounterToString(EncounterType)` [static]
    // static std::string encounterToString(EncounterType type) {
    //         switch (type) {
    //             case (ENCOUNTER_TYPE_NOCONFLICT_AHEAD):
    //                 return ("NOCONFLICT_AHEAD");
    //             case (ENCOUNTER_TYPE_FOLLOWING):
    //                 return ("FOLLOWING");
    //             case (ENCOUNTER_TYPE_FOLLOWING_FOLLOWER):
    //                 return ("FOLLOWING_FOLLOWER");
    //             case (ENCOUNTER_TYPE_FOLLOWING_LEADER):
    //                 return ("FOLLOWING_LEADER");
    //             case (ENCOUNTER_TYPE_ON_ADJACENT_LANES):
    //                 return ("ON_ADJACENT_LANES");
    //             case (ENCOUNTER_TYPE_MERGING):
    //                 return ("MERGING");
    //             case (ENCOUNTER_TYPE_MERGING_LEADER):
    //                 return ("MERGING_LEADER");
    //             case (ENCOUNTER_TYPE_MERGING_FOLLOWER):
    //                 return ("MERGING_FOLLOWER");
    //             case (ENCOUNTER_TYPE_MERGING_ADJACENT):
    //                 return ("MERGING_ADJACENT");
    //             case (ENCOUNTER_TYPE_CROSSING):
    //                 return ("CROSSING");
    //             case (ENCOUNTER_TYPE_CROSSING_LEADER):
    //                 return ("CROSSING_LEADER");
    //             case (ENCOUNTER_TYPE_CROSSING_FOLLOWER):
    //                 return ("CROSSING_FOLLOWER");
    //             case (ENCOUNTER_TYPE_EGO_ENTERED_CONFLICT_AREA):
    //                 return ("EGO_ENTERED_CONFLICT_AREA");
    //             case (ENCOUNTER_TYPE_FOE_ENTERED_CONFLICT_AREA):
    //                 return ("FOE_ENTERED_CONFLICT_AREA");
    //             case (ENCOUNTER_TYPE_EGO_LEFT_CONFLICT_AREA):
    //                 return ("EGO_LEFT_CONFLICT_AREA");
    //             case (ENCOUNTER_TYPE_FOE_LEFT_CONFLICT_AREA):
    //                 return ("FOE_LEFT_CONFLICT_AREA");
    //             case (ENCOUNTER_TYPE_BOTH_ENTERED_CONFLICT_AREA):
    //                 return ("BOTH_ENTERED_CONFLICT_AREA");
    //             case (ENCOUNTER_TYPE_BOTH_LEFT_CONFLICT_AREA):
    //                 return ("BOTH_LEFT_CONFLICT_AREA");
    //             case (ENCOUNTER_TYPE_FOLLOWING_PASSED):
    //                 return ("FOLLOWING_PASSED");
    //             case (ENCOUNTER_TYPE_MERGING_PASSED):
    //                 return ("MERGING_PASSED");
    //             case (ENCOUNTER_TYPE_ONCOMING):
    //                 return ("ONCOMING");
    //             case (ENCOUNTER_TYPE_COLLISION):
    //                 return ("COLLISION");
    //         }
    //         return ("UNKNOWN");
    //     }
    #[allow(unused_variables)]
    pub fn encounter_to_string(type_: MSDevice_SSM_EncounterType) -> String {
        use MSDevice_SSM_EncounterType::*;
        let s = match type_ {
            ENCOUNTER_TYPE_NOCONFLICT_AHEAD => "NOCONFLICT_AHEAD",
            ENCOUNTER_TYPE_FOLLOWING => "FOLLOWING",
            ENCOUNTER_TYPE_FOLLOWING_FOLLOWER => "FOLLOWING_FOLLOWER",
            ENCOUNTER_TYPE_FOLLOWING_LEADER => "FOLLOWING_LEADER",
            ENCOUNTER_TYPE_ON_ADJACENT_LANES => "ON_ADJACENT_LANES",
            ENCOUNTER_TYPE_MERGING => "MERGING",
            ENCOUNTER_TYPE_MERGING_LEADER => "MERGING_LEADER",
            ENCOUNTER_TYPE_MERGING_FOLLOWER => "MERGING_FOLLOWER",
            ENCOUNTER_TYPE_MERGING_ADJACENT => "MERGING_ADJACENT",
            ENCOUNTER_TYPE_CROSSING => "CROSSING",
            ENCOUNTER_TYPE_CROSSING_LEADER => "CROSSING_LEADER",
            ENCOUNTER_TYPE_CROSSING_FOLLOWER => "CROSSING_FOLLOWER",
            ENCOUNTER_TYPE_EGO_ENTERED_CONFLICT_AREA => "EGO_ENTERED_CONFLICT_AREA",
            ENCOUNTER_TYPE_FOE_ENTERED_CONFLICT_AREA => "FOE_ENTERED_CONFLICT_AREA",
            ENCOUNTER_TYPE_EGO_LEFT_CONFLICT_AREA => "EGO_LEFT_CONFLICT_AREA",
            ENCOUNTER_TYPE_FOE_LEFT_CONFLICT_AREA => "FOE_LEFT_CONFLICT_AREA",
            ENCOUNTER_TYPE_BOTH_ENTERED_CONFLICT_AREA => "BOTH_ENTERED_CONFLICT_AREA",
            ENCOUNTER_TYPE_BOTH_LEFT_CONFLICT_AREA => "BOTH_LEFT_CONFLICT_AREA",
            ENCOUNTER_TYPE_FOLLOWING_PASSED => "FOLLOWING_PASSED",
            ENCOUNTER_TYPE_MERGING_PASSED => "MERGING_PASSED",
            ENCOUNTER_TYPE_ONCOMING => "ONCOMING",
            ENCOUNTER_TYPE_COLLISION => "COLLISION",
        };
        s.to_string()
    }

    /// C++: `void MSDevice_SSM::insertOptions(OptionsCont &)` [static]
    /// Inserts MSDevice_SSM-options
    // void
    // MSDevice_SSM::insertOptions(OptionsCont& oc) {
    //     oc.addOptionSubTopic("SSM Device");
    //     insertDefaultAssignmentOptions("ssm", "SSM Device", oc);
    //
    //     // custom options
    //     oc.doRegister("device.ssm.measures", new Option_String(""));
    //     oc.addDescription("device.ssm.measures", "SSM Device", TL("Specifies which measures will be logged (as a space or comma-separated sequence of IDs in ('TTC', 'DRAC', 'PET', 'PPET', 'MDRAC'))"));
    //     oc.doRegister("device.ssm.thresholds", new Option_String(""));
    //     oc.addDescription("device.ssm.thresholds", "SSM Device", TL("Specifies space or comma-separated thresholds corresponding to the specified measures (see documentation and watch the order!). Only events exceeding the thresholds will be logged."));
    //     oc.doRegister("device.ssm.trajectories",  new Option_Bool(false));
    //     oc.addDescription("device.ssm.trajectories", "SSM Device", TL("Specifies whether trajectories will be logged (if false, only the extremal values and times are reported)."));
    //     oc.doRegister("device.ssm.range", new Option_Float(50.));
    //     oc.addDescription("device.ssm.range", "SSM Device", TL("Specifies the detection range in meters. For vehicles below this distance from the equipped vehicle, SSM values are traced."));
    //     oc.doRegister("device.ssm.extratime", new Option_Float(DEFAULT_EXTRA_TIME));
    //     oc.addDescription("device.ssm.extratime", "SSM Device", TL("Specifies the time in seconds to be logged after a conflict is over. Required >0 if PET is to be calculated for crossing conflicts."));
    //     oc.doRegister("device.ssm.mdrac.prt", new Option_Float(1.));
    //     oc.addDescription("device.ssm.mdrac.prt", "SSM Device", TL("Specifies the perception reaction time for MDRAC computation."));
    //     oc.doRegister("device.ssm.file", new Option_String(""));
    //     oc.addDescription("device.ssm.file", "SSM Device", TL("Give a global default filename for the SSM output"));
    //     oc.doRegister("device.ssm.geo", new Option_Bool(false));
    //     oc.addDescription("device.ssm.geo", "SSM Device", TL("Whether to use coordinates of the original reference system in output"));
    //     oc.doRegister("device.ssm.write-positions", new Option_Bool(false));
    //     oc.addDescription("device.ssm.write-positions", "SSM Device", TL("Whether to write positions (coordinates) for each timestep"));
    //     oc.doRegister("device.ssm.write-lane-positions", new Option_Bool(false));
    //     oc.addDescription("device.ssm.write-lane-positions", "SSM Device", TL("Whether to write lanes and their positions for each timestep"));
    //     oc.doRegister("device.ssm.write-na", new Option_Bool(true));
    //     oc.addDescription("device.ssm.write-na", "SSM Device", TL("Whether to write conflict outputs with no data as NA values or skip it"));
    //     oc.doRegister("device.ssm.exclude-conflict-types", new Option_String(""));
    //     oc.addDescription("device.ssm.exclude-conflict-types", "SSM Device", TL("Which conflicts will be excluded from the log according to the conflict type they have been classified (combination of values in 'ego', 'foe' , '', any numerical valid conflict type code). An empty value will log all and 'ego'/'foe' refer to a certain conflict type subset."));
    // }
    #[allow(unused_variables)]
    pub fn insert_options(oc: &mut OptionsCont) {
        oc.add_option_sub_topic("SSM Device");
        MSDevice::insert_default_assignment_options("ssm", "SSM Device", oc, false);

        // custom options
        oc.do_register("device.ssm.measures", SumoOption::string_with(""));
        oc.add_description("device.ssm.measures", "SSM Device", "Specifies which measures will be logged (as a space or comma-separated sequence of IDs in ('TTC', 'DRAC', 'PET', 'PPET', 'MDRAC'))");
        oc.do_register("device.ssm.thresholds", SumoOption::string_with(""));
        oc.add_description("device.ssm.thresholds", "SSM Device", "Specifies space or comma-separated thresholds corresponding to the specified measures (see documentation and watch the order!). Only events exceeding the thresholds will be logged.");
        oc.do_register("device.ssm.trajectories", SumoOption::bool(false));
        oc.add_description("device.ssm.trajectories", "SSM Device", "Specifies whether trajectories will be logged (if false, only the extremal values and times are reported).");
        oc.do_register("device.ssm.range", SumoOption::float(50.));
        oc.add_description("device.ssm.range", "SSM Device", "Specifies the detection range in meters. For vehicles below this distance from the equipped vehicle, SSM values are traced.");
        // DEFAULT_EXTRA_TIME (MSDevice_SSM.cpp) = 5. seconds
        oc.do_register("device.ssm.extratime", SumoOption::float(5.));
        oc.add_description("device.ssm.extratime", "SSM Device", "Specifies the time in seconds to be logged after a conflict is over. Required >0 if PET is to be calculated for crossing conflicts.");
        oc.do_register("device.ssm.mdrac.prt", SumoOption::float(1.));
        oc.add_description("device.ssm.mdrac.prt", "SSM Device", "Specifies the perception reaction time for MDRAC computation.");
        oc.do_register("device.ssm.file", SumoOption::string_with(""));
        oc.add_description("device.ssm.file", "SSM Device", "Give a global default filename for the SSM output");
        oc.do_register("device.ssm.geo", SumoOption::bool(false));
        oc.add_description("device.ssm.geo", "SSM Device", "Whether to use coordinates of the original reference system in output");
        oc.do_register("device.ssm.write-positions", SumoOption::bool(false));
        oc.add_description("device.ssm.write-positions", "SSM Device", "Whether to write positions (coordinates) for each timestep");
        oc.do_register("device.ssm.write-lane-positions", SumoOption::bool(false));
        oc.add_description("device.ssm.write-lane-positions", "SSM Device", "Whether to write lanes and their positions for each timestep");
        oc.do_register("device.ssm.write-na", SumoOption::bool(true));
        oc.add_description("device.ssm.write-na", "SSM Device", "Whether to write conflict outputs with no data as NA values or skip it");
        oc.do_register("device.ssm.exclude-conflict-types", SumoOption::string_with(""));
        oc.add_description("device.ssm.exclude-conflict-types", "SSM Device", "Which conflicts will be excluded from the log according to the conflict type they have been classified (combination of values in 'ego', 'foe' , '', any numerical valid conflict type code). An empty value will log all and 'ego'/'foe' refer to a certain conflict type subset.");
    }

    /// C++: `void MSDevice_SSM::buildVehicleDevices(SUMOVehicle &, int &)` [static]
    /// Build devices for the given vehicle, if needed
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn build_vehicle_devices(v: &mut SUMOVehicle, into: &mut Vec<Ptr<MSVehicleDevice>>) {
        todo!("port MSDevice_SSM::buildVehicleDevices")
    }

    /// C++: `const int & MSDevice_SSM::getInstances()` [static]
    /// returns all currently existing SSM devices
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_instances() -> &'static i32 {
        todo!("port MSDevice_SSM::getInstances")
    }

    /// C++: `const int & MSDevice_SSM::getEdgeFilter()` [static]
    /// return the edges where the SSM device should scan
    // static const std::set<const MSEdge*>& getEdgeFilter() {
    //         return myEdgeFilter;
    //     }
    #[allow(unused_variables)]
    pub fn get_edge_filter() -> &'static i32 {
        todo!("port MSDevice_SSM::getEdgeFilter")
    }

    /// C++: `void MSDevice_SSM::updateAndWriteOutput()`
    /// This is called once per time step in MSNet::writeOutput() and collects the surrounding vehicles, updates information on encounters and flushes the encounters qualified as conflicts (
    // void
    // MSDevice_SSM::updateAndWriteOutput() {
    //     if (myHolder.isOnRoad()) {
    //         update();
    //         // Write out past conflicts
    //         flushConflicts();
    //     } else {
    // #ifdef DEBUG_SSM
    //         if (DEBUG_COND(myHolderMS))
    //             std::cout << "\n" << SIMTIME << " Device '" << getID() << "' updateAndWriteOutput()\n"
    //                       << "  Holder is off-road! Calling resetEncounters()."
    //                       << std::endl;
    // #endif
    //         resetEncounters();
    //         // Write out past conflicts
    //         flushConflicts(true);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn update_and_write_output(&mut self) {
        if self.base.myHolder.is_on_road() {
            self.update();
            // Write out past conflicts
            self.flush_conflicts(false);
        } else {
            self.reset_encounters();
            // Write out past conflicts
            self.flush_conflicts(true);
        }
    }

    /// C++: `std::string MSDevice_SSM::getParameter(const std::string &) const` [const, virtual]
    /// try to retrieve the given parameter from this device. Throw exception for unsupported key
    // std::string
    // MSDevice_SSM::getParameter(const std::string& key) const {
    //     if (key == "minTTC" && !myComputeTTC) {
    //         throw InvalidArgument("Measure TTC is not tracked by ssm device");
    //     }
    //     if (key == "maxDRAC" && !myComputeDRAC) {
    //         throw InvalidArgument("Measure DRAC is not tracked by ssm device");
    //     }
    //     if (key == "maxMDRAC" && !myComputeMDRAC) {
    //         throw InvalidArgument("Measure MDRAC is not tracked by ssm device");
    //     }
    //     if (key == "minPET" && !myComputePET) {
    //         throw InvalidArgument("Measure PET is not tracked by ssm device");
    //     }
    //     if (key == "minPPET" && !myComputePPET) {
    //         throw InvalidArgument("Measure PPET is not tracked by ssm device");
    //     }
    //     if (key == "minTTC" ||
    //             key == "maxDRAC" ||
    //             key == "maxMDRAC" ||
    //             key == "minPET" ||
    //             key == "minPPET") {
    //         double value = INVALID_DOUBLE;
    //         double minTTC = INVALID_DOUBLE;
    //         double minPET = INVALID_DOUBLE;
    //         double maxDRAC = -INVALID_DOUBLE;
    //         double maxMDRAC = -INVALID_DOUBLE;
    //         double minPPET = INVALID_DOUBLE;
    //         for (Encounter* e : myActiveEncounters) {
    //             minTTC = MIN2(minTTC, e->minTTC.value);
    //             minPET = MIN2(minPET, e->PET.value);
    //             maxDRAC = MAX2(maxDRAC, e->maxDRAC.value);
    //             maxMDRAC = MAX2(maxMDRAC, e->maxMDRAC.value);
    //             minPPET = MIN2(minPPET, e->minPPET.value);
    //         }
    //         if (key == "minTTC") {
    //             value = minTTC;
    //         } else if (key == "maxDRAC") {
    //             value = maxDRAC;
    //         } else if (key == "maxMDRAC") {
    //             value = maxMDRAC;
    //         } else if (key == "minPET") {
    //             value = minPET;
    //         } else if (key == "minPPET") {
    //             value = minPPET;
    //         }
    //         if (fabs(value) == INVALID_DOUBLE) {
    //             return "";
    //         } else {
    //             return toString(value);
    //         }
    //     }
    //     throw InvalidArgument("Parameter '" + key + "' is not supported for device of type '" + deviceName() + "'.");
    // }
    #[allow(unused_variables)]
    pub fn get_parameter(&self, key: &str) -> String {
        if key == "minTTC" && !self.myComputeTTC {
            panic!("{}", InvalidArgument("Measure TTC is not tracked by ssm device".to_string()).0);
        }
        if key == "maxDRAC" && !self.myComputeDRAC {
            panic!("{}", InvalidArgument("Measure DRAC is not tracked by ssm device".to_string()).0);
        }
        if key == "maxMDRAC" && !self.myComputeMDRAC {
            panic!("{}", InvalidArgument("Measure MDRAC is not tracked by ssm device".to_string()).0);
        }
        if key == "minPET" && !self.myComputePET {
            panic!("{}", InvalidArgument("Measure PET is not tracked by ssm device".to_string()).0);
        }
        if key == "minPPET" && !self.myComputePPET {
            panic!("{}", InvalidArgument("Measure PPET is not tracked by ssm device".to_string()).0);
        }
        if key == "minTTC" || key == "maxDRAC" || key == "maxMDRAC" || key == "minPET" || key == "minPPET" {
            let mut value = INVALID_DOUBLE;
            let mut minTTC = INVALID_DOUBLE;
            let mut minPET = INVALID_DOUBLE;
            let mut maxDRAC = -INVALID_DOUBLE;
            let mut maxMDRAC = -INVALID_DOUBLE;
            let mut minPPET = INVALID_DOUBLE;
            // MIN2/MAX2 macros: (a < b ? a : b) and (a > b ? a : b)
            for e in self.myActiveEncounters.iter() {
                minTTC = if minTTC < e.minTTC.value { minTTC } else { e.minTTC.value };
                minPET = if minPET < e.PET.value { minPET } else { e.PET.value };
                maxDRAC = if maxDRAC > e.maxDRAC.value { maxDRAC } else { e.maxDRAC.value };
                maxMDRAC = if maxMDRAC > e.maxMDRAC.value { maxMDRAC } else { e.maxMDRAC.value };
                minPPET = if minPPET < e.minPPET.value { minPPET } else { e.minPPET.value };
            }
            if key == "minTTC" {
                value = minTTC;
            } else if key == "maxDRAC" {
                value = maxDRAC;
            } else if key == "maxMDRAC" {
                value = maxMDRAC;
            } else if key == "minPET" {
                value = minPET;
            } else if key == "minPPET" {
                value = minPPET;
            }
            if value.abs() == INVALID_DOUBLE {
                return String::new();
            } else {
                return utils::common::to_string::to_string(&value);
            }
        }
        panic!("{}", InvalidArgument(format!("Parameter '{}' is not supported for device of type '{}'.", key, self.device_name())).0);
    }

    /// C++: `void MSDevice_SSM::setParameter(const std::string &, const std::string &)` [virtual]
    /// try to set the given parameter for this device. Throw exception for unsupported key
    // void
    // MSDevice_SSM::setParameter(const std::string& key, const std::string& value) {
    //     double doubleValue;
    //     try {
    //         doubleValue = StringUtils::toDouble(value);
    //     } catch (NumberFormatException&) {
    //         throw InvalidArgument("Setting parameter '" + key + "' requires a number for device of type '" + deviceName() + "'.");
    //     }
    //     if (false || key == "foo") {
    //         UNUSED_PARAMETER(doubleValue);
    //     } else {
    //         throw InvalidArgument("Setting parameter '" + key + "' is not supported for device of type '" + deviceName() + "'.");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(&mut self, key: &str, value: &str) {
        let doubleValue = match utils::common::string_utils::to_double(value) {
            Ok(d) => d,
            Err(_) => panic!("{}", InvalidArgument(format!("Setting parameter '{}' requires a number for device of type '{}'.", key, self.device_name())).0),
        };
        if false || key == "foo" {
            let _ = doubleValue;
        } else {
            panic!("{}", InvalidArgument(format!("Setting parameter '{}' is not supported for device of type '{}'.", key, self.device_name())).0);
        }
    }

    /// C++: `void MSDevice_SSM::update()` [private]
    // void
    // MSDevice_SSM::update() {
    // #ifdef DEBUG_SSM
    //     if (DEBUG_COND(myHolderMS))
    //         std::cout << "\n" << SIMTIME << " Device '" << getID() << "' update()\n"
    //                   << "Size of myActiveEncounters: " << myActiveEncounters.size()
    //                   << "\nSize of myPastConflicts: " << myPastConflicts.size()
    //                   << std::endl;
    // #endif
    //     // Scan surroundings for other vehicles
    //     FoeInfoMap foes;
    //     bool scan = true;
    //     if (myEdgeFilterActive) {
    //         // Is the ego vehicle inside the filtered edge subset?
    //         const MSEdge* egoEdge = &((*myHolderMS).getLane()->getEdge());
    //         scan = myEdgeFilter.find(egoEdge) != myEdgeFilter.end();
    //     }
    //     if (scan) {
    //         findSurroundingVehicles(*myHolderMS, myRange, foes);
    //     }
    //
    // #ifdef DEBUG_SSM
    //     if (DEBUG_COND(myHolderMS)) {
    //         if (foes.size() > 0) {
    //             std::cout << "Scanned surroundings: Found potential foes:\n";
    //             for (FoeInfoMap::const_iterator i = foes.begin(); i != foes.end(); ++i) {
    //                 std::cout << i->first->getID() << " ";
    //             }
    //             std::cout << std::endl;
    //         } else {
    //             std::cout << "Scanned surroundings: No potential conflict could be identified." << std::endl;
    //         }
    //     }
    // #endif
    //
    //     // Update encounters and conflicts -> removes all foes (and deletes corresponding FoeInfos) for which already a corresponding encounter exists
    //     processEncounters(foes);
    //
    //     // Make new encounters for all foes, which were not removed by processEncounters (and deletes corresponding FoeInfos)
    //     createEncounters(foes);
    //     foes.clear();
    //
    //     // Compute "global SSMs" (only computed once per time-step)
    //     computeGlobalMeasures();
    //
    // }
    #[allow(unused_variables)]
    pub fn update(&mut self) {
        // Scan surroundings for other vehicles
        let mut foes: MSDevice_SSM_FoeInfoMap = BTreeMap::new();
        let mut scan = true;
        if MY_EDGE_FILTER_ACTIVE.with(|c| c.get()) {
            // Is the ego vehicle inside the filtered edge subset?
            let egoEdge = Ptr::from_mut(self.myHolderMS.get_lane().get_edge());
            scan = MY_EDGE_FILTER.with(|f| f.borrow().contains(&egoEdge));
        }
        if scan {
            MSDevice_SSM::find_surrounding_vehicles(&*self.myHolderMS, self.myRange, &mut foes);
        }

        // Update encounters and conflicts -> removes all foes (and deletes corresponding FoeInfos) for which already a corresponding encounter exists
        self.process_encounters(&mut foes, false);

        // Make new encounters for all foes, which were not removed by processEncounters (and deletes corresponding FoeInfos)
        self.create_encounters(&mut foes);
        foes.clear();

        // Compute "global SSMs" (only computed once per time-step)
        self.compute_global_measures();
    }

    /// C++: `void MSDevice_SSM::writeOutConflict(Encounter *)` [private]
    // void
    // MSDevice_SSM::writeOutConflict(Encounter* e) {
    // #ifdef DEBUG_SSM
    //     if (DEBUG_COND(myHolderMS))
    //         std::cout << SIMTIME << " writeOutConflict() of vehicles '"
    //                   << e->egoID << "' and '" << e->foeID
    //                   << "'\ntoGeo=" << myUseGeoCoords << std::endl;
    // #endif
    //     myOutputFile->openTag("conflict");
    //     myOutputFile->writeAttr("begin", e->begin).writeAttr("end", e->end);
    //     myOutputFile->writeAttr("ego", e->egoID).writeAttr("foe", e->foeID);
    //
    //     if (mySaveTrajectories) {
    //         myOutputFile->openTag("timeSpan").writeAttr("values", e->timeSpan).closeTag();
    //         myOutputFile->openTag("typeSpan").writeAttr("values", e->typeSpan).closeTag();
    //
    //         // Some useful snippets for that (from MSFCDExport.cpp):
    //         if (myUseGeoCoords) {
    //             toGeo(e->egoTrajectory.x);
    //             toGeo(e->foeTrajectory.x);
    //             toGeo(e->conflictPointSpan);
    //         }
    //
    //         myOutputFile->openTag("egoPosition").writeAttr("values", ::toString(e->egoTrajectory.x, myUseGeoCoords ? gPrecisionGeo : gPrecision)).closeTag();
    //         if (myWriteLanesPositions) {
    //             myOutputFile->openTag("egoLane").writeAttr("values", ::toString(e->egoTrajectory.lane)).closeTag();
    //             myOutputFile->openTag("egoLanePosition").writeAttr("values", ::toString(e->egoTrajectory.lanePos)).closeTag();
    //         }
    //         myOutputFile->openTag("egoVelocity").writeAttr("values", ::toString(e->egoTrajectory.v)).closeTag();
    //
    //         myOutputFile->openTag("foePosition").writeAttr("values", ::toString(e->foeTrajectory.x, myUseGeoCoords ? gPrecisionGeo : gPrecision)).closeTag();
    //         if (myWriteLanesPositions) {
    //             myOutputFile->openTag("foeLane").writeAttr("values", ::toString(e->foeTrajectory.lane)).closeTag();
    //             myOutputFile->openTag("foeLanePosition").writeAttr("values", ::toString(e->foeTrajectory.lanePos)).closeTag();
    //         }
    //         myOutputFile->openTag("foeVelocity").writeAttr("values", ::toString(e->foeTrajectory.v)).closeTag();
    //
    //         myOutputFile->openTag("conflictPoint").writeAttr("values", makeStringWithNAs(e->conflictPointSpan)).closeTag();
    //     }
    //
    //     if (myComputeTTC) {
    //         if (mySaveTrajectories) {
    //             myOutputFile->openTag("TTCSpan").writeAttr("values", makeStringWithNAs(e->TTCspan, INVALID_DOUBLE)).closeTag();
    //         }
    //         if (e->minTTC.time == INVALID_DOUBLE) {
    //             if (myWriteNA) {
    //                 myOutputFile->openTag("minTTC").writeAttr("time", "NA").writeAttr("position", "NA").writeAttr("type", "NA").writeAttr("value", "NA").writeAttr("speed", "NA").closeTag();
    //             }
    //         } else {
    //             std::string time = ::toString(e->minTTC.time);
    //             std::string type = ::toString(int(e->minTTC.type));
    //             std::string value = ::toString(e->minTTC.value);
    //             std::string speed = ::toString(e->minTTC.speed);
    //             if (myUseGeoCoords) {
    //                 toGeo(e->minTTC.pos);
    //             }
    //             std::string position = makeStringWithNAs(e->minTTC.pos);
    //             myOutputFile->openTag("minTTC").writeAttr("time", time).writeAttr("position", position).writeAttr("type", type).writeAttr("value", value).writeAttr("speed", speed).closeTag();
    //         }
    //     }
    //     if (myComputeDRAC) {
    //         if (mySaveTrajectories) {
    //             myOutputFile->openTag("DRACSpan").writeAttr("values", makeStringWithNAs(e->DRACspan, {0.0, INVALID_DOUBLE})).closeTag();
    //         }
    //         if (e->maxDRAC.time == INVALID_DOUBLE) {
    //             if (myWriteNA) {
    //                 myOutputFile->openTag("maxDRAC").writeAttr("time", "NA").writeAttr("position", "NA").writeAttr("type", "NA").writeAttr("value", "NA").writeAttr("speed", "NA").closeTag();
    //             }
    //         } else {
    //             std::string time = ::toString(e->maxDRAC.time);
    //             std::string type = ::toString(int(e->maxDRAC.type));
    //             std::string value = ::toString(e->maxDRAC.value);
    //             std::string speed = ::toString(e->maxDRAC.speed);
    //             if (myUseGeoCoords) {
    //                 toGeo(e->maxDRAC.pos);
    //             }
    //             std::string position = makeStringWithNAs(e->maxDRAC.pos);
    //             myOutputFile->openTag("maxDRAC").writeAttr("time", time).writeAttr("position", position).writeAttr("type", type).writeAttr("value", value).writeAttr("speed", speed).closeTag();
    //         }
    //     }
    //     if (myComputePET) {
    //         if (e->PET.time == INVALID_DOUBLE) {
    //             if (myWriteNA) {
    //                 myOutputFile->openTag("PET").writeAttr("time", "NA").writeAttr("position", "NA").writeAttr("type", "NA").writeAttr("value", "NA").writeAttr("speed", "NA").closeTag();
    //             }
    //         } else {
    //             std::string time = ::toString(e->PET.time);
    //             std::string type = ::toString(int(e->PET.type));
    //             std::string value = ::toString(e->PET.value);
    //             std::string speed = ::toString(e->PET.speed);
    //             if (myUseGeoCoords) {
    //                 toGeo(e->PET.pos);
    //             }
    //             std::string position = ::toString(e->PET.pos, myUseGeoCoords ? gPrecisionGeo : gPrecision);
    //             myOutputFile->openTag("PET").writeAttr("time", time).writeAttr("position", position).writeAttr("type", type).writeAttr("value", value).writeAttr("speed", speed).closeTag();
    //         }
    //     }
    //     if (myComputePPET) {
    //         if (mySaveTrajectories) {
    //             myOutputFile->openTag("PPETSpan").writeAttr("values", makeStringWithNAs(e->PPETspan, INVALID_DOUBLE)).closeTag();
    //         }
    //         if (e->minPPET.time == INVALID_DOUBLE) {
    //             if (myWriteNA) {
    //                 myOutputFile->openTag("minPPET").writeAttr("time", "NA").writeAttr("position", "NA").writeAttr("type", "NA").writeAttr("value", "NA").writeAttr("speed", "NA").closeTag();
    //             }
    //         } else {
    //             std::string time = ::toString(e->minPPET.time);
    //             std::string type = ::toString(int(e->minPPET.type));
    //             std::string value = ::toString(e->minPPET.value);
    //             std::string speed = ::toString(e->minPPET.speed);
    //             if (myUseGeoCoords) {
    //                 toGeo(e->minPPET.pos);
    //             }
    //             std::string position = makeStringWithNAs(e->minPPET.pos);
    //             myOutputFile->openTag("minPPET").writeAttr("time", time).writeAttr("position", position).writeAttr("type", type).writeAttr("value", value).writeAttr("speed", speed).closeTag();
    //         }
    //     }
    //     if (myComputeMDRAC) {
    //         if (mySaveTrajectories) {
    //             myOutputFile->openTag("MDRACSpan").writeAttr("values", makeStringWithNAs(e->MDRACspan, {0.0, INVALID_DOUBLE})).closeTag();
    //         }
    //         if (e->maxMDRAC.time == INVALID_DOUBLE) {
    //             if (myWriteNA) {
    //                 myOutputFile->openTag("maxMDRAC").writeAttr("time", "NA").writeAttr("position", "NA").writeAttr("type", "NA").writeAttr("value", "NA").writeAttr("speed", "NA").closeTag();
    //             }
    //         } else {
    //             std::string time = ::toString(e->maxMDRAC.time);
    //             std::string type = ::toString(int(e->maxMDRAC.type));
    //             std::string value = ::toString(e->maxMDRAC.value);
    //             std::string speed = ::toString(e->maxMDRAC.speed);
    //             if (myUseGeoCoords) {
    //                 toGeo(e->maxMDRAC.pos);
    //             }
    //             std::string position = makeStringWithNAs(e->maxMDRAC.pos);
    //             myOutputFile->openTag("maxMDRAC").writeAttr("time", time).writeAttr("position", position).writeAttr("type", type).writeAttr("value", value).writeAttr("speed", speed).closeTag();
    //         }
    //     }
    //     myOutputFile->closeTag();
    // }
    #[allow(unused_variables)]
    pub fn write_out_conflict(&mut self, e: Ptr<MSDevice_SSM_Encounter>) {
        todo!("port MSDevice_SSM::writeOutConflict")
    }

    /// C++: `void MSDevice_SSM::toGeo(Position &)` [static, private]
    /// convert SUMO-positions to geo coordinates (in place)
    // void
    // MSDevice_SSM::toGeo(Position& x) {
    //     GeoConvHelper::getFinal().cartesian2geo(x);
    // }
    #[allow(unused_variables)]
    pub fn to_geo(x: &mut Position) {
        GeoConvHelper::with_final(|g| g.cartesian2geo(x));
    }

    /// C++: `void MSDevice_SSM::toGeo(PositionVector &)` [static, private]
    /// convert SUMO-positions to geo coordinates (in place)
    // void
    // MSDevice_SSM::toGeo(PositionVector& xv) {
    //     for (Position& x : xv) {
    //         if (x != Position::INVALID) {
    //             toGeo(x);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn to_geo_2(x: &mut PositionVector) {
        for x in x.iter_mut() {
            if *x != Position::INVALID {
                MSDevice_SSM::to_geo(x);
            }
        }
    }

    /// C++: `void MSDevice_SSM::cleanup()` [static]
    /// Clean up remaining devices instances
    // void
    // MSDevice_SSM::cleanup() {
    //     // Close current encounters and flush conflicts to file for all existing devices
    //     if (myInstances != nullptr) {
    //         for (MSDevice_SSM* device : *myInstances) {
    //             device->resetEncounters();
    //             device->flushConflicts(true);
    //             device->flushGlobalMeasures();
    //         }
    //         myInstances->clear();
    //     }
    //     for (const std::string& fn : myCreatedOutputFiles) {
    //         OutputDevice::getDevice(fn).closeTag();
    //     }
    //     myCreatedOutputFiles.clear();
    //     myEdgeFilter.clear();
    //     myEdgeFilterInitialized = false;
    //     myEdgeFilterActive = false;
    // }
    #[allow(unused_variables)]
    pub fn cleanup() {
        todo!("port MSDevice_SSM::cleanup")
    }

    /// C++: `void MSDevice_SSM::~MSDevice_SSM()` [virtual]
    /// Destructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSDevice_SSM::~MSDevice_SSM")
    }

    /// C++: `void MSDevice_SSM::findSurroundingVehicles(const MSVehicle &, double, FoeInfoMap &)` [static]
    /// Returns all vehicles, which are within the given range of the given vehicle.
    // void
    // MSDevice_SSM::findSurroundingVehicles(const MSVehicle& veh, double range, FoeInfoMap& foeCollector) {
    //     if (!veh.isOnRoad()) {
    //         return;
    //     }
    // #ifdef DEBUG_SSM_SURROUNDING
    //
    //     gDebugFlag3 = DEBUG_COND_FIND(veh);
    //     if (gDebugFlag3) {
    //         std::cout << SIMTIME << " Looking for surrounding vehicles for ego vehicle '" << veh.getID()
    //                   << "' on edge '" << veh.getLane()->getEdge().getID()
    //                   << "'."
    //                   << "\nVehicle's best lanes = " << ::toString(veh.getBestLanesContinuation())
    //                   << std::endl;
    //     }
    // #endif
    //
    //
    //     // The requesting vehicle's current route
    //     // XXX: Restriction to route scanning may have to be generalized to scanning of possible continuations when
    //     //      considering situations involving sudden route changes. See also the definition of the EncounterTypes.
    //     //      A second problem is that following situations on deviating routes may result in closing encounters
    //     //      too early if a leading foe is not traced on its new lane. (see test 'foe_leader_deviating_routes')
    //
    //     // If veh is on an internal edge, the edgeIter points towards the last edge before the junction
    //     //ConstMSEdgeVector::const_iterator edgeIter = veh.getCurrentRouteEdge();
    //     //assert(*edgeIter != 0);
    //
    //     // Best continuation lanes for the ego vehicle
    //     std::vector<MSLane*> egoBestLanes = veh.getBestLanesContinuation();
    //
    //     // current lane in loop below
    //     const MSLane* lane = veh.getLane();
    //     const MSEdge* egoEdge = &(lane->getEdge());
    //     const bool isOpposite = veh.getLaneChangeModel().isOpposite();
    //     std::vector<MSLane*>::const_iterator laneIter = egoBestLanes.begin();
    //     assert(lane->isInternal() || lane == *laneIter || isOpposite);
    //     assert(lane != 0);
    //     if (lane->isInternal() && egoBestLanes[0] != nullptr) { // outdated BestLanes, see #11336
    //         return;
    //     }
    //
    //     if (isOpposite) {
    //         for (int i = 0; i < (int)egoBestLanes.size(); i++) {
    //             if (egoBestLanes[i] != nullptr && egoBestLanes[i]->getEdge().getOppositeEdge() != nullptr) {
    //                 egoBestLanes[i] = egoBestLanes[i]->getEdge().getOppositeEdge()->getLanes().back();
    //             }
    //         }
    //     }
    //
    //     // next non-internal lane on the route
    //     const MSLane* nextNonInternalLane = nullptr;
    //
    //     const MSEdge* edge; // current edge in loop below
    //
    //     // Init pos with vehicle's current position. Below pos is set to zero to denote
    //     // the beginning position of the currently considered edge
    //     double pos = veh.getPositionOnLane();
    //     // remainingDownstreamRange is the range minus the distance that is already scanned downstream along the vehicles route
    //     double remainingDownstreamRange = range;
    //     // distToConflictLane is the distance of the ego vehicle to the start of the currently considered potential conflict lane (can be negative for its current lane)
    //     double distToConflictLane = isOpposite ? pos - veh.getLane()->getLength() : -pos;
    //
    //     // remember already visited lanes (no matter whether internal or not) and junctions downstream along the route
    //     std::set<const MSLane*> seenLanes;
    //     std::set<const MSJunction*> routeJunctions;
    //
    //     // Starting points for upstream scans to be executed after downstream scan is complete.
    //     // Holds pairs (starting edge, starting position on edge)
    //     std::vector<UpstreamScanStartInfo> upstreamScanStartPositions;
    //
    //
    //     // if the current edge is internal, collect all vehicles from the junction and within upstream range (except on the vehicles own edge),
    //     // this is analogous to the code treating junctions in the loop below. Note that the distance on the junction itself is not included into
    //     // range, so vehicles farther away than range can be collected, too.
    //     if (lane->isInternal()) {
    //         edge = &(lane->getEdge());
    //
    // #ifdef DEBUG_SSM_SURROUNDING
    //         if (gDebugFlag3) {
    //             std::cout << SIMTIME << " Vehicle '" << veh.getID() << "' is on internal edge " << edge->getID() << "'." << std::endl;
    // //                  << "Previous edge of its route: '" << (*edgeIter)->getID() << "'" << std::endl;
    //         }
    // #endif
    //
    //         assert(edge->getToJunction() == edge->getFromJunction());
    //
    //         const MSJunction* junction = edge->getToJunction();
    //         // Collect vehicles on the junction
    //         getVehiclesOnJunction(junction, lane, distToConflictLane, lane, foeCollector, seenLanes);
    //         routeJunctions.insert(junction);
    //
    //         // Collect vehicles on incoming edges.
    //         // Note that this includes the previous edge on the ego vehicle's route.
    //         // (The distance on the current internal edge is ignored)
    //         const ConstMSEdgeVector& incoming = junction->getIncoming();
    //         for (ConstMSEdgeVector::const_iterator ei = incoming.begin(); ei != incoming.end(); ++ei) {
    //             if ((*ei)->isInternal()) {
    //                 continue;
    //             }
    //             // Upstream range is taken from the vehicle's back
    //             upstreamScanStartPositions.push_back(UpstreamScanStartInfo(*ei, (*ei)->getLength(), range + veh.getLength(), distToConflictLane, lane));
    //         }
    //
    // //        // Take into account internal distance covered on the current lane
    // //        (commented out, because upstream scanning disregards internal lanes on the last scanned junction
    // //        -- this makes the scanning symmetric between leader and follower)
    // //        remainingDownstreamRange -= lane->getLength() - pos;
    //
    //         // Take into account non-internal lengths until next non-internal lane
    //         MSLink* link = lane->getLinkCont()[0];
    //         remainingDownstreamRange -= link->getInternalLengthsAfter();
    //         distToConflictLane += lane->getLength() + link->getInternalLengthsAfter();
    //
    //         // The next non-internal lane
    //         pos = 0.;
    //         lane = *(++laneIter);
    //         edge = &lane->getEdge();
    //     } else {
    //         // Collect all vehicles in range behind ego vehicle
    //         edge = &(lane->getEdge());
    //         double edgeLength = edge->getLength();
    //         double startScanPos = std::min(pos + remainingDownstreamRange, edgeLength);
    //         upstreamScanStartPositions.push_back(UpstreamScanStartInfo(edge, startScanPos, std::max(0., startScanPos - pos + range + veh.getLength()), distToConflictLane, lane));
    //     }
    //
    //     assert(lane != 0);
    //     assert(!lane->isInternal());
    //
    //     // Advance downstream the ego vehicle's route for distance 'range'.
    //     // Collect all vehicles on the traversed Edges and on incoming edges at junctions
    //     // and starting points for upstream vehicle collection strated below after downstream scan.
    //     while (remainingDownstreamRange > 0.) {
    //
    // #ifdef DEBUG_SSM_SURROUNDING
    //         if (gDebugFlag3) {
    //             std::cout << SIMTIME << " Scanning downstream for vehicle '" << veh.getID() << "' on lane '" << veh.getLane()->getID() << "', position=" << pos << ".\n"
    //                       << "Considering edge '" << edge->getID() << "' Remaining downstream range = " << remainingDownstreamRange
    //                       << "\nbestLanes=" << ::toString(egoBestLanes) << "\n"
    //                       << std::endl;
    //         }
    // #endif
    //         assert(!edge->isInternal());
    //         assert(!lane->isInternal());
    //         assert(pos == 0 || lane == veh.getLane());
    //         if (pos + remainingDownstreamRange < lane->getLength()) {
    //             // scan range ends on this lane
    //             if (edge->getID() != egoEdge->getID()) {
    //                 upstreamScanStartPositions.push_back(UpstreamScanStartInfo(edge, pos + remainingDownstreamRange, remainingDownstreamRange, distToConflictLane, lane));
    //             }
    //             // scanned required downstream range
    //             break;
    //         } else {
    //             // Also need to scan area that reaches beyond the lane
    //             // Collecting vehicles on non-internal edge ahead
    //             if (edge->getID() != egoEdge->getID()) {
    //                 upstreamScanStartPositions.push_back(UpstreamScanStartInfo(edge, edge->getLength(), edge->getLength() - pos, distToConflictLane, lane));
    //             }
    //             // account for scanned distance on lane
    //             remainingDownstreamRange -= lane->getLength() - pos;
    //             distToConflictLane += lane->getLength();
    //             pos = 0.;
    //
    //             // proceed to next non-internal lane
    //             ++laneIter;
    //             assert(laneIter == egoBestLanes.end() || *laneIter != 0);
    //
    //             // If the vehicle's best lanes go on, collect vehicles on the upcoming junction
    //             if (laneIter != egoBestLanes.end()) {
    //                 // Upcoming junction
    //                 const MSJunction* junction;
    //                 if (isOpposite) {
    //                     junction = lane->getParallelOpposite()->getEdge().getToJunction();
    //                 } else {
    //                     junction = lane->getEdge().getToJunction();
    //                 }
    //
    //
    //                 // Find connection for ego on the junction
    //                 nextNonInternalLane = *laneIter;
    //                 const MSLink* link = lane->getLinkTo(nextNonInternalLane);
    //                 if (isOpposite && link == nullptr) {
    //                     link = nextNonInternalLane->getLinkTo(lane);
    //                     if (link == nullptr) {
    //                         link = lane->getParallelOpposite()->getLinkTo(nextNonInternalLane);
    //                     }
    //                 }
    //                 if (link == nullptr) {
    //                     // disconnected route
    //                     break;
    //                 }
    //
    //                 // First lane of the connection
    //                 lane = link->getViaLane();
    //                 if (lane == nullptr) {
    //                     // link without internal lane
    //                     lane = nextNonInternalLane;
    //                     edge = &(lane->getEdge());
    //                     if (seenLanes.count(lane) == 0) {
    //                         seenLanes.insert(lane);
    //                         continue;
    //                     } else {
    //                         break;
    //                     }
    //                 }
    //
    //                 if (seenLanes.count(lane) == 0) {
    //                     // Collect vehicles on the junction, if it wasn't considered already
    //                     getVehiclesOnJunction(junction, lane, distToConflictLane, lane, foeCollector, seenLanes);
    //                     routeJunctions.insert(junction);
    //
    //                     // Collect vehicles on incoming edges (except the last edge, where we already collected). Use full range.
    //                     if (isOpposite) {
    //                         // look for vehicles that are also driving on the opposite side behind ego
    //                         const ConstMSEdgeVector& outgoing = junction->getOutgoing();
    //                         for (ConstMSEdgeVector::const_iterator ei = outgoing.begin(); ei != outgoing.end(); ++ei) {
    //                             if (*ei == edge || (*ei)->isInternal()) {
    //                                 continue;
    //                             }
    //                             upstreamScanStartPositions.push_back(UpstreamScanStartInfo(*ei, (*ei)->getLength(), range, distToConflictLane, lane));
    //                         }
    //                     } else {
    //                         const ConstMSEdgeVector& incoming = junction->getIncoming();
    //                         for (ConstMSEdgeVector::const_iterator ei = incoming.begin(); ei != incoming.end(); ++ei) {
    //                             if (*ei == edge || (*ei)->isInternal()) {
    //                                 continue;
    //                             }
    //                             upstreamScanStartPositions.push_back(UpstreamScanStartInfo(*ei, (*ei)->getLength(), range, distToConflictLane, lane));
    //                         }
    //                     }
    //
    //                     // account for scanned distance on junction
    //                     double linkLength = link->getInternalLengthsAfter();
    //                     remainingDownstreamRange -= linkLength;
    //                     distToConflictLane += linkLength;
    // #ifdef DEBUG_SSM_SURROUNDING
    //                     if (gDebugFlag3) {
    //                         std::cout << "    Downstream Scan for vehicle '" << veh.getID() << "' proceeded over junction '" << junction->getID()
    //                                   << "',\n    linkLength=" << linkLength << ", remainingDownstreamRange=" << remainingDownstreamRange
    //                                   << std::endl;
    //                     }
    // #endif
    //
    //                     // update ego's lane to next non internal edge
    //                     lane = nextNonInternalLane;
    //                     edge = &(lane->getEdge());
    //                 } else {
    // #ifdef DEBUG_SSM_SURROUNDING
    //                     if (gDebugFlag3) {
    //                         std::cout << "    Downstream Scan for vehicle '" << veh.getID() << "' stops at lane '" << lane->getID()
    //                                   << "', which has already been scanned."
    //                                   << std::endl;
    //                     }
    // #endif
    //                     break;
    //                 }
    //             } else {
    //                 // Further vehicle path unknown, break search
    //                 break;
    //             }
    //         }
    //     }
    //     // add junction from the end of the route
    //     routeJunctions.insert(lane->getEdge().getToJunction());
    //
    //
    //     // Scan upstream branches from collected starting points
    //     for (UpstreamScanStartInfo& i : upstreamScanStartPositions) {
    //         getUpstreamVehicles(i, foeCollector, seenLanes, routeJunctions);
    //     }
    //
    // #ifdef DEBUG_SSM_SURROUNDING
    //     if (gDebugFlag3) {
    //         for (std::pair<const MSVehicle*, FoeInfo*> foeInfo : foeCollector) {
    //             std::cout << "    foe " << foeInfo.first->getID() << " conflict at " << foeInfo.second->egoConflictLane->getID() << " egoDist " << foeInfo.second->egoDistToConflictLane << std::endl;
    //         }
    //     }
    // #endif
    //
    //     // remove ego vehicle
    //     const auto& it = foeCollector.find(&veh);
    //     if (it != foeCollector.end()) {
    //         delete it->second;
    //         foeCollector.erase(it);
    //     }
    //     gDebugFlag3 = false;
    // }
    #[allow(unused_variables)]
    pub fn find_surrounding_vehicles(veh: &MSVehicle, range: f64, foeCollector: &mut MSDevice_SSM_FoeInfoMap) {
        todo!("port MSDevice_SSM::findSurroundingVehicles")
    }

    /// C++: `void MSDevice_SSM::getUpstreamVehicles(const UpstreamScanStartInfo &, FoeInfoMap &, int &, const int &)` [static]
    /// Collects all vehicles within range 'range' upstream of the position 'pos' on the edge 'edge' into foeCollector
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_upstream_vehicles(scanStart: &MSDevice_SSM_UpstreamScanStartInfo, foeCollector: &mut MSDevice_SSM_FoeInfoMap, seenLanes: &mut i32, routeJunctions: &i32) {
        todo!("port MSDevice_SSM::getUpstreamVehicles")
    }

    /// C++: `void MSDevice_SSM::getVehiclesOnJunction(const MSJunction *, const MSLane *, double, const MSLane *const, FoeInfoMap &, int &)` [static]
    /// Collects all vehicles on the junction into foeCollector
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_vehicles_on_junction(arg0: Ptr<MSJunction>, egoJunctionLane: Ptr<MSLane>, egoDistToConflictLane: f64, egoConflictLane: Ptr<MSLane>, foeCollector: &mut MSDevice_SSM_FoeInfoMap, seenLanes: &mut i32) {
        todo!("port MSDevice_SSM::getVehiclesOnJunction")
    }

    /// C++: `bool MSDevice_SSM::notifyMove(SUMOTrafficObject &, double, double, double)` [virtual]
    /// Checks for waiting steps when the vehicle moves
    // bool
    // MSDevice_SSM::notifyMove(SUMOTrafficObject& veh, double /* oldPos */,
    //                          double /* newPos */, double newSpeed) {
    // #ifdef DEBUG_SSM_NOTIFICATIONS
    //     MSBaseVehicle* v = (MSBaseVehicle*) &veh;
    //     if (DEBUG_COND(v)) {
    //         std::cout << SIMTIME << "device '" << getID() << "' notifyMove: newSpeed=" << newSpeed << "\n";
    //     }
    // #else
    //     UNUSED_PARAMETER(veh);
    //     UNUSED_PARAMETER(newSpeed);
    // #endif
    //     return true; // keep the device
    // }
    #[allow(unused_variables)]
    pub fn notify_move(&mut self, veh: &mut SUMOTrafficObject, oldPos: f64, newPos: f64, newSpeed: f64) -> bool {
        todo!("port MSDevice_SSM::notifyMove")
    }

    /// C++: `bool MSDevice_SSM::notifyEnter(SUMOTrafficObject &, MSMoveReminder::Notification, const MSLane *)` [virtual]
    /// Called whenever the holder enteres a lane
    // bool
    // MSDevice_SSM::notifyEnter(SUMOTrafficObject& veh, MSMoveReminder::Notification reason, const MSLane* /* enteredLane */) {
    //     assert(veh.isVehicle());
    // #ifdef DEBUG_SSM_NOTIFICATIONS
    //     MSBaseVehicle* v = (MSBaseVehicle*) &veh;
    //     if (DEBUG_COND(v)) {
    //         std::cout << SIMTIME << "device '" << getID() << "' notifyEnter: reason=" << reason << " currentEdge=" << v->getLane()->getEdge().getID() << "\n";
    //     }
    // #else
    //     UNUSED_PARAMETER(veh);
    //     UNUSED_PARAMETER(reason);
    // #endif
    //     return true; // keep the device
    // }
    #[allow(unused_variables)]
    pub fn notify_enter(&mut self, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        true // keep the device
    }

    /// C++: `bool MSDevice_SSM::notifyLeave(SUMOTrafficObject &, double, MSMoveReminder::Notification, const MSLane *)` [virtual]
    /// Called whenever the holder leaves a lane
    // bool
    // MSDevice_SSM::notifyLeave(SUMOTrafficObject& veh, double /*lastPos*/,
    //                           MSMoveReminder::Notification reason, const MSLane* /* enteredLane */) {
    //     assert(veh.isVehicle());
    // #ifdef DEBUG_SSM_NOTIFICATIONS
    //     MSBaseVehicle* v = (MSBaseVehicle*) &veh;
    //     if (DEBUG_COND(v)) {
    //         std::cout << SIMTIME << "device '" << getID() << "' notifyLeave: reason=" << reason << " currentEdge=" << v->getLane()->getEdge().getID() << "\n";
    //     }
    // #else
    //     UNUSED_PARAMETER(veh);
    //     UNUSED_PARAMETER(reason);
    // #endif
    //     return true; // keep the device
    // }
    #[allow(unused_variables)]
    pub fn notify_leave(&mut self, veh: &mut SUMOTrafficObject, lastPos: f64, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        true // keep the device
    }

    /// C++: `const std::string MSDevice_SSM::deviceName() const` [const, virtual]
    /// return the name for this type of device
    // const std::string deviceName() const {
    //         return "ssm";
    //     }
    #[allow(unused_variables)]
    pub fn device_name(&self) -> String {
        "ssm".to_string()
    }

    /// C++: `void MSDevice_SSM::generateOutput(OutputDevice *) const` [const, virtual]
    /// Finalizes output. Called on vehicle removal
    // void
    // MSDevice_SSM::generateOutput(OutputDevice* /*tripinfoOut*/) const {
    //     // This is called once at vehicle removal.
    //     //       Also: flush myOutputFile? Or is this done automatically?
    //     // myOutputFile->closeTag();
    // }
    #[allow(unused_variables)]
    pub fn generate_output(&self, tripinfoOut: Ptr<OutputDevice>) {
        // C++ body is only comments (closeTag on myOutputFile is disabled upstream): nothing to do.
    }

    /// C++: `void MSDevice_SSM::MSDevice_SSM(SUMOVehicle &, const std::string &, std::string, int, bool, double, double, bool, bool, bool, int)` [private]
    /// Constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new(holder: &mut SUMOVehicle, id: &str, outputFilename: String, thresholds: i32, trajectories: bool, range: f64, extraTime: f64, useGeoCoords: bool, writePositions: bool, writeLanesPositions: bool, conflictOrder: i32) -> Self {
        todo!("port MSDevice_SSM::MSDevice_SSM")
    }

    /// C++: `void MSDevice_SSM::processEncounters(FoeInfoMap &, bool)` [private]
    /// Finds encounters for which the foe vehicle has disappeared from range. remainingExtraTime is decreased until it reaches zero, which triggers closing the encounter. If an ended encounter is qualified as a conflict, it is transferred to myPastConflicts All vehicles for which an encounter instance already exists (vehicle is already tracked) are removed from 'foes' during processing.
    // void
    // MSDevice_SSM::processEncounters(FoeInfoMap& foes, bool forceClose) {
    // #ifdef DEBUG_SSM
    //     if (DEBUG_COND(myHolderMS)) {
    //         std::cout << "\n" << SIMTIME << " Device '" << getID() << "' processEncounters(forceClose = " << forceClose << ")" << std::endl;
    //         std::cout << "Currently present foes:\n";
    //         for (FoeInfoMap::const_iterator vi = foes.begin(); vi != foes.end(); ++vi) {
    //             std::cout << vi->first->getID() << "\n";
    //         }
    //         std::cout << std::endl;
    //     }
    // #endif
    //
    //     // Run through active encounters. If corresponding foe is still present in foes update and
    //     // remove foe from foes. If the foe has disappeared close the encounter (check if it qualifies
    //     // as a conflict and in case transfer it to myPastConflicts).
    //     // Afterwards run through remaining elements in foes and create new encounters for them.
    //     EncounterVector::iterator ei = myActiveEncounters.begin();
    //     while (ei != myActiveEncounters.end()) {
    //         Encounter* e = *ei;
    //         // check whether foe is still on net
    //         bool foeExists = !(MSNet::getInstance()->getVehicleControl().getVehicle(e->foeID) == nullptr);
    //         if (!foeExists) {
    //             e->foe = nullptr;
    //         }
    //         if (foes.find(e->foe) != foes.end()) {
    //             FoeInfo* foeInfo = foes[e->foe];
    //             EncounterType prevType = e->currentType;
    //             // Update encounter
    //             updateEncounter(e, foeInfo);
    //             if (prevType == ENCOUNTER_TYPE_BOTH_LEFT_CONFLICT_AREA
    //                     && e->currentType != ENCOUNTER_TYPE_BOTH_LEFT_CONFLICT_AREA) {
    //                 // The encounter classification switched from BOTH_LEFT to another
    //                 // => Start new encounter (i.e. don't erase the foe, don't delete the foeInfo and request closing)
    //                 // Note that updateEncounter did not add another trajectory point in this case.
    // #ifdef DEBUG_SSM
    //                 if (DEBUG_COND(myHolderMS)) {
    //                     std::cout << "  Requesting encounter closure because both left conflict area of previous encounter but another encounter lies ahead." << std::endl;
    //                 }
    // #endif
    //                 e->closingRequested = true;
    //             } else {
    //                 // Erase foes which were already encountered and should not be used to open a new conflict
    //                 delete foeInfo;
    //                 foes.erase(e->foe);
    //             }
    //         } else {
    //             if (e->getRemainingExtraTime() <= 0. || forceClose || !foeExists) {
    //                 // Close encounter, extra time has expired (deletes e if it does not qualify as conflict)
    // #ifdef DEBUG_SSM
    //                 if (DEBUG_COND(myHolderMS)) {
    //                     std::cout << "  Requesting encounter closure because..." << std::endl;
    //                     if (e->getRemainingExtraTime() <= 0.) {
    //                         std::cout << "  ... extra time elapsed." << std::endl;
    //                     } else if (forceClose) {
    //                         std::cout << "  ... closing was forced." << std::endl;
    //                     } else {
    //                         std::cout << "  ... foe disappeared." << std::endl;
    //                     }
    //                 }
    // #endif
    //                 e->closingRequested = true;
    //             } else {
    //                 updateEncounter(e, nullptr); // counts down extra time
    //             }
    //         }
    //
    //         if (e->closingRequested) {
    //             double eBegin = e->begin;
    //             closeEncounter(e);
    //             ei = myActiveEncounters.erase(ei);
    //             if (myActiveEncounters.empty()) {
    //                 myOldestActiveEncounterBegin = INVALID_DOUBLE;
    //             } else if (eBegin == myOldestActiveEncounterBegin) {
    //                 // Erased the oldest encounter, update myOldestActiveEncounterBegin
    //                 auto i = myActiveEncounters.begin();
    //                 myOldestActiveEncounterBegin = (*i++)->begin;
    //                 while (i != myActiveEncounters.end()) {
    //                     myOldestActiveEncounterBegin = MIN2(myOldestActiveEncounterBegin, (*i++)->begin);
    //                 }
    //             }
    //         } else {
    //             ++ei;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn process_encounters(&mut self, foes: &mut MSDevice_SSM_FoeInfoMap, forceClose: bool) {
        todo!("port MSDevice_SSM::processEncounters")
    }

    /// C++: `void MSDevice_SSM::storeEncountersExceedingMaxLength()` [private]
    /// Closes encounters, whose duration exceeds the maximal encounter length. If it is classified as conflict, the encounter is saved. In any case, a new active encounter is created holding the trailing part (of length myOverlapTime) of the original.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn store_encounters_exceeding_max_length(&mut self) {
        todo!("port MSDevice_SSM::storeEncountersExceedingMaxLength")
    }

    /// C++: `void MSDevice_SSM::createEncounters(FoeInfoMap &)` [private]
    /// Makes new encounters for all given vehicles (these should be the ones entering the device's range in the current timestep)
    // void
    // MSDevice_SSM::createEncounters(FoeInfoMap& foes) {
    // #ifdef DEBUG_SSM
    //     if (DEBUG_COND(myHolderMS)) {
    //         std::cout << "\n" << SIMTIME << " Device '" << getID() << "' createEncounters()" << std::endl;
    //         std::cout << "New foes:\n";
    //         for (FoeInfoMap::const_iterator vi = foes.begin(); vi != foes.end(); ++vi) {
    //             std::cout << vi->first->getID() << "\n";
    //         }
    //         std::cout << std::endl;
    //     }
    // #endif
    //
    //     for (FoeInfoMap::const_iterator foe = foes.begin(); foe != foes.end(); ++foe) {
    //         Encounter* e = new Encounter(myHolderMS, foe->first, SIMTIME, myExtraTime);
    //         if (updateEncounter(e, foe->second)) {
    //             if (myOldestActiveEncounterBegin == INVALID_DOUBLE) {
    //                 assert(myActiveEncounters.empty());
    //                 myOldestActiveEncounterBegin = e->begin;
    //             }
    //             assert(myOldestActiveEncounterBegin <= e->begin);
    //             myActiveEncounters.push_back(e);
    //         } else {
    //             // Discard encounters, where one vehicle already left the conflict area
    //             delete e;
    //         }
    //         // free foeInfo
    //         delete foe->second;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn create_encounters(&mut self, foes: &mut MSDevice_SSM_FoeInfoMap) {
        todo!("port MSDevice_SSM::createEncounters")
    }

    /// C++: `void MSDevice_SSM::computeGlobalMeasures()` [private]
    /// Stores measures, that are not associated to a specific encounter as headways and brake rates
    // void
    // MSDevice_SSM::computeGlobalMeasures() {
    //     if (myComputeBR || myComputeSGAP || myComputeTGAP) {
    //         myGlobalMeasuresTimeSpan.push_back(SIMTIME);
    //         if (myWritePositions) {
    //             myGlobalMeasuresPositions.push_back(myHolderMS->getPosition());
    //         }
    //         if (myWriteLanesPositions) {
    //             myGlobalMeasuresLaneIDs.push_back(myHolderMS->getLane()->getID());
    //             myGlobalMeasuresLanesPositions.push_back(myHolderMS->getPositionOnLane());
    //         }
    //         if (myComputeBR) {
    //             double br = MAX2(-myHolderMS->getAcceleration(), 0.0);
    //             if (br > myMaxBR.second) {
    //                 myMaxBR = std::make_pair(std::make_pair(SIMTIME, myHolderMS->getPosition()), br);
    //             }
    //             myBRspan.push_back(br);
    //         }
    //
    //         double leaderSearchDist = 0;
    //         std::pair<const MSVehicle*, double> leader(nullptr, 0.);
    //         if (myComputeSGAP) {
    //             leaderSearchDist = myThresholds["SGAP"];
    //         }
    //         if (myComputeTGAP) {
    //             leaderSearchDist = MAX2(leaderSearchDist, myThresholds["TGAP"] * myHolderMS->getSpeed());
    //         }
    //
    //         if (leaderSearchDist > 0.) {
    //             leader = myHolderMS->getLeader(leaderSearchDist);
    //         }
    //
    //         // negative gap indicates theoretical car-following relationship for paths that cross at an intersection
    //         if (myComputeSGAP) {
    //             if (leader.first == nullptr || leader.second < 0) {
    //                 mySGAPspan.push_back(INVALID_DOUBLE);
    //             } else {
    //                 double sgap = leader.second + myHolder.getVehicleType().getMinGap();
    //                 mySGAPspan.push_back(sgap);
    //                 if (sgap < myMinSGAP.first.second) {
    //                     myMinSGAP = std::make_pair(std::make_pair(std::make_pair(SIMTIME, myHolderMS->getPosition()), sgap), leader.first->getID());
    //                 }
    //             }
    //         }
    //
    //         if (myComputeTGAP) {
    //             if (leader.first == nullptr || myHolderMS->getSpeed() == 0. || leader.second < 0) {
    //                 myTGAPspan.push_back(INVALID_DOUBLE);
    //             } else {
    //                 const double tgap = (leader.second + myHolder.getVehicleType().getMinGap()) / myHolderMS->getSpeed();
    //                 myTGAPspan.push_back(tgap);
    //                 if (tgap < myMinTGAP.first.second) {
    //                     myMinTGAP = std::make_pair(std::make_pair(std::make_pair(SIMTIME, myHolderMS->getPosition()), tgap), leader.first->getID());
    //                 }
    //             }
    //         }
    //
    //     }
    // }
    #[allow(unused_variables)]
    pub fn compute_global_measures(&mut self) {
        todo!("port MSDevice_SSM::computeGlobalMeasures")
    }

    /// C++: `void MSDevice_SSM::resetEncounters()` [private]
    /// Closes all current Encounters and moves conflicts to myPastConflicts,
    // void
    // MSDevice_SSM::resetEncounters() {
    //     // Call processEncounters() with empty vehicle set
    //     FoeInfoMap foes;
    //     // processEncounters with empty argument closes all encounters
    //     processEncounters(foes, true);
    // }
    #[allow(unused_variables)]
    pub fn reset_encounters(&mut self) {
        todo!("port MSDevice_SSM::resetEncounters")
    }

    /// C++: `void MSDevice_SSM::flushConflicts(bool)` [private]
    /// Writes out all past conflicts that have begun earlier than the oldest active encounter
    // void
    // MSDevice_SSM::flushConflicts(bool flushAll) {
    // #ifdef DEBUG_SSM
    //     if (DEBUG_COND(myHolderMS)) {
    //         std::cout << "\n" << SIMTIME << " Device '" << getID() << "' flushConflicts past=" << myPastConflicts.size()
    //                   << " oldestActive=" << (myOldestActiveEncounterBegin == INVALID_DOUBLE ? -1 : myOldestActiveEncounterBegin)
    //                   << " topBegin=" << (myPastConflicts.size() > 0 ? myPastConflicts.top()->begin : -1)
    //                   << "\n";
    //     }
    // #endif
    //     while (!myPastConflicts.empty()) {
    //         Encounter* top = myPastConflicts.top();
    //         if (flushAll || top->begin <= myOldestActiveEncounterBegin) {
    //             bool write = true;
    //             if (myFilterConflictTypes) {
    //                 std::vector<int> foundTypes;
    //                 std::set<int> encounterTypes(top->typeSpan.begin(), top->typeSpan.end());
    //                 std::set_intersection(
    //                     myDroppedConflictTypes.begin(), myDroppedConflictTypes.end(),
    //                     encounterTypes.begin(), encounterTypes.end(),
    //                     std::back_inserter(foundTypes));
    //                 write = foundTypes.size() == 0;
    //             }
    //             if (write) {
    //                 writeOutConflict(top);
    //             }
    //             myPastConflicts.pop();
    //             delete top;
    //         } else {
    //             break;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn flush_conflicts(&mut self, all: bool) {
        todo!("port MSDevice_SSM::flushConflicts")
    }

    /// C++: `void MSDevice_SSM::flushGlobalMeasures()` [private]
    /// Write out all non-encounter specific measures as headways and braking rates.
    // void
    // MSDevice_SSM::flushGlobalMeasures() {
    //     std::string egoID = myHolderMS->getID();
    // #ifdef DEBUG_SSM
    //     if (DEBUG_COND(myHolderMS))
    //         std::cout << SIMTIME << " flushGlobalMeasures() of vehicle '"
    //                   << egoID << "'"
    //                   << "'\ntoGeo=" << myUseGeoCoords << std::endl;
    // #endif
    //     if (myComputeBR || myComputeSGAP || myComputeTGAP) {
    //         myOutputFile->openTag("globalMeasures");
    //         myOutputFile->writeAttr("ego", egoID);
    //         myOutputFile->openTag("timeSpan").writeAttr("values", myGlobalMeasuresTimeSpan).closeTag();
    //         if (myWritePositions) {
    //             myOutputFile->openTag("positions").writeAttr("values", myGlobalMeasuresPositions).closeTag();
    //         }
    //         if (myWriteLanesPositions) {
    //             myOutputFile->openTag("lane").writeAttr("values", myGlobalMeasuresLaneIDs).closeTag();
    //             myOutputFile->openTag("lanePosition").writeAttr("values", myGlobalMeasuresLanesPositions).closeTag();
    //         }
    //         if (myComputeBR) {
    //             myOutputFile->openTag("BRSpan").writeAttr("values", myBRspan).closeTag();
    //
    //             if (myMaxBR.second != 0.0) {
    //                 if (myUseGeoCoords) {
    //                     toGeo(myMaxBR.first.second);
    //                 }
    //                 myOutputFile->openTag("maxBR").writeAttr("time", myMaxBR.first.first).writeAttr("position", makeStringWithNAs(myMaxBR.first.second)).writeAttr("value", myMaxBR.second).closeTag();
    //             }
    //         }
    //
    //         if (myComputeSGAP) {
    //             myOutputFile->openTag("SGAPSpan").writeAttr("values", makeStringWithNAs(mySGAPspan, INVALID_DOUBLE)).closeTag();
    //             if (myMinSGAP.second != "") {
    //                 if (myUseGeoCoords) {
    //                     toGeo(myMinSGAP.first.first.second);
    //                 }
    //                 myOutputFile->openTag("minSGAP").writeAttr("time", myMinSGAP.first.first.first)
    //                 .writeAttr("position", makeStringWithNAs(myMinSGAP.first.first.second))
    //                 .writeAttr("value", myMinSGAP.first.second)
    //                 .writeAttr("leader", myMinSGAP.second).closeTag();
    //             }
    //         }
    //
    //         if (myComputeTGAP) {
    //             myOutputFile->openTag("TGAPSpan").writeAttr("values", makeStringWithNAs(myTGAPspan, INVALID_DOUBLE)).closeTag();
    //             if (myMinTGAP.second != "") {
    //                 if (myUseGeoCoords) {
    //                     toGeo(myMinTGAP.first.first.second);
    //                 }
    //                 myOutputFile->openTag("minTGAP").writeAttr("time", myMinTGAP.first.first.first)
    //                 .writeAttr("position", makeStringWithNAs(myMinTGAP.first.first.second))
    //                 .writeAttr("value", myMinTGAP.first.second)
    //                 .writeAttr("leader", myMinTGAP.second).closeTag();
    //             }
    //         }
    //         // close globalMeasures
    //         myOutputFile->closeTag();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn flush_global_measures(&mut self) {
        todo!("port MSDevice_SSM::flushGlobalMeasures")
    }

    /// C++: `bool MSDevice_SSM::updateEncounter(Encounter *, FoeInfo *)` [private]
    /// Updates the encounter (adds a new trajectory point).
    // bool
    // MSDevice_SSM::updateEncounter(Encounter* e, FoeInfo* foeInfo) {
    // #ifdef DEBUG_ENCOUNTER
    //     if (DEBUG_COND_ENCOUNTER(e)) {
    //         std::cout << SIMTIME << " updateEncounter() of vehicles '" << e->egoID << "' and '" << e->foeID << "'\n";
    //     }
    // #endif
    //     assert(e->foe != 0);
    //
    //     // Struct storing distances (determined in classifyEncounter()) and times to potential conflict entry / exit (in estimateConflictTimes())
    //     EncounterApproachInfo eInfo(e);
    //
    //     // Classify encounter type based on the present information
    //     // More details on follower/lead relation are determined in a second step below, see estimateConflictTimes()
    //     // If a crossing situation is ongoing (i.e. one of the vehicles entered the conflict area already in the last step,
    //     // this is handled by passedEncounter by only tracing the vehicle's movements)
    //     // The further development of the encounter type is done in checkConflictEntryAndExit()
    //     eInfo.type = classifyEncounter(foeInfo, eInfo);
    //
    //     // Discard new encounters, where one vehicle has already left the conflict area
    //     if (eInfo.encounter->size() == 0) {
    //         if (eInfo.type == ENCOUNTER_TYPE_EGO_LEFT_CONFLICT_AREA
    //                 || eInfo.type == ENCOUNTER_TYPE_FOE_LEFT_CONFLICT_AREA) {
    //             // Signalize to discard
    //             return false;
    //         }
    //     }
    //
    //     if (eInfo.type == ENCOUNTER_TYPE_NOCONFLICT_AHEAD) {
    //         // At this state, eInfo.type == ENCOUNTER_TYPE_NOCONFLICT_AHEAD implies that the foe
    //         // is either out of the device's range or its route does not interfere with the ego's route.
    // #ifdef DEBUG_ENCOUNTER
    //         if (DEBUG_COND_ENCOUNTER(e)) {
    //             std::cout << SIMTIME << " Encounter of vehicles '" << e->egoID << "' and '" << e->foeID << "' does not imply any conflict.\n";
    //         }
    // #endif
    //         updatePassedEncounter(e, foeInfo, eInfo);
    // //        return;
    //     } else if (eInfo.type == ENCOUNTER_TYPE_EGO_ENTERED_CONFLICT_AREA
    //                || eInfo.type == ENCOUNTER_TYPE_FOE_ENTERED_CONFLICT_AREA
    //                || eInfo.type == ENCOUNTER_TYPE_EGO_LEFT_CONFLICT_AREA
    //                || eInfo.type == ENCOUNTER_TYPE_FOE_LEFT_CONFLICT_AREA
    //                || eInfo.type == ENCOUNTER_TYPE_BOTH_ENTERED_CONFLICT_AREA
    //                || eInfo.type == ENCOUNTER_TYPE_BOTH_LEFT_CONFLICT_AREA) {
    //         // Ongoing encounter. Treat with update passed encounter (trace covered distances)
    //         // eInfo.type only holds the previous type
    //         updatePassedEncounter(e, foeInfo, eInfo);
    //
    //         // Estimate times until a possible conflict / collision
    //         estimateConflictTimes(eInfo);
    //
    //     } else {
    //         // Estimate times until a possible conflict / collision
    //         // Not all are used for all types of encounters:
    //         // Follow/lead situation doesn't need them at all, currently (might change if more SSMs are implemented).
    //         // Crossing / Merging calculates entry times to determine leader/follower and calculates the exit time for the leader.
    //         estimateConflictTimes(eInfo);
    //
    //         // reset the remaining extra time (foe could have re-entered the device range after beginning extra time countdown already)
    //         e->resetExtraTime(myExtraTime);
    //     }
    //
    //     // update entry/exit times for conflict area
    //     checkConflictEntryAndExit(eInfo);
    //     if (e->size() == 0) {
    // #ifdef DEBUG_ENCOUNTER
    //         if (DEBUG_COND_ENCOUNTER(e)) {
    //             std::cout << SIMTIME << " type when creating encounter: " << eInfo.type << "\n";
    //         }
    // #endif
    //         if (eInfo.type == ENCOUNTER_TYPE_FOE_LEFT_CONFLICT_AREA
    //                 || eInfo.type == ENCOUNTER_TYPE_EGO_LEFT_CONFLICT_AREA
    //                 || eInfo.type == ENCOUNTER_TYPE_BOTH_LEFT_CONFLICT_AREA
    //                 || eInfo.type == ENCOUNTER_TYPE_NOCONFLICT_AHEAD
    //                 || eInfo.type == ENCOUNTER_TYPE_BOTH_ENTERED_CONFLICT_AREA) {
    //             return false;
    //         }
    //     }
    //
    //     // update (x,y)-coords of conflict point
    //     determineConflictPoint(eInfo);
    //
    //     // Compute SSMs
    //     computeSSMs(eInfo);
    //
    //     if (e->currentType == ENCOUNTER_TYPE_BOTH_LEFT_CONFLICT_AREA
    //             && eInfo.type != ENCOUNTER_TYPE_BOTH_LEFT_CONFLICT_AREA) {
    //         // Don't add a point which switches back to a different encounter type from a passed encounter.
    //         // For this situation this encounter will be closed and a new encounter will be created,
    //         // @see correspondingly conditionalized code in processEncounters()
    //         e->currentType = eInfo.type;
    //     } else {
    //         // Add current states to trajectories and update type
    //         e->add(SIMTIME, eInfo.type, e->ego->getPosition(), e->ego->getLane()->getID(), e->ego->getPositionOnLane(), e->ego->getVelocityVector(),
    //                e->foe->getPosition(), e->foe->getLane()->getID(), e->foe->getPositionOnLane(), e->foe->getVelocityVector(),
    //                eInfo.conflictPoint, eInfo.egoConflictEntryDist, eInfo.foeConflictEntryDist, eInfo.ttc, eInfo.drac, eInfo.pet, eInfo.ppet, eInfo.mdrac);
    //     }
    //     // Keep encounter
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn update_encounter(&mut self, e: Ptr<MSDevice_SSM_Encounter>, foeInfo: Ptr<MSDevice_SSM_FoeInfo>) -> bool {
        todo!("port MSDevice_SSM::updateEncounter")
    }

    /// C++: `void MSDevice_SSM::updatePassedEncounter(Encounter *, FoeInfo *, EncounterApproachInfo &)` [private]
    /// Updates an encounter, which was classified as ENCOUNTER_TYPE_NOCONFLICT_AHEAD this may be the case because the foe is out of the detection range but the encounter is still in extra time (in this case foeInfo==0), or because the foe does not head for a lane conflicting with the route of the ego vehicle. It is also used for an ongoing crossing conflict, where only the covered distances are traced until the situation is over. (A crossing conflict is ongoing, if one vehicle entered the conflict area) Writes the type of encounter which is determined for the current state into eInfo. And if appropriate some information concerning vehicles positions in relation to a crossed crossing point (for PET calculation).
    // void
    // MSDevice_SSM::updatePassedEncounter(Encounter* e, FoeInfo* foeInfo, EncounterApproachInfo& eInfo) {
    //
    // #ifdef DEBUG_ENCOUNTER
    //     if (DEBUG_COND_ENCOUNTER(e)) {
    //         std::cout << SIMTIME << " updatePassedEncounter() for vehicles '" << e->egoID << "' and '" << e->foeID << "'\n";
    //     }
    // #endif
    //
    //     if (foeInfo == nullptr) {
    //         // the foe is out of the device's range, proceed counting down the remaining extra time to trace
    //         e->countDownExtraTime(TS);
    // #ifdef DEBUG_ENCOUNTER
    //         if (DEBUG_COND_ENCOUNTER(e)) std::cout << "    Foe is out of range. Counting down extra time."
    //                                                    << " Remaining seconds before closing encounter: " << e->getRemainingExtraTime() << std::endl;
    // #endif
    //
    //     } else {
    //         // reset the remaining extra time (foe could have re-entered the device range after beginning extra time countdown already)
    //         e->resetExtraTime(myExtraTime);
    //     }
    //
    //     // Check, whether this was really a potential conflict at some time:
    //     // Search through typeSpan for a type other than no conflict
    //     EncounterType lastPotentialConflictType = e->typeSpan.size() > 0 ? static_cast<EncounterType>(e->typeSpan.back()) : ENCOUNTER_TYPE_NOCONFLICT_AHEAD;
    //
    //     if (lastPotentialConflictType == ENCOUNTER_TYPE_NOCONFLICT_AHEAD) {
    //         // This encounter was no conflict in the last step -> remains so
    // #ifdef DEBUG_ENCOUNTER
    //         if (DEBUG_COND_ENCOUNTER(e)) {
    //             std::cout << "    This encounter wasn't classified as a potential conflict lately.\n";
    //         }
    // #endif
    //         if (foeInfo == nullptr) {
    //             // Encounter was either never a potential conflict and foe is out of range
    //             // or the foe has left the network
    //             // -> no use in further tracing this encounter
    // #ifdef DEBUG_SSM
    //             if (DEBUG_COND(myHolderMS)) {
    //                 std::cout << "  Requesting encounter closure because foeInfo==nullptr" << std::endl;
    //             }
    // #endif
    //             e->closingRequested = true;
    // #ifdef DEBUG_ENCOUNTER
    //             if (DEBUG_COND_ENCOUNTER(e)) {
    //                 std::cout << "    Closing encounter.\n";
    //             }
    // #endif
    //             eInfo.type = ENCOUNTER_TYPE_NOCONFLICT_AHEAD;
    //         }
    //     } else if (lastPotentialConflictType == ENCOUNTER_TYPE_FOLLOWING_FOLLOWER
    //                || lastPotentialConflictType == ENCOUNTER_TYPE_FOLLOWING_LEADER
    //                || lastPotentialConflictType == ENCOUNTER_TYPE_FOLLOWING_PASSED) {
    //         // if a following situation leads to a no-conflict situation this encounter switches no-conflict, since no further computations (PET) are needed.
    //         eInfo.type = ENCOUNTER_TYPE_FOLLOWING_PASSED;
    // #ifdef DEBUG_ENCOUNTER
    //         if (DEBUG_COND_ENCOUNTER(e)) {
    //             std::cout << "    Encounter was previously classified as a follow/lead situation.\n";
    //         }
    // #endif
    //     } else if (lastPotentialConflictType == ENCOUNTER_TYPE_MERGING_FOLLOWER
    //                || lastPotentialConflictType == ENCOUNTER_TYPE_MERGING_LEADER
    //                || lastPotentialConflictType == ENCOUNTER_TYPE_MERGING_PASSED) {
    //         // if a merging situation leads to a no-conflict situation the leader was either removed from the net (we disregard special treatment)
    //         // or route- or lane-changes removed the conflict.
    //         eInfo.type = ENCOUNTER_TYPE_MERGING_PASSED;
    // #ifdef DEBUG_ENCOUNTER
    //         if (DEBUG_COND_ENCOUNTER(e)) {
    //             std::cout << "    Encounter was previously classified as a merging situation.\n";
    //         }
    // #endif
    //     }
    //     if (lastPotentialConflictType == ENCOUNTER_TYPE_CROSSING_FOLLOWER
    //             || lastPotentialConflictType == ENCOUNTER_TYPE_CROSSING_LEADER
    //             || lastPotentialConflictType == ENCOUNTER_TYPE_EGO_ENTERED_CONFLICT_AREA
    //             || lastPotentialConflictType == ENCOUNTER_TYPE_FOE_ENTERED_CONFLICT_AREA
    //             || lastPotentialConflictType == ENCOUNTER_TYPE_BOTH_ENTERED_CONFLICT_AREA
    //             || lastPotentialConflictType == ENCOUNTER_TYPE_EGO_LEFT_CONFLICT_AREA
    //             || lastPotentialConflictType == ENCOUNTER_TYPE_FOE_LEFT_CONFLICT_AREA
    //             || lastPotentialConflictType == ENCOUNTER_TYPE_BOTH_LEFT_CONFLICT_AREA
    //             || lastPotentialConflictType == ENCOUNTER_TYPE_COLLISION) {
    //         // Encounter has been a crossing situation.
    //
    // #ifdef DEBUG_ENCOUNTER
    //         if (DEBUG_COND_ENCOUNTER(e)) {
    //             std::cout << "    Encounter was previously classified as a crossing situation of type " << lastPotentialConflictType << ".\n";
    //         }
    // #endif
    //         // For passed encounters, the xxxConflictAreaLength variables are not determined before -> we use the stored values.
    //
    //         // TODO: This could also more precisely be calculated wrt the angle of the crossing *at the conflict point*
    //         if (eInfo.egoConflictAreaLength == INVALID_DOUBLE) {
    //             eInfo.egoConflictAreaLength = e->foe->getWidth();
    //         }
    //         if (eInfo.foeConflictAreaLength == INVALID_DOUBLE) {
    //             eInfo.foeConflictAreaLength = e->ego->getWidth();
    //         }
    //
    //         eInfo.egoConflictEntryDist = e->egoDistsToConflict.back() - e->ego->getLastStepDist();
    //         eInfo.egoConflictExitDist = eInfo.egoConflictEntryDist + eInfo.egoConflictAreaLength + e->ego->getLength();
    //         eInfo.foeConflictEntryDist = e->foeDistsToConflict.back() - e->foe->getLastStepDist();
    //         eInfo.foeConflictExitDist = eInfo.foeConflictEntryDist + eInfo.foeConflictAreaLength + e->foe->getLength();
    //
    // #ifdef DEBUG_ENCOUNTER
    //         if (DEBUG_COND_ENCOUNTER(e))
    //             std::cout << "    egoConflictEntryDist = " << eInfo.egoConflictEntryDist
    //                       << ", egoConflictExitDist = " << eInfo.egoConflictExitDist
    //                       << "\n    foeConflictEntryDist = " << eInfo.foeConflictEntryDist
    //                       << ", foeConflictExitDist = " << eInfo.foeConflictExitDist
    //                       << std::endl;
    // #endif
    //
    //         // Determine actual encounter type
    //         bool egoEnteredConflict = eInfo.egoConflictEntryDist < 0.;
    //         bool foeEnteredConflict = eInfo.foeConflictEntryDist < 0.;
    //         bool egoLeftConflict = eInfo.egoConflictExitDist < 0.;
    //         bool foeLeftConflict = eInfo.foeConflictExitDist < 0.;
    //         if ((!egoEnteredConflict) && !foeEnteredConflict) {
    //             // XXX: do we need to recompute the follow/lead order, here?
    //             assert(lastPotentialConflictType == ENCOUNTER_TYPE_CROSSING_FOLLOWER
    //                    || lastPotentialConflictType == ENCOUNTER_TYPE_CROSSING_LEADER);
    //             eInfo.type = lastPotentialConflictType;
    //         } else if (egoEnteredConflict && !foeEnteredConflict) {
    //             eInfo.type = ENCOUNTER_TYPE_EGO_ENTERED_CONFLICT_AREA;
    //         } else if ((!egoEnteredConflict) && foeEnteredConflict) {
    //             eInfo.type = ENCOUNTER_TYPE_FOE_ENTERED_CONFLICT_AREA;
    //         } else { // (egoEnteredConflict && foeEnteredConflict) {
    //             eInfo.type = ENCOUNTER_TYPE_BOTH_ENTERED_CONFLICT_AREA;
    //         }
    //
    //         if ((!egoLeftConflict) && !foeLeftConflict) {
    //             if (eInfo.type == ENCOUNTER_TYPE_BOTH_ENTERED_CONFLICT_AREA) {
    //                 eInfo.type = ENCOUNTER_TYPE_COLLISION;
    //             }
    //         } else if (egoLeftConflict && !foeLeftConflict) {
    //             if (eInfo.type != ENCOUNTER_TYPE_BOTH_ENTERED_CONFLICT_AREA) {
    //                 eInfo.type = ENCOUNTER_TYPE_EGO_LEFT_CONFLICT_AREA;
    //             }
    //         } else if ((!egoLeftConflict) && foeLeftConflict) {
    //             if (eInfo.type != ENCOUNTER_TYPE_BOTH_ENTERED_CONFLICT_AREA) {
    //                 eInfo.type = ENCOUNTER_TYPE_FOE_LEFT_CONFLICT_AREA;
    //             }
    //         } else {
    //             eInfo.type = ENCOUNTER_TYPE_BOTH_LEFT_CONFLICT_AREA;
    //             // It should not occur that both leave the conflict at the same step
    //             assert(lastPotentialConflictType == ENCOUNTER_TYPE_FOE_LEFT_CONFLICT_AREA
    //                    || lastPotentialConflictType == ENCOUNTER_TYPE_EGO_LEFT_CONFLICT_AREA
    //                    || lastPotentialConflictType == ENCOUNTER_TYPE_BOTH_ENTERED_CONFLICT_AREA
    //                    || lastPotentialConflictType == ENCOUNTER_TYPE_BOTH_LEFT_CONFLICT_AREA);
    //         }
    //
    //         // TODO: adjust the conflict distances according to lateral movement for single ENTERED-cases
    //
    // #ifdef DEBUG_ENCOUNTER
    //         if (DEBUG_COND_ENCOUNTER(e)) {
    //             std::cout << "    Updated classification: " << eInfo.type << "\n";
    //         }
    // #endif
    //     }
    // }
    #[allow(unused_variables)]
    pub fn update_passed_encounter(&mut self, e: Ptr<MSDevice_SSM_Encounter>, foeInfo: Ptr<MSDevice_SSM_FoeInfo>, eInfo: &mut MSDevice_SSM_EncounterApproachInfo) {
        todo!("port MSDevice_SSM::updatePassedEncounter")
    }

    /// C++: `EncounterType MSDevice_SSM::classifyEncounter(const FoeInfo *, EncounterApproachInfo &) const` [const, private]
    /// Classifies the current type of the encounter provided some information on the opponents
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn classify_encounter(&self, foeInfo: Ptr<MSDevice_SSM_FoeInfo>, eInfo: &mut MSDevice_SSM_EncounterApproachInfo) -> MSDevice_SSM_EncounterType {
        todo!("port MSDevice_SSM::classifyEncounter")
    }

    /// C++: `void MSDevice_SSM::determineConflictPoint(EncounterApproachInfo &)` [static, private]
    /// Calculates the (x,y)-coordinate for the eventually predicted conflict point and stores the result in eInfo.conflictPoint. In case of MERGING and CROSSING, this is the entry point to conflict area for follower In case of FOLLOWING it is the position of leader's back.
    // void
    // MSDevice_SSM::determineConflictPoint(EncounterApproachInfo& eInfo) {
    //     /*  Calculates the (x,y)-coordinate for the eventually predicted conflict point and stores the result in
    //      *         eInfo.conflictPoint. In case of MERGING and CROSSING, this is the entry point to conflict area for follower
    //      *         In case of FOLLOWING it is the position of leader's back. */
    //
    // #ifdef DEBUG_SSM
    //     if (DEBUG_COND(eInfo.encounter->ego)) {
    //         std::cout << SIMTIME << " determineConflictPoint()" << std::endl;
    //     }
    // #endif
    //
    //     const EncounterType& type = eInfo.type;
    //     const Encounter* e = eInfo.encounter;
    //     if (type == ENCOUNTER_TYPE_BOTH_LEFT_CONFLICT_AREA
    //             || type == ENCOUNTER_TYPE_BOTH_ENTERED_CONFLICT_AREA
    //             || type == ENCOUNTER_TYPE_COLLISION) {
    //         // Both vehicles have already past the conflict entry.
    //         if (e->size() == 0) {
    //             eInfo.conflictPoint = e->ego->getPosition();
    //             WRITE_WARNINGF(TL("SSM device of vehicle '%' encountered an unexpected conflict with foe % at time=%. Please review your vehicle behavior settings."), e->egoID, e->foeID, time2string(SIMSTEP));
    //             return;
    //         }
    //         assert(e->size() > 0); // A new encounter should not be created if both vehicles already entered the conflict area
    //         eInfo.conflictPoint = e->conflictPointSpan.back();
    //     } else if (type == ENCOUNTER_TYPE_CROSSING_FOLLOWER
    //                || type == ENCOUNTER_TYPE_MERGING_FOLLOWER
    //                || type == ENCOUNTER_TYPE_FOE_ENTERED_CONFLICT_AREA
    //                || type == ENCOUNTER_TYPE_FOE_LEFT_CONFLICT_AREA) {
    //         eInfo.conflictPoint = e->ego->getPositionAlongBestLanes(eInfo.egoConflictEntryDist);
    //     } else if (type == ENCOUNTER_TYPE_CROSSING_LEADER
    //                || type == ENCOUNTER_TYPE_MERGING_LEADER
    //                || type == ENCOUNTER_TYPE_EGO_ENTERED_CONFLICT_AREA
    //                || type == ENCOUNTER_TYPE_EGO_LEFT_CONFLICT_AREA) {
    //         eInfo.conflictPoint = e->foe->getPositionAlongBestLanes(eInfo.foeConflictEntryDist);
    //     } else if (type == ENCOUNTER_TYPE_FOLLOWING_FOLLOWER) {
    //         eInfo.conflictPoint = e->foe->getPosition(-e->foe->getLength());
    //     } else if (type == ENCOUNTER_TYPE_FOLLOWING_LEADER) {
    //         eInfo.conflictPoint = e->ego->getPosition(-e->ego->getLength());
    //     } else if (type == ENCOUNTER_TYPE_ONCOMING) {
    //         eInfo.conflictPoint = (e->ego->getPosition() + e->foe->getPosition()) * 0.5;
    //     } else {
    // #ifdef DEBUG_SSM
    //         if (DEBUG_COND(eInfo.encounter->ego)) {
    //             std::cout << "No conflict point associated with encounter type " << type << std::endl;
    //         }
    // #endif
    //         return;
    //     }
    //
    // #ifdef DEBUG_SSM
    //     if (DEBUG_COND(eInfo.encounter->ego)) {
    //         std::cout << "    Conflict at " << eInfo.conflictPoint << std::endl;
    //     }
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn determine_conflict_point(eInfo: &mut MSDevice_SSM_EncounterApproachInfo) {
        todo!("port MSDevice_SSM::determineConflictPoint")
    }

    /// C++: `void MSDevice_SSM::estimateConflictTimes(EncounterApproachInfo &)` [static, private]
    /// Estimates the time until conflict for the vehicles based on the distance to the conflict entry points. For acceleration profiles, we assume that the acceleration is <= 0 (that is, braking is extrapolated, while for acceleration it is assumed that the vehicle will continue with its current speed)
    // void
    // MSDevice_SSM::estimateConflictTimes(EncounterApproachInfo& eInfo) {
    //
    //     EncounterType& type = eInfo.type;
    //     Encounter* e = eInfo.encounter;
    //
    //     assert(type != ENCOUNTER_TYPE_NOCONFLICT_AHEAD); // arrival times not defined, if no conflict is ahead.
    // #ifdef DEBUG_SSM
    //     if (DEBUG_COND(e->ego))
    //         std::cout << SIMTIME << " estimateConflictTimes() for ego '" << e->egoID << "' and foe '" << e->foeID << "'\n"
    //                   << "    encounter type: " << eInfo.type << "\n"
    //                   << "    egoConflictEntryDist=" << writeNA(eInfo.egoConflictEntryDist)
    //                   << " foeConflictEntryDist=" << writeNA(eInfo.foeConflictEntryDist)
    //                   << " egoConflictExitDist=" << writeNA(eInfo.egoConflictExitDist)
    //                   << " foeConflictExitDist=" << writeNA(eInfo.foeConflictExitDist)
    //                   << "\n    ego speed=" << e->ego->getSpeed()
    //                   << ", foe speed=" << e->foe->getSpeed()
    //                   << std::endl;
    // #endif
    //     if (type == ENCOUNTER_TYPE_COLLISION) {
    // #ifdef DEBUG_SSM
    //         eInfo.egoEstimatedConflictEntryTime = 0;
    //         eInfo.foeEstimatedConflictEntryTime = 0;
    //         if (DEBUG_COND(e->ego))
    //             std::cout << "    encouter type " << type << " -> no exit times to be calculated."
    //                       << std::endl;
    // #endif
    //         return;
    //     }
    //
    //     if (type == ENCOUNTER_TYPE_FOLLOWING_FOLLOWER || type == ENCOUNTER_TYPE_FOLLOWING_LEADER || type == ENCOUNTER_TYPE_MERGING_ADJACENT || type == ENCOUNTER_TYPE_ON_ADJACENT_LANES) {
    //         // No need to know the times until ...ConflictDistEntry, currently. They would correspond to an estimated time headway or similar.
    //         // TTC must take into account the movement of the leader, as would DRAC, PET doesn't need the time either, since it uses aposteriori
    //         // values.
    // #ifdef DEBUG_SSM
    //         if (DEBUG_COND(e->ego))
    //             std::cout << "    encouter type " << type << " -> no entry/exit times to be calculated."
    //                       << std::endl;
    // #endif
    //         return;
    //     }
    //
    //     assert(type == ENCOUNTER_TYPE_MERGING || type == ENCOUNTER_TYPE_CROSSING
    //            || type == ENCOUNTER_TYPE_EGO_ENTERED_CONFLICT_AREA
    //            || type == ENCOUNTER_TYPE_FOE_ENTERED_CONFLICT_AREA
    //            || type == ENCOUNTER_TYPE_EGO_LEFT_CONFLICT_AREA
    //            || type == ENCOUNTER_TYPE_FOE_LEFT_CONFLICT_AREA
    //            || type == ENCOUNTER_TYPE_BOTH_ENTERED_CONFLICT_AREA
    //            || type == ENCOUNTER_TYPE_BOTH_LEFT_CONFLICT_AREA
    //            || type == ENCOUNTER_TYPE_ONCOMING);
    //
    //     // Determine exit distances
    //     if (type == ENCOUNTER_TYPE_MERGING || type == ENCOUNTER_TYPE_ONCOMING)  {
    //         eInfo.egoConflictExitDist = eInfo.egoConflictEntryDist + e->ego->getVehicleType().getLength();
    //         eInfo.foeConflictExitDist = eInfo.foeConflictEntryDist + e->foe->getVehicleType().getLength();
    //     } else {
    //         eInfo.egoConflictExitDist = eInfo.egoConflictEntryDist + eInfo.egoConflictAreaLength + e->ego->getVehicleType().getLength();
    //         eInfo.foeConflictExitDist = eInfo.foeConflictEntryDist + eInfo.foeConflictAreaLength + e->foe->getVehicleType().getLength();
    //     }
    //
    //     // Estimate entry times to stipulate a leader / follower relation for the encounter.
    //     if (eInfo.egoConflictEntryDist > NUMERICAL_EPS) {
    //         eInfo.egoEstimatedConflictEntryTime = e->ego->getCarFollowModel().estimateArrivalTime(eInfo.egoConflictEntryDist, e->ego->getSpeed(), e->ego->getMaxSpeedOnLane(), MIN2(0., e->ego->getAcceleration()));
    //         assert(eInfo.egoEstimatedConflictEntryTime > 0.);
    //     } else {
    //         // ego already entered conflict area
    //         eInfo.egoEstimatedConflictEntryTime = 0.;
    //     }
    //     if (eInfo.foeConflictEntryDist > NUMERICAL_EPS) {
    //         eInfo.foeEstimatedConflictEntryTime = e->foe->getCarFollowModel().estimateArrivalTime(eInfo.foeConflictEntryDist, e->foe->getSpeed(), e->foe->getMaxSpeedOnLane(), MIN2(0., e->foe->getAcceleration()));
    //         assert(eInfo.foeEstimatedConflictEntryTime > 0.);
    //     } else {
    //         // foe already entered conflict area
    //         eInfo.foeEstimatedConflictEntryTime = 0.;
    //     }
    //
    //     if (type == ENCOUNTER_TYPE_ONCOMING) {
    //         eInfo.egoEstimatedConflictEntryTime = eInfo.egoConflictEntryDist / (e->ego->getSpeed() + e->foe->getSpeed());
    //         eInfo.foeEstimatedConflictEntryTime = eInfo.egoEstimatedConflictEntryTime;
    //     }
    //
    // #ifdef DEBUG_SSM
    //     if (DEBUG_COND(e->ego))
    //         std::cout << "    Conflict type: " << encounterToString(type) << "\n"
    //                   << "    egoConflictEntryTime=" << (eInfo.egoEstimatedConflictEntryTime == INVALID_DOUBLE ? "NA" : ::toString(eInfo.egoEstimatedConflictEntryTime))
    //                   << ", foeConflictEntryTime=" << (eInfo.foeEstimatedConflictEntryTime == INVALID_DOUBLE ? "NA" : ::toString(eInfo.foeEstimatedConflictEntryTime))
    //                   << std::endl;
    // #endif
    //
    //     // Estimate exit times from conflict area for leader / follower.
    //     // assume vehicles start to accelerate after entring the intersection
    //     if (eInfo.egoConflictExitDist >= 0.) {
    //         eInfo.egoEstimatedConflictExitTime = e->ego->getCarFollowModel().estimateArrivalTime(eInfo.egoConflictExitDist, e->ego->getSpeed(), e->ego->getMaxSpeedOnLane(), MIN2(0., e->ego->getAcceleration()));
    //         //eInfo.egoEstimatedConflictExitTime = eInfo.egoEstimatedConflictEntryTime + e->ego->getCarFollowModel().estimateArrivalTime(
    //         //        eInfo.egoConflictExitDist - MAX2(0.0, eInfo.egoConflictEntryDist),
    //         //        e->ego->getSpeed(), e->ego->getMaxSpeedOnLane(),
    //         //        e->ego->getCarFollowModel().getMaxAccel());
    //     } else {
    //         eInfo.egoEstimatedConflictExitTime = 0.;
    //     }
    //     if (eInfo.foeConflictExitDist >= 0.) {
    //         eInfo.foeEstimatedConflictExitTime = e->foe->getCarFollowModel().estimateArrivalTime(eInfo.foeConflictExitDist, e->foe->getSpeed(), e->foe->getMaxSpeedOnLane(), MIN2(0., e->foe->getAcceleration()));
    //         //eInfo.foeEstimatedConflictExitTime = eInfo.foeEstimatedConflictEntryTime + e->foe->getCarFollowModel().estimateArrivalTime(
    //         //        eInfo.foeConflictExitDist - MAX2(0.0, eInfo.foeConflictEntryDist),
    //         //        e->foe->getSpeed(), e->foe->getMaxSpeedOnLane(),
    //         //        e->foe->getCarFollowModel().getMaxAccel());
    //     } else {
    //         eInfo.foeEstimatedConflictExitTime = 0.;
    //     }
    //
    //     if (type == ENCOUNTER_TYPE_ONCOMING) {
    //         eInfo.egoEstimatedConflictExitTime = eInfo.egoEstimatedConflictEntryTime;
    //         eInfo.foeEstimatedConflictExitTime = eInfo.egoEstimatedConflictEntryTime;
    //     }
    //
    //     if (type != ENCOUNTER_TYPE_MERGING && type != ENCOUNTER_TYPE_CROSSING) {
    //         // this call is issued in context of an ongoing conflict, therefore complete type is already known for the encounter
    //         // (One of EGO_ENTERED_CONFLICT_AREA, FOE_ENTERED_CONFLICT_AREA, EGO_LEFT_CONFLICT_AREA, FOE_LEFT_CONFLICT_AREA, BOTH_ENTERED_CONFLICT_AREA)
    //         // --> no need to specify incomplete encounter type
    //         return;
    //     }
    //
    //     // For merging and crossing situation, the leader/follower relation not determined by classifyEncounter()
    //     // This is done below based on the estimated conflict entry times
    //     if (eInfo.egoEstimatedConflictEntryTime == 0. && eInfo.foeEstimatedConflictEntryTime == 0. &&
    //             eInfo.egoConflictExitDist >= 0 && eInfo.foeConflictExitDist >= 0) {
    //         type = ENCOUNTER_TYPE_COLLISION;
    //         WRITE_WARNINGF(TL("SSM device of vehicle '%' detected collision with vehicle '%' at time=%."), e->egoID, e->foeID, time2string(SIMSTEP));
    //     } else if (eInfo.egoEstimatedConflictEntryTime == INVALID_DOUBLE && eInfo.foeEstimatedConflictEntryTime == INVALID_DOUBLE) {
    //         type = ENCOUNTER_TYPE_NOCONFLICT_AHEAD;
    //     } else if (eInfo.egoEstimatedConflictEntryTime < eInfo.foeEstimatedConflictEntryTime || eInfo.foeEstimatedConflictEntryTime == INVALID_DOUBLE) {
    //         // ego is estimated first at conflict point
    // #ifdef DEBUG_SSM
    //         if (DEBUG_COND(e->ego))
    //             std::cout << "    -> ego is estimated leader at conflict entry."
    //                       << " egoConflictExitTime=" << (eInfo.egoEstimatedConflictExitTime == INVALID_DOUBLE ? "NA" : ::toString(eInfo.egoEstimatedConflictExitTime))
    //                       << " egoEstimatedConflictEntryTime=" << (eInfo.egoEstimatedConflictEntryTime == INVALID_DOUBLE ? "NA" : ::toString(eInfo.egoEstimatedConflictEntryTime))
    //                       << " foeEstimatedConflictEntryTime=" << (eInfo.foeEstimatedConflictEntryTime == INVALID_DOUBLE ? "NA" : ::toString(eInfo.foeEstimatedConflictEntryTime))
    //                       << std::endl;
    // #endif
    //         type = type == ENCOUNTER_TYPE_CROSSING ? ENCOUNTER_TYPE_CROSSING_LEADER : ENCOUNTER_TYPE_MERGING_LEADER;
    //     } else {
    //         // ego is estimated second at conflict point
    // #ifdef DEBUG_SSM
    //         if (DEBUG_COND(e->ego))
    //             std::cout << "    -> foe is estimated leader at conflict entry."
    //                       << " foeConflictExitTime=" << (eInfo.foeEstimatedConflictExitTime == INVALID_DOUBLE ? "NA" : ::toString(eInfo.foeEstimatedConflictExitTime))
    //                       << " egoEstimatedConflictEntryTime=" << (eInfo.egoEstimatedConflictEntryTime == INVALID_DOUBLE ? "NA" : ::toString(eInfo.egoEstimatedConflictEntryTime))
    //                       << " foeEstimatedConflictEntryTime=" << (eInfo.foeEstimatedConflictEntryTime == INVALID_DOUBLE ? "NA" : ::toString(eInfo.foeEstimatedConflictEntryTime))
    //                       << std::endl;
    // #endif
    //         type = type == ENCOUNTER_TYPE_CROSSING ? ENCOUNTER_TYPE_CROSSING_FOLLOWER : ENCOUNTER_TYPE_MERGING_FOLLOWER;
    //     }
    //
    // }
    #[allow(unused_variables)]
    pub fn estimate_conflict_times(eInfo: &mut MSDevice_SSM_EncounterApproachInfo) {
        todo!("port MSDevice_SSM::estimateConflictTimes")
    }

    /// C++: `void MSDevice_SSM::checkConflictEntryAndExit(EncounterApproachInfo &)` [static, private]
    /// Checks whether ego or foe have entered or left the conflict area in the last step and eventually writes the corresponding entry or exit times to eInfo.encounter. For ongoing crossing conflicts, it also manages the evolution of the conflict type.
    // void
    // MSDevice_SSM::checkConflictEntryAndExit(EncounterApproachInfo& eInfo) {
    //     // determine exact entry and exit times
    //     Encounter* e = eInfo.encounter;
    //
    //     const bool foePastConflictEntry = eInfo.foeConflictEntryDist < 0.0 && eInfo.foeConflictEntryDist != INVALID_DOUBLE;
    //     const bool egoPastConflictEntry = eInfo.egoConflictEntryDist < 0.0 && eInfo.egoConflictEntryDist != INVALID_DOUBLE;
    //     const bool foePastConflictExit = eInfo.foeConflictExitDist < 0.0 && eInfo.foeConflictExitDist != INVALID_DOUBLE;
    //     const bool egoPastConflictExit = eInfo.egoConflictExitDist < 0.0 && eInfo.egoConflictExitDist != INVALID_DOUBLE;
    //
    // #ifdef DEBUG_ENCOUNTER
    //     if (DEBUG_COND_ENCOUNTER(e)) {
    //         std::cout << SIMTIME << " checkConflictEntryAndExit() for encounter of vehicles '" << e->egoID << "' and '" << e->foeID << "'"
    //                   << "  foeEntryDist=" << eInfo.foeConflictEntryDist
    //                   << "  egoEntryDist=" << eInfo.egoConflictEntryDist
    //                   << "  foeExitDist=" << eInfo.foeConflictExitDist
    //                   << "  egoExitDist=" << eInfo.egoConflictExitDist
    //                   << "\n";
    //     }
    // #endif
    //
    //
    //     if (e->size() == 0) {
    //         // This is a new conflict (or a conflict that was considered earlier
    //         // but disregarded due to being 'over')
    //
    //         if (egoPastConflictExit) {
    //             if (foePastConflictExit) {
    //                 eInfo.type = ENCOUNTER_TYPE_BOTH_LEFT_CONFLICT_AREA;
    //             } else if (foePastConflictEntry) {
    //                 eInfo.type = ENCOUNTER_TYPE_BOTH_ENTERED_CONFLICT_AREA;
    //             } else {
    //                 eInfo.type = ENCOUNTER_TYPE_EGO_LEFT_CONFLICT_AREA;
    //             }
    //         } else if (foePastConflictExit) {
    //             if (egoPastConflictEntry) {
    //                 eInfo.type = ENCOUNTER_TYPE_BOTH_ENTERED_CONFLICT_AREA;
    //             } else {
    //                 eInfo.type = ENCOUNTER_TYPE_FOE_LEFT_CONFLICT_AREA;
    //             }
    //         } else {
    //             // No one left conflict area
    //             if (egoPastConflictEntry) {
    //                 if (foePastConflictEntry) {
    //                     eInfo.type = ENCOUNTER_TYPE_COLLISION;
    //                 } else {
    //                     eInfo.type = ENCOUNTER_TYPE_EGO_ENTERED_CONFLICT_AREA;
    //                 }
    //             } else if (foePastConflictEntry) {
    //                 eInfo.type = ENCOUNTER_TYPE_FOE_ENTERED_CONFLICT_AREA;
    //             }
    //             // else: both before conflict, keep current type
    //         }
    //         return;
    //     }
    //
    //     // Distances to conflict area boundaries in previous step
    //     double prevEgoConflictEntryDist = eInfo.egoConflictEntryDist + e->ego->getLastStepDist();
    //     double prevFoeConflictEntryDist = eInfo.foeConflictEntryDist + e->foe->getLastStepDist();
    //     double prevEgoConflictExitDist = prevEgoConflictEntryDist + eInfo.egoConflictAreaLength + e->ego->getLength();
    //     double prevFoeConflictExitDist = prevFoeConflictEntryDist + eInfo.foeConflictAreaLength + e->foe->getLength();
    //     EncounterType prevType = e->currentType;
    //
    // //#ifdef DEBUG_ENCOUNTER
    // //    if (DEBUG_COND_ENCOUNTER(eInfo.encounter))
    // //        std::cout << "\nEgo's prev distance to conflict entry: " << prevEgoConflictEntryDist
    // //                  << "\nEgo's prev distance to conflict exit:  " << prevEgoConflictExitDist
    // //                  << "\nFoe's prev distance to conflict entry: " << prevFoeConflictEntryDist
    // //                  << "\nFoe's prev distance to conflict exit:  " << prevFoeConflictExitDist
    // //                  << std::endl;
    // //#endif
    //
    //     // Check if ego entered in last step
    //     if (e->egoConflictEntryTime == INVALID_DOUBLE && egoPastConflictEntry && prevEgoConflictEntryDist >= 0) {
    //         // ego must have entered the conflict in the last step. Determine exact entry time
    //         e->egoConflictEntryTime = SIMTIME - TS + MSCFModel::passingTime(-prevEgoConflictEntryDist, 0., -eInfo.egoConflictEntryDist, e->ego->getPreviousSpeed(), e->ego->getSpeed());
    // #ifdef DEBUG_ENCOUNTER
    //         if (DEBUG_COND_ENCOUNTER(e)) {
    //             std::cout << "    ego entered conflict area at t=" << e->egoConflictEntryTime << std::endl;
    //         }
    // #endif
    //         // Update encounter type (only done here for entering, the other transitions are done in updatePassedEncounter)
    //         if (prevType == ENCOUNTER_TYPE_CROSSING_FOLLOWER
    //                 || prevType == ENCOUNTER_TYPE_CROSSING_LEADER) {
    //             eInfo.type = ENCOUNTER_TYPE_EGO_ENTERED_CONFLICT_AREA;
    //         }
    //     }
    //
    //     // Check if foe entered in last step
    //     if (e->foeConflictEntryTime == INVALID_DOUBLE && foePastConflictEntry && prevFoeConflictEntryDist >= 0) {
    //         // foe must have entered the conflict in the last step. Determine exact entry time
    //         e->foeConflictEntryTime = SIMTIME - TS + MSCFModel::passingTime(-prevFoeConflictEntryDist, 0., -eInfo.foeConflictEntryDist, e->foe->getPreviousSpeed(), e->foe->getSpeed());
    // #ifdef DEBUG_ENCOUNTER
    //         if (DEBUG_COND_ENCOUNTER(e)) {
    //             std::cout << "    foe entered conflict area at t=" << e->foeConflictEntryTime << std::endl;
    //         }
    // #endif
    //         // Update encounter type (only done here for entering, the other transitions are done in updatePassedEncounter)
    //         if (prevType == ENCOUNTER_TYPE_CROSSING_FOLLOWER
    //                 || prevType == ENCOUNTER_TYPE_CROSSING_LEADER) {
    //             eInfo.type = ENCOUNTER_TYPE_FOE_ENTERED_CONFLICT_AREA;
    //         }
    //     }
    //
    //     // Check if ego left conflict area
    //     if (e->egoConflictExitTime == INVALID_DOUBLE && eInfo.egoConflictExitDist < 0 && prevEgoConflictExitDist >= 0) {
    //         // ego must have left the conflict area in the last step. Determine exact exit time
    //         e->egoConflictExitTime = SIMTIME - TS + MSCFModel::passingTime(-prevEgoConflictExitDist, 0., -eInfo.egoConflictExitDist, e->ego->getPreviousSpeed(), e->ego->getSpeed());
    //         // Add cross section to calculate PET for foe
    // //        e->foePETCrossSections.push_back(std::make_pair(eInfo.foeConflictEntryCrossSection, e->egoConflictExitTime));
    // #ifdef DEBUG_ENCOUNTER
    //         if (DEBUG_COND_ENCOUNTER(e)) {
    //             std::cout << "    ego left conflict area at t=" << e->egoConflictExitTime << std::endl;
    //         }
    // #endif
    //         // Update encounter type (only done here for entering, the other transitions are done in updatePassedEncounter)
    //         if (prevType == ENCOUNTER_TYPE_CROSSING_FOLLOWER
    //                 || prevType == ENCOUNTER_TYPE_CROSSING_LEADER) {
    //             eInfo.type = ENCOUNTER_TYPE_EGO_LEFT_CONFLICT_AREA;
    //         }
    //     }
    //
    //     // Check if foe left conflict area
    //     if (e->foeConflictExitTime == INVALID_DOUBLE && eInfo.foeConflictExitDist < 0 && prevFoeConflictExitDist >= 0) {
    //         // foe must have left the conflict area in the last step. Determine exact exit time
    //         e->foeConflictExitTime = SIMTIME - TS + MSCFModel::passingTime(-prevFoeConflictExitDist, 0., -eInfo.foeConflictExitDist, e->foe->getPreviousSpeed(), e->foe->getSpeed());
    //         // Add cross section to calculate PET for ego
    // //        e->egoPETCrossSections.push_back(std::make_pair(eInfo.egoConflictEntryCrossSection, e->foeConflictExitTime));
    // #ifdef DEBUG_ENCOUNTER
    //         if (DEBUG_COND_ENCOUNTER(e)) {
    //             std::cout << "    foe left conflict area at t=" << e->foeConflictExitTime << std::endl;
    //         }
    // #endif
    //         // Update encounter type (only done here for entering, the other transitions are done in updatePassedEncounter)
    //         if (prevType == ENCOUNTER_TYPE_CROSSING_FOLLOWER
    //                 || prevType == ENCOUNTER_TYPE_CROSSING_LEADER) {
    //             eInfo.type = ENCOUNTER_TYPE_FOE_LEFT_CONFLICT_AREA;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn check_conflict_entry_and_exit(eInfo: &mut MSDevice_SSM_EncounterApproachInfo) {
        todo!("port MSDevice_SSM::checkConflictEntryAndExit")
    }

    /// C++: `const MSLane * MSDevice_SSM::findFoeConflictLane(const MSVehicle *, const MSLane *, double &) const` [const, private]
    /// Computes the conflict lane for the foe
    // const MSLane*
    // MSDevice_SSM::findFoeConflictLane(const MSVehicle* foe, const MSLane* egoConflictLane, double& distToConflictLane) const {
    //
    // #ifdef DEBUG_SSM
    //     if (DEBUG_COND(myHolderMS))
    //         std::cout << SIMTIME << " findFoeConflictLane() for foe '"
    //                   << foe->getID() << "' on lane '" << foe->getLane()->getID()
    //                   << "' (with egoConflictLane=" << (egoConflictLane == 0 ? "NULL" : egoConflictLane->getID())
    //                   << ")\nfoeBestLanes: " << ::toString(foe->getBestLanesContinuation())
    //                   << std::endl;
    // #endif
    //     if (foe->getLaneChangeModel().isOpposite()) {
    //         // distinguish three cases
    //         // 1) foe is driving in the same direction as ego and ego is driving in lane direction -> ENCOUNTER_TYPE_ON_ADJACENT_LANES
    //         // 2) foe is driving in the same direction as ego and ego is also driving in the opposite direction -> ENCOUNTER_TYPE_FOLLOWING
    //         // 3) foe is driving in the opposite direction as ego and both are driving way from each other -> ENCOUNTER_TYPE_NOCONFLICT_AHEAD
    //         // 3) foe is driving in the opposite direction as ego and both are driving towards each other -> ENCOUNTER_TYPE_ONCOMING
    // #ifdef DEBUG_SSM_OPPOSITE
    // #endif
    //         auto egoIt = std::find(myHolder.getCurrentRouteEdge(), myHolder.getRoute().end(), foe->getEdge());
    //         if (egoIt != myHolder.getRoute().end()) {
    //             // same direction, foe is leader
    //             if (myHolderMS->getLaneChangeModel().isOpposite()) {
    //                 if (egoConflictLane->isInternal() && !foe->getLane()->isInternal()) {
    //                     // lead/follow situation resolved elsewhere
    //                     return nullptr;
    //                 }
    //                 return foe->getLane();
    //             } else {
    //                 // adjacent
    //                 return nullptr;
    //             }
    //         }
    //         auto foeIt = std::find(foe->getCurrentRouteEdge(), foe->getRoute().end(), myHolder.getEdge());
    //         if (foeIt != foe->getRoute().end()) {
    //             // same direction, ego is leader
    //             if (myHolderMS->getLaneChangeModel().isOpposite()) {
    //                 return egoConflictLane;
    //             } else {
    //                 // adjacent
    //                 return nullptr;
    //             }
    //         }
    //         auto egoIt2 = std::find(myHolder.getCurrentRouteEdge(), myHolder.getRoute().end(), foe->getEdge()->getOppositeEdge());
    //         if (egoIt2 != myHolder.getRoute().end()) {
    //             // opposite direction, driving towards each other
    //             return egoConflictLane;
    //         } else {
    //             // opposite direction, driving away from each other
    //             return nullptr;
    //         }
    //     }
    //
    //     const MSLane* foeLane = foe->getLane();
    //     std::vector<MSLane*>::const_iterator laneIter = foe->getBestLanesContinuation().begin();
    //     std::vector<MSLane*>::const_iterator foeBestLanesEnd = foe->getBestLanesContinuation().end();
    //     assert(foeLane->isInternal() || *laneIter == foeLane);
    //     distToConflictLane = -foe->getPositionOnLane();
    //
    //     // Potential conflict lies on junction if egoConflictLane is internal
    //     const MSJunction* conflictJunction = egoConflictLane->isInternal() ? egoConflictLane->getEdge().getToJunction() : nullptr;
    // #ifdef DEBUG_SSM
    //     if (DEBUG_COND(myHolderMS))
    //         if (conflictJunction != 0) {
    //             std::cout << "Potential conflict on junction '" << conflictJunction->getID()
    //                       << std::endl;
    //         }
    // #endif
    //     if (foeLane->isInternal() && foeLane->getEdge().getToJunction() == conflictJunction) {
    //         // foe is already on the conflict junction
    //         if (egoConflictLane != nullptr && egoConflictLane->isInternal() && egoConflictLane->getLinkCont()[0]->getViaLane() == foeLane) {
    //             distToConflictLane += egoConflictLane->getLength();
    //         }
    //         return foeLane;
    //     }
    //
    //     // Foe is not on the conflict junction
    //
    //     // Leading internal lanes in bestlanes are resembled as a single NULL-pointer skip them
    //     if (*laneIter == nullptr) {
    //         while (foeLane != nullptr && foeLane->isInternal()) {
    //             distToConflictLane += foeLane->getLength();
    //             foeLane = foeLane->getLinkCont()[0]->getViaLane();
    //         }
    //         ++laneIter;
    //         assert(laneIter == foeBestLanesEnd || *laneIter != 0);
    //     }
    //
    //     // Look for the junction downstream along foeBestLanes
    //     while (laneIter != foeBestLanesEnd && distToConflictLane <= myRange) {
    //         // Eventual internal lanes were skipped
    //         assert(*laneIter == foeLane || foeLane == 0);
    //         foeLane = *laneIter;
    //         assert(!foeLane->isInternal());
    //         if (&foeLane->getEdge() == &egoConflictLane->getEdge()) {
    // #ifdef DEBUG_SSM
    //             if (DEBUG_COND(myHolderMS)) {
    //                 std::cout << "Found conflict lane for foe: '" << foeLane->getID() << "'" << std::endl;
    //             }
    // #endif
    //             // found the potential conflict edge along foeBestLanes
    //             return foeLane;
    //         }
    //         // No conflict on foeLane
    //         distToConflictLane += foeLane->getLength();
    //
    //         // set laneIter to next non internal lane along foeBestLanes
    //         ++laneIter;
    //         if (laneIter == foeBestLanesEnd) {
    //             return nullptr;
    //         }
    //         MSLane* const nextNonInternalLane = *laneIter;
    //         const MSLink* const link = foeLane->getLinkTo(nextNonInternalLane);
    //         if (link == nullptr) {
    //             // encountered incomplete route
    //             return nullptr;
    //         }
    //         // Set foeLane to first internal lane on the next junction
    //         foeLane = link->getViaLane();
    //         assert(foeLane == 0 || foeLane->isInternal());
    //         if (foeLane == nullptr) {
    //             foeLane = nextNonInternalLane;
    //             continue;
    //         }
    //         if (foeLane->getEdge().getToJunction() == conflictJunction) {
    //             assert(foeLane != 0);
    // #ifdef DEBUG_SSM
    //             if (DEBUG_COND(myHolderMS)) {
    //                 std::cout << "Found conflict lane for foe: '" << foeLane->getID() << "'" << std::endl;
    //             }
    // #endif
    //             // found egoConflictLane, resp. the conflict junction, along foeBestLanes
    //             return foeLane;
    //         }
    //         // No conflict on junction
    //         distToConflictLane += link->getInternalLengthsAfter();
    //         foeLane = nextNonInternalLane;
    //     }
    //     // Didn't find conflicting lane on foeBestLanes within range.
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn find_foe_conflict_lane(&self, foe: Ptr<MSVehicle>, egoConflictLane: Ptr<MSLane>, distToConflictLane: &mut f64) -> Ptr<MSLane> {
        todo!("port MSDevice_SSM::findFoeConflictLane")
    }

    /// C++: `void MSDevice_SSM::closeEncounter(Encounter *)` [private]
    /// Finalizes the encounter and calculates SSM values.
    // void
    // MSDevice_SSM::closeEncounter(Encounter* e) {
    //     assert(e->size() > 0);
    //     // erase pointers (encounter is stored before being destroyed and pointers could become invalid)
    //     e->ego = nullptr;
    //     e->foe = nullptr;
    //     e->end = e->timeSpan.back();
    //     bool wasConflict = qualifiesAsConflict(e);
    // #ifdef DEBUG_SSM
    //     if (DEBUG_COND(myHolderMS)) {
    //         std::cout << SIMTIME << " closeEncounter() of vehicles '"
    //                   << e->egoID << "' and '" << e->foeID
    //                   << "' (was ranked as " << (wasConflict ? "conflict" : "non-conflict") << ")" << std::endl;
    //     }
    // #endif
    //     if (wasConflict) {
    //         myPastConflicts.push(e);
    // #ifdef DEBUG_SSM
    //         if (!myPastConflicts.empty()) {
    //             if (DEBUG_COND(myHolderMS)) {
    //                 std::cout << "pastConflictsQueue of veh '" << myHolderMS->getID() << "':\n";
    //             }
    //             auto myPastConflicts_bak = myPastConflicts;
    //             double lastBegin = myPastConflicts.top()->begin;
    //             while (!myPastConflicts.empty()) {
    //                 auto c = myPastConflicts.top();
    //                 myPastConflicts.pop();
    //                 if (DEBUG_COND(myHolderMS)) {
    //                     std::cout << "  Conflict with foe '" << c->foe << "' (time=" << c->begin << "-" << c->end << ")\n";
    //                 }
    //                 if (c->begin < lastBegin) {
    //                     std::cout << "  Queue corrupt...\n";
    //                     assert(false);
    //                 }
    //                 lastBegin = c->begin;
    //             }
    //             std::cout << std::endl;
    //             myPastConflicts = myPastConflicts_bak;
    //         }
    // #endif
    //     } else {
    //         delete e;
    //     }
    //     return;
    // }
    #[allow(unused_variables)]
    pub fn close_encounter(&mut self, e: Ptr<MSDevice_SSM_Encounter>) {
        todo!("port MSDevice_SSM::closeEncounter")
    }

    /// C++: `bool MSDevice_SSM::qualifiesAsConflict(Encounter *)` [private]
    /// Tests if the SSM values exceed the threshold for qualification as conflict.
    // bool
    // MSDevice_SSM::qualifiesAsConflict(Encounter* e) {
    //     // Check if conflict measure thresholds are exceeded (to decide whether to keep the encounter for writing out)
    // #ifdef DEBUG_SSM
    //     if (DEBUG_COND(myHolderMS))
    //         std::cout << SIMTIME << " qualifiesAsConflict() for encounter of vehicles '"
    //                   << e->egoID << "' and '" << e->foeID
    //                   << "'" << std::endl;
    // #endif
    //
    //     if (myComputePET && e->PET.value != INVALID_DOUBLE && e->PET.value <= myThresholds["PET"]) {
    //         return true;
    //     }
    //     if (myComputeTTC && e->minTTC.value != INVALID_DOUBLE && e->minTTC.value <= myThresholds["TTC"]) {
    //         return true;
    //     }
    //     if (myComputeDRAC && e->maxDRAC.value != INVALID_DOUBLE && e->maxDRAC.value >= myThresholds["DRAC"]) {
    //         return true;
    //     }
    //     if (myComputePPET && e->minPPET.value != INVALID_DOUBLE && e->minPPET.value <= myThresholds["PPET"]) {
    //         return true;
    //     }
    //     if (myComputeMDRAC && e->maxMDRAC.value != INVALID_DOUBLE && e->maxMDRAC.value >= myThresholds["MDRAC"]) {
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn qualifies_as_conflict(&mut self, e: Ptr<MSDevice_SSM_Encounter>) -> bool {
        todo!("port MSDevice_SSM::qualifiesAsConflict")
    }

    /// C++: `void MSDevice_SSM::computeSSMs(EncounterApproachInfo &) const` [const, private]
    /// Compute current values of the logged SSMs (myMeasures) for the given encounter 'e' and update 'e' accordingly (add point to SSM time-series, update maximal/minimal value) This is called just after adding the current vehicle positions and velocity vectors to the encounter.
    // void
    // MSDevice_SSM::computeSSMs(EncounterApproachInfo& eInfo) const {
    // #ifdef DEBUG_SSM
    //     if (DEBUG_COND(myHolderMS)) {
    //         Encounter* e = eInfo.encounter;
    //         std::cout << SIMTIME << " computeSSMs() for vehicles '"
    //                   << e->ego->getID() << "' and '" << e->foe->getID()
    //                   << "'" << std::endl;
    //     }
    // #endif
    //
    //     const EncounterType& type = eInfo.type;
    //
    //     if (type == ENCOUNTER_TYPE_CROSSING_FOLLOWER || type == ENCOUNTER_TYPE_CROSSING_LEADER
    //             || type == ENCOUNTER_TYPE_EGO_ENTERED_CONFLICT_AREA || type == ENCOUNTER_TYPE_FOE_ENTERED_CONFLICT_AREA
    //             || type == ENCOUNTER_TYPE_MERGING_FOLLOWER || type == ENCOUNTER_TYPE_MERGING_LEADER
    //             || type == ENCOUNTER_TYPE_FOLLOWING_FOLLOWER || type == ENCOUNTER_TYPE_FOLLOWING_LEADER
    //             || type == ENCOUNTER_TYPE_ONCOMING) {
    //         if (myComputeTTC || myComputeDRAC || myComputePPET || myComputeMDRAC) {
    //             determineTTCandDRACandPPETandMDRAC(eInfo);
    //         }
    //         determinePET(eInfo);
    //     } else if (type == ENCOUNTER_TYPE_BOTH_LEFT_CONFLICT_AREA) {
    //         determinePET(eInfo);
    //     } else if (type == ENCOUNTER_TYPE_COLLISION) {
    //         // TODO: handle collision
    //     } else if (type == ENCOUNTER_TYPE_EGO_LEFT_CONFLICT_AREA || type == ENCOUNTER_TYPE_FOE_LEFT_CONFLICT_AREA
    //                || type == ENCOUNTER_TYPE_BOTH_ENTERED_CONFLICT_AREA || type == ENCOUNTER_TYPE_NOCONFLICT_AHEAD) {
    //         // No conflict measures apply for these states, which correspond to intermediate times between
    //         // one vehicle leaving the conflict area and the arrival time for the other (difference corresponds to the PET)
    //     } else if (type == ENCOUNTER_TYPE_ON_ADJACENT_LANES || type == ENCOUNTER_TYPE_MERGING_ADJACENT) {
    //         // No conflict measures apply for this state
    //     } else if (type == ENCOUNTER_TYPE_MERGING_PASSED || type == ENCOUNTER_TYPE_FOLLOWING_PASSED) {
    //         // No conflict measures apply for this state
    //     } else if (type == ENCOUNTER_TYPE_NOCONFLICT_AHEAD) {
    //         // No conflict measures apply for this state
    //     } else {
    //         std::stringstream ss;
    //         ss << "'" << type << "'";
    //         WRITE_WARNING("Unknown or undetermined encounter type at computeSSMs(): " + ss.str());
    //     }
    //
    // #ifdef DEBUG_SSM
    //     if (DEBUG_COND(myHolderMS)) {
    //         Encounter* e = eInfo.encounter;
    //         std::cout << "computeSSMs() for encounter of vehicles '" << e->egoID << "' and '" << e->foeID << "':\n"
    //                   << "  ttc=" << (eInfo.ttc == INVALID_DOUBLE ? "NA" : ::toString(eInfo.ttc))
    //                   << ", drac=" << (eInfo.drac == INVALID_DOUBLE ? "NA" : ::toString(eInfo.drac))
    //                   << ", pet=" << (eInfo.pet.second == INVALID_DOUBLE ? "NA" : ::toString(eInfo.pet.second))
    //                   << std::endl;
    //     }
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn compute_ss_ms(&self, e: &mut MSDevice_SSM_EncounterApproachInfo) {
        todo!("port MSDevice_SSM::computeSSMs")
    }

    /// C++: `void MSDevice_SSM::determinePET(EncounterApproachInfo &) const` [const, private]
    /// Discriminates between different encounter types and correspondingly determines the PET for those cases and writes the result to eInfo.pet (after both vehicles have left the conflict area)
    // void
    // MSDevice_SSM::determinePET(EncounterApproachInfo& eInfo) const {
    //     Encounter* e = eInfo.encounter;
    //     if (e->size() == 0) {
    //         return;
    //     }
    //     const EncounterType& type = eInfo.type;
    //     std::pair<double, double>& pet = eInfo.pet;
    //
    // #ifdef DEBUG_SSM
    //     if (DEBUG_COND(myHolderMS))
    //         std::cout << SIMTIME << " determinePET() for encounter of vehicles '" << e->egoID << "' and '" << e->foeID << "'"
    //                   << "(type: " << encounterToString(static_cast<EncounterType>(e->typeSpan.back())) << ")" << std::endl;
    // #endif
    //
    //     if (type == ENCOUNTER_TYPE_FOLLOWING_FOLLOWER || type == ENCOUNTER_TYPE_FOLLOWING_LEADER) {
    //         // For a following situation, the corresponding PET-value is merely the time-headway.
    //         //       Determining these could be done by comparison of memorized gaps with memorized covered distances
    //         //       Implementation is postponed. Tracing the time gaps (in contrast to crossing PET) corresponds to
    //         //       a vector of values not a single value.
    //         // pass
    //     } else if (type == ENCOUNTER_TYPE_BOTH_LEFT_CONFLICT_AREA) {
    //         EncounterType prevType = static_cast<EncounterType>(e->typeSpan.back());
    //         if (prevType == ENCOUNTER_TYPE_BOTH_LEFT_CONFLICT_AREA) {
    // #ifdef DEBUG_SSM
    //             if (DEBUG_COND(myHolderMS))
    //                 std::cout << "PET for crossing encounter already calculated as " << e->PET.value
    //                           << std::endl;
    // #endif
    //             // pet must have been calculated already
    //             assert(e->PET.value != INVALID_DOUBLE);
    //             return;
    //         }
    //
    //         // this situation should have emerged from one of the following
    //         assert(prevType == ENCOUNTER_TYPE_CROSSING_FOLLOWER
    //                || prevType == ENCOUNTER_TYPE_CROSSING_LEADER
    //                || prevType == ENCOUNTER_TYPE_EGO_LEFT_CONFLICT_AREA
    //                || prevType == ENCOUNTER_TYPE_FOE_LEFT_CONFLICT_AREA
    //                || prevType == ENCOUNTER_TYPE_EGO_ENTERED_CONFLICT_AREA
    //                || prevType == ENCOUNTER_TYPE_FOE_ENTERED_CONFLICT_AREA
    //                || prevType == ENCOUNTER_TYPE_BOTH_ENTERED_CONFLICT_AREA);
    //
    //
    // #ifdef DEBUG_SSM
    //         if (DEBUG_COND(myHolderMS))
    //             std::cout << "e->egoDistsToConflict.back() = " << e->egoDistsToConflict.back()
    //                       << "\ne->egoConflictEntryTime = " << e->egoConflictEntryTime
    //                       << "\ne->egoConflictExitTime = " << e->egoConflictExitTime
    //                       << "\ne->foeDistsToConflict.back() = " << e->foeDistsToConflict.back()
    //                       << "\ne->foeConflictEntryTime = " << e->foeConflictEntryTime
    //                       << "\ne->foeConflictExitTime = " << e->foeConflictExitTime
    //                       << std::endl;
    // #endif
    //
    //         // But both have passed the conflict area
    //         assert(e->egoConflictEntryTime != INVALID_DOUBLE || e->foeConflictEntryTime != INVALID_DOUBLE);
    //
    //         // Both have left the conflict region
    //         // (Conflict may have started as one was already within the conflict area - thus the check for invalid entry times)
    //         if (e->foeConflictEntryTime == INVALID_DOUBLE || (e->egoConflictEntryTime != INVALID_DOUBLE && e->egoConflictEntryTime > e->foeConflictExitTime)) {
    //             pet.first = e->egoConflictEntryTime;
    //             pet.second = e->egoConflictEntryTime - e->foeConflictExitTime;
    //         } else if (e->egoConflictEntryTime == INVALID_DOUBLE || (e->egoConflictEntryTime != INVALID_DOUBLE && e->foeConflictEntryTime > e->egoConflictExitTime)) {
    //             pet.first = e->foeConflictEntryTime;
    //             pet.second = e->foeConflictEntryTime - e->egoConflictExitTime;
    //         } else {
    // #ifdef DEBUG_SSM
    //             if (DEBUG_COND(myHolderMS))
    //                 std::cout << "determinePET: Both passed conflict area in the same step. Assume collision"
    //                           << std::endl;
    // #endif
    //             pet.first = e->egoConflictEntryTime;
    //             pet.second = 0;
    //         }
    //
    //         // Reset entry and exit times two allow an eventual subsequent re-use
    //         e->egoConflictEntryTime = INVALID_DOUBLE;
    //         e->egoConflictExitTime = INVALID_DOUBLE;
    //         e->foeConflictEntryTime = INVALID_DOUBLE;
    //         e->foeConflictExitTime = INVALID_DOUBLE;
    //
    // #ifdef DEBUG_SSM
    //         if (DEBUG_COND(myHolderMS))
    //             std::cout << "Calculated PET = " << pet.second << " (at t=" << pet.first << ")"
    //                       << std::endl;
    // #endif
    //     } else {
    //         // other cases (merging and pre-crossing situations) do not correspond to a PET calculation.
    // #ifdef DEBUG_SSM
    //         if (DEBUG_COND(myHolderMS))
    //             std::cout << "PET unappropriate for merging and pre-crossing situations. No calculation performed."
    //                       << std::endl;
    // #endif
    //         return;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn determine_pet(&self, eInfo: &mut MSDevice_SSM_EncounterApproachInfo) {
        todo!("port MSDevice_SSM::determinePET")
    }

    /// C++: `void MSDevice_SSM::determineTTCandDRACandPPETandMDRAC(EncounterApproachInfo &) const` [const, private]
    /// Discriminates between different encounter types and correspondingly determines TTC and DRAC for those cases and writes the result to eInfo.ttc and eInfo.drac
    // void
    // MSDevice_SSM::determineTTCandDRACandPPETandMDRAC(EncounterApproachInfo& eInfo) const {
    //     Encounter* e = eInfo.encounter;
    //     const EncounterType& type = eInfo.type;
    //     double& ttc = eInfo.ttc;
    //     double& drac = eInfo.drac;
    //     double& ppet = eInfo.ppet;
    //     double& mdrac = eInfo.mdrac;
    //
    // #ifdef DEBUG_SSM
    //     if (DEBUG_COND(myHolderMS))
    //         std::cout << SIMTIME << " determineTTCandDRAC() for encounter of vehicles '" << e->egoID << "' and '" << e->foeID << "' (type = " << eInfo.type << ")"
    //                   << std::endl;
    // #endif
    //
    //     // Dependent on the actual encounter situation (eInfo.type) calculate the TTC.
    //     // For merging and crossing, different cases occur when a collision during the merging / crossing process is predicted.
    //     if (type == ENCOUNTER_TYPE_FOLLOWING_FOLLOWER) {
    //         double gap = eInfo.egoConflictEntryDist;
    //         if (myComputeTTC) {
    //             ttc = computeTTC(gap, e->ego->getSpeed(), e->foe->getSpeed());
    //         }
    //         if (myComputeDRAC) {
    //             drac = computeDRAC(gap, e->ego->getSpeed(), e->foe->getSpeed());
    //         }
    //         if (myComputeMDRAC) {
    //             mdrac = computeMDRAC(gap, e->ego->getSpeed(), e->foe->getSpeed(), myMDRACPRT);
    //         }
    //     } else if (type == ENCOUNTER_TYPE_FOLLOWING_LEADER) {
    //         double gap = eInfo.foeConflictEntryDist;
    //         if (myComputeTTC) {
    //             ttc = computeTTC(gap, e->foe->getSpeed(), e->ego->getSpeed());
    //         }
    //         if (myComputeDRAC) {
    //             drac = computeDRAC(gap, e->foe->getSpeed(), e->ego->getSpeed());
    //         }
    //         if (myComputeMDRAC) {
    //             mdrac = computeMDRAC(gap, e->foe->getSpeed(), e->ego->getSpeed(), myMDRACPRT);
    //         }
    //     } else if (type == ENCOUNTER_TYPE_ONCOMING) {
    //         if (myComputeTTC) {
    //             const double dv = e->ego->getSpeed() + e->foe->getSpeed();
    //             if (dv > 0) {
    //                 ttc = eInfo.egoConflictEntryDist / dv;
    //             }
    //         }
    //     } else if (type == ENCOUNTER_TYPE_MERGING_FOLLOWER || type == ENCOUNTER_TYPE_MERGING_LEADER) {
    //         // TODO: calculate more specifically whether a following situation in the merge conflict area
    //         //       is predicted when assuming constant speeds or whether a side collision is predicted.
    //         //       Currently, we ignore any conflict area before the actual merging point of the lanes.
    //
    //         // linearly extrapolated arrival times at the conflict
    //         // NOTE: These differ from the estimated times stored in eInfo
    //         double egoEntryTime = e->ego->getSpeed() > 0 ? eInfo.egoConflictEntryDist / e->ego->getSpeed() : INVALID_DOUBLE;
    //         double egoExitTime = e->ego->getSpeed() > 0 ? eInfo.egoConflictExitDist / e->ego->getSpeed() : INVALID_DOUBLE;
    //         double foeEntryTime = e->foe->getSpeed() > 0 ? eInfo.foeConflictEntryDist / e->foe->getSpeed() : INVALID_DOUBLE;
    //         double foeExitTime = e->foe->getSpeed() > 0 ? eInfo.foeConflictExitDist / e->foe->getSpeed() : INVALID_DOUBLE;
    //
    // #ifdef DEBUG_SSM
    //         if (DEBUG_COND(myHolderMS))
    //             std::cout << "   Conflict times with constant speed extrapolation for merging situation:\n   "
    //                       << " egoEntryTime=" << (egoEntryTime == INVALID_DOUBLE ? "NA" : ::toString(egoEntryTime))
    //                       << ", egoExitTime=" << (egoExitTime == INVALID_DOUBLE ? "NA" : ::toString(egoExitTime))
    //                       << ", foeEntryTime=" << (foeEntryTime == INVALID_DOUBLE ? "NA" : ::toString(foeEntryTime))
    //                       << ", foeExitTime=" << (foeExitTime == INVALID_DOUBLE ? "NA" : ::toString(foeExitTime))
    //                       << std::endl;
    // #endif
    //
    //         // based on that, we obtain
    //         if (egoEntryTime == INVALID_DOUBLE || foeEntryTime == INVALID_DOUBLE) {
    //             // at least one vehicle is stopped
    //             ttc = INVALID_DOUBLE;
    //             drac = INVALID_DOUBLE;
    //             mdrac = INVALID_DOUBLE;
    // #ifdef DEBUG_SSM
    //             if (DEBUG_COND(myHolderMS)) {
    //                 std::cout << "    No TTC and DRAC computed as one vehicle is stopped." << std::endl;
    //             }
    // #endif
    //             return;
    //         }
    //         double leaderEntryTime = MIN2(egoEntryTime, foeEntryTime);
    //         double followerEntryTime = MAX2(egoEntryTime, foeEntryTime);
    //         double leaderExitTime = leaderEntryTime == egoEntryTime ? egoExitTime : foeExitTime;
    //         //double followerExitTime = leaderEntryTime==egoEntryTime?foeExitTime:egoExitTime;
    //         double leaderSpeed = leaderEntryTime == egoEntryTime ? e->ego->getSpeed() : e->foe->getSpeed();
    //         double followerSpeed = leaderEntryTime == egoEntryTime ? e->foe->getSpeed() : e->ego->getSpeed();
    //         double leaderConflictDist = leaderEntryTime == egoEntryTime ? eInfo.egoConflictEntryDist : eInfo.foeConflictEntryDist;
    //         double followerConflictDist = leaderEntryTime == egoEntryTime ? eInfo.foeConflictEntryDist : eInfo.egoConflictEntryDist;
    //         double leaderLength = leaderEntryTime == egoEntryTime ? e->ego->getLength() : e->foe->getLength();
    //         if (myComputePPET && followerEntryTime != INVALID_DOUBLE && leaderEntryTime != INVALID_DOUBLE) {
    //             ppet = followerEntryTime - leaderExitTime;
    //             //std::cout << " debug1 ppet=" << ppet << "\n";
    //         }
    //         if (leaderExitTime >= followerEntryTime) {
    //             // collision would occur at merge area
    //             if (myComputeTTC) {
    //                 ttc = computeTTC(followerConflictDist, followerSpeed, 0.);
    //             }
    //             // TODO: Calculate more specific drac for merging case here (complete stop is not always necessary -> see calculation for crossing case)
    //             //       Rather the
    //             if (myComputeDRAC) {
    //                 drac = computeDRAC(followerConflictDist, followerSpeed, 0.);
    //             }
    //             if (myComputeMDRAC) {
    //                 mdrac = computeMDRAC(followerConflictDist, followerSpeed, 0., myMDRACPRT);
    //             }
    //
    // #ifdef DEBUG_SSM
    //             if (DEBUG_COND(myHolderMS))
    //                 std::cout << "    Extrapolation predicts collision *at* merge point with TTC=" << ttc
    //                           << ", drac=" << drac << std::endl;
    // #endif
    //
    //         } else {
    //             // -> No collision at the merge area
    //             if (myComputeTTC) {
    //                 // Check if after merge a collision would occur if speeds are hold constant.
    //                 double gapAfterMerge = followerConflictDist - leaderExitTime * followerSpeed;
    //                 assert(gapAfterMerge >= 0);
    //
    //                 // ttc as for following situation (assumes no collision until leader merged)
    //                 double ttcAfterMerge = computeTTC(gapAfterMerge, followerSpeed, leaderSpeed);
    //                 ttc = ttcAfterMerge == INVALID_DOUBLE ? INVALID_DOUBLE : leaderExitTime + ttcAfterMerge;
    //             }
    //             if (myComputeDRAC) {
    //                 // Intitial gap. (May be negative only if the leader speed is higher than the follower speed, i.e., dv < 0)
    //                 double g0 = followerConflictDist - leaderConflictDist - leaderLength;
    //                 if (g0 < 0) {
    //                     // Speed difference must be positive if g0<0.
    //                     assert(leaderSpeed - followerSpeed > 0);
    //                     // no deceleration needed for dv>0 and gap after merge >= 0
    //                     drac = INVALID_DOUBLE;
    //                 } else {
    //                     // compute drac as for a following situation
    //                     drac = computeDRAC(g0, followerSpeed, leaderSpeed);
    //                 }
    //             }
    //             if (myComputeMDRAC) {
    //                 // Intitial gap. (May be negative only if the leader speed is higher than the follower speed, i.e., dv < 0)
    //                 double g0 = followerConflictDist - leaderConflictDist - leaderLength;
    //                 if (g0 < 0) {
    //                     // Speed difference must be positive if g0<0.
    //                     assert(leaderSpeed - followerSpeed > 0);
    //                     // no deceleration needed for dv>0 and gap after merge >= 0
    //                     mdrac = INVALID_DOUBLE;
    //                 } else {
    //                     // compute drac as for a following situation
    //                     mdrac = computeMDRAC(g0, followerSpeed, leaderSpeed, myMDRACPRT);
    //                 }
    //             }
    // #ifdef DEBUG_SSM
    //             if (DEBUG_COND(myHolderMS)) {
    //                 if (ttc == INVALID_DOUBLE) {
    //                     // assert(dv >= 0);
    //                     assert(drac == INVALID_DOUBLE || drac == 0.0);
    //                     std::cout << "    Extrapolation does not predict any collision." << std::endl;
    //                 } else {
    //                     std::cout << "    Extrapolation predicts collision *after* merge point with TTC="
    //                               << (ttc == INVALID_DOUBLE ? "NA" : ::toString(ttc))
    //                               << ", drac=" << (drac == INVALID_DOUBLE ? "NA" : ::toString(drac)) << std::endl;
    //                 }
    //             }
    // #endif
    //
    //         }
    //
    //     } else if (type == ENCOUNTER_TYPE_CROSSING_FOLLOWER
    //                || type == ENCOUNTER_TYPE_FOE_ENTERED_CONFLICT_AREA) {
    //         if (myComputeDRAC) {
    //             drac = computeDRAC(eInfo);
    //         }
    //         if (eInfo.egoEstimatedConflictEntryTime <= eInfo.foeEstimatedConflictExitTime && eInfo.egoEstimatedConflictEntryTime != INVALID_DOUBLE) {
    //             // follower's predicted arrival at the crossing area is earlier than the leader's predicted exit -> collision predicted
    //             double gap = eInfo.egoConflictEntryDist;
    //             if (myComputeTTC) {
    //                 ttc = computeTTC(gap, e->ego->getSpeed(), 0.);
    //             }
    //             if (myComputeMDRAC) {
    //                 mdrac = computeMDRAC(gap, e->ego->getSpeed(), 0., myMDRACPRT);
    //             }
    //         } else {
    //             // encounter is expected to happen without collision
    //             ttc = INVALID_DOUBLE;
    //             mdrac = INVALID_DOUBLE;
    //         }
    //         if (myComputePPET) {
    //             const double entryTime = eInfo.egoEstimatedConflictEntryTime;
    //             const double exitTime = (e->foeConflictExitTime == INVALID_DOUBLE
    //                                      ? eInfo.foeEstimatedConflictExitTime : e->foeConflictExitTime);
    //             if (entryTime != INVALID_DOUBLE && exitTime != INVALID_DOUBLE) {
    //                 ppet = entryTime - exitTime;
    //             }
    //             //std::cout << " debug2 ppet=" << ppet << "\n";
    //         }
    //
    //     } else if (type == ENCOUNTER_TYPE_CROSSING_LEADER
    //                || type == ENCOUNTER_TYPE_EGO_ENTERED_CONFLICT_AREA) {
    //         if (myComputeDRAC) {
    //             drac = computeDRAC(eInfo);
    //         }
    //         if (eInfo.foeEstimatedConflictEntryTime <= eInfo.egoEstimatedConflictExitTime && eInfo.foeEstimatedConflictEntryTime != INVALID_DOUBLE) {
    //             // follower's predicted arrival at the crossing area is earlier than the leader's predicted exit -> collision predicted
    //             double gap = eInfo.foeConflictEntryDist;
    //             if (myComputeTTC) {
    //                 ttc = computeTTC(gap, e->foe->getSpeed(), 0.);
    //             }
    //             if (myComputeMDRAC) {
    //                 mdrac = computeMDRAC(gap, e->foe->getSpeed(), 0., myMDRACPRT);
    //             }
    //         } else {
    //             // encounter is expected to happen without collision
    //             ttc = INVALID_DOUBLE;
    //             mdrac = INVALID_DOUBLE;
    //         }
    //         if (myComputePPET) {
    //             const double entryTime = eInfo.foeEstimatedConflictEntryTime;
    //             const double exitTime = (e->egoConflictExitTime == INVALID_DOUBLE
    //                                      ? eInfo.egoEstimatedConflictExitTime : e->egoConflictExitTime);
    //             if (entryTime != INVALID_DOUBLE && exitTime != INVALID_DOUBLE) {
    //                 ppet = entryTime - exitTime;
    //             }
    //             //std::cout << SIMTIME << " debug3 ppet=" << writeNA(ppet)
    //             //    << " fecet=" << writeNA(eInfo.foeEstimatedConflictEntryTime)
    //             //    << " ecet=" << writeNA(e->egoConflictExitTime)
    //             //    << " eecec=" << writeNA(eInfo.egoEstimatedConflictExitTime)
    //             //    << "\n";
    //         }
    //     } else {
    // #ifdef DEBUG_SSM
    //         if (DEBUG_COND(myHolderMS)) {
    //             std::stringstream ss;
    //             ss << "'" << type << "'";
    //             WRITE_WARNING("Underspecified or unknown encounter type in MSDevice_SSM::determineTTCandDRAC(): " + ss.str());
    //         }
    // #endif
    //     }
    //
    // #ifdef DEBUG_SSM
    //     if (DEBUG_COND(myHolderMS))
    //         std::cout << "ttc=" << (ttc == INVALID_DOUBLE ? "NA" : ::toString(ttc)) << ", drac=" << (drac == INVALID_DOUBLE ? "NA" : ::toString(drac))
    //                   << std::endl;
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn determine_tt_cand_dra_cand_ppe_tand_mdrac(&self, eInfo: &mut MSDevice_SSM_EncounterApproachInfo) {
        todo!("port MSDevice_SSM::determineTTCandDRACandPPETandMDRAC")
    }

    /// C++: `double MSDevice_SSM::computeTTC(double, double, double) const` [const, private]
    /// Computes the time to collision (in seconds) for two vehicles with a given initial gap under the assumption that both maintain their current speeds. Returns INVALID if no collision would occur under this assumption.
    // double
    // MSDevice_SSM::computeTTC(double gap, double followerSpeed, double leaderSpeed) const {
    //     // TODO: in merging situations, the TTC may be lower than the one computed here for following situations
    //     //  (currently only the cross section corresponding to the target lane's begin is considered)
    //     // More specifically, the minimum has to be taken from the two if a collision at merge was predicted.
    // #ifdef DEBUG_SSM
    //     if (DEBUG_COND(myHolderMS))
    //         std::cout << "computeTTC() with gap=" << gap << ", followerSpeed=" << followerSpeed << ", leaderSpeed=" << leaderSpeed
    //                   << std::endl;
    // #endif
    //     if (gap <= 0.) {
    //         return 0.;    // collision already happend
    //     }
    //     double dv = followerSpeed - leaderSpeed;
    //     if (dv <= 0.) {
    //         return INVALID_DOUBLE;    // no collision
    //     }
    //
    //     return gap / dv;
    // }
    #[allow(unused_variables)]
    pub fn compute_ttc(&self, gap: f64, followerSpeed: f64, leaderSpeed: f64) -> f64 {
        todo!("port MSDevice_SSM::computeTTC")
    }

    /// C++: `double MSDevice_SSM::computeDRAC(double, double, double)` [static, private]
    /// Computes the DRAC (deceleration to avoid a collision) for a lead/follow situation as defined, e.g., in Guido et al. (2011, Safety performance measures: a comparison between microsimulation and observational data) for two vehicles with a given gap. Returns 0.0 if no deceleration is required by the follower to avoid a crash, INVALID if collision is detected.
    // double
    // MSDevice_SSM::computeDRAC(double gap, double followerSpeed, double leaderSpeed) {
    // //#ifdef DEBUG_SSM_DRAC
    // //    if (DEBUG_COND)
    // //    std::cout << "computeDRAC() with gap=" << gap << ", followerSpeed=" << followerSpeed << ", leaderSpeed=" << leaderSpeed
    // //              << std::endl;
    // //#endif
    //     if (gap <= 0.) {
    //         return INVALID_DOUBLE;    // collision!
    //     }
    //     double dv = followerSpeed - leaderSpeed;
    //     if (dv <= 0.) {
    //         return 0.0;    // no need to break
    //     }
    //     assert(followerSpeed > 0.);
    //     return 0.5 * dv * dv / gap; // following Guido et al. (2011)
    // }
    #[allow(unused_variables)]
    pub fn compute_drac(gap: f64, followerSpeed: f64, leaderSpeed: f64) -> f64 {
        todo!("port MSDevice_SSM::computeDRAC")
    }

    /// C++: `double MSDevice_SSM::computeMDRAC(double, double, double, double)` [static, private]
    /// Computes the MDRAC (deceleration to avoid a collision) for a lead/follow situation as defined considering a reaction time of follower, e.g., in Fazekas et al. (2017, A Novel Surrogate Indicator Based on Constant Initial Acceleration and Reaction Time Assumption) for two vehicles with a given gap. Returns 0.0 if no deceleration is required by the follower to avoid a crash, INVALID if collision is detected.
    // double
    // MSDevice_SSM::computeMDRAC(double gap, double followerSpeed, double leaderSpeed, double prt) {
    // //#ifdef DEBUG_SSM_DRAC
    // //    if (DEBUG_COND)
    // //    std::cout << "computeMDRAC() with gap=" << gap << ", followerSpeed=" << followerSpeed << ", leaderSpeed=" << leaderSpeed
    // //              << std::endl;
    // //#endif
    //     if (gap <= 0.) {
    //         return INVALID_DOUBLE;    // collision!
    //     }
    //     double dv = followerSpeed - leaderSpeed;
    //     if (dv <= 0.) {
    //         return 0.0;    // no need to brake
    //     }
    //     if (gap / dv == prt) {
    //         return INVALID_DOUBLE;
    //     }
    //     assert(followerSpeed > 0.);
    //     return 0.5 * dv / (gap / dv - prt);
    // }
    #[allow(unused_variables)]
    pub fn compute_mdrac(gap: f64, followerSpeed: f64, leaderSpeed: f64, prt: f64) -> f64 {
        todo!("port MSDevice_SSM::computeMDRAC")
    }

    /// C++: `double MSDevice_SSM::computeDRAC(const EncounterApproachInfo &)` [static, private]
    /// Computes the DRAC a crossing situation, determining the minimal constant deceleration needed for one of the vehicles to reach the conflict area after the other has left. for estimated leaving times, current deceleration is extrapolated, and acceleration is neglected. Returns 0.0 if no deceleration is required by the follower to avoid a crash, INVALID if collision is detected.
    // double
    // MSDevice_SSM::computeDRAC(const EncounterApproachInfo& eInfo) {
    //     // Introduce concise variable names
    //     double dEntry1 = eInfo.egoConflictEntryDist;
    //     double dEntry2 = eInfo.foeConflictEntryDist;
    //     double dExit1 = eInfo.egoConflictExitDist;
    //     double dExit2 = eInfo.foeConflictExitDist;
    //     double v1 = eInfo.encounter->ego->getSpeed();
    //     double v2 = eInfo.encounter->foe->getSpeed();
    //     double tEntry1 = eInfo.egoEstimatedConflictEntryTime;
    //     double tEntry2 = eInfo.foeEstimatedConflictEntryTime;
    //     double tExit1 = eInfo.egoEstimatedConflictExitTime;
    //     double tExit2 = eInfo.foeEstimatedConflictExitTime;
    // #ifdef DEBUG_SSM_DRAC
    //     if (DEBUG_COND(eInfo.encounter->ego))
    //         std::cout << SIMTIME << "computeDRAC() with"
    //                   << "\ndEntry1=" << dEntry1 << ", dEntry2=" << dEntry2
    //                   << ", dExit1=" << dExit1 << ", dExit2=" << dExit2
    //                   << ",\nv1=" << v1 << ", v2=" << v2
    //                   << "\ntEntry1=" << (tEntry1 == INVALID_DOUBLE ? "NA" : ::toString(tEntry1)) << ", tEntry2=" << (tEntry2 == INVALID_DOUBLE ? "NA" : ::toString(tEntry2))
    //                   << ", tExit1=" << (tExit1 == INVALID_DOUBLE ? "NA" : ::toString(tExit1)) << ", tExit2=" << (tExit2 == INVALID_DOUBLE ? "NA" : ::toString(tExit2))
    //                   << std::endl;
    // #endif
    //     if (dExit1 <= 0. || dExit2 <= 0.) {
    //         // At least one vehicle already left or is not about to enter conflict area at all => no breaking needed.
    // #ifdef DEBUG_SSM_DRAC
    //         if (DEBUG_COND(eInfo.encounter->ego)) {
    //             std::cout << "One already left conflict area -> drac == 0." << std::endl;
    //         }
    // #endif
    //         return 0.;
    //     }
    //     if (dEntry1 <= 0. && dEntry2 <= 0.) {
    //         // collision... (both already entered conflict area but none left)
    // #ifdef DEBUG_SSM_DRAC
    //         if (DEBUG_COND(eInfo.encounter->ego)) {
    //             std::cout << "Both entered conflict area but neither left. -> collision!" << std::endl;
    //         }
    // #endif
    //         return INVALID_DOUBLE;
    //     }
    //
    //     double drac = std::numeric_limits<double>::max();
    //     if (dEntry1 > 0.) {
    //         // vehicle 1 could break
    // #ifdef DEBUG_SSM_DRAC
    //         if (DEBUG_COND(eInfo.encounter->ego)) {
    //             std::cout << "Ego could break..." << std::endl;
    //         }
    // #endif
    //         if (tExit2 != INVALID_DOUBLE) {
    //             // Vehicle 2 is expected to leave conflict area at t2
    //             drac = MIN2(drac, 2 * (v1 - dEntry1 / tExit2) / tExit2);
    // #ifdef DEBUG_SSM_DRAC
    //             if (DEBUG_COND(eInfo.encounter->ego)) {
    //                 std::cout << "  Foe expected to leave in " << tExit2 << "-> Ego needs drac=" << drac << std::endl;
    //             }
    // #endif
    //         } else {
    //             // Vehicle 2 is expected to stop on conflict area or earlier
    //             if (tEntry2 != INVALID_DOUBLE) {
    //                 // ... on conflict area => veh1 has to stop before entry
    //                 drac = MIN2(drac, computeDRAC(dEntry1, v1, 0));
    // #ifdef DEBUG_SSM_DRAC
    //                 if (DEBUG_COND(eInfo.encounter->ego)) {
    //                     std::cout << "  Foe is expected stop on conflict area -> Ego needs drac=" << drac << std::endl;
    //                 }
    // #endif
    //             } else {
    //                 // ... before conflict area
    // #ifdef DEBUG_SSM_DRAC
    //                 if (DEBUG_COND(eInfo.encounter->ego)) {
    //                     std::cout << "  Foe is expected stop before conflict area -> no drac computation for ego (will be done for foe if applicable)" << std::endl;
    //                 }
    // #endif
    //             }
    //         }
    //     }
    //
    //     if (dEntry2 > 0.) {
    //         // vehicle 2 could break
    // #ifdef DEBUG_SSM_DRAC
    //         if (DEBUG_COND(eInfo.encounter->ego)) {
    //             std::cout << "Foe could break..." << std::endl;
    //         }
    // #endif
    //         if (tExit1 != INVALID_DOUBLE) {
    //             // Vehicle 1 is expected to leave conflict area at t1
    // #ifdef DEBUG_SSM_DRAC
    //             if (DEBUG_COND(eInfo.encounter->ego)) {
    //                 std::cout << "  Ego expected to leave in " << tExit1 << "-> Foe needs drac=" << (2 * (v2 - dEntry2 / tExit1) / tExit1) << std::endl;
    //             }
    // #endif
    //             drac = MIN2(drac, 2 * (v2 - dEntry2 / tExit1) / tExit1);
    //         } else {
    //             // Vehicle 1 is expected to stop on conflict area or earlier
    //             if (tEntry1 != INVALID_DOUBLE) {
    //                 // ... on conflict area => veh2 has to stop before entry
    // #ifdef DEBUG_SSM_DRAC
    //                 if (DEBUG_COND(eInfo.encounter->ego)) {
    //                     std::cout << "  Ego is expected stop on conflict area -> Foe needs drac=" << computeDRAC(dEntry2, v2, 0) << std::endl;
    //                 }
    // #endif
    //                 drac = MIN2(drac, computeDRAC(dEntry2, v2, 0));
    //             } else {
    //                 // ... before conflict area
    // #ifdef DEBUG_SSM_DRAC
    //                 if (DEBUG_COND(eInfo.encounter->ego)) {
    //                     std::cout << "  Ego is expected stop before conflict area -> no drac computation for foe (done for ego if applicable)" << std::endl;
    //                 }
    // #endif
    //             }
    //         }
    //     }
    //
    //     return drac > 0 ? drac : INVALID_DOUBLE;
    // }
    #[allow(unused_variables)]
    pub fn compute_drac_2(eInfo: &MSDevice_SSM_EncounterApproachInfo) -> f64 {
        todo!("port MSDevice_SSM::computeDRAC")
    }

    /// C++: `std::string MSDevice_SSM::makeStringWithNAs(const int &, const double)` [static, private]
    /// make a string of a double vector and treat a special value as invalid ("NA")
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn make_string_with_n_as(v: &i32, NA: f64) -> String {
        todo!("port MSDevice_SSM::makeStringWithNAs")
    }

    /// C++: `std::string MSDevice_SSM::makeStringWithNAs(const int &, const int &)` [static, private]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn make_string_with_n_as_2(v: &i32, NAs: &i32) -> String {
        todo!("port MSDevice_SSM::makeStringWithNAs")
    }

    /// C++: `std::string MSDevice_SSM::makeStringWithNAs(const PositionVector &)` [private]
    // std::string
    // MSDevice_SSM::makeStringWithNAs(const PositionVector& v) {
    //     const int precision = myUseGeoCoords ? gPrecisionGeo : gPrecision;
    //     std::string res = "";
    //     for (PositionVector::const_iterator i = v.begin(); i != v.end(); ++i) {
    //         res += (i == v.begin() ? "" : " ") + (*i == Position::INVALID ? "NA" : ::toString(*i, precision));
    //     }
    //     return res;
    // }
    #[allow(unused_variables)]
    pub fn make_string_with_n_as_3(&mut self, v: &PositionVector) -> String {
        todo!("port MSDevice_SSM::makeStringWithNAs")
    }

    /// C++: `std::string MSDevice_SSM::makeStringWithNAs(const Position &)` [private]
    // std::string
    // MSDevice_SSM::makeStringWithNAs(const Position& p) {
    //     const int precision = myUseGeoCoords ? gPrecisionGeo : gPrecision;
    //     return p == Position::INVALID ? "NA" : toString(p, precision);
    // }
    #[allow(unused_variables)]
    pub fn make_string_with_n_as_4(&mut self, p: &Position) -> String {
        todo!("port MSDevice_SSM::makeStringWithNAs")
    }

    /// C++: `std::string MSDevice_SSM::writeNA(double, double)` [static, private]
    // std::string
    // MSDevice_SSM::writeNA(double v, double NA) {
    //     return v == NA ? "NA" : toString(v);
    // }
    #[allow(unused_variables)]
    pub fn write_na(v: f64, NA: f64) -> String {
        todo!("port MSDevice_SSM::writeNA")
    }

    /// C++: `std::string MSDevice_SSM::getOutputFilename(const SUMOVehicle &, std::string)` [static, private]
    /// @{
    // std::string
    // MSDevice_SSM::getOutputFilename(const SUMOVehicle& v, std::string deviceID) {
    //     OptionsCont& oc = OptionsCont::getOptions();
    //     std::string file = deviceID + ".xml";
    //     std::string basePath = "";
    //     if (v.getParameter().hasParameter("device.ssm.file")) {
    //         try {
    //             file = v.getParameter().getParameter("device.ssm.file", file);
    //         } catch (...) {
    //             WRITE_WARNINGF(TL("Invalid value '%' for vehicle parameter 'ssm.measures'."), v.getParameter().getParameter("device.ssm.file", file));
    //         }
    //     } else if (v.getVehicleType().getParameter().hasParameter("device.ssm.file")) {
    //         try {
    //             file = v.getVehicleType().getParameter().getParameter("device.ssm.file", file);
    //         } catch (...) {
    //             WRITE_WARNINGF(TL("Invalid value '%' for vType parameter 'ssm.measures'."), v.getVehicleType().getParameter().getParameter("device.ssm.file", file));
    //         }
    //     } else {
    //         file = oc.getString("device.ssm.file") == "" ? file : oc.getString("device.ssm.file");
    //         if (oc.isDefault("device.ssm.file") && (myIssuedParameterWarnFlags & SSM_WARN_FILE) == 0) {
    //             WRITE_MESSAGEF(TL("Vehicle '%' does not supply vehicle parameter 'device.ssm.file'. Using default of '%'."), v.getID(), file);
    //             myIssuedParameterWarnFlags |= SSM_WARN_FILE;
    //         }
    //         if (OptionsCont::getOptions().isSet("configuration-file") && !oc.isDefault("device.ssm.file")) {
    //             basePath = OptionsCont::getOptions().getString("configuration-file");
    //         }
    //     }
    //     if (basePath != "") {
    //         file = FileHelpers::checkForRelativity(file, basePath);
    //         try {
    //             file = StringUtils::urlDecode(file);
    //         } catch (NumberFormatException& e) {
    //             WRITE_WARNING(toString(e.what()) + " when trying to decode filename '" + file + "'.");
    //         }
    //     }
    //     return file;
    // }
    #[allow(unused_variables)]
    pub fn get_output_filename(v: &SUMOVehicle, deviceID: String) -> String {
        todo!("port MSDevice_SSM::getOutputFilename")
    }

    /// C++: `double MSDevice_SSM::getDetectionRange(const SUMOVehicle &)` [static, private]
    // double
    // MSDevice_SSM::getDetectionRange(const SUMOVehicle& v) {
    //     OptionsCont& oc = OptionsCont::getOptions();
    //     double range = -INVALID_DOUBLE;
    //     if (v.getParameter().hasParameter("device.ssm.range")) {
    //         try {
    //             range = StringUtils::toDouble(v.getParameter().getParameter("device.ssm.range", ""));
    //         } catch (...) {
    //             WRITE_WARNINGF(TL("Invalid value '%' for vehicle parameter 'ssm.range'."), v.getParameter().getParameter("device.ssm.range", ""));
    //         }
    //     } else if (v.getVehicleType().getParameter().hasParameter("device.ssm.range")) {
    //         try {
    //             range = StringUtils::toDouble(v.getVehicleType().getParameter().getParameter("device.ssm.range", ""));
    //         } catch (...) {
    //             WRITE_WARNINGF(TL("Invalid value '%' for vType parameter 'ssm.range'."), v.getVehicleType().getParameter().getParameter("device.ssm.range", ""));
    //         }
    //     } else {
    //         range = oc.getFloat("device.ssm.range");
    //         if (oc.isDefault("device.ssm.range") && (myIssuedParameterWarnFlags & SSM_WARN_RANGE) == 0) {
    //             WRITE_MESSAGEF(TL("Vehicle '%' does not supply vehicle parameter 'device.ssm.range'. Using default of '%'."), v.getID(), toString(range));
    //             myIssuedParameterWarnFlags |= SSM_WARN_RANGE;
    //         }
    //     }
    //     return range;
    // }
    #[allow(unused_variables)]
    pub fn get_detection_range(v: &SUMOVehicle) -> f64 {
        todo!("port MSDevice_SSM::getDetectionRange")
    }

    /// C++: `double MSDevice_SSM::getMDRAC_PRT(const SUMOVehicle &)` [static, private]
    // double
    // MSDevice_SSM::getMDRAC_PRT(const SUMOVehicle& v) {
    //     OptionsCont& oc = OptionsCont::getOptions();
    //     double prt = 1;
    //     if (v.getParameter().hasParameter("device.ssm.mdrac.prt")) {
    //         try {
    //             prt = StringUtils::toDouble(v.getParameter().getParameter("device.ssm.mdrac.prt", ""));
    //         } catch (...) {
    //             WRITE_WARNINGF(TL("Invalid value '%' for vehicle parameter 'ssm.mdrac.prt'."), v.getParameter().getParameter("device.ssm.mdrac.prt", ""));
    //         }
    //     } else if (v.getVehicleType().getParameter().hasParameter("device.ssm.mdrac.prt")) {
    //         try {
    //             prt = StringUtils::toDouble(v.getVehicleType().getParameter().getParameter("device.ssm.mdrac.prt", ""));
    //         } catch (...) {
    //             WRITE_WARNINGF(TL("Invalid value '%' for vType parameter 'ssm.mdrac.prt'."), v.getVehicleType().getParameter().getParameter("device.ssm.mdrac.prt", ""));
    //         }
    //     } else {
    //         prt = oc.getFloat("device.ssm.mdrac.prt");
    //         if (oc.isDefault("device.ssm.mdrac.prt") && (myIssuedParameterWarnFlags & SSM_WARN_MDRAC_PRT) == 0) {
    //             WRITE_MESSAGEF(TL("Vehicle '%' does not supply vehicle parameter 'device.ssm.mdrac.prt'. Using default of '%'."), v.getID(), toString(prt));
    //             myIssuedParameterWarnFlags |= SSM_WARN_MDRAC_PRT;
    //         }
    //     }
    //     return prt;
    // }
    #[allow(unused_variables)]
    pub fn get_mdrac_prt(v: &SUMOVehicle) -> f64 {
        todo!("port MSDevice_SSM::getMDRAC_PRT")
    }

    /// C++: `double MSDevice_SSM::getExtraTime(const SUMOVehicle &)` [static, private]
    // double
    // MSDevice_SSM::getExtraTime(const SUMOVehicle& v) {
    //     OptionsCont& oc = OptionsCont::getOptions();
    //     double extraTime = INVALID_DOUBLE;
    //     if (v.getParameter().hasParameter("device.ssm.extratime")) {
    //         try {
    //             extraTime = StringUtils::toDouble(v.getParameter().getParameter("device.ssm.extratime", ""));
    //         } catch (...) {
    //             WRITE_WARNINGF(TL("Invalid value '%' for vehicle parameter 'ssm.extratime'."), v.getParameter().getParameter("device.ssm.extratime", ""));
    //         }
    //     } else if (v.getVehicleType().getParameter().hasParameter("device.ssm.extratime")) {
    //         try {
    //             extraTime = StringUtils::toDouble(v.getVehicleType().getParameter().getParameter("device.ssm.extratime", ""));
    //         } catch (...) {
    //             WRITE_WARNINGF(TL("Invalid value '%' for vType parameter 'ssm.extratime'."), v.getVehicleType().getParameter().getParameter("device.ssm.extratime", ""));
    //         }
    //     } else {
    //         extraTime = oc.getFloat("device.ssm.extratime");
    //         if (oc.isDefault("device.ssm.extratime") && (myIssuedParameterWarnFlags & SSM_WARN_EXTRATIME) == 0) {
    //             WRITE_MESSAGEF(TL("Vehicle '%' does not supply vehicle parameter 'device.ssm.extratime'. Using default of '%'."), v.getID(), toString(extraTime));
    //             myIssuedParameterWarnFlags |= SSM_WARN_EXTRATIME;
    //         }
    //     }
    //     if (extraTime < 0.) {
    //         extraTime = DEFAULT_EXTRA_TIME;
    //         WRITE_WARNINGF(TL("Negative (or no) value encountered for vehicle parameter 'device.ssm.extratime' in vehicle '%' using default value % instead."), v.getID(), ::toString(extraTime));
    //     }
    //     return extraTime;
    // }
    #[allow(unused_variables)]
    pub fn get_extra_time(v: &SUMOVehicle) -> f64 {
        todo!("port MSDevice_SSM::getExtraTime")
    }

    /// C++: `bool MSDevice_SSM::useGeoCoords(const SUMOVehicle &)` [static, private]
    // bool
    // MSDevice_SSM::useGeoCoords(const SUMOVehicle& v) {
    //     OptionsCont& oc = OptionsCont::getOptions();
    //     bool useGeo = false;
    //     if (v.getParameter().hasParameter("device.ssm.geo")) {
    //         try {
    //             useGeo = StringUtils::toBool(v.getParameter().getParameter("device.ssm.geo", "no"));
    //         } catch (...) {
    //             WRITE_WARNINGF(TL("Invalid value '%' for vehicle parameter 'ssm.geo'."), v.getParameter().getParameter("device.ssm.geo", "no"));
    //         }
    //     } else if (v.getVehicleType().getParameter().hasParameter("device.ssm.geo")) {
    //         try {
    //             useGeo = StringUtils::toBool(v.getVehicleType().getParameter().getParameter("device.ssm.geo", "no"));
    //         } catch (...) {
    //             WRITE_WARNINGF(TL("Invalid value '%' for vType parameter 'ssm.geo'."), v.getVehicleType().getParameter().getParameter("device.ssm.geo", "no"));
    //         }
    //     } else {
    //         useGeo = oc.getBool("device.ssm.geo");
    //         if (oc.isDefault("device.ssm.geo") && (myIssuedParameterWarnFlags & SSM_WARN_GEO) == 0) {
    //             WRITE_MESSAGEF(TL("Vehicle '%' does not supply vehicle parameter 'device.ssm.geo'. Using default of '%'."), v.getID(), toString(useGeo));
    //             myIssuedParameterWarnFlags |= SSM_WARN_GEO;
    //         }
    //     }
    //     return useGeo;
    // }
    #[allow(unused_variables)]
    pub fn use_geo_coords(v: &SUMOVehicle) -> bool {
        todo!("port MSDevice_SSM::useGeoCoords")
    }

    /// C++: `bool MSDevice_SSM::writePositions(const SUMOVehicle &)` [static, private]
    // bool
    // MSDevice_SSM::writePositions(const SUMOVehicle& v) {
    //     OptionsCont& oc = OptionsCont::getOptions();
    //     bool writePos = false;
    //     if (v.getParameter().hasParameter("device.ssm.write-positions")) {
    //         try {
    //             writePos = StringUtils::toBool(v.getParameter().getParameter("device.ssm.write-positions", "no"));
    //         } catch (...) {
    //             WRITE_WARNINGF(TL("Invalid value '%' for vehicle parameter 'ssm.write-positions'."), v.getParameter().getParameter("device.ssm.write-positions", "no"));
    //         }
    //     } else if (v.getVehicleType().getParameter().hasParameter("device.ssm.write-positions")) {
    //         try {
    //             writePos = StringUtils::toBool(v.getVehicleType().getParameter().getParameter("device.ssm.write-positions", "no"));
    //         } catch (...) {
    //             WRITE_WARNINGF(TL("Invalid value '%' for vType parameter 'ssm.write-positions'."), v.getVehicleType().getParameter().getParameter("device.ssm.write-positions", "no"));
    //         }
    //     } else {
    //         writePos = oc.getBool("device.ssm.write-positions");
    //         if (oc.isDefault("device.ssm.write-positions") && (myIssuedParameterWarnFlags & SSM_WARN_POS) == 0) {
    //             WRITE_MESSAGEF(TL("Vehicle '%' does not supply vehicle parameter 'device.ssm.write-positions'. Using default of '%'."), v.getID(), toString(writePos));
    //             myIssuedParameterWarnFlags |= SSM_WARN_POS;
    //         }
    //     }
    //     return writePos;
    // }
    #[allow(unused_variables)]
    pub fn write_positions(v: &SUMOVehicle) -> bool {
        todo!("port MSDevice_SSM::writePositions")
    }

    /// C++: `bool MSDevice_SSM::writeLanesPositions(const SUMOVehicle &)` [static, private]
    // bool
    // MSDevice_SSM::writeLanesPositions(const SUMOVehicle& v) {
    //     OptionsCont& oc = OptionsCont::getOptions();
    //     bool writeLanesPos = false;
    //     if (v.getParameter().hasParameter("device.ssm.write-lane-positions")) {
    //         try {
    //             writeLanesPos = StringUtils::toBool(v.getParameter().getParameter("device.ssm.write-lane-positions", "no"));
    //         } catch (...) {
    //             WRITE_WARNINGF(TL("Invalid value '%' for vehicle parameter 'ssm.write-lane-positions'."), v.getParameter().getParameter("device.ssm.write-lane-positions", "no"));
    //         }
    //     } else if (v.getVehicleType().getParameter().hasParameter("device.ssm.write-lane-positions")) {
    //         try {
    //             writeLanesPos = StringUtils::toBool(v.getVehicleType().getParameter().getParameter("device.ssm.write-lane-positions", "no"));
    //         } catch (...) {
    //             WRITE_WARNINGF(TL("Invalid value '%' for vType parameter 'ssm.write-lane-positions'."), v.getVehicleType().getParameter().getParameter("device.ssm.write-lane-positions", "no"));
    //         }
    //     } else {
    //         writeLanesPos = oc.getBool("device.ssm.write-lane-positions");
    //         if (oc.isDefault("device.ssm.write-lane-positions") && (myIssuedParameterWarnFlags & SSM_WARN_LANEPOS) == 0) {
    //             WRITE_MESSAGEF(TL("Vehicle '%' does not supply vehicle parameter 'device.ssm.write-lane-positions'. Using default of '%'."), v.getID(), toString(writeLanesPos));
    //             myIssuedParameterWarnFlags |= SSM_WARN_LANEPOS;
    //         }
    //     }
    //     return writeLanesPos;
    // }
    #[allow(unused_variables)]
    pub fn write_lanes_positions(v: &SUMOVehicle) -> bool {
        todo!("port MSDevice_SSM::writeLanesPositions")
    }

    /// C++: `bool MSDevice_SSM::filterByConflictType(const SUMOVehicle &, std::string, int &)` [static, private]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn filter_by_conflict_type(v: &SUMOVehicle, deviceID: String, conflictTypes: &mut i32) -> bool {
        todo!("port MSDevice_SSM::filterByConflictType")
    }

    /// C++: `bool MSDevice_SSM::requestsTrajectories(const SUMOVehicle &)` [static, private]
    // bool
    // MSDevice_SSM::requestsTrajectories(const SUMOVehicle& v) {
    //     OptionsCont& oc = OptionsCont::getOptions();
    //     bool trajectories = false;
    //     if (v.getParameter().hasParameter("device.ssm.trajectories")) {
    //         try {
    //             trajectories = StringUtils::toBool(v.getParameter().getParameter("device.ssm.trajectories", "no"));
    //         } catch (...) {
    //             WRITE_WARNINGF(TL("Invalid value '%' for vehicle parameter 'ssm.trajectories'."), v.getParameter().getParameter("device.ssm.trajectories", "no"));
    //         }
    //     } else if (v.getVehicleType().getParameter().hasParameter("device.ssm.trajectories")) {
    //         try {
    //             trajectories = StringUtils::toBool(v.getVehicleType().getParameter().getParameter("device.ssm.trajectories", "no"));
    //         } catch (...) {
    //             WRITE_WARNINGF(TL("Invalid value '%' for vType parameter 'ssm.trajectories'."), v.getVehicleType().getParameter().getParameter("device.ssm.trajectories", "no"));
    //         }
    //     } else {
    //         trajectories = oc.getBool("device.ssm.trajectories");
    //         if (oc.isDefault("device.ssm.trajectories") && (myIssuedParameterWarnFlags & SSM_WARN_TRAJECTORIES) == 0) {
    //             WRITE_MESSAGEF(TL("Vehicle '%' does not supply vehicle parameter 'device.ssm.trajectories'. Using default of '%'."), v.getID(), toString(trajectories));
    //             myIssuedParameterWarnFlags |= SSM_WARN_TRAJECTORIES;
    //         }
    //     }
    //     return trajectories;
    // }
    #[allow(unused_variables)]
    pub fn requests_trajectories(v: &SUMOVehicle) -> bool {
        todo!("port MSDevice_SSM::requestsTrajectories")
    }

    /// C++: `bool MSDevice_SSM::getMeasuresAndThresholds(const SUMOVehicle &, std::string, int &)` [static, private]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_measures_and_thresholds(v: &SUMOVehicle, deviceID: String, thresholds: &mut i32) -> bool {
        todo!("port MSDevice_SSM::getMeasuresAndThresholds")
    }

    /// C++: `void MSDevice_SSM::initEdgeFilter()` [static, private]
    /// initialize edge filter (once)
    // void
    // MSDevice_SSM::initEdgeFilter() {
    //     myEdgeFilterInitialized = true;
    //     if (OptionsCont::getOptions().isSet("device.ssm.filter-edges.input-file")) {
    //         const std::string file = OptionsCont::getOptions().getString("device.ssm.filter-edges.input-file");
    //         std::ifstream strm(file.c_str());
    //         if (!strm.good()) {
    //             throw ProcessError(TLF("Could not load names of edges for filtering SSM device output from '%'.", file));
    //         }
    //         myEdgeFilterActive = true;
    //         while (strm.good()) {
    //             std::string line;
    //             strm >> line;
    //             // maybe we're loading an edge-selection
    //             if (StringUtils::startsWith(line, "edge:")) {
    //                 std::string edgeID = line.substr(5);
    //                 MSEdge* edge = MSEdge::dictionary(edgeID);
    //                 if (edge != nullptr) {
    //                     myEdgeFilter.insert(edge);
    //                 } else {
    //                     WRITE_WARNING("Unknown edge ID '" + edgeID + "' in SSM device edge filter (" + file + "): " + line);
    //                 }
    //             } else if (StringUtils::startsWith(line, "junction:")) {
    //                 // get the internal edge(s) of a junction
    //                 std::string junctionID = line.substr(9);
    //                 MSJunction* junction = MSNet::getInstance()->getJunctionControl().get(junctionID);
    //                 if (junction != nullptr) {
    //                     for (MSLane* const internalLane : junction->getInternalLanes()) {
    //                         myEdgeFilter.insert(&(internalLane->getEdge()));
    //                     }
    //                 } else {
    //                     WRITE_WARNING("Unknown junction ID '" + junctionID + "' in SSM device edge filter (" + file + "): " + line);
    //                 }
    //             } else if (line == "") { // ignore empty lines (mostly last line)
    //             } else {
    //                 WRITE_WARNING("Cannot interpret line in SSM device edge filter (" + file + "): " + line);
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn init_edge_filter() {
        todo!("port MSDevice_SSM::initEdgeFilter")
    }

    /// C++: `void MSDevice_SSM::MSDevice_SSM(const MSDevice_SSM &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSDevice_SSM) -> Self {
        todo!("port MSDevice_SSM::MSDevice_SSM")
    }

    /// C++: `MSDevice_SSM & MSDevice_SSM::operator=(const MSDevice_SSM &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSDevice_SSM) -> &mut MSDevice_SSM {
        todo!("port MSDevice_SSM::operator=")
    }

}

/// A trajectory encloses a series of positions x and speeds v for one vehicle (the times are stored only once in the enclosing encounter)
/// C++ class `Encounter::Trajectory` (microsim/devices/MSDevice_SSM.h)
pub struct Encounter_Trajectory {
    pub x: PositionVector,
    pub lane: Vec<String>,
    pub lanePos: Vec<f64>,
    pub v: PositionVector,
}

impl Encounter_Trajectory {
}

/// Auxiliary structure used to handle upstream scanning start points Upstream scan has to be started after downstream scan is completed, see #5644
/// C++ class `MSDevice_SSM::UpstreamScanStartInfo` (microsim/devices/MSDevice_SSM.h)
pub struct MSDevice_SSM_UpstreamScanStartInfo {
    pub edge: Ptr<MSEdge>,
    pub pos: f64,
    pub range: f64,
    pub egoDistToConflictLane: f64,
    pub egoConflictLane: Ptr<MSLane>,
}

impl MSDevice_SSM_UpstreamScanStartInfo {
    /// C++: `void UpstreamScanStartInfo::UpstreamScanStartInfo(const MSEdge *, double, double, double, const MSLane *)`
    // UpstreamScanStartInfo(const MSEdge* edge, double pos, double range, double egoDistToConflictLane, const MSLane* egoConflictLane) :
    //             edge(edge), pos(pos), range(range), egoDistToConflictLane(egoDistToConflictLane), egoConflictLane(egoConflictLane) {}
    #[allow(unused_variables)]
    pub fn new(edge: Ptr<MSEdge>, pos: f64, range: f64, egoDistToConflictLane: f64, egoConflictLane: Ptr<MSLane>) -> Self {
        todo!("port UpstreamScanStartInfo::UpstreamScanStartInfo")
    }

}

/// Compares encounters regarding to their start time
/// C++ class `Encounter::compare` (microsim/devices/MSDevice_SSM.h)
pub struct Encounter_compare {
}

impl Encounter_compare {
    /// C++: `bool compare::operator()(Encounter *, Encounter *)`
    // bool operator()(Encounter* e1, Encounter* e2) {
    //                 if (e1->begin == e2->begin) {
    //                     return e1->foeID > e2->foeID;
    //                 } else {
    //                     return e1->begin > e2->begin;
    //                 }
    //             }
    #[allow(unused_variables)]
    pub fn call_op(&mut self, e1: Ptr<MSDevice_SSM_Encounter>, e2: Ptr<MSDevice_SSM_Encounter>) -> bool {
        todo!("port compare::operator()")
    }

}

/// C++: `std::ostream &(std::ostream &, MSDevice_SSM::EncounterType) operator<<`
// std::ostream& operator<<(std::ostream& out, MSDevice_SSM::EncounterType type) {
//     switch (type) {
//         case MSDevice_SSM::ENCOUNTER_TYPE_NOCONFLICT_AHEAD:
//             out << "NOCONFLICT_AHEAD";
//             break;
//         case MSDevice_SSM::ENCOUNTER_TYPE_FOLLOWING:
//             out << "FOLLOWING";
//             break;
//         case MSDevice_SSM::ENCOUNTER_TYPE_FOLLOWING_FOLLOWER:
//             out << "FOLLOWING_FOLLOWER";
//             break;
//         case MSDevice_SSM::ENCOUNTER_TYPE_FOLLOWING_LEADER:
//             out << "FOLLOWING_LEADER";
//             break;
//         case MSDevice_SSM::ENCOUNTER_TYPE_ON_ADJACENT_LANES:
//             out << "ON_ADJACENT_LANES";
//             break;
//         case MSDevice_SSM::ENCOUNTER_TYPE_MERGING:
//             out << "MERGING";
//             break;
//         case MSDevice_SSM::ENCOUNTER_TYPE_MERGING_LEADER:
//             out << "MERGING_LEADER";
//             break;
//         case MSDevice_SSM::ENCOUNTER_TYPE_MERGING_FOLLOWER:
//             out << "MERGING_FOLLOWER";
//             break;
//         case MSDevice_SSM::ENCOUNTER_TYPE_MERGING_ADJACENT:
//             out << "MERGING_ADJACENT";
//             break;
//         case MSDevice_SSM::ENCOUNTER_TYPE_CROSSING:
//             out << "CROSSING";
//             break;
//         case MSDevice_SSM::ENCOUNTER_TYPE_CROSSING_LEADER:
//             out << "CROSSING_LEADER";
//             break;
//         case MSDevice_SSM::ENCOUNTER_TYPE_CROSSING_FOLLOWER:
//             out << "CROSSING_FOLLOWER";
//             break;
//         case MSDevice_SSM::ENCOUNTER_TYPE_EGO_ENTERED_CONFLICT_AREA:
//             out << "EGO_ENTERED_CONFLICT_AREA";
//             break;
//         case MSDevice_SSM::ENCOUNTER_TYPE_FOE_ENTERED_CONFLICT_AREA:
//             out << "FOE_ENTERED_CONFLICT_AREA";
//             break;
//         case MSDevice_SSM::ENCOUNTER_TYPE_BOTH_ENTERED_CONFLICT_AREA:
//             out << "BOTH_ENTERED_CONFLICT_AREA";
//             break;
//         case MSDevice_SSM::ENCOUNTER_TYPE_EGO_LEFT_CONFLICT_AREA:
//             out << "EGO_LEFT_CONFLICT_AREA";
//             break;
//         case MSDevice_SSM::ENCOUNTER_TYPE_FOE_LEFT_CONFLICT_AREA:
//             out << "FOE_LEFT_CONFLICT_AREA";
//             break;
//         case MSDevice_SSM::ENCOUNTER_TYPE_BOTH_LEFT_CONFLICT_AREA:
//             out << "BOTH_LEFT_CONFLICT_AREA";
//             break;
//         case MSDevice_SSM::ENCOUNTER_TYPE_FOLLOWING_PASSED:
//             out << "FOLLOWING_PASSED";
//             break;
//         case MSDevice_SSM::ENCOUNTER_TYPE_MERGING_PASSED:
//             out << "MERGING_PASSED";
//             break;
//         // Collision (currently unused, might be differentiated further)
//         case MSDevice_SSM::ENCOUNTER_TYPE_COLLISION:
//             out << "COLLISION";
//             break;
//         case MSDevice_SSM::ENCOUNTER_TYPE_ONCOMING:
//             out << "ONCOMING";
//             break;
//         default:
//             out << "unknown type (" << int(type) << ")";
//             break;
//     }
//     return out;
// }
#[allow(unused_variables)]
pub fn shl_op(out: &mut Unmapped /* std::ostream */, type_: MSDevice_SSM_EncounterType) -> &'static mut Unmapped /* std::ostream */ {
    todo!("port operator<<")
}

