//! Port of microsim/trigger/MSLaneSpeedTrigger.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Changes the speed allowed on a set of lanes
/// C++ class `MSLaneSpeedTrigger` (microsim/trigger/MSLaneSpeedTrigger.h)
#[repr(C)]
pub struct MSLaneSpeedTrigger {
    /// C++ base class `Named`
    pub base: Named,
    /// C++ base class `SUMOSAXHandler`
    pub base2: Unmapped /* SUMOSAXHandler */,
    /// the lane the trigger is responsible for
    pub myDestLanes: Vec<Ptr<MSLane>>,
    /// The original speed allowed on the lanes
    pub myDefaultSpeed: f64,
    /// The original friction on the lanes
    pub myDefaultFriction: f64,
    /// The information whether the read speed shall be overridden
    pub myAmOverriding: bool,
    /// The speed to use if overriding the read speed
    pub mySpeedOverrideValue: f64,
    pub myLoadedSpeeds: Vec<(SUMOTime, f64)>,
    pub myLoadedFrictions: Vec<(SUMOTime, f64)>,
    pub myCurrentSpeedEntry: Unmapped /* std::vector<std::pair<SUMOTime, double>>::iterator */,
    pub myCurrentFrictionEntry: Unmapped /* std::vector<std::pair<SUMOTime, double>>::iterator */,
    /// The information whether init was called
    pub myDidInit: bool,
}

impl std::ops::Deref for MSLaneSpeedTrigger {
    type Target = Named;
    fn deref(&self) -> &Named {
        &self.base
    }
}
impl std::ops::DerefMut for MSLaneSpeedTrigger {
    fn deref_mut(&mut self) -> &mut Named {
        &mut self.base
    }
}

// C++ static members of MSLaneSpeedTrigger (port as module-level statics, see AGENTS.md):
//   static std::map<std::string, MSLaneSpeedTrigger *> MSLaneSpeedTrigger::myInstances;

impl MSLaneSpeedTrigger {
    /// C++: `void MSLaneSpeedTrigger::MSLaneSpeedTrigger(const std::string &, const std::vector<MSLane *> &, const std::string &)`
    /// Constructor
    // MSLaneSpeedTrigger::MSLaneSpeedTrigger(const std::string& id,
    //                                        const std::vector<MSLane*>& destLanes,
    //                                        const std::string& file) :
    //     Named(id),
    //     SUMOSAXHandler(file),
    //     myDestLanes(destLanes),
    //     myDefaultSpeed(destLanes[0]->getSpeedLimit()),
    //     myDefaultFriction(destLanes[0]->getFrictionCoefficient()),
    //     myAmOverriding(false),
    //     mySpeedOverrideValue(destLanes[0]->getSpeedLimit()),
    //     myDidInit(false) {
    //     myInstances[id] = this;
    //     if (file != "") {
    //         if (!XMLSubSys::runParser(*this, file)) {
    //             throw ProcessError();
    //         }
    //         if (!myDidInit) {
    //             init();
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(id: &str, destLanes: &Vec<Ptr<MSLane>>, file: &str) -> Self {
        todo!("port MSLaneSpeedTrigger::MSLaneSpeedTrigger")
    }

    /// C++: `void MSLaneSpeedTrigger::~MSLaneSpeedTrigger() noexcept` [virtual]
    /// Destructor
    // MSLaneSpeedTrigger::~MSLaneSpeedTrigger() {
    //     myInstances.erase(getID());
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSLaneSpeedTrigger::~MSLaneSpeedTrigger")
    }

    /// C++: `SUMOTime MSLaneSpeedTrigger::executeSpeedChange(SUMOTime)`
    /// Executes a switch command
    // SUMOTime
    // MSLaneSpeedTrigger::executeSpeedChange(SUMOTime currentTime) {
    //     return processCommand(true, currentTime);
    // }
    #[allow(unused_variables)]
    pub fn execute_speed_change(&mut self, currentTime: SUMOTime) -> SUMOTime {
        todo!("port MSLaneSpeedTrigger::executeSpeedChange")
    }

    /// C++: `SUMOTime MSLaneSpeedTrigger::executeFrictionChange(SUMOTime)`
    // SUMOTime
    // MSLaneSpeedTrigger::executeFrictionChange(SUMOTime currentTime) {
    //     const double friction = getCurrentFriction();
    //     for (MSLane* const lane : myDestLanes) {
    //         lane->setFrictionCoefficient(friction);
    //     }
    //     if (myCurrentFrictionEntry != myLoadedFrictions.end()) {
    //         ++myCurrentFrictionEntry;
    //     }
    //     if (myCurrentFrictionEntry != myLoadedFrictions.end()) {
    //         return myCurrentFrictionEntry->first - currentTime;
    //     }
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn execute_friction_change(&mut self, currentTime: SUMOTime) -> SUMOTime {
        todo!("port MSLaneSpeedTrigger::executeFrictionChange")
    }

    /// C++: `SUMOTime MSLaneSpeedTrigger::processCommand(bool, SUMOTime)`
    // SUMOTime
    // MSLaneSpeedTrigger::processCommand(bool move2next, SUMOTime currentTime) {
    //     const double speed = getCurrentSpeed();
    //     const bool altered = speed != myDefaultSpeed;
    //     for (MSLane* const lane : myDestLanes) {
    //         lane->setMaxSpeed(speed, altered);
    //     }
    //     if (!move2next) {
    //         // changed from the gui
    //         return 0;
    //     }
    //     if (myCurrentSpeedEntry != myLoadedSpeeds.end()) {
    //         ++myCurrentSpeedEntry;
    //     }
    //     if (myCurrentSpeedEntry != myLoadedSpeeds.end()) {
    //         return myCurrentSpeedEntry->first - currentTime;
    //     }
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn process_command(&mut self, move2next: bool, currentTime: SUMOTime) -> SUMOTime {
        todo!("port MSLaneSpeedTrigger::processCommand")
    }

    /// C++: `double MSLaneSpeedTrigger::getDefaultSpeed() const` [const]
    // double
    // MSLaneSpeedTrigger::getDefaultSpeed() const {
    //     return myDefaultSpeed;
    // }
    #[allow(unused_variables)]
    pub fn get_default_speed(&self) -> f64 {
        todo!("port MSLaneSpeedTrigger::getDefaultSpeed")
    }

    /// C++: `void MSLaneSpeedTrigger::setOverriding(bool)`
    // void
    // MSLaneSpeedTrigger::setOverriding(bool val) {
    //     myAmOverriding = val;
    //     processCommand(false, MSNet::getInstance()->getCurrentTimeStep());
    // }
    #[allow(unused_variables)]
    pub fn set_overriding(&mut self, val: bool) {
        todo!("port MSLaneSpeedTrigger::setOverriding")
    }

    /// C++: `void MSLaneSpeedTrigger::setOverridingValue(double)`
    // void
    // MSLaneSpeedTrigger::setOverridingValue(double val) {
    //     mySpeedOverrideValue = val;
    //     processCommand(false, MSNet::getInstance()->getCurrentTimeStep());
    // }
    #[allow(unused_variables)]
    pub fn set_overriding_value(&mut self, val: f64) {
        todo!("port MSLaneSpeedTrigger::setOverridingValue")
    }

    /// C++: `double MSLaneSpeedTrigger::getLoadedSpeed()`
    // double
    // MSLaneSpeedTrigger::getLoadedSpeed() {
    //     if (myLoadedSpeeds.empty()) {
    //         return myDefaultSpeed;
    //     }
    //     if (myCurrentSpeedEntry != myLoadedSpeeds.begin()) {
    //         return (*(myCurrentSpeedEntry - 1)).second;
    //     } else {
    //         return myCurrentSpeedEntry->second;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_loaded_speed(&mut self) -> f64 {
        todo!("port MSLaneSpeedTrigger::getLoadedSpeed")
    }

    /// C++: `double MSLaneSpeedTrigger::getCurrentSpeed() const` [const]
    /// Returns the current speed
    // double
    // MSLaneSpeedTrigger::getCurrentSpeed() const {
    //     if (myAmOverriding) {
    //         return mySpeedOverrideValue;
    //     } else {
    //         if (myLoadedSpeeds.empty()) {
    //             return myDefaultSpeed;
    //         }
    //         const SUMOTime now = MSNet::getInstance()->getCurrentTimeStep();
    //         // ok, maybe the first shall not yet be the valid one
    //         if (myCurrentSpeedEntry == myLoadedSpeeds.begin() && myCurrentSpeedEntry->first > now) {
    //             return myDefaultSpeed;
    //         }
    //         // try the loaded
    //         if (myCurrentSpeedEntry != myLoadedSpeeds.end() && myCurrentSpeedEntry->first <= now) {
    //             return myCurrentSpeedEntry->second;
    //         } else {
    //             // we have run past the end of the loaded steps or the current step is not yet active:
    //             // -> use the value of the previous step
    //             return (*(myCurrentSpeedEntry - 1)).second;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_current_speed(&self) -> f64 {
        todo!("port MSLaneSpeedTrigger::getCurrentSpeed")
    }

    /// C++: `double MSLaneSpeedTrigger::getCurrentFriction() const` [const]
    /// Returns the current friction
    // double
    // MSLaneSpeedTrigger::getCurrentFriction() const {
    //     if (myLoadedFrictions.empty()) {
    //         return myDefaultFriction;
    //     }
    //     const SUMOTime now = MSNet::getInstance()->getCurrentTimeStep();
    //     // ok, maybe the first shall not yet be the valid one
    //     if (myCurrentFrictionEntry == myLoadedFrictions.begin() && myCurrentFrictionEntry->first > now) {
    //         return myDefaultFriction;
    //     }
    //     // try the loaded
    //     if (myCurrentFrictionEntry != myLoadedFrictions.end() && myCurrentFrictionEntry->first <= now) {
    //         return myCurrentFrictionEntry->second;
    //     } else {
    //         // we have run past the end of the loaded steps or the current step is not yet active:
    //         // -> use the value of the previous step
    //         return (*(myCurrentFrictionEntry - 1)).second;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_current_friction(&self) -> f64 {
        todo!("port MSLaneSpeedTrigger::getCurrentFriction")
    }

    /// C++: `const std::map<std::string, MSLaneSpeedTrigger *> & MSLaneSpeedTrigger::getInstances()` [static]
    /// return all MSLaneSpeedTrigger instances
    // static const std::map<std::string, MSLaneSpeedTrigger*>& getInstances() {
    //         return myInstances;
    //     }
    #[allow(unused_variables)]
    pub fn get_instances() -> &'static BTreeMap<String, Ptr<MSLaneSpeedTrigger>> {
        todo!("port MSLaneSpeedTrigger::getInstances")
    }

    /// C++: `const std::vector<MSLane *> & MSLaneSpeedTrigger::getLanes()`
    // const std::vector<MSLane*>& getLanes() {
    //         return myDestLanes;
    //     }
    #[allow(unused_variables)]
    pub fn get_lanes(&mut self) -> &Vec<Ptr<MSLane>> {
        todo!("port MSLaneSpeedTrigger::getLanes")
    }

    /// C++: `void MSLaneSpeedTrigger::myStartElement(int, const SUMOSAXAttributes &)` [virtual, protected]
    /// Called on the opening of a tag;
    // void
    // MSLaneSpeedTrigger::myStartElement(int element, const SUMOSAXAttributes& attrs) {
    //     // check whether the correct tag is read
    //     if (element != SUMO_TAG_STEP) {
    //         return;
    //     }
    //     // extract the values
    //     bool ok = true;
    //     const SUMOTime next = attrs.getSUMOTimeReporting(SUMO_ATTR_TIME, getID().c_str(), ok);
    //     double speed = attrs.getOpt<double>(SUMO_ATTR_SPEED, getID().c_str(), ok, -1);
    //     double friction = attrs.getOpt<double>(SUMO_ATTR_FRICTION, getID().c_str(), ok, -1);
    //     // check the values
    //     if (next < 0 || (speed > 0 && !myLoadedSpeeds.empty() && myLoadedSpeeds.back().first > next) ||
    //             (friction > 0 && !myLoadedFrictions.empty() && myLoadedFrictions.back().first > next)) {
    //         WRITE_ERRORF(TL("Invalid or unsorted time entry in vss '%'."), getID());
    //         return;
    //     }
    //     if (speed < 0 && friction < 0) {
    //         speed = myDefaultSpeed;
    //         friction = myDefaultFriction;
    //     }
    //     if (speed < 0 && attrs.hasAttribute(SUMO_ATTR_SPEED)) {
    //         speed = myDefaultSpeed;
    //     }
    //     if (friction < 0 && attrs.hasAttribute(SUMO_ATTR_FRICTION)) {
    //         friction = myDefaultFriction;
    //     }
    //     // set the values for the next step if they are valid
    //     if (speed >= 0) {
    //         if (myLoadedSpeeds.size() != 0 && myLoadedSpeeds.back().first == next) {
    //             WRITE_WARNINGF(TL("Time % was set twice for vss '%'; replacing first entry."), time2string(next), getID());
    //             myLoadedSpeeds.back().second = speed;
    //         } else {
    //             myLoadedSpeeds.push_back(std::make_pair(next, speed));
    //         }
    //     }
    //     if (friction >= 0) {
    //         myLoadedFrictions.push_back(std::make_pair(next, friction));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) {
        todo!("port MSLaneSpeedTrigger::myStartElement")
    }

    /// C++: `void MSLaneSpeedTrigger::myEndElement(int)` [virtual, protected]
    /// Called on the closing of a tag;
    // void
    // MSLaneSpeedTrigger::myEndElement(int element) {
    //     if (element == SUMO_TAG_VSS && !myDidInit) {
    //         init();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_end_element(&mut self, element: i32) {
        todo!("port MSLaneSpeedTrigger::myEndElement")
    }

    /// C++: `void MSLaneSpeedTrigger::MSLaneSpeedTrigger(const MSLaneSpeedTrigger &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSLaneSpeedTrigger) -> Self {
        todo!("port MSLaneSpeedTrigger::MSLaneSpeedTrigger")
    }

    /// C++: `MSLaneSpeedTrigger & MSLaneSpeedTrigger::operator=(const MSLaneSpeedTrigger &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSLaneSpeedTrigger) -> &mut MSLaneSpeedTrigger {
        todo!("port MSLaneSpeedTrigger::operator=")
    }

    /// C++: `void MSLaneSpeedTrigger::init()` [private]
    // void
    // MSLaneSpeedTrigger::init() {
    //     // set the process to the begin
    //     myCurrentSpeedEntry = myLoadedSpeeds.begin();
    //     myCurrentFrictionEntry = myLoadedFrictions.begin();
    //     // pass previous time steps
    //     const SUMOTime now = MSNet::getInstance()->getCurrentTimeStep();
    //     while (myCurrentSpeedEntry != myLoadedSpeeds.end() && myCurrentSpeedEntry->first < now) {
    //         processCommand(true, now);
    //     }
    //     while (myCurrentFrictionEntry != myLoadedFrictions.end() && myCurrentFrictionEntry->first < now) {
    //         executeFrictionChange(now);
    //     }
    //
    //     // add the processing to the event handler
    //     if (myCurrentSpeedEntry != myLoadedSpeeds.end()) {
    //         MSNet::getInstance()->getBeginOfTimestepEvents()->addEvent(
    //             new WrappingCommand<MSLaneSpeedTrigger>(this, &MSLaneSpeedTrigger::executeSpeedChange),
    //             myCurrentSpeedEntry->first);
    //     }
    //     if (myCurrentFrictionEntry != myLoadedFrictions.end()) {
    //         MSNet::getInstance()->getBeginOfTimestepEvents()->addEvent(
    //             new WrappingCommand<MSLaneSpeedTrigger>(this, &MSLaneSpeedTrigger::executeFrictionChange),
    //             myCurrentFrictionEntry->first);
    //     }
    //     myDidInit = true;
    // }
    #[allow(unused_variables)]
    pub fn init(&mut self) {
        todo!("port MSLaneSpeedTrigger::init")
    }

}

