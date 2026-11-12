//! Port of microsim/trigger/MSLaneSpeedTrigger.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use crate::microsim::ms_event_control::Command;
use std::cell::RefCell;
use utils::common::util_exceptions::ProcessError;
use utils::xml::generic_sax_handler::{GenericSAXHandler, SaxHandler};
use utils::xml::sumo_sax_attributes::SUMOSAXAttributes;

/// Changes the speed allowed on a set of lanes
/// C++ class `MSLaneSpeedTrigger` (microsim/trigger/MSLaneSpeedTrigger.h)
pub struct MSLaneSpeedTrigger {
    /// C++ base class `Named`
    pub base: Named,
    /// C++ base class `SUMOSAXHandler`
    pub base2: GenericSAXHandler,
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
    /// iterator into myLoadedSpeeds; an index, myLoadedSpeeds.len() is end()
    pub myCurrentSpeedEntry: usize,
    /// iterator into myLoadedFrictions; an index, myLoadedFrictions.len() is end()
    pub myCurrentFrictionEntry: usize,
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

thread_local! {
    /// C++ `MSLaneSpeedTrigger::myInstances`
    static MY_INSTANCES: RefCell<BTreeMap<String, Ptr<MSLaneSpeedTrigger>>> = RefCell::new(BTreeMap::new());
}

/// C++ `WrappingCommand<MSLaneSpeedTrigger>` bound to executeSpeedChange or executeFrictionChange
struct LaneSpeedTriggerCommand {
    trigger: Ptr<MSLaneSpeedTrigger>,
    friction: bool,
}

impl Command for LaneSpeedTriggerCommand {
    fn execute(&mut self, currentTime: SUMOTime) -> SUMOTime {
        if self.friction {
            self.trigger.m().execute_friction_change(currentTime)
        } else {
            self.trigger.m().execute_speed_change(currentTime)
        }
    }
}

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
    // Returns a Box: init() schedules events and the registry stores `this`, so the object must not
    // move after that point. C++ throws ProcessError, which is the Err arm here.
    pub fn new(id: &str, destLanes: &Vec<Ptr<MSLane>>, file: &str) -> Result<Box<Self>, ProcessError> {
        let mut this = Box::new(MSLaneSpeedTrigger {
            base: Named::new(id),
            base2: utils::xml::sumo_sax_handler::new(file, ""),
            myDestLanes: destLanes.clone(),
            myDefaultSpeed: destLanes[0].get_speed_limit(),
            myDefaultFriction: destLanes[0].get_friction_coefficient(),
            myAmOverriding: false,
            mySpeedOverrideValue: destLanes[0].get_speed_limit(),
            myLoadedSpeeds: Vec::new(),
            myLoadedFrictions: Vec::new(),
            myCurrentSpeedEntry: 0,
            myCurrentFrictionEntry: 0,
            myDidInit: false,
        });
        MY_INSTANCES.with(|m| {
            m.borrow_mut().insert(id.to_string(), Ptr::from_box(&this));
        });
        if file != "" {
            let parsed = match utils::xml::xml_sub_sys::run_parser(&mut *this, file, false, false, false, true) {
                Ok(parsed) => parsed,
                Err(e) => {
                    // C++ leaves the dangling registry entry; removing it keeps the registry valid
                    MY_INSTANCES.with(|m| m.borrow_mut().remove(id));
                    return Err(e);
                }
            };
            if !parsed {
                MY_INSTANCES.with(|m| m.borrow_mut().remove(id));
                return Err(ProcessError::new(""));
            }
            if !this.myDidInit {
                this.init();
            }
        }
        Ok(this)
    }

    /// C++: `void MSLaneSpeedTrigger::~MSLaneSpeedTrigger() noexcept` [virtual]
    /// Destructor
    // MSLaneSpeedTrigger::~MSLaneSpeedTrigger() {
    //     myInstances.erase(getID());
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        let id = self.base.get_id().to_string();
        MY_INSTANCES.with(|m| m.borrow_mut().remove(&id));
    }

    /// C++: `SUMOTime MSLaneSpeedTrigger::executeSpeedChange(SUMOTime)`
    /// Executes a switch command
    // SUMOTime
    // MSLaneSpeedTrigger::executeSpeedChange(SUMOTime currentTime) {
    //     return processCommand(true, currentTime);
    // }
    #[allow(unused_variables)]
    pub fn execute_speed_change(&mut self, currentTime: SUMOTime) -> SUMOTime {
        self.process_command(true, currentTime)
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
        let friction = self.get_current_friction();
        for lane in &self.myDestLanes {
            lane.m().set_friction_coefficient(friction);
        }
        if self.myCurrentFrictionEntry != self.myLoadedFrictions.len() {
            self.myCurrentFrictionEntry += 1;
        }
        if self.myCurrentFrictionEntry != self.myLoadedFrictions.len() {
            return self.myLoadedFrictions[self.myCurrentFrictionEntry].0 - currentTime;
        }
        0
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
        let speed = self.get_current_speed();
        let altered = speed != self.myDefaultSpeed;
        for lane in &self.myDestLanes {
            lane.m().set_max_speed(speed, altered, -1.0);
        }
        if !move2next {
            // changed from the gui
            return 0;
        }
        if self.myCurrentSpeedEntry != self.myLoadedSpeeds.len() {
            self.myCurrentSpeedEntry += 1;
        }
        if self.myCurrentSpeedEntry != self.myLoadedSpeeds.len() {
            return self.myLoadedSpeeds[self.myCurrentSpeedEntry].0 - currentTime;
        }
        0
    }

    /// C++: `double MSLaneSpeedTrigger::getDefaultSpeed() const` [const]
    // double
    // MSLaneSpeedTrigger::getDefaultSpeed() const {
    //     return myDefaultSpeed;
    // }
    #[allow(unused_variables)]
    pub fn get_default_speed(&self) -> f64 {
        self.myDefaultSpeed
    }

    /// C++: `void MSLaneSpeedTrigger::setOverriding(bool)`
    // void
    // MSLaneSpeedTrigger::setOverriding(bool val) {
    //     myAmOverriding = val;
    //     processCommand(false, MSNet::getInstance()->getCurrentTimeStep());
    // }
    #[allow(unused_variables)]
    pub fn set_overriding(&mut self, val: bool) {
        self.myAmOverriding = val;
        self.process_command(false, MSNet::get_instance().get_current_time_step());
    }

    /// C++: `void MSLaneSpeedTrigger::setOverridingValue(double)`
    // void
    // MSLaneSpeedTrigger::setOverridingValue(double val) {
    //     mySpeedOverrideValue = val;
    //     processCommand(false, MSNet::getInstance()->getCurrentTimeStep());
    // }
    #[allow(unused_variables)]
    pub fn set_overriding_value(&mut self, val: f64) {
        self.mySpeedOverrideValue = val;
        self.process_command(false, MSNet::get_instance().get_current_time_step());
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
        if self.myLoadedSpeeds.is_empty() {
            return self.myDefaultSpeed;
        }
        if self.myCurrentSpeedEntry != 0 {
            self.myLoadedSpeeds[self.myCurrentSpeedEntry - 1].1
        } else {
            self.myLoadedSpeeds[self.myCurrentSpeedEntry].1
        }
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
        if self.myAmOverriding {
            return self.mySpeedOverrideValue;
        }
        if self.myLoadedSpeeds.is_empty() {
            return self.myDefaultSpeed;
        }
        let now = MSNet::get_instance().get_current_time_step();
        // ok, maybe the first shall not yet be the valid one
        if self.myCurrentSpeedEntry == 0 && self.myLoadedSpeeds[0].0 > now {
            return self.myDefaultSpeed;
        }
        // try the loaded
        if self.myCurrentSpeedEntry != self.myLoadedSpeeds.len() && self.myLoadedSpeeds[self.myCurrentSpeedEntry].0 <= now {
            self.myLoadedSpeeds[self.myCurrentSpeedEntry].1
        } else {
            // we have run past the end of the loaded steps or the current step is not yet active:
            // -> use the value of the previous step
            self.myLoadedSpeeds[self.myCurrentSpeedEntry - 1].1
        }
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
        if self.myLoadedFrictions.is_empty() {
            return self.myDefaultFriction;
        }
        let now = MSNet::get_instance().get_current_time_step();
        // ok, maybe the first shall not yet be the valid one
        if self.myCurrentFrictionEntry == 0 && self.myLoadedFrictions[0].0 > now {
            return self.myDefaultFriction;
        }
        // try the loaded
        if self.myCurrentFrictionEntry != self.myLoadedFrictions.len() && self.myLoadedFrictions[self.myCurrentFrictionEntry].0 <= now {
            self.myLoadedFrictions[self.myCurrentFrictionEntry].1
        } else {
            // we have run past the end of the loaded steps or the current step is not yet active:
            // -> use the value of the previous step
            self.myLoadedFrictions[self.myCurrentFrictionEntry - 1].1
        }
    }

    /// C++: `const std::map<std::string, MSLaneSpeedTrigger *> & MSLaneSpeedTrigger::getInstances()` [static]
    /// return all MSLaneSpeedTrigger instances
    // static const std::map<std::string, MSLaneSpeedTrigger*>& getInstances() {
    //         return myInstances;
    //     }
    #[allow(unused_variables)]
    pub fn get_instances() -> BTreeMap<String, Ptr<MSLaneSpeedTrigger>> {
        // returned by value, as in MSTriggeredRerouter: the C++ static map lives in a thread_local RefCell
        MY_INSTANCES.with(|m| m.borrow().clone())
    }

    /// C++: `const std::vector<MSLane *> & MSLaneSpeedTrigger::getLanes()`
    // const std::vector<MSLane*>& getLanes() {
    //         return myDestLanes;
    //     }
    #[allow(unused_variables)]
    pub fn get_lanes(&mut self) -> &Vec<Ptr<MSLane>> {
        &self.myDestLanes
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
        // check whether the correct tag is read
        if element != SUMO_TAG_STEP {
            return;
        }
        // extract the values
        let mut ok = true;
        let id = self.base.get_id().to_string();
        let next = attrs.get_sumo_time_reporting(SUMO_ATTR_TIME, &id, &mut ok, true);
        let mut speed = attrs.get_opt::<f64>(SUMO_ATTR_SPEED, &id, &mut ok, -1.0, true);
        let mut friction = attrs.get_opt::<f64>(SUMO_ATTR_FRICTION, &id, &mut ok, -1.0, true);
        // check the values
        if next < 0 || (speed > 0.0 && !self.myLoadedSpeeds.is_empty() && self.myLoadedSpeeds.last().unwrap().0 > next) || (friction > 0.0 && !self.myLoadedFrictions.is_empty() && self.myLoadedFrictions.last().unwrap().0 > next) {
            write_errorf!("Invalid or unsorted time entry in vss '%'.", self.base.get_id());
            return;
        }
        if speed < 0.0 && friction < 0.0 {
            speed = self.myDefaultSpeed;
            friction = self.myDefaultFriction;
        }
        if speed < 0.0 && attrs.has_attribute(SUMO_ATTR_SPEED) {
            speed = self.myDefaultSpeed;
        }
        if friction < 0.0 && attrs.has_attribute(SUMO_ATTR_FRICTION) {
            friction = self.myDefaultFriction;
        }
        // set the values for the next step if they are valid
        if speed >= 0.0 {
            if self.myLoadedSpeeds.len() != 0 && self.myLoadedSpeeds.last().unwrap().0 == next {
                write_warningf!("Time % was set twice for vss '%'; replacing first entry.", time2string(next), self.base.get_id());
                self.myLoadedSpeeds.last_mut().unwrap().1 = speed;
            } else {
                self.myLoadedSpeeds.push((next, speed));
            }
        }
        if friction >= 0.0 {
            self.myLoadedFrictions.push((next, friction));
        }
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
        if element == SUMO_TAG_VSS && !self.myDidInit {
            self.init();
        }
    }

    /// C++: `void MSLaneSpeedTrigger::MSLaneSpeedTrigger(const MSLaneSpeedTrigger &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSLaneSpeedTrigger) -> Self {
        panic!("MSLaneSpeedTrigger copy constructor is private and invalidated in C++")
    }

    /// C++: `MSLaneSpeedTrigger & MSLaneSpeedTrigger::operator=(const MSLaneSpeedTrigger &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSLaneSpeedTrigger) -> &mut MSLaneSpeedTrigger {
        panic!("MSLaneSpeedTrigger assignment is private and invalidated in C++")
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
        // set the process to the begin
        self.myCurrentSpeedEntry = 0;
        self.myCurrentFrictionEntry = 0;
        // pass previous time steps
        let now = MSNet::get_instance().get_current_time_step();
        while self.myCurrentSpeedEntry != self.myLoadedSpeeds.len() && self.myLoadedSpeeds[self.myCurrentSpeedEntry].0 < now {
            self.process_command(true, now);
        }
        while self.myCurrentFrictionEntry != self.myLoadedFrictions.len() && self.myLoadedFrictions[self.myCurrentFrictionEntry].0 < now {
            self.execute_friction_change(now);
        }

        // add the processing to the event handler
        let this = Ptr::from_mut(self);
        if self.myCurrentSpeedEntry != self.myLoadedSpeeds.len() {
            let at = self.myLoadedSpeeds[self.myCurrentSpeedEntry].0;
            let cmd = LaneSpeedTriggerCommand { trigger: this, friction: false };
            MSNet::get_instance().m().get_begin_of_timestep_events().m().add_event(Ptr::from_raw(Box::into_raw(Box::new(cmd)) as *mut dyn Command), at);
        }
        if self.myCurrentFrictionEntry != self.myLoadedFrictions.len() {
            let at = self.myLoadedFrictions[self.myCurrentFrictionEntry].0;
            let cmd = LaneSpeedTriggerCommand { trigger: this, friction: true };
            MSNet::get_instance().m().get_begin_of_timestep_events().m().add_event(Ptr::from_raw(Box::into_raw(Box::new(cmd)) as *mut dyn Command), at);
        }
        self.myDidInit = true;
    }
}

impl SaxHandler for MSLaneSpeedTrigger {
    fn handler(&mut self) -> &mut GenericSAXHandler {
        &mut self.base2
    }
    fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) -> Result<(), ProcessError> {
        MSLaneSpeedTrigger::my_start_element(self, element, attrs);
        Ok(())
    }
    fn my_end_element(&mut self, element: i32) -> Result<(), ProcessError> {
        MSLaneSpeedTrigger::my_end_element(self, element);
        Ok(())
    }
}

