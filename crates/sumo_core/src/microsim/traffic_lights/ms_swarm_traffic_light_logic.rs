//! Port of microsim/traffic_lights/MSSwarmTrafficLightLogic.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use super::mssotl_traffic_light_logic::MS_SOTL_TRAFFIC_LIGHT_LOGIC_VT;

fn swarm_init(l: &mut MSTrafficLightLogic, nb: &mut NLDetectorBuilder) {
    Ptr::from_mut(l).cast::<MSSwarmTrafficLightLogic>().m().init(nb)
}
fn swarm_decide_next_phase(l: &mut MSTrafficLightLogic) -> i32 {
    Ptr::from_mut(l).cast::<MSSwarmTrafficLightLogic>().m().decide_next_phase()
}
fn swarm_can_release(l: &mut MSTrafficLightLogic) -> bool {
    Ptr::from_mut(l).cast::<MSSwarmTrafficLightLogic>().m().can_release()
}
fn swarm_compute_return_time(l: &mut MSTrafficLightLogic) -> SUMOTime {
    Ptr::from_mut(l).cast::<MSSwarmTrafficLightLogic>().m().compute_return_time()
}

static MS_SWARM_TRAFFIC_LIGHT_LOGIC_VT: MSTrafficLightLogicVt = MSTrafficLightLogicVt {
    init: swarm_init,
    decide_next_phase: swarm_decide_next_phase,
    can_release: swarm_can_release,
    compute_return_time: swarm_compute_return_time,
    ..MS_SOTL_TRAFFIC_LIGHT_LOGIC_VT
};

/// Leaks a heap object for the whole simulation, like the C++ `new` the constructor uses.
fn leak<T>(value: T) -> &'static mut T {
    Box::leak(Box::new(value))
}

fn min2(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}

fn max2(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}

/// C++ `beta * oldPhero + gamma * pheroAdd * updatePheromone` (bool as 0/1)
fn beta_gamma_phero(beta: f64, oldPhero: f64, gamma: f64, pheroAdd: f64, updatePheromone: bool) -> f64 {
    beta * oldPhero + gamma * pheroAdd * (updatePheromone as i32 as f64)
}

/// Returns (max value, average of the other values) as computed by the C++ getDistanceOfMaxPhero* methods
fn distance_of_max_phero(map: &MSLaneId_PheromoneMap) -> (f64, f64) {
    let mut max_phero_val_current: f64 = 0.0;
    let mut max_phero_val_old: f64 = 0.0;
    let mut temp_avg_other_lanes: f64 = 0.0;
    let mut counter: i32 = 0;
    for &lanePhero in map.values() {
        if counter == 0 {
            max_phero_val_current = lanePhero;
            counter += 1;
            continue;
        }
        if lanePhero > max_phero_val_current {
            max_phero_val_old = max_phero_val_current;
            max_phero_val_current = lanePhero;
            temp_avg_other_lanes = (temp_avg_other_lanes * (counter - 1) as f64 + max_phero_val_old) / counter as f64;
        } else {
            temp_avg_other_lanes = (temp_avg_other_lanes * (counter - 1) as f64 + lanePhero) / counter as f64;
        }
        counter += 1;
    }
    (max_phero_val_current, temp_avg_other_lanes)
}

/// Owned copy of a phase. The generated countVehicles takes the phase by value, the C++ takes const&.
// shortcut: copy only because MSSOTLTrafficLightLogic::count_vehicles takes MSPhaseDefinition by value, change it to &MSPhaseDefinition and drop this copy
fn phase_copy(p: &MSPhaseDefinition) -> MSPhaseDefinition {
    MSPhaseDefinition {
        duration: p.duration,
        lastDuration: p.lastDuration,
        minDuration: p.minDuration,
        maxDuration: p.maxDuration,
        earliestEnd: p.earliestEnd,
        latestEnd: p.latestEnd,
        earlyTarget: p.earlyTarget.clone(),
        finalTarget: p.finalTarget.clone(),
        myLastSwitch: p.myLastSwitch,
        myLastEnd: p.myLastEnd,
        nextPhases: p.nextPhases.clone(),
        name: p.name.clone(),
        yellow: p.yellow,
        red: p.red,
        vehext: p.vehext,
        myTransientNotDecisional: p.myTransientNotDecisional,
        myCommit: p.myCommit,
        myUndefined: p.myUndefined,
        myTargetLaneSet: p.myTargetLaneSet.clone(),
        myState: p.myState.clone(),
    }
}

/// C++ class `CircularBuffer` (microsim/traffic_lights/MSSwarmTrafficLightLogic.h)
pub struct CircularBuffer<T> {
    pub m_buffer: Ptr<T>,
    pub m_size: i32,
    pub m_currentIndex: i32,
    pub m_firstTime: bool,
    _p: std::marker::PhantomData<(T,)>,
}

impl<T> CircularBuffer<T> {
    /// C++: `void CircularBuffer::CircularBuffer<T>(int)`
    // CircularBuffer(int size) :
    //         m_size(size), m_currentIndex(0), m_firstTime(true) {
    //         m_buffer = new T[m_size];
    //     }
    #[allow(unused_variables)]
    pub fn new(size: i32) -> Self {
        let layout = std::alloc::Layout::array::<T>(size as usize).unwrap();
        // C++ `new T[m_size]`: uninitialised storage, the elements are plain values (f64 in practice)
        let buffer = if layout.size() == 0 {
            std::ptr::NonNull::<T>::dangling().as_ptr()
        } else {
            unsafe { std::alloc::alloc(layout) as *mut T }
        };
        Self {
            m_buffer: Ptr::from_raw(buffer),
            m_size: size,
            m_currentIndex: 0,
            m_firstTime: true,
            _p: std::marker::PhantomData,
        }
    }

    /// C++: `void CircularBuffer::~CircularBuffer<T>()` [virtual]
    // virtual ~CircularBuffer() {
    //         delete[] m_buffer;
    //     }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ `delete[] m_buffer`
        if std::mem::size_of::<T>() != 0 && self.m_size > 0 {
            let layout = std::alloc::Layout::array::<T>(self.m_size as usize).unwrap();
            unsafe { std::alloc::dealloc(self.m_buffer.as_raw() as *mut u8, layout) };
        }
    }

    /// C++: `bool CircularBuffer::addValue(const T, T &)`
    // bool addValue(const T newValue, T& replacedValue) {
    //         bool result = !m_firstTime;
    //         if (result) {
    //             replacedValue = m_buffer[m_currentIndex];
    //         }
    //         insert(newValue);
    //         return result;
    //     }
    #[allow(unused_variables)]
    pub fn add_value(&mut self, newValue: T, replacedValue: &mut T) -> bool {
        let result = !self.m_firstTime;
        if result {
            unsafe { std::ptr::copy_nonoverlapping(self.m_buffer.as_raw().add(self.m_currentIndex as usize), replacedValue as *mut T, 1) };
        }
        self.insert(&newValue);
        result
    }

    /// C++: `void CircularBuffer::push_front(const T)`
    // void push_front(const T value) {
    //         insert(value);
    //     }
    #[allow(unused_variables)]
    pub fn push_front(&mut self, value: T) {
        self.insert(&value);
    }

    /// C++: `T CircularBuffer::at(const int) const` [const]
    // T at(const int index) const {
    //         int idx = (m_currentIndex - 1 - index + m_size) % m_size;
    //         return m_buffer[idx];
    //     }
    #[allow(unused_variables)]
    pub fn at(&self, index: i32) -> T {
        let idx = (self.m_currentIndex - 1 - index + self.m_size) % self.m_size;
        unsafe { std::ptr::read(self.m_buffer.as_raw().add(idx as usize)) }
    }

    /// C++: `T CircularBuffer::front() const` [const]
    // T front() const {
    //         return at(0);
    //     }
    #[allow(unused_variables)]
    pub fn front(&self) -> T {
        self.at(0)
    }

    /// C++: `T CircularBuffer::back() const` [const]
    // T back() const {
    //         return at(size() - 1);
    //     }
    #[allow(unused_variables)]
    pub fn back(&self) -> T {
        self.at(self.size() - 1)
    }

    /// C++: `int CircularBuffer::size() const` [const]
    // int size() const {
    //         if (m_firstTime) {
    //             return m_currentIndex;
    //         }
    //         return m_size;
    //     }
    #[allow(unused_variables)]
    pub fn size(&self) -> i32 {
        if self.m_firstTime {
            return self.m_currentIndex;
        }
        self.m_size
    }

    /// C++: `void CircularBuffer::clear()`
    // void clear() {
    //         m_currentIndex = 0;
    //         m_firstTime = true;
    //     }
    #[allow(unused_variables)]
    pub fn clear(&mut self) {
        self.m_currentIndex = 0;
        self.m_firstTime = true;
    }

    /// C++: `void CircularBuffer::insert(const T &)` [private]
    // inline void insert(const T& value) {
    //         m_buffer[m_currentIndex++] = value;
    //         if (m_currentIndex == m_size) {
    //             m_currentIndex = 0;
    //             m_firstTime = false;
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn insert(&mut self, value: &T) {
        // m_buffer[m_currentIndex++] = value;
        unsafe { std::ptr::copy_nonoverlapping(value as *const T, self.m_buffer.as_raw().add(self.m_currentIndex as usize), 1) };
        self.m_currentIndex += 1;
        if self.m_currentIndex == self.m_size {
            self.m_currentIndex = 0;
            self.m_firstTime = false;
        }
    }

}

/// C++ class `MSSwarmTrafficLightLogic` (microsim/traffic_lights/MSSwarmTrafficLightLogic.h)
#[repr(C)]
pub struct MSSwarmTrafficLightLogic {
    /// C++ base class `MSSOTLHiLevelTrafficLightLogic`
    pub base: MSSOTLHiLevelTrafficLightLogic,
    /// This pheronome is an indicator of congestion on input lanes. Its levels refer to the average speed of vehicles passing the input lane: the lower the speed the higher the pheromone. These levels are updated on every input lane, independently on lights state.
    pub pheromoneInputLanes: MSLaneId_PheromoneMap,
    /// This pheromone is an indicator of congestion on output lanes. Its levels refer to the average speed of vehicles passing the output lane: the lower the speed the higher the pheromone. These levels are updated on every output lane, independently on lights state.
    pub pheromoneOutputLanes: MSLaneId_PheromoneMap,
    /// This member keeps track of the last thresholds update, s.t. updates can be correctly performed even on time-variable interations.
    pub lastThetaSensitivityUpdate: SUMOTime,
    pub logData: bool,
    pub swarmLogFile: Option<std::fs::File> /* std::ofstream */,
    /// When true, indicates that the current policy MUST be changed. It's used to force the exit from the congestion policy
    pub mustChange: bool,
    pub congestion_steps: SUMOTime,
    /// Map to check if a lane was already controlled during the elaboration of eta.
    pub laneCheck: LaneCheckMap,
    /// A copy of the target lanes of this phase.
    pub targetLanes: LaneIdVector,
    /// When true indicates that we can skip the evaluation of eta since we've a congestion policy that is lasting too much.
    pub skipEta: bool,
    /// When true indicates that we've already acquired the target lanes for this particular phase.
    pub gotTargetLane: bool,
    pub carsIn: i32,
    pub carsOut: i32,
    pub inTarget: i32,
    pub notTarget: i32,
    /// to scale pheromoneDispersion in range [0, 10]
    pub scaleFactorDispersionIn: f64,
    pub scaleFactorDispersionOut: f64,
    pub m_laneIndexMap: BTreeMap<String, Vec<i32>>,
    pub m_pheroLevelLog: BTreeMap<String, String>,
    pub m_meanSpeedHistory: BTreeMap<String, Ptr<CircularBuffer<f64>>>,
    pub m_derivativeHistory: BTreeMap<String, Ptr<CircularBuffer<f64>>>,
    pub m_derivativeAlpha: f64,
    pub m_losCounter: i32,
    pub m_losMaxLimit: i32,
    pub m_useVehicleTypesWeights: bool,
}

impl std::ops::Deref for MSSwarmTrafficLightLogic {
    type Target = MSSOTLHiLevelTrafficLightLogic;
    fn deref(&self) -> &MSSOTLHiLevelTrafficLightLogic {
        &self.base
    }
}
impl std::ops::DerefMut for MSSwarmTrafficLightLogic {
    fn deref_mut(&mut self) -> &mut MSSOTLHiLevelTrafficLightLogic {
        &mut self.base
    }
}

impl MSSwarmTrafficLightLogic {
    /// C++: `void MSSwarmTrafficLightLogic::MSSwarmTrafficLightLogic(MSTLLogicControl &, const std::string &, const std::string &, const Phases &, int, SUMOTime, const Parameterised::Map &)`
    /// Constructor without sensors passed
    // MSSwarmTrafficLightLogic::MSSwarmTrafficLightLogic(MSTLLogicControl& tlcontrol, const std::string& id,
    //         const std::string& programID, const Phases& phases, int step, SUMOTime delay,
    //         const Parameterised::Map& parameters) :
    //     MSSOTLHiLevelTrafficLightLogic(tlcontrol, id, programID, TrafficLightType::SWARM_BASED, phases, step, delay, parameters) {
    //
    //     std::string pols = getPoliciesParam();
    //     pols = StringUtils::to_lower_case(pols);
    // #ifdef SWARM_DEBUG
    //     std::ostringstream str;
    //     str << "policies: " << pols;
    //     WRITE_MESSAGE(str.str());
    // #endif
    //
    //     if (pols.find("platoon") != std::string::npos) {
    //         addPolicy(new MSSOTLPlatoonPolicy(new MSSOTLPolicy5DFamilyStimulus("PLATOON", parameters), parameters));
    //     }
    //     if (pols.find("phase") != std::string::npos) {
    //         addPolicy(new MSSOTLPhasePolicy(new MSSOTLPolicy5DFamilyStimulus("PHASE", parameters), parameters));
    //     }
    //     if (pols.find("marching") != std::string::npos) {
    //         addPolicy(new MSSOTLMarchingPolicy(new MSSOTLPolicy5DFamilyStimulus("MARCHING", parameters), parameters));
    //     }
    //     if (pols.find("congestion") != std::string::npos) {
    //         addPolicy(new MSSOTLCongestionPolicy(new MSSOTLPolicy5DFamilyStimulus("CONGESTION", parameters), parameters));
    //     }
    //
    //     if (myPolicies.empty()) {
    //         WRITE_ERROR(TL("NO VALID POLICY LIST READ"));
    //     }
    //
    //     mustChange = false;
    //     skipEta = false;
    //     gotTargetLane = false;
    //
    // #ifdef SWARM_DEBUG
    //     std::ostringstream d_str;
    //     d_str << getMaxCongestionDuration();
    //     vector<MSSOTLPolicy*> policies = myPolicies;
    //
    //     WRITE_MESSAGE("getMaxCongestionDuration " + d_str.str());
    //     for (int i = 0; i < policies.size(); i++) {
    //         MSSOTLPolicy* policy = policies[i];
    //         MSSOTLPolicyDesirability* stim = policy->getDesirabilityAlgorithm();
    //         std::ostringstream _str;
    //         _str << policy->getName() << stim->getMessage() << " getThetaSensitivity " << policy->getThetaSensitivity() << " .";
    //         WRITE_MESSAGE(_str.str());
    //     }
    // #endif
    //     congestion_steps = 0;
    //     m_useVehicleTypesWeights = getParameter("USE_VEHICLE_TYPES_WEIGHTS", "0") == "1";
    //     if (m_useVehicleTypesWeights && pols.find("phase") == std::string::npos) {
    //         WRITE_ERROR(TL("VEHICLE TYPES WEIGHT only works with phase policy, which is missing"));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(tlcontrol: &mut MSTLLogicControl, id: &str, programID: &str, phases: &MSTrafficLightLogic_Phases, step: i32, delay: SUMOTime, parameters: &ParameterisedMap) -> Self {
        let mut hi = MSSOTLHiLevelTrafficLightLogic::new(tlcontrol, id, programID, TrafficLightType::SWARM_BASED, phases, step, delay, parameters);
        hi.base.base.base.vt = &MS_SWARM_TRAFFIC_LIGHT_LOGIC_VT;
        let mut this = Self {
            base: hi,
            pheromoneInputLanes: BTreeMap::new(),
            pheromoneOutputLanes: BTreeMap::new(),
            lastThetaSensitivityUpdate: 0,
            logData: false,
            swarmLogFile: None,
            mustChange: false,
            congestion_steps: 0,
            laneCheck: BTreeMap::new(),
            targetLanes: Vec::new(),
            skipEta: false,
            gotTargetLane: false,
            carsIn: 0,
            carsOut: 0,
            inTarget: 0,
            notTarget: 0,
            scaleFactorDispersionIn: 0.0,
            scaleFactorDispersionOut: 0.0,
            m_laneIndexMap: BTreeMap::new(),
            m_pheroLevelLog: BTreeMap::new(),
            m_meanSpeedHistory: BTreeMap::new(),
            m_derivativeHistory: BTreeMap::new(),
            m_derivativeAlpha: 0.0,
            m_losCounter: 0,
            m_losMaxLimit: 0,
            m_useVehicleTypesWeights: false,
        };

        let pols = utils::common::string_utils::to_lower_case(&this.get_policies_param());
        // the policies are owned for the whole simulation, as the C++ `new` calls leak them
        if pols.contains("platoon") {
            let stimulus = leak(MSSOTLPolicy5DFamilyStimulus::new("PLATOON".to_string(), parameters));
            let policy = leak(MSSOTLPlatoonPolicy::new_3(Ptr::from_mut(&mut stimulus.base), parameters));
            this.add_policy(Ptr::from_mut(&mut policy.base));
        }
        if pols.contains("phase") {
            let stimulus = leak(MSSOTLPolicy5DFamilyStimulus::new("PHASE".to_string(), parameters));
            let policy = leak(MSSOTLPhasePolicy::new_3(Ptr::from_mut(&mut stimulus.base), parameters));
            this.add_policy(Ptr::from_mut(&mut policy.base));
        }
        if pols.contains("marching") {
            let stimulus = leak(MSSOTLPolicy5DFamilyStimulus::new("MARCHING".to_string(), parameters));
            let policy = leak(MSSOTLMarchingPolicy::new_3(Ptr::from_mut(&mut stimulus.base), parameters));
            this.add_policy(Ptr::from_mut(&mut policy.base));
        }
        if pols.contains("congestion") {
            let stimulus = leak(MSSOTLPolicy5DFamilyStimulus::new("CONGESTION".to_string(), parameters));
            let policy = leak(MSSOTLCongestionPolicy::new_3(Ptr::from_mut(&mut stimulus.base), parameters));
            this.add_policy(Ptr::from_mut(&mut policy.base));
        }

        if this.myPolicies.is_empty() {
            utils::common::msg_handler::write_error("NO VALID POLICY LIST READ");
        }

        this.mustChange = false;
        this.skipEta = false;
        this.gotTargetLane = false;

        this.congestion_steps = 0;
        this.m_useVehicleTypesWeights = this.get_parameter("USE_VEHICLE_TYPES_WEIGHTS", "0".to_string()) == "1";
        if this.m_useVehicleTypesWeights && !pols.contains("phase") {
            utils::common::msg_handler::write_error("VEHICLE TYPES WEIGHT only works with phase policy, which is missing");
        }
        this
    }

    /// C++: `void MSSwarmTrafficLightLogic::~MSSwarmTrafficLightLogic() throw()` [virtual]
    // MSSwarmTrafficLightLogic::~MSSwarmTrafficLightLogic() {
    //     if (logData && swarmLogFile.is_open()) {
    //         swarmLogFile.close();
    //     }
    //     for (std::map<std::string, CircularBuffer<double>*>::iterator it = m_meanSpeedHistory.begin();
    //             it != m_meanSpeedHistory.end(); ++it) {
    //         delete it->second;
    //     }
    //     m_meanSpeedHistory.clear();
    //     for (std::map<std::string, CircularBuffer<double>*>::iterator it = m_derivativeHistory.begin();
    //             it != m_derivativeHistory.end(); ++it) {
    //         delete it->second;
    //     }
    //     m_derivativeHistory.clear();
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // the log file is closed when swarmLogFile is dropped
        self.swarmLogFile = None;
        for buffer in self.m_meanSpeedHistory.values() {
            buffer.m().drop_();
            unsafe { drop(Box::from_raw(buffer.as_raw())) };
        }
        self.m_meanSpeedHistory.clear();
        for buffer in self.m_derivativeHistory.values() {
            buffer.m().drop_();
            unsafe { drop(Box::from_raw(buffer.as_raw())) };
        }
        self.m_derivativeHistory.clear();
    }

    /// C++: `void MSSwarmTrafficLightLogic::init(NLDetectorBuilder &)` [virtual]
    /// Initialises the tls with sensors on incoming and outgoing lanes Sensors are built in the simulation according to the type of sensor specified in the simulation parameter
    // void MSSwarmTrafficLightLogic::init(NLDetectorBuilder& nb) {
    //     MSSOTLHiLevelTrafficLightLogic::init(nb);
    //     //Setting the startup policy
    //     choosePolicy(0, 0, 0, 0);
    //     //Initializing the random number generator to a time-dependent seed
    //     srand((int) time(nullptr));
    //     //Initializing pheromone maps according to input lanes
    //     //For each lane insert a pair into maps
    //     MSLane* currentLane = nullptr;
    //
    // //	Derivative
    //     const int derivativeHistorySize = StringUtils::toInt(getParameter("PHERO_DERIVATIVE_HISTORY_SIZE", "3"));
    //     const int meanSpeedHistorySize = StringUtils::toInt(getParameter("PHERO_MEAN_SPEED_HISTORY_SIZE", "3"));
    //     m_derivativeAlpha = StringUtils::toDouble(getParameter("PHERO_DERIVATIVE_ALPHA", "1"));
    //     m_losCounter = 0;
    //     m_losMaxLimit = StringUtils::toInt(getParameter("LOSS_OF_SIGNAL_LIMIT", "10"));
    //
    //     int index = 0;
    //     for (MSTrafficLightLogic::LaneVectorVector::const_iterator laneVector = myLanes.begin();
    //             laneVector != myLanes.end(); laneVector++) {
    //         for (MSTrafficLightLogic::LaneVector::const_iterator lane = laneVector->begin(); lane != laneVector->end();
    //                 lane++) {
    //             currentLane = (*lane);
    //             if (pheromoneInputLanes.find(currentLane->getID()) == pheromoneInputLanes.end()) {
    //                 laneCheck[currentLane] = false;
    //                 if (allowLine(currentLane)) {
    //                     pheromoneInputLanes.insert(MSLaneId_Pheromone(currentLane->getID(), 0.0));
    // //					Consider the derivative only for the input lane
    //                     m_meanSpeedHistory.insert(std::make_pair(currentLane->getID(), new CircularBuffer<double>(meanSpeedHistorySize)));
    //                     m_derivativeHistory.insert(std::make_pair(currentLane->getID(), new CircularBuffer<double>(derivativeHistorySize)));
    // #ifdef ANALYSIS_DEBUG
    //                     WRITE_MESSAGE("MSSwarmTrafficLightLogic::init Intersection " + getID() + " pheromoneInputLanes adding " + currentLane->getID());
    // #endif
    //                 } else {
    // #ifdef ANALYSIS_DEBUG
    //                     WRITE_MESSAGEF(TL("MSSwarmTrafficLightLogic::init Intersection % pheromoneInputLanes: lane % not allowed"), getID(), currentLane->getID());
    // #endif
    //                 }
    //             }
    //             m_laneIndexMap[currentLane->getID()].push_back(index++);
    //         }
    //     }
    //
    //     LinkVectorVector links = getLinks();
    //     for (int i = 0; i < (int)links.size(); i++) {
    //         LinkVector oneLink = getLinksAt(i);
    //         for (int j = 0; j < (int)oneLink.size(); j++) {
    //             currentLane = oneLink[j]->getLane();
    //             if (pheromoneOutputLanes.find(currentLane->getID()) == pheromoneOutputLanes.end()) {
    //                 laneCheck[currentLane] = false;
    //                 if (allowLine(currentLane)) {
    //                     pheromoneOutputLanes.insert(MSLaneId_Pheromone(currentLane->getID(), 0.0));
    // #ifdef ANALYSIS_DEBUG
    //                     WRITE_MESSAGE("MSSwarmTrafficLightLogic::init Intersection " + getID() + " pheromoneOutputLanes adding " + currentLane->getID());
    // #endif
    //                 } else {
    // #ifdef ANALYSIS_DEBUG
    //                     WRITE_MESSAGEF(TL("MSSwarmTrafficLightLogic::init Intersection % pheromoneOutputLanes lane % not allowed"), getID(), currentLane->getID());
    // #endif
    //                 }
    //             }
    //         }
    //     }
    //
    //     initScaleFactorDispersionIn((int)pheromoneInputLanes.size());
    //     initScaleFactorDispersionOut((int)pheromoneOutputLanes.size());
    //     //Initializing thresholds for theta evaluations
    //     lastThetaSensitivityUpdate = MSNet::getInstance()->getCurrentTimeStep();
    //
    //     WRITE_MESSAGEF(TL("*** Intersection % will run using MSSwarmTrafficLightLogic ***"), getID());
    //     std::string logFileName = getParameter("SWARMLOG", "");
    //     logData = logFileName.compare("") != 0;
    //     if (logData) {
    //         swarmLogFile.open(logFileName.c_str(), std::ios::out | std::ios::binary);
    //     }
    // //	Log the initial state
    // #ifdef ANALYSIS_DEBUG
    //     WRITE_MESSAGEF(TL("TL % time 0 Policy: % (pheroIn= 0 ,pheroOut= 0 ) OldPolicy: % ."), getID(), myCurrentPolicy->getName(), myCurrentPolicy->getName());
    // //	ostringstream maplog;
    // //	for(map<string, vector<int> >::const_iterator mIt = m_laneIndexMap.begin();mIt != m_laneIndexMap.end();++mIt)
    // //	{
    // //		maplog << mIt->first <<'[';
    // //		for(vector<int>::const_iterator vIt = mIt->second.begin();vIt != mIt->second.end();++vIt)
    // //			maplog<<*vIt<<", ";
    // //		maplog << "] ";
    // //	}
    // //	WRITE_MESSAGE("Map content " + maplog.str());
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn init(&mut self, nb: &mut NLDetectorBuilder) {
        self.base.init(nb);
        // Setting the startup policy
        self.choose_policy(0.0, 0.0, 0.0, 0.0);
        // Initializing the random number generator to a time-dependent seed
        utils::common::rand_helper::init_rand(None, true, 0);
        // Initializing pheromone maps according to input lanes
        // For each lane insert a pair into maps

        // Derivative
        let derivativeHistorySize = utils::common::string_utils::to_int(&self.get_parameter("PHERO_DERIVATIVE_HISTORY_SIZE", "3".to_string())).unwrap();
        let meanSpeedHistorySize = utils::common::string_utils::to_int(&self.get_parameter("PHERO_MEAN_SPEED_HISTORY_SIZE", "3".to_string())).unwrap();
        self.m_derivativeAlpha = utils::common::string_utils::to_double(&self.get_parameter("PHERO_DERIVATIVE_ALPHA", "1".to_string())).unwrap();
        self.m_losCounter = 0;
        self.m_losMaxLimit = utils::common::string_utils::to_int(&self.get_parameter("LOSS_OF_SIGNAL_LIMIT", "10".to_string())).unwrap();

        let mut index: i32 = 0;
        let myLanes = self.myLanes.clone();
        for laneVector in myLanes.iter() {
            for &currentLane in laneVector.iter() {
                if !self.pheromoneInputLanes.contains_key(currentLane.get_id()) {
                    self.laneCheck.insert(currentLane, false);
                    if self.allow_line(currentLane) {
                        self.pheromoneInputLanes.insert(currentLane.get_id().to_string(), 0.0);
                        // Consider the derivative only for the input lane
                        let meanBuffer = Ptr::from_raw(Box::into_raw(Box::new(CircularBuffer::<f64>::new(meanSpeedHistorySize))));
                        self.m_meanSpeedHistory.insert(currentLane.get_id().to_string(), meanBuffer);
                        let derivativeBuffer = Ptr::from_raw(Box::into_raw(Box::new(CircularBuffer::<f64>::new(derivativeHistorySize))));
                        self.m_derivativeHistory.insert(currentLane.get_id().to_string(), derivativeBuffer);
                    }
                }
                self.m_laneIndexMap.entry(currentLane.get_id().to_string()).or_default().push(index);
                index += 1;
            }
        }

        let links = self.myLinks.clone();
        for i in 0..links.len() {
            let oneLink = links[i].clone();
            for j in 0..oneLink.len() {
                let currentLane = oneLink[j].get_lane();
                if !self.pheromoneOutputLanes.contains_key(currentLane.get_id()) {
                    self.laneCheck.insert(currentLane, false);
                    if self.allow_line(currentLane) {
                        self.pheromoneOutputLanes.insert(currentLane.get_id().to_string(), 0.0);
                    }
                }
            }
        }

        let lanesIn = self.pheromoneInputLanes.len() as i32;
        self.init_scale_factor_dispersion_in(lanesIn);
        let lanesOut = self.pheromoneOutputLanes.len() as i32;
        self.init_scale_factor_dispersion_out(lanesOut);
        // Initializing thresholds for theta evaluations
        self.lastThetaSensitivityUpdate = MSNet::get_instance().get_current_time_step();

        utils::common::msg_handler::write_message(&format!("*** Intersection {} will run using MSSwarmTrafficLightLogic ***", self.get_id()));
        let logFileName = self.get_parameter("SWARMLOG", String::new());
        self.logData = logFileName != "";
        if self.logData {
            self.swarmLogFile = std::fs::File::create(&logFileName).ok();
        }
    }

    /// C++: `SUMOTime MSSwarmTrafficLightLogic::getMaxCongestionDuration()`
    // SUMOTime getMaxCongestionDuration() {
    //         return StringUtils::toInt(getParameter("MAX_CONGESTION_DUR", "120"));
    //     }
    #[allow(unused_variables)]
    pub fn get_max_congestion_duration(&mut self) -> SUMOTime {
        utils::common::string_utils::to_int(&self.get_parameter("MAX_CONGESTION_DUR", "120".to_string())).unwrap() as SUMOTime
    }

    /// C++: `double MSSwarmTrafficLightLogic::getPheroMaxVal()`
    // double getPheroMaxVal() {
    //         return StringUtils::toDouble(getParameter("PHERO_MAXVAL", "10"));
    //     }
    #[allow(unused_variables)]
    pub fn get_phero_max_val(&mut self) -> f64 {
        utils::common::string_utils::to_double(&self.get_parameter("PHERO_MAXVAL", "10".to_string())).unwrap()
    }

    /// C++: `double MSSwarmTrafficLightLogic::getBetaNo()`
    // double getBetaNo() {
    //         return StringUtils::toDouble(getParameter("BETA_NO", "0.99"));
    //     }
    #[allow(unused_variables)]
    pub fn get_beta_no(&mut self) -> f64 {
        utils::common::string_utils::to_double(&self.get_parameter("BETA_NO", "0.99".to_string())).unwrap()
    }

    /// C++: `double MSSwarmTrafficLightLogic::getGammaNo()`
    // double getGammaNo() {
    //         return StringUtils::toDouble(getParameter("GAMMA_NO", "1.0"));
    //     }
    #[allow(unused_variables)]
    pub fn get_gamma_no(&mut self) -> f64 {
        utils::common::string_utils::to_double(&self.get_parameter("GAMMA_NO", "1.0".to_string())).unwrap()
    }

    /// C++: `double MSSwarmTrafficLightLogic::getBetaSp()`
    // double getBetaSp() {
    //         return StringUtils::toDouble(getParameter("BETA_SP", "0.99"));
    //     }
    #[allow(unused_variables)]
    pub fn get_beta_sp(&mut self) -> f64 {
        utils::common::string_utils::to_double(&self.get_parameter("BETA_SP", "0.99".to_string())).unwrap()
    }

    /// C++: `double MSSwarmTrafficLightLogic::getGammaSp()`
    // double getGammaSp() {
    //         return StringUtils::toDouble(getParameter("GAMMA_SP", "1.0"));
    //     }
    #[allow(unused_variables)]
    pub fn get_gamma_sp(&mut self) -> f64 {
        utils::common::string_utils::to_double(&self.get_parameter("GAMMA_SP", "1.0".to_string())).unwrap()
    }

    /// C++: `double MSSwarmTrafficLightLogic::getChangePlanProbability()`
    // double getChangePlanProbability() {
    //         return StringUtils::toDouble(getParameter("CHANGE_PLAN_PROBABILITY", "0.003"));
    //     }
    #[allow(unused_variables)]
    pub fn get_change_plan_probability(&mut self) -> f64 {
        utils::common::string_utils::to_double(&self.get_parameter("CHANGE_PLAN_PROBABILITY", "0.003".to_string())).unwrap()
    }

    /// C++: `double MSSwarmTrafficLightLogic::getThetaMax()`
    // double getThetaMax() {
    //         return StringUtils::toDouble(getParameter("THETA_MAX", "0.8"));
    //     }
    #[allow(unused_variables)]
    pub fn get_theta_max(&mut self) -> f64 {
        utils::common::string_utils::to_double(&self.get_parameter("THETA_MAX", "0.8".to_string())).unwrap()
    }

    /// C++: `double MSSwarmTrafficLightLogic::getThetaMin()`
    // double getThetaMin() {
    //         return StringUtils::toDouble(getParameter("THETA_MIN", "0.2"));
    //     }
    #[allow(unused_variables)]
    pub fn get_theta_min(&mut self) -> f64 {
        utils::common::string_utils::to_double(&self.get_parameter("THETA_MIN", "0.2".to_string())).unwrap()
    }

    /// C++: `double MSSwarmTrafficLightLogic::getThetaInit()`
    // double getThetaInit() {
    //         return StringUtils::toDouble(getParameter("THETA_INIT", "0.5"));
    //     }
    #[allow(unused_variables)]
    pub fn get_theta_init(&mut self) -> f64 {
        utils::common::string_utils::to_double(&self.get_parameter("THETA_INIT", "0.5".to_string())).unwrap()
    }

    /// C++: `double MSSwarmTrafficLightLogic::getLearningCox()`
    // double getLearningCox() {
    //         return StringUtils::toDouble(getParameter("LEARNING_COX", "0.0005"));
    //     }
    #[allow(unused_variables)]
    pub fn get_learning_cox(&mut self) -> f64 {
        utils::common::string_utils::to_double(&self.get_parameter("LEARNING_COX", "0.0005".to_string())).unwrap()
    }

    /// C++: `double MSSwarmTrafficLightLogic::getForgettingCox()`
    // double getForgettingCox() {
    //         return StringUtils::toDouble(getParameter("FORGETTING_COX", "0.0005"));
    //     }
    #[allow(unused_variables)]
    pub fn get_forgetting_cox(&mut self) -> f64 {
        utils::common::string_utils::to_double(&self.get_parameter("FORGETTING_COX", "0.0005".to_string())).unwrap()
    }

    /// C++: `double MSSwarmTrafficLightLogic::getScaleFactorDispersionIn()`
    // double getScaleFactorDispersionIn() {
    //         return scaleFactorDispersionIn;
    //     }
    #[allow(unused_variables)]
    pub fn get_scale_factor_dispersion_in(&mut self) -> f64 {
        self.scaleFactorDispersionIn
    }

    /// C++: `double MSSwarmTrafficLightLogic::getScaleFactorDispersionOut()`
    // double getScaleFactorDispersionOut() {
    //         return scaleFactorDispersionOut;
    //     }
    #[allow(unused_variables)]
    pub fn get_scale_factor_dispersion_out(&mut self) -> f64 {
        self.scaleFactorDispersionOut
    }

    /// C++: `const std::string MSSwarmTrafficLightLogic::getLogicType() const` [const]
    /// Returns the type of the logic as a string
    // const std::string getLogicType() const {
    //         return "swarmBasedTrafficLogic";
    //     }
    #[allow(unused_variables)]
    pub fn get_logic_type(&self) -> String {
        "swarmBasedTrafficLogic".to_string()
    }

    /// C++: `int MSSwarmTrafficLightLogic::decideNextPhase()` [virtual, protected]
    // int MSSwarmTrafficLightLogic::decideNextPhase() {
    //
    // #ifdef SWARM_DEBUG
    //     MsgHandler::getMessageInstance()->inform("\n" + time2string(MSNet::getInstance()->getCurrentTimeStep()) + " MSSwarmTrafficLightLogic decideNextPhase()");
    //     std::ostringstream dnp;
    //     dnp << (MSNet::getInstance()->getCurrentTimeStep()) << " MSSwarmTrafficLightLogic::decideNextPhase:: " << "tlsid=" << getID() << " getCurrentPhaseDef().getState()=" << getCurrentPhaseDef().getState() << " is commit?" << getCurrentPhaseDef().isCommit();
    //     MsgHandler::getMessageInstance()->inform(dnp.str());
    // #endif
    //     // if we're congested, it should be wise to reset and recalculate the pheromone levels after X steps
    //
    //     if (getCurrentPhaseDef().isTarget()) {
    //         targetLanes = getCurrentPhaseDef().getTargetLaneSet();
    //     }
    //
    //     if (myCurrentPolicy->getName().compare("Congestion") == 0 && getCurrentPhaseDef().isCommit()) {
    //         congestion_steps += 1;	//STEPS2TIME(getCurrentPhaseDef().duration);
    // #ifdef SWARM_DEBUG
    //         WRITE_MESSAGEF("\n% MSSwarmTrafficLightLogic decideNextPhase()", time2string(MSNet::getInstance()->getCurrentTimeStep()));
    // std:
    //         ostringstream dnp;
    //         dnp << (MSNet::getInstance()->getCurrentTimeStep()) << " MSSwarmTrafficLightLogic::decideNextPhase:: " << "tlsid=" << getID() << " congestion_steps=" << congestion_steps;
    //         WRITE_MESSAGE(dnp.str());
    // #endif
    //         if (congestion_steps >= getMaxCongestionDuration()) {
    //             resetPheromone();
    //             congestion_steps = 0;
    //             mustChange = true;
    //             if (getReinforcementMode() != 0) {
    //                 skipEta = true;
    //             }
    // #ifdef SWARM_DEBUG
    //             WRITE_MESSAGEF("\n% MSSwarmTrafficLightLogic decideNextPhase()", time2string(MSNet::getInstance()->getCurrentTimeStep()));
    //             std::ostringstream dnp;
    //             dnp << (MSNet::getInstance()->getCurrentTimeStep()) << " MSSwarmTrafficLightLogic::decideNextPhase:: " << "tlsid=" << getID() << " max congestion reached, congestion_steps=" << congestion_steps;
    //             WRITE_MESSAGE(dnp.str());
    // #endif
    //         }
    //     }
    //
    //     //Update pheromone levels
    //     updatePheromoneLevels();
    //
    //     /* Since we changed the behaviour of computeReturnTime() in order to update pheromone levels every step
    //      * it is now mandatory to check if the duration of a transient phase is elapsed or not*/
    //     if (getCurrentPhaseDef().isTransient() && getCurrentPhaseElapsed() < getCurrentPhaseDef().duration) {
    //         return getCurrentPhaseIndex();
    //     }
    //
    //     //Decide the current policy according to pheromone levels. this should be done only at the end of a chain, before selecting the new one
    //     if (getCurrentPhaseDef().isCommit()) {
    //         //Update learning and forgetting thresholds
    //         updateSensitivities();
    //         decidePolicy();
    //         gotTargetLane = false;
    //     }
    //
    // //    double phero =0;
    // //	if(getCurrentPhaseDef().isDecisional())
    // //	{
    // //    for(LaneIdVector::const_iterator it = targetLanes.begin(); it != targetLanes.end(); ++it)
    // //    {
    // //      string name = (*it);
    // //      phero +=pheromoneInputLanes[name];
    // //    }
    // //    phero /= targetLanes.size() == 0 ? 1 : targetLanes.size();
    // //    if(getCurrentPhaseElapsed() >= getCurrentPhaseDef().minDuration)
    // //      if(abs(phero-pheroBegin) <= 2)
    // //        return getCurrentPhaseIndex() + 1;
    // //	}
    // #ifdef SWARM_DEBUG
    //     std::ostringstream str;
    //     str << "tlsID=" << getID() << " currentPolicyname=" + myCurrentPolicy->getName();
    //     WRITE_MESSAGE(str.str());
    // #endif
    //
    //     //Execute current policy. congestion "policy" must maintain the commit phase, and that must be an all-red one
    //     return myCurrentPolicy->decideNextPhase(getCurrentPhaseElapsed(), &getCurrentPhaseDef(), getCurrentPhaseIndex(),
    //                                             getPhaseIndexWithMaxCTS(), isThresholdPassed(), isPushButtonPressed(), countVehicles(getCurrentPhaseDef()));
    // //	int newStep =myCurrentPolicy->decideNextPhase(getCurrentPhaseElapsed(), &getCurrentPhaseDef(), getCurrentPhaseIndex(),
    // //	          getPhaseIndexWithMaxCTS(), isThresholdPassed(), isPushButtonPressed(), countVehicles(getCurrentPhaseDef()));
    // //	if(newStep != myStep)
    // //	  pheroBegin = phero;
    // //	return newStep;
    // }
    #[allow(unused_variables)]
    pub fn decide_next_phase(&mut self) -> i32 {
        // if we're congested, it should be wise to reset and recalculate the pheromone levels after X steps
        if self.get_current_phase_def().is_target() {
            self.targetLanes = self.get_current_phase_def().get_target_lane_set().clone();
        }

        if self.myCurrentPolicy.m().get_name() == "Congestion" && self.get_current_phase_def().is_commit() {
            self.congestion_steps += 1; // STEPS2TIME(getCurrentPhaseDef().duration);
            if self.congestion_steps >= self.get_max_congestion_duration() {
                self.reset_pheromone();
                self.congestion_steps = 0;
                self.mustChange = true;
                if self.get_reinforcement_mode() != 0 {
                    self.skipEta = true;
                }
            }
        }

        // Update pheromone levels
        self.update_pheromone_levels();

        /* Since we changed the behaviour of computeReturnTime() in order to update pheromone levels every step
         * it is now mandatory to check if the duration of a transient phase is elapsed or not*/
        if self.get_current_phase_def().is_transient() && self.get_current_phase_elapsed() < self.get_current_phase_def().duration {
            return self.get_current_phase_index();
        }

        // Decide the current policy according to pheromone levels. this should be done only at the end of a chain, before selecting the new one
        if self.get_current_phase_def().is_commit() {
            // Update learning and forgetting thresholds
            self.update_sensitivities();
            self.decide_policy();
            self.gotTargetLane = false;
        }

        // Execute current policy. congestion "policy" must maintain the commit phase, and that must be an all-red one
        let policy = self.myCurrentPolicy;
        let curPhase: Ptr<MSPhaseDefinition> = Ptr::new(self.get_current_phase_def());
        let elapsed = self.get_current_phase_elapsed();
        let currentPhaseIndex = self.get_current_phase_index();
        let phaseMaxCTS = self.get_phase_index_with_max_cts();
        let thresholdPassed = self.is_threshold_passed();
        let pushButtonPressed = self.is_push_button_pressed();
        let vehicleCount = self.count_vehicles(curPhase);
        policy.m().decide_next_phase(elapsed, curPhase, currentPhaseIndex, phaseMaxCTS, thresholdPassed, pushButtonPressed, vehicleCount)
    }

    /// C++: `bool MSSwarmTrafficLightLogic::canRelease()` [virtual, protected]
    // bool MSSwarmTrafficLightLogic::canRelease() {
    // #ifdef SWARM_DEBUG
    //     std::ostringstream phero_str;
    //     phero_str << "getCurrentPhaseElapsed()=" << time2string(getCurrentPhaseElapsed()) << " isThresholdPassed()=" << isThresholdPassed() << " currentPhase=" << (&getCurrentPhaseDef())->getState() << " countVehicles()=" << countVehicles(getCurrentPhaseDef());
    //     WRITE_MESSAGE("MSSwamTrafficLightLogic::canRelease(): " + phero_str.str());
    // #endif
    //     return myCurrentPolicy->canRelease(getCurrentPhaseElapsed(), isThresholdPassed(), isPushButtonPressed(), &getCurrentPhaseDef(),
    //                                        countVehicles(getCurrentPhaseDef()));
    // }
    #[allow(unused_variables)]
    pub fn can_release(&mut self) -> bool {
        let policy = self.myCurrentPolicy;
        let elapsed = self.get_current_phase_elapsed();
        let thresholdPassed = self.is_threshold_passed();
        let pushButtonPressed = self.is_push_button_pressed();
        let curPhase: Ptr<MSPhaseDefinition> = Ptr::new(self.get_current_phase_def());
        let vehicleCount = self.count_vehicles(curPhase);
        policy.m().can_release(elapsed, thresholdPassed, pushButtonPressed, curPhase, vehicleCount)
    }

    /// C++: `SUMOTime MSSwarmTrafficLightLogic::computeReturnTime()` [virtual, protected]
    // virtual SUMOTime computeReturnTime() {
    //
    //         return DELTA_T;
    //
    //     }
    #[allow(unused_variables)]
    pub fn compute_return_time(&mut self) -> SUMOTime {
        utils::common::time::delta_t()
    }

    /// C++: `void MSSwarmTrafficLightLogic::resetPheromone()` [protected]
    /// Resets pheromone levels
    // void MSSwarmTrafficLightLogic::resetPheromone() {
    //     //input
    //     for (MSLaneId_PheromoneMap::iterator laneIterator = pheromoneInputLanes.begin();
    //             laneIterator != pheromoneInputLanes.end(); laneIterator++) {
    //         std::string laneId = laneIterator->first;
    //         pheromoneInputLanes[laneId] = 0;
    //     }
    //     //output
    //     for (MSLaneId_PheromoneMap::iterator laneIterator = pheromoneOutputLanes.begin();
    //             laneIterator != pheromoneOutputLanes.end(); laneIterator++) {
    //         std::string laneId = laneIterator->first;
    //         pheromoneOutputLanes[laneId] = 0;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn reset_pheromone(&mut self) {
        // input
        for v in self.pheromoneInputLanes.values_mut() {
            *v = 0.0;
        }
        // output
        for v in self.pheromoneOutputLanes.values_mut() {
            *v = 0.0;
        }
    }

    /// C++: `double MSSwarmTrafficLightLogic::getPheromoneForInputLanes()` [protected]
    // double MSSwarmTrafficLightLogic::getPheromoneForInputLanes() {
    //     if (pheromoneInputLanes.size() == 0) {
    //         return 0;
    //     }
    //     double pheroIn = 0;
    //     for (MSLaneId_PheromoneMap::const_iterator iterator = pheromoneInputLanes.begin();
    //             iterator != pheromoneInputLanes.end(); iterator++) {
    //         std::string laneId = iterator->first;
    //         pheroIn += iterator->second;
    // #ifdef SWARM_DEBUG
    //         std::ostringstream phero_str;
    //         phero_str << " lane " << iterator->first << " pheromoneIN  " << iterator->second << " id " << getID() << " .";
    //         WRITE_MESSAGE(time2string(MSNet::getInstance()->getCurrentTimeStep()) + " MSSwarmTrafficLightLogic::getPheromoneForInputLanes::" + phero_str.str());
    // #endif
    //     }
    //
    // #ifdef SWARM_DEBUG
    //     std::ostringstream o_str;
    //     o_str << " TOTpheromoneIN  " << pheroIn << " return  " << pheroIn / pheromoneInputLanes.size() << getID() << " .";
    //     WRITE_MESSAGE(time2string(MSNet::getInstance()->getCurrentTimeStep()) + " MSSwarmTrafficLightLogic::getPheromoneForInputLanes::" + o_str.str());
    // #endif
    //     return pheroIn / (double)pheromoneInputLanes.size();
    // }
    #[allow(unused_variables)]
    pub fn get_pheromone_for_input_lanes(&mut self) -> f64 {
        if self.pheromoneInputLanes.len() == 0 {
            return 0.0;
        }
        let mut pheroIn: f64 = 0.0;
        for v in self.pheromoneInputLanes.values() {
            pheroIn += *v;
        }
        pheroIn / (self.pheromoneInputLanes.len() as f64)
    }

    /// C++: `double MSSwarmTrafficLightLogic::getPheromoneForOutputLanes()` [protected]
    // double MSSwarmTrafficLightLogic::getPheromoneForOutputLanes() {
    //     if (pheromoneOutputLanes.size() == 0) {
    //         return 0;
    //     }
    //     double pheroOut = 0;
    //     for (MSLaneId_PheromoneMap::const_iterator iterator = pheromoneOutputLanes.begin();
    //             iterator != pheromoneOutputLanes.end(); iterator++) {
    // #ifdef SWARM_DEBUG
    //         std::ostringstream phero_str;
    //         phero_str << " lane " << iterator->first << " pheromoneOUT  " << iterator->second << " id " << getID() << " .";
    //         WRITE_MESSAGE(time2string(MSNet::getInstance()->getCurrentTimeStep()) + " MSSwarmTrafficLightLogic::getPheromoneForOutputLanes::" + phero_str.str());
    // #endif
    //         pheroOut += iterator->second;
    //     }
    // #ifdef SWARM_DEBUG
    //     std::ostringstream o_str;
    //     o_str << " TOTpheromoneOUT  " << pheroOut << " return  " << pheroOut / pheromoneOutputLanes.size() << " id " << getID() << " .";
    //     WRITE_MESSAGE(time2string(MSNet::getInstance()->getCurrentTimeStep()) + " MSSwarmTrafficLightLogic::getPheromoneForOutputLanes::" + o_str.str());
    // #endif
    //     return pheroOut / (double)pheromoneOutputLanes.size();
    // }
    #[allow(unused_variables)]
    pub fn get_pheromone_for_output_lanes(&mut self) -> f64 {
        if self.pheromoneOutputLanes.len() == 0 {
            return 0.0;
        }
        let mut pheroOut: f64 = 0.0;
        for v in self.pheromoneOutputLanes.values() {
            pheroOut += *v;
        }
        pheroOut / (self.pheromoneOutputLanes.len() as f64)
    }

    /// C++: `double MSSwarmTrafficLightLogic::getDispersionForInputLanes(double)` [protected]
    // double MSSwarmTrafficLightLogic::getDispersionForInputLanes(double average_phero_in) {
    //     if (pheromoneInputLanes.size() == 0) {
    //         return 0;
    //     }
    //     double sum = 0;
    //     for (MSLaneId_PheromoneMap::const_iterator iterator = pheromoneInputLanes.begin();
    //             iterator != pheromoneInputLanes.end(); iterator++) {
    //         std::string laneId = iterator->first;
    //         sum += pow(iterator->second - average_phero_in, 2);
    //     }
    //
    //     double result = sqrt(sum / (double)pheromoneInputLanes.size()) * getScaleFactorDispersionIn();
    // #ifdef SWARM_DEBUG
    //     ostringstream so_str;
    //     so_str << " dispersionIn " << result;
    //     WRITE_MESSAGE("MSSwarmTrafficLightLogic::getDispersionForInputLanes::" + so_str.str());
    // #endif
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_dispersion_for_input_lanes(&mut self, average_phero_in: f64) -> f64 {
        if self.pheromoneInputLanes.len() == 0 {
            return 0.0;
        }
        let mut sum: f64 = 0.0;
        for v in self.pheromoneInputLanes.values() {
            sum += (*v - average_phero_in).powf(2.0);
        }
        let result = (sum / (self.pheromoneInputLanes.len() as f64)).sqrt() * self.get_scale_factor_dispersion_in();
        result
    }

    /// C++: `double MSSwarmTrafficLightLogic::getDispersionForOutputLanes(double)` [protected]
    // double MSSwarmTrafficLightLogic::getDispersionForOutputLanes(double average_phero_out) {
    //     if (pheromoneOutputLanes.size() == 0) {
    //         return 0;
    //     }
    //     double sum = 0;
    //     for (MSLaneId_PheromoneMap::const_iterator iterator = pheromoneOutputLanes.begin();
    //             iterator != pheromoneOutputLanes.end(); iterator++) {
    //         sum += pow(iterator->second - average_phero_out, 2);
    //     }
    //
    //     double result = sqrt(sum / (double)pheromoneOutputLanes.size()) * getScaleFactorDispersionOut();
    // #ifdef SWARM_DEBUG
    //     ostringstream so_str;
    //     so_str << " dispersionOut " << result;
    //     WRITE_MESSAGE("MSSwarmTrafficLightLogic::getDispersionForOutputLanes::" + so_str.str());
    // #endif
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_dispersion_for_output_lanes(&mut self, average_phero_out: f64) -> f64 {
        if self.pheromoneOutputLanes.len() == 0 {
            return 0.0;
        }
        let mut sum: f64 = 0.0;
        for v in self.pheromoneOutputLanes.values() {
            sum += (*v - average_phero_out).powf(2.0);
        }
        let result = (sum / (self.pheromoneOutputLanes.len() as f64)).sqrt() * self.get_scale_factor_dispersion_out();
        result
    }

    /// C++: `double MSSwarmTrafficLightLogic::getDistanceOfMaxPheroForInputLanes()` [protected]
    // double MSSwarmTrafficLightLogic::getDistanceOfMaxPheroForInputLanes() {
    //     if (pheromoneInputLanes.size() == 0) {
    //         return 0;
    //     }
    //     double max_phero_val_current = 0;
    //     double max_phero_val_old = 0;
    //     double temp_avg_other_lanes = 0;
    //     std::string laneId_max;
    //     int counter = 0;
    //     for (MSLaneId_PheromoneMap::const_iterator iterator = pheromoneInputLanes.begin();
    //             iterator != pheromoneInputLanes.end(); iterator++) {
    //         std::string laneId = iterator->first;
    //         double lanePhero = iterator->second;
    //         if (counter == 0) {
    //             max_phero_val_current = lanePhero;
    //             counter++;
    //             continue;
    //         }
    //         if (lanePhero > max_phero_val_current) {
    //             max_phero_val_old = max_phero_val_current;
    //             max_phero_val_current = lanePhero;
    //             temp_avg_other_lanes = (temp_avg_other_lanes * (counter - 1) + max_phero_val_old) / counter;
    //         } else {
    //             temp_avg_other_lanes = (temp_avg_other_lanes * (counter - 1) + lanePhero) / counter;
    //         }
    //
    //         counter++;
    //     }
    //
    //     double result = max_phero_val_current - temp_avg_other_lanes;
    // #ifdef SWARM_DEBUG
    //     ostringstream so_str;
    //     so_str << " currentMaxPhero " << max_phero_val_current << " lane " << laneId_max << " avgOtherLanes " << temp_avg_other_lanes << " distance " << result;
    //     WRITE_MESSAGE("MSSwarmTrafficLightLogic::getDistanceOfMaxPheroForInputLanes::" + so_str.str());
    // #endif
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_distance_of_max_phero_for_input_lanes(&mut self) -> f64 {
        if self.pheromoneInputLanes.len() == 0 {
            return 0.0;
        }
        let (max_phero_val_current, temp_avg_other_lanes) = distance_of_max_phero(&self.pheromoneInputLanes);
        max_phero_val_current - temp_avg_other_lanes
    }

    /// C++: `double MSSwarmTrafficLightLogic::getDistanceOfMaxPheroForOutputLanes()` [protected]
    // double MSSwarmTrafficLightLogic::getDistanceOfMaxPheroForOutputLanes() {
    //     if (pheromoneOutputLanes.size() == 0) {
    //         return 0;
    //     }
    //     double max_phero_val_current = 0;
    //     double max_phero_val_old = 0;
    //     double temp_avg_other_lanes = 0;
    //     std::string laneId_max;
    //     int counter = 0;
    //     for (MSLaneId_PheromoneMap::const_iterator iterator = pheromoneOutputLanes.begin();
    //             iterator != pheromoneOutputLanes.end(); iterator++) {
    //         std::string laneId = iterator->first;
    //         double lanePhero = iterator->second;
    //         if (counter == 0) {
    //             max_phero_val_current = lanePhero;
    //             counter++;
    //             continue;
    //         }
    //         if (lanePhero > max_phero_val_current) {
    //             max_phero_val_old = max_phero_val_current;
    //             max_phero_val_current = lanePhero;
    //             temp_avg_other_lanes = (temp_avg_other_lanes * (counter - 1) + max_phero_val_old) / counter;
    //         } else {
    //             temp_avg_other_lanes = (temp_avg_other_lanes * (counter - 1) + lanePhero) / counter;
    //         }
    //
    //         counter++;
    //     }
    //
    //     double result = max_phero_val_current - temp_avg_other_lanes;
    // #ifdef SWARM_DEBUG
    //     ostringstream so_str;
    //     so_str << " currentMaxPhero " << max_phero_val_current << " lane " << laneId_max << " avgOtherLanes " << temp_avg_other_lanes << " distance " << result;
    //     WRITE_MESSAGE("MSSwarmTrafficLightLogic::getDistanceOfMaxPheroForOutputLanes::" + so_str.str());
    // #endif
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_distance_of_max_phero_for_output_lanes(&mut self) -> f64 {
        if self.pheromoneOutputLanes.len() == 0 {
            return 0.0;
        }
        let (max_phero_val_current, temp_avg_other_lanes) = distance_of_max_phero(&self.pheromoneOutputLanes);
        max_phero_val_current - temp_avg_other_lanes
    }

    /// C++: `void MSSwarmTrafficLightLogic::updatePheromoneLevels()` [protected]
    /// Update pheromone levels Pheromone on input lanes is costantly updated Pheromone follows a discrete-time dynamic law "pheromone(k+1) = beta*pheromone(k) + gamma * sensed_val(k)"
    // void MSSwarmTrafficLightLogic::updatePheromoneLevels() {
    //     //Updating input lanes pheromone: all input lanes without distinction
    //     //BETA_NO, GAMMA_NO
    //     updatePheromoneLevels(pheromoneInputLanes, "PheroIn", getBetaNo(), getGammaNo());
    //
    //     //BETA_SP, GAMMA_SP
    //     //Updating output lanes pheromone: only input lanes currently having green light. Pheromone for non green lanes is "freezed"
    // //    if (getCurrentPhaseDef().isDecisional()) {
    //     updatePheromoneLevels(pheromoneOutputLanes, "PheroOut", getBetaSp(), getGammaSp());
    // //    }
    // }
    #[allow(unused_variables)]
    pub fn update_pheromone_levels(&mut self) {
        // Updating input lanes pheromone: all input lanes without distinction
        // BETA_NO, GAMMA_NO
        let beta = self.get_beta_no();
        let gamma = self.get_gamma_no();
        let mut inMap = std::mem::take(&mut self.pheromoneInputLanes);
        self.update_pheromone_levels_2(&mut inMap, "PheroIn".to_string(), beta, gamma);
        self.pheromoneInputLanes = inMap;

        // BETA_SP, GAMMA_SP
        // Updating output lanes pheromone: only input lanes currently having green light. Pheromone for non green lanes is "freezed"
        let beta = self.get_beta_sp();
        let gamma = self.get_gamma_sp();
        let mut outMap = std::mem::take(&mut self.pheromoneOutputLanes);
        self.update_pheromone_levels_2(&mut outMap, "PheroOut".to_string(), beta, gamma);
        self.pheromoneOutputLanes = outMap;
    }

    /// C++: `void MSSwarmTrafficLightLogic::updatePheromoneLevels(MSLaneId_PheromoneMap &, std::string, const double, const double)` [protected]
    /// Utility method to avoid code duplication
    // void MSSwarmTrafficLightLogic::updatePheromoneLevels(MSLaneId_PheromoneMap& pheroMap, std::string logString,
    //         const double beta, const double gamma) {
    //     //	ANALYSIS_DBG(
    // #ifdef SWARM_DEBUG
    //     std::ostringstream _str;
    //     _str << logString << " Lanes " << pheroMap.size() << " TL " << getID() << " .";
    //     WRITE_MESSAGE(time2string(MSNet::getInstance()->getCurrentTimeStep()) + " MSSwarmTrafficLightLogic::updatePheromoneLevels:: " + _str.str());
    // #else
    //     UNUSED_PARAMETER(logString);
    // #endif
    //     for (MSLaneId_PheromoneMap::iterator laneIterator = pheroMap.begin(); laneIterator != pheroMap.end();
    //             ++laneIterator) {
    //         std::string laneId = laneIterator->first;
    //         double oldPhero = laneIterator->second;
    //         double maxSpeed = getSensors()->getMaxSpeed(laneId);
    //         double meanVehiclesSpeed = getSensors()->meanVehiclesSpeed(laneId);
    //         bool updatePheromone = (meanVehiclesSpeed > -1);
    //         //		double pheroAdd = getSensors()->countVehicles(laneId);
    //
    //         //derivative
    //         double derivative = 0;
    //         //If i need to use the derivative for the lane
    //         if (m_meanSpeedHistory.find(laneId) != m_meanSpeedHistory.end()) {
    //             //Update the derivative
    //             if (updatePheromone) {
    //                 double currentDerivative = 0;
    //                 m_losCounter = 0;
    //                 if (m_meanSpeedHistory[laneId]->size() > 0) {
    //                     //Calculate the current derivative mean with the old speed points
    //                     for (int i = 0; i < m_meanSpeedHistory[laneId]->size(); ++i)
    //                         if (i == 0) {
    //                             currentDerivative += fabs(meanVehiclesSpeed - m_meanSpeedHistory[laneId]->at(i));
    //                         } else {
    //                             currentDerivative += fabs(m_meanSpeedHistory[laneId]->at(i - 1) - m_meanSpeedHistory[laneId]->at(i));
    //                         }
    //                     currentDerivative /= m_meanSpeedHistory[laneId]->size(); //Non weighted mean
    //                 }
    //                 m_meanSpeedHistory[laneId]->push_front(meanVehiclesSpeed);
    //                 //Check if the current value of the derivative is above the set alpha
    //                 if (currentDerivative >= m_derivativeAlpha) {
    //                     m_derivativeHistory[laneId]->push_front(currentDerivative);
    //                 }
    //                 if (m_derivativeHistory[laneId]->size() > 0) {
    //                     //Calculate the mean derivative with the old derivative
    //                     for (int i = 0; i < m_derivativeHistory[laneId]->size(); ++i) {
    //                         derivative += m_derivativeHistory[laneId]->at(i);
    //                     }
    //                     derivative /= m_derivativeHistory[laneId]->size();
    //                 }
    //             } else {
    //                 //Reset the values if no information is received after a timeout
    //                 ++m_losCounter;
    //                 if (m_losCounter >= m_losMaxLimit) {
    //                     m_derivativeHistory[laneId]->clear();
    //                     m_meanSpeedHistory[laneId]->clear();
    //                     m_meanSpeedHistory[laneId]->push_front(maxSpeed);
    //                 }
    //             }
    //         }
    //         double pheroAdd = MAX2((maxSpeed - meanVehiclesSpeed) * 10 / maxSpeed, 0.0);
    // //		Use the derivative only if it has a value
    //         if (derivative > 0)
    // //		Correct the pheromone value by dividing it for the derivative.
    //         {
    //             pheroAdd /= MAX2(derivative, m_derivativeAlpha);
    //         }
    // //    pheroAdd /= max(derivative, 1.0);
    // #ifdef ANALYSIS_DEBUG
    //         if (updatePheromone) {
    //             std::ostringstream oss;
    //             oss << time2string(MSNet::getInstance()->getCurrentTimeStep()) << " l " << laneId;
    //             oss << " der " << derivative << " phero " << pheroAdd << " maxS " << maxSpeed << " meanS " << meanVehiclesSpeed;
    //             WRITE_MESSAGE(oss.str())
    //         }
    // #endif
    //
    //         // Evaporation + current contribute
    //         double phero = beta * oldPhero + gamma * pheroAdd * updatePheromone;
    // #ifdef ANALYSIS_DEBUG
    //         if (phero > 10) {
    //             std::ostringstream i_str;
    //             i_str << "MSSwarmTrafficLightLogic::updatePheromoneLevels " << logString << " > 10. Value: " << phero;
    //             WRITE_MESSAGE(i_str.str())
    //         }
    // #endif
    //         phero = MIN2(MAX2(phero, 0.0), getPheroMaxVal());
    //         pheroMap[laneId] = phero;
    // #ifdef ANALYSIS_DEBUG
    //         //		DBG(
    //         std::ostringstream i_str;
    //         //					i_str << " oldPheroIn " << oldPheroIn
    //         //						<< " inMeanVehiclesSpeed " << meanVehiclesSpeed
    //         //						<< " pheroInAdd " << pheroAdd * updatePheromoneIn
    //         //						<< " pheroInEvaporated " << oldPheroIn-oldPheroIn*getBetaNo()
    //         //						<< " pheroInDeposited " << getGammaNo() * pheroAdd * updatePheromoneIn
    //         //						<<" newPheroIn "<<pheromoneInputLanes[laneId]
    //         //						<< " inLane "<< laneId<<" ID "<< getID() <<" .";
    //         i_str << " op " << oldPhero << " ms " << meanVehiclesSpeed << " p " << pheroAdd* updatePheromone <<
    //               " pe " << oldPhero - oldPhero* beta << " pd " << gamma* pheroAdd* updatePheromone << " np " <<
    //               pheroMap[laneId] << " l " << laneId << " ID " << getID() << " c " << getSensors()->countVehicles(laneId) << " s " << getLaneLightState(laneId) << " .";
    //         if (m_pheroLevelLog[laneId] != i_str.str()) {
    //             m_pheroLevelLog[laneId] = i_str.str();
    //             WRITE_MESSAGE(time2string(MSNet::getInstance()->getCurrentTimeStep()) + " MSSwarmTrafficLightLogic::updatePheromoneLevels:: " + logString + i_str.str());
    //         }
    // #endif
    // #ifdef SWARM_DEBUG
    //         std::ostringstream str;
    //         str << time2string(MSNet::getInstance()->getCurrentTimeStep()) << " MSSwarmTrafficLightLogic::countSensors:: lane " << laneId << " passedVeh " << getCountSensors()->getPassedVeh(laneId, false);
    //         WRITE_MESSAGE(str.str());
    // #endif
    // //		int vehicles = getSensors()->countVehicles(laneId);
    // //		double pheroIn = getBetaNo() * oldPheroIn + // Evaporation
    // //		getGammaNo() * vehicles;
    // //		DBG(
    // //		std::ostringstream i_str;
    // //		i_str << " vehicles " << getSensors()->countVehicles(laneId)<<" pheromoneInputLanes "<<pheromoneInputLanes[laneId] << " lane "<< laneId<<" ID "<<  getID() <<" .";
    // //		MsgHandler::getMessageInstance()->inform(time2string(MSNet::getInstance()->getCurrentTimeStep()) +" MSSwarmTrafficLightLogic::updatePheromoneLevels:: PheroIn"+i_str.str());
    // //		)
    // //
    // //      pheroIn = MIN2(MAX2(pheroIn, 0.0), getPheroMaxVal());
    // //		pheromoneInputLanes[laneId] = pheroIn;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn update_pheromone_levels_2(&mut self, arg0: &mut MSLaneId_PheromoneMap, arg1: String, arg2: f64, arg3: f64) {
        // Utility method to avoid code duplication
        let laneIds: Vec<String> = arg0.keys().cloned().collect();
        for laneId in laneIds.iter() {
            let oldPhero: f64 = arg0[laneId];
            let maxSpeed = self.get_sensors().m().get_max_speed(laneId.clone());
            let meanVehiclesSpeed = self.get_sensors().m().mean_vehicles_speed_2(laneId.clone());
            let updatePheromone = meanVehiclesSpeed > -1.0;

            // derivative
            let mut derivative: f64 = 0.0;
            // If i need to use the derivative for the lane
            if self.m_meanSpeedHistory.contains_key(laneId) {
                // Update the derivative
                if updatePheromone {
                    let mut currentDerivative: f64 = 0.0;
                    self.m_losCounter = 0;
                    let meanHistory: Ptr<CircularBuffer<f64>> = self.m_meanSpeedHistory[laneId];
                    if meanHistory.size() > 0 {
                        // Calculate the current derivative mean with the old speed points
                        for i in 0..meanHistory.size() {
                            if i == 0 {
                                currentDerivative += (meanVehiclesSpeed - meanHistory.at(i)).abs();
                            } else {
                                currentDerivative += (meanHistory.at(i - 1) - meanHistory.at(i)).abs();
                            }
                        }
                        currentDerivative /= meanHistory.size() as f64; // Non weighted mean
                    }
                    meanHistory.m().push_front(meanVehiclesSpeed);
                    // Check if the current value of the derivative is above the set alpha
                    if currentDerivative >= self.m_derivativeAlpha {
                        self.m_derivativeHistory[laneId].m().push_front(currentDerivative);
                    }
                    let derivativeHistory: Ptr<CircularBuffer<f64>> = self.m_derivativeHistory[laneId];
                    if derivativeHistory.size() > 0 {
                        // Calculate the mean derivative with the old derivative
                        for i in 0..derivativeHistory.size() {
                            derivative += derivativeHistory.at(i);
                        }
                        derivative /= derivativeHistory.size() as f64;
                    }
                } else {
                    // Reset the values if no information is received after a timeout
                    self.m_losCounter += 1;
                    if self.m_losCounter >= self.m_losMaxLimit {
                        let derivativeHistory: Ptr<CircularBuffer<f64>> = self.m_derivativeHistory[laneId];
                        derivativeHistory.m().clear();
                        let meanHistory: Ptr<CircularBuffer<f64>> = self.m_meanSpeedHistory[laneId];
                        meanHistory.m().clear();
                        meanHistory.m().push_front(maxSpeed);
                    }
                }
            }
            let mut pheroAdd = max2((maxSpeed - meanVehiclesSpeed) * 10.0 / maxSpeed, 0.0);
            // Use the derivative only if it has a value
            if derivative > 0.0 {
                // Correct the pheromone value by dividing it for the derivative.
                pheroAdd /= max2(derivative, self.m_derivativeAlpha);
            }

            // Evaporation + current contribute
            let mut phero = beta_gamma_phero(arg2, oldPhero, arg3, pheroAdd, updatePheromone);
            phero = min2(max2(phero, 0.0), self.get_phero_max_val());
            arg0.insert(laneId.clone(), phero);
        }
    }

    /// C++: `void MSSwarmTrafficLightLogic::updateSensitivities()` [protected]
    /// After a policy has been chosen, for every iteration thresholds has to be updated. Thresholds reinforcement lowers the theta_sensitivity for the current policy and raises the ones for currently unused policies. Thresholds belongs to the interval [THETA_MIN THETA_MAX]
    // void MSSwarmTrafficLightLogic::updateSensitivities() {
    //     double elapsedTime = STEPS2TIME(MSNet::getInstance()->getCurrentTimeStep() - lastThetaSensitivityUpdate);
    //     lastThetaSensitivityUpdate = MSNet::getInstance()->getCurrentTimeStep();
    //
    //     //reset of the sensitivity thresholds in case of 0 pheromone on the input lanes
    //     if (getPheromoneForInputLanes() == 0) {
    //         for (MSSOTLPolicy* const policy : myPolicies) {
    //             policy->setThetaSensitivity(getThetaInit());
    // //			ANALYSIS_DBG(
    // #ifdef SWARM_DEBUG
    //             std::ostringstream phero_str;
    //             phero_str << "Policy " << policy->getName() << " sensitivity reset to " << policy->getThetaSensitivity() << " due to evaporated input pheromone.";
    //             WRITE_MESSAGE(time2string(MSNet::getInstance()->getCurrentTimeStep()) + " MSSwarmTrafficLightLogic::updateSensitivities::" + phero_str.str());
    // #endif
    //         }
    //         return;
    //     }
    //
    //     double eta = -1.;
    //     // If skipEta it means that we've had Congestion for too much time. Forcing forgetting.
    //     if (!skipEta || myCurrentPolicy->getName().compare("Congestion") != 0) {
    //         switch (getReinforcementMode()) {
    //             case 0:
    //                 if (elapsedTime == STEPS2TIME(MSNet::getInstance()->getCurrentTimeStep())) {
    //                     return;	//we don't want to reinforce the policy selected at the beginning of the simulation since it's time-based
    //                 }
    //                 eta = elapsedTime;
    //                 break;
    //             case 1:
    //                 eta = calculateEtaDiff();
    //                 break;
    //             case 2:
    //                 eta = calculateEtaRatio();
    //                 break;
    //         }
    //     }
    //     for (MSSOTLPolicy* const policy : myPolicies) {
    //         double newSensitivity;
    //         if (eta < 0) {	//bad performance
    //             if (policy == myCurrentPolicy) { // punish the current policy
    //                 newSensitivity = policy->getThetaSensitivity() + getForgettingCox() * (-eta);
    //             } else
    //                 // reward the other ones
    //             {
    //                 newSensitivity = policy->getThetaSensitivity() - getLearningCox() * (-eta);
    //             }
    //         } else {	//good performance
    //             if (policy == myCurrentPolicy) {	//reward the current policy
    //                 newSensitivity = policy->getThetaSensitivity() - getLearningCox() * eta;
    //             } else
    //                 //	punish the other ones
    //             {
    //                 newSensitivity = policy->getThetaSensitivity() + getForgettingCox() * eta;
    //             }
    //         }
    // //			ANALYSIS_DBG(
    // #ifdef SWARM_DEBUG
    //         std::ostringstream lf;
    //         std::ostringstream phero_str;
    //         if (getReinforcementMode() == 0) {
    //             if (policy == currentPolicy) {
    //                 lf << " ,LearningCox " << getLearningCox() << " ,LCox*Time " << getLearningCox() * elapsedTime;
    //             } else {
    //                 lf << " ,ForgettingCox " << getForgettingCox() << " ,FCox*Time " << getForgettingCox() * elapsedTime;
    //             }
    //
    //             phero_str << " policy " << policy->getName() << " newSensitivity " << newSensitivity << " ,pol.Sensitivity " << policy->getThetaSensitivity() << " ,elapsedTime " << elapsedTime << lf.str() << " NEWERSensitivity= " << max(min(newSensitivity, getThetaMax()), getThetaMin()) << " ID " << getID() << " .";
    //         } else {
    //             if (policy == currentPolicy && eta > 0) {
    //                 lf << " ,LearningCox " << getLearningCox() << " ,LCox*Eta " << getLearningCox() * eta;
    //             } else if (policy == currentPolicy && eta < 0) {
    //                 lf << " ,ForgettingCox " << getForgettingCox() << " ,FCox*Eta " << getForgettingCox() * eta;
    //             } else if (eta > 0) {
    //                 lf << " ,ForgettingCox " << getForgettingCox() << " ,FCox*Eta " << getForgettingCox() * eta;
    //             } else if (eta < 0) {
    //                 lf << " ,LearningCox " << getLearningCox() << " ,LCox*Eta " << getLearningCox() * eta;
    //             }
    //             phero_str << " policy " << policy->getName() << " newSensitivity " << newSensitivity << " ,pol.Sensitivity " << policy->getThetaSensitivity() << " ,eta " << eta << " ,carsIn " << carsIn << " ,inTarget " << inTarget << " ,notTarget " << notTarget << " ,carsOut " << carsOut << lf.str() << " NEWERSensitivity= " << max(min(newSensitivity, getThetaMax()), getThetaMin()) << " ID " << getID() << " .";
    //         }
    //         WRITE_MESSAGE(time2string(MSNet::getInstance()->getCurrentTimeStep()) + " MSSwarmTrafficLightLogic::updateSensitivities::" + phero_str.str());
    // #endif
    //         newSensitivity = MAX2(MIN2(newSensitivity, getThetaMax()), getThetaMin());
    //         policy->setThetaSensitivity(newSensitivity);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn update_sensitivities(&mut self) {
        let elapsedTime = utils::common::time::steps2time(MSNet::get_instance().get_current_time_step() - self.lastThetaSensitivityUpdate);
        self.lastThetaSensitivityUpdate = MSNet::get_instance().get_current_time_step();

        // reset of the sensitivity thresholds in case of 0 pheromone on the input lanes
        if self.get_pheromone_for_input_lanes() == 0.0 {
            let policies = self.myPolicies.clone();
            for policy in policies.iter() {
                policy.m().set_theta_sensitivity(self.get_theta_init());
            }
            return;
        }

        let mut eta: f64 = -1.0;
        // If skipEta it means that we've had Congestion for too much time. Forcing forgetting.
        if !self.skipEta || self.myCurrentPolicy.m().get_name() != "Congestion" {
            match self.get_reinforcement_mode() {
                0 => {
                    if elapsedTime == utils::common::time::steps2time(MSNet::get_instance().get_current_time_step()) {
                        return; // we don't want to reinforce the policy selected at the beginning of the simulation since it's time-based
                    }
                    eta = elapsedTime;
                }
                1 => {
                    eta = self.calculate_eta_diff();
                }
                2 => {
                    eta = self.calculate_eta_ratio();
                }
                _ => {}
            }
        }
        let policies = self.myPolicies.clone();
        for policy in policies.iter() {
            let policy = *policy;
            let mut newSensitivity: f64;
            if eta < 0.0 {
                // bad performance
                if policy == self.myCurrentPolicy {
                    // punish the current policy
                    newSensitivity = policy.m().get_theta_sensitivity() + self.get_forgetting_cox() * (-eta);
                } else {
                    // reward the other ones
                    newSensitivity = policy.m().get_theta_sensitivity() - self.get_learning_cox() * (-eta);
                }
            } else {
                // good performance
                if policy == self.myCurrentPolicy {
                    // reward the current policy
                    newSensitivity = policy.m().get_theta_sensitivity() - self.get_learning_cox() * eta;
                } else {
                    // punish the other ones
                    newSensitivity = policy.m().get_theta_sensitivity() + self.get_forgetting_cox() * eta;
                }
            }
            newSensitivity = max2(min2(newSensitivity, self.get_theta_max()), self.get_theta_min());
            policy.m().set_theta_sensitivity(newSensitivity);
        }
    }

    /// C++: `void MSSwarmTrafficLightLogic::decidePolicy()` [virtual, protected]
    /// Decide the current policy according to pheromone levels The decision reflects on currentPolicy value
    // void MSSwarmTrafficLightLogic::decidePolicy() {
    // //	MSSOTLPolicy* currentPolicy = myCurrentPolicy;
    //     // Decide if it is the case to check for another plan
    // //	double sampled = (double) RandHelper::rand(RAND_MAX);
    //     double sampled = RandHelper::rand();
    //     double changeProb = getChangePlanProbability();
    // //	changeProb = changeProb * RAND_MAX;
    //
    //     if (sampled <= changeProb || mustChange) { // Check for another plan
    //
    //         double pheroIn = getPheromoneForInputLanes();
    //         double pheroOut = getPheromoneForOutputLanes();
    //         //double dispersionIn = getDispersionForInputLanes(pheroIn);
    //         //double dispersionOut = getDispersionForOutputLanes(pheroOut);
    //         double distancePheroIn = getDistanceOfMaxPheroForInputLanes();
    //         double distancePheroOut = getDistanceOfMaxPheroForOutputLanes();
    //         MSSOTLPolicy* oldPolicy = myCurrentPolicy;
    //         choosePolicy(pheroIn, pheroOut, distancePheroIn, distancePheroOut);
    //         MSSOTLPolicy* newPolicy = myCurrentPolicy;
    //
    //         if (newPolicy != oldPolicy) {
    // #ifdef ANALYSIS_DEBUG
    //             SUMOTime step = MSNet::getInstance()->getCurrentTimeStep();
    //             std::ostringstream phero_str;
    //             phero_str << " (pheroIn= " << pheroIn << " ,pheroOut= " << pheroOut << " )";
    //             WRITE_MESSAGE("TL " + getID() + " time=" + time2string(step) + " Policy: " + newPolicy->getName() + phero_str.str() + " OldPolicy: " + oldPolicy->getName() + " id " + getID() + " .");
    // #endif
    //             if (oldPolicy->getName().compare("Congestion") == 0) {
    //                 congestion_steps = 0;
    //             }
    //         } else { //debug purpose only
    // #ifdef ANALYSIS_DEBUG
    //             std::ostringstream phero_str;
    //             phero_str << " (pheroIn= " << pheroIn << " ,pheroOut= " << pheroOut << " )";
    //             SUMOTime step = MSNet::getInstance()->getCurrentTimeStep();
    //             WRITE_MESSAGE("TL " + getID() + " time=" + time2string(step) + " Policy: Nochanges" + phero_str.str() + " OldPolicy: " + oldPolicy->getName() + " id " + getID() + " .");
    // #endif
    //         }
    //
    //         mustChange = false;
    //         skipEta = false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn decide_policy(&mut self) {
        // Decide if it is the case to check for another plan
        let sampled = utils::common::rand_helper::rand();
        let changeProb = self.get_change_plan_probability();

        if sampled <= changeProb || self.mustChange {
            // Check for another plan
            let pheroIn = self.get_pheromone_for_input_lanes();
            let pheroOut = self.get_pheromone_for_output_lanes();
            let distancePheroIn = self.get_distance_of_max_phero_for_input_lanes();
            let distancePheroOut = self.get_distance_of_max_phero_for_output_lanes();
            let oldPolicy = self.myCurrentPolicy;
            self.choose_policy(pheroIn, pheroOut, distancePheroIn, distancePheroOut);
            let newPolicy = self.myCurrentPolicy;

            if newPolicy != oldPolicy {
                if oldPolicy.m().get_name() == "Congestion" {
                    self.congestion_steps = 0;
                }
            }

            self.mustChange = false;
            self.skipEta = false;
        }
    }

    /// C++: `double MSSwarmTrafficLightLogic::calculatePhi(int)` [protected]
    /// Method that should calculate the valor of phi a coefficient to amplify/attenuate eta based on a factor. The factor depends on the situation when the function is called; should be the number of cars in the target lanes or the number of cars in the lanes with a red tl.
    // double MSSwarmTrafficLightLogic::calculatePhi(int factor) {
    //     if (factor == 0) {
    //         return 1;
    //     }
    //     if (factor == 1) {
    //         return 0.2;
    //     } else {
    //         return 1 - (1 / ((double) factor));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn calculate_phi(&mut self, factor: i32) -> f64 {
        if factor == 0 {
            1.0
        } else if factor == 1 {
            0.2
        } else {
            1.0 - (1.0 / (factor as f64))
        }
    }

    /// C++: `double MSSwarmTrafficLightLogic::calculateEtaDiff()` [protected]
    /// Method that should calculate the valor of eta a coefficient to evaluate the current policy's work. This eta is based on the difference between the number of vehicles that enters a tl and the ones that exit it. It consider vehicles on a lane with a tl set to red as well to determinate policy work.
    // double MSSwarmTrafficLightLogic::calculateEtaDiff() {
    //
    //     MSLane* currentLane = nullptr;
    //     int count = 0, minIn = 0, minOut = 0, toSub, tmp;
    //     bool inInit = true, outInit = true;
    //     double eta, normalized, diff, phi, delta;
    //     LaneIdVector toReset;
    //
    //     carsIn = 0;
    //     carsOut = 0;
    //     inTarget = 0;
    //     notTarget = 0;
    //
    //     MSSOTLE2Sensors* sensors = (MSSOTLE2Sensors*) getCountSensors();
    //
    //     // Search the incoming lane to get the count of the vehicles passed. [IN]
    //     for (MSTrafficLightLogic::LaneVectorVector::const_iterator laneVector = myLanes.begin();
    //             laneVector != myLanes.end(); laneVector++) {
    //         for (MSTrafficLightLogic::LaneVector::const_iterator lane = laneVector->begin(); lane != laneVector->end();
    //                 lane++) {
    //             currentLane = (*lane);
    //
    //             // Map to avoid check the lane for every possible direction
    //             if (laneCheck[currentLane] == false) {
    //                 // Get the vehicles passed from this lane.
    //                 count = sensors->getPassedVeh(currentLane->getID(), false);
    //
    // #ifdef SWARM_DEBUG
    //                 std::ostringstream cars_str;
    //                 cars_str << "Lane " << currentLane->getID() << ": vehicles entered - " << count;
    //                 WRITE_MESSAGE(time2string(MSNet::getInstance()->getCurrentTimeStep()) + " MSSwarmTrafficLightLogic::calculateEta::" + cars_str.str());
    // #endif
    //
    //                 // Increment the global count of the cars passed through the tl
    //                 carsIn += count;
    //                 // Set to true to skip similar lane since there's just one sensor
    //                 laneCheck[currentLane] = true;
    //             }
    //         }
    //     }
    //
    //     // Search the outgoing lane to get the count of the vehicles passed. [OUT]
    //     // We use the links to get the respective lane id.
    //     for (MSTrafficLightLogic::LinkVectorVector::const_iterator linkVector = myLinks.begin();
    //             linkVector != myLinks.end(); linkVector++) {
    //         for (MSTrafficLightLogic::LinkVector::const_iterator link = linkVector->begin(); link != linkVector->end();
    //                 link++) {
    //             currentLane = (*link)->getLane();
    //
    //             // Map to avoid check the lane for every possible direction
    //             if (laneCheck[currentLane] == false) {
    //                 // Get the vehicles passed from this lane.
    //                 count = sensors->getPassedVeh(currentLane->getID(), true);
    //
    // #ifdef SWARM_DEBUG
    //                 std::ostringstream cars_str;
    //                 cars_str << "Lane " << currentLane->getID() << ": vehicles gone out- " << count;
    //                 WRITE_MESSAGE(time2string(MSNet::getInstance()->getCurrentTimeStep()) + " MSSwarmTrafficLightLogic::calculateEta::" + cars_str.str());
    // #endif
    //
    //                 // Increment the global count of the cars passed through the tl
    //                 carsOut += count;
    //
    //                 // Since there's no output target lanes we check here the minimum number of
    //                 // cars passed though the tl. This ahs to be done to all the output lanes since cars can go
    //                 // in any direction from a target lane. If a direction isn't reachable the sensor count will be 0.
    //                 // This is done to update the sensorCount value in order to don't make it grow too much.
    //                 if (count != 0) {
    //                     toReset.push_back(currentLane->getID());
    //                     if (outInit) {
    //                         minOut = count;
    //                         outInit = false;
    //                     } else if (count <= minOut) {
    //                         minOut = count;
    //                     }
    //                 }
    //                 // Set to true to skip similar lane since there's just one sensor
    //                 laneCheck[currentLane] = true;
    //             }
    //         }
    //     }
    //     // Reset the map to check again all the lane on the next commit.
    //     resetLaneCheck();
    //
    //     // We retrieve the minimum number of cars passed from the target lanes.
    //     for (LaneIdVector::const_iterator laneId = targetLanes.begin(); laneId < targetLanes.end(); laneId++) {
    //         std::string lane = (*laneId);
    //         tmp = sensors->getPassedVeh(lane, false);
    //         inTarget += tmp;
    //         if (inInit && tmp != 0) {
    //             minIn = tmp;
    //             inInit = false;
    //         }
    //         if (tmp < minIn && tmp != 0) {
    //             minIn = tmp;
    //         }
    //         if (tmp != 0) {
    //             toReset.push_back(lane);
    //         }
    // #ifdef SWARM_DEBUG
    //         std::ostringstream cars_str;
    //         cars_str << "Lane " << lane << " passed: " << tmp;
    //         WRITE_MESSAGE(time2string(MSNet::getInstance()->getCurrentTimeStep()) + " MSSwarmTrafficLightLogic::calculateEta::" + cars_str.str());
    // #endif
    //     }
    //
    //     // The cars not on a target lane counted as in.
    //     notTarget = carsIn - inTarget;
    //
    //     // Calculate the min beetween the min number of cars entered the tl (minIn) and the
    //     // ones that have exit the tl (minOut)
    //     toSub = std::min(minIn, minOut);
    //
    //     // Subtract the value to all the sensor on the target lanes.
    //     while (!toReset.empty()) {
    //         std::string laneId = toReset.back();
    //         toReset.pop_back();
    //         sensors->subtractPassedVeh(laneId, toSub);
    //     }
    //
    //     //Normalized to 1
    //     diff = inTarget - carsOut;
    //     normalized = diff / inTarget;
    //
    //     // Analize difference to return an appropriate eta to reinforce/forget the policies.
    //
    // #ifdef SWARM_DEBUG
    //     std::ostringstream final_str;
    //     final_str << "Total cars in lanes: " << carsIn << " Total cars out: " << carsOut << " Difference: " << diff << " Pure eta: " << normalized;
    //     WRITE_MESSAGE(time2string(MSNet::getInstance()->getCurrentTimeStep()) + " MSSwarmTrafficLightLogic::calculateEta::" + final_str.str());
    //     std::ostringstream eta_str;
    //     eta_str << "IN:" << inTarget << " OUT:" << carsOut << " R:" << notTarget;
    //     WRITE_MESSAGE(time2string(MSNet::getInstance()->getCurrentTimeStep()) + " MSSwarmTrafficLightLogic::calculateEta::" + eta_str.str());
    //     std::ostringstream eta_str;
    //     eta_str << "Min found:" << toSub << " MinIn:" << minIn << " MinOut:" << minOut;
    //     WRITE_MESSAGE(time2string(MSNet::getInstance()->getCurrentTimeStep()) + " MSSwarmTrafficLightLogic::calculateEta::" + eta_str.str());
    // #endif
    //     // IN > OUT
    //     if (inTarget > carsOut) {
    //         if (carsOut == 0) {
    //             // We're in Congestion but not for long so we don't do nothing. When we reach max steps for
    //             // Congestion the evaluation of eta is skipped and we force a forget of the policy
    //             if (myCurrentPolicy->getName().compare("Congestion") == 0) {
    //                 eta = 0;
    //             }
    //             // vehicles aren't going out and we've additional vehicle on a red lane. We set
    //             // eta to -1 to forget
    //             else {
    //                 eta = -1;
    //             }
    //         } else {
    //             // Forget - Amplify to R
    //             phi = calculatePhi(notTarget);
    //             eta = (-normalized * (1 / phi));
    //             if (eta < -1.0) {
    //                 eta = -1.0;
    //             }
    //         }
    //     }
    //
    //     // IN = OUT
    //     else if (inTarget == carsOut) {
    //         // Can't say nothing
    //         if (inTarget == 0) {
    //             eta = 0;
    //         }
    //
    //         // Reinforce - Attenuate to R
    //         // Normalized = 0 --> use delta = 1-1/IN
    //         else {
    //             delta = calculatePhi(inTarget);
    //             phi = calculatePhi(notTarget);
    //             eta = delta * phi;
    //             if (eta > 1.0) {
    //                 eta = 1.0;
    //             }
    //         }
    //     }
    //
    //     // IN < OUT
    //     else {
    //         // Can't say nothing
    //         if (inTarget == 0) {
    //             eta = 0;
    //         }
    //
    //         // Reinforce - Attenuate to R
    //         else {
    //             phi = calculatePhi(notTarget);
    //             diff = inTarget - carsOut;
    //             normalized = diff / carsOut;
    //             eta = normalized * phi;
    //             if (eta > 1.0) {
    //                 eta = 1.0;
    //             }
    //         }
    //     }
    //
    // #ifdef SWARM_DEBUG
    //     std::ostringstream eta_str;
    //     eta_str << "Eta Normalized: " << eta;
    //     WRITE_MESSAGE(time2string(MSNet::getInstance()->getCurrentTimeStep()) + " MSSwarmTrafficLightLogic::calculateEta::" + eta_str.str());
    // #endif
    //     return eta;
    // }
    #[allow(unused_variables)]
    pub fn calculate_eta_diff(&mut self) -> f64 {
        let mut count: i32 = 0;
        let mut minIn: i32 = 0;
        let mut minOut: i32 = 0;
        let mut toSub: i32;
        let mut tmp: i32;
        let mut inInit = true;
        let mut outInit = true;
        let mut eta: f64;
        let mut normalized: f64;
        let mut diff: f64;
        let mut phi: f64;
        let mut delta: f64;
        let mut toReset: LaneIdVector = Vec::new();

        self.carsIn = 0;
        self.carsOut = 0;
        self.inTarget = 0;
        self.notTarget = 0;

        let sensors: Ptr<MSSOTLE2Sensors> = self.get_count_sensors();
        let myLanes = self.myLanes.clone();
        let myLinks = self.myLinks.clone();

        // Search the incoming lane to get the count of the vehicles passed. [IN]
        for laneVector in myLanes.iter() {
            for &currentLane in laneVector.iter() {
                // Map to avoid check the lane for every possible direction
                if !*self.laneCheck.entry(currentLane).or_insert(false) {
                    // Get the vehicles passed from this lane.
                    count = sensors.m().get_passed_veh(currentLane.get_id().to_string(), false);
                    // Increment the global count of the cars passed through the tl
                    self.carsIn += count;
                    // Set to true to skip similar lane since there's just one sensor
                    self.laneCheck.insert(currentLane, true);
                }
            }
        }

        // Search the outgoing lane to get the count of the vehicles passed. [OUT]
        // We use the links to get the respective lane id.
        for linkVector in myLinks.iter() {
            for link in linkVector.iter() {
                let currentLane = link.get_lane();
                // Map to avoid check the lane for every possible direction
                if !*self.laneCheck.entry(currentLane).or_insert(false) {
                    // Get the vehicles passed from this lane.
                    count = sensors.m().get_passed_veh(currentLane.get_id().to_string(), true);
                    // Increment the global count of the cars passed through the tl
                    self.carsOut += count;

                    // Since there's no output target lanes we check here the minimum number of
                    // cars passed though the tl. This ahs to be done to all the output lanes since cars can go
                    // in any direction from a target lane. If a direction isn't reachable the sensor count will be 0.
                    // This is done to update the sensorCount value in order to don't make it grow too much.
                    if count != 0 {
                        toReset.push(currentLane.get_id().to_string());
                        if outInit {
                            minOut = count;
                            outInit = false;
                        } else if count <= minOut {
                            minOut = count;
                        }
                    }
                    // Set to true to skip similar lane since there's just one sensor
                    self.laneCheck.insert(currentLane, true);
                }
            }
        }
        // Reset the map to check again all the lane on the next commit.
        self.reset_lane_check();

        // We retrieve the minimum number of cars passed from the target lanes.
        let targetLanes = self.targetLanes.clone();
        for laneId in targetLanes.iter() {
            let lane = laneId.clone();
            tmp = sensors.m().get_passed_veh(lane.clone(), false);
            self.inTarget += tmp;
            if inInit && tmp != 0 {
                minIn = tmp;
                inInit = false;
            }
            if tmp < minIn && tmp != 0 {
                minIn = tmp;
            }
            if tmp != 0 {
                toReset.push(lane);
            }
        }

        // The cars not on a target lane counted as in.
        self.notTarget = self.carsIn - self.inTarget;

        // Calculate the min beetween the min number of cars entered the tl (minIn) and the
        // ones that have exit the tl (minOut)
        toSub = std::cmp::min(minIn, minOut);

        // Subtract the value to all the sensor on the target lanes.
        while let Some(laneId) = toReset.pop() {
            sensors.m().subtract_passed_veh(laneId, toSub);
        }

        // Normalized to 1
        diff = (self.inTarget - self.carsOut) as f64;
        normalized = diff / (self.inTarget as f64);

        // Analize difference to return an appropriate eta to reinforce/forget the policies.
        // IN > OUT
        if self.inTarget > self.carsOut {
            if self.carsOut == 0 {
                // We're in Congestion but not for long so we don't do nothing. When we reach max steps for
                // Congestion the evaluation of eta is skipped and we force a forget of the policy
                if self.myCurrentPolicy.m().get_name() == "Congestion" {
                    eta = 0.0;
                } else {
                    // vehicles aren't going out and we've additional vehicle on a red lane. We set
                    // eta to -1 to forget
                    eta = -1.0;
                }
            } else {
                // Forget - Amplify to R
                phi = self.calculate_phi(self.notTarget);
                eta = -normalized * (1.0 / phi);
                if eta < -1.0 {
                    eta = -1.0;
                }
            }
        }
        // IN = OUT
        else if self.inTarget == self.carsOut {
            // Can't say nothing
            if self.inTarget == 0 {
                eta = 0.0;
            }
            // Reinforce - Attenuate to R
            // Normalized = 0 --> use delta = 1-1/IN
            else {
                delta = self.calculate_phi(self.inTarget);
                phi = self.calculate_phi(self.notTarget);
                eta = delta * phi;
                if eta > 1.0 {
                    eta = 1.0;
                }
            }
        }
        // IN < OUT
        else {
            // Can't say nothing
            if self.inTarget == 0 {
                eta = 0.0;
            }
            // Reinforce - Attenuate to R
            else {
                phi = self.calculate_phi(self.notTarget);
                diff = (self.inTarget - self.carsOut) as f64;
                normalized = diff / (self.carsOut as f64);
                eta = normalized * phi;
                if eta > 1.0 {
                    eta = 1.0;
                }
            }
        }
        eta
    }

    /// C++: `double MSSwarmTrafficLightLogic::calculateEtaRatio()` [protected]
    // double MSSwarmTrafficLightLogic::calculateEtaRatio() {
    //     MSLane* currentLane = nullptr;
    //     int count = 0, minIn = 0, minOut = 0, toSub, tmp;
    //     bool inInit = true, outInit = true;
    //     double eta, ratio, phi, normalized, delta;
    //     LaneIdVector toReset;
    //
    //     carsIn = 0;
    //     carsOut = 0;
    //     inTarget = 0;
    //     notTarget = 0;
    //
    //     MSSOTLE2Sensors* sensors = (MSSOTLE2Sensors*) getCountSensors();
    //
    //     // Search the incoming lane to get the count of the vehicles passed. [IN]
    //     for (MSTrafficLightLogic::LaneVectorVector::const_iterator laneVector = myLanes.begin();
    //             laneVector != myLanes.end(); laneVector++) {
    //         for (MSTrafficLightLogic::LaneVector::const_iterator lane = laneVector->begin(); lane != laneVector->end();
    //                 lane++) {
    //             currentLane = (*lane);
    //
    //             // Map to avoid check the lane for every possible direction
    //             if (laneCheck[currentLane] == false) {
    //                 // Get the vehicles passed from this lane.
    //                 count = sensors->getPassedVeh(currentLane->getID(), false);
    //
    // #ifdef SWARM_DEBUG
    //                 std::ostringstream cars_str;
    //                 cars_str << "Lane " << currentLane->getID() << ": vehicles entered - " << count;
    //                 WRITE_MESSAGE(time2string(MSNet::getInstance()->getCurrentTimeStep()) + " MSSwarmTrafficLightLogic::calculateEta::" + cars_str.str());
    // #endif
    //
    //                 // Increment the global count of the cars passed through the tl
    //                 carsIn += count;
    //                 // Set to true to skip similar lane since there's just one sensor
    //                 laneCheck[currentLane] = true;
    //             }
    //         }
    //     }
    //
    //     // Search the outgoing lane to get the count of the vehicles passed. [OUT]
    //     // We use the links to get the respective lane id.
    //     for (MSTrafficLightLogic::LinkVectorVector::const_iterator linkVector = myLinks.begin();
    //             linkVector != myLinks.end(); linkVector++) {
    //         for (MSTrafficLightLogic::LinkVector::const_iterator link = linkVector->begin(); link != linkVector->end();
    //                 link++) {
    //             currentLane = (*link)->getLane();
    //
    //             // Map to avoid check the lane for every possible direction
    //             if (laneCheck[currentLane] == false) {
    //                 // Get the vehicles passed from this lane.
    //                 count = sensors->getPassedVeh(currentLane->getID(), true);
    //
    // #ifdef SWARM_DEBUG
    //                 std::ostringstream cars_str;
    //                 cars_str << "Lane " << currentLane->getID() << ": vehicles gone out- " << count;
    //                 WRITE_MESSAGE(time2string(MSNet::getInstance()->getCurrentTimeStep()) + " MSSwarmTrafficLightLogic::calculateEta::" + cars_str.str());
    // #endif
    //                 // Increment the global count of the cars passed through the tl
    //                 carsOut += count;
    //
    //                 // Since there's no output target lanes we check here the minimum number of
    //                 // cars passed though the tl. This has to be done to all the output lanes since cars can go
    //                 // in any direction from a target lane. If a direction isn't reachable the sensor count will be 0.
    //                 // This is done to update the sensorCount value in order to don't make it grow too much.
    //                 if (count != 0) {
    //                     toReset.push_back(currentLane->getID());
    //                     if (outInit) {
    //                         minOut = count;
    //                         outInit = false;
    //                     } else if (count <= minOut) {
    //                         minOut = count;
    //                     }
    //                 }
    //
    //                 // Set to true to skip similar lane since there's just one sensor
    //                 laneCheck[currentLane] = true;
    //             }
    //         }
    //     }
    //     // Reset the map to check again all the lane on the next commit.
    //     resetLaneCheck();
    //
    //     // We retrieve the minimum number of cars passed from the target lanes.
    //     for (LaneIdVector::const_iterator laneId = targetLanes.begin(); laneId < targetLanes.end(); laneId++) {
    //         std::string lane = (*laneId);
    //         tmp = sensors->getPassedVeh(lane, false);
    //         inTarget += tmp;
    //         if (inInit && tmp != 0) {
    //             minIn = tmp;
    //             inInit = false;
    //         }
    //         if (tmp < minIn && tmp != 0) {
    //             minIn = tmp;
    //         }
    //         if (tmp != 0) {
    //             toReset.push_back(lane);
    //         }
    // #ifdef SWARM_DEBUG
    //         std::ostringstream cars_str;
    //         cars_str << "Lane " << lane << " passed: " << tmp;
    //         WRITE_MESSAGE(time2string(MSNet::getInstance()->getCurrentTimeStep()) + " MSSwarmTrafficLightLogic::calculateEta::" + cars_str.str());
    // #endif
    //     }
    //
    //     // The cars not on a target lane counted as in.
    //     notTarget = carsIn - inTarget;
    //
    //     // Calculate the min beetween the min number of cars entered the tl (minIn) and the
    //     // ones that have exit the tl (minOut)
    //     toSub = std::min(minIn, minOut);
    //
    //     // Subtract the value to all the sensor on the target lanes.
    //     while (!toReset.empty()) {
    //         std::string laneId = toReset.back();
    //         toReset.pop_back();
    //         sensors->subtractPassedVeh(laneId, toSub);
    //     }
    //
    //     //Normalized to 1
    //     if (carsOut != 0) {
    //         ratio = ((double) inTarget) / carsOut;
    //         normalized = ratio / (inTarget + carsOut);
    //     } else {
    //         ratio = std::numeric_limits<double>::infinity();
    //         normalized = std::numeric_limits<double>::infinity();
    //     }
    //
    // #ifdef SWARM_DEBUG
    //     std::ostringstream final_str;
    //     final_str << "Total cars in lanes: " << carsIn << " Total cars out: " << carsOut << " Ratio: " << ratio << " Pure eta: " << normalized;
    //     WRITE_MESSAGE(time2string(MSNet::getInstance()->getCurrentTimeStep()) + " MSSwarmTrafficLightLogic::calculateEta::" + final_str.str());
    //     std::ostringstream eta_str;
    //     eta_str << "IN:" << inTarget << ". OUT:" << carsOut << " R:" << notTarget;
    //     WRITE_MESSAGE(time2string(MSNet::getInstance()->getCurrentTimeStep()) + " MSSwarmTrafficLightLogic::calculateEta::" + eta_str.str());
    //     std::ostringstream eta_str;
    //     eta_str << "Min found:" << toSub << ". MinIn:" << minIn << " MinOut:" << minOut;
    //     WRITE_MESSAGE(time2string(MSNet::getInstance()->getCurrentTimeStep()) + " MSSwarmTrafficLightLogic::calculateEta::" + eta_str.str());
    // #endif
    //     // Analize ratio to return an appropriate eta to reinforce/forget the policies.
    //
    //     // IN > OUT
    //     if (inTarget > carsOut) {
    //         if (carsOut == 0) {
    //             // we're in Congestion but not for long so we don't do nothing. When we reach max steps for
    //             // Congestion the evaluation of eta is skipped and we force a forget of the policy
    //             if (myCurrentPolicy->getName().compare("Congestion") == 0) {
    //                 eta = 0;
    //             }
    //             // vehicles aren't going out and we've additional vehicle on a red lane. We set
    //             // eta to -1 to forget
    //             else {
    //                 eta = -1;
    //             }
    //         } else {
    //             // Forget according to the ratio. Amplify due to the cars in the red lanes
    //             phi = calculatePhi(notTarget);
    //             eta = (-(normalized) * (1 / phi));
    //             if (eta < -1.0) {
    //                 eta = -1.0;
    //             }
    //         }
    //     }
    //     // IN = OUT
    //     else if (inTarget == carsOut) {
    //         // We can't say nothing.
    //         if (inTarget == 0) {
    //             eta = 0;
    //         }
    //         // Reinforce - Attenuate to R
    //         // same number of vehicles that are getting IN is getting OUT
    //         // Normalized = 1/TOT ---> change to delta = 1-1/IN
    //         else {
    //             delta = calculatePhi(inTarget);
    //             phi = calculatePhi(notTarget);
    //             eta = delta * phi;
    //             if (eta > 1.0) {
    //                 eta = 1.0;
    //             }
    //         }
    //     }
    //     // IN < OUT
    //     else {
    //         // We can't say nothing.
    //         if (inTarget == 0) {
    //             eta = 0;
    //         }
    //
    //         // There was a queue and now cars are getting over it
    //         // There're vehicles on the red lanes (R)
    //         // We reinforce and attenuate according to R
    //         else {
    //             phi = calculatePhi(notTarget);
    //             eta = (normalized) * phi;
    //             if (eta > 1.0) {
    //                 eta = 1.0;
    //             }
    //         }
    //     }
    //
    // #ifdef SWARM_DEBUG
    //     std::ostringstream eta_str;
    //     eta_str << "Eta Normalized: " << eta << ".";
    //     WRITE_MESSAGE(time2string(MSNet::getInstance()->getCurrentTimeStep()) + " MSSwarmTrafficLightLogic::calculateEta::" + eta_str.str());
    // #endif
    //     return eta;
    //
    // }
    #[allow(unused_variables)]
    pub fn calculate_eta_ratio(&mut self) -> f64 {
        let mut count: i32 = 0;
        let mut minIn: i32 = 0;
        let mut minOut: i32 = 0;
        let mut toSub: i32;
        let mut tmp: i32;
        let mut inInit = true;
        let mut outInit = true;
        let mut eta: f64;
        let mut ratio: f64;
        let mut phi: f64;
        let mut normalized: f64;
        let mut delta: f64;
        let mut toReset: LaneIdVector = Vec::new();

        self.carsIn = 0;
        self.carsOut = 0;
        self.inTarget = 0;
        self.notTarget = 0;

        let sensors: Ptr<MSSOTLE2Sensors> = self.get_count_sensors();
        let myLanes = self.myLanes.clone();
        let myLinks = self.myLinks.clone();

        // Search the incoming lane to get the count of the vehicles passed. [IN]
        for laneVector in myLanes.iter() {
            for &currentLane in laneVector.iter() {
                // Map to avoid check the lane for every possible direction
                if !*self.laneCheck.entry(currentLane).or_insert(false) {
                    // Get the vehicles passed from this lane.
                    count = sensors.m().get_passed_veh(currentLane.get_id().to_string(), false);
                    // Increment the global count of the cars passed through the tl
                    self.carsIn += count;
                    // Set to true to skip similar lane since there's just one sensor
                    self.laneCheck.insert(currentLane, true);
                }
            }
        }

        // Search the outgoing lane to get the count of the vehicles passed. [OUT]
        // We use the links to get the respective lane id.
        for linkVector in myLinks.iter() {
            for link in linkVector.iter() {
                let currentLane = link.get_lane();
                // Map to avoid check the lane for every possible direction
                if !*self.laneCheck.entry(currentLane).or_insert(false) {
                    // Get the vehicles passed from this lane.
                    count = sensors.m().get_passed_veh(currentLane.get_id().to_string(), true);
                    // Increment the global count of the cars passed through the tl
                    self.carsOut += count;

                    // Since there's no output target lanes we check here the minimum number of
                    // cars passed though the tl. This has to be done to all the output lanes since cars can go
                    // in any direction from a target lane. If a direction isn't reachable the sensor count will be 0.
                    // This is done to update the sensorCount value in order to don't make it grow too much.
                    if count != 0 {
                        toReset.push(currentLane.get_id().to_string());
                        if outInit {
                            minOut = count;
                            outInit = false;
                        } else if count <= minOut {
                            minOut = count;
                        }
                    }

                    // Set to true to skip similar lane since there's just one sensor
                    self.laneCheck.insert(currentLane, true);
                }
            }
        }
        // Reset the map to check again all the lane on the next commit.
        self.reset_lane_check();

        // We retrieve the minimum number of cars passed from the target lanes.
        let targetLanes = self.targetLanes.clone();
        for laneId in targetLanes.iter() {
            let lane = laneId.clone();
            tmp = sensors.m().get_passed_veh(lane.clone(), false);
            self.inTarget += tmp;
            if inInit && tmp != 0 {
                minIn = tmp;
                inInit = false;
            }
            if tmp < minIn && tmp != 0 {
                minIn = tmp;
            }
            if tmp != 0 {
                toReset.push(lane);
            }
        }

        // The cars not on a target lane counted as in.
        self.notTarget = self.carsIn - self.inTarget;

        // Calculate the min beetween the min number of cars entered the tl (minIn) and the
        // ones that have exit the tl (minOut)
        toSub = std::cmp::min(minIn, minOut);

        // Subtract the value to all the sensor on the target lanes.
        while let Some(laneId) = toReset.pop() {
            sensors.m().subtract_passed_veh(laneId, toSub);
        }

        // Normalized to 1
        if self.carsOut != 0 {
            ratio = (self.inTarget as f64) / (self.carsOut as f64);
            normalized = ratio / ((self.inTarget + self.carsOut) as f64);
        } else {
            ratio = f64::INFINITY;
            normalized = f64::INFINITY;
        }

        // Analize ratio to return an appropriate eta to reinforce/forget the policies.
        // IN > OUT
        if self.inTarget > self.carsOut {
            if self.carsOut == 0 {
                // we're in Congestion but not for long so we don't do nothing. When we reach max steps for
                // Congestion the evaluation of eta is skipped and we force a forget of the policy
                if self.myCurrentPolicy.m().get_name() == "Congestion" {
                    eta = 0.0;
                }
                // vehicles aren't going out and we've additional vehicle on a red lane. We set
                // eta to -1 to forget
                else {
                    eta = -1.0;
                }
            } else {
                // Forget according to the ratio. Amplify due to the cars in the red lanes
                phi = self.calculate_phi(self.notTarget);
                eta = -(normalized) * (1.0 / phi);
                if eta < -1.0 {
                    eta = -1.0;
                }
            }
        }
        // IN = OUT
        else if self.inTarget == self.carsOut {
            // We can't say nothing.
            if self.inTarget == 0 {
                eta = 0.0;
            }
            // Reinforce - Attenuate to R
            // same number of vehicles that are getting IN is getting OUT
            // Normalized = 1/TOT ---> change to delta = 1-1/IN
            else {
                delta = self.calculate_phi(self.inTarget);
                phi = self.calculate_phi(self.notTarget);
                eta = delta * phi;
                if eta > 1.0 {
                    eta = 1.0;
                }
            }
        }
        // IN < OUT
        else {
            // We can't say nothing.
            if self.inTarget == 0 {
                eta = 0.0;
            }
            // There was a queue and now cars are getting over it
            // There're vehicles on the red lanes (R)
            // We reinforce and attenuate according to R
            else {
                phi = self.calculate_phi(self.notTarget);
                eta = normalized * phi;
                if eta > 1.0 {
                    eta = 1.0;
                }
            }
        }
        eta
    }

    /// C++: `void MSSwarmTrafficLightLogic::resetLaneCheck()` [protected]
    // void MSSwarmTrafficLightLogic::resetLaneCheck() {
    //
    //     MSLane* currentLane = nullptr;
    //
    //     // reset both the input and the output lanes.
    //     for (MSTrafficLightLogic::LaneVectorVector::const_iterator laneVector = myLanes.begin();
    //             laneVector != myLanes.end(); laneVector++) {
    //
    //         for (MSTrafficLightLogic::LaneVector::const_iterator lane = laneVector->begin(); lane != laneVector->end();
    //                 lane++) {
    //             currentLane = (*lane);
    //             laneCheck[currentLane] = false;
    //         }
    //     }
    //
    //     for (MSTrafficLightLogic::LinkVectorVector::const_iterator linkVector = myLinks.begin();
    //             linkVector != myLinks.end(); linkVector++) {
    //         for (MSTrafficLightLogic::LinkVector::const_iterator link = linkVector->begin(); link != linkVector->end();
    //                 link++) {
    //             currentLane = (*link)->getLane();
    //             laneCheck[currentLane] = false;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn reset_lane_check(&mut self) {
        let myLanes = self.myLanes.clone();
        let myLinks = self.myLinks.clone();

        // reset both the input and the output lanes.
        for laneVector in myLanes.iter() {
            for &currentLane in laneVector.iter() {
                self.laneCheck.insert(currentLane, false);
            }
        }

        for linkVector in myLinks.iter() {
            for link in linkVector.iter() {
                let currentLane = link.get_lane();
                self.laneCheck.insert(currentLane, false);
            }
        }
    }

    /// C++: `void MSSwarmTrafficLightLogic::choosePolicy(double, double, double, double)` [protected]
    // void MSSwarmTrafficLightLogic::choosePolicy(double phero_in, double phero_out, double dispersion_in,
    //         double dispersion_out) {
    //     if (m_useVehicleTypesWeights) {
    //         for (std::vector<MSSOTLPolicy*>::iterator it = myPolicies.begin(); it != myPolicies.end(); ++it) {
    //             if (it.operator * ()->getName() == "Phase") {
    //                 activate(*it);
    //                 return;
    //             }
    //         }
    //     }
    //     std::vector<double> thetaStimuli;
    //     double thetaSum = 0.0;
    //     // Compute stimulus for each policy
    //     for (int i = 0; i < (int)myPolicies.size(); i++) {
    //         double stimulus = myPolicies[i]->computeDesirability(phero_in, phero_out, dispersion_in, dispersion_out);
    //         double thetaStimulus = pow(stimulus, 2) / (pow(stimulus, 2) + pow(myPolicies[i]->getThetaSensitivity(), 2));
    //
    //         thetaStimuli.push_back(thetaStimulus);
    //         thetaSum += thetaStimulus;
    //
    // //		ANALYSIS_DBG(
    // #ifdef SWARM_DEBUG
    //         ostringstream so_str;
    //         so_str << " policy " << myPolicies[i]->getName() << " stimulus " << stimulus << " pow(stimulus,2) " << pow(stimulus, 2) << " pow(Threshold,2) " << pow(myPolicies[i]->getThetaSensitivity(), 2) << " thetaStimulus " << thetaStimulus << " thetaSum " << thetaSum << " TL " << getID();
    //         WRITE_MESSAGE("MSSwarmTrafficLightLogic::choosePolicy::" + so_str.str());
    // #endif
    //     }
    //
    //     // Compute a random value between 0 and the sum of the thetaSum
    // //	double r = RandHelper::rand(RAND_MAX);
    // //	r = r / RAND_MAX * thetaSum;
    //     double r = RandHelper::rand((double)thetaSum);
    //
    //     double partialSum = 0;
    //     for (int i = 0; i < (int)myPolicies.size(); i++) {
    //         partialSum += thetaStimuli[i];
    //
    // //		ANALYSIS_DBG(
    // #ifdef SWARM_DEBUG
    //         ostringstream aao_str;
    //         aao_str << " policy " << myPolicies[i]->getName() << " partialSum " << partialSum << " thetaStimuls " << thetaStimuli[i] << " r " << r << " TL " << getID();
    //         WRITE_MESSAGE("MSSwarmTrafficLightLogic::choosePolicy::" + aao_str.str());
    // #endif
    //         if (partialSum >= r) {
    //             activate(myPolicies[i]);
    //             break;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn choose_policy(&mut self, phero_in: f64, phero_out: f64, dispersion_in: f64, dispersion_out: f64) {
        if self.m_useVehicleTypesWeights {
            let policies = self.myPolicies.clone();
            for &it in policies.iter() {
                if it.m().get_name() == "Phase" {
                    self.activate(it);
                    return;
                }
            }
        }
        let mut thetaStimuli: Vec<f64> = Vec::new();
        let mut thetaSum: f64 = 0.0;
        // Compute stimulus for each policy
        for i in 0..self.myPolicies.len() {
            let policy = self.myPolicies[i];
            let stimulus = policy.m().compute_desirability(phero_in, phero_out, dispersion_in, dispersion_out);
            let thetaStimulus = stimulus.powf(2.0) / (stimulus.powf(2.0) + policy.m().get_theta_sensitivity().powf(2.0));
            thetaStimuli.push(thetaStimulus);
            thetaSum += thetaStimulus;
        }

        // Compute a random value between 0 and the sum of the thetaSum
        let r = utils::common::rand_helper::with_default(|rng| rng.rand_max(thetaSum));

        let mut partialSum: f64 = 0.0;
        for i in 0..self.myPolicies.len() {
            partialSum += thetaStimuli[i];
            if partialSum >= r {
                let chosen = self.myPolicies[i];
                self.activate(chosen);
                break;
            }
        }
    }

    /// C++: `void MSSwarmTrafficLightLogic::choosePolicy(double, double)` [protected]
    // void MSSwarmTrafficLightLogic::choosePolicy(double phero_in, double phero_out) {
    //     choosePolicy(phero_in, phero_out, 0, 0);
    // }
    #[allow(unused_variables)]
    pub fn choose_policy_2(&mut self, phero_in: f64, phero_out: f64) {
        self.choose_policy(phero_in, phero_out, 0.0, 0.0);
    }

    /// C++: `std::string MSSwarmTrafficLightLogic::getPoliciesParam()` [protected]
    // std::string getPoliciesParam() {
    //         return getParameter("POLICIES", "Platoon;Phase;Marching;Congestion");
    //     }
    #[allow(unused_variables)]
    pub fn get_policies_param(&mut self) -> String {
        self.get_parameter("POLICIES", "Platoon;Phase;Marching;Congestion".to_string())
    }

    /// C++: `int MSSwarmTrafficLightLogic::getReinforcementMode()` [protected]
    // int getReinforcementMode() {
    //         return StringUtils::toInt(getParameter("REIMODE", "0"));
    //     }
    #[allow(unused_variables)]
    pub fn get_reinforcement_mode(&mut self) -> i32 {
        utils::common::string_utils::to_int(&self.get_parameter("REIMODE", "0".to_string())).unwrap()
    }

    /// C++: `void MSSwarmTrafficLightLogic::initScaleFactorDispersionIn(int)` [protected]
    // void initScaleFactorDispersionIn(int lanes_in) {
    //         std::vector<double> phero_values;
    //
    //         for (int i = 0; i < lanes_in / 2; i++) {
    //             phero_values.push_back(getPheroMaxVal());
    //         }
    //         for (int i = lanes_in / 2; i < lanes_in; i++) {
    //             phero_values.push_back(0.0);
    //         }
    //
    //         double sum_avg_tmp = 0;
    //
    //         for (int i = 0; i < (int)phero_values.size(); i++) {
    //             sum_avg_tmp += phero_values[i];
    //         }
    //
    //         const double mean = sum_avg_tmp / (double)phero_values.size();
    //
    //         double sum_dev_tmp = 0;
    //         for (int i = 0; i < (int)phero_values.size(); i++) {
    //             sum_dev_tmp += pow(phero_values[i] - mean, 2);
    //         }
    //
    //         const double deviation = sqrt(sum_dev_tmp / (double)phero_values.size());
    //
    //         scaleFactorDispersionIn = getPheroMaxVal() / deviation;
    //     }
    #[allow(unused_variables)]
    pub fn init_scale_factor_dispersion_in(&mut self, lanes_in: i32) {
        let mut phero_values: Vec<f64> = Vec::new();
        for _ in 0..lanes_in / 2 {
            phero_values.push(self.get_phero_max_val());
        }
        for _ in (lanes_in / 2)..lanes_in {
            phero_values.push(0.0);
        }
        let mut sum_avg_tmp: f64 = 0.0;
        for v in phero_values.iter() {
            sum_avg_tmp += *v;
        }
        let mean = sum_avg_tmp / (phero_values.len() as f64);
        let mut sum_dev_tmp: f64 = 0.0;
        for v in phero_values.iter() {
            sum_dev_tmp += (*v - mean).powf(2.0);
        }
        let deviation = (sum_dev_tmp / (phero_values.len() as f64)).sqrt();
        self.scaleFactorDispersionIn = self.get_phero_max_val() / deviation;
    }

    /// C++: `void MSSwarmTrafficLightLogic::initScaleFactorDispersionOut(int)` [protected]
    // void initScaleFactorDispersionOut(int lanes_out) {
    //         std::vector<double> phero_values;
    //
    //         for (int i = 0; i < lanes_out / 2; i++) {
    //             phero_values.push_back(getPheroMaxVal());
    //         }
    //         for (int i = lanes_out / 2; i < lanes_out; i++) {
    //             phero_values.push_back(0.0);
    //         }
    //
    //         double sum_avg_tmp = 0;
    //         for (int i = 0; i < (int)phero_values.size(); i++) {
    //             sum_avg_tmp += phero_values[i];
    //         }
    //         const double mean = sum_avg_tmp / (double)phero_values.size();
    //
    //         double sum_dev_tmp = 0;
    //
    //         for (int i = 0; i < (int)phero_values.size(); i++) {
    //             sum_dev_tmp += pow(phero_values[i] - mean, 2);
    //         }
    //
    //         const double deviation = sqrt(sum_dev_tmp / (double)phero_values.size());
    //
    //         scaleFactorDispersionOut = getPheroMaxVal() / deviation;
    //     }
    #[allow(unused_variables)]
    pub fn init_scale_factor_dispersion_out(&mut self, lanes_out: i32) {
        let mut phero_values: Vec<f64> = Vec::new();
        for _ in 0..lanes_out / 2 {
            phero_values.push(self.get_phero_max_val());
        }
        for _ in (lanes_out / 2)..lanes_out {
            phero_values.push(0.0);
        }
        let mut sum_avg_tmp: f64 = 0.0;
        for v in phero_values.iter() {
            sum_avg_tmp += *v;
        }
        let mean = sum_avg_tmp / (phero_values.len() as f64);
        let mut sum_dev_tmp: f64 = 0.0;
        for v in phero_values.iter() {
            sum_dev_tmp += (*v - mean).powf(2.0);
        }
        let deviation = (sum_dev_tmp / (phero_values.len() as f64)).sqrt();
        self.scaleFactorDispersionOut = self.get_phero_max_val() / deviation;
    }

    /// C++: `bool MSSwarmTrafficLightLogic::allowLine(MSLane *)` [protected]
    /// Check if a lane is allowed to be added to the maps pheromoneInputLanes and pheromoneOutputLanes Control in this function if the lane is a walking area, a crossing, or if only pedestrian are allowed. Return true if the lane has to be added, false otherwise.
    // bool MSSwarmTrafficLightLogic::allowLine(MSLane* lane) {
    //     //No walking areas
    //     if (lane->isWalkingArea()) {
    //         return false;
    //     }
    //     //No pedestrian crossing
    //     if (lane->isCrossing()) {
    //         return false;
    //     }
    //     //No pedestrian only lanes
    //     if (lane->getPermissions() == SVC_PEDESTRIAN) {
    //         return false;
    //     }
    //     //No bicycle only lanes
    //     if (lane->getPermissions() == SVC_BICYCLE) {
    //         return false;
    //     }
    //     //No pedestrian and bicycle only lanes
    //     if (lane->getPermissions() == (SVC_PEDESTRIAN | SVC_BICYCLE)) {
    //         return false;
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn allow_line(&mut self, arg0: Ptr<MSLane>) -> bool {
        // No walking areas
        if arg0.is_walking_area() {
            return false;
        }
        // No pedestrian crossing
        if arg0.is_crossing() {
            return false;
        }
        // No pedestrian only lanes
        if arg0.get_permissions() == SVC_PEDESTRIAN {
            return false;
        }
        // No bicycle only lanes
        if arg0.get_permissions() == SVC_BICYCLE {
            return false;
        }
        // No pedestrian and bicycle only lanes
        if arg0.get_permissions() == (SVC_PEDESTRIAN | SVC_BICYCLE) {
            return false;
        }
        true
    }

    /// C++: `std::string MSSwarmTrafficLightLogic::getLaneLightState(const std::string &)` [protected]
    // std::string MSSwarmTrafficLightLogic::getLaneLightState(const std::string& laneId) {
    //     std::string laneState = "";
    //     if (m_laneIndexMap.find(laneId) != m_laneIndexMap.end()) {
    //         std::string state = getCurrentPhaseDef().getState();
    //         for (std::vector<int>::const_iterator it = m_laneIndexMap[laneId].begin(); it != m_laneIndexMap[laneId].end(); ++it) {
    //             laneState += state[*it];
    //         }
    //     }
    //     return laneState;
    // }
    #[allow(unused_variables)]
    pub fn get_lane_light_state(&mut self, laneId: &str) -> String {
        let mut laneState = String::new();
        if self.m_laneIndexMap.contains_key(laneId) {
            let state = self.get_current_phase_def().get_state().to_string();
            for &idx in self.m_laneIndexMap[laneId].clone().iter() {
                laneState.push(state.as_bytes()[idx as usize] as char);
            }
        }
        laneState
    }

}

